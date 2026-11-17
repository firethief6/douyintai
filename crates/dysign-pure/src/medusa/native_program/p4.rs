#![cfg_attr(rustfmt, rustfmt::skip)]
// 由 scripts/medusa-vm/gen_native.py 生成，勿手改。
use super::super::Vm;
use super::super::Error;

#[allow(clippy::all, unused_parens, unused_variables, unused_mut)]
pub fn exec(vm: &mut Vm, pc: u64) -> Result<u64, Error> {
    match pc {
        0x1027d7f0 => { vm.decode(0x3422031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 163i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1027d7f4u64).wrapping_add((imm * 4) as u64) } else { 0x1027d7f4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027d7f4u64) }
        0x1027d7f4 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027d7f8u64) }
        0x1027d7f8 => { vm.decode(0x20c8e32fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027d7fcu64) }
        0x1027d7fc => { vm.decode(0x803a436u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 31i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027d800u64) }
        0x1027d800 => { vm.decode(0x433d3062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027d804u64) }
        0x1027d804 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027d808u64) }
        0x1027d808 => { vm.decode(0x2000036u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027d80cu64) }
        0x1027d80c => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027d810u64) }
        0x1027d810 => { vm.decode(0x2001099u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027d814u64) }
        0x1027d814 => { vm.decode(0xa0102640u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 31i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1027d818u64) }
        0x1027d818 => { vm.decode(0x82002031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1027d81cu64).wrapping_add((imm * 4) as u64) } else { 0x1027d81cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027d81cu64) }
        0x1027d81c => { vm.decode(0x8f76efd9u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027d820u64) }
        0x1027d820 => { vm.decode(0xa540101du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 34212i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1027d824u64) }
        0x1027d828 => { vm.decode(0xa036u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027d82cu64) }
        0x1027d82c => { vm.decode(0x1a680231u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 144i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1027d830u64).wrapping_add((imm * 4) as u64) } else { 0x1027d830u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027d830u64) }
        0x1027d830 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027d834u64) } }
        0x1027d834 => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027d838u64) }
        0x1027d838 => { vm.decode(0xa0102640u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 31i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1027d83cu64) }
        0x1027d83c => { vm.decode(0x482a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 137i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027d840u64).wrapping_add((imm * 4) as u64) } else { 0x1027d840u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027d840u64) }
        0x1027d840 => { vm.decode(0x40809099u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027d844u64) }
        0x1027d844 => { vm.decode(0x2057822u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027d848u64) }
        0x1027d848 => { vm.decode(0xbf4feff6u32); let d = 15i16; let s = 4i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027d84cu64) }
        0x1027d84c => { vm.decode(0x8002036u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027d850u64) }
        0x1027d850 => { vm.decode(0x8ad6f849u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1908483i64;
        let src = vm.ld(s) as u32;
        let dst = vm.ld(d) as u32;
        let v = (src << 3) | (dst & 7);
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1027d854u64) }
        0x1027d854 => { vm.decode(0x87580ff5u32); let d = 1i16; let s = 6i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shl(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x1027d858u64) }
        0x1027d858 => { vm.decode(0x76008835u32); let d = 24i16; let s = 13i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d85cu64) }
        0x1027d85c => { vm.decode(0xc823c909u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1027d860u64) }
        0x1027d860 => { vm.decode(0x8c532fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027d864u64) }
        0x1027d864 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027d868u64) }
        0x1027d868 => { vm.decode(0x39081430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1032i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027d86cu64) }
        0x1027d86c => { vm.decode(0x6000092fu32); let d = 25i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027d870u64) }
        0x1027d870 => { vm.decode(0x39181530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1048i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027d874u64) }
        0x1027d874 => { vm.decode(0x200c023eu32); let d = 17i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1027d878u64) }
        0x1027d878 => { vm.decode(0xc4101349u32); let d = 18i16; let s = 17i16; let s2 = -1i16; let imm = 260i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1027d87cu64) }
        0x1027d87c => { vm.decode(0x81cc17f5u32); let d = 24i16; let s = 24i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d880u64) }
        0x1027d880 => { vm.decode(0xb9634909u32); let d = 25i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1027d884u64) }
        0x1027d884 => { vm.decode(0x508236u32); let d = 18i16; let s = 4i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027d888u64) }
        0x1027d888 => { vm.decode(0x8008236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027d88cu64) }
        0x1027d88c => { vm.decode(0x42e6e849u32); let d = 18i16; let s = 2i16; let s2 = -1i16; let imm = 1908483i64;
        let src = vm.ld(s) as u32;
        let dst = vm.ld(d) as u32;
        let v = (src << 3) | (dst & 7);
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1027d890u64) }
        0x1027d890 => { vm.decode(0xdd506ff5u32); let d = 25i16; let s = 25i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shl(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x1027d894u64) }
        0x1027d894 => { vm.decode(0xa508036u32); let d = 20i16; let s = 4i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027d898u64) }
        0x1027d898 => { vm.decode(0x8c909349u32); let d = 21i16; let s = 17i16; let s2 = -1i16; let imm = 262i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1027d89cu64) }
        0x1027d89c => { vm.decode(0xa520236u32); let d = 22i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027d8a0u64) }
        0x1027d8a0 => { vm.decode(0x300a236u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027d8a4u64) }
        0x1027d8a4 => { vm.decode(0xb382875u32); let d = 24i16; let s = 25i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d8a8u64) }
        0x1027d8a8 => { vm.decode(0x921a4ff5u32); let d = 25i16; let s = 6i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shl(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x1027d8acu64) }
        0x1027d8ac => { vm.decode(0x62fef849u32); let d = 22i16; let s = 15i16; let s2 = -1i16; let imm = 1908483i64;
        let src = vm.ld(s) as u32;
        let dst = vm.ld(d) as u32;
        let v = (src << 3) | (dst & 7);
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1027d8b0u64) }
        0x1027d8b0 => { vm.decode(0x252a236u32); let d = 23i16; let s = 4i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027d8b4u64) }
        0x1027d8b4 => { vm.decode(0xc967c909u32); let d = 21i16; let s = 21i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1027d8b8u64) }
        0x1027d8b8 => { vm.decode(0x8028236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027d8bcu64) }
        0x1027d8bc => { vm.decode(0x42e2f849u32); let d = 20i16; let s = 2i16; let s2 = -1i16; let imm = 1908483i64;
        let src = vm.ld(s) as u32;
        let dst = vm.ld(d) as u32;
        let v = (src << 3) | (dst & 7);
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1027d8c0u64) }
        0x1027d8c0 => { vm.decode(0x7c909349u32); let d = 31i16; let s = 17i16; let s2 = -1i16; let imm = 259i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1027d8c4u64) }
        0x1027d8c4 => { vm.decode(0x3420036u32); let d = 12i16; let s = 4i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027d8c8u64) }
        0x1027d8c8 => { vm.decode(0x963a0ff5u32); let d = 19i16; let s = 6i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shl(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x1027d8ccu64) }
        0x1027d8cc => { vm.decode(0x503a2ff5u32); let d = 18i16; let s = 21i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shl(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x1027d8d0u64) }
        0x1027d8d0 => { vm.decode(0x8020236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027d8d4u64) }
        0x1027d8d4 => { vm.decode(0xc2e6f849u32); let d = 23i16; let s = 2i16; let s2 = -1i16; let imm = 1908483i64;
        let src = vm.ld(s) as u32;
        let dst = vm.ld(d) as u32;
        let v = (src << 3) | (dst & 7);
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1027d8d8u64) }
        0x1027d8d8 => { vm.decode(0x2a480adu32); let d = 21i16; let s = 17i16; let s2 = -1i16; let imm = 128i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1027d8dcu64) }
        0x1027d8dc => { vm.decode(0x42804875u32); let d = 14i16; let s = 0i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d8e0u64) }
        0x1027d8e0 => { vm.decode(0x28236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027d8e4u64) }
        0x1027d8e4 => { vm.decode(0xae6f849u32); let d = 14i16; let s = 2i16; let s2 = -1i16; let imm = 1908483i64;
        let src = vm.ld(s) as u32;
        let dst = vm.ld(d) as u32;
        let v = (src << 3) | (dst & 7);
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1027d8e8u64) }
        0x1027d8e8 => { vm.decode(0x20236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027d8ecu64) }
        0x1027d8ec => { vm.decode(0xae2f849u32); let d = 12i16; let s = 2i16; let s2 = -1i16; let imm = 1908483i64;
        let src = vm.ld(s) as u32;
        let dst = vm.ld(d) as u32;
        let v = (src << 3) | (dst & 7);
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1027d8f0u64) }
        0x1027d8f0 => { vm.decode(0x7612a835u32); let d = 25i16; let s = 13i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d8f4u64) }
        0x1027d8f4 => { vm.decode(0x83ce07f5u32); let d = 25i16; let s = 25i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d8f8u64) }
        0x1027d8f8 => { vm.decode(0xb4812du32); let d = 7i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1027d8fcu64) }
        0x1027d8fc => { vm.decode(0xfb6fc909u32); let d = 31i16; let s = 31i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1027d900u64) }
        0x1027d900 => { vm.decode(0x1c100349u32); let d = 8i16; let s = 17i16; let s2 = -1i16; let imm = 258i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1027d904u64) }
        0x1027d904 => { vm.decode(0x9500036u32); let d = 24i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027d908u64) }
        0x1027d908 => { vm.decode(0x860a4ff5u32); let d = 9i16; let s = 6i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shl(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x1027d90cu64) }
        0x1027d90c => { vm.decode(0x6684ff5u32); let d = 10i16; let s = 6i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shl(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x1027d910u64) }
        0x1027d910 => { vm.decode(0x86784ff5u32); let d = 11i16; let s = 6i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shl(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x1027d914u64) }
        0x1027d914 => { vm.decode(0xc0f863b5u32); let d = 21i16; let s = 21i16; let s2 = -1i16; let imm = 7i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1027d918u64) }
        0x1027d918 => { vm.decode(0xda27c909u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1027d91cu64) }
        0x1027d91c => { vm.decode(0x401636u32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 16i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027d920u64) }
        0x1027d920 => { vm.decode(0xe4900349u32); let d = 3i16; let s = 17i16; let s2 = -1i16; let imm = 261i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1027d924u64) }
        0x1027d924 => { vm.decode(0x871a0ff5u32); let d = 1i16; let s = 6i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shl(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x1027d928u64) }
        0x1027d928 => { vm.decode(0xe70a875u32); let d = 25i16; let s = 18i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d92cu64) }
        0x1027d92c => { vm.decode(0x74588835u32); let d = 18i16; let s = 13i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d930u64) }
        0x1027d930 => { vm.decode(0x2413036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027d934u64) }
        0x1027d934 => { vm.decode(0x637d1062u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027d938u64) }
        0x1027d938 => { vm.decode(0x10b211adu32); let d = 19i16; let s = 24i16; let s2 = -1i16; let imm = 7i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1027d93cu64) }
        0x1027d93c => { vm.decode(0x9a23c909u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1027d940u64) }
        0x1027d940 => { vm.decode(0xa8294909u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1027d944u64) }
        0x1027d944 => { vm.decode(0x540a3ff5u32); let d = 20i16; let s = 21i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shl(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x1027d948u64) }
        0x1027d948 => { vm.decode(0x1d14c151u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027d94cu64) }
        0x1027d94c => { vm.decode(0x2c900e35u32); let d = 2i16; let s = 21i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s) < vm.ld(s2)) as u64;
        vm.st(d, v);
        Ok(0x1027d950u64) }
        0x1027d950 => { vm.decode(0x4e787ff5u32); let d = 14i16; let s = 31i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shl(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x1027d954u64) }
        0x1027d954 => { vm.decode(0x724aa835u32); let d = 11i16; let s = 13i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d958u64) }
        0x1027d958 => { vm.decode(0x86683ff5u32); let d = 7i16; let s = 7i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shl(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x1027d95cu64) }
        0x1027d95c => { vm.decode(0x524a8835u32); let d = 10i16; let s = 13i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d960u64) }
        0x1027d960 => { vm.decode(0x7202a835u32); let d = 9i16; let s = 13i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d964u64) }
        0x1027d964 => { vm.decode(0xd124ff5u32); let d = 8i16; let s = 8i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shl(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x1027d968u64) }
        0x1027d968 => { vm.decode(0x7100a835u32); let d = 5i16; let s = 13i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d96cu64) }
        0x1027d96c => { vm.decode(0x83322ff5u32); let d = 3i16; let s = 3i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shl(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x1027d970u64) }
        0x1027d970 => { vm.decode(0x31a5ff5u32); let d = 12i16; let s = 6i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shl(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x1027d974u64) }
        0x1027d974 => { vm.decode(0x24901349u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 257i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1027d978u64) }
        0x1027d978 => { vm.decode(0x858e17f5u32); let d = 25i16; let s = 18i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d97cu64) }
        0x1027d97c => { vm.decode(0x8963c909u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1027d980u64) }
        0x1027d980 => { vm.decode(0xd4122ff5u32); let d = 17i16; let s = 17i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shl(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x1027d984u64) }
        0x1027d984 => { vm.decode(0xe38a875u32); let d = 25i16; let s = 17i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d988u64) }
        0x1027d988 => { vm.decode(0x53038835u32); let d = 12i16; let s = 13i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d98cu64) }
        0x1027d98c => { vm.decode(0x14d57f5u32); let d = 12i16; let s = 12i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d990u64) }
        0x1027d990 => { vm.decode(0x4068c875u32); let d = 3i16; let s = 3i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d994u64) }
        0x1027d994 => { vm.decode(0x421317f5u32); let d = 3i16; let s = 5i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d998u64) }
        0x1027d998 => { vm.decode(0x45009875u32); let d = 3i16; let s = 8i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d99cu64) }
        0x1027d99c => { vm.decode(0x425217f5u32); let d = 3i16; let s = 9i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d9a0u64) }
        0x1027d9a0 => { vm.decode(0x5400b875u32); let d = 3i16; let s = 20i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d9a4u64) }
        0x1027d9a4 => { vm.decode(0x445217f5u32); let d = 3i16; let s = 10i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d9a8u64) }
        0x1027d9a8 => { vm.decode(0x54489875u32); let d = 3i16; let s = 7i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d9acu64) }
        0x1027d9ac => { vm.decode(0x465217f5u32); let d = 3i16; let s = 11i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d9b0u64) }
        0x1027d9b0 => { vm.decode(0x55409875u32); let d = 3i16; let s = 14i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d9b4u64) }
        0x1027d9b4 => { vm.decode(0x2080732fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1032i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027d9b8u64) }
        0x1027d9b8 => { vm.decode(0x88002071u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 14i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1027d9bcu64).wrapping_add((imm * 4) as u64) } else { 0x1027d9bcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027d9bcu64) }
        0x1027d9bc => { vm.decode(0x10000530u32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027d9c0u64) }
        0x1027d9c0 => { vm.decode(0x6082736fu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027d9c4u64) }
        0x1027d9c4 => { vm.decode(0x408a536fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027d9c8u64) }
        0x1027d9c8 => { vm.decode(0x4a86c3efu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027d9ccu64) }
        0x1027d9cc => { vm.decode(0x2288732fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1048i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027d9d0u64) }
        0x1027d9d0 => { vm.decode(0x45104851u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027d9d4u64) }
        0x1027d9d4 => { vm.decode(0xd9ce311u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027d9d8u64) }
        0x1027d9d8 => { vm.decode(0xa3000446u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1027d9dcu64) }
        0x1027d9dc => { vm.decode(0x631c3ca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -110i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027d9e0u64).wrapping_add((imm * 4) as u64) } else { 0x1027d9e0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027d9e0u64) }
        0x1027d9e0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027d9e4u64) } }
        0x1027d9e4 => { vm.decode(0x6b1dbda7u32); let d = 0i16; let s = 15i16; let s2 = -1i16; let imm = -109i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027d9e8u64).wrapping_add((imm * 4) as u64) } else { 0x1027d9e8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027d9e8u64) }
        0x1027da64 => { vm.decode(0x4598ea11u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027da68u64) }
        0x1027da68 => { vm.decode(0x8460101du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 34240i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1027da6cu64) }
        0x1027da6c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027da70u64) } }
        0x1027da70 => { vm.decode(0x4598ea11u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027da74u64) }
        0x1027da80 => { vm.decode(0x423d7062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027da84u64) }
        0x1027da84 => { vm.decode(0xf106091u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2308i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027da88u64) }
        0x1027da88 => { vm.decode(0x80808b35u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_sub(vm.ld(s2) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1027da8cu64) }
        0x1027da8c => { vm.decode(0x4a120973u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1027da90u64) }
        0x1027da90 => { vm.decode(0x5b106c73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2548i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1027da94u64) }
        0x1027da94 => { vm.decode(0x66798559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3952i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027da98u64) }
        0x1027da98 => { vm.decode(0x22036u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027da9cu64) }
        0x1027da9c => { vm.decode(0x451c4211u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027daa0u64) }
        0x1027daa0 => { vm.decode(0x82248a05u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u16;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027daa4u64) }
        0x1027daa4 => { vm.decode(0xaa8606fu32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027daa8u64) }
        0x1027daa8 => { vm.decode(0xb3d1862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027daacu64) }
        0x1027daac => { vm.decode(0xab01dd70u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dab0u64) }
        0x1027dab0 => { vm.decode(0x2a84e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dab4u64) }
        0x1027dab4 => { vm.decode(0x3b391cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dab8u64) }
        0x1027dab8 => { vm.decode(0x2b311db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dabcu64) }
        0x1027dabc => { vm.decode(0xe10c875u32); let d = 25i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dac0u64) }
        0x1027dac0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027dac4u64 + 4);
        vm.wr(37, 0x1027dac4u64 + 4);
        Ok(0x1027dac4u64) }
        0x1027dac4 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dac8u64) }
        0x1027dac8 => { vm.decode(0xd14c051u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027daccu64) }
        0x1027dacc => { vm.decode(0x8865c909u32); let d = 20i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1027dad0u64) }
        0x1027dad0 => { vm.decode(0xd9062d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1860i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027dad4u64) }
        0x1027dad4 => { vm.decode(0x459cc2d1u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1772i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027dad8u64) }
        0x1027dad8 => { vm.decode(0x40210fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1027dadcu64) }
        0x1027dadc => { vm.decode(0xac406fu32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dae0u64) }
        0x1027dae0 => { vm.decode(0x52184b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 3912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1027dae4u64) }
        0x1027dae4 => { vm.decode(0x628ac3efu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dae8u64) }
        0x1027dae8 => { vm.decode(0xbb215d30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 3904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027daecu64) }
        0x1027daec => { vm.decode(0x2b191db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027daf0u64) }
        0x1027daf0 => { vm.decode(0x82403e3eu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1027daf4u64) }
        0x1027daf4 => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027daf8u64) }
        0x1027daf8 => { vm.decode(0x216004f5u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027dafcu64) }
        0x1027dafc => { vm.decode(0x820bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1027db00u64) }
        0x1027db00 => { vm.decode(0x23036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027db04u64) }
        0x1027db04 => { vm.decode(0x4b1d1c62u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3916i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027db08u64) }
        0x1027db08 => { vm.decode(0x882c3efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027db0cu64) }
        0x1027db0c => { vm.decode(0xe10c875u32); let d = 25i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027db10u64) }
        0x1027db10 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027db14u64 + 4);
        vm.wr(37, 0x1027db14u64 + 4);
        Ok(0x1027db14u64) }
        0x1027db14 => { vm.decode(0x667d8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3896i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027db18u64) }
        0x1027db18 => { vm.decode(0x74698459u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 3856i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027db1cu64) }
        0x1027db1c => { vm.decode(0x20a0406fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027db20u64) }
        0x1027db20 => { vm.decode(0x459cc2d1u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1772i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027db24u64) }
        0x1027db24 => { vm.decode(0x2b091db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027db28u64) }
        0x1027db28 => { vm.decode(0xa8c53efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027db2cu64) }
        0x1027db2c => { vm.decode(0x2b011db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027db30u64) }
        0x1027db30 => { vm.decode(0x88c73afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027db34u64) }
        0x1027db34 => { vm.decode(0x2b191d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027db38u64) }
        0x1027db38 => { vm.decode(0x3b111c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027db3cu64) }
        0x1027db3c => { vm.decode(0x688ce3afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027db40u64) }
        0x1027db40 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027db44u64) }
        0x1027db44 => { vm.decode(0xe10c875u32); let d = 25i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027db48u64) }
        0x1027db48 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027db4cu64 + 4);
        vm.wr(37, 0x1027db4cu64 + 4);
        Ok(0x1027db4cu64) }
        0x1027db4c => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027db50u64) }
        0x1027db50 => { vm.decode(0x20a0406fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027db54u64) }
        0x1027db54 => { vm.decode(0x459062d1u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1860i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027db58u64) }
        0x1027db58 => { vm.decode(0x2b091db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027db5cu64) }
        0x1027db5c => { vm.decode(0x88f3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027db60u64) }
        0x1027db60 => { vm.decode(0x2b011db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027db64u64) }
        0x1027db64 => { vm.decode(0x3b111c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027db68u64) }
        0x1027db68 => { vm.decode(0xd9cc2d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1772i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027db6cu64) }
        0x1027db6c => { vm.decode(0x88c73afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027db70u64) }
        0x1027db70 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027db74u64) }
        0x1027db74 => { vm.decode(0x3b191c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027db78u64) }
        0x1027db78 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027db7cu64) }
        0x1027db7c => { vm.decode(0xe10c875u32); let d = 25i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027db80u64) }
        0x1027db80 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027db84u64 + 4);
        vm.wr(37, 0x1027db84u64 + 4);
        Ok(0x1027db84u64) }
        0x1027db84 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027db88u64) }
        0x1027db88 => { vm.decode(0x10001099u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027db8cu64) }
        0x1027db8c => { vm.decode(0x20a0406fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027db90u64) }
        0x1027db90 => { vm.decode(0xab091db0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 3880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027db94u64) }
        0x1027db94 => { vm.decode(0x288e3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027db98u64) }
        0x1027db98 => { vm.decode(0x2b011db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027db9cu64) }
        0x1027db9c => { vm.decode(0x3b111c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dba0u64) }
        0x1027dba0 => { vm.decode(0xd9cc2d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1772i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027dba4u64) }
        0x1027dba4 => { vm.decode(0x88c73afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dba8u64) }
        0x1027dba8 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027dbacu64) }
        0x1027dbac => { vm.decode(0x459062d1u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1860i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027dbb0u64) }
        0x1027dbb0 => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027dbb4u64) }
        0x1027dbb4 => { vm.decode(0x3b191c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dbb8u64) }
        0x1027dbb8 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dbbcu64) }
        0x1027dbbc => { vm.decode(0xe10c875u32); let d = 25i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dbc0u64) }
        0x1027dbc0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027dbc4u64 + 4);
        vm.wr(37, 0x1027dbc4u64 + 4);
        Ok(0x1027dbc4u64) }
        0x1027dbc4 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dbc8u64) }
        0x1027dbc8 => { vm.decode(0x20a0406fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dbccu64) }
        0x1027dbcc => { vm.decode(0xab091db0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 3880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dbd0u64) }
        0x1027dbd0 => { vm.decode(0x88c63afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dbd4u64) }
        0x1027dbd4 => { vm.decode(0x2b011db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dbd8u64) }
        0x1027dbd8 => { vm.decode(0x3b111c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dbdcu64) }
        0x1027dbdc => { vm.decode(0xd9cc2d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1772i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027dbe0u64) }
        0x1027dbe0 => { vm.decode(0x88c73afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dbe4u64) }
        0x1027dbe4 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027dbe8u64) }
        0x1027dbe8 => { vm.decode(0x459062d1u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1860i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027dbecu64) }
        0x1027dbec => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027dbf0u64) }
        0x1027dbf0 => { vm.decode(0x40008099u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027dbf4u64) }
        0x1027dbf4 => { vm.decode(0x3b191c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dbf8u64) }
        0x1027dbf8 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dbfcu64) }
        0x1027dbfc => { vm.decode(0xe10c875u32); let d = 25i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dc00u64) }
        0x1027dc00 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027dc04u64 + 4);
        vm.wr(37, 0x1027dc04u64 + 4);
        Ok(0x1027dc04u64) }
        0x1027dc04 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dc08u64) }
        0x1027dc08 => { vm.decode(0xd98ea11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027dc0cu64) }
        0x1027dc0c => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1027dc10u64) }
        0x1027dc10 => { vm.decode(0xc8e32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dc14u64) }
        0x1027dc14 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027dc18u64) }
        0x1027dc18 => { vm.decode(0xa8406fu32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dc1cu64) }
        0x1027dc1c => { vm.decode(0xa2403e3eu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1027dc20u64) }
        0x1027dc20 => { vm.decode(0x3b091db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 3880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dc24u64) }
        0x1027dc24 => { vm.decode(0x2b111d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dc28u64) }
        0x1027dc28 => { vm.decode(0x3b011cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dc2cu64) }
        0x1027dc2c => { vm.decode(0xd9cc2d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1772i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027dc30u64) }
        0x1027dc30 => { vm.decode(0x88c73afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dc34u64) }
        0x1027dc34 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027dc38u64) }
        0x1027dc38 => { vm.decode(0x459062d1u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1860i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027dc3cu64) }
        0x1027dc3c => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027dc40u64) }
        0x1027dc40 => { vm.decode(0xc0008019u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027dc44u64) }
        0x1027dc44 => { vm.decode(0x3b191c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dc48u64) }
        0x1027dc48 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dc4cu64) }
        0x1027dc4c => { vm.decode(0xe10c875u32); let d = 25i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dc50u64) }
        0x1027dc50 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027dc54u64 + 4);
        vm.wr(37, 0x1027dc54u64 + 4);
        Ok(0x1027dc54u64) }
        0x1027dc54 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dc58u64) }
        0x1027dc58 => { vm.decode(0x20a0406fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dc5cu64) }
        0x1027dc5c => { vm.decode(0x459cc2d1u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1772i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027dc60u64) }
        0x1027dc60 => { vm.decode(0x288c73afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dc64u64) }
        0x1027dc64 => { vm.decode(0xab095cb0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 3880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dc68u64) }
        0x1027dc68 => { vm.decode(0x86532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dc6cu64) }
        0x1027dc6c => { vm.decode(0x2b015cb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 3872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dc70u64) }
        0x1027dc70 => { vm.decode(0x3b111c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dc74u64) }
        0x1027dc74 => { vm.decode(0x624004f5u32); let d = 1i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027dc78u64) }
        0x1027dc78 => { vm.decode(0x459062d1u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1860i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027dc7cu64) }
        0x1027dc7c => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027dc80u64) }
        0x1027dc80 => { vm.decode(0x82403e3eu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1027dc84u64) }
        0x1027dc84 => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027dc88u64) }
        0x1027dc88 => { vm.decode(0xc0008019u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027dc8cu64) }
        0x1027dc8c => { vm.decode(0x3b191c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dc90u64) }
        0x1027dc90 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dc94u64) }
        0x1027dc94 => { vm.decode(0xe10c875u32); let d = 25i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dc98u64) }
        0x1027dc98 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027dc9cu64 + 4);
        vm.wr(37, 0x1027dc9cu64 + 4);
        Ok(0x1027dc9cu64) }
        0x1027dc9c => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dca0u64) }
        0x1027dca0 => { vm.decode(0x22a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dca4u64) }
        0x1027dca4 => { vm.decode(0x9b095d30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dca8u64) }
        0x1027dca8 => { vm.decode(0x88c3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dcacu64) }
        0x1027dcac => { vm.decode(0xb011d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dcb0u64) }
        0x1027dcb0 => { vm.decode(0x1b3994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dcb4u64) }
        0x1027dcb4 => { vm.decode(0x82e3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dcb8u64) }
        0x1027dcb8 => { vm.decode(0xe10c875u32); let d = 25i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dcbcu64) }
        0x1027dcbc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027dcc0u64 + 4);
        vm.wr(37, 0x1027dcc0u64 + 4);
        Ok(0x1027dcc0u64) }
        0x1027dcc0 => { vm.decode(0x665f8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2808i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027dcc4u64) }
        0x1027dcc4 => { vm.decode(0x2aa0406fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dcc8u64) }
        0x1027dcc8 => { vm.decode(0x8c73efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dcccu64) }
        0x1027dccc => { vm.decode(0xb3195b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dcd0u64) }
        0x1027dcd0 => { vm.decode(0x88cc3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dcd4u64) }
        0x1027dcd4 => { vm.decode(0xb2995b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dcd8u64) }
        0x1027dcd8 => { vm.decode(0x1b2194b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dcdcu64) }
        0x1027dcdc => { vm.decode(0xa86c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dce0u64) }
        0x1027dce0 => { vm.decode(0xe10c875u32); let d = 25i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dce4u64) }
        0x1027dce4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027dce8u64 + 4);
        vm.wr(37, 0x1027dce8u64 + 4);
        Ok(0x1027dce8u64) }
        0x1027dce8 => { vm.decode(0x661b8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2784i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027dcecu64) }
        0x1027dcec => { vm.decode(0x20a4c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dcf0u64) }
        0x1027dcf0 => { vm.decode(0x8c73efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dcf4u64) }
        0x1027dcf4 => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dcf8u64) }
        0x1027dcf8 => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dcfcu64) }
        0x1027dcfc => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dd00u64) }
        0x1027dd00 => { vm.decode(0x82532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dd04u64) }
        0x1027dd04 => { vm.decode(0xe10c875u32); let d = 25i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dd08u64) }
        0x1027dd08 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027dd0cu64 + 4);
        vm.wr(37, 0x1027dd0cu64 + 4);
        Ok(0x1027dd0cu64) }
        0x1027dd0c => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027dd10u64) }
        0x1027dd10 => { vm.decode(0x28c53efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dd14u64) }
        0x1027dd14 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027dd18u64).wrapping_add((imm * 4) as u64) } else { 0x1027dd18u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027dd18u64) }
        0x1027dd18 => { vm.decode(0x4c90c875u32); let d = 23i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dd1cu64) }
        0x1027dd1c => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027dd20u64) }
        0x1027dd20 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027dd24u64) }
        0x1027dd24 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027dd28u64) }
        0x1027dd28 => { vm.decode(0x28c53efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dd2cu64) }
        0x1027dd2c => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027dd30u64) }
        0x1027dd30 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x1027dd34u64).wrapping_add((imm * 4) as u64) } else { 0x1027dd34u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027dd34u64) }
        0x1027dd34 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027dd38u64) } }
        0x1027dd38 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027dd3cu64).wrapping_add((imm * 4) as u64) } else { 0x1027dd3cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027dd3cu64) }
        0x1027dd3c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027dd40u64) } }
        0x1027dd40 => { vm.decode(0x22a0c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dd44u64) }
        0x1027dd44 => { vm.decode(0x2b311530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dd48u64) }
        0x1027dd48 => { vm.decode(0x3b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dd4cu64) }
        0x1027dd4c => { vm.decode(0xa82c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dd50u64) }
        0x1027dd50 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dd54u64) }
        0x1027dd54 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027dd58u64 + 4);
        vm.wr(37, 0x1027dd58u64 + 4);
        Ok(0x1027dd58u64) }
        0x1027dd58 => { vm.decode(0x662d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3656i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027dd5cu64) }
        0x1027dd5c => { vm.decode(0x2b3014b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dd60u64) }
        0x1027dd60 => { vm.decode(0x8c73efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dd64u64) }
        0x1027dd64 => { vm.decode(0x8000a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027dd68u64).wrapping_add((imm * 4) as u64) } else { 0x1027dd68u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027dd68u64) }
        0x1027dd68 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027dd6cu64) } }
        0x1027dd6c => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dd70u64) }
        0x1027dd70 => { vm.decode(0x2000212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dd74u64) }
        0x1027dd74 => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dd78u64) }
        0x1027dd78 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dd7cu64) }
        0x1027dd7c => { vm.decode(0x286532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dd80u64) }
        0x1027dd80 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dd84u64) }
        0x1027dd84 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027dd88u64 + 4);
        vm.wr(37, 0x1027dd88u64 + 4);
        Ok(0x1027dd88u64) }
        0x1027dd88 => { vm.decode(0x62198219u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027dd8cu64) }
        0x1027dd8c => { vm.decode(0x2b2814b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dd90u64) }
        0x1027dd90 => { vm.decode(0x20a4e06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dd94u64) }
        0x1027dd94 => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dd98u64) }
        0x1027dd98 => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dd9cu64) }
        0x1027dd9c => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dda0u64) }
        0x1027dda0 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dda4u64) }
        0x1027dda4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027dda8u64 + 4);
        vm.wr(37, 0x1027dda8u64 + 4);
        Ok(0x1027dda8u64) }
        0x1027dda8 => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027ddacu64) }
        0x1027ddac => { vm.decode(0x70098219u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 4096i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027ddb0u64) }
        0x1027ddb0 => { vm.decode(0x4a099u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 13i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027ddb4u64) }
        0x1027ddb4 => { vm.decode(0x2ac60efu32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ddb8u64) }
        0x1027ddb8 => { vm.decode(0x1d181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027ddbcu64) }
        0x1027ddbc => { vm.decode(0x288043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ddc0u64) }
        0x1027ddc0 => { vm.decode(0x1d101430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027ddc4u64) }
        0x1027ddc4 => { vm.decode(0x8d085570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 4104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027ddc8u64) }
        0x1027ddc8 => { vm.decode(0xd001530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027ddccu64) }
        0x1027ddcc => { vm.decode(0x6882e3efu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ddd0u64) }
        0x1027ddd0 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027ddd4u64) }
        0x1027ddd4 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027ddd8u64) }
        0x1027ddd8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027dddcu64 + 4);
        vm.wr(37, 0x1027dddcu64 + 4);
        Ok(0x1027dddcu64) }
        0x1027dddc => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dde0u64) }
        0x1027dde0 => { vm.decode(0x701d9519u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2152i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027dde4u64) }
        0x1027dde4 => { vm.decode(0x90001099u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027dde8u64) }
        0x1027dde8 => { vm.decode(0x22a460efu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ddecu64) }
        0x1027ddec => { vm.decode(0x8d181530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 4120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027ddf0u64) }
        0x1027ddf0 => { vm.decode(0x88863efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ddf4u64) }
        0x1027ddf4 => { vm.decode(0xd101530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027ddf8u64) }
        0x1027ddf8 => { vm.decode(0x9d081530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 4104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027ddfcu64) }
        0x1027ddfc => { vm.decode(0x1d001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027de00u64) }
        0x1027de00 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027de04u64) }
        0x1027de04 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027de08u64) }
        0x1027de08 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027de0cu64 + 4);
        vm.wr(37, 0x1027de0cu64 + 4);
        Ok(0x1027de0cu64) }
        0x1027de0c => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027de10u64) }
        0x1027de10 => { vm.decode(0x724d0519u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2136i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027de14u64) }
        0x1027de14 => { vm.decode(0x22a460efu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027de18u64) }
        0x1027de18 => { vm.decode(0x8d181530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 4120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027de1cu64) }
        0x1027de1c => { vm.decode(0xa8843efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027de20u64) }
        0x1027de20 => { vm.decode(0xd101530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027de24u64) }
        0x1027de24 => { vm.decode(0x8d085430u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 4104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027de28u64) }
        0x1027de28 => { vm.decode(0x1d001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027de2cu64) }
        0x1027de2c => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027de30u64) }
        0x1027de30 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027de34u64) }
        0x1027de34 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027de38u64 + 4);
        vm.wr(37, 0x1027de38u64 + 4);
        Ok(0x1027de38u64) }
        0x1027de38 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027de3cu64) }
        0x1027de3c => { vm.decode(0x48099u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 9i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027de40u64) }
        0x1027de40 => { vm.decode(0x2ac60efu32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027de44u64) }
        0x1027de44 => { vm.decode(0x1d181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027de48u64) }
        0x1027de48 => { vm.decode(0x288443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027de4cu64) }
        0x1027de4c => { vm.decode(0x1d101430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027de50u64) }
        0x1027de50 => { vm.decode(0x4088c3efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027de54u64) }
        0x1027de54 => { vm.decode(0x8d081530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 4104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027de58u64) }
        0x1027de58 => { vm.decode(0xd001530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027de5cu64) }
        0x1027de5c => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027de60u64) }
        0x1027de60 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027de64u64) }
        0x1027de64 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027de68u64 + 4);
        vm.wr(37, 0x1027de68u64 + 4);
        Ok(0x1027de68u64) }
        0x1027de68 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027de6cu64) }
        0x1027de6c => { vm.decode(0x74598459u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2864i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027de70u64) }
        0x1027de70 => { vm.decode(0xa122036u32); let d = 21i16; let s = 0i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027de74u64) }
        0x1027de74 => { vm.decode(0x28a0606fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027de78u64) }
        0x1027de78 => { vm.decode(0x4b4d1162u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027de7cu64) }
        0x1027de7c => { vm.decode(0x8b195cb0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027de80u64) }
        0x1027de80 => { vm.decode(0x1b111cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027de84u64) }
        0x1027de84 => { vm.decode(0x428acbafu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027de88u64) }
        0x1027de88 => { vm.decode(0xb05875u32); let d = 4i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027de8cu64) }
        0x1027de8c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027de90u64) }
        0x1027de90 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027de94u64 + 4);
        vm.wr(37, 0x1027de94u64 + 4);
        Ok(0x1027de94u64) }
        0x1027de94 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027de98u64) }
        0x1027de98 => { vm.decode(0x8012036u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 9i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027de9cu64) }
        0x1027de9c => { vm.decode(0xaac406fu32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dea0u64) }
        0x1027dea0 => { vm.decode(0x4b0d1062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027dea4u64) }
        0x1027dea4 => { vm.decode(0x8b191db0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dea8u64) }
        0x1027dea8 => { vm.decode(0xb111db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027deacu64) }
        0x1027deac => { vm.decode(0x40c2e3efu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027deb0u64) }
        0x1027deb0 => { vm.decode(0xb05875u32); let d = 4i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027deb4u64) }
        0x1027deb4 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027deb8u64) }
        0x1027deb8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027debcu64 + 4);
        vm.wr(37, 0x1027debcu64 + 4);
        Ok(0x1027debcu64) }
        0x1027debc => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dec0u64) }
        0x1027dec0 => { vm.decode(0x705f1559u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2552i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027dec4u64) }
        0x1027dec4 => { vm.decode(0x28a4606fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dec8u64) }
        0x1027dec8 => { vm.decode(0xc8e3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027deccu64) }
        0x1027decc => { vm.decode(0xb111530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027ded0u64) }
        0x1027ded0 => { vm.decode(0x8b095430u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027ded4u64) }
        0x1027ded4 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ded8u64) }
        0x1027ded8 => { vm.decode(0xb011530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dedcu64) }
        0x1027dedc => { vm.decode(0x19399cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dee0u64) }
        0x1027dee0 => { vm.decode(0x4a8643efu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dee4u64) }
        0x1027dee4 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dee8u64) }
        0x1027dee8 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027deecu64) }
        0x1027deec => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027def0u64 + 4);
        vm.wr(37, 0x1027def0u64 + 4);
        Ok(0x1027def0u64) }
        0x1027def0 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027def4u64) }
        0x1027def4 => { vm.decode(0x2aa4606fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027def8u64) }
        0x1027def8 => { vm.decode(0x4b4d1162u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027defcu64) }
        0x1027defc => { vm.decode(0x9b191db0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027df00u64) }
        0x1027df00 => { vm.decode(0x1b111cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027df04u64) }
        0x1027df04 => { vm.decode(0xb05875u32); let d = 4i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027df08u64) }
        0x1027df08 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027df0cu64) }
        0x1027df0c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027df10u64 + 4);
        vm.wr(37, 0x1027df10u64 + 4);
        Ok(0x1027df10u64) }
        0x1027df10 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027df14u64) }
        0x1027df14 => { vm.decode(0x28a4606fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027df18u64) }
        0x1027df18 => { vm.decode(0xa8853afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027df1cu64) }
        0x1027df1c => { vm.decode(0xb111530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027df20u64) }
        0x1027df20 => { vm.decode(0xc8e3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027df24u64) }
        0x1027df24 => { vm.decode(0xb091530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027df28u64) }
        0x1027df28 => { vm.decode(0xa8c63afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027df2cu64) }
        0x1027df2c => { vm.decode(0xb011530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027df30u64) }
        0x1027df30 => { vm.decode(0x19399cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027df34u64) }
        0x1027df34 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027df38u64) }
        0x1027df38 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027df3cu64) }
        0x1027df3c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027df40u64 + 4);
        vm.wr(37, 0x1027df40u64 + 4);
        Ok(0x1027df40u64) }
        0x1027df40 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027df44u64) }
        0x1027df44 => { vm.decode(0x70198219u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027df48u64) }
        0x1027df48 => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027df4cu64) }
        0x1027df4c => { vm.decode(0x88cc3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027df50u64) }
        0x1027df50 => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027df54u64) }
        0x1027df54 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027df58u64) }
        0x1027df58 => { vm.decode(0x428c532fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027df5cu64) }
        0x1027df5c => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027df60u64) }
        0x1027df60 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027df64u64) }
        0x1027df64 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027df68u64 + 4);
        vm.wr(37, 0x1027df68u64 + 4);
        Ok(0x1027df68u64) }
        0x1027df68 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027df6cu64) }
        0x1027df6c => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027df70u64) }
        0x1027df70 => { vm.decode(0x288c3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027df74u64) }
        0x1027df74 => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027df78u64) }
        0x1027df78 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027df7cu64) }
        0x1027df7c => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027df80u64) }
        0x1027df80 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027df84u64) }
        0x1027df84 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027df88u64 + 4);
        vm.wr(37, 0x1027df88u64 + 4);
        Ok(0x1027df88u64) }
        0x1027df88 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027df8cu64) }
        0x1027df8c => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027df90u64) }
        0x1027df90 => { vm.decode(0xa8ce3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027df94u64) }
        0x1027df94 => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027df98u64) }
        0x1027df98 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027df9cu64) }
        0x1027df9c => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dfa0u64) }
        0x1027dfa0 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dfa4u64) }
        0x1027dfa4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027dfa8u64 + 4);
        vm.wr(37, 0x1027dfa8u64 + 4);
        Ok(0x1027dfa8u64) }
        0x1027dfa8 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dfacu64) }
        0x1027dfac => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dfb0u64) }
        0x1027dfb0 => { vm.decode(0x8d0814b0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dfb4u64) }
        0x1027dfb4 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dfb8u64) }
        0x1027dfb8 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dfbcu64) }
        0x1027dfbc => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dfc0u64) }
        0x1027dfc0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027dfc4u64 + 4);
        vm.wr(37, 0x1027dfc4u64 + 4);
        Ok(0x1027dfc4u64) }
        0x1027dfc4 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dfc8u64) }
        0x1027dfc8 => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dfccu64) }
        0x1027dfcc => { vm.decode(0x288e3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dfd0u64) }
        0x1027dfd0 => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dfd4u64) }
        0x1027dfd4 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dfd8u64) }
        0x1027dfd8 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dfdcu64) }
        0x1027dfdc => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dfe0u64) }
        0x1027dfe0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027dfe4u64 + 4);
        vm.wr(37, 0x1027dfe4u64 + 4);
        Ok(0x1027dfe4u64) }
        0x1027dfe4 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dfe8u64) }
        0x1027dfe8 => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dfecu64) }
        0x1027dfec => { vm.decode(0xa88e3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027dff0u64) }
        0x1027dff0 => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dff4u64) }
        0x1027dff4 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027dff8u64) }
        0x1027dff8 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027dffcu64) }
        0x1027dffc => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e000u64) }
        0x1027e000 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027e004u64 + 4);
        vm.wr(37, 0x1027e004u64 + 4);
        Ok(0x1027e004u64) }
        0x1027e004 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e008u64) }
        0x1027e008 => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027e00cu64) }
        0x1027e00c => { vm.decode(0x8cc3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027e010u64) }
        0x1027e010 => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027e014u64) }
        0x1027e014 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027e018u64) }
        0x1027e018 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e01cu64) }
        0x1027e01c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e020u64) }
        0x1027e020 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027e024u64 + 4);
        vm.wr(37, 0x1027e024u64 + 4);
        Ok(0x1027e024u64) }
        0x1027e024 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e028u64) }
        0x1027e028 => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027e02cu64) }
        0x1027e02c => { vm.decode(0xa8ce3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027e030u64) }
        0x1027e030 => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027e034u64) }
        0x1027e034 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027e038u64) }
        0x1027e038 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e03cu64) }
        0x1027e03c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e040u64) }
        0x1027e040 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027e044u64 + 4);
        vm.wr(37, 0x1027e044u64 + 4);
        Ok(0x1027e044u64) }
        0x1027e044 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e048u64) }
        0x1027e048 => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027e04cu64) }
        0x1027e04c => { vm.decode(0x88cc3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027e050u64) }
        0x1027e050 => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027e054u64) }
        0x1027e054 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027e058u64) }
        0x1027e058 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e05cu64) }
        0x1027e05c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e060u64) }
        0x1027e060 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027e064u64 + 4);
        vm.wr(37, 0x1027e064u64 + 4);
        Ok(0x1027e064u64) }
        0x1027e064 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e068u64) }
        0x1027e068 => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027e06cu64) }
        0x1027e06c => { vm.decode(0x88e3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027e070u64) }
        0x1027e070 => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027e074u64) }
        0x1027e074 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027e078u64) }
        0x1027e078 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e07cu64) }
        0x1027e07c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e080u64) }
        0x1027e080 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027e084u64 + 4);
        vm.wr(37, 0x1027e084u64 + 4);
        Ok(0x1027e084u64) }
        0x1027e084 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e088u64) }
        0x1027e088 => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027e08cu64) }
        0x1027e08c => { vm.decode(0x28cc3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027e090u64) }
        0x1027e090 => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027e094u64) }
        0x1027e094 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027e098u64) }
        0x1027e098 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e09cu64) }
        0x1027e09c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e0a0u64) }
        0x1027e0a0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027e0a4u64 + 4);
        vm.wr(37, 0x1027e0a4u64 + 4);
        Ok(0x1027e0a4u64) }
        0x1027e0a4 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e0a8u64) }
        0x1027e0a8 => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027e0acu64) }
        0x1027e0ac => { vm.decode(0x28ce3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027e0b0u64) }
        0x1027e0b0 => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027e0b4u64) }
        0x1027e0b4 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027e0b8u64) }
        0x1027e0b8 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e0bcu64) }
        0x1027e0bc => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e0c0u64) }
        0x1027e0c0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027e0c4u64 + 4);
        vm.wr(37, 0x1027e0c4u64 + 4);
        Ok(0x1027e0c4u64) }
        0x1027e0c4 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e0c8u64) }
        0x1027e0c8 => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027e0ccu64) }
        0x1027e0cc => { vm.decode(0x888e3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027e0d0u64) }
        0x1027e0d0 => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027e0d4u64) }
        0x1027e0d4 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027e0d8u64) }
        0x1027e0d8 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e0dcu64) }
        0x1027e0dc => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e0e0u64) }
        0x1027e0e0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027e0e4u64 + 4);
        vm.wr(37, 0x1027e0e4u64 + 4);
        Ok(0x1027e0e4u64) }
        0x1027e0e4 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e0e8u64) }
        0x1027e0e8 => { vm.decode(0x4084e3afu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027e0ecu64) }
        0x1027e0ec => { vm.decode(0x20408119u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 80i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027e0f0u64) }
        0x1027e0f0 => { vm.decode(0x8a8402fu32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027e0f4u64) }
        0x1027e0f4 => { vm.decode(0x1d0814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027e0f8u64) }
        0x1027e0f8 => { vm.decode(0xd0015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027e0fcu64) }
        0x1027e0fc => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e100u64) }
        0x1027e100 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e104u64) }
        0x1027e104 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027e108u64 + 4);
        vm.wr(37, 0x1027e108u64 + 4);
        Ok(0x1027e108u64) }
        0x1027e108 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e10cu64) }
        0x1027e10c => { vm.decode(0x20548019u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 56i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027e110u64) }
        0x1027e110 => { vm.decode(0x8a8402fu32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027e114u64) }
        0x1027e114 => { vm.decode(0x1d0814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027e118u64) }
        0x1027e118 => { vm.decode(0xd0015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027e11cu64) }
        0x1027e11c => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e120u64) }
        0x1027e120 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e124u64) }
        0x1027e124 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027e128u64 + 4);
        vm.wr(37, 0x1027e128u64 + 4);
        Ok(0x1027e128u64) }
        0x1027e128 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e12cu64) }
        0x1027e12c => { vm.decode(0x20108019u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027e130u64) }
        0x1027e130 => { vm.decode(0x8a8402fu32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027e134u64) }
        0x1027e134 => { vm.decode(0x1d0814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027e138u64) }
        0x1027e138 => { vm.decode(0xd0015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027e13cu64) }
        0x1027e13c => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e140u64) }
        0x1027e140 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e144u64) }
        0x1027e144 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027e148u64 + 4);
        vm.wr(37, 0x1027e148u64 + 4);
        Ok(0x1027e148u64) }
        0x1027e148 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027e14cu64) }
        0x1027e14c => { vm.decode(0x4294432fu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 4144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027e150u64) }
        0x1027e150 => { vm.decode(0x6294632fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 4152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027e154u64) }
        0x1027e154 => { vm.decode(0x4098436fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 4160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027e158u64) }
        0x1027e158 => { vm.decode(0x6098636fu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 4168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027e15cu64) }
        0x1027e15c => { vm.decode(0x4292436fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 4176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027e160u64) }
        0x1027e160 => { vm.decode(0x6292636fu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 4184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027e164u64) }
        0x1027e164 => { vm.decode(0x409e436fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 4192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027e168u64) }
        0x1027e168 => { vm.decode(0x609e636fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 4200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027e16cu64) }
        0x1027e16c => { vm.decode(0x429e4b6fu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 4208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027e170u64) }
        0x1027e170 => { vm.decode(0x629e6b6fu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 4216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027e174u64) }
        0x1027e174 => { vm.decode(0x54104cf5u32); let d = -1i16; let s = -1i16; let s2 = -1i16; let imm = 0i64;
        vm.jump_flag = 2;
        vm.wr(36, vm.slots[31]);
        vm.wr(35, 2);
        Ok(0x1027e178u64) }
        0x1027e178 => { vm.decode(0x730b8219u32); let d = 29i16; let s = 29i16; let s2 = -1i16; let imm = 4224i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027e17cu64) }
        0x1027e17c => { vm.decode(0x2ac4436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027e180u64) }
        0x1027f0d4 => { vm.decode(0xf9cc091u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2284i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027f0d8u64) }
        0x1027f0d8 => { vm.decode(0x8402a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 140i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027f0dcu64).wrapping_add((imm * 4) as u64) } else { 0x1027f0dcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027f0dcu64) }
        0x1027f0dc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027f0e0u64) } }
        0x1027f0e0 => { vm.decode(0xd1ccad1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1724i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027f0e4u64) }
        0x1027f0e4 => { vm.decode(0x482a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 137i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027f0e8u64).wrapping_add((imm * 4) as u64) } else { 0x1027f0e8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027f0e8u64) }
        0x1027f0e8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027f0ecu64) } }
        0x1027f0ec => { vm.decode(0x2aa0406fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027f0f0u64) }
        0x1027f0f0 => { vm.decode(0x28ce3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027f0f4u64) }
        0x1027f0f4 => { vm.decode(0xb3195b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027f0f8u64) }
        0x1027f0f8 => { vm.decode(0xa8ce3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027f0fcu64) }
        0x1027f0fc => { vm.decode(0xb2995b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027f100u64) }
        0x1027f100 => { vm.decode(0x1b2194b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027f104u64) }
        0x1027f104 => { vm.decode(0xa86c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027f108u64) }
        0x1027f108 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027f10cu64) }
        0x1027f10c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027f110u64 + 4);
        vm.wr(37, 0x1027f110u64 + 4);
        Ok(0x1027f110u64) }
        0x1027f110 => { vm.decode(0x661b8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2784i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027f114u64) }
        0x1027f114 => { vm.decode(0x2436u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027f118u64) }
        0x1027f118 => { vm.decode(0x459062d1u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1860i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027f11cu64) }
        0x1027f11c => { vm.decode(0x890c027u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1029i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027f120u64).wrapping_add((imm * 4) as u64) } else { 0x1027f120u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027f120u64) }
        0x1027f120 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027f124u64) } }
        0x1027f124 => { vm.decode(0x12436u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027f128u64) }
        0x10280134 => { vm.decode(0x2ac4436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280138u64) }
        0x10280138 => { vm.decode(0x42008099u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028013cu64) }
        0x1028013c => { vm.decode(0x1000036u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280140u64) }
        0x10280140 => { vm.decode(0x8c8632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280144u64) }
        0x10280144 => { vm.decode(0x688ce3afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280148u64) }
        0x10280148 => { vm.decode(0x21000040u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1028014cu64) }
        0x1028014c => { vm.decode(0x8c0027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 20i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280150u64).wrapping_add((imm * 4) as u64) } else { 0x10280150u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280150u64) }
        0x10280150 => { vm.decode(0xe8025018u32); let d = 3i16; let s = 30i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10280154u64) }
        0x10280154 => { vm.decode(0xf5b1dffeu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = -1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280158u64) }
        0x10280158 => { vm.decode(0x24085aa0u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 119i64;
        let v = vm.ld(s) ^ imm as u64;
        vm.st(d, v);
        Ok(0x1028015cu64) }
        0x1028015c => { vm.decode(0x9dbe7fb3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = -1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280160u64) }
        0x10280160 => { vm.decode(0x2000123eu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280164u64) }
        0x10280164 => { vm.decode(0x24000aa0u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 19i64;
        let v = vm.ld(s) ^ imm as u64;
        vm.st(d, v);
        Ok(0x10280168u64) }
        0x10280168 => { vm.decode(0x220833u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028016cu64) }
        0x1028016c => { vm.decode(0x4080a019u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10280170u64) }
        0x10280170 => { vm.decode(0x42013019u32); let d = 6i16; let s = 5i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10280174u64) }
        0x10280174 => { vm.decode(0x49000036u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280178u64) }
        0x10280178 => { vm.decode(0x2a0012beu32); let d = 7i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1028017cu64) }
        0x1028017c => { vm.decode(0x38b012a0u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 134i64;
        let v = vm.ld(s) ^ imm as u64;
        vm.st(d, v);
        Ok(0x10280180u64) }
        0x10280180 => { vm.decode(0xa0620833u32); let d = 7i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280184u64) }
        0x10280184 => { vm.decode(0x3800123eu32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280188u64) }
        0x10280188 => { vm.decode(0x309842a0u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 226i64;
        let v = vm.ld(s) ^ imm as u64;
        vm.st(d, v);
        Ok(0x1028018cu64) }
        0x1028018c => { vm.decode(0x6208b3u32); let d = 5i16; let s = 3i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280190u64) }
        0x10280190 => { vm.decode(0x44008875u32); let d = 3i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280194u64) }
        0x10280194 => { vm.decode(0x21000040u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x10280198u64) }
        0x10280198 => { vm.decode(0xa7d12df1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -18i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028019cu64).wrapping_add((imm * 4) as u64) } else { 0x1028019cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028019cu64) }
        0x1028019c => { vm.decode(0x80d875u32); let d = 5i16; let s = 0i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102801a0u64) }
        0x102801a0 => { vm.decode(0x100a236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102801a4u64) }
        0x102801a4 => { vm.decode(0x20073ceu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -494534656i64;
        vm.st(d, imm as u64);
        Ok(0x102801a8u64) }
        0x102801a8 => { vm.decode(0x33a54558u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 4983i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102801acu64) }
        0x102801ac => { vm.decode(0xb1000240u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 12i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102801b0u64) }
        0x102801b0 => { vm.decode(0x2222031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 19i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102801b4u64).wrapping_add((imm * 4) as u64) } else { 0x102801b4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102801b4u64) }
        0x102801b4 => { vm.decode(0xa0c99099u32); let d = 3i16; let s = 30i16; let s2 = -1i16; let imm = 19i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102801b8u64) }
        0x102801b8 => { vm.decode(0x8d40511du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 44456i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102801bcu64) }
        0x102801c0 => { vm.decode(0xf4b9cf7eu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = -18i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102801c4u64) }
        0x102801c4 => { vm.decode(0xd6bbcffeu32); let d = 10i16; let s = 3i16; let s2 = -1i16; let imm = -17i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102801c8u64) }
        0x102801c8 => { vm.decode(0x52422835u32); let d = 9i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102801ccu64) }
        0x102801cc => { vm.decode(0x30002835u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102801d0u64) }
        0x102801d0 => { vm.decode(0xc4bbcffeu32); let d = 8i16; let s = 3i16; let s2 = -1i16; let imm = -19i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102801d4u64) }
        0x102801d4 => { vm.decode(0x410ea835u32); let d = 7i16; let s = 7i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102801d8u64) }
        0x102801d8 => { vm.decode(0xc5bbc73eu32); let d = 8i16; let s = 3i16; let s2 = -1i16; let imm = -16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102801dcu64) }
        0x102801dc => { vm.decode(0xbdfe7f33u32); let d = 7i16; let s = 3i16; let s2 = -1i16; let imm = -3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102801e0u64) }
        0x102801e0 => { vm.decode(0x1dbe7fb3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = -2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102801e4u64) }
        0x102801e4 => { vm.decode(0x9dbf7fb3u32); let d = 9i16; let s = 3i16; let s2 = -1i16; let imm = -1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102801e8u64) }
        0x102801e8 => { vm.decode(0x6a835u32); let d = 1i16; let s = 6i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102801ecu64) }
        0x102801ec => { vm.decode(0x220833u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102801f0u64) }
        0x102801f0 => { vm.decode(0x49203236u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102801f4u64) }
        0x102801f4 => { vm.decode(0xb1000240u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 12i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102801f8u64) }
        0x102801f8 => { vm.decode(0xc8a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102801fcu64).wrapping_add((imm * 4) as u64) } else { 0x102801fcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102801fcu64) }
        0x102801fc => { vm.decode(0x4080b019u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10280200u64) }
        0x10280200 => { vm.decode(0xcfb9cf7eu32); let d = 6i16; let s = 3i16; let s2 = -1i16; let imm = -4i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280204u64) }
        0x10280204 => { vm.decode(0xd5bbcf3eu32); let d = 8i16; let s = 3i16; let s2 = -1i16; let imm = -6i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280208u64) }
        0x10280208 => { vm.decode(0xefb9cfbeu32); let d = 7i16; let s = 3i16; let s2 = -1i16; let imm = -7i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1028020cu64) }
        0x1028020c => { vm.decode(0xf5bbcfbeu32); let d = 9i16; let s = 3i16; let s2 = -1i16; let imm = -5i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280210u64) }
        0x10280210 => { vm.decode(0x82d12du32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280214u64) }
        0x10280214 => { vm.decode(0xa7d12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -22i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10280218u64).wrapping_add((imm * 4) as u64) } else { 0x10280218u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280218u64) }
        0x10280218 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028021cu64) } }
        0x1028021c => { vm.decode(0xd2ca416du32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280220u64) }
        0x10280220 => { vm.decode(0xda5a912du32); let d = 10i16; let s = 9i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280224u64) }
        0x10280224 => { vm.decode(0xd25e112du32); let d = 6i16; let s = 8i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280228u64) }
        0x10280228 => { vm.decode(0xd2dec16du32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1028022cu64) }
        0x1028022c => { vm.decode(0x482324f5u32); let d = 6i16; let s = 6i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10280230u64) }
        0x10280230 => { vm.decode(0xc0a124f5u32); let d = 8i16; let s = 7i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10280234u64) }
        0x10280234 => { vm.decode(0xa08103eu32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280238u64) }
        0x10280238 => { vm.decode(0xc7abb5u32); let d = 7i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028023cu64) }
        0x1028023c => { vm.decode(0x409cc06du32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280240u64) }
        0x10280240 => { vm.decode(0x44b34675u32); let d = 7i16; let s = 5i16; let s2 = 7i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shr(vm.ld(s2) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10280244u64) }
        0x10280244 => { vm.decode(0xd2dec16du32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280248u64) }
        0x10280248 => { vm.decode(0x414da835u32); let d = 7i16; let s = 7i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028024cu64) }
        0x1028024c => { vm.decode(0xa00203eu32); let d = 6i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280250u64) }
        0x10280250 => { vm.decode(0x804124f5u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10280254u64) }
        0x10280254 => { vm.decode(0x2002023eu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280258u64) }
        0x10280258 => { vm.decode(0x404134f5u32); let d = 1i16; let s = 10i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028025cu64) }
        0x1028025c => { vm.decode(0x2023eu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280260u64) }
        0x10280260 => { vm.decode(0x440511du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 44288i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10280264u64) }
        0x10280264 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280268u64) } }
        0x10280268 => { vm.decode(0x20a0406fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028026cu64) }
        0x1028026c => { vm.decode(0x409019u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10280270u64) }
        0x10280270 => { vm.decode(0x3b091db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 3880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280274u64) }
        0x10280274 => { vm.decode(0x2b011db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280278u64) }
        0x10280278 => { vm.decode(0x20db1019u32); let d = 2i16; let s = 30i16; let s2 = -1i16; let imm = 176i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028027cu64) }
        0x1028027c => { vm.decode(0x2b191d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280280u64) }
        0x10280280 => { vm.decode(0x3b111c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280284u64) }
        0x10280284 => { vm.decode(0x74698459u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 3856i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10280288u64) }
        0x10280288 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028028cu64) }
        0x1028028c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280290u64) }
        0x10280290 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10280294u64 + 4);
        vm.wr(37, 0x10280294u64 + 4);
        Ok(0x10280294u64) }
        0x10280294 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280298u64) }
        0x10280298 => { vm.decode(0x20a4e02fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028029cu64) }
        0x1028029c => { vm.decode(0x451ccad1u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1724i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102802a0u64) }
        0x102802a0 => { vm.decode(0x392194b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102802a4u64) }
        0x102802a4 => { vm.decode(0x202436u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102802a8u64) }
        0x102802a8 => { vm.decode(0xb40c4ff6u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = -16i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102802acu64) }
        0x102802ac => { vm.decode(0x381ac7b5u32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 31i64;
        let v = ((vm.ld(s) as i32) >> (imm as u32)) as i64;
        vm.st(d, v as u64);
        Ok(0x102802b0u64) }
        0x102802b0 => { vm.decode(0x70930d35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 28i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102802b4u64) }
        0x102802b4 => { vm.decode(0x44012fb5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102802b8u64) }
        0x102802b8 => { vm.decode(0x840257f5u32); let d = 21i16; let s = 2i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102802bcu64) }
        0x102802bc => { vm.decode(0x48105875u32); let d = 18i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102802c0u64) }
        0x102802c0 => { vm.decode(0xc0e40bb5u32); let d = 22i16; let s = 21i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102802c4u64) }
        0x102802c4 => { vm.decode(0xa929d5b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 3304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102802c8u64) }
        0x102802c8 => { vm.decode(0x286c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102802ccu64) }
        0x102802cc => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102802d0u64) }
        0x102802d0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102802d4u64 + 4);
        vm.wr(37, 0x102802d4u64 + 4);
        Ok(0x102802d4u64) }
        0x102802d4 => { vm.decode(0x623b8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3296i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102802d8u64) }
        0x102802d8 => { vm.decode(0x2020406fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102802dcu64) }
        0x102802dc => { vm.decode(0x451ccad1u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1724i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102802e0u64) }
        0x102802e0 => { vm.decode(0x2b091db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102802e4u64) }
        0x102802e4 => { vm.decode(0x88853efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102802e8u64) }
        0x102802e8 => { vm.decode(0x2b011db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102802ecu64) }
        0x102802ec => { vm.decode(0x4acc536fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 3312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102802f0u64) }
        0x102802f0 => { vm.decode(0xab191d30u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 3864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102802f4u64) }
        0x102802f4 => { vm.decode(0x3b111c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102802f8u64) }
        0x102802f8 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102802fcu64) }
        0x102802fc => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280300u64) }
        0x10280300 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10280304u64 + 4);
        vm.wr(37, 0x10280304u64 + 4);
        Ok(0x10280304u64) }
        0x10280304 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280308u64) }
        0x10280308 => { vm.decode(0x2436u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028030cu64) }
        0x1028030c => { vm.decode(0x451ccad1u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1724i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10280310u64) }
        0x10280310 => { vm.decode(0x301ae7b5u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = 31i64;
        let v = ((vm.ld(s) as i32) >> (imm as u32)) as i64;
        vm.st(d, v as u64);
        Ok(0x10280314u64) }
        0x10280314 => { vm.decode(0x70828d35u32); let d = 4i16; let s = 3i16; let s2 = -1i16; let imm = 28i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10280318u64) }
        0x10280318 => { vm.decode(0x430bb5u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028031cu64) }
        0x1028031c => { vm.decode(0xa20fb5u32); let d = 4i16; let s = 4i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10280320u64) }
        0x10280320 => { vm.decode(0xc244006du32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 240i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280324u64) }
        0x10280324 => { vm.decode(0xd0b35u32); let d = 4i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_sub(vm.ld(s2) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10280328u64) }
        0x10280328 => { vm.decode(0xd24e016du32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1028032cu64) }
        0x1028032c => { vm.decode(0x58b35u32); let d = 4i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_sub(vm.ld(s2) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10280330u64) }
        0x10280330 => { vm.decode(0x1388375u32); let d = 1i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = ((vm.ld(s) as i64) < (vm.ld(s2) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10280334u64) }
        0x10280334 => { vm.decode(0x840827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 6i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280338u64).wrapping_add((imm * 4) as u64) } else { 0x10280338u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280338u64) }
        0x10280338 => { vm.decode(0xca4104f5u32); let d = 3i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028033cu64) }
        0x1028033c => { vm.decode(0x600833u32); let d = 4i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280340u64) }
        0x10280340 => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280344u64) }
        0x10280344 => { vm.decode(0x1388375u32); let d = 1i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = ((vm.ld(s) as i64) < (vm.ld(s2) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10280348u64) }
        0x10280348 => { vm.decode(0xa7f10df1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -4i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028034cu64).wrapping_add((imm * 4) as u64) } else { 0x1028034cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028034cu64) }
        0x1028034c => { vm.decode(0x40809099u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10280350u64) }
        0x10280350 => { vm.decode(0x20db1019u32); let d = 2i16; let s = 30i16; let s2 = -1i16; let imm = 176i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10280354u64) }
        0x10280354 => { vm.decode(0x9019u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10280358u64) }
        0x10280358 => { vm.decode(0x2020036u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028035cu64) }
        0x1028035c => { vm.decode(0x40909875u32); let d = 7i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280360u64) }
        0x10280360 => { vm.decode(0xc2d0e35u32); let d = 1i16; let s = 3i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s) < vm.ld(s2)) as u64;
        vm.st(d, v);
        Ok(0x10280364u64) }
        0x10280364 => { vm.decode(0x82000031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10280368u64).wrapping_add((imm * 4) as u64) } else { 0x10280368u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280368u64) }
        0x10280368 => { vm.decode(0x40801875u32); let d = 6i16; let s = 0i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028036cu64) }
        0x1028036c => { vm.decode(0x8964451du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 47352i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10280370u64) }
        0x10280374 => { vm.decode(0x40c09019u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10280378u64) }
        0x10280378 => { vm.decode(0x42419019u32); let d = 7i16; let s = 5i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028037cu64) }
        0x1028037c => { vm.decode(0xc2d0e35u32); let d = 1i16; let s = 3i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s) < vm.ld(s2)) as u64;
        vm.st(d, v);
        Ok(0x10280380u64) }
        0x10280380 => { vm.decode(0x28482a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 653i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280384u64).wrapping_add((imm * 4) as u64) } else { 0x10280384u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280384u64) }
        0x10280384 => { vm.decode(0x44804875u32); let d = 6i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280388u64) }
        0x10280388 => { vm.decode(0x480d875u32); let d = 5i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028038cu64) }
        0x1028038c => { vm.decode(0x2002236u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280390u64) }
        0x10280390 => { vm.decode(0xd2cac16du32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280394u64) }
        0x10280394 => { vm.decode(0x20002406u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10280398u64) }
        0x10280398 => { vm.decode(0x40a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028039cu64).wrapping_add((imm * 4) as u64) } else { 0x1028039cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028039cu64) }
        0x1028039c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102803a0u64) } }
        0x102803a0 => { vm.decode(0xd2cac16du32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102803a4u64) }
        0x102803a4 => { vm.decode(0x828024f5u32); let d = 8i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102803a8u64) }
        0x102803a8 => { vm.decode(0x2203eu32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102803acu64) }
        0x102803ac => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102803b0u64) }
        0x102803b0 => { vm.decode(0x2002023eu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102803b4u64) }
        0x102803b4 => { vm.decode(0x32020835u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102803b8u64) }
        0x102803b8 => { vm.decode(0x10833u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102803bcu64) }
        0x102803bc => { vm.decode(0x84e0451du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 44752i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102803c0u64) }
        0x102803c0 => { vm.decode(0xa603236u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102803c4u64) }
        0x102803c4 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102803c8u64) }
        0x102803c8 => { vm.decode(0xd2de416du32); let d = 7i16; let s = 6i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102803ccu64) }
        0x102803cc => { vm.decode(0x23000446u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102803d0u64) }
        0x102803d0 => { vm.decode(0x4040827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 34i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102803d4u64).wrapping_add((imm * 4) as u64) } else { 0x102803d4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102803d4u64) }
        0x102803d4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102803d8u64) } }
        0x102803d8 => { vm.decode(0x4a30bb5u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102803dcu64) }
        0x102803dc => { vm.decode(0x8a27c909u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102803e0u64) }
        0x102803e0 => { vm.decode(0xca63a4f5u32); let d = 7i16; let s = 7i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102803e4u64) }
        0x102803e4 => { vm.decode(0x2a08123eu32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102803e8u64) }
        0x102803e8 => { vm.decode(0xda4a416du32); let d = 8i16; let s = 6i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102803ecu64) }
        0x102803ec => { vm.decode(0x81080d75u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102803f0u64) }
        0x102803f0 => { vm.decode(0x208034f5u32); let d = 8i16; let s = 8i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102803f4u64) }
        0x102803f4 => { vm.decode(0x2002203eu32); let d = 9i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102803f8u64) }
        0x102803f8 => { vm.decode(0x610ea835u32); let d = 7i16; let s = 7i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102803fcu64) }
        0x102803fc => { vm.decode(0x20420073u32); let d = 7i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280400u64) }
        0x10280400 => { vm.decode(0xa0204118u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10280404u64) }
        0x10280404 => { vm.decode(0xda27c909u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10280408u64) }
        0x10280408 => { vm.decode(0xca63a4f5u32); let d = 7i16; let s = 7i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028040cu64) }
        0x1028040c => { vm.decode(0x20808118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10280410u64) }
        0x10280410 => { vm.decode(0x2a08123eu32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280414u64) }
        0x10280414 => { vm.decode(0xa82bc909u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10280418u64) }
        0x10280418 => { vm.decode(0x20804118u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x1028041cu64) }
        0x1028041c => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10280420u64) }
        0x10280420 => { vm.decode(0x120220beu32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280424u64) }
        0x10280424 => { vm.decode(0x8241a4f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10280428u64) }
        0x10280428 => { vm.decode(0x3202203eu32); let d = 11i16; let s = 8i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1028042cu64) }
        0x1028042c => { vm.decode(0x82c1b4f5u32); let d = 9i16; let s = 9i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10280430u64) }
        0x10280430 => { vm.decode(0x80220beu32); let d = 12i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280434u64) }
        0x10280434 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280438u64) }
        0x10280438 => { vm.decode(0x410fa835u32); let d = 7i16; let s = 7i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028043cu64) }
        0x1028043c => { vm.decode(0xa0420073u32); let d = 7i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280440u64) }
        0x10280440 => { vm.decode(0x2a00223eu32); let d = 7i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280444u64) }
        0x10280444 => { vm.decode(0x614ea835u32); let d = 7i16; let s = 7i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280448u64) }
        0x10280448 => { vm.decode(0x204200f3u32); let d = 7i16; let s = 8i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028044cu64) }
        0x1028044c => { vm.decode(0x2000023eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280450u64) }
        0x10280450 => { vm.decode(0x40422835u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280454u64) }
        0x10280454 => { vm.decode(0xcc0511du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 44808i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10280458u64) }
        0x10280458 => { vm.decode(0x800200f3u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028045cu64) }
        0x1028045c => { vm.decode(0x8dc0511du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 44968i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10280460u64) }
        0x10280460 => { vm.decode(0xa000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280464u64) }
        0x10280464 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280468u64) }
        0x10280468 => { vm.decode(0x2002236u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028046cu64) }
        0x1028046c => { vm.decode(0xd2cac16du32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280470u64) }
        0x10280470 => { vm.decode(0x20000446u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10280474u64) }
        0x10280474 => { vm.decode(0x8c8227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 29i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280478u64).wrapping_add((imm * 4) as u64) } else { 0x10280478u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280478u64) }
        0x10280478 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028047cu64) } }
        0x1028047c => { vm.decode(0xd2cac16du32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280480u64) }
        0x10280480 => { vm.decode(0x808104f5u32); let d = 8i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10280484u64) }
        0x10280484 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10280488u64) }
        0x10280488 => { vm.decode(0x2002023eu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1028048cu64) }
        0x1028048c => { vm.decode(0x80c134f5u32); let d = 9i16; let s = 9i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10280490u64) }
        0x10280490 => { vm.decode(0x2002223eu32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280494u64) }
        0x10280494 => { vm.decode(0x30073u32); let d = 9i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280498u64) }
        0x10280498 => { vm.decode(0xa603236u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028049cu64) }
        0x1028049c => { vm.decode(0x202027eu32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 4i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102804a0u64) }
        0x102804a0 => { vm.decode(0x488134f5u32); let d = 10i16; let s = 10i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102804a4u64) }
        0x102804a4 => { vm.decode(0x20a203eu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102804a8u64) }
        0x102804a8 => { vm.decode(0x20010473u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 4i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102804acu64) }
        0x102804ac => { vm.decode(0x2020a3eu32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102804b0u64) }
        0x102804b0 => { vm.decode(0x488134f5u32); let d = 10i16; let s = 10i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102804b4u64) }
        0x102804b4 => { vm.decode(0x20a203eu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102804b8u64) }
        0x102804b8 => { vm.decode(0x20010173u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102804bcu64) }
        0x102804bc => { vm.decode(0x2020a7eu32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102804c0u64) }
        0x102804c0 => { vm.decode(0x488134f5u32); let d = 10i16; let s = 10i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102804c4u64) }
        0x102804c4 => { vm.decode(0x20a203eu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102804c8u64) }
        0x102804c8 => { vm.decode(0x20010573u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102804ccu64) }
        0x102804cc => { vm.decode(0x30833u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102804d0u64) }
        0x102804d0 => { vm.decode(0x2002207eu32); let d = 9i16; let s = 8i16; let s2 = -1i16; let imm = 4i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102804d4u64) }
        0x102804d4 => { vm.decode(0x30c33u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 4i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102804d8u64) }
        0x102804d8 => { vm.decode(0x2002283eu32); let d = 9i16; let s = 8i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102804dcu64) }
        0x102804dc => { vm.decode(0x30933u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102804e0u64) }
        0x102804e0 => { vm.decode(0x2287eu32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102804e4u64) }
        0x102804e4 => { vm.decode(0xadc0511du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 44972i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102804e8u64) }
        0x102804e8 => { vm.decode(0x10d33u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102804ecu64) }
        0x102804ec => { vm.decode(0x3000123eu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102804f0u64) }
        0x102804f0 => { vm.decode(0x3a00127eu32); let d = 7i16; let s = 5i16; let s2 = -1i16; let imm = 6i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102804f4u64) }
        0x102804f4 => { vm.decode(0x10021a3eu32); let d = 8i16; let s = 5i16; let s2 = -1i16; let imm = 10i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102804f8u64) }
        0x102804f8 => { vm.decode(0x30021a7eu32); let d = 9i16; let s = 5i16; let s2 = -1i16; let imm = 14i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102804fcu64) }
        0x102804fc => { vm.decode(0x20212feu32); let d = 10i16; let s = 5i16; let s2 = -1i16; let imm = 5i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280500u64) }
        0x10280500 => { vm.decode(0x22021afeu32); let d = 11i16; let s = 5i16; let s2 = -1i16; let imm = 13i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280504u64) }
        0x10280504 => { vm.decode(0x8021abeu32); let d = 12i16; let s = 5i16; let s2 = -1i16; let imm = 9i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280508u64) }
        0x10280508 => { vm.decode(0x280212beu32); let d = 13i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1028050cu64) }
        0x1028050c => { vm.decode(0xc0430933u32); let d = 13i16; let s = 5i16; let s2 = -1i16; let imm = 9i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280510u64) }
        0x10280510 => { vm.decode(0xc0410833u32); let d = 12i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280514u64) }
        0x10280514 => { vm.decode(0xe0030c33u32); let d = 11i16; let s = 5i16; let s2 = -1i16; let imm = 5i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280518u64) }
        0x10280518 => { vm.decode(0xe0010d33u32); let d = 10i16; let s = 5i16; let s2 = -1i16; let imm = 13i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028051cu64) }
        0x1028051c => { vm.decode(0x400308b3u32); let d = 9i16; let s = 5i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280520u64) }
        0x10280520 => { vm.decode(0x40010db3u32); let d = 8i16; let s = 5i16; let s2 = -1i16; let imm = 14i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280524u64) }
        0x10280524 => { vm.decode(0x604209b3u32); let d = 7i16; let s = 5i16; let s2 = -1i16; let imm = 10i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280528u64) }
        0x10280528 => { vm.decode(0x40020cb3u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 6i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028052cu64) }
        0x1028052c => { vm.decode(0x300012feu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 7i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280530u64) }
        0x10280530 => { vm.decode(0x3a001afeu32); let d = 7i16; let s = 5i16; let s2 = -1i16; let imm = 15i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280534u64) }
        0x10280534 => { vm.decode(0x100212beu32); let d = 8i16; let s = 5i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280538u64) }
        0x10280538 => { vm.decode(0x30021abeu32); let d = 9i16; let s = 5i16; let s2 = -1i16; let imm = 11i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1028053cu64) }
        0x1028053c => { vm.decode(0xc0030cb3u32); let d = 9i16; let s = 5i16; let s2 = -1i16; let imm = 7i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280540u64) }
        0x10280540 => { vm.decode(0xc0010db3u32); let d = 8i16; let s = 5i16; let s2 = -1i16; let imm = 15i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280544u64) }
        0x10280544 => { vm.decode(0xe04209b3u32); let d = 7i16; let s = 5i16; let s2 = -1i16; let imm = 11i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280548u64) }
        0x10280548 => { vm.decode(0xd2de416du32); let d = 7i16; let s = 6i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1028054cu64) }
        0x1028054c => { vm.decode(0x5c4588a7u32); let d = 4i16; let s = 7i16; let s2 = -1i16; let imm = 483i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280550u64).wrapping_add((imm * 4) as u64) } else { 0x10280550u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280550u64) }
        0x10280550 => { vm.decode(0xc00208b3u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280554u64) }
        0x10280554 => { vm.decode(0x1000036u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280558u64) }
        0x10280558 => { vm.decode(0x80000446u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028055cu64) }
        0x1028055c => { vm.decode(0x848a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280560u64).wrapping_add((imm * 4) as u64) } else { 0x10280560u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280560u64) }
        0x10280560 => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10280564u64) }
        0x10280564 => { vm.decode(0x804114f5u32); let d = 1i16; let s = 9i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10280568u64) }
        0x10280568 => { vm.decode(0xa88034f5u32); let d = 10i16; let s = 9i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028056cu64) }
        0x1028056c => { vm.decode(0x220a203eu32); let d = 11i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280570u64) }
        0x10280570 => { vm.decode(0x20030833u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280574u64) }
        0x10280574 => { vm.decode(0x220a20beu32); let d = 11i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280578u64) }
        0x10280578 => { vm.decode(0xa0030833u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028057cu64) }
        0x1028057c => { vm.decode(0x320a203eu32); let d = 11i16; let s = 10i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280580u64) }
        0x10280580 => { vm.decode(0x200308b3u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280584u64) }
        0x10280584 => { vm.decode(0x49000036u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280588u64) }
        0x10280588 => { vm.decode(0x120a20beu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1028058cu64) }
        0x1028058c => { vm.decode(0xa00108b3u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280590u64) }
        0x10280590 => { vm.decode(0x80000446u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10280594u64) }
        0x10280594 => { vm.decode(0x27f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -13i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10280598u64).wrapping_add((imm * 4) as u64) } else { 0x10280598u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280598u64) }
        0x10280598 => { vm.decode(0x4108a019u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028059cu64) }
        0x1028059c => { vm.decode(0x1000036u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102805a0u64) }
        0x102805a0 => { vm.decode(0x80000446u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102805a4u64) }
        0x102805a4 => { vm.decode(0x82002031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102805a8u64).wrapping_add((imm * 4) as u64) } else { 0x102805a8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102805a8u64) }
        0x102805a8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102805acu64) } }
        0x102805ac => { vm.decode(0xa0c4011du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 45700i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102805b0u64) }
        0x102805b4 => { vm.decode(0xa04034f5u32); let d = 1i16; let s = 9i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102805b8u64) }
        0x102805b8 => { vm.decode(0x32462835u32); let d = 9i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102805bcu64) }
        0x102805bc => { vm.decode(0x30833u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102805c0u64) }
        0x102805c0 => { vm.decode(0x49000036u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102805c4u64) }
        0x102805c4 => { vm.decode(0x80000446u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102805c8u64) }
        0x102805c8 => { vm.decode(0x108c0a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 94i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102805ccu64).wrapping_add((imm * 4) as u64) } else { 0x102805ccu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102805ccu64) }
        0x102805cc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102805d0u64) } }
        0x102805d0 => { vm.decode(0x1022bb5u32); let d = 9i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102805d4u64) }
        0x102805d4 => { vm.decode(0x888114f5u32); let d = 10i16; let s = 9i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102805d8u64) }
        0x102805d8 => { vm.decode(0x80a203eu32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102805dcu64) }
        0x102805dc => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102805e0u64) }
        0x102805e0 => { vm.decode(0x3022036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102805e4u64) }
        0x102805e4 => { vm.decode(0xda5e916du32); let d = 14i16; let s = 13i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102805e8u64) }
        0x102805e8 => { vm.decode(0x8090127u32); let d = 0i16; let s = 14i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102805ecu64).wrapping_add((imm * 4) as u64) } else { 0x102805ecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102805ecu64) }
        0x102805ec => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102805f0u64) } }
        0x102805f0 => { vm.decode(0x80916du32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102805f4u64) }
        0x102805f4 => { vm.decode(0x48827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102805f8u64).wrapping_add((imm * 4) as u64) } else { 0x102805f8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102805f8u64) }
        0x102805f8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102805fcu64) } }
        0x102805fc => { vm.decode(0x3042a835u32); let d = 1i16; let s = 12i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280600u64) }
        0x10280600 => { vm.decode(0xdada812du32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280604u64) }
        0x10280604 => { vm.decode(0x201ced35u32); let d = 13i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10280608u64) }
        0x10280608 => { vm.decode(0x22abb5u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028060cu64) }
        0x1028060c => { vm.decode(0xda5e822du32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 510i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280610u64) }
        0x10280610 => { vm.decode(0x280106du32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 128i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280614u64) }
        0x10280614 => { vm.decode(0x771cbca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -13i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280618u64).wrapping_add((imm * 4) as u64) } else { 0x10280618u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280618u64) }
        0x10280618 => { vm.decode(0x2a05875u32); let d = 12i16; let s = 0i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028061cu64) }
        0x1028061c => { vm.decode(0xd2ca516du32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280620u64) }
        0x10280620 => { vm.decode(0xda5e916du32); let d = 14i16; let s = 13i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280624u64) }
        0x10280624 => { vm.decode(0x3df12fb1u32); let d = 0i16; let s = 14i16; let s2 = -1i16; let imm = -14i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10280628u64).wrapping_add((imm * 4) as u64) } else { 0x10280628u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280628u64) }
        0x10280628 => { vm.decode(0x4812aa0u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s) ^ imm as u64;
        vm.st(d, v);
        Ok(0x1028062cu64) }
        0x1028062c => { vm.decode(0x280a207eu32); let d = 13i16; let s = 10i16; let s2 = -1i16; let imm = 4i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280630u64) }
        0x10280630 => { vm.decode(0xdadad12du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280634u64) }
        0x10280634 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280638u64) }
        0x10280638 => { vm.decode(0xb020236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028063cu64) }
        0x1028063c => { vm.decode(0xdade516du32); let d = 15i16; let s = 14i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280640u64) }
        0x10280640 => { vm.decode(0x80d0127u32); let d = 0i16; let s = 15i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280644u64).wrapping_add((imm * 4) as u64) } else { 0x10280644u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280644u64) }
        0x10280644 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280648u64) } }
        0x10280648 => { vm.decode(0x80516du32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1028064cu64) }
        0x1028064c => { vm.decode(0x48827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280650u64).wrapping_add((imm * 4) as u64) } else { 0x10280650u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280650u64) }
        0x10280650 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280654u64) } }
        0x10280654 => { vm.decode(0x5003a835u32); let d = 1i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280658u64) }
        0x10280658 => { vm.decode(0xda4e812du32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1028065cu64) }
        0x1028065c => { vm.decode(0x208ded35u32); let d = 14i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10280660u64) }
        0x10280660 => { vm.decode(0xa2abb5u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10280664u64) }
        0x10280664 => { vm.decode(0xdade822du32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 510i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280668u64) }
        0x10280668 => { vm.decode(0x280906du32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 128i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1028066cu64) }
        0x1028066c => { vm.decode(0x771cbca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -13i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280670u64).wrapping_add((imm * 4) as u64) } else { 0x10280670u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280670u64) }
        0x10280670 => { vm.decode(0x6a0d875u32); let d = 13i16; let s = 0i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280674u64) }
        0x10280674 => { vm.decode(0xd2cad16du32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280678u64) }
        0x10280678 => { vm.decode(0xdade516du32); let d = 15i16; let s = 14i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1028067cu64) }
        0x1028067c => { vm.decode(0x3ff12fb1u32); let d = 0i16; let s = 15i16; let s2 = -1i16; let imm = -14i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10280680u64).wrapping_add((imm * 4) as u64) } else { 0x10280680u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280680u64) }
        0x10280680 => { vm.decode(0x24812aa0u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s) ^ imm as u64;
        vm.st(d, v);
        Ok(0x10280684u64) }
        0x10280684 => { vm.decode(0xd2ca116du32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280688u64) }
        0x10280688 => { vm.decode(0x624a2835u32); let d = 11i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028068cu64) }
        0x1028068c => { vm.decode(0x280a283eu32); let d = 13i16; let s = 10i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280690u64) }
        0x10280690 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280694u64) }
        0x10280694 => { vm.decode(0xb000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280698u64) }
        0x10280698 => { vm.decode(0xdade516du32); let d = 15i16; let s = 14i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1028069cu64) }
        0x1028069c => { vm.decode(0x80d0127u32); let d = 0i16; let s = 15i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102806a0u64).wrapping_add((imm * 4) as u64) } else { 0x102806a0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102806a0u64) }
        0x102806a0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102806a4u64) } }
        0x102806a4 => { vm.decode(0x80516du32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102806a8u64) }
        0x102806a8 => { vm.decode(0x48827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102806acu64).wrapping_add((imm * 4) as u64) } else { 0x102806acu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102806acu64) }
        0x102806ac => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102806b0u64) } }
        0x102806b0 => { vm.decode(0x5003a835u32); let d = 1i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102806b4u64) }
        0x102806b4 => { vm.decode(0xda4e812du32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102806b8u64) }
        0x102806b8 => { vm.decode(0x208ded35u32); let d = 14i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102806bcu64) }
        0x102806bc => { vm.decode(0xa2abb5u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102806c0u64) }
        0x102806c0 => { vm.decode(0xdade822du32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 510i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102806c4u64) }
        0x102806c4 => { vm.decode(0x280906du32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 128i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102806c8u64) }
        0x102806c8 => { vm.decode(0x771cbca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -13i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102806ccu64).wrapping_add((imm * 4) as u64) } else { 0x102806ccu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102806ccu64) }
        0x102806cc => { vm.decode(0x6a0d875u32); let d = 13i16; let s = 0i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102806d0u64) }
        0x102806d0 => { vm.decode(0xd2cad16du32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102806d4u64) }
        0x102806d4 => { vm.decode(0xdade516du32); let d = 15i16; let s = 14i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102806d8u64) }
        0x102806d8 => { vm.decode(0x3ff12fb1u32); let d = 0i16; let s = 15i16; let s2 = -1i16; let imm = -14i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102806dcu64).wrapping_add((imm * 4) as u64) } else { 0x102806dcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102806dcu64) }
        0x102806dc => { vm.decode(0x24812aa0u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s) ^ imm as u64;
        vm.st(d, v);
        Ok(0x102806e0u64) }
        0x102806e0 => { vm.decode(0xd2ca116du32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102806e4u64) }
        0x102806e4 => { vm.decode(0x624a2835u32); let d = 11i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102806e8u64) }
        0x102806e8 => { vm.decode(0x80a287eu32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102806ecu64) }
        0x102806ec => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102806f0u64) }
        0x102806f0 => { vm.decode(0xb002036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102806f4u64) }
        0x102806f4 => { vm.decode(0xda5e916du32); let d = 14i16; let s = 13i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102806f8u64) }
        0x102806f8 => { vm.decode(0x6f913fa7u32); let d = 0i16; let s = 14i16; let s2 = -1i16; let imm = -82i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102806fcu64).wrapping_add((imm * 4) as u64) } else { 0x102806fcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102806fcu64) }
        0x102806fc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280700u64) } }
        0x10280700 => { vm.decode(0x80916du32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280704u64) }
        0x10280704 => { vm.decode(0x48827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280708u64).wrapping_add((imm * 4) as u64) } else { 0x10280708u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280708u64) }
        0x10280708 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028070cu64) } }
        0x1028070c => { vm.decode(0x1042a835u32); let d = 1i16; let s = 12i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280710u64) }
        0x10280710 => { vm.decode(0xda5a812du32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280714u64) }
        0x10280714 => { vm.decode(0x201ced35u32); let d = 13i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10280718u64) }
        0x10280718 => { vm.decode(0x22abb5u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028071cu64) }
        0x1028071c => { vm.decode(0xda5e822du32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 510i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280720u64) }
        0x10280720 => { vm.decode(0x280106du32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 128i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280724u64) }
        0x10280724 => { vm.decode(0x771cbca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -13i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280728u64).wrapping_add((imm * 4) as u64) } else { 0x10280728u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280728u64) }
        0x10280728 => { vm.decode(0x2a05875u32); let d = 12i16; let s = 0i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028072cu64) }
        0x1028072c => { vm.decode(0xd2ca516du32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280730u64) }
        0x10280730 => { vm.decode(0xda5e916du32); let d = 14i16; let s = 13i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280734u64) }
        0x10280734 => { vm.decode(0x3df12fb1u32); let d = 0i16; let s = 14i16; let s2 = -1i16; let imm = -14i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10280738u64).wrapping_add((imm * 4) as u64) } else { 0x10280738u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280738u64) }
        0x10280738 => { vm.decode(0x4812aa0u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s) ^ imm as u64;
        vm.st(d, v);
        Ok(0x1028073cu64) }
        0x1028073c => { vm.decode(0xa164051du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 45300i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10280740u64) }
        0x10280740 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280744u64) } }
        0x10280744 => { vm.decode(0x1000036u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280748u64) }
        0x10280748 => { vm.decode(0x80000446u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028074cu64) }
        0x1028074c => { vm.decode(0x82002031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10280750u64).wrapping_add((imm * 4) as u64) } else { 0x10280750u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280750u64) }
        0x10280750 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280754u64) } }
        0x10280754 => { vm.decode(0x2d44011du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 46124i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10280758u64) }
        0x1028075c => { vm.decode(0xa04034f5u32); let d = 1i16; let s = 9i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10280760u64) }
        0x10280760 => { vm.decode(0x32462835u32); let d = 9i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280764u64) }
        0x10280764 => { vm.decode(0x30c33u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 4i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280768u64) }
        0x10280768 => { vm.decode(0x49000036u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028076cu64) }
        0x1028076c => { vm.decode(0x80000446u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10280770u64) }
        0x10280770 => { vm.decode(0x108c0a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 94i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280774u64).wrapping_add((imm * 4) as u64) } else { 0x10280774u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280774u64) }
        0x10280774 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280778u64) } }
        0x10280778 => { vm.decode(0x1022bb5u32); let d = 9i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028077cu64) }
        0x1028077c => { vm.decode(0x888114f5u32); let d = 10i16; let s = 9i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10280780u64) }
        0x10280780 => { vm.decode(0x80a203eu32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280784u64) }
        0x10280784 => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280788u64) }
        0x10280788 => { vm.decode(0xb002036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028078cu64) }
        0x1028078c => { vm.decode(0xda5e916du32); let d = 14i16; let s = 13i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280790u64) }
        0x10280790 => { vm.decode(0x8090127u32); let d = 0i16; let s = 14i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280794u64).wrapping_add((imm * 4) as u64) } else { 0x10280794u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280794u64) }
        0x10280794 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280798u64) } }
        0x10280798 => { vm.decode(0x80916du32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1028079cu64) }
        0x1028079c => { vm.decode(0x48827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102807a0u64).wrapping_add((imm * 4) as u64) } else { 0x102807a0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102807a0u64) }
        0x102807a0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102807a4u64) } }
        0x102807a4 => { vm.decode(0x3042a835u32); let d = 1i16; let s = 12i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102807a8u64) }
        0x102807a8 => { vm.decode(0xdada812du32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102807acu64) }
        0x102807ac => { vm.decode(0x201ced35u32); let d = 13i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102807b0u64) }
        0x102807b0 => { vm.decode(0x22abb5u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102807b4u64) }
        0x102807b4 => { vm.decode(0xda5e822du32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 510i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102807b8u64) }
        0x102807b8 => { vm.decode(0x280106du32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 128i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102807bcu64) }
        0x102807bc => { vm.decode(0x771cbca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -13i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102807c0u64).wrapping_add((imm * 4) as u64) } else { 0x102807c0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102807c0u64) }
        0x102807c0 => { vm.decode(0x2a05875u32); let d = 12i16; let s = 0i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102807c4u64) }
        0x102807c4 => { vm.decode(0xd2ca516du32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102807c8u64) }
        0x102807c8 => { vm.decode(0xda5e916du32); let d = 14i16; let s = 13i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102807ccu64) }
        0x102807cc => { vm.decode(0x3df12fb1u32); let d = 0i16; let s = 14i16; let s2 = -1i16; let imm = -14i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102807d0u64).wrapping_add((imm * 4) as u64) } else { 0x102807d0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102807d0u64) }
        0x102807d0 => { vm.decode(0x4812aa0u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s) ^ imm as u64;
        vm.st(d, v);
        Ok(0x102807d4u64) }
        0x102807d4 => { vm.decode(0x280a207eu32); let d = 13i16; let s = 10i16; let s2 = -1i16; let imm = 4i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102807d8u64) }
        0x102807d8 => { vm.decode(0xdadad12du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102807dcu64) }
        0x102807dc => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102807e0u64) }
        0x102807e0 => { vm.decode(0x3020236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102807e4u64) }
        0x102807e4 => { vm.decode(0xdade516du32); let d = 15i16; let s = 14i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102807e8u64) }
        0x102807e8 => { vm.decode(0x80d0127u32); let d = 0i16; let s = 15i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102807ecu64).wrapping_add((imm * 4) as u64) } else { 0x102807ecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102807ecu64) }
        0x102807ec => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102807f0u64) } }
        0x102807f0 => { vm.decode(0x80516du32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102807f4u64) }
        0x102807f4 => { vm.decode(0x48827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102807f8u64).wrapping_add((imm * 4) as u64) } else { 0x102807f8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102807f8u64) }
        0x102807f8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102807fcu64) } }
        0x102807fc => { vm.decode(0x5003a835u32); let d = 1i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280800u64) }
        0x10280800 => { vm.decode(0xda4e812du32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280804u64) }
        0x10280804 => { vm.decode(0x208ded35u32); let d = 14i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10280808u64) }
        0x10280808 => { vm.decode(0xa2abb5u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028080cu64) }
        0x1028080c => { vm.decode(0xdade822du32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 510i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280810u64) }
        0x10280810 => { vm.decode(0x280906du32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 128i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280814u64) }
        0x10280814 => { vm.decode(0x771cbca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -13i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280818u64).wrapping_add((imm * 4) as u64) } else { 0x10280818u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280818u64) }
        0x10280818 => { vm.decode(0x6a0d875u32); let d = 13i16; let s = 0i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028081cu64) }
        0x1028081c => { vm.decode(0xd2cad16du32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280820u64) }
        0x10280820 => { vm.decode(0xdade516du32); let d = 15i16; let s = 14i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280824u64) }
        0x10280824 => { vm.decode(0x3ff12fb1u32); let d = 0i16; let s = 15i16; let s2 = -1i16; let imm = -14i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10280828u64).wrapping_add((imm * 4) as u64) } else { 0x10280828u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280828u64) }
        0x10280828 => { vm.decode(0x24812aa0u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s) ^ imm as u64;
        vm.st(d, v);
        Ok(0x1028082cu64) }
        0x1028082c => { vm.decode(0xd2ca116du32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280830u64) }
        0x10280830 => { vm.decode(0x624a2835u32); let d = 11i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280834u64) }
        0x10280834 => { vm.decode(0x280a283eu32); let d = 13i16; let s = 10i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280838u64) }
        0x10280838 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028083cu64) }
        0x1028083c => { vm.decode(0xb020236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280840u64) }
        0x10280840 => { vm.decode(0xdade516du32); let d = 15i16; let s = 14i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280844u64) }
        0x10280844 => { vm.decode(0x80d0127u32); let d = 0i16; let s = 15i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280848u64).wrapping_add((imm * 4) as u64) } else { 0x10280848u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280848u64) }
        0x10280848 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028084cu64) } }
        0x1028084c => { vm.decode(0x80516du32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280850u64) }
        0x10280850 => { vm.decode(0x48827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280854u64).wrapping_add((imm * 4) as u64) } else { 0x10280854u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280854u64) }
        0x10280854 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280858u64) } }
        0x10280858 => { vm.decode(0x5003a835u32); let d = 1i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028085cu64) }
        0x1028085c => { vm.decode(0xda4e812du32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280860u64) }
        0x10280860 => { vm.decode(0x208ded35u32); let d = 14i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10280864u64) }
        0x10280864 => { vm.decode(0xa2abb5u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10280868u64) }
        0x10280868 => { vm.decode(0xdade822du32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 510i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1028086cu64) }
        0x1028086c => { vm.decode(0x280906du32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 128i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280870u64) }
        0x10280870 => { vm.decode(0x771cbca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -13i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280874u64).wrapping_add((imm * 4) as u64) } else { 0x10280874u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280874u64) }
        0x10280874 => { vm.decode(0x6a0d875u32); let d = 13i16; let s = 0i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280878u64) }
        0x10280878 => { vm.decode(0xd2cad16du32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1028087cu64) }
        0x1028087c => { vm.decode(0xdade516du32); let d = 15i16; let s = 14i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280880u64) }
        0x10280880 => { vm.decode(0x3ff12fb1u32); let d = 0i16; let s = 15i16; let s2 = -1i16; let imm = -14i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10280884u64).wrapping_add((imm * 4) as u64) } else { 0x10280884u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280884u64) }
        0x10280884 => { vm.decode(0x24812aa0u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s) ^ imm as u64;
        vm.st(d, v);
        Ok(0x10280888u64) }
        0x10280888 => { vm.decode(0xd2ca116du32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1028088cu64) }
        0x1028088c => { vm.decode(0x624a2835u32); let d = 11i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280890u64) }
        0x10280890 => { vm.decode(0x80a287eu32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280894u64) }
        0x10280894 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280898u64) }
        0x10280898 => { vm.decode(0xb002036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028089cu64) }
        0x1028089c => { vm.decode(0xda5e916du32); let d = 14i16; let s = 13i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102808a0u64) }
        0x102808a0 => { vm.decode(0x6f913fa7u32); let d = 0i16; let s = 14i16; let s2 = -1i16; let imm = -82i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102808a4u64).wrapping_add((imm * 4) as u64) } else { 0x102808a4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102808a4u64) }
        0x102808a4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102808a8u64) } }
        0x102808a8 => { vm.decode(0x80916du32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102808acu64) }
        0x102808ac => { vm.decode(0x48827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102808b0u64).wrapping_add((imm * 4) as u64) } else { 0x102808b0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102808b0u64) }
        0x102808b0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102808b4u64) } }
        0x102808b4 => { vm.decode(0x1042a835u32); let d = 1i16; let s = 12i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102808b8u64) }
        0x102808b8 => { vm.decode(0xda5a812du32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102808bcu64) }
        0x102808bc => { vm.decode(0x201ced35u32); let d = 13i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102808c0u64) }
        0x102808c0 => { vm.decode(0x22abb5u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102808c4u64) }
        0x102808c4 => { vm.decode(0xda5e822du32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 510i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102808c8u64) }
        0x102808c8 => { vm.decode(0x280106du32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 128i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102808ccu64) }
        0x102808cc => { vm.decode(0x771cbca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -13i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102808d0u64).wrapping_add((imm * 4) as u64) } else { 0x102808d0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102808d0u64) }
        0x102808d0 => { vm.decode(0x2a05875u32); let d = 12i16; let s = 0i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102808d4u64) }
        0x102808d4 => { vm.decode(0xd2ca516du32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102808d8u64) }
        0x102808d8 => { vm.decode(0xda5e916du32); let d = 14i16; let s = 13i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102808dcu64) }
        0x102808dc => { vm.decode(0x3df12fb1u32); let d = 0i16; let s = 14i16; let s2 = -1i16; let imm = -14i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102808e0u64).wrapping_add((imm * 4) as u64) } else { 0x102808e0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102808e0u64) }
        0x102808e0 => { vm.decode(0x4812aa0u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s) ^ imm as u64;
        vm.st(d, v);
        Ok(0x102808e4u64) }
        0x102808e4 => { vm.decode(0xa8c4051du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 45724i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102808e8u64) }
        0x102808e8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102808ecu64) } }
        0x102808ec => { vm.decode(0x1000036u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102808f0u64) }
        0x102808f0 => { vm.decode(0x80000446u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102808f4u64) }
        0x102808f4 => { vm.decode(0x82002031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102808f8u64).wrapping_add((imm * 4) as u64) } else { 0x102808f8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102808f8u64) }
        0x102808f8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102808fcu64) } }
        0x102808fc => { vm.decode(0xa464151du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 46548i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10280900u64) }
        0x10280904 => { vm.decode(0xa04034f5u32); let d = 1i16; let s = 9i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10280908u64) }
        0x10280908 => { vm.decode(0x32462835u32); let d = 9i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028090cu64) }
        0x1028090c => { vm.decode(0x30933u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280910u64) }
        0x10280910 => { vm.decode(0x49000036u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280914u64) }
        0x10280914 => { vm.decode(0x80000446u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10280918u64) }
        0x10280918 => { vm.decode(0x108c0a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 94i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028091cu64).wrapping_add((imm * 4) as u64) } else { 0x1028091cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028091cu64) }
        0x1028091c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280920u64) } }
        0x10280920 => { vm.decode(0x1022bb5u32); let d = 9i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10280924u64) }
        0x10280924 => { vm.decode(0x888114f5u32); let d = 10i16; let s = 9i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10280928u64) }
        0x10280928 => { vm.decode(0x80a203eu32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1028092cu64) }
        0x1028092c => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280930u64) }
        0x10280930 => { vm.decode(0xb002036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280934u64) }
        0x10280934 => { vm.decode(0xda5e916du32); let d = 14i16; let s = 13i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280938u64) }
        0x10280938 => { vm.decode(0x8090127u32); let d = 0i16; let s = 14i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028093cu64).wrapping_add((imm * 4) as u64) } else { 0x1028093cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028093cu64) }
        0x1028093c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280940u64) } }
        0x10280940 => { vm.decode(0x80916du32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280944u64) }
        0x10280944 => { vm.decode(0x48827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280948u64).wrapping_add((imm * 4) as u64) } else { 0x10280948u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280948u64) }
        0x10280948 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028094cu64) } }
        0x1028094c => { vm.decode(0x3042a835u32); let d = 1i16; let s = 12i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280950u64) }
        0x10280950 => { vm.decode(0xdada812du32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280954u64) }
        0x10280954 => { vm.decode(0x201ced35u32); let d = 13i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10280958u64) }
        0x10280958 => { vm.decode(0x22abb5u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028095cu64) }
        0x1028095c => { vm.decode(0xda5e822du32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 510i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280960u64) }
        0x10280960 => { vm.decode(0x280106du32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 128i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280964u64) }
        0x10280964 => { vm.decode(0x771cbca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -13i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280968u64).wrapping_add((imm * 4) as u64) } else { 0x10280968u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280968u64) }
        0x10280968 => { vm.decode(0x2a05875u32); let d = 12i16; let s = 0i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028096cu64) }
        0x1028096c => { vm.decode(0xd2ca516du32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280970u64) }
        0x10280970 => { vm.decode(0xda5e916du32); let d = 14i16; let s = 13i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280974u64) }
        0x10280974 => { vm.decode(0x3df12fb1u32); let d = 0i16; let s = 14i16; let s2 = -1i16; let imm = -14i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10280978u64).wrapping_add((imm * 4) as u64) } else { 0x10280978u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280978u64) }
        0x10280978 => { vm.decode(0x4812aa0u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s) ^ imm as u64;
        vm.st(d, v);
        Ok(0x1028097cu64) }
        0x1028097c => { vm.decode(0x280a207eu32); let d = 13i16; let s = 10i16; let s2 = -1i16; let imm = 4i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280980u64) }
        0x10280980 => { vm.decode(0xdadad12du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280984u64) }
        0x10280984 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280988u64) }
        0x10280988 => { vm.decode(0xb000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028098cu64) }
        0x1028098c => { vm.decode(0xdade516du32); let d = 15i16; let s = 14i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280990u64) }
        0x10280990 => { vm.decode(0x80d0127u32); let d = 0i16; let s = 15i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280994u64).wrapping_add((imm * 4) as u64) } else { 0x10280994u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280994u64) }
        0x10280994 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280998u64) } }
        0x10280998 => { vm.decode(0x80516du32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1028099cu64) }
        0x1028099c => { vm.decode(0x48827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102809a0u64).wrapping_add((imm * 4) as u64) } else { 0x102809a0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102809a0u64) }
        0x102809a0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102809a4u64) } }
        0x102809a4 => { vm.decode(0x5003a835u32); let d = 1i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102809a8u64) }
        0x102809a8 => { vm.decode(0xda4e812du32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102809acu64) }
        0x102809ac => { vm.decode(0x208ded35u32); let d = 14i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102809b0u64) }
        0x102809b0 => { vm.decode(0xa2abb5u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102809b4u64) }
        0x102809b4 => { vm.decode(0xdade822du32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 510i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102809b8u64) }
        0x102809b8 => { vm.decode(0x280906du32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 128i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102809bcu64) }
        0x102809bc => { vm.decode(0x771cbca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -13i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102809c0u64).wrapping_add((imm * 4) as u64) } else { 0x102809c0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102809c0u64) }
        0x102809c0 => { vm.decode(0x6a0d875u32); let d = 13i16; let s = 0i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102809c4u64) }
        0x102809c4 => { vm.decode(0xd2cad16du32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102809c8u64) }
        0x102809c8 => { vm.decode(0xdade516du32); let d = 15i16; let s = 14i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102809ccu64) }
        0x102809cc => { vm.decode(0x3ff12fb1u32); let d = 0i16; let s = 15i16; let s2 = -1i16; let imm = -14i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102809d0u64).wrapping_add((imm * 4) as u64) } else { 0x102809d0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102809d0u64) }
        0x102809d0 => { vm.decode(0x24812aa0u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s) ^ imm as u64;
        vm.st(d, v);
        Ok(0x102809d4u64) }
        0x102809d4 => { vm.decode(0xd2ca116du32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102809d8u64) }
        0x102809d8 => { vm.decode(0x624a2835u32); let d = 11i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102809dcu64) }
        0x102809dc => { vm.decode(0x280a283eu32); let d = 13i16; let s = 10i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102809e0u64) }
        0x102809e0 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102809e4u64) }
        0x102809e4 => { vm.decode(0x3020236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102809e8u64) }
        0x102809e8 => { vm.decode(0xdade516du32); let d = 15i16; let s = 14i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102809ecu64) }
        0x102809ec => { vm.decode(0x80d0127u32); let d = 0i16; let s = 15i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102809f0u64).wrapping_add((imm * 4) as u64) } else { 0x102809f0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102809f0u64) }
        0x102809f0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102809f4u64) } }
        0x102809f4 => { vm.decode(0x80516du32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102809f8u64) }
        0x102809f8 => { vm.decode(0x48827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102809fcu64).wrapping_add((imm * 4) as u64) } else { 0x102809fcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102809fcu64) }
        0x102809fc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280a00u64) } }
        0x10280a00 => { vm.decode(0x5003a835u32); let d = 1i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280a04u64) }
        0x10280a04 => { vm.decode(0xda4e812du32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280a08u64) }
        0x10280a08 => { vm.decode(0x208ded35u32); let d = 14i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10280a0cu64) }
        0x10280a0c => { vm.decode(0xa2abb5u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10280a10u64) }
        0x10280a10 => { vm.decode(0xdade822du32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 510i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280a14u64) }
        0x10280a14 => { vm.decode(0x280906du32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 128i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280a18u64) }
        0x10280a18 => { vm.decode(0x771cbca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -13i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280a1cu64).wrapping_add((imm * 4) as u64) } else { 0x10280a1cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280a1cu64) }
        0x10280a1c => { vm.decode(0x6a0d875u32); let d = 13i16; let s = 0i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280a20u64) }
        0x10280a20 => { vm.decode(0xd2cad16du32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280a24u64) }
        0x10280a24 => { vm.decode(0xdade516du32); let d = 15i16; let s = 14i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280a28u64) }
        0x10280a28 => { vm.decode(0x3ff12fb1u32); let d = 0i16; let s = 15i16; let s2 = -1i16; let imm = -14i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10280a2cu64).wrapping_add((imm * 4) as u64) } else { 0x10280a2cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280a2cu64) }
        0x10280a2c => { vm.decode(0x24812aa0u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s) ^ imm as u64;
        vm.st(d, v);
        Ok(0x10280a30u64) }
        0x10280a30 => { vm.decode(0xd2ca116du32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280a34u64) }
        0x10280a34 => { vm.decode(0x624a2835u32); let d = 11i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280a38u64) }
        0x10280a38 => { vm.decode(0x80a287eu32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280a3cu64) }
        0x10280a3c => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280a40u64) }
        0x10280a40 => { vm.decode(0xb022036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280a44u64) }
        0x10280a44 => { vm.decode(0xda5e916du32); let d = 14i16; let s = 13i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280a48u64) }
        0x10280a48 => { vm.decode(0x6f913fa7u32); let d = 0i16; let s = 14i16; let s2 = -1i16; let imm = -82i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280a4cu64).wrapping_add((imm * 4) as u64) } else { 0x10280a4cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280a4cu64) }
        0x10280a4c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280a50u64) } }
        0x10280a50 => { vm.decode(0x80916du32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280a54u64) }
        0x10280a54 => { vm.decode(0x48827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280a58u64).wrapping_add((imm * 4) as u64) } else { 0x10280a58u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280a58u64) }
        0x10280a58 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280a5cu64) } }
        0x10280a5c => { vm.decode(0x1042a835u32); let d = 1i16; let s = 12i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280a60u64) }
        0x10280a60 => { vm.decode(0xda5a812du32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280a64u64) }
        0x10280a64 => { vm.decode(0x201ced35u32); let d = 13i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10280a68u64) }
        0x10280a68 => { vm.decode(0x22abb5u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10280a6cu64) }
        0x10280a6c => { vm.decode(0xda5e822du32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 510i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280a70u64) }
        0x10280a70 => { vm.decode(0x280106du32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 128i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280a74u64) }
        0x10280a74 => { vm.decode(0x771cbca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -13i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280a78u64).wrapping_add((imm * 4) as u64) } else { 0x10280a78u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280a78u64) }
        0x10280a78 => { vm.decode(0x2a05875u32); let d = 12i16; let s = 0i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280a7cu64) }
        0x10280a7c => { vm.decode(0xd2ca516du32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280a80u64) }
        0x10280a80 => { vm.decode(0xda5e916du32); let d = 14i16; let s = 13i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280a84u64) }
        0x10280a84 => { vm.decode(0x3df12fb1u32); let d = 0i16; let s = 14i16; let s2 = -1i16; let imm = -14i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10280a88u64).wrapping_add((imm * 4) as u64) } else { 0x10280a88u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280a88u64) }
        0x10280a88 => { vm.decode(0x4812aa0u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s) ^ imm as u64;
        vm.st(d, v);
        Ok(0x10280a8cu64) }
        0x10280a8c => { vm.decode(0x2464011du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 46148i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10280a90u64) }
        0x10280a90 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280a94u64) } }
        0x10280a94 => { vm.decode(0x1000036u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280a98u64) }
        0x10280a98 => { vm.decode(0x80000446u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10280a9cu64) }
        0x10280a9c => { vm.decode(0x82002031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10280aa0u64).wrapping_add((imm * 4) as u64) } else { 0x10280aa0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280aa0u64) }
        0x10280aa0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280aa4u64) } }
        0x10280aa4 => { vm.decode(0x2de4151du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 46972i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10280aa8u64) }
        0x10280aac => { vm.decode(0xa04034f5u32); let d = 1i16; let s = 9i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10280ab0u64) }
        0x10280ab0 => { vm.decode(0x32462835u32); let d = 9i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280ab4u64) }
        0x10280ab4 => { vm.decode(0x30d33u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280ab8u64) }
        0x10280ab8 => { vm.decode(0x49000036u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280abcu64) }
        0x10280abc => { vm.decode(0x80000446u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10280ac0u64) }
        0x10280ac0 => { vm.decode(0x108c0a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 94i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280ac4u64).wrapping_add((imm * 4) as u64) } else { 0x10280ac4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280ac4u64) }
        0x10280ac4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280ac8u64) } }
        0x10280ac8 => { vm.decode(0x1022bb5u32); let d = 9i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10280accu64) }
        0x10280acc => { vm.decode(0x888114f5u32); let d = 10i16; let s = 9i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10280ad0u64) }
        0x10280ad0 => { vm.decode(0x80a203eu32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280ad4u64) }
        0x10280ad4 => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280ad8u64) }
        0x10280ad8 => { vm.decode(0xb022036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280adcu64) }
        0x10280adc => { vm.decode(0xda5e916du32); let d = 14i16; let s = 13i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280ae0u64) }
        0x10280ae0 => { vm.decode(0x8090127u32); let d = 0i16; let s = 14i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280ae4u64).wrapping_add((imm * 4) as u64) } else { 0x10280ae4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280ae4u64) }
        0x10280ae4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280ae8u64) } }
        0x10280ae8 => { vm.decode(0x80916du32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280aecu64) }
        0x10280aec => { vm.decode(0x48827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280af0u64).wrapping_add((imm * 4) as u64) } else { 0x10280af0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280af0u64) }
        0x10280af0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280af4u64) } }
        0x10280af4 => { vm.decode(0x3042a835u32); let d = 1i16; let s = 12i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280af8u64) }
        0x10280af8 => { vm.decode(0xdada812du32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280afcu64) }
        0x10280afc => { vm.decode(0x201ced35u32); let d = 13i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10280b00u64) }
        0x10280b00 => { vm.decode(0x22abb5u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10280b04u64) }
        0x10280b04 => { vm.decode(0xda5e822du32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 510i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280b08u64) }
        0x10280b08 => { vm.decode(0x280106du32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 128i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280b0cu64) }
        0x10280b0c => { vm.decode(0x771cbca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -13i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280b10u64).wrapping_add((imm * 4) as u64) } else { 0x10280b10u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280b10u64) }
        0x10280b10 => { vm.decode(0x2a05875u32); let d = 12i16; let s = 0i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280b14u64) }
        0x10280b14 => { vm.decode(0xd2ca516du32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280b18u64) }
        0x10280b18 => { vm.decode(0xda5e916du32); let d = 14i16; let s = 13i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280b1cu64) }
        0x10280b1c => { vm.decode(0x3df12fb1u32); let d = 0i16; let s = 14i16; let s2 = -1i16; let imm = -14i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10280b20u64).wrapping_add((imm * 4) as u64) } else { 0x10280b20u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280b20u64) }
        0x10280b20 => { vm.decode(0x4812aa0u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s) ^ imm as u64;
        vm.st(d, v);
        Ok(0x10280b24u64) }
        0x10280b24 => { vm.decode(0x280a207eu32); let d = 13i16; let s = 10i16; let s2 = -1i16; let imm = 4i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280b28u64) }
        0x10280b28 => { vm.decode(0xdadad12du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280b2cu64) }
        0x10280b2c => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280b30u64) }
        0x10280b30 => { vm.decode(0xb000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280b34u64) }
        0x10280b34 => { vm.decode(0xdade516du32); let d = 15i16; let s = 14i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280b38u64) }
        0x10280b38 => { vm.decode(0x80d0127u32); let d = 0i16; let s = 15i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280b3cu64).wrapping_add((imm * 4) as u64) } else { 0x10280b3cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280b3cu64) }
        0x10280b3c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280b40u64) } }
        0x10280b40 => { vm.decode(0x80516du32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280b44u64) }
        0x10280b44 => { vm.decode(0x48827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280b48u64).wrapping_add((imm * 4) as u64) } else { 0x10280b48u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280b48u64) }
        0x10280b48 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280b4cu64) } }
        0x10280b4c => { vm.decode(0x5003a835u32); let d = 1i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280b50u64) }
        0x10280b50 => { vm.decode(0xda4e812du32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280b54u64) }
        0x10280b54 => { vm.decode(0x208ded35u32); let d = 14i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10280b58u64) }
        0x10280b58 => { vm.decode(0xa2abb5u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10280b5cu64) }
        0x10280b5c => { vm.decode(0xdade822du32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 510i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280b60u64) }
        0x10280b60 => { vm.decode(0x280906du32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 128i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280b64u64) }
        0x10280b64 => { vm.decode(0x771cbca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -13i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280b68u64).wrapping_add((imm * 4) as u64) } else { 0x10280b68u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280b68u64) }
        0x10280b68 => { vm.decode(0x6a0d875u32); let d = 13i16; let s = 0i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280b6cu64) }
        0x10280b6c => { vm.decode(0xd2cad16du32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280b70u64) }
        0x10280b70 => { vm.decode(0xdade516du32); let d = 15i16; let s = 14i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280b74u64) }
        0x10280b74 => { vm.decode(0x3ff12fb1u32); let d = 0i16; let s = 15i16; let s2 = -1i16; let imm = -14i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10280b78u64).wrapping_add((imm * 4) as u64) } else { 0x10280b78u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280b78u64) }
        0x10280b78 => { vm.decode(0x24812aa0u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s) ^ imm as u64;
        vm.st(d, v);
        Ok(0x10280b7cu64) }
        0x10280b7c => { vm.decode(0xd2ca116du32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280b80u64) }
        0x10280b80 => { vm.decode(0x624a2835u32); let d = 11i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280b84u64) }
        0x10280b84 => { vm.decode(0x280a283eu32); let d = 13i16; let s = 10i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280b88u64) }
        0x10280b88 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280b8cu64) }
        0x10280b8c => { vm.decode(0xb000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280b90u64) }
        0x10280b90 => { vm.decode(0xdade516du32); let d = 15i16; let s = 14i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280b94u64) }
        0x10280b94 => { vm.decode(0x80d0127u32); let d = 0i16; let s = 15i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280b98u64).wrapping_add((imm * 4) as u64) } else { 0x10280b98u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280b98u64) }
        0x10280b98 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280b9cu64) } }
        0x10280b9c => { vm.decode(0x80516du32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280ba0u64) }
        0x10280ba0 => { vm.decode(0x48827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280ba4u64).wrapping_add((imm * 4) as u64) } else { 0x10280ba4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280ba4u64) }
        0x10280ba4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280ba8u64) } }
        0x10280ba8 => { vm.decode(0x5003a835u32); let d = 1i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280bacu64) }
        0x10280bac => { vm.decode(0xda4e812du32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280bb0u64) }
        0x10280bb0 => { vm.decode(0x208ded35u32); let d = 14i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10280bb4u64) }
        0x10280bb4 => { vm.decode(0xa2abb5u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10280bb8u64) }
        0x10280bb8 => { vm.decode(0xdade822du32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 510i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280bbcu64) }
        0x10280bbc => { vm.decode(0x280906du32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 128i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280bc0u64) }
        0x10280bc0 => { vm.decode(0x771cbca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -13i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280bc4u64).wrapping_add((imm * 4) as u64) } else { 0x10280bc4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280bc4u64) }
        0x10280bc4 => { vm.decode(0x6a0d875u32); let d = 13i16; let s = 0i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280bc8u64) }
        0x10280bc8 => { vm.decode(0xd2cad16du32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280bccu64) }
        0x10280bcc => { vm.decode(0xdade516du32); let d = 15i16; let s = 14i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280bd0u64) }
        0x10280bd0 => { vm.decode(0x3ff12fb1u32); let d = 0i16; let s = 15i16; let s2 = -1i16; let imm = -14i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10280bd4u64).wrapping_add((imm * 4) as u64) } else { 0x10280bd4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280bd4u64) }
        0x10280bd4 => { vm.decode(0x24812aa0u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s) ^ imm as u64;
        vm.st(d, v);
        Ok(0x10280bd8u64) }
        0x10280bd8 => { vm.decode(0xd2ca116du32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280bdcu64) }
        0x10280bdc => { vm.decode(0x624a2835u32); let d = 11i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280be0u64) }
        0x10280be0 => { vm.decode(0x80a287eu32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280be4u64) }
        0x10280be4 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280be8u64) }
        0x10280be8 => { vm.decode(0x3022036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280becu64) }
        0x10280bec => { vm.decode(0xda5e916du32); let d = 14i16; let s = 13i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280bf0u64) }
        0x10280bf0 => { vm.decode(0x6f913fa7u32); let d = 0i16; let s = 14i16; let s2 = -1i16; let imm = -82i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280bf4u64).wrapping_add((imm * 4) as u64) } else { 0x10280bf4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280bf4u64) }
        0x10280bf4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280bf8u64) } }
        0x10280bf8 => { vm.decode(0x80916du32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280bfcu64) }
        0x10280bfc => { vm.decode(0x48827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280c00u64).wrapping_add((imm * 4) as u64) } else { 0x10280c00u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280c00u64) }
        0x10280c00 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280c04u64) } }
        0x10280c04 => { vm.decode(0x1042a835u32); let d = 1i16; let s = 12i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280c08u64) }
        0x10280c08 => { vm.decode(0xda5a812du32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280c0cu64) }
        0x10280c0c => { vm.decode(0x201ced35u32); let d = 13i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10280c10u64) }
        0x10280c10 => { vm.decode(0x22abb5u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10280c14u64) }
        0x10280c14 => { vm.decode(0xda5e822du32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 510i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280c18u64) }
        0x10280c18 => { vm.decode(0x280106du32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 128i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280c1cu64) }
        0x10280c1c => { vm.decode(0x771cbca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -13i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280c20u64).wrapping_add((imm * 4) as u64) } else { 0x10280c20u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280c20u64) }
        0x10280c20 => { vm.decode(0x2a05875u32); let d = 12i16; let s = 0i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280c24u64) }
        0x10280c24 => { vm.decode(0xd2ca516du32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280c28u64) }
        0x10280c28 => { vm.decode(0xda5e916du32); let d = 14i16; let s = 13i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280c2cu64) }
        0x10280c2c => { vm.decode(0x3df12fb1u32); let d = 0i16; let s = 14i16; let s2 = -1i16; let imm = -14i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10280c30u64).wrapping_add((imm * 4) as u64) } else { 0x10280c30u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280c30u64) }
        0x10280c30 => { vm.decode(0x4812aa0u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s) ^ imm as u64;
        vm.st(d, v);
        Ok(0x10280c34u64) }
        0x10280c34 => { vm.decode(0xad64111du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 46572i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10280c38u64) }
        0x10280c38 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280c3cu64) } }
        0x10280c3c => { vm.decode(0x1000036u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280c40u64) }
        0x10280c40 => { vm.decode(0xdaca112du32); let d = 9i16; let s = 8i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280c44u64) }
        0x10280c44 => { vm.decode(0xa0000446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10280c48u64) }
        0x10280c48 => { vm.decode(0x23943c27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -506i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280c4cu64).wrapping_add((imm * 4) as u64) } else { 0x10280c4cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280c4cu64) }
        0x10280c4c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280c50u64) } }
        0x10280c50 => { vm.decode(0xd2ca112du32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280c54u64) }
        0x10280c54 => { vm.decode(0x80402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10280c58u64) }
        0x10280c58 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10280c5cu64) }
        0x10280c5c => { vm.decode(0x202023eu32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280c60u64) }
        0x10280c60 => { vm.decode(0x5822bb5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10280c64u64) }
        0x10280c64 => { vm.decode(0x1f30bb5u32); let d = 11i16; let s = 7i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10280c68u64) }
        0x10280c68 => { vm.decode(0x55214fb5u32); let d = 9i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10280c6cu64) }
        0x10280c6c => { vm.decode(0xaa2bc909u32); let d = 11i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10280c70u64) }
        0x10280c70 => { vm.decode(0xcac1b4f5u32); let d = 11i16; let s = 11i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10280c74u64) }
        0x10280c74 => { vm.decode(0x220a223eu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280c78u64) }
        0x10280c78 => { vm.decode(0x524e0835u32); let d = 10i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280c7cu64) }
        0x10280c7c => { vm.decode(0xa0828118u32); let d = 11i16; let s = 9i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10280c80u64) }
        0x10280c80 => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10280c84u64) }
        0x10280c84 => { vm.decode(0xa2c118u32); let d = 12i16; let s = 9i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10280c88u64) }
        0x10280c88 => { vm.decode(0xe82d4909u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10280c8cu64) }
        0x10280c8c => { vm.decode(0x20010833u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280c90u64) }
        0x10280c90 => { vm.decode(0x120202beu32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280c94u64) }
        0x10280c94 => { vm.decode(0x2a3b4f5u32); let d = 12i16; let s = 12i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10280c98u64) }
        0x10280c98 => { vm.decode(0x3802023eu32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280c9cu64) }
        0x10280c9c => { vm.decode(0xcac1b4f5u32); let d = 11i16; let s = 11i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10280ca0u64) }
        0x10280ca0 => { vm.decode(0xa0202beu32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280ca4u64) }
        0x10280ca4 => { vm.decode(0x2002c118u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10280ca8u64) }
        0x10280ca8 => { vm.decode(0xa82bc909u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10280cacu64) }
        0x10280cac => { vm.decode(0x82c1b4f5u32); let d = 9i16; let s = 9i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10280cb0u64) }
        0x10280cb0 => { vm.decode(0x49000036u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280cb4u64) }
        0x10280cb4 => { vm.decode(0x2002223eu32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280cb8u64) }
        0x10280cb8 => { vm.decode(0x52432835u32); let d = 9i16; let s = 9i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280cbcu64) }
        0x10280cbc => { vm.decode(0x80030833u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280cc0u64) }
        0x10280cc0 => { vm.decode(0x200a223eu32); let d = 9i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280cc4u64) }
        0x10280cc4 => { vm.decode(0x72032835u32); let d = 9i16; let s = 9i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280cc8u64) }
        0x10280cc8 => { vm.decode(0x308b3u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280cccu64) }
        0x10280ccc => { vm.decode(0x2002303eu32); let d = 9i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280cd0u64) }
        0x10280cd0 => { vm.decode(0x52422835u32); let d = 9i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280cd4u64) }
        0x10280cd4 => { vm.decode(0x84c4111du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 46976i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10280cd8u64) }
        0x10280cd8 => { vm.decode(0x800308b3u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280cdcu64) }
        0x10280cdc => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280ce0u64) }
        0x10280ce0 => { vm.decode(0xd2de416du32); let d = 7i16; let s = 6i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280ce4u64) }
        0x10280ce4 => { vm.decode(0x23000446u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10280ce8u64) }
        0x10280ce8 => { vm.decode(0x4048827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 35i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280cecu64).wrapping_add((imm * 4) as u64) } else { 0x10280cecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280cecu64) }
        0x10280cec => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280cf0u64) } }
        0x10280cf0 => { vm.decode(0x4a30bb5u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10280cf4u64) }
        0x10280cf4 => { vm.decode(0x6003236u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280cf8u64) }
        0x10280cf8 => { vm.decode(0xda27c909u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10280cfcu64) }
        0x10280cfc => { vm.decode(0xca63a4f5u32); let d = 7i16; let s = 7i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10280d00u64) }
        0x10280d00 => { vm.decode(0x2a08123eu32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280d04u64) }
        0x10280d04 => { vm.decode(0xda4a416du32); let d = 8i16; let s = 6i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280d08u64) }
        0x10280d08 => { vm.decode(0x81080d75u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10280d0cu64) }
        0x10280d0c => { vm.decode(0x208034f5u32); let d = 8i16; let s = 8i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10280d10u64) }
        0x10280d10 => { vm.decode(0x2002203eu32); let d = 9i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280d14u64) }
        0x10280d14 => { vm.decode(0x610ea835u32); let d = 7i16; let s = 7i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280d18u64) }
        0x10280d18 => { vm.decode(0x20420073u32); let d = 7i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280d1cu64) }
        0x10280d1c => { vm.decode(0xe003236u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 33i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280d20u64) }
        0x10280d20 => { vm.decode(0xda27c909u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10280d24u64) }
        0x10280d24 => { vm.decode(0xca63a4f5u32); let d = 7i16; let s = 7i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10280d28u64) }
        0x10280d28 => { vm.decode(0x5023036u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 34i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280d2cu64) }
        0x10280d2c => { vm.decode(0x2a08123eu32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280d30u64) }
        0x10280d30 => { vm.decode(0xa82bc909u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10280d34u64) }
        0x10280d34 => { vm.decode(0xc023036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 35i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280d38u64) }
        0x10280d38 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10280d3cu64) }
        0x10280d3c => { vm.decode(0x120220beu32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280d40u64) }
        0x10280d40 => { vm.decode(0x8241a4f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10280d44u64) }
        0x10280d44 => { vm.decode(0x3202203eu32); let d = 11i16; let s = 8i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280d48u64) }
        0x10280d48 => { vm.decode(0x82c1b4f5u32); let d = 9i16; let s = 9i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10280d4cu64) }
        0x10280d4c => { vm.decode(0x80220beu32); let d = 12i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280d50u64) }
        0x10280d50 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280d54u64) }
        0x10280d54 => { vm.decode(0x410fa835u32); let d = 7i16; let s = 7i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280d58u64) }
        0x10280d58 => { vm.decode(0xa0420073u32); let d = 7i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280d5cu64) }
        0x10280d5c => { vm.decode(0x2a00223eu32); let d = 7i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280d60u64) }
        0x10280d60 => { vm.decode(0x614ea835u32); let d = 7i16; let s = 7i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280d64u64) }
        0x10280d64 => { vm.decode(0x204200f3u32); let d = 7i16; let s = 8i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280d68u64) }
        0x10280d68 => { vm.decode(0x2000023eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280d6cu64) }
        0x10280d6c => { vm.decode(0x40422835u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280d70u64) }
        0x10280d70 => { vm.decode(0x144411du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 47136i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10280d74u64) }
        0x10280d74 => { vm.decode(0x800200f3u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280d78u64) }
        0x10280d78 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280d7cu64) }
        0x10280d7c => { vm.decode(0xd2de416du32); let d = 7i16; let s = 6i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280d80u64) }
        0x10280d80 => { vm.decode(0x23002406u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10280d84u64) }
        0x10280d84 => { vm.decode(0x751cbe27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -645i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280d88u64).wrapping_add((imm * 4) as u64) } else { 0x10280d88u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280d88u64) }
        0x10280d88 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280d8cu64) } }
        0x10280d8c => { vm.decode(0x603436u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 16i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280d90u64) }
        0x10280d90 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10280d94u64) }
        0x10280d94 => { vm.decode(0x8241a4f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10280d98u64) }
        0x10280d98 => { vm.decode(0x2000023eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280d9cu64) }
        0x10280d9c => { vm.decode(0xd2de416du32); let d = 7i16; let s = 6i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10280da0u64) }
        0x10280da0 => { vm.decode(0xe86224f5u32); let d = 7i16; let s = 7i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10280da4u64) }
        0x10280da4 => { vm.decode(0xa123eu32); let d = 8i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10280da8u64) }
        0x10280da8 => { vm.decode(0x40022835u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280dacu64) }
        0x10280dac => { vm.decode(0x40220833u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10280db0u64) }
        0x10280db0 => { vm.decode(0xa944451du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 47292i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10280db4u64) }
        0x10280db4 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280db8u64) }
        0x10280db8 => { vm.decode(0x408a536fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280dbcu64) }
        0x10280dbc => { vm.decode(0x22a0c02fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280dc0u64) }
        0x10280dc0 => { vm.decode(0x409019u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10280dc4u64) }
        0x10280dc4 => { vm.decode(0x1d181530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 4120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280dc8u64) }
        0x10280dc8 => { vm.decode(0xd105530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 4112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280dccu64) }
        0x10280dcc => { vm.decode(0xd081530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280dd0u64) }
        0x10280dd0 => { vm.decode(0x1d001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280dd4u64) }
        0x10280dd4 => { vm.decode(0x882e3efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280dd8u64) }
        0x10280dd8 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280ddcu64) }
        0x10280ddc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10280de0u64 + 4);
        vm.wr(37, 0x10280de0u64 + 4);
        Ok(0x10280de0u64) }
        0x10280de0 => { vm.decode(0x62098219u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 4096i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10280de4u64) }
        0x10280de4 => { vm.decode(0x22a4402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280de8u64) }
        0x10280de8 => { vm.decode(0x441019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10280decu64) }
        0x10280dec => { vm.decode(0x292995b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280df0u64) }
        0x10280df0 => { vm.decode(0x392194b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280df4u64) }
        0x10280df4 => { vm.decode(0x286c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280df8u64) }
        0x10280df8 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280dfcu64) }
        0x10280dfc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10280e00u64 + 4);
        vm.wr(37, 0x10280e00u64 + 4);
        Ok(0x10280e00u64) }
        0x10280e00 => { vm.decode(0x623b8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3296i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10280e04u64) }
        0x10280e04 => { vm.decode(0x66798559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3952i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10280e08u64) }
        0x10280e08 => { vm.decode(0x2aa0606fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280e0cu64) }
        0x10280e0c => { vm.decode(0xb7d1962u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 3976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10280e10u64) }
        0x10280e10 => { vm.decode(0xab019d30u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 3968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280e14u64) }
        0x10280e14 => { vm.decode(0x6ac4536fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 3312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280e18u64) }
        0x10280e18 => { vm.decode(0xbb391cb0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 3960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280e1cu64) }
        0x10280e1c => { vm.decode(0x3b311cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280e20u64) }
        0x10280e20 => { vm.decode(0x4a86c3efu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280e24u64) }
        0x10280e24 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280e28u64) }
        0x10280e28 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10280e2cu64 + 4);
        vm.wr(37, 0x10280e2cu64 + 4);
        Ok(0x10280e2cu64) }
        0x10280e2c => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280e30u64) }
        0x10280e30 => { vm.decode(0x8020227u32); let d = 0i16; let s = 18i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280e34u64).wrapping_add((imm * 4) as u64) } else { 0x10280e34u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280e34u64) }
        0x10280e34 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280e38u64) } }
        0x10280e38 => { vm.decode(0x20a0e06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280e3cu64) }
        0x10280e3c => { vm.decode(0xab311530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 3664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280e40u64) }
        0x10280e40 => { vm.decode(0x3b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280e44u64) }
        0x10280e44 => { vm.decode(0xa82c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280e48u64) }
        0x10280e48 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280e4cu64) }
        0x10280e4c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10280e50u64 + 4);
        vm.wr(37, 0x10280e50u64 + 4);
        Ok(0x10280e50u64) }
        0x10280e50 => { vm.decode(0x662d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3656i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10280e54u64) }
        0x10280e54 => { vm.decode(0x2b089430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280e58u64) }
        0x10280e58 => { vm.decode(0x2b009430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280e5cu64) }
        0x10280e5c => { vm.decode(0x20a4c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280e60u64) }
        0x10280e60 => { vm.decode(0xbb199430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280e64u64) }
        0x10280e64 => { vm.decode(0xab119430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280e68u64) }
        0x10280e68 => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280e6cu64) }
        0x10280e6c => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10280e70u64) }
        0x10280e70 => { vm.decode(0x6088532fu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 1024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280e74u64) }
        0x10280e74 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280e78u64) }
        0x10280e78 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10280e7cu64 + 4);
        vm.wr(37, 0x10280e7cu64 + 4);
        Ok(0x10280e7cu64) }
        0x10280e7c => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280e80u64) }
        0x10280e80 => { vm.decode(0xa019u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10280e84u64) }
        0x10280e84 => { vm.decode(0xbb009430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 1664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280e88u64) }
        0x10280e88 => { vm.decode(0x2ac402fu32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280e8cu64) }
        0x10280e8c => { vm.decode(0x392994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280e90u64) }
        0x10280e90 => { vm.decode(0x292195b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280e94u64) }
        0x10280e94 => { vm.decode(0x286c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280e98u64) }
        0x10280e98 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280e9cu64) }
        0x10280e9c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10280ea0u64 + 4);
        vm.wr(37, 0x10280ea0u64 + 4);
        Ok(0x10280ea0u64) }
        0x10280ea0 => { vm.decode(0x623b8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3296i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10280ea4u64) }
        0x10280ea4 => { vm.decode(0xacc536fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280ea8u64) }
        0x10280ea8 => { vm.decode(0x4000022u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10280eacu64) }
        0x10280eac => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280eb0u64).wrapping_add((imm * 4) as u64) } else { 0x10280eb0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280eb0u64) }
        0x10280eb0 => { vm.decode(0x2b089530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280eb4u64) }
        0x10280eb4 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10280eb8u64) }
        0x10280eb8 => { vm.decode(0x8003036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280ebcu64) }
        0x10280ebc => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10280ec0u64) }
        0x10280ec0 => { vm.decode(0x20a4e06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280ec4u64) }
        0x10280ec4 => { vm.decode(0xab019430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280ec8u64) }
        0x10280ec8 => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280eccu64) }
        0x10280ecc => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280ed0u64) }
        0x10280ed0 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280ed4u64) }
        0x10280ed4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10280ed8u64 + 4);
        vm.wr(37, 0x10280ed8u64 + 4);
        Ok(0x10280ed8u64) }
        0x10280ed8 => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10280edcu64) }
        0x10280edc => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280ee0u64) }
        0x10280ee0 => { vm.decode(0xa8ce3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280ee4u64) }
        0x10280ee4 => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280ee8u64) }
        0x10280ee8 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280eecu64) }
        0x10280eec => { vm.decode(0x286532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280ef0u64) }
        0x10280ef0 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280ef4u64) }
        0x10280ef4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10280ef8u64 + 4);
        vm.wr(37, 0x10280ef8u64 + 4);
        Ok(0x10280ef8u64) }
        0x10280ef8 => { vm.decode(0x62198219u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10280efcu64) }
        0x10280efc => { vm.decode(0x28a4406fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280f00u64) }
        0x10280f00 => { vm.decode(0x88cc3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280f04u64) }
        0x10280f04 => { vm.decode(0xb1195b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280f08u64) }
        0x10280f08 => { vm.decode(0x288c3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280f0cu64) }
        0x10280f0c => { vm.decode(0xb0995b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280f10u64) }
        0x10280f10 => { vm.decode(0x1b0194b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280f14u64) }
        0x10280f14 => { vm.decode(0x286e3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280f18u64) }
        0x10280f18 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280f1cu64) }
        0x10280f1c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10280f20u64 + 4);
        vm.wr(37, 0x10280f20u64 + 4);
        Ok(0x10280f20u64) }
        0x10280f20 => { vm.decode(0x661b8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10280f24u64) }
        0x10280f24 => { vm.decode(0x20a4c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280f28u64) }
        0x10280f28 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10280f2cu64) }
        0x10280f2c => { vm.decode(0x88853afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280f30u64) }
        0x10280f30 => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280f34u64) }
        0x10280f34 => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280f38u64) }
        0x10280f38 => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280f3cu64) }
        0x10280f3c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280f40u64) }
        0x10280f40 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10280f44u64 + 4);
        vm.wr(37, 0x10280f44u64 + 4);
        Ok(0x10280f44u64) }
        0x10280f44 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280f48u64) }
        0x10280f48 => { vm.decode(0x88873afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280f4cu64) }
        0x10280f4c => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280f50u64).wrapping_add((imm * 4) as u64) } else { 0x10280f50u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280f50u64) }
        0x10280f50 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280f54u64) } }
        0x10280f54 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10280f58u64) }
        0x10280f58 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10280f5cu64) }
        0x10280f5c => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10280f60u64) }
        0x10280f60 => { vm.decode(0x88873afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280f64u64) }
        0x10280f64 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10280f68u64) }
        0x10280f68 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x10280f6cu64).wrapping_add((imm * 4) as u64) } else { 0x10280f6cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280f6cu64) }
        0x10280f6c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280f70u64) } }
        0x10280f70 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10280f74u64).wrapping_add((imm * 4) as u64) } else { 0x10280f74u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280f74u64) }
        0x10280fc8 => { vm.decode(0x20a4e06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280fccu64) }
        0x10280fcc => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280fd0u64) }
        0x10280fd0 => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10280fd4u64) }
        0x10280fd4 => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280fd8u64) }
        0x10280fd8 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10280fdcu64) }
        0x10280fdc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10280fe0u64 + 4);
        vm.wr(37, 0x10280fe0u64 + 4);
        Ok(0x10280fe0u64) }
        0x10280fe0 => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10280fe4u64) }
        0x10280fe4 => { vm.decode(0x2a8073afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10280fe8u64) }
        0x10280fe8 => { vm.decode(0x22e12cf1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -4714i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10280fecu64).wrapping_add((imm * 4) as u64) } else { 0x10280fecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10280fecu64) }
        0x10280fec => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10280ff0u64) } }
        0x10280ff0 => { vm.decode(0x8140041du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 32944i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10280ff4u64) }
        0x10281000 => { vm.decode(0x7f5bce59u32); let d = 29i16; let s = 29i16; let s2 = -1i16; let imm = -1104i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10281004u64) }
        0x10281004 => { vm.decode(0xb9285570u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281008u64) }
        0x10281008 => { vm.decode(0xa9205570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028100cu64) }
        0x1028100c => { vm.decode(0xb91855b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281010u64) }
        0x10281010 => { vm.decode(0xa91055b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281014u64) }
        0x10281014 => { vm.decode(0xb90854b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281018u64) }
        0x10281018 => { vm.decode(0xa90054b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028101cu64) }
        0x1028101c => { vm.decode(0xb9181530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 1048i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281020u64) }
        0x10281020 => { vm.decode(0xa9101530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281024u64) }
        0x10281024 => { vm.decode(0xb9081430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 1032i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281028u64) }
        0x10281028 => { vm.decode(0xa9001430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 1024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028102cu64) }
        0x1028102c => { vm.decode(0xc005875u32); let d = 16i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281030u64) }
        0x10281030 => { vm.decode(0x1315b68eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -9437184i64;
        vm.st(d, imm as u64);
        Ok(0x10281034u64) }
        0x10281034 => { vm.decode(0x225d0dd8u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 47032i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10281038u64) }
        0x10281038 => { vm.decode(0x1f15b68eu32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = -9437184i64;
        vm.st(d, imm as u64);
        Ok(0x1028103cu64) }
        0x1028103c => { vm.decode(0x4082012fu32); let d = 20i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281040u64) }
        0x10281040 => { vm.decode(0x804724f5u32); let d = 21i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10281044u64) }
        0x10281044 => { vm.decode(0x2084212fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281048u64) }
        0x10281048 => { vm.decode(0x19381430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028104cu64) }
        0x1028104c => { vm.decode(0x2284012fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281050u64) }
        0x10281050 => { vm.decode(0x190894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281054u64) }
        0x10281054 => { vm.decode(0x2284212fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281058u64) }
        0x10281058 => { vm.decode(0x19009430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028105cu64) }
        0x1028105c => { vm.decode(0x2080016fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281060u64) }
        0x10281060 => { vm.decode(0x193814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281064u64) }
        0x10281064 => { vm.decode(0x2080216fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281068u64) }
        0x10281068 => { vm.decode(0x193014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028106cu64) }
        0x1028106c => { vm.decode(0x20a4202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281070u64) }
        0x10281070 => { vm.decode(0x19301430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281074u64) }
        0x10281074 => { vm.decode(0x22a4002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281078u64) }
        0x10281078 => { vm.decode(0x19281430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028107cu64) }
        0x1028107c => { vm.decode(0x22a4202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281080u64) }
        0x10281080 => { vm.decode(0x191814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281084u64) }
        0x10281084 => { vm.decode(0x20a0006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281088u64) }
        0x10281088 => { vm.decode(0x191014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028108cu64) }
        0x1028108c => { vm.decode(0x20a0206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281090u64) }
        0x10281090 => { vm.decode(0x191094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281094u64) }
        0x10281094 => { vm.decode(0x22a0006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281098u64) }
        0x10281098 => { vm.decode(0x190014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028109cu64) }
        0x1028109c => { vm.decode(0x22a0206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102810a0u64) }
        0x102810a0 => { vm.decode(0x19181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102810a4u64) }
        0x102810a4 => { vm.decode(0x20a4006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102810a8u64) }
        0x102810a8 => { vm.decode(0x19389430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102810acu64) }
        0x102810ac => { vm.decode(0x20a4206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102810b0u64) }
        0x102810b0 => { vm.decode(0x19309430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102810b4u64) }
        0x102810b4 => { vm.decode(0x22a4006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102810b8u64) }
        0x102810b8 => { vm.decode(0x19289430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102810bcu64) }
        0x102810bc => { vm.decode(0x22a4206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102810c0u64) }
        0x102810c0 => { vm.decode(0x192814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102810c4u64) }
        0x102810c4 => { vm.decode(0x28a0002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102810c8u64) }
        0x102810c8 => { vm.decode(0x192014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102810ccu64) }
        0x102810cc => { vm.decode(0x20a4002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102810d0u64) }
        0x102810d0 => { vm.decode(0x191894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102810d4u64) }
        0x102810d4 => { vm.decode(0x22a0202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102810d8u64) }
        0x102810d8 => { vm.decode(0x19209430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102810dcu64) }
        0x102810dc => { vm.decode(0x22a0002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102810e0u64) }
        0x102810e0 => { vm.decode(0x190094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102810e4u64) }
        0x102810e4 => { vm.decode(0x20a0202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102810e8u64) }
        0x102810e8 => { vm.decode(0x19081430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102810ecu64) }
        0x102810ec => { vm.decode(0x408a002fu32); let d = 22i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102810f0u64) }
        0x102810f0 => { vm.decode(0x20a0002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102810f4u64) }
        0x102810f4 => { vm.decode(0x8c012fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102810f8u64) }
        0x102810f8 => { vm.decode(0x9189530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102810fcu64) }
        0x102810fc => { vm.decode(0x6280212fu32); let d = 17i16; let s = 5i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281100u64) }
        0x10281100 => { vm.decode(0x628a012fu32); let d = 23i16; let s = 5i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281104u64) }
        0x10281104 => { vm.decode(0x88212fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281108u64) }
        0x10281108 => { vm.decode(0x2088412fu32); let d = 3i16; let s = 21i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028110cu64) }
        0x1028110c => { vm.decode(0x2102019u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = 36i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10281110u64) }
        0x10281110 => { vm.decode(0xb38dcb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281114u64) }
        0x10281114 => { vm.decode(0x8c359dd8u32); let d = 30i16; let s = 7i16; let s2 = -1i16; let imm = 46880i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10281118u64) }
        0x10281118 => { vm.decode(0x4b2014f5u32); let d = 2i16; let s = 2i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028111cu64) }
        0x1028111c => { vm.decode(0xb309db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281120u64) }
        0x10281120 => { vm.decode(0x640d3019u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 524i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10281124u64) }
        0x10281124 => { vm.decode(0xb289db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281128u64) }
        0x10281128 => { vm.decode(0x1b209db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028112cu64) }
        0x1028112c => { vm.decode(0x696414f5u32); let d = 19i16; let s = 30i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10281130u64) }
        0x10281130 => { vm.decode(0x741b1159u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10281134u64) }
        0x10281134 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281138u64) }
        0x10281138 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028113cu64) }
        0x1028113c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10281140u64 + 4);
        vm.wr(37, 0x10281140u64 + 4);
        Ok(0x10281140u64) }
        0x10281140 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281144u64) }
        0x10281144 => { vm.decode(0x636134f5u32); let d = 1i16; let s = 30i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10281148u64) }
        0x10281148 => { vm.decode(0x88412fu32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028114cu64) }
        0x1028114c => { vm.decode(0x1b309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281150u64) }
        0x10281150 => { vm.decode(0x601ba159u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 484i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10281154u64) }
        0x10281154 => { vm.decode(0x1b289cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281158u64) }
        0x10281158 => { vm.decode(0xb209db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028115cu64) }
        0x1028115c => { vm.decode(0x12149019u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 40i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10281160u64) }
        0x10281160 => { vm.decode(0x9b38ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281164u64) }
        0x10281164 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281168u64) }
        0x10281168 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028116cu64) }
        0x1028116c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10281170u64 + 4);
        vm.wr(37, 0x10281170u64 + 4);
        Ok(0x10281170u64) }
        0x10281170 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281174u64) }
        0x10281174 => { vm.decode(0x80148019u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 42i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10281178u64) }
        0x10281178 => { vm.decode(0x88412fu32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028117cu64) }
        0x1028117c => { vm.decode(0x1b389cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281180u64) }
        0x10281180 => { vm.decode(0x616114f5u32); let d = 1i16; let s = 17i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10281184u64) }
        0x10281184 => { vm.decode(0x1b309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281188u64) }
        0x10281188 => { vm.decode(0x605f8059u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028118cu64) }
        0x1028118c => { vm.decode(0x1b289cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281190u64) }
        0x10281190 => { vm.decode(0xb209db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281194u64) }
        0x10281194 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281198u64) }
        0x10281198 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028119cu64) }
        0x1028119c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102811a0u64 + 4);
        vm.wr(37, 0x102811a0u64 + 4);
        Ok(0x102811a0u64) }
        0x102811a0 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102811a4u64) }
        0x102811a4 => { vm.decode(0x2080412fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102811a8u64) }
        0x102811a8 => { vm.decode(0x9b38ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102811acu64) }
        0x102811ac => { vm.decode(0xa88632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102811b0u64) }
        0x102811b0 => { vm.decode(0x4b2014f5u32); let d = 2i16; let s = 2i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102811b4u64) }
        0x102811b4 => { vm.decode(0xb309db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102811b8u64) }
        0x102811b8 => { vm.decode(0x604b1059u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102811bcu64) }
        0x102811bc => { vm.decode(0xb289db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102811c0u64) }
        0x102811c0 => { vm.decode(0x1b209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102811c4u64) }
        0x102811c4 => { vm.decode(0x99109530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102811c8u64) }
        0x102811c8 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102811ccu64) }
        0x102811cc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102811d0u64) }
        0x102811d0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102811d4u64 + 4);
        vm.wr(37, 0x102811d4u64 + 4);
        Ok(0x102811d4u64) }
        0x102811d4 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102811d8u64) }
        0x102811d8 => { vm.decode(0x1955b68eu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = -9437184i64;
        vm.st(d, imm as u64);
        Ok(0x102811dcu64) }
        0x102811dc => { vm.decode(0x6b140cd8u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 46928i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102811e0u64) }
        0x102811e0 => { vm.decode(0x804124f5u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102811e4u64) }
        0x102811e4 => { vm.decode(0xac602fu32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102811e8u64) }
        0x102811e8 => { vm.decode(0x9189530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102811ecu64) }
        0x102811ec => { vm.decode(0xac402fu32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102811f0u64) }
        0x102811f0 => { vm.decode(0x9089530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102811f4u64) }
        0x102811f4 => { vm.decode(0x2a8602fu32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102811f8u64) }
        0x102811f8 => { vm.decode(0x90815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102811fcu64) }
        0x102811fc => { vm.decode(0xa8602fu32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281200u64) }
        0x10281200 => { vm.decode(0x9201530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281204u64) }
        0x10281204 => { vm.decode(0xa8402fu32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281208u64) }
        0x10281208 => { vm.decode(0x9101530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028120cu64) }
        0x1028120c => { vm.decode(0x62a0402fu32); let d = 17i16; let s = 22i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281210u64) }
        0x10281210 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281214u64) }
        0x10281214 => { vm.decode(0x1b309c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281218u64) }
        0x10281218 => { vm.decode(0x2080632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028121cu64) }
        0x1028121c => { vm.decode(0x612214f5u32); let d = 4i16; let s = 30i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10281220u64) }
        0x10281220 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281224u64) }
        0x10281224 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10281228u64 + 4);
        vm.wr(37, 0x10281228u64 + 4);
        Ok(0x10281228u64) }
        0x10281228 => { vm.decode(0x664b8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028122cu64) }
        0x1028122c => { vm.decode(0xd98ea11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10281230u64) }
        0x10281230 => { vm.decode(0x3351462u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 396i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10281234u64) }
        0x10281234 => { vm.decode(0x69140cd8u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 46912i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10281238u64) }
        0x10281238 => { vm.decode(0x1955b68eu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = -9437184i64;
        vm.st(d, imm as u64);
        Ok(0x1028123cu64) }
        0x1028123c => { vm.decode(0x804124f5u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10281240u64) }
        0x10281240 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281244u64) }
        0x10281244 => { vm.decode(0x9b289c30u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281248u64) }
        0x10281248 => { vm.decode(0x724d1159u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028124cu64) }
        0x1028124c => { vm.decode(0x8b20dd30u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281250u64) }
        0x10281250 => { vm.decode(0x1b189cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281254u64) }
        0x10281254 => { vm.decode(0x2884432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281258u64) }
        0x10281258 => { vm.decode(0x612214f5u32); let d = 4i16; let s = 30i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028125cu64) }
        0x1028125c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281260u64) }
        0x10281260 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10281264u64 + 4);
        vm.wr(37, 0x10281264u64 + 4);
        Ok(0x10281264u64) }
        0x10281264 => { vm.decode(0x665f8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10281268u64) }
        0x10281268 => { vm.decode(0x69550cd8u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 46952i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x1028126cu64) }
        0x1028126c => { vm.decode(0x804124f5u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10281270u64) }
        0x10281270 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281274u64) }
        0x10281274 => { vm.decode(0x288e36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281278u64) }
        0x10281278 => { vm.decode(0xb109db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028127cu64) }
        0x1028127c => { vm.decode(0x60591159u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10281280u64) }
        0x10281280 => { vm.decode(0x90095b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281284u64) }
        0x10281284 => { vm.decode(0xb089db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281288u64) }
        0x10281288 => { vm.decode(0x1b009cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028128cu64) }
        0x1028128c => { vm.decode(0x2880436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281290u64) }
        0x10281290 => { vm.decode(0x612214f5u32); let d = 4i16; let s = 30i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10281294u64) }
        0x10281294 => { vm.decode(0x661b8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10281298u64) }
        0x10281298 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028129cu64) }
        0x1028129c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102812a0u64 + 4);
        vm.wr(37, 0x102812a0u64 + 4);
        Ok(0x102812a0u64) }
        0x102812a0 => { vm.decode(0x920d430u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102812a4u64) }
        0x102812a4 => { vm.decode(0x685d0cd8u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 47016i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102812a8u64) }
        0x102812a8 => { vm.decode(0x804124f5u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102812acu64) }
        0x102812ac => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102812b0u64) }
        0x102812b0 => { vm.decode(0x288e36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102812b4u64) }
        0x102812b4 => { vm.decode(0xb189d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102812b8u64) }
        0x102812b8 => { vm.decode(0x76599019u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102812bcu64) }
        0x102812bc => { vm.decode(0x9b10dd30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102812c0u64) }
        0x102812c0 => { vm.decode(0x1b089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102812c4u64) }
        0x102812c4 => { vm.decode(0x2a84632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102812c8u64) }
        0x102812c8 => { vm.decode(0x612214f5u32); let d = 4i16; let s = 30i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102812ccu64) }
        0x102812cc => { vm.decode(0x660f8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102812d0u64) }
        0x102812d0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102812d4u64) }
        0x102812d4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102812d8u64 + 4);
        vm.wr(37, 0x102812d8u64 + 4);
        Ok(0x102812d8u64) }
        0x102812d8 => { vm.decode(0x918d4b0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102812dcu64) }
        0x102812dc => { vm.decode(0x2280632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102812e0u64) }
        0x102812e0 => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102812e4u64) }
        0x102812e4 => { vm.decode(0x19181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102812e8u64) }
        0x102812e8 => { vm.decode(0x2084432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102812ecu64) }
        0x102812ec => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102812f0u64) }
        0x102812f0 => { vm.decode(0x190014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102812f4u64) }
        0x102812f4 => { vm.decode(0x2a84432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102812f8u64) }
        0x102812f8 => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102812fcu64) }
        0x102812fc => { vm.decode(0x191094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281300u64) }
        0x10281300 => { vm.decode(0x2284432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281304u64) }
        0x10281304 => { vm.decode(0x696414f5u32); let d = 19i16; let s = 30i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10281308u64) }
        0x10281308 => { vm.decode(0x2284632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028130cu64) }
        0x1028130c => { vm.decode(0x616414f5u32); let d = 17i16; let s = 30i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10281310u64) }
        0x10281310 => { vm.decode(0x2080636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281314u64) }
        0x10281314 => { vm.decode(0x692414f5u32); let d = 18i16; let s = 30i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10281318u64) }
        0x10281318 => { vm.decode(0x2280436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028131cu64) }
        0x1028131c => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10281320u64) }
        0x10281320 => { vm.decode(0x19081430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281324u64) }
        0x10281324 => { vm.decode(0x2280636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281328u64) }
        0x10281328 => { vm.decode(0x616614f5u32); let d = 21i16; let s = 30i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028132cu64) }
        0x1028132c => { vm.decode(0x8cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281330u64) }
        0x10281330 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10281334u64).wrapping_add((imm * 4) as u64) } else { 0x10281334u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10281334u64) }
        0x10281334 => { vm.decode(0x412734f5u32); let d = 20i16; let s = 30i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10281338u64) }
        0x10281338 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028133cu64) }
        0x1028133c => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10281340u64) }
        0x10281340 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10281344u64) }
        0x10281344 => { vm.decode(0x8cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281348u64) }
        0x10281348 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028134cu64) }
        0x1028134c => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x10281350u64).wrapping_add((imm * 4) as u64) } else { 0x10281350u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10281350u64) }
        0x10281350 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10281354u64) } }
        0x10281354 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10281358u64).wrapping_add((imm * 4) as u64) } else { 0x10281358u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10281358u64) }
        0x102813ac => { vm.decode(0x2880636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102813b0u64) }
        0x102813b0 => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102813b4u64) }
        0x102813b4 => { vm.decode(0x19381430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102813b8u64) }
        0x102813b8 => { vm.decode(0x2a80436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102813bcu64) }
        0x102813bc => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102813c0u64) }
        0x102813c0 => { vm.decode(0x19301430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102813c4u64) }
        0x102813c4 => { vm.decode(0x2a80636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102813c8u64) }
        0x102813c8 => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102813ccu64) }
        0x102813cc => { vm.decode(0x19281430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102813d0u64) }
        0x102813d0 => { vm.decode(0x2a80402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102813d4u64) }
        0x102813d4 => { vm.decode(0x9b205db0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102813d8u64) }
        0x102813d8 => { vm.decode(0x1b381c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102813dcu64) }
        0x102813dc => { vm.decode(0x664d8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102813e0u64) }
        0x102813e0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102813e4u64) }
        0x102813e4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102813e8u64 + 4);
        vm.wr(37, 0x102813e8u64 + 4);
        Ok(0x102813e8u64) }
        0x102813e8 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102813ecu64) }
        0x102813ec => { vm.decode(0x661d8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102813f0u64) }
        0x102813f0 => { vm.decode(0x2080602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102813f4u64) }
        0x102813f4 => { vm.decode(0x8b305db0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102813f8u64) }
        0x102813f8 => { vm.decode(0x1b281cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102813fcu64) }
        0x102813fc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281400u64) }
        0x10281400 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10281404u64 + 4);
        vm.wr(37, 0x10281404u64 + 4);
        Ok(0x10281404u64) }
        0x10281404 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281408u64) }
        0x10281408 => { vm.decode(0x2280432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028140cu64) }
        0x1028140c => { vm.decode(0x820bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10281410u64) }
        0x10281410 => { vm.decode(0x661d8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10281414u64) }
        0x10281414 => { vm.decode(0x8c606fu32); let d = 2i16; let s = 20i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281418u64) }
        0x10281418 => { vm.decode(0x208c632fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028141cu64) }
        0x1028141c => { vm.decode(0x2228012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281420u64) }
        0x10281420 => { vm.decode(0x882c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281424u64) }
        0x10281424 => { vm.decode(0xb305c30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281428u64) }
        0x10281428 => { vm.decode(0x1b281d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028142cu64) }
        0x1028142c => { vm.decode(0x430d1062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10281430u64) }
        0x10281430 => { vm.decode(0x9b185cb0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281434u64) }
        0x10281434 => { vm.decode(0x8b105db0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281438u64) }
        0x10281438 => { vm.decode(0xb081db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028143cu64) }
        0x1028143c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281440u64) }
        0x10281440 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10281444u64 + 4);
        vm.wr(37, 0x10281444u64 + 4);
        Ok(0x10281444u64) }
        0x10281444 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281448u64) }
        0x10281448 => { vm.decode(0x72498059u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 272i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028144cu64) }
        0x1028144c => { vm.decode(0x600f3059u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 396i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10281450u64) }
        0x10281450 => { vm.decode(0xa236u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10281454u64) }
        0x10281454 => { vm.decode(0x2280406fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281458u64) }
        0x10281458 => { vm.decode(0x230d5062u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028145cu64) }
        0x1028145c => { vm.decode(0x90815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281460u64) }
        0x10281460 => { vm.decode(0xb181d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281464u64) }
        0x10281464 => { vm.decode(0x9b105c30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281468u64) }
        0x10281468 => { vm.decode(0x1b081c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028146cu64) }
        0x1028146c => { vm.decode(0xa86432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281470u64) }
        0x10281470 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281474u64) }
        0x10281474 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10281478u64 + 4);
        vm.wr(37, 0x10281478u64 + 4);
        Ok(0x10281478u64) }
        0x10281478 => { vm.decode(0x660d8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028147cu64) }
        0x1028147c => { vm.decode(0x701d9059u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10281480u64) }
        0x10281480 => { vm.decode(0x2280602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281484u64) }
        0x10281484 => { vm.decode(0x88436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281488u64) }
        0x10281488 => { vm.decode(0xb001d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028148cu64) }
        0x1028148c => { vm.decode(0x9b38d4b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 760i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281490u64) }
        0x10281490 => { vm.decode(0x9b3095b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281494u64) }
        0x10281494 => { vm.decode(0x1b2894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281498u64) }
        0x10281498 => { vm.decode(0x86432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028149cu64) }
        0x1028149c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102814a0u64) }
        0x102814a0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102814a4u64 + 4);
        vm.wr(37, 0x102814a4u64 + 4);
        Ok(0x102814a4u64) }
        0x102814a4 => { vm.decode(0x661f8119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102814a8u64) }
        0x102814a8 => { vm.decode(0x605f9119u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102814acu64) }
        0x102814ac => { vm.decode(0x8002036u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102814b0u64) }
        0x102814b0 => { vm.decode(0x288402fu32); let d = 2i16; let s = 20i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102814b4u64) }
        0x102814b4 => { vm.decode(0x5b020a73u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102814b8u64) }
        0x102814b8 => { vm.decode(0x99389530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102814bcu64) }
        0x102814bc => { vm.decode(0x9b389530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102814c0u64) }
        0x102814c0 => { vm.decode(0x19289530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102814c4u64) }
        0x102814c4 => { vm.decode(0x1b309530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102814c8u64) }
        0x102814c8 => { vm.decode(0xb289530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102814ccu64) }
        0x102814cc => { vm.decode(0x282632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102814d0u64) }
        0x102814d0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102814d4u64) }
        0x102814d4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102814d8u64 + 4);
        vm.wr(37, 0x102814d8u64 + 4);
        Ok(0x102814d8u64) }
        0x102814d8 => { vm.decode(0x660f8119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102814dcu64) }
        0x102814dc => { vm.decode(0x60099159u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102814e0u64) }
        0x102814e0 => { vm.decode(0x2080606fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102814e4u64) }
        0x102814e4 => { vm.decode(0xa8c636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102814e8u64) }
        0x102814e8 => { vm.decode(0xb109db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102814ecu64) }
        0x102814ec => { vm.decode(0x19309530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102814f0u64) }
        0x102814f0 => { vm.decode(0x1b089db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102814f4u64) }
        0x102814f4 => { vm.decode(0x1b009cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102814f8u64) }
        0x102814f8 => { vm.decode(0x882436fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102814fcu64) }
        0x102814fc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281500u64) }
        0x10281500 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10281504u64 + 4);
        vm.wr(37, 0x10281504u64 + 4);
        Ok(0x10281504u64) }
        0x10281504 => { vm.decode(0x661b8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10281508u64) }
        0x10281508 => { vm.decode(0x2880602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028150cu64) }
        0x1028150c => { vm.decode(0xa8c636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281510u64) }
        0x10281510 => { vm.decode(0xb189d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281514u64) }
        0x10281514 => { vm.decode(0x9b10dd30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281518u64) }
        0x10281518 => { vm.decode(0x1b089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028151cu64) }
        0x1028151c => { vm.decode(0xa86632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281520u64) }
        0x10281520 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281524u64) }
        0x10281524 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10281528u64 + 4);
        vm.wr(37, 0x10281528u64 + 4);
        Ok(0x10281528u64) }
        0x10281528 => { vm.decode(0x660f8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028152cu64) }
        0x1028152c => { vm.decode(0x88c32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281530u64) }
        0x10281530 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10281534u64).wrapping_add((imm * 4) as u64) } else { 0x10281534u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10281534u64) }
        0x10281534 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10281538u64) } }
        0x10281538 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028153cu64) }
        0x1028153c => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10281540u64) }
        0x10281540 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10281544u64) }
        0x10281544 => { vm.decode(0x88c32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281548u64) }
        0x10281548 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028154cu64) }
        0x1028154c => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x10281550u64).wrapping_add((imm * 4) as u64) } else { 0x10281550u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10281550u64) }
        0x10281550 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10281554u64) } }
        0x10281554 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10281558u64).wrapping_add((imm * 4) as u64) } else { 0x10281558u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10281558u64) }
        0x10281558 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028155cu64) } }
        0x1028155c => { vm.decode(0x2880402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281560u64) }
        0x10281560 => { vm.decode(0xb009d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281564u64) }
        0x10281564 => { vm.decode(0x1b381cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281568u64) }
        0x10281568 => { vm.decode(0x82632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028156cu64) }
        0x1028156c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281570u64) }
        0x10281570 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10281574u64 + 4);
        vm.wr(37, 0x10281574u64 + 4);
        Ok(0x10281574u64) }
        0x10281574 => { vm.decode(0x665d8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10281578u64) }
        0x10281578 => { vm.decode(0x9001c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028157cu64) }
        0x1028157c => { vm.decode(0xa8c636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281580u64) }
        0x10281580 => { vm.decode(0x8000a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10281584u64).wrapping_add((imm * 4) as u64) } else { 0x10281584u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10281584u64) }
        0x10281584 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10281588u64) } }
        0x10281588 => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028158cu64) }
        0x1028158c => { vm.decode(0x2000212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281590u64) }
        0x10281590 => { vm.decode(0xb301db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281594u64) }
        0x10281594 => { vm.decode(0x1b281cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281598u64) }
        0x10281598 => { vm.decode(0x661d8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028159cu64) }
        0x1028159c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102815a0u64) }
        0x102815a0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102815a4u64 + 4);
        vm.wr(37, 0x102815a4u64 + 4);
        Ok(0x102815a4u64) }
        0x102815a4 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102815a8u64) }
        0x102815a8 => { vm.decode(0x93894b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102815acu64) }
        0x102815ac => { vm.decode(0x2084436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102815b0u64) }
        0x102815b0 => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102815b4u64) }
        0x102815b4 => { vm.decode(0x192014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102815b8u64) }
        0x102815b8 => { vm.decode(0x2084636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102815bcu64) }
        0x102815bc => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102815c0u64) }
        0x102815c0 => { vm.decode(0x192814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102815c4u64) }
        0x102815c4 => { vm.decode(0x2284436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102815c8u64) }
        0x102815c8 => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102815ccu64) }
        0x102815cc => { vm.decode(0x193014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102815d0u64) }
        0x102815d0 => { vm.decode(0x2284636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102815d4u64) }
        0x102815d4 => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102815d8u64) }
        0x102815d8 => { vm.decode(0x19201430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102815dcu64) }
        0x102815dc => { vm.decode(0x2880432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102815e0u64) }
        0x102815e0 => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102815e4u64) }
        0x102815e4 => { vm.decode(0x191814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102815e8u64) }
        0x102815e8 => { vm.decode(0x2884632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102815ecu64) }
        0x102815ec => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102815f0u64) }
        0x102815f0 => { vm.decode(0x191014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102815f4u64) }
        0x102815f4 => { vm.decode(0x2a80402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102815f8u64) }
        0x102815f8 => { vm.decode(0x9b205db0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102815fcu64) }
        0x102815fc => { vm.decode(0x1b381c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281600u64) }
        0x10281600 => { vm.decode(0x664d8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10281604u64) }
        0x10281604 => { vm.decode(0x993814b0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281608u64) }
        0x10281608 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028160cu64) }
        0x1028160c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10281610u64 + 4);
        vm.wr(37, 0x10281610u64 + 4);
        Ok(0x10281610u64) }
        0x10281610 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281614u64) }
        0x10281614 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281618u64) }
        0x10281618 => { vm.decode(0x741d9159u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028161cu64) }
        0x1028161c => { vm.decode(0x6a80636fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281620u64) }
        0x10281620 => { vm.decode(0x9b301cb0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281624u64) }
        0x10281624 => { vm.decode(0x1b281cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281628u64) }
        0x10281628 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028162cu64) }
        0x1028162c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281630u64) }
        0x10281630 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10281634u64 + 4);
        vm.wr(37, 0x10281634u64 + 4);
        Ok(0x10281634u64) }
        0x10281634 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281638u64) }
        0x10281638 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028163cu64) }
        0x1028163c => { vm.decode(0x9b305cb0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281640u64) }
        0x10281640 => { vm.decode(0x1b281cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281644u64) }
        0x10281644 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281648u64) }
        0x10281648 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028164cu64) }
        0x1028164c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10281650u64 + 4);
        vm.wr(37, 0x10281650u64 + 4);
        Ok(0x10281650u64) }
        0x10281650 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281654u64) }
        0x10281654 => { vm.decode(0x2280406fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281658u64) }
        0x10281658 => { vm.decode(0x8236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028165cu64) }
        0x1028165c => { vm.decode(0x220d5062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10281660u64) }
        0x10281660 => { vm.decode(0x8c632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281664u64) }
        0x10281664 => { vm.decode(0xb181d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281668u64) }
        0x10281668 => { vm.decode(0x9b101c30u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028166cu64) }
        0x1028166c => { vm.decode(0x1b081c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281670u64) }
        0x10281670 => { vm.decode(0xa86432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281674u64) }
        0x10281674 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281678u64) }
        0x10281678 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028167cu64 + 4);
        vm.wr(37, 0x1028167cu64 + 4);
        Ok(0x1028167cu64) }
        0x1028167c => { vm.decode(0x660d8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10281680u64) }
        0x10281680 => { vm.decode(0x2080602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281684u64) }
        0x10281684 => { vm.decode(0x9b305cb0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281688u64) }
        0x10281688 => { vm.decode(0x1b281cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028168cu64) }
        0x1028168c => { vm.decode(0x891095b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281690u64) }
        0x10281690 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281694u64) }
        0x10281694 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281698u64) }
        0x10281698 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028169cu64 + 4);
        vm.wr(37, 0x1028169cu64 + 4);
        Ok(0x1028169cu64) }
        0x1028169c => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102816a0u64) }
        0x102816a0 => { vm.decode(0x2284406fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102816a4u64) }
        0x102816a4 => { vm.decode(0xa88436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102816a8u64) }
        0x102816a8 => { vm.decode(0xb001d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102816acu64) }
        0x102816ac => { vm.decode(0x9b38d4b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 760i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102816b0u64) }
        0x102816b0 => { vm.decode(0x8908d5b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102816b4u64) }
        0x102816b4 => { vm.decode(0x8b30d5b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102816b8u64) }
        0x102816b8 => { vm.decode(0x1b2894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102816bcu64) }
        0x102816bc => { vm.decode(0x82636fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102816c0u64) }
        0x102816c0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102816c4u64) }
        0x102816c4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102816c8u64 + 4);
        vm.wr(37, 0x102816c8u64 + 4);
        Ok(0x102816c8u64) }
        0x102816c8 => { vm.decode(0x661f8119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102816ccu64) }
        0x102816cc => { vm.decode(0x2284606fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102816d0u64) }
        0x102816d0 => { vm.decode(0x9900d430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102816d4u64) }
        0x102816d4 => { vm.decode(0x9b18d4b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102816d8u64) }
        0x102816d8 => { vm.decode(0x9b1094b0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102816dcu64) }
        0x102816dc => { vm.decode(0x1b0894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102816e0u64) }
        0x102816e0 => { vm.decode(0x282436fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102816e4u64) }
        0x102816e4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102816e8u64) }
        0x102816e8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102816ecu64 + 4);
        vm.wr(37, 0x102816ecu64 + 4);
        Ok(0x102816ecu64) }
        0x102816ec => { vm.decode(0x661f8019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102816f0u64) }
        0x102816f0 => { vm.decode(0x731f1119u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102816f4u64) }
        0x102816f4 => { vm.decode(0x2080402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102816f8u64) }
        0x102816f8 => { vm.decode(0x9b0094b0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102816fcu64) }
        0x102816fc => { vm.decode(0x8b18d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281700u64) }
        0x10281700 => { vm.decode(0x1b109430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281704u64) }
        0x10281704 => { vm.decode(0x282636fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281708u64) }
        0x10281708 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028170cu64) }
        0x1028170c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10281710u64 + 4);
        vm.wr(37, 0x10281710u64 + 4);
        Ok(0x10281710u64) }
        0x10281710 => { vm.decode(0x664b8019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10281714u64) }
        0x10281714 => { vm.decode(0x2080606fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281718u64) }
        0x10281718 => { vm.decode(0x88c636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028171cu64) }
        0x1028171c => { vm.decode(0xb109db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281720u64) }
        0x10281720 => { vm.decode(0x888636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281724u64) }
        0x10281724 => { vm.decode(0xb089db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281728u64) }
        0x10281728 => { vm.decode(0x1b009cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028172cu64) }
        0x1028172c => { vm.decode(0x882436fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281730u64) }
        0x10281730 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281734u64) }
        0x10281734 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10281738u64 + 4);
        vm.wr(37, 0x10281738u64 + 4);
        Ok(0x10281738u64) }
        0x10281738 => { vm.decode(0x661b8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028173cu64) }
        0x1028173c => { vm.decode(0x2880602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281740u64) }
        0x10281740 => { vm.decode(0x88c636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281744u64) }
        0x10281744 => { vm.decode(0xb189d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281748u64) }
        0x10281748 => { vm.decode(0x9b10dd30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028174cu64) }
        0x1028174c => { vm.decode(0x1b089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281750u64) }
        0x10281750 => { vm.decode(0xa86632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281754u64) }
        0x10281754 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281758u64) }
        0x10281758 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028175cu64 + 4);
        vm.wr(37, 0x1028175cu64 + 4);
        Ok(0x1028175cu64) }
        0x1028175c => { vm.decode(0x660f8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10281760u64) }
        0x10281760 => { vm.decode(0xa8c436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281764u64) }
        0x10281764 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10281768u64).wrapping_add((imm * 4) as u64) } else { 0x10281768u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10281768u64) }
        0x10281768 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028176cu64) } }
        0x1028176c => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10281770u64) }
        0x10281770 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10281774u64) }
        0x10281774 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10281778u64) }
        0x10281778 => { vm.decode(0xa8c436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028177cu64) }
        0x1028177c => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10281780u64) }
        0x10281780 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x10281784u64).wrapping_add((imm * 4) as u64) } else { 0x10281784u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10281784u64) }
        0x10281784 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10281788u64) } }
        0x10281788 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028178cu64).wrapping_add((imm * 4) as u64) } else { 0x1028178cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028178cu64) }
        0x1028178c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10281790u64) } }
        0x10281790 => { vm.decode(0x2880402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281794u64) }
        0x10281794 => { vm.decode(0xb009d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281798u64) }
        0x10281798 => { vm.decode(0x1b381cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028179cu64) }
        0x1028179c => { vm.decode(0x82632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102817a0u64) }
        0x102817a0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102817a4u64) }
        0x102817a4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102817a8u64 + 4);
        vm.wr(37, 0x102817a8u64 + 4);
        Ok(0x102817a8u64) }
        0x102817a8 => { vm.decode(0x665d8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102817acu64) }
        0x102817ac => { vm.decode(0x93094b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102817b0u64) }
        0x102817b0 => { vm.decode(0x88c636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102817b4u64) }
        0x102817b4 => { vm.decode(0x8000a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102817b8u64).wrapping_add((imm * 4) as u64) } else { 0x102817b8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102817b8u64) }
        0x102817b8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102817bcu64) } }
        0x102817bc => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102817c0u64) }
        0x102817c0 => { vm.decode(0x2000212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102817c4u64) }
        0x102817c4 => { vm.decode(0xb301db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102817c8u64) }
        0x102817c8 => { vm.decode(0x1b281cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102817ccu64) }
        0x102817cc => { vm.decode(0xa86432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102817d0u64) }
        0x102817d0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102817d4u64) }
        0x102817d4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102817d8u64 + 4);
        vm.wr(37, 0x102817d8u64 + 4);
        Ok(0x102817d8u64) }
        0x102817d8 => { vm.decode(0x661d8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102817dcu64) }
        0x102817dc => { vm.decode(0x92894b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102817e0u64) }
        0x102817e0 => { vm.decode(0x2a80402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102817e4u64) }
        0x102817e4 => { vm.decode(0x9b205db0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102817e8u64) }
        0x102817e8 => { vm.decode(0x1b381c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102817ecu64) }
        0x102817ec => { vm.decode(0x286636fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102817f0u64) }
        0x102817f0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102817f4u64) }
        0x102817f4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102817f8u64 + 4);
        vm.wr(37, 0x102817f8u64 + 4);
        Ok(0x102817f8u64) }
        0x102817f8 => { vm.decode(0x664d8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102817fcu64) }
        0x102817fc => { vm.decode(0x741b9159u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10281800u64) }
        0x10281800 => { vm.decode(0x4a099u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 13i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10281804u64) }
        0x10281804 => { vm.decode(0xa88602fu32); let d = 2i16; let s = 20i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281808u64) }
        0x10281808 => { vm.decode(0x1b389cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028180cu64) }
        0x1028180c => { vm.decode(0x2284432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281810u64) }
        0x10281810 => { vm.decode(0x1b309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281814u64) }
        0x10281814 => { vm.decode(0x9b28ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281818u64) }
        0x10281818 => { vm.decode(0xb209db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028181cu64) }
        0x1028181c => { vm.decode(0x4a8a432fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281820u64) }
        0x10281820 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281824u64) }
        0x10281824 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281828u64) }
        0x10281828 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028182cu64 + 4);
        vm.wr(37, 0x1028182cu64 + 4);
        Ok(0x1028182cu64) }
        0x1028182c => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281830u64) }
        0x10281830 => { vm.decode(0x721bb119u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 228i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10281834u64) }
        0x10281834 => { vm.decode(0x90008099u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10281838u64) }
        0x10281838 => { vm.decode(0x2a80602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028183cu64) }
        0x1028183c => { vm.decode(0x9b389cb0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281840u64) }
        0x10281840 => { vm.decode(0x28c632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281844u64) }
        0x10281844 => { vm.decode(0xb309db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281848u64) }
        0x10281848 => { vm.decode(0x9b28ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028184cu64) }
        0x1028184c => { vm.decode(0x1b209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281850u64) }
        0x10281850 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281854u64) }
        0x10281854 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281858u64) }
        0x10281858 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028185cu64 + 4);
        vm.wr(37, 0x1028185cu64 + 4);
        Ok(0x1028185cu64) }
        0x1028185c => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281860u64) }
        0x10281860 => { vm.decode(0x721b8119u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10281864u64) }
        0x10281864 => { vm.decode(0x2a80602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281868u64) }
        0x10281868 => { vm.decode(0x9b389cb0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028186cu64) }
        0x1028186c => { vm.decode(0x88436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281870u64) }
        0x10281870 => { vm.decode(0xb309db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281874u64) }
        0x10281874 => { vm.decode(0x9b28dcb0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281878u64) }
        0x10281878 => { vm.decode(0x1b209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028187cu64) }
        0x1028187c => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281880u64) }
        0x10281880 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281884u64) }
        0x10281884 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10281888u64 + 4);
        vm.wr(37, 0x10281888u64 + 4);
        Ok(0x10281888u64) }
        0x10281888 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028188cu64) }
        0x1028188c => { vm.decode(0x48099u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 9i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10281890u64) }
        0x10281890 => { vm.decode(0xa88602fu32); let d = 2i16; let s = 20i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281894u64) }
        0x10281894 => { vm.decode(0x1b389cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281898u64) }
        0x10281898 => { vm.decode(0x2284436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028189cu64) }
        0x1028189c => { vm.decode(0x1b309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102818a0u64) }
        0x102818a0 => { vm.decode(0x8b28ddf0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102818a4u64) }
        0x102818a4 => { vm.decode(0xb209db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102818a8u64) }
        0x102818a8 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102818acu64) }
        0x102818ac => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102818b0u64) }
        0x102818b0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102818b4u64 + 4);
        vm.wr(37, 0x102818b4u64 + 4);
        Ok(0x102818b4u64) }
        0x102818b4 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102818b8u64) }
        0x102818b8 => { vm.decode(0x74598119u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102818bcu64) }
        0x102818bc => { vm.decode(0xa120236u32); let d = 22i16; let s = 0i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102818c0u64) }
        0x102818c0 => { vm.decode(0x2084406fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102818c4u64) }
        0x102818c4 => { vm.decode(0x26d5122u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102818c8u64) }
        0x102818c8 => { vm.decode(0x9b3854b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102818ccu64) }
        0x102818cc => { vm.decode(0x1b3014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102818d0u64) }
        0x102818d0 => { vm.decode(0x408c636fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102818d4u64) }
        0x102818d4 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102818d8u64) }
        0x102818d8 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102818dcu64) }
        0x102818dc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102818e0u64 + 4);
        vm.wr(37, 0x102818e0u64 + 4);
        Ok(0x102818e0u64) }
        0x102818e0 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102818e4u64) }
        0x102818e4 => { vm.decode(0x8012036u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 9i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102818e8u64) }
        0x102818e8 => { vm.decode(0x28c602fu32); let d = 2i16; let s = 20i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102818ecu64) }
        0x102818ec => { vm.decode(0x32d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102818f0u64) }
        _ => Err(Error::UnknownPc(pc)),
    }
}
