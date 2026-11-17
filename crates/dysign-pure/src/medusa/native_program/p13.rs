#![cfg_attr(rustfmt, rustfmt::skip)]
// 由 scripts/medusa-vm/gen_native.py 生成，勿手改。
use super::super::Vm;
use super::super::Error;

#[allow(clippy::all, unused_parens, unused_variables, unused_mut)]
pub fn exec(vm: &mut Vm, pc: u64) -> Result<u64, Error> {
    match pc {
        0x102a9bb0 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102a9bb4u64) }
        0x102a9bb4 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102a9bb8u64).wrapping_add((imm * 4) as u64) } else { 0x102a9bb8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a9bb8u64) }
        0x102a9bb8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102a9bbcu64) } }
        0x102a9bbc => { vm.decode(0x2fa4151du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 145276i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102a9bc0u64) }
        0x102a9bc0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102a9bc4u64) } }
        0x102a9bc4 => { vm.decode(0x208c43efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9bc8u64) }
        0x102a9bc8 => { vm.decode(0xb2000000u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102a9bccu64) }
        0x102a9bcc => { vm.decode(0x508480a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 453i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102a9bd0u64).wrapping_add((imm * 4) as u64) } else { 0x102a9bd0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a9bd0u64) }
        0x102a9bd0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102a9bd4u64) } }
        0x102a9bd4 => { vm.decode(0x228063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9bd8u64) }
        0x102a9bd8 => { vm.decode(0xe04004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a9bdcu64) }
        0x102a9bdc => { vm.decode(0x8740323eu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102a9be0u64) }
        0x102a9be0 => { vm.decode(0x20000833u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a9be4u64) }
        0x102a9be4 => { vm.decode(0x40801099u32); let d = 2i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a9be8u64) }
        0x102a9be8 => { vm.decode(0xa0006600u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 57i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102a9becu64) }
        0x102a9bec => { vm.decode(0x82422171u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 207i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102a9bf0u64).wrapping_add((imm * 4) as u64) } else { 0x102a9bf0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a9bf0u64) }
        0x102a9bf0 => { vm.decode(0xa200ad75u32); let d = 4i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102a9bf4u64) }
        0x102a9f2c => { vm.decode(0x228463efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9f30u64) }
        0x102a9f30 => { vm.decode(0x286004f5u32); let d = 3i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a9f34u64) }
        0x102a9f34 => { vm.decode(0xa0004600u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 56i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102a9f38u64) }
        0x102a9f38 => { vm.decode(0x48227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 9i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102a9f3cu64).wrapping_add((imm * 4) as u64) } else { 0x102a9f3cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a9f3cu64) }
        0x102a9f3c => { vm.decode(0x40204018u32); let d = 4i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102a9f40u64) }
        0x102a9f40 => { vm.decode(0x228063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9f44u64) }
        0x102a9f44 => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102a9f48u64) }
        0x102a9f48 => { vm.decode(0x2800103eu32); let d = 5i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102a9f4cu64) }
        0x102a9f4c => { vm.decode(0x420833u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a9f50u64) }
        0x102a9f50 => { vm.decode(0x801099u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a9f54u64) }
        0x102a9f54 => { vm.decode(0xa0004600u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 56i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102a9f58u64) }
        0x102a9f58 => { vm.decode(0xa7f30db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -7i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102a9f5cu64).wrapping_add((imm * 4) as u64) } else { 0x102a9f5cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102a9f5cu64) }
        0x102a9f5c => { vm.decode(0x2010099u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a9f60u64) }
        0x102a9f60 => { vm.decode(0x2300f5u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a9f64u64) }
        0x102a9f64 => { vm.decode(0x930403b5u32); let d = 2i16; let s = 3i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a9f68u64) }
        0x102a9f68 => { vm.decode(0x834003b5u32); let d = 4i16; let s = 3i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a9f6cu64) }
        0x102a9f6c => { vm.decode(0x81c403b5u32); let d = 5i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a9f70u64) }
        0x102a9f70 => { vm.decode(0xf20a4ef3u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1863i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a9f74u64) }
        0x102a9f74 => { vm.decode(0x524a4ef3u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1862i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a9f78u64) }
        0x102a9f78 => { vm.decode(0xd2484e73u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1861i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a9f7cu64) }
        0x102a9f7c => { vm.decode(0x72084e73u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1860i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a9f80u64) }
        0x102a9f80 => { vm.decode(0xd20a4af3u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1859i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a9f84u64) }
        0x102a9f84 => { vm.decode(0x22300f5u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 40i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a9f88u64) }
        0x102a9f88 => { vm.decode(0x520a4af3u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1858i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a9f8cu64) }
        0x102a9f8c => { vm.decode(0x2320f5u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 48i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a9f90u64) }
        0x102a9f90 => { vm.decode(0xd20a4a73u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1857i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a9f94u64) }
        0x102a9f94 => { vm.decode(0x22320f5u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 56i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102a9f98u64) }
        0x102a9f98 => { vm.decode(0x520a4a73u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a9f9cu64) }
        0x102a9f9c => { vm.decode(0x6a8263afu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9fa0u64) }
        0x102a9fa0 => { vm.decode(0x2280416fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9fa4u64) }
        0x102a9fa4 => { vm.decode(0x101019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a9fa8u64) }
        0x102a9fa8 => { vm.decode(0xb2115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9facu64) }
        0x102a9fac => { vm.decode(0x102036u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102a9fb0u64) }
        0x102a9fb0 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a9fb4u64) }
        0x102a9fb4 => { vm.decode(0x746b1159u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2000i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a9fb8u64) }
        0x102a9fb8 => { vm.decode(0x8b311530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9fbcu64) }
        0x102a9fbc => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9fc0u64) }
        0x102a9fc0 => { vm.decode(0x760d0519u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a9fc4u64) }
        0x102a9fc4 => { vm.decode(0x408ec32fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9fc8u64) }
        0x102a9fc8 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a9fccu64) }
        0x102a9fcc => { vm.decode(0x688c43afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9fd0u64) }
        0x102a9fd0 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a9fd4u64) }
        0x102a9fd4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102a9fd8u64 + 4);
        vm.wr(37, 0x102a9fd8u64 + 4);
        Ok(0x102a9fd8u64) }
        0x102a9fd8 => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a9fdcu64) }
        0x102a9fdc => { vm.decode(0x740d0059u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a9fe0u64) }
        0x102a9fe0 => { vm.decode(0x4008019u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 512i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102a9fe4u64) }
        0x102a9fe4 => { vm.decode(0x288416fu32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102a9fe8u64) }
        0x102a9fe8 => { vm.decode(0x1b2114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9fecu64) }
        0x102a9fec => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102a9ff0u64) }
        0x102a9ff0 => { vm.decode(0x8b311430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9ff4u64) }
        0x102a9ff4 => { vm.decode(0xb291530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102a9ff8u64) }
        0x102a9ff8 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102a9ffcu64) }
        0x102a9ffc => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102aa000u64) }
        0x102aa000 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102aa004u64 + 4);
        vm.wr(37, 0x102aa004u64 + 4);
        Ok(0x102aa004u64) }
        0x102aa004 => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102aa008u64) }
        0x102aa008 => { vm.decode(0x228863afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102aa00cu64) }
        0x102aa00c => { vm.decode(0x24000506u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102aa010u64) }
        0x102aa010 => { vm.decode(0x40227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102aa014u64).wrapping_add((imm * 4) as u64) } else { 0x102aa014u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102aa014u64) }
        0x102aa014 => { vm.decode(0x60491519u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102aa018u64) }
        0x102aa018 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102aa01cu64) }
        0x102aa01c => { vm.decode(0x5040022u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102aa020u64) }
        0x102aa020 => { vm.decode(0x803019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102aa024u64) }
        0x102aa024 => { vm.decode(0x8903036u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102aa028u64) }
        0x102aa028 => { vm.decode(0x24000506u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102aa02cu64) }
        0x102aa02c => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102aa030u64).wrapping_add((imm * 4) as u64) } else { 0x102aa030u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102aa030u64) }
        0x102aa030 => { vm.decode(0x30803019u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102aa034u64) }
        0x102aa034 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102aa038u64) }
        0x102aa038 => { vm.decode(0x2002406u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102aa03cu64) }
        0x102aa03c => { vm.decode(0x48a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 11i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102aa040u64).wrapping_add((imm * 4) as u64) } else { 0x102aa040u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102aa040u64) }
        0x102aa040 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102aa044u64) } }
        0x102aa044 => { vm.decode(0x8120002eu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = -3i64;
        let a = vm.ld(s).wrapping_add(imm as u64).wrapping_add(3) as u32;
        let sh = (a & 3) * 8;
        let b = vm.mem.read((a & !3) as u64, 4)?;
        let w0 = u32::from_le_bytes(b.try_into().unwrap());
        let v = if sh == 24 { w0 as i32 as i64 } else { let x = vm.ld(d) as u32; (w0.wrapping_shl(24 - sh) | (x & (0x00ff_ffffu32 >> sh))) as i32 as i64 };
        vm.st(d, v as u64);
        Ok(0x102aa048u64) }
        0x102aa048 => { vm.decode(0x2ff57e5fu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = -3i64;
        let a = vm.ld(s).wrapping_add(imm as u64) as u32;
        let sh = (a & 3) * 8;
        let b = vm.mem.read((a & !3) as u64, 4)?;
        let w0 = u32::from_le_bytes(b.try_into().unwrap());
        let v = if sh == 0 { w0 as i32 as i64 } else { let x = vm.ld(d) as u32; (w0.wrapping_shr(sh) | (x & 0xffff_ffffu32.wrapping_shl(32 - sh))) as i32 as i64 };
        vm.st(d, v as u64);
        Ok(0x102aa04cu64) }
        0x102aa04c => { vm.decode(0x4003a649u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let x = vm.ld(s) as u32;
        let v = ((x >> 8) & 0x00ff_00ff) | ((x << 8) & 0xff00_ff00);
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102aa050u64) }
        0x102aa050 => { vm.decode(0x18014d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102aa054u64) }
        0x102aa054 => { vm.decode(0x1040022u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102aa058u64) }
        0x102aa058 => { vm.decode(0x4080b019u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102aa05cu64) }
        0x102aa05c => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102aa060u64) }
        0x102aa060 => { vm.decode(0x2002406u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102aa064u64) }
        0x102aa064 => { vm.decode(0x27f32df1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -9i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102aa068u64).wrapping_add((imm * 4) as u64) } else { 0x102aa068u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102aa068u64) }
        0x102aa068 => { vm.decode(0x30002019u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102aa06cu64) }
        0x102aa06c => { vm.decode(0x640d8059u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102aa070u64) }
        0x102aa070 => { vm.decode(0x1b009430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102aa074u64) }
        0x102aa074 => { vm.decode(0x40001119u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102aa078u64) }
        0x102aa078 => { vm.decode(0x2636u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102aa07cu64) }
        0x102aa07c => { vm.decode(0x22008406u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 128i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102aa080u64) }
        0x102aa080 => { vm.decode(0xc8a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102aa084u64).wrapping_add((imm * 4) as u64) } else { 0x102aa084u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102aa084u64) }
        0x102aa084 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102aa088u64) } }
        0x102aa088 => { vm.decode(0xaaecae51u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = -8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102aa08cu64) }
        0x102aa08c => { vm.decode(0xa2ec27d1u32); let d = 4i16; let s = 2i16; let s2 = -1i16; let imm = -28i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102aa090u64) }
        0x102aa090 => { vm.decode(0x1c05cd35u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 19i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102aa094u64) }
        0x102aa094 => { vm.decode(0x815cd35u32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 17i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102aa098u64) }
        0x102aa098 => { vm.decode(0x4141a835u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102aa09cu64) }
        0x102aa09c => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102aa0a0u64) }
        0x102aa0a0 => { vm.decode(0x80a403b5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 10i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102aa0a4u64) }
        0x102aa0a4 => { vm.decode(0x60012835u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102aa0a8u64) }
        0x102aa0a8 => { vm.decode(0xaae827d1u32); let d = 5i16; let s = 2i16; let s2 = -1i16; let imm = -60i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102aa0acu64) }
        0x102aa0ac => { vm.decode(0x206004f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102aa0b0u64) }
        0x102aa0b0 => { vm.decode(0xc254d35u32); let d = 4i16; let s = 5i16; let s2 = -1i16; let imm = 18i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102aa0b4u64) }
        0x102aa0b4 => { vm.decode(0xcb4cd35u32); let d = 6i16; let s = 5i16; let s2 = -1i16; let imm = 7i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102aa0b8u64) }
        0x102aa0b8 => { vm.decode(0x1418835u32); let d = 4i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102aa0bcu64) }
        0x102aa0bc => { vm.decode(0xc827c909u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102aa0c0u64) }
        0x102aa0c0 => { vm.decode(0x80e843b5u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102aa0c4u64) }
        0x102aa0c4 => { vm.decode(0x41018835u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102aa0c8u64) }
        0x102aa0c8 => { vm.decode(0x206004f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102aa0ccu64) }
        0x102aa0cc => { vm.decode(0xa2e82751u32); let d = 4i16; let s = 2i16; let s2 = -1i16; let imm = -64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102aa0d0u64) }
        0x102aa0d0 => { vm.decode(0x280a019u32); let d = 5i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102aa0d4u64) }
        0x102aa0d4 => { vm.decode(0x8203236u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102aa0d8u64) }
        0x102aa0d8 => { vm.decode(0x206004f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102aa0dcu64) }
        0x102aa0dc => { vm.decode(0x820bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102aa0e0u64) }
        0x102aa0e0 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102aa0e4u64) }
        0x102aa0e4 => { vm.decode(0x22008406u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 128i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102aa0e8u64) }
        0x102aa0e8 => { vm.decode(0x27d32df1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -25i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102aa0ecu64).wrapping_add((imm * 4) as u64) } else { 0x102aa0ecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102aa0ecu64) }
        0x102aa0ec => { vm.decode(0x44004875u32); let d = 2i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102aa0f0u64) }
        0x102aa0f0 => { vm.decode(0x208043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102aa0f4u64) }
        0x102aa0f4 => { vm.decode(0xda361c59u32); let d = 22i16; let s = 1i16; let s2 = -1i16; let imm = -12886i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102aa0f8u64) }
        0x102aa0f8 => { vm.decode(0x102036u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102aa0fcu64) }
        0x102aa0fc => { vm.decode(0x640d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102aa100u64) }
        0x102aa100 => { vm.decode(0x1b3814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102aa104u64) }
        0x102aa104 => { vm.decode(0x6a8843afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102aa108u64) }
        0x102aa108 => { vm.decode(0x428c63efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102aa10cu64) }
        0x102aa10c => { vm.decode(0x488443afu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102aa110u64) }
        0x102aa110 => { vm.decode(0x24900486u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 97i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102aa114u64) }
        0x102aa114 => { vm.decode(0x10848a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 79i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102aa118u64).wrapping_add((imm * 4) as u64) } else { 0x102aa118u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102aa118u64) }
        0x102aa118 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102aa11cu64) } }
        0x102aa11c => { vm.decode(0x6a8263afu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102aa120u64) }
        0x102aa120 => { vm.decode(0x2884612fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102aa124u64) }
        0x102aa124 => { vm.decode(0x1119u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102aa128u64) }
        0x102aa128 => { vm.decode(0xb191530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102aa12cu64) }
        0x102aa12c => { vm.decode(0x8b115530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102aa130u64) }
        0x102aa130 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102aa134u64) }
        0x102aa134 => { vm.decode(0xd98ea51u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102aa138u64) }
        0x102aa138 => { vm.decode(0x33d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102aa13cu64) }
        0x102aa13c => { vm.decode(0x5d9c63d1u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2020i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102aa140u64) }
        0x102aa140 => { vm.decode(0x559cf351u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102aa144u64) }
        0x102aa144 => { vm.decode(0x15986bd1u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2004i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102aa148u64) }
        0x102aa148 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102aa14cu64) }
        0x102aa14c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102aa150u64) }
        0x102aa150 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102aa154u64 + 4);
        vm.wr(37, 0x102aa154u64 + 4);
        Ok(0x102aa154u64) }
        0x102aa154 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102aa158u64) }
        0x102aa158 => { vm.decode(0x2884612fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102aa15cu64) }
        0x102aa15c => { vm.decode(0x21019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102aa160u64) }
        0x102aa160 => { vm.decode(0xb191530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102aa164u64) }
        0x102aa164 => { vm.decode(0x8b115530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102aa168u64) }
        0x102aa168 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102aa16cu64) }
        0x102aa16c => { vm.decode(0x20c443afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102aa170u64) }
        0x102aa170 => { vm.decode(0x80402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102aa174u64) }
        0x102aa174 => { vm.decode(0x8c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102aa178u64) }
        0x102aa178 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102aa17cu64) }
        0x102aa17c => { vm.decode(0x1c000111u32); let d = 21i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102aa180u64) }
        0x102aa180 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102aa184u64) }
        0x102aa184 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102aa188u64) }
        0x102aa188 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102aa18cu64 + 4);
        vm.wr(37, 0x102aa18cu64 + 4);
        Ok(0x102aa18cu64) }
        0x102aa18c => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102aa190u64) }
        0x102aa190 => { vm.decode(0x53b835u32); let d = 1i16; let s = 20i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102aa194u64) }
        0x102aa194 => { vm.decode(0x131217f5u32); let d = 1i16; let s = 1i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102aa198u64) }
        0x102aa198 => { vm.decode(0x2f30cd35u32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 11i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102aa19cu64) }
        0x102aa19c => { vm.decode(0x3db04d35u32); let d = 3i16; let s = 23i16; let s2 = -1i16; let imm = 6i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102aa1a0u64) }
        0x102aa1a0 => { vm.decode(0x40112835u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102aa1a4u64) }
        0x102aa1a4 => { vm.decode(0x204c0835u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102aa1a8u64) }
        0x102aa1a8 => { vm.decode(0x3b31cd35u32); let d = 3i16; let s = 23i16; let s2 = -1i16; let imm = 25i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102aa1acu64) }
        0x102aa1ac => { vm.decode(0x404c0835u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102aa1b0u64) }
        0x102aa1b0 => { vm.decode(0x4d18c211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102aa1b4u64) }
        0x102aa1b4 => { vm.decode(0x24202fb5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102aa1b8u64) }
        0x102aa1b8 => { vm.decode(0x8903036u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102aa1bcu64) }
        0x102aa1bc => { vm.decode(0x40210fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102aa1c0u64) }
        0x102aa1c0 => { vm.decode(0x45986a51u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102aa1c4u64) }
        0x102aa1c4 => { vm.decode(0x4d9c6251u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102aa1c8u64) }
        0x102aa1c8 => { vm.decode(0x41057f5u32); let d = 4i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102aa1ccu64) }
        0x102aa1cc => { vm.decode(0x3a84cd35u32); let d = 5i16; let s = 3i16; let s2 = -1i16; let imm = 13i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102aa1d0u64) }
        0x102aa1d0 => { vm.decode(0x2c144d35u32); let d = 6i16; let s = 3i16; let s2 = -1i16; let imm = 2i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102aa1d4u64) }
        0x102aa1d4 => { vm.decode(0xf5b9d0cdu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 2027i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let w0 = vm.mem.r64(a & !7u64)?;
        let sh = ((a & 7) * 8) as u32;
        let v = w0.wrapping_shl(56 - sh) | (vm.ld(d) & 0x00ff_ffff_ffff_ffffu64.wrapping_shr(sh));
        vm.st(d, v);
        Ok(0x102aa1d8u64) }
        0x102aa1d8 => { vm.decode(0x49bcdb02u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 2020i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let w0 = vm.mem.r64(a & !7u64)?;
        let sh = ((a & 7) * 8) as u32;
        let v = if sh == 0 { w0 } else { w0.wrapping_shr(sh) | (vm.ld(d) & u64::MAX.wrapping_shl(64 - sh)) };
        vm.st(d, v);
        Ok(0x102aa1dcu64) }
        0x102aa1dc => { vm.decode(0x59cf2d1u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 2028i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102aa1e0u64) }
        0x102aa1e0 => { vm.decode(0xe93b779eu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 2031i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let w8 = a & !7u64;
        let sh = ((a & 7) * 8) as u32;
        let v = vm.ld(d);
        let old = vm.mem.r64(w8)?;
        let mk = if sh + 8 >= 64 { u64::MAX } else { (1u64 << (sh + 8)) - 1 };
        vm.sw64(w8, (old & !mk) | (v >> (56 - sh)))?;
        Ok(0x102aa1e4u64) }
        0x102aa1e4 => { vm.decode(0x4141a835u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102aa1e8u64) }
        0x102aa1e8 => { vm.decode(0x44c01875u32); let d = 6i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102aa1ecu64) }
        0x102aa1ec => { vm.decode(0xd98fad1u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 2012i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102aa1f0u64) }
        0x102aa1f0 => { vm.decode(0x525047f5u32); let d = 6i16; let s = 6i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102aa1f4u64) }
        0x102aa1f4 => { vm.decode(0x2c916d35u32); let d = 10i16; let s = 3i16; let s2 = -1i16; let imm = 22i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102aa1f8u64) }
        0x102aa1f8 => { vm.decode(0x10805875u32); let d = 4i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102aa1fcu64) }
        0x102aa1fc => { vm.decode(0x31052835u32); let d = 5i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102aa200u64) }
        0x102aa200 => { vm.decode(0x45986bd1u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 2004i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102aa204u64) }
        0x102aa204 => { vm.decode(0x427d7862u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 2008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102aa208u64) }
        0x102aa208 => { vm.decode(0x632f1862u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 2016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102aa20cu64) }
        0x102aa20c => { vm.decode(0x422d7c62u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2004i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102aa210u64) }
        0x102aa210 => { vm.decode(0x632d5c62u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2020i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102aa214u64) }
        0x102aa214 => { vm.decode(0x9a9935ebu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 2024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let w8 = a & !7u64;
        let sh = ((a & 7) * 8) as u32;
        let v = vm.ld(d);
        let old = vm.mem.r64(w8)?;
        let mk = if sh == 0 { 0 } else { (1u64 << sh) - 1 };
        vm.sw64(w8, (old & mk) | (v << sh))?;
        Ok(0x102aa218u64) }
        0x102aa218 => { vm.decode(0x24900fb5u32); let d = 2i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102aa21cu64) }
        0x102aa21c => { vm.decode(0xc4810fb5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102aa220u64) }
        0x102aa220 => { vm.decode(0x20cc43afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102aa224u64) }
        0x102aa224 => { vm.decode(0x82403d75u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102aa228u64) }
        0x102aa228 => { vm.decode(0x88243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102aa22cu64) }
        0x102aa22c => { vm.decode(0xc84024f5u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102aa230u64) }
        0x102aa230 => { vm.decode(0x4c400011u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102aa234u64) }
        0x102aa234 => { vm.decode(0x44012fb5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102aa238u64) }
        0x102aa238 => { vm.decode(0x24002fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102aa23cu64) }
        0x102aa23c => { vm.decode(0x423d7c62u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2012i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102aa240u64) }
        0x102aa240 => { vm.decode(0x40014fb5u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102aa244u64) }
        0x102aa244 => { vm.decode(0x432d3862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102aa248u64) }
        0x102aa248 => { vm.decode(0x24900486u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 97i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102aa24cu64) }
        0x102aa24c => { vm.decode(0x27f32cb1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -77i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102aa250u64).wrapping_add((imm * 4) as u64) } else { 0x102aa250u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102aa250u64) }
        0x102aa250 => { vm.decode(0x32811099u32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102aa254u64) }
        0x102aa254 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102aa258u64) }
        0x102aa258 => { vm.decode(0x646b9159u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2000i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102aa25cu64) }
        0x102aa25c => { vm.decode(0x2000506u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102aa260u64) }
        0x102aa260 => { vm.decode(0x40a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102aa264u64).wrapping_add((imm * 4) as u64) } else { 0x102aa264u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102aa264u64) }
        0x102aa264 => { vm.decode(0x62490519u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 2128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102aa268u64) }
        0x102aa268 => { vm.decode(0xc400011u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102aa26cu64) }
        0x102aa26c => { vm.decode(0x8100111u32); let d = 5i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102aa270u64) }
        0x102aa270 => { vm.decode(0x44110fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102aa274u64) }
        0x102aa274 => { vm.decode(0x1001022u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102aa278u64) }
        0x102aa278 => { vm.decode(0x2012019u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102aa27cu64) }
        0x102aa27c => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102aa280u64) }
        0x102aa280 => { vm.decode(0x2000506u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102aa284u64) }
        0x102aa284 => { vm.decode(0xa7f10db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -8i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102aa288u64).wrapping_add((imm * 4) as u64) } else { 0x102aa288u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102aa288u64) }
        0x102aa288 => { vm.decode(0x4080b019u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102aa28cu64) }
        0x102aa28c => { vm.decode(0x2080436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102aa290u64) }
        0x102aa290 => { vm.decode(0xa0804118u32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102aa294u64) }
        0x102aa294 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102aa298u64) }
        0x102aa298 => { vm.decode(0x2000506u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102aa29cu64) }
        0x102aa29c => { vm.decode(0xc8027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 17i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102aa2a0u64).wrapping_add((imm * 4) as u64) } else { 0x102aa2a0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102aa2a0u64) }
        0x102aa2a0 => { vm.decode(0x62490519u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 2128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102aa2a4u64) }
        0x102aa2a4 => { vm.decode(0x300010beu32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102aa2a8u64) }
        0x102aa2a8 => { vm.decode(0x9dbe7f33u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = -3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102aa2acu64) }
        0x102aa2ac => { vm.decode(0x808823u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 2)?;
        let v = u16::from_le_bytes(b.try_into().unwrap()) as u64;
        vm.st(d, v);
        Ok(0x102aa2b0u64) }
        0x102aa2b0 => { vm.decode(0x1dbe7fb3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = -2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102aa2b4u64) }
        0x102aa2b4 => { vm.decode(0x8100011u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102aa2b8u64) }
        0x102aa2b8 => { vm.decode(0x40900d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102aa2bcu64) }
        0x102aa2bc => { vm.decode(0x9dbe7fb3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = -1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102aa2c0u64) }
        0x102aa2c0 => { vm.decode(0x1a019u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102aa2c4u64) }
        0x102aa2c4 => { vm.decode(0x4280a019u32); let d = 5i16; let s = 3i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102aa2c8u64) }
        0x102aa2c8 => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102aa2ccu64) }
        0x102aa2cc => { vm.decode(0x100111u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102aa2d0u64) }
        0x102aa2d0 => { vm.decode(0x600833u32); let d = 4i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102aa2d4u64) }
        0x102aa2d4 => { vm.decode(0x4800875u32); let d = 4i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102aa2d8u64) }
        0x102aa2d8 => { vm.decode(0x2000506u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102aa2dcu64) }
        0x102aa2dc => { vm.decode(0x27f30db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102aa2e0u64).wrapping_add((imm * 4) as u64) } else { 0x102aa2e0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102aa2e0u64) }
        0x102aa2e0 => { vm.decode(0x4400c875u32); let d = 3i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102aa2e4u64) }
        0x102aa2e4 => { vm.decode(0x2a8463afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102aa2e8u64) }
        0x102aa2e8 => { vm.decode(0x88c63afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102aa2ecu64) }
        0x102aa2ec => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102aa2f0u64) }
        0x102aa2f0 => { vm.decode(0x2888636fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102aa2f4u64) }
        0x102aa2f4 => { vm.decode(0x10400430u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102aa2f8u64) }
        0x102aa2f8 => { vm.decode(0x288043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102aa2fcu64) }
        0x102aa2fc => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102aa300u64) }
        0x102aa300 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102aa304u64) }
        0x102aa304 => { vm.decode(0x10480430u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102aa308u64) }
        0x102aa308 => { vm.decode(0x408c436fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102aa30cu64) }
        0x102aa30c => { vm.decode(0x120d19du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 289120i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102aa310u64) }
        0x102aa310 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102aa314u64) } }
        0x102aa314 => { vm.decode(0x3c0041du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 164400i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102aa318u64) }
        0x102aa318 => { vm.decode(0x23257022u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102aa31cu64) }
        0x102aa31c => { vm.decode(0xbe4541du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 170872i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102aa320u64) }
        0x102aa320 => { vm.decode(0x23257022u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102aa324u64) }
        0x102aa324 => { vm.decode(0x8e64011du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 177352i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102aa328u64) }
        0x102aa328 => { vm.decode(0x23257022u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102aa32cu64) }
        0x102aa32c => { vm.decode(0x6c0c41du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 183824i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102aa330u64) }
        0x102aa330 => { vm.decode(0x43255022u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102aa334u64) }
        0x102aa334 => { vm.decode(0x8ec0911du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 190344i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102aa338u64) }
        0x102aa338 => { vm.decode(0x23257022u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102aa33cu64) }
        0x102aa33c => { vm.decode(0x3c0041du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 164400i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102aa340u64) }
        0x102ae5b0 => { vm.decode(0x641d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ae5b4u64) }
        0x102ae5b4 => { vm.decode(0x193094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ae5b8u64) }
        0x102ae5b8 => { vm.decode(0x640d8519u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ae5bcu64) }
        0x102ae5bc => { vm.decode(0x192894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ae5c0u64) }
        0x102ae5c0 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ae5c4u64) }
        0x102ae5c4 => { vm.decode(0x442237d9u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 8133i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ae5c8u64) }
        0x102ae5c8 => { vm.decode(0x12100019u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ae5ccu64) }
        0x102ae5cc => { vm.decode(0x640d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ae5d0u64) }
        0x102ae5d0 => { vm.decode(0x1b2014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ae5d4u64) }
        0x102ae5d4 => { vm.decode(0x4100236u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102ae5d8u64) }
        0x102ae5d8 => { vm.decode(0x32028000u32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102ae5dcu64) }
        0x102ae5dc => { vm.decode(0x48c08a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 182i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102ae5e0u64).wrapping_add((imm * 4) as u64) } else { 0x102ae5e0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ae5e0u64) }
        0x102ae5e0 => { vm.decode(0x92095b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ae5e4u64) }
        0x102ae5e4 => { vm.decode(0x2f44041du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 169020i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102ae5e8u64) }
        0x102ae5e8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102ae5ecu64) } }
        0x102ae5ec => { vm.decode(0x110039b5u32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102ae5f0u64) }
        0x102af7bc => { vm.decode(0xf0027u32); let d = 0i16; let s = 21i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102af7c0u64).wrapping_add((imm * 4) as u64) } else { 0x102af7c0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102af7c0u64) }
        0x102af7c0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102af7c4u64) } }
        0x102af7c4 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102af7c8u64) }
        0x102af7c8 => { vm.decode(0x2000016fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102af7ccu64) }
        0x102af7cc => { vm.decode(0x99215cb0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102af7d0u64) }
        0x102af7d0 => { vm.decode(0x99395d30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102af7d4u64) }
        0x102af7d4 => { vm.decode(0x19291c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102af7d8u64) }
        0x102af7d8 => { vm.decode(0xd1c4811u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102af7dcu64) }
        0x102af7dc => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102af7e0u64) }
        0x102af7e0 => { vm.decode(0xa88636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102af7e4u64) }
        0x102af7e4 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102af7e8u64) }
        0x102af7e8 => { vm.decode(0x40448019u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102af7ecu64) }
        0x102af7ec => { vm.decode(0x19311c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102af7f0u64) }
        0x102af7f0 => { vm.decode(0x886632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102af7f4u64) }
        0x102af7f4 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102af7f8u64) }
        0x102af7f8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102af7fcu64 + 4);
        vm.wr(37, 0x102af7fcu64 + 4);
        Ok(0x102af7fcu64) }
        0x102af7fc => { vm.decode(0x620d8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102af800u64) }
        0x102af800 => { vm.decode(0x1510e211u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102af804u64) }
        0x102af804 => { vm.decode(0x10560d35u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 21i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102af808u64) }
        0x102af808 => { vm.decode(0xd20a4bf3u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1867i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102af80cu64) }
        0x102af80c => { vm.decode(0x50540d35u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 13i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102af810u64) }
        0x102af810 => { vm.decode(0x520a4bf3u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1866i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102af814u64) }
        0x102af814 => { vm.decode(0x10540d35u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102af818u64) }
        0x102af818 => { vm.decode(0xd20a4b73u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1865i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102af81cu64) }
        0x102af81c => { vm.decode(0x44028bb5u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102af820u64) }
        0x102af820 => { vm.decode(0x520a4b73u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102af824u64) }
        0x102af824 => { vm.decode(0x4a8263afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102af828u64) }
        0x102af828 => { vm.decode(0x2884402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102af82cu64) }
        0x102af82c => { vm.decode(0x10001119u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102af830u64) }
        0x102af830 => { vm.decode(0x8b211530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102af834u64) }
        0x102af834 => { vm.decode(0x888436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102af838u64) }
        0x102af838 => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102af83cu64) }
        0x102af83c => { vm.decode(0x28863efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102af840u64) }
        0x102af840 => { vm.decode(0xb1115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102af844u64) }
        0x102af844 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102af848u64) }
        0x102af848 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102af84cu64) }
        0x102af84c => { vm.decode(0x688c43afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102af850u64) }
        0x102af850 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102af854u64) }
        0x102af854 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102af858u64 + 4);
        vm.wr(37, 0x102af858u64 + 4);
        Ok(0x102af858u64) }
        0x102af858 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102af85cu64) }
        0x102af85c => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102af860u64) }
        0x102af860 => { vm.decode(0x760f9119u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102af864u64) }
        0x102af864 => { vm.decode(0x2280406fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102af868u64) }
        0x102af868 => { vm.decode(0x8b2115b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102af86cu64) }
        0x102af86c => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102af870u64) }
        0x102af870 => { vm.decode(0x9b315530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102af874u64) }
        0x102af874 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102af878u64) }
        0x102af878 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102af87cu64) }
        0x102af87c => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102af880u64) }
        0x102af880 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102af884u64 + 4);
        vm.wr(37, 0x102af884u64 + 4);
        Ok(0x102af884u64) }
        0x102af884 => { vm.decode(0x102036u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102af888u64) }
        0x102af888 => { vm.decode(0x50ea012du32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 63i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102af88cu64) }
        0x102af88c => { vm.decode(0x4018676u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 124i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102af890u64) }
        0x102af890 => { vm.decode(0x60080b35u32); let d = 16i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102af894u64) }
        0x102af894 => { vm.decode(0x4018636u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 60i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102af898u64) }
        0x102af898 => { vm.decode(0x20280b35u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102af89cu64) }
        0x102af89c => { vm.decode(0x30004640u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 60i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102af8a0u64) }
        0x102af8a0 => { vm.decode(0x40032f5u32); let d = 16i16; let s = 2i16; let s2 = 1i16; let imm = 0i64;
        if vm.ld(s2) != 0 {
            let v = vm.ld(s);
            vm.st(d, v);
        }
        Ok(0x102af8a4u64) }
        0x102af8a4 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102af8a8u64) }
        0x102af8a8 => { vm.decode(0x542617d9u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = 8137i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102af8acu64) }
        0x102af8ac => { vm.decode(0x10049019u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102af8b0u64) }
        0x102af8b0 => { vm.decode(0x770d1419u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102af8b4u64) }
        0x102af8b4 => { vm.decode(0x24000486u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102af8b8u64) }
        0x102af8b8 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102af8bcu64).wrapping_add((imm * 4) as u64) } else { 0x102af8bcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102af8bcu64) }
        0x102af8bc => { vm.decode(0x2110036u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102af8c0u64) }
        0x102af8c0 => { vm.decode(0x2ec4041du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 169500i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102af8c4u64) }
        0x102af8c8 => { vm.decode(0x8624735u32); let d = 1i16; let s = 4i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102af8ccu64) }
        0x102af8cc => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102af8d0u64) }
        0x102af8d0 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102af8d4u64) }
        0x102af8d4 => { vm.decode(0x74304675u32); let d = 3i16; let s = 3i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102af8d8u64) }
        0x102af8d8 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102af8dcu64) }
        0x102af8dc => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102af8e0u64) }
        0x102af8e0 => { vm.decode(0xab2124f5u32); let d = 2i16; let s = 21i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102af8e4u64) }
        0x102af8e4 => { vm.decode(0x220033u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102af8e8u64) }
        0x102af8e8 => { vm.decode(0x8903036u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102af8ecu64) }
        0x102af8ec => { vm.decode(0x24000486u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102af8f0u64) }
        0x102af8f0 => { vm.decode(0x4040a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 42i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102af8f4u64).wrapping_add((imm * 4) as u64) } else { 0x102af8f4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102af8f4u64) }
        0x102af8f4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102af8f8u64) } }
        0x102af8f8 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102af8fcu64) }
        0x102af8fc => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102af900u64) }
        0x102af900 => { vm.decode(0x8cc3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102af904u64) }
        0x102af904 => { vm.decode(0x9b191530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102af908u64) }
        0x102af908 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102af90cu64) }
        0x102af90c => { vm.decode(0x434004f5u32); let d = 1i16; let s = 2i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102af910u64) }
        0x102af910 => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102af914u64) }
        0x102af914 => { vm.decode(0xc0860bb5u32); let d = 21i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102af918u64) }
        0x102af918 => { vm.decode(0x228063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102af91cu64) }
        0x102af91c => { vm.decode(0xa16004f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102af920u64) }
        0x102af920 => { vm.decode(0xa04023eu32); let d = 22i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102af924u64) }
        0x102af924 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102af928u64) }
        0x102af928 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102af92cu64) }
        0x102af92c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102af930u64 + 4);
        vm.wr(37, 0x102af930u64 + 4);
        Ok(0x102af930u64) }
        0x102af930 => { vm.decode(0xb0d875u32); let d = 5i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102af934u64) }
        0x102af934 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102af938u64) }
        0x102af938 => { vm.decode(0x28c40b35u32); let d = 4i16; let s = 3i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102af93cu64) }
        0x102af93c => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102af940u64) }
        0x102af940 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102af944u64) }
        0x102af944 => { vm.decode(0x40227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102af948u64).wrapping_add((imm * 4) as u64) } else { 0x102af948u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102af948u64) }
        0x102af948 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102af94cu64) }
        0x102af94c => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102af950u64) }
        0x102af950 => { vm.decode(0xd2ca412du32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102af954u64) }
        0x102af954 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102af958u64) }
        0x102af958 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102af95cu64) }
        0x102af95c => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102af960u64) }
        0x102af960 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102af964u64).wrapping_add((imm * 4) as u64) } else { 0x102af964u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102af964u64) }
        0x102af964 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102af968u64) } }
        0x102af968 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102af96cu64) }
        0x102af96c => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102af970u64) }
        0x102af970 => { vm.decode(0x739cb4a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -43i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102af974u64).wrapping_add((imm * 4) as u64) } else { 0x102af974u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102af974u64) }
        0x102af974 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102af978u64) }
        0x102af978 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102af97cu64) }
        0x102af97c => { vm.decode(0xd2ca816du32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102af980u64) }
        0x102af980 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102af984u64) }
        0x102af984 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102af988u64) }
        0x102af988 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102af98cu64) }
        0x102af98c => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102af990u64).wrapping_add((imm * 4) as u64) } else { 0x102af990u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102af990u64) }
        0x102af990 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102af994u64) } }
        0x102af994 => { vm.decode(0xe64101du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 169288i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102af998u64) }
        0x102af998 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102af99cu64) } }
        0x102af99c => { vm.decode(0x408e6befu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102af9a0u64) }
        0x102af9a0 => { vm.decode(0x628243efu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102af9a4u64) }
        0x102af9a4 => { vm.decode(0x661a0427u32); let d = 0i16; let s = 16i16; let s2 = -1i16; let imm = 12080i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102af9a8u64).wrapping_add((imm * 4) as u64) } else { 0x102af9a8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102af9a8u64) }
        0x102af9a8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102af9acu64) } }
        0x102af9ac => { vm.decode(0xd10e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102af9b0u64) }
        0x102af9b0 => { vm.decode(0xa4000fb5u32); let d = 2i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102af9b4u64) }
        0x102af9b4 => { vm.decode(0x21d5062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102af9b8u64) }
        0x102af9b8 => { vm.decode(0x20082e35u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) < vm.ld(s)) as u64;
        vm.st(d, v);
        Ok(0x102af9bcu64) }
        0x102af9bc => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102af9c0u64).wrapping_add((imm * 4) as u64) } else { 0x102af9c0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102af9c0u64) }
        0x102af9c0 => { vm.decode(0x50da812du32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 63i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102af9c4u64) }
        0x102af9c4 => { vm.decode(0xd10e291u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 780i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102af9c8u64) }
        0x102af9d0 => { vm.decode(0x80c08a7u32); let d = 0i16; let s = 3i16; let s2 = -1i16; let imm = 146i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102af9d4u64).wrapping_add((imm * 4) as u64) } else { 0x102af9d4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102af9d4u64) }
        0x102af9d4 => { vm.decode(0x8b654909u32); let d = 22i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102af9d8u64) }
        0x102af9d8 => { vm.decode(0x2076u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102af9dcu64) }
        0x102af9dc => { vm.decode(0xa0808b35u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_sub(vm.ld(s2) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102af9e0u64) }
        0x102af9e0 => { vm.decode(0x8a21c909u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102af9e4u64) }
        0x102af9e4 => { vm.decode(0x8b80e35u32); let d = 1i16; let s = 2i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s2) < vm.ld(s)) as u64;
        vm.st(d, v);
        Ok(0x102af9e8u64) }
        0x102af9e8 => { vm.decode(0x87410531u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 6632i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102af9ecu64).wrapping_add((imm * 4) as u64) } else { 0x102af9ecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102af9ecu64) }
        0x102af9ec => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102af9f0u64) } }
        0x102af9f0 => { vm.decode(0x8920dd30u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102af9f4u64) }
        0x102afef8 => { vm.decode(0x641d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102afefcu64) }
        0x102afefc => { vm.decode(0x193094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102aff00u64) }
        0x102aff00 => { vm.decode(0x640d8519u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102aff04u64) }
        0x102aff04 => { vm.decode(0x192894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102aff08u64) }
        0x102aff08 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102aff0cu64) }
        0x102aff0c => { vm.decode(0x405673d9u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 12797i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102aff10u64) }
        0x102aff10 => { vm.decode(0x12100019u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102aff14u64) }
        0x102aff14 => { vm.decode(0x640d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102aff18u64) }
        0x102aff18 => { vm.decode(0x1b2014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102aff1cu64) }
        0x102aff1c => { vm.decode(0x4100236u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102aff20u64) }
        0x102aff20 => { vm.decode(0x32028000u32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102aff24u64) }
        0x102aff24 => { vm.decode(0x48c08a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 182i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102aff28u64).wrapping_add((imm * 4) as u64) } else { 0x102aff28u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102aff28u64) }
        0x102aff28 => { vm.decode(0x92095b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102aff2cu64) }
        0x102aff2c => { vm.decode(0xa640511du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 175492i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102aff30u64) }
        0x102aff30 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102aff34u64) } }
        0x102b1104 => { vm.decode(0xf0027u32); let d = 0i16; let s = 21i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b1108u64).wrapping_add((imm * 4) as u64) } else { 0x102b1108u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b1108u64) }
        0x102b1108 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b110cu64) } }
        0x102b110c => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b1110u64) }
        0x102b1110 => { vm.decode(0x2000016fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b1114u64) }
        0x102b1114 => { vm.decode(0x99215cb0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b1118u64) }
        0x102b1118 => { vm.decode(0x99395d30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b111cu64) }
        0x102b111c => { vm.decode(0x19291c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b1120u64) }
        0x102b1120 => { vm.decode(0xd1c4811u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102b1124u64) }
        0x102b1124 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102b1128u64) }
        0x102b1128 => { vm.decode(0xa88636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b112cu64) }
        0x102b112c => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102b1130u64) }
        0x102b1130 => { vm.decode(0x40448019u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b1134u64) }
        0x102b1134 => { vm.decode(0x19311c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b1138u64) }
        0x102b1138 => { vm.decode(0x886632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b113cu64) }
        0x102b113c => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b1140u64) }
        0x102b1140 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102b1144u64 + 4);
        vm.wr(37, 0x102b1144u64 + 4);
        Ok(0x102b1144u64) }
        0x102b1144 => { vm.decode(0x620d8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b1148u64) }
        0x102b1148 => { vm.decode(0x1510e211u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102b114cu64) }
        0x102b114c => { vm.decode(0x10560d35u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 21i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102b1150u64) }
        0x102b1150 => { vm.decode(0xd20a4bf3u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1867i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102b1154u64) }
        0x102b1154 => { vm.decode(0x50540d35u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 13i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102b1158u64) }
        0x102b1158 => { vm.decode(0x520a4bf3u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1866i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102b115cu64) }
        0x102b115c => { vm.decode(0x10540d35u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102b1160u64) }
        0x102b1160 => { vm.decode(0xd20a4b73u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1865i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102b1164u64) }
        0x102b1164 => { vm.decode(0x44028bb5u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b1168u64) }
        0x102b1168 => { vm.decode(0x520a4b73u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102b116cu64) }
        0x102b116c => { vm.decode(0x4a8263afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b1170u64) }
        0x102b1170 => { vm.decode(0x2884402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b1174u64) }
        0x102b1174 => { vm.decode(0x10001119u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b1178u64) }
        0x102b1178 => { vm.decode(0x8b211530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b117cu64) }
        0x102b117c => { vm.decode(0x888436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b1180u64) }
        0x102b1180 => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b1184u64) }
        0x102b1184 => { vm.decode(0x28863efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b1188u64) }
        0x102b1188 => { vm.decode(0xb1115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b118cu64) }
        0x102b118c => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b1190u64) }
        0x102b1190 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b1194u64) }
        0x102b1194 => { vm.decode(0x688c43afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b1198u64) }
        0x102b1198 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102b119cu64) }
        0x102b119c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102b11a0u64 + 4);
        vm.wr(37, 0x102b11a0u64 + 4);
        Ok(0x102b11a0u64) }
        0x102b11a0 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b11a4u64) }
        0x102b11a4 => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b11a8u64) }
        0x102b11a8 => { vm.decode(0x760f9119u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b11acu64) }
        0x102b11ac => { vm.decode(0x2280406fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b11b0u64) }
        0x102b11b0 => { vm.decode(0x8b2115b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b11b4u64) }
        0x102b11b4 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102b11b8u64) }
        0x102b11b8 => { vm.decode(0x9b315530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b11bcu64) }
        0x102b11bc => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b11c0u64) }
        0x102b11c0 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b11c4u64) }
        0x102b11c4 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102b11c8u64) }
        0x102b11c8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102b11ccu64 + 4);
        vm.wr(37, 0x102b11ccu64 + 4);
        Ok(0x102b11ccu64) }
        0x102b11cc => { vm.decode(0x102036u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b11d0u64) }
        0x102b11d0 => { vm.decode(0x50ea012du32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 63i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102b11d4u64) }
        0x102b11d4 => { vm.decode(0x4018676u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 124i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b11d8u64) }
        0x102b11d8 => { vm.decode(0x60080b35u32); let d = 16i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b11dcu64) }
        0x102b11dc => { vm.decode(0x4018636u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 60i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b11e0u64) }
        0x102b11e0 => { vm.decode(0x20280b35u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b11e4u64) }
        0x102b11e4 => { vm.decode(0x30004640u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 60i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102b11e8u64) }
        0x102b11e8 => { vm.decode(0x40032f5u32); let d = 16i16; let s = 2i16; let s2 = 1i16; let imm = 0i64;
        if vm.ld(s2) != 0 {
            let v = vm.ld(s);
            vm.st(d, v);
        }
        Ok(0x102b11ecu64) }
        0x102b11ec => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b11f0u64) }
        0x102b11f0 => { vm.decode(0x54005299u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = 12801i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b11f4u64) }
        0x102b11f4 => { vm.decode(0x10049019u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b11f8u64) }
        0x102b11f8 => { vm.decode(0x770d1419u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b11fcu64) }
        0x102b11fc => { vm.decode(0x24000486u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102b1200u64) }
        0x102b1200 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102b1204u64).wrapping_add((imm * 4) as u64) } else { 0x102b1204u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b1204u64) }
        0x102b1204 => { vm.decode(0x2110036u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b1208u64) }
        0x102b1210 => { vm.decode(0x8624735u32); let d = 1i16; let s = 4i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b1214u64) }
        0x102b1214 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102b1218u64) }
        0x102b1218 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102b121cu64) }
        0x102b121c => { vm.decode(0x74304675u32); let d = 3i16; let s = 3i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b1220u64) }
        0x102b1220 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102b1224u64) }
        0x102b1224 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102b1228u64) }
        0x102b1228 => { vm.decode(0xab2124f5u32); let d = 2i16; let s = 21i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102b122cu64) }
        0x102b122c => { vm.decode(0x220033u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102b1230u64) }
        0x102b1230 => { vm.decode(0x8903036u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b1234u64) }
        0x102b1234 => { vm.decode(0x24000486u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102b1238u64) }
        0x102b1238 => { vm.decode(0x4040a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 42i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b123cu64).wrapping_add((imm * 4) as u64) } else { 0x102b123cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b123cu64) }
        0x102b123c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b1240u64) } }
        0x102b1240 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b1244u64) }
        0x102b1244 => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b1248u64) }
        0x102b1248 => { vm.decode(0x8cc3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b124cu64) }
        0x102b124c => { vm.decode(0x9b191530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b1250u64) }
        0x102b1250 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b1254u64) }
        0x102b1254 => { vm.decode(0x434004f5u32); let d = 1i16; let s = 2i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102b1258u64) }
        0x102b1258 => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b125cu64) }
        0x102b125c => { vm.decode(0xc0860bb5u32); let d = 21i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b1260u64) }
        0x102b1260 => { vm.decode(0x228063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b1264u64) }
        0x102b1264 => { vm.decode(0xa16004f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102b1268u64) }
        0x102b1268 => { vm.decode(0xa04023eu32); let d = 22i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102b126cu64) }
        0x102b126c => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b1270u64) }
        0x102b1270 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b1274u64) }
        0x102b1274 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102b1278u64 + 4);
        vm.wr(37, 0x102b1278u64 + 4);
        Ok(0x102b1278u64) }
        0x102b1278 => { vm.decode(0xb0d875u32); let d = 5i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102b127cu64) }
        0x102b127c => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102b1280u64) }
        0x102b1280 => { vm.decode(0x28c40b35u32); let d = 4i16; let s = 3i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b1284u64) }
        0x102b1284 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b1288u64) }
        0x102b1288 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102b128cu64) }
        0x102b128c => { vm.decode(0x40227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b1290u64).wrapping_add((imm * 4) as u64) } else { 0x102b1290u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b1290u64) }
        0x102b1290 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b1294u64) }
        0x102b1294 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b1298u64) }
        0x102b1298 => { vm.decode(0xd2ca412du32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102b129cu64) }
        0x102b129c => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b12a0u64) }
        0x102b12a0 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102b12a4u64) }
        0x102b12a4 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102b12a8u64) }
        0x102b12a8 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102b12acu64).wrapping_add((imm * 4) as u64) } else { 0x102b12acu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b12acu64) }
        0x102b12ac => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b12b0u64) } }
        0x102b12b0 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b12b4u64) }
        0x102b12b4 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102b12b8u64) }
        0x102b12b8 => { vm.decode(0x739cb4a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -43i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b12bcu64).wrapping_add((imm * 4) as u64) } else { 0x102b12bcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b12bcu64) }
        0x102b12bc => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b12c0u64) }
        0x102b12c0 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b12c4u64) }
        0x102b12c4 => { vm.decode(0xd2ca816du32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102b12c8u64) }
        0x102b12c8 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b12ccu64) }
        0x102b12cc => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102b12d0u64) }
        0x102b12d0 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102b12d4u64) }
        0x102b12d4 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102b12d8u64).wrapping_add((imm * 4) as u64) } else { 0x102b12d8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b12d8u64) }
        0x102b12d8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b12dcu64) } }
        0x102b12dc => { vm.decode(0x86c0451du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 175760i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102b12e0u64) }
        0x102b12e0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b12e4u64) } }
        0x102b12e4 => { vm.decode(0x408e6befu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b12e8u64) }
        0x102b12e8 => { vm.decode(0x628243efu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b12ecu64) }
        0x102b12ec => { vm.decode(0x529a9ca7u32); let d = 0i16; let s = 16i16; let s2 = -1i16; let imm = 14295i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b12f0u64).wrapping_add((imm * 4) as u64) } else { 0x102b12f0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b12f0u64) }
        0x102b12f0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b12f4u64) } }
        0x102b12f4 => { vm.decode(0xd10e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102b12f8u64) }
        0x102b12f8 => { vm.decode(0xa4000fb5u32); let d = 2i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b12fcu64) }
        0x102b12fc => { vm.decode(0x21d5062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102b1300u64) }
        0x102b1300 => { vm.decode(0x20082e35u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) < vm.ld(s)) as u64;
        vm.st(d, v);
        Ok(0x102b1304u64) }
        0x102b1304 => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b1308u64).wrapping_add((imm * 4) as u64) } else { 0x102b1308u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b1308u64) }
        0x102b1308 => { vm.decode(0x50da812du32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 63i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102b130cu64) }
        0x102b1318 => { vm.decode(0x80c08a7u32); let d = 0i16; let s = 3i16; let s2 = -1i16; let imm = 146i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b131cu64).wrapping_add((imm * 4) as u64) } else { 0x102b131cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b131cu64) }
        0x102b131c => { vm.decode(0x8b654909u32); let d = 22i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102b1320u64) }
        0x102b1320 => { vm.decode(0x2076u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b1324u64) }
        0x102b1324 => { vm.decode(0xa0808b35u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_sub(vm.ld(s2) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b1328u64) }
        0x102b1328 => { vm.decode(0x8a21c909u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102b132cu64) }
        0x102b132c => { vm.decode(0x8b80e35u32); let d = 1i16; let s = 2i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s2) < vm.ld(s)) as u64;
        vm.st(d, v);
        Ok(0x102b1330u64) }
        0x102b1330 => { vm.decode(0x87422871u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 5295i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102b1334u64).wrapping_add((imm * 4) as u64) } else { 0x102b1334u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b1334u64) }
        0x102b1334 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b1338u64) } }
        0x102b1848 => { vm.decode(0x641d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b184cu64) }
        0x102b184c => { vm.decode(0x192894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b1850u64) }
        0x102b1850 => { vm.decode(0x640d8519u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b1854u64) }
        0x102b1854 => { vm.decode(0x192094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b1858u64) }
        0x102b1858 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b185cu64) }
        0x102b185c => { vm.decode(0x4c645619u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = -16872i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b1860u64) }
        0x102b1860 => { vm.decode(0x10109019u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b1864u64) }
        0x102b1864 => { vm.decode(0x640d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b1868u64) }
        0x102b1868 => { vm.decode(0x1b2014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b186cu64) }
        0x102b186c => { vm.decode(0x6100036u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b1870u64) }
        0x102b1870 => { vm.decode(0x32020000u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102b1874u64) }
        0x102b1874 => { vm.decode(0x48c88a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 183i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b1878u64).wrapping_add((imm * 4) as u64) } else { 0x102b1878u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b1878u64) }
        0x102b1878 => { vm.decode(0x9309530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b187cu64) }
        0x102b187c => { vm.decode(0x86e0841du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 181968i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102b1880u64) }
        0x102b1880 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b1884u64) } }
        0x102b1884 => { vm.decode(0x110039b5u32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102b1888u64) }
        0x102b2a50 => { vm.decode(0xe0027u32); let d = 0i16; let s = 17i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b2a54u64).wrapping_add((imm * 4) as u64) } else { 0x102b2a54u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b2a54u64) }
        0x102b2a54 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b2a58u64) } }
        0x102b2a58 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b2a5cu64) }
        0x102b2a5c => { vm.decode(0x2000016fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b2a60u64) }
        0x102b2a60 => { vm.decode(0x99211cb0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b2a64u64) }
        0x102b2a64 => { vm.decode(0x89395d30u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b2a68u64) }
        0x102b2a68 => { vm.decode(0x19291c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b2a6cu64) }
        0x102b2a6c => { vm.decode(0xd1c4811u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102b2a70u64) }
        0x102b2a70 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102b2a74u64) }
        0x102b2a74 => { vm.decode(0x888436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b2a78u64) }
        0x102b2a78 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102b2a7cu64) }
        0x102b2a7c => { vm.decode(0x40448019u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b2a80u64) }
        0x102b2a80 => { vm.decode(0x19311c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b2a84u64) }
        0x102b2a84 => { vm.decode(0x886632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b2a88u64) }
        0x102b2a88 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b2a8cu64) }
        0x102b2a8c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102b2a90u64 + 4);
        vm.wr(37, 0x102b2a90u64 + 4);
        Ok(0x102b2a90u64) }
        0x102b2a90 => { vm.decode(0x620d8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b2a94u64) }
        0x102b2a94 => { vm.decode(0x1510e211u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102b2a98u64) }
        0x102b2a98 => { vm.decode(0x10560d35u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 21i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102b2a9cu64) }
        0x102b2a9c => { vm.decode(0xd20a4bf3u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1867i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102b2aa0u64) }
        0x102b2aa0 => { vm.decode(0x50540d35u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 13i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102b2aa4u64) }
        0x102b2aa4 => { vm.decode(0x520a4bf3u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1866i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102b2aa8u64) }
        0x102b2aa8 => { vm.decode(0x10540d35u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102b2aacu64) }
        0x102b2aac => { vm.decode(0xd20a4b73u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1865i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102b2ab0u64) }
        0x102b2ab0 => { vm.decode(0x44028bb5u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b2ab4u64) }
        0x102b2ab4 => { vm.decode(0x520a4b73u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102b2ab8u64) }
        0x102b2ab8 => { vm.decode(0x4a8263afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b2abcu64) }
        0x102b2abc => { vm.decode(0x2884402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b2ac0u64) }
        0x102b2ac0 => { vm.decode(0x10001119u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b2ac4u64) }
        0x102b2ac4 => { vm.decode(0x8b211530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b2ac8u64) }
        0x102b2ac8 => { vm.decode(0x88636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b2accu64) }
        0x102b2acc => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b2ad0u64) }
        0x102b2ad0 => { vm.decode(0x28863efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b2ad4u64) }
        0x102b2ad4 => { vm.decode(0xb1115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b2ad8u64) }
        0x102b2ad8 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b2adcu64) }
        0x102b2adc => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b2ae0u64) }
        0x102b2ae0 => { vm.decode(0x688c43afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b2ae4u64) }
        0x102b2ae4 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102b2ae8u64) }
        0x102b2ae8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102b2aecu64 + 4);
        vm.wr(37, 0x102b2aecu64 + 4);
        Ok(0x102b2aecu64) }
        0x102b2aec => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b2af0u64) }
        0x102b2af0 => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b2af4u64) }
        0x102b2af4 => { vm.decode(0x2280406fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b2af8u64) }
        0x102b2af8 => { vm.decode(0x8b2115b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b2afcu64) }
        0x102b2afc => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102b2b00u64) }
        0x102b2b00 => { vm.decode(0x28843efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b2b04u64) }
        0x102b2b04 => { vm.decode(0xb311530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b2b08u64) }
        0x102b2b08 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b2b0cu64) }
        0x102b2b0c => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b2b10u64) }
        0x102b2b10 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102b2b14u64) }
        0x102b2b14 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102b2b18u64 + 4);
        vm.wr(37, 0x102b2b18u64 + 4);
        Ok(0x102b2b18u64) }
        0x102b2b18 => { vm.decode(0x102036u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b2b1cu64) }
        0x102b2b1c => { vm.decode(0x50ea012du32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 63i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102b2b20u64) }
        0x102b2b20 => { vm.decode(0x4018676u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 124i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b2b24u64) }
        0x102b2b24 => { vm.decode(0x60080b35u32); let d = 16i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b2b28u64) }
        0x102b2b28 => { vm.decode(0x4018636u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 60i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b2b2cu64) }
        0x102b2b2c => { vm.decode(0x20280b35u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b2b30u64) }
        0x102b2b30 => { vm.decode(0x30004640u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 60i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102b2b34u64) }
        0x102b2b34 => { vm.decode(0x40032f5u32); let d = 16i16; let s = 2i16; let s2 = 1i16; let imm = 0i64;
        if vm.ld(s2) != 0 {
            let v = vm.ld(s);
            vm.st(d, v);
        }
        Ok(0x102b2b38u64) }
        0x102b2b38 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b2b3cu64) }
        0x102b2b3c => { vm.decode(0x5c647619u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = -16868i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b2b40u64) }
        0x102b2b40 => { vm.decode(0x10049019u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b2b44u64) }
        0x102b2b44 => { vm.decode(0x760d1419u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b2b48u64) }
        0x102b2b48 => { vm.decode(0x24000486u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102b2b4cu64) }
        0x102b2b4c => { vm.decode(0x2220031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 17i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102b2b50u64).wrapping_add((imm * 4) as u64) } else { 0x102b2b50u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b2b50u64) }
        0x102b2b50 => { vm.decode(0x2110036u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b2b54u64) }
        0x102b2b54 => { vm.decode(0x8b40c41du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 182456i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102b2b58u64) }
        0x102b2b5c => { vm.decode(0xa624735u32); let d = 1i16; let s = 4i16; let s2 = 23i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b2b60u64) }
        0x102b2b60 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102b2b64u64) }
        0x102b2b64 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102b2b68u64) }
        0x102b2b68 => { vm.decode(0x74314675u32); let d = 3i16; let s = 3i16; let s2 = 23i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b2b6cu64) }
        0x102b2b6c => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102b2b70u64) }
        0x102b2b70 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102b2b74u64) }
        0x102b2b74 => { vm.decode(0x28843efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b2b78u64) }
        0x102b2b78 => { vm.decode(0x8b2004f5u32); let d = 2i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102b2b7cu64) }
        0x102b2b7c => { vm.decode(0x220033u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102b2b80u64) }
        0x102b2b80 => { vm.decode(0x8903036u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b2b84u64) }
        0x102b2b84 => { vm.decode(0x6a82636fu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b2b88u64) }
        0x102b2b88 => { vm.decode(0x24000486u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102b2b8cu64) }
        0x102b2b8c => { vm.decode(0x4040a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 42i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b2b90u64).wrapping_add((imm * 4) as u64) } else { 0x102b2b90u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b2b90u64) }
        0x102b2b90 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b2b94u64) } }
        0x102b2b94 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b2b98u64) }
        0x102b2b98 => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b2b9cu64) }
        0x102b2b9c => { vm.decode(0x8cc3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b2ba0u64) }
        0x102b2ba0 => { vm.decode(0x9b191530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b2ba4u64) }
        0x102b2ba4 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b2ba8u64) }
        0x102b2ba8 => { vm.decode(0x434004f5u32); let d = 1i16; let s = 2i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102b2bacu64) }
        0x102b2bac => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b2bb0u64) }
        0x102b2bb0 => { vm.decode(0xc0860bb5u32); let d = 21i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b2bb4u64) }
        0x102b2bb4 => { vm.decode(0x228063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b2bb8u64) }
        0x102b2bb8 => { vm.decode(0xa16004f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102b2bbcu64) }
        0x102b2bbc => { vm.decode(0x2a04023eu32); let d = 23i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102b2bc0u64) }
        0x102b2bc0 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b2bc4u64) }
        0x102b2bc4 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b2bc8u64) }
        0x102b2bc8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102b2bccu64 + 4);
        vm.wr(37, 0x102b2bccu64 + 4);
        Ok(0x102b2bccu64) }
        0x102b2bcc => { vm.decode(0x90d875u32); let d = 5i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102b2bd0u64) }
        0x102b2bd0 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102b2bd4u64) }
        0x102b2bd4 => { vm.decode(0x28c40b35u32); let d = 4i16; let s = 3i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b2bd8u64) }
        0x102b2bd8 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b2bdcu64) }
        0x102b2bdc => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102b2be0u64) }
        0x102b2be0 => { vm.decode(0x40227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b2be4u64).wrapping_add((imm * 4) as u64) } else { 0x102b2be4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b2be4u64) }
        0x102b2be4 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b2be8u64) }
        0x102b2be8 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b2becu64) }
        0x102b2bec => { vm.decode(0xd2ca412du32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102b2bf0u64) }
        0x102b2bf0 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b2bf4u64) }
        0x102b2bf4 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102b2bf8u64) }
        0x102b2bf8 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102b2bfcu64) }
        0x102b2bfc => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102b2c00u64).wrapping_add((imm * 4) as u64) } else { 0x102b2c00u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b2c00u64) }
        0x102b2c00 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b2c04u64) } }
        0x102b2c04 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b2c08u64) }
        0x102b2c08 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102b2c0cu64) }
        0x102b2c0c => { vm.decode(0x731cbca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -45i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b2c10u64).wrapping_add((imm * 4) as u64) } else { 0x102b2c10u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b2c10u64) }
        0x102b2c10 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b2c14u64) }
        0x102b2c14 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b2c18u64) }
        0x102b2c18 => { vm.decode(0xd2ca816du32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102b2c1cu64) }
        0x102b2c1c => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b2c20u64) }
        0x102b2c20 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102b2c24u64) }
        0x102b2c24 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102b2c28u64) }
        0x102b2c28 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102b2c2cu64).wrapping_add((imm * 4) as u64) } else { 0x102b2c2cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b2c2cu64) }
        0x102b2c2c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b2c30u64) } }
        0x102b2c30 => { vm.decode(0xaee0941du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 182236i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102b2c34u64) }
        0x102b2c34 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b2c38u64) } }
        0x102b2c38 => { vm.decode(0x158a8227u32); let d = 0i16; let s = 16i16; let s2 = -1i16; let imm = 16509i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b2c3cu64).wrapping_add((imm * 4) as u64) } else { 0x102b2c3cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b2c3cu64) }
        0x102b2c3c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b2c40u64) } }
        0x102b2c40 => { vm.decode(0xd10e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102b2c44u64) }
        0x102b2c44 => { vm.decode(0xa4000fb5u32); let d = 2i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b2c48u64) }
        0x102b2c48 => { vm.decode(0x21d5062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102b2c4cu64) }
        0x102b2c4c => { vm.decode(0x20082e35u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) < vm.ld(s)) as u64;
        vm.st(d, v);
        Ok(0x102b2c50u64) }
        0x102b2c50 => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b2c54u64).wrapping_add((imm * 4) as u64) } else { 0x102b2c54u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b2c54u64) }
        0x102b2c54 => { vm.decode(0x50da812du32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 63i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102b2c58u64) }
        0x102b2c58 => { vm.decode(0xd10e291u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 780i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102b2c5cu64) }
        0x102b2c64 => { vm.decode(0x80c08a7u32); let d = 0i16; let s = 3i16; let s2 = -1i16; let imm = 146i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b2c68u64).wrapping_add((imm * 4) as u64) } else { 0x102b2c68u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b2c68u64) }
        0x102b2c68 => { vm.decode(0x8b654909u32); let d = 22i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102b2c6cu64) }
        0x102b2c6c => { vm.decode(0x2076u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b2c70u64) }
        0x102b2c70 => { vm.decode(0xa0808b35u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_sub(vm.ld(s2) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b2c74u64) }
        0x102b2c74 => { vm.decode(0x8a21c909u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102b2c78u64) }
        0x102b2c78 => { vm.decode(0x8b80e35u32); let d = 1i16; let s = 2i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s2) < vm.ld(s)) as u64;
        vm.st(d, v);
        Ok(0x102b2c7cu64) }
        0x102b2c7c => { vm.decode(0x3330d71u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 3957i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102b2c80u64).wrapping_add((imm * 4) as u64) } else { 0x102b2c80u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b2c80u64) }
        0x102b2c80 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b2c84u64) } }
        0x102b2c84 => { vm.decode(0x8920dd30u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b2c88u64) }
        0x102b3190 => { vm.decode(0x641d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b3194u64) }
        0x102b3194 => { vm.decode(0x193894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b3198u64) }
        0x102b3198 => { vm.decode(0x640d8519u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b319cu64) }
        0x102b319c => { vm.decode(0x193094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b31a0u64) }
        0x102b31a0 => { vm.decode(0x8004e0d9u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 8463i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b31a4u64) }
        0x102b31a4 => { vm.decode(0xc002bd75u32); let d = 22i16; let s = 1i16; let s2 = -1i16; let imm = 18i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102b31a8u64) }
        0x102b31a8 => { vm.decode(0x2ca17d99u32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = -4539i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b31acu64) }
        0x102b31ac => { vm.decode(0x12100019u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b31b0u64) }
        0x102b31b0 => { vm.decode(0x640d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b31b4u64) }
        0x102b31b4 => { vm.decode(0x1b3014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b31b8u64) }
        0x102b31b8 => { vm.decode(0x4100236u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b31bcu64) }
        0x102b31bc => { vm.decode(0x32020000u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102b31c0u64) }
        0x102b31c0 => { vm.decode(0x40c82a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 185i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b31c4u64).wrapping_add((imm * 4) as u64) } else { 0x102b31c4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b31c4u64) }
        0x102b31c4 => { vm.decode(0x92895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b31c8u64) }
        0x102b31c8 => { vm.decode(0xb40851du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 188472i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102b31ccu64) }
        0x102b31cc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b31d0u64) } }
        0x102b43b8 => { vm.decode(0xe8027u32); let d = 0i16; let s = 17i16; let s2 = -1i16; let imm = 17i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b43bcu64).wrapping_add((imm * 4) as u64) } else { 0x102b43bcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b43bcu64) }
        0x102b43bc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b43c0u64) } }
        0x102b43c0 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b43c4u64) }
        0x102b43c4 => { vm.decode(0x2000016fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b43c8u64) }
        0x102b43c8 => { vm.decode(0x99211cb0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b43ccu64) }
        0x102b43cc => { vm.decode(0x88ce32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b43d0u64) }
        0x102b43d0 => { vm.decode(0x9391d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b43d4u64) }
        0x102b43d4 => { vm.decode(0x19291c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b43d8u64) }
        0x102b43d8 => { vm.decode(0xd984011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102b43dcu64) }
        0x102b43dc => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102b43e0u64) }
        0x102b43e0 => { vm.decode(0x88c436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b43e4u64) }
        0x102b43e4 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102b43e8u64) }
        0x102b43e8 => { vm.decode(0x40448019u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b43ecu64) }
        0x102b43ec => { vm.decode(0x19311c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b43f0u64) }
        0x102b43f0 => { vm.decode(0x886632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b43f4u64) }
        0x102b43f4 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b43f8u64) }
        0x102b43f8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102b43fcu64 + 4);
        vm.wr(37, 0x102b43fcu64 + 4);
        Ok(0x102b43fcu64) }
        0x102b43fc => { vm.decode(0x620d8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b4400u64) }
        0x102b4400 => { vm.decode(0x1510e211u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102b4404u64) }
        0x102b4404 => { vm.decode(0x10560d35u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 21i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102b4408u64) }
        0x102b4408 => { vm.decode(0xd20a4bf3u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1867i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102b440cu64) }
        0x102b440c => { vm.decode(0x50540d35u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 13i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102b4410u64) }
        0x102b4410 => { vm.decode(0x520a4bf3u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1866i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102b4414u64) }
        0x102b4414 => { vm.decode(0x10540d35u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102b4418u64) }
        0x102b4418 => { vm.decode(0xd20a4b73u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1865i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102b441cu64) }
        0x102b441c => { vm.decode(0x44028bb5u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b4420u64) }
        0x102b4420 => { vm.decode(0x520a4b73u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102b4424u64) }
        0x102b4424 => { vm.decode(0x4a8263afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b4428u64) }
        0x102b4428 => { vm.decode(0x2884402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b442cu64) }
        0x102b442c => { vm.decode(0x10001119u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b4430u64) }
        0x102b4430 => { vm.decode(0x8b211530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b4434u64) }
        0x102b4434 => { vm.decode(0xa88436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b4438u64) }
        0x102b4438 => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b443cu64) }
        0x102b443c => { vm.decode(0x28863efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b4440u64) }
        0x102b4440 => { vm.decode(0xb1115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b4444u64) }
        0x102b4444 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b4448u64) }
        0x102b4448 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b444cu64) }
        0x102b444c => { vm.decode(0x688c43afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b4450u64) }
        0x102b4450 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102b4454u64) }
        0x102b4454 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102b4458u64 + 4);
        vm.wr(37, 0x102b4458u64 + 4);
        Ok(0x102b4458u64) }
        0x102b4458 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b445cu64) }
        0x102b445c => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b4460u64) }
        0x102b4460 => { vm.decode(0x760f9119u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b4464u64) }
        0x102b4464 => { vm.decode(0x2280406fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b4468u64) }
        0x102b4468 => { vm.decode(0x8b2115b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b446cu64) }
        0x102b446c => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102b4470u64) }
        0x102b4470 => { vm.decode(0x9b315530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b4474u64) }
        0x102b4474 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b4478u64) }
        0x102b4478 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b447cu64) }
        0x102b447c => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102b4480u64) }
        0x102b4480 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102b4484u64 + 4);
        vm.wr(37, 0x102b4484u64 + 4);
        Ok(0x102b4484u64) }
        0x102b4484 => { vm.decode(0x102036u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b4488u64) }
        0x102b4488 => { vm.decode(0x50ea012du32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 63i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102b448cu64) }
        0x102b448c => { vm.decode(0x4018676u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 124i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b4490u64) }
        0x102b4490 => { vm.decode(0x60080b35u32); let d = 16i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b4494u64) }
        0x102b4494 => { vm.decode(0x4018636u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 60i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b4498u64) }
        0x102b4498 => { vm.decode(0x20280b35u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b449cu64) }
        0x102b449c => { vm.decode(0x30004640u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 60i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102b44a0u64) }
        0x102b44a0 => { vm.decode(0x40032f5u32); let d = 16i16; let s = 2i16; let s2 = 1i16; let imm = 0i64;
        if vm.ld(s2) != 0 {
            let v = vm.ld(s);
            vm.st(d, v);
        }
        Ok(0x102b44a4u64) }
        0x102b44a4 => { vm.decode(0x8004e0d9u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 8463i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b44a8u64) }
        0x102b44a8 => { vm.decode(0x80422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 18i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102b44acu64) }
        0x102b44ac => { vm.decode(0x5c245d99u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = -4535i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b44b0u64) }
        0x102b44b0 => { vm.decode(0x10049019u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b44b4u64) }
        0x102b44b4 => { vm.decode(0x770d1419u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b44b8u64) }
        0x102b44b8 => { vm.decode(0x408e43efu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b44bcu64) }
        0x102b44bc => { vm.decode(0x24000486u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102b44c0u64) }
        0x102b44c0 => { vm.decode(0x2200031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102b44c4u64).wrapping_add((imm * 4) as u64) } else { 0x102b44c4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b44c4u64) }
        0x102b44c4 => { vm.decode(0x2110036u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b44c8u64) }
        0x102b44d0 => { vm.decode(0x8624735u32); let d = 1i16; let s = 4i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b44d4u64) }
        0x102b44d4 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102b44d8u64) }
        0x102b44d8 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102b44dcu64) }
        0x102b44dc => { vm.decode(0x74304675u32); let d = 3i16; let s = 3i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b44e0u64) }
        0x102b44e0 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102b44e4u64) }
        0x102b44e4 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102b44e8u64) }
        0x102b44e8 => { vm.decode(0xab2124f5u32); let d = 2i16; let s = 21i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102b44ecu64) }
        0x102b44ec => { vm.decode(0x220033u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102b44f0u64) }
        0x102b44f0 => { vm.decode(0x8903036u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b44f4u64) }
        0x102b44f4 => { vm.decode(0x408e43efu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b44f8u64) }
        0x102b44f8 => { vm.decode(0x24000486u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102b44fcu64) }
        0x102b44fc => { vm.decode(0x4040a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 42i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b4500u64).wrapping_add((imm * 4) as u64) } else { 0x102b4500u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b4500u64) }
        0x102b4500 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b4504u64) } }
        0x102b4504 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b4508u64) }
        0x102b4508 => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b450cu64) }
        0x102b450c => { vm.decode(0x8cc3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b4510u64) }
        0x102b4510 => { vm.decode(0x9b191530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b4514u64) }
        0x102b4514 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b4518u64) }
        0x102b4518 => { vm.decode(0x434004f5u32); let d = 1i16; let s = 2i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102b451cu64) }
        0x102b451c => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b4520u64) }
        0x102b4520 => { vm.decode(0xc0860bb5u32); let d = 21i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b4524u64) }
        0x102b4524 => { vm.decode(0x228063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b4528u64) }
        0x102b4528 => { vm.decode(0xa16004f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102b452cu64) }
        0x102b452c => { vm.decode(0xa04023eu32); let d = 22i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102b4530u64) }
        0x102b4530 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b4534u64) }
        0x102b4534 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b4538u64) }
        0x102b4538 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102b453cu64 + 4);
        vm.wr(37, 0x102b453cu64 + 4);
        Ok(0x102b453cu64) }
        0x102b453c => { vm.decode(0xb0d875u32); let d = 5i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102b4540u64) }
        0x102b4540 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102b4544u64) }
        0x102b4544 => { vm.decode(0x28c40b35u32); let d = 4i16; let s = 3i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b4548u64) }
        0x102b4548 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b454cu64) }
        0x102b454c => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102b4550u64) }
        0x102b4550 => { vm.decode(0x40227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b4554u64).wrapping_add((imm * 4) as u64) } else { 0x102b4554u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b4554u64) }
        0x102b4554 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b4558u64) }
        0x102b4558 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b455cu64) }
        0x102b455c => { vm.decode(0xd2ca412du32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102b4560u64) }
        0x102b4560 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b4564u64) }
        0x102b4564 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102b4568u64) }
        0x102b4568 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102b456cu64) }
        0x102b456c => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102b4570u64).wrapping_add((imm * 4) as u64) } else { 0x102b4570u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b4570u64) }
        0x102b4570 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b4574u64) } }
        0x102b4574 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b4578u64) }
        0x102b4578 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102b457cu64) }
        0x102b457c => { vm.decode(0x739c34a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -44i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b4580u64).wrapping_add((imm * 4) as u64) } else { 0x102b4580u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b4580u64) }
        0x102b4580 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b4584u64) }
        0x102b4584 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b4588u64) }
        0x102b4588 => { vm.decode(0xd2ca816du32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102b458cu64) }
        0x102b458c => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b4590u64) }
        0x102b4590 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102b4594u64) }
        0x102b4594 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102b4598u64) }
        0x102b4598 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102b459cu64).wrapping_add((imm * 4) as u64) } else { 0x102b459cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b459cu64) }
        0x102b459c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b45a0u64) } }
        0x102b45a0 => { vm.decode(0x260951du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 188752i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102b45a4u64) }
        0x102b45a4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b45a8u64) } }
        0x102b45a8 => { vm.decode(0x408e6befu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b45acu64) }
        0x102b45ac => { vm.decode(0x618a8a27u32); let d = 0i16; let s = 16i16; let s2 = -1i16; let imm = 18719i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b45b0u64).wrapping_add((imm * 4) as u64) } else { 0x102b45b0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b45b0u64) }
        0x102b45b0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b45b4u64) } }
        0x102b45b4 => { vm.decode(0xd10e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102b45b8u64) }
        0x102b45b8 => { vm.decode(0xa4000fb5u32); let d = 2i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b45bcu64) }
        0x102b45bc => { vm.decode(0x21d5062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102b45c0u64) }
        0x102b45c0 => { vm.decode(0x20082e35u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) < vm.ld(s)) as u64;
        vm.st(d, v);
        Ok(0x102b45c4u64) }
        0x102b45c4 => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b45c8u64).wrapping_add((imm * 4) as u64) } else { 0x102b45c8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b45c8u64) }
        0x102b45c8 => { vm.decode(0x50da812du32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 63i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102b45ccu64) }
        0x102b45d8 => { vm.decode(0x80c08a7u32); let d = 0i16; let s = 3i16; let s2 = -1i16; let imm = 146i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b45dcu64).wrapping_add((imm * 4) as u64) } else { 0x102b45dcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b45dcu64) }
        0x102b45dc => { vm.decode(0x89674909u32); let d = 21i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102b45e0u64) }
        0x102b45e0 => { vm.decode(0x2076u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b45e4u64) }
        0x102b45e4 => { vm.decode(0xa0808b35u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_sub(vm.ld(s2) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b45e8u64) }
        0x102b45e8 => { vm.decode(0x8a21c909u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102b45ecu64) }
        0x102b45ec => { vm.decode(0xcb00e35u32); let d = 1i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = (vm.ld(s2) < vm.ld(s)) as u64;
        vm.st(d, v);
        Ok(0x102b45f0u64) }
        0x102b45f0 => { vm.decode(0x3310031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 2608i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102b45f4u64).wrapping_add((imm * 4) as u64) } else { 0x102b45f4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b45f4u64) }
        0x102b45f4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b45f8u64) } }
        0x102b4b08 => { vm.decode(0x641d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b4b0cu64) }
        0x102b4b0c => { vm.decode(0x193094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b4b10u64) }
        0x102b4b10 => { vm.decode(0x640d8519u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b4b14u64) }
        0x102b4b14 => { vm.decode(0x192894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b4b18u64) }
        0x102b4b18 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b4b1cu64) }
        0x102b4b1c => { vm.decode(0xcc407059u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = -23786i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b4b20u64) }
        0x102b4b20 => { vm.decode(0x10109019u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b4b24u64) }
        0x102b4b24 => { vm.decode(0x640d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b4b28u64) }
        0x102b4b28 => { vm.decode(0x1b2014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b4b2cu64) }
        0x102b4b2c => { vm.decode(0x6102236u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b4b30u64) }
        0x102b4b30 => { vm.decode(0x22028000u32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102b4b34u64) }
        0x102b4b34 => { vm.decode(0x40c82a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 185i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b4b38u64).wrapping_add((imm * 4) as u64) } else { 0x102b4b38u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b4b38u64) }
        0x102b4b38 => { vm.decode(0x92095b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b4b3cu64) }
        0x102b4b3c => { vm.decode(0x8a44d51du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 194968i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102b4b40u64) }
        0x102b4b40 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b4b44u64) } }
        0x102b4b44 => { vm.decode(0x100039b5u32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102b4b48u64) }
        0x102b5d18 => { vm.decode(0x6a8a636fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b5d1cu64) }
        0x102b5d1c => { vm.decode(0xb0027u32); let d = 0i16; let s = 20i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b5d20u64).wrapping_add((imm * 4) as u64) } else { 0x102b5d20u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b5d20u64) }
        0x102b5d20 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b5d24u64) } }
        0x102b5d24 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b5d28u64) }
        0x102b5d28 => { vm.decode(0x2000016fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b5d2cu64) }
        0x102b5d2c => { vm.decode(0x89215cb0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b5d30u64) }
        0x102b5d30 => { vm.decode(0x888c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b5d34u64) }
        0x102b5d34 => { vm.decode(0x9391d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b5d38u64) }
        0x102b5d38 => { vm.decode(0x19291c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b5d3cu64) }
        0x102b5d3c => { vm.decode(0xd1c4811u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102b5d40u64) }
        0x102b5d40 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102b5d44u64) }
        0x102b5d44 => { vm.decode(0xa24124f5u32); let d = 1i16; let s = 1i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102b5d48u64) }
        0x102b5d48 => { vm.decode(0x40448019u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b5d4cu64) }
        0x102b5d4c => { vm.decode(0x19311c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b5d50u64) }
        0x102b5d50 => { vm.decode(0x886632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b5d54u64) }
        0x102b5d54 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b5d58u64) }
        0x102b5d58 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102b5d5cu64 + 4);
        vm.wr(37, 0x102b5d5cu64 + 4);
        Ok(0x102b5d5cu64) }
        0x102b5d5c => { vm.decode(0x620d8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b5d60u64) }
        0x102b5d60 => { vm.decode(0x1510e211u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102b5d64u64) }
        0x102b5d64 => { vm.decode(0x10560d35u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 21i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102b5d68u64) }
        0x102b5d68 => { vm.decode(0xd20a4bf3u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1867i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102b5d6cu64) }
        0x102b5d6c => { vm.decode(0x50540d35u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 13i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102b5d70u64) }
        0x102b5d70 => { vm.decode(0x520a4bf3u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1866i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102b5d74u64) }
        0x102b5d74 => { vm.decode(0x10540d35u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102b5d78u64) }
        0x102b5d78 => { vm.decode(0xd20a4b73u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1865i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102b5d7cu64) }
        0x102b5d7c => { vm.decode(0x44028bb5u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b5d80u64) }
        0x102b5d80 => { vm.decode(0x520a4b73u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102b5d84u64) }
        0x102b5d84 => { vm.decode(0x4a8263afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b5d88u64) }
        0x102b5d88 => { vm.decode(0x2884402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b5d8cu64) }
        0x102b5d8c => { vm.decode(0x10001119u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b5d90u64) }
        0x102b5d90 => { vm.decode(0x8b211530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b5d94u64) }
        0x102b5d94 => { vm.decode(0x888436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b5d98u64) }
        0x102b5d98 => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b5d9cu64) }
        0x102b5d9c => { vm.decode(0x28863efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b5da0u64) }
        0x102b5da0 => { vm.decode(0xb1115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b5da4u64) }
        0x102b5da4 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b5da8u64) }
        0x102b5da8 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b5dacu64) }
        0x102b5dac => { vm.decode(0x688c43afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b5db0u64) }
        0x102b5db0 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102b5db4u64) }
        0x102b5db4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102b5db8u64 + 4);
        vm.wr(37, 0x102b5db8u64 + 4);
        Ok(0x102b5db8u64) }
        0x102b5db8 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b5dbcu64) }
        0x102b5dbc => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b5dc0u64) }
        0x102b5dc0 => { vm.decode(0x640f1119u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b5dc4u64) }
        0x102b5dc4 => { vm.decode(0x2280406fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b5dc8u64) }
        0x102b5dc8 => { vm.decode(0x8b2115b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b5dccu64) }
        0x102b5dcc => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102b5dd0u64) }
        0x102b5dd0 => { vm.decode(0x9389d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b5dd4u64) }
        0x102b5dd4 => { vm.decode(0xb311530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b5dd8u64) }
        0x102b5dd8 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b5ddcu64) }
        0x102b5ddc => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b5de0u64) }
        0x102b5de0 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102b5de4u64) }
        0x102b5de4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102b5de8u64 + 4);
        vm.wr(37, 0x102b5de8u64 + 4);
        Ok(0x102b5de8u64) }
        0x102b5de8 => { vm.decode(0x102036u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b5decu64) }
        0x102b5dec => { vm.decode(0x50ea012du32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 63i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102b5df0u64) }
        0x102b5df0 => { vm.decode(0x4018676u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 124i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b5df4u64) }
        0x102b5df4 => { vm.decode(0x60080b35u32); let d = 16i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b5df8u64) }
        0x102b5df8 => { vm.decode(0x4018636u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 60i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b5dfcu64) }
        0x102b5dfc => { vm.decode(0x20280b35u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b5e00u64) }
        0x102b5e00 => { vm.decode(0x30004640u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 60i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102b5e04u64) }
        0x102b5e04 => { vm.decode(0x40032f5u32); let d = 16i16; let s = 2i16; let s2 = 1i16; let imm = 0i64;
        if vm.ld(s2) != 0 {
            let v = vm.ld(s);
            vm.st(d, v);
        }
        Ok(0x102b5e08u64) }
        0x102b5e08 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b5e0cu64) }
        0x102b5e0c => { vm.decode(0xdc445059u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = -23782i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b5e10u64) }
        0x102b5e10 => { vm.decode(0x10049019u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b5e14u64) }
        0x102b5e14 => { vm.decode(0x770d1419u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102b5e18u64) }
        0x102b5e18 => { vm.decode(0x24000486u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102b5e1cu64) }
        0x102b5e1c => { vm.decode(0x2220031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 17i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102b5e20u64).wrapping_add((imm * 4) as u64) } else { 0x102b5e20u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b5e20u64) }
        0x102b5e20 => { vm.decode(0x2110036u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b5e24u64) }
        0x102b5e24 => { vm.decode(0x8ac4d11du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 195464i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102b5e28u64) }
        0x102b5e2c => { vm.decode(0x8624735u32); let d = 1i16; let s = 4i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b5e30u64) }
        0x102b5e30 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102b5e34u64) }
        0x102b5e34 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102b5e38u64) }
        0x102b5e38 => { vm.decode(0x74304675u32); let d = 3i16; let s = 3i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b5e3cu64) }
        0x102b5e3c => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102b5e40u64) }
        0x102b5e40 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102b5e44u64) }
        0x102b5e44 => { vm.decode(0xa88e36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b5e48u64) }
        0x102b5e48 => { vm.decode(0x8b2004f5u32); let d = 2i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102b5e4cu64) }
        0x102b5e4c => { vm.decode(0x220033u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102b5e50u64) }
        0x102b5e50 => { vm.decode(0x8903036u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b5e54u64) }
        0x102b5e54 => { vm.decode(0x628243efu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b5e58u64) }
        0x102b5e58 => { vm.decode(0x24000486u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102b5e5cu64) }
        0x102b5e5c => { vm.decode(0x4040a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 42i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b5e60u64).wrapping_add((imm * 4) as u64) } else { 0x102b5e60u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b5e60u64) }
        0x102b5e60 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b5e64u64) } }
        0x102b5e64 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b5e68u64) }
        0x102b5e68 => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b5e6cu64) }
        0x102b5e6c => { vm.decode(0x8cc3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b5e70u64) }
        0x102b5e70 => { vm.decode(0x9b191530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b5e74u64) }
        0x102b5e74 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b5e78u64) }
        0x102b5e78 => { vm.decode(0x434004f5u32); let d = 1i16; let s = 2i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102b5e7cu64) }
        0x102b5e7c => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b5e80u64) }
        0x102b5e80 => { vm.decode(0xc0860bb5u32); let d = 21i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b5e84u64) }
        0x102b5e84 => { vm.decode(0x228063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b5e88u64) }
        0x102b5e88 => { vm.decode(0xa16004f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102b5e8cu64) }
        0x102b5e8c => { vm.decode(0xa04023eu32); let d = 22i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102b5e90u64) }
        0x102b5e90 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b5e94u64) }
        0x102b5e94 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b5e98u64) }
        0x102b5e98 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102b5e9cu64 + 4);
        vm.wr(37, 0x102b5e9cu64 + 4);
        Ok(0x102b5e9cu64) }
        0x102b5e9c => { vm.decode(0xb0d875u32); let d = 5i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102b5ea0u64) }
        0x102b5ea0 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102b5ea4u64) }
        0x102b5ea4 => { vm.decode(0x28c40b35u32); let d = 4i16; let s = 3i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b5ea8u64) }
        0x102b5ea8 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b5eacu64) }
        0x102b5eac => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102b5eb0u64) }
        0x102b5eb0 => { vm.decode(0x40227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b5eb4u64).wrapping_add((imm * 4) as u64) } else { 0x102b5eb4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b5eb4u64) }
        0x102b5eb4 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b5eb8u64) }
        0x102b5eb8 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b5ebcu64) }
        0x102b5ebc => { vm.decode(0xd2ca412du32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102b5ec0u64) }
        0x102b5ec0 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b5ec4u64) }
        0x102b5ec4 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102b5ec8u64) }
        0x102b5ec8 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102b5eccu64) }
        0x102b5ecc => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102b5ed0u64).wrapping_add((imm * 4) as u64) } else { 0x102b5ed0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b5ed0u64) }
        0x102b5ed0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b5ed4u64) } }
        0x102b5ed4 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b5ed8u64) }
        0x102b5ed8 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102b5edcu64) }
        0x102b5edc => { vm.decode(0x731cbca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -45i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b5ee0u64).wrapping_add((imm * 4) as u64) } else { 0x102b5ee0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b5ee0u64) }
        0x102b5ee0 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b5ee4u64) }
        0x102b5ee4 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b5ee8u64) }
        0x102b5ee8 => { vm.decode(0xd2ca816du32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102b5eecu64) }
        0x102b5eec => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b5ef0u64) }
        0x102b5ef0 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102b5ef4u64) }
        0x102b5ef4 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102b5ef8u64) }
        0x102b5ef8 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102b5efcu64).wrapping_add((imm * 4) as u64) } else { 0x102b5efcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b5efcu64) }
        0x102b5efc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b5f00u64) } }
        0x102b5f00 => { vm.decode(0xabc4c11du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 195244i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102b5f04u64) }
        0x102b5f04 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b5f08u64) } }
        0x102b5f08 => { vm.decode(0x408e6befu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102b5f0cu64) }
        0x102b5f0c => { vm.decode(0x510292a7u32); let d = 0i16; let s = 16i16; let s2 = -1i16; let imm = 20937i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b5f10u64).wrapping_add((imm * 4) as u64) } else { 0x102b5f10u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b5f10u64) }
        0x102b5f10 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b5f14u64) } }
        0x102b5f14 => { vm.decode(0xd10e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102b5f18u64) }
        0x102b5f18 => { vm.decode(0xa4000fb5u32); let d = 2i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b5f1cu64) }
        0x102b5f1c => { vm.decode(0x21d5062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102b5f20u64) }
        0x102b5f20 => { vm.decode(0x20082e35u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) < vm.ld(s)) as u64;
        vm.st(d, v);
        Ok(0x102b5f24u64) }
        0x102b5f24 => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b5f28u64).wrapping_add((imm * 4) as u64) } else { 0x102b5f28u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b5f28u64) }
        0x102b5f28 => { vm.decode(0x50da812du32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 63i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102b5f2cu64) }
        0x102b5f2c => { vm.decode(0xd10e291u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 780i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102b5f30u64) }
        0x102b5f38 => { vm.decode(0x80c08a7u32); let d = 0i16; let s = 3i16; let s2 = -1i16; let imm = 146i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102b5f3cu64).wrapping_add((imm * 4) as u64) } else { 0x102b5f3cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b5f3cu64) }
        0x102b5f3c => { vm.decode(0x8b654909u32); let d = 22i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102b5f40u64) }
        0x102b5f40 => { vm.decode(0x2076u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102b5f44u64) }
        0x102b5f44 => { vm.decode(0xa0808b35u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_sub(vm.ld(s2) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102b5f48u64) }
        0x102b5f48 => { vm.decode(0x8a21c909u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102b5f4cu64) }
        0x102b5f4c => { vm.decode(0x8b80e35u32); let d = 1i16; let s = 2i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s2) < vm.ld(s)) as u64;
        vm.st(d, v);
        Ok(0x102b5f50u64) }
        0x102b5f50 => { vm.decode(0x3602931u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1266i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102b5f54u64).wrapping_add((imm * 4) as u64) } else { 0x102b5f54u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102b5f54u64) }
        0x102b5f54 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102b5f58u64) } }
        0x102b5f58 => { vm.decode(0x8920dd30u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b5f5cu64) }
        0x102b618c => { vm.decode(0x63255022u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102b6190u64) }
        0x102b6190 => { vm.decode(0xa208841du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 213140i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102b6194u64) }
        0x102b6194 => { vm.decode(0x9928dd30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b6198u64) }
        0x102b6198 => { vm.decode(0xd9060d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1348i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102b619cu64) }
        0x102b65f0 => { vm.decode(0x63255022u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102b65f4u64) }
        0x102b65f4 => { vm.decode(0xf2cc51du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 228472i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102b65f8u64) }
        0x102b65f8 => { vm.decode(0x9928dd30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b65fcu64) }
        0x102b6a54 => { vm.decode(0x2a6c451du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 243804i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102b6a58u64) }
        0x102b6a58 => { vm.decode(0x63255022u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102b6a5cu64) }
        0x102b6a5c => { vm.decode(0xd9060d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1348i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102b6a60u64) }
        0x102b6eb4 => { vm.decode(0x43257022u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102b6eb8u64) }
        0x102b6eb8 => { vm.decode(0x2f4c851du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 259132i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102b6ebcu64) }
        0x102b6ebc => { vm.decode(0x9930dd30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102b6ec0u64) }
        0x102b731c => { vm.decode(0x124019du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 274528i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102b7320u64) }
        0x102b7320 => { vm.decode(0x63255022u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102b7324u64) }
        0x102b7324 => { vm.decode(0xd9060d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1348i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102b7328u64) }
        0x102ba414 => { vm.decode(0x641d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ba418u64) }
        0x102ba418 => { vm.decode(0x193894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ba41cu64) }
        0x102ba41c => { vm.decode(0x640d8519u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ba420u64) }
        0x102ba420 => { vm.decode(0x193094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ba424u64) }
        0x102ba424 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ba428u64) }
        0x102ba428 => { vm.decode(0x442237d9u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 8133i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ba42cu64) }
        0x102ba42c => { vm.decode(0x12109019u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ba430u64) }
        0x102ba430 => { vm.decode(0x640d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ba434u64) }
        0x102ba434 => { vm.decode(0x1b2014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ba438u64) }
        0x102ba438 => { vm.decode(0x4100236u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102ba43cu64) }
        0x102ba43c => { vm.decode(0xa2028000u32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102ba440u64) }
        0x102ba440 => { vm.decode(0x40c02a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 184i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102ba444u64).wrapping_add((imm * 4) as u64) } else { 0x102ba444u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ba444u64) }
        0x102ba444 => { vm.decode(0x92895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ba448u64) }
        0x102ba448 => { vm.decode(0x838c801du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 217760i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102ba44cu64) }
        0x102ba44c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102ba450u64) } }
        0x102ba450 => { vm.decode(0x100039b5u32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102ba454u64) }
        0x102bb620 => { vm.decode(0x888e36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bb624u64) }
        0x102bb624 => { vm.decode(0x80b0027u32); let d = 0i16; let s = 22i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bb628u64).wrapping_add((imm * 4) as u64) } else { 0x102bb628u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bb628u64) }
        0x102bb628 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bb62cu64) } }
        0x102bb62c => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bb630u64) }
        0x102bb630 => { vm.decode(0x2000016fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bb634u64) }
        0x102bb634 => { vm.decode(0x89215db0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bb638u64) }
        0x102bb638 => { vm.decode(0x9391d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bb63cu64) }
        0x102bb63c => { vm.decode(0x19291c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bb640u64) }
        0x102bb640 => { vm.decode(0xd9c4011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb644u64) }
        0x102bb644 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102bb648u64) }
        0x102bb648 => { vm.decode(0xa88636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bb64cu64) }
        0x102bb64c => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102bb650u64) }
        0x102bb650 => { vm.decode(0x40448019u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bb654u64) }
        0x102bb654 => { vm.decode(0x19311c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bb658u64) }
        0x102bb658 => { vm.decode(0x886632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bb65cu64) }
        0x102bb65c => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bb660u64) }
        0x102bb660 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102bb664u64 + 4);
        vm.wr(37, 0x102bb664u64 + 4);
        Ok(0x102bb664u64) }
        0x102bb664 => { vm.decode(0x620d8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bb668u64) }
        0x102bb668 => { vm.decode(0xd10e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb66cu64) }
        0x102bb66c => { vm.decode(0x9236u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bb670u64) }
        0x102bb670 => { vm.decode(0x21d5062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb674u64) }
        0x102bb674 => { vm.decode(0x20082e35u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) < vm.ld(s)) as u64;
        vm.st(d, v);
        Ok(0x102bb678u64) }
        0x102bb678 => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bb67cu64).wrapping_add((imm * 4) as u64) } else { 0x102bb67cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bb67cu64) }
        0x102bb67c => { vm.decode(0x50da812du32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 63i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102bb680u64) }
        0x102bb680 => { vm.decode(0xd10e291u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 780i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb684u64) }
        0x102bb68c => { vm.decode(0x12002019u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bb690u64) }
        0x102bb690 => { vm.decode(0x642d8159u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1864i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bb694u64) }
        0x102bb694 => { vm.decode(0x80c80a7u32); let d = 0i16; let s = 3i16; let s2 = -1i16; let imm = 145i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bb698u64).wrapping_add((imm * 4) as u64) } else { 0x102bb698u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bb698u64) }
        0x102bb698 => { vm.decode(0x19209c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bb69cu64) }
        0x102bb69c => { vm.decode(0x2076u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bb6a0u64) }
        0x102bb6a0 => { vm.decode(0xa0808b35u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_sub(vm.ld(s2) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bb6a4u64) }
        0x102bb6a4 => { vm.decode(0x8a21c909u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102bb6a8u64) }
        0x102bb6a8 => { vm.decode(0xa0002040u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102bb6acu64) }
        0x102bb6ac => { vm.decode(0x8482a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 141i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bb6b0u64).wrapping_add((imm * 4) as u64) } else { 0x102bb6b0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bb6b0u64) }
        0x102bb6b0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bb6b4u64) } }
        0x102bb6b4 => { vm.decode(0x4a8863afu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bb6b8u64) }
        0x102bb6b8 => { vm.decode(0x2020406fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bb6bcu64) }
        0x102bb6bc => { vm.decode(0x662d0159u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1864i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bb6c0u64) }
        0x102bb6c0 => { vm.decode(0x9309d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bb6c4u64) }
        0x102bb6c4 => { vm.decode(0x9211db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bb6c8u64) }
        0x102bb6c8 => { vm.decode(0x928dc30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bb6ccu64) }
        0x102bb6cc => { vm.decode(0x9395c30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 2392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bb6d0u64) }
        0x102bb6d0 => { vm.decode(0x19291c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bb6d4u64) }
        0x102bb6d4 => { vm.decode(0x9823c909u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102bb6d8u64) }
        0x102bb6d8 => { vm.decode(0xa88636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bb6dcu64) }
        0x102bb6dc => { vm.decode(0xc41019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bb6e0u64) }
        0x102bb6e0 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102bb6e4u64) }
        0x102bb6e4 => { vm.decode(0x19311c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bb6e8u64) }
        0x102bb6e8 => { vm.decode(0x886632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bb6ecu64) }
        0x102bb6ec => { vm.decode(0x688c43afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bb6f0u64) }
        0x102bb6f0 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bb6f4u64) }
        0x102bb6f4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102bb6f8u64 + 4);
        vm.wr(37, 0x102bb6f8u64 + 4);
        Ok(0x102bb6f8u64) }
        0x102bb6f8 => { vm.decode(0x620d8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bb6fcu64) }
        0x102bb6fc => { vm.decode(0xd10ea91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 796i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb700u64) }
        0x102bb700 => { vm.decode(0x3353062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb704u64) }
        0x102bb704 => { vm.decode(0xd10ea11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb708u64) }
        0x102bb708 => { vm.decode(0x43251062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb70cu64) }
        0x102bb70c => { vm.decode(0x15106b91u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 788i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb710u64) }
        0x102bb710 => { vm.decode(0xd106a11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb714u64) }
        0x102bb714 => { vm.decode(0x3351062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb718u64) }
        0x102bb718 => { vm.decode(0xd90ea91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 860i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb71cu64) }
        0x102bb71c => { vm.decode(0x230d3022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb720u64) }
        0x102bb720 => { vm.decode(0xd90ea11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb724u64) }
        0x102bb724 => { vm.decode(0x43353062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb728u64) }
        0x102bb728 => { vm.decode(0xd906a91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 852i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb72cu64) }
        0x102bb72c => { vm.decode(0x63353062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb730u64) }
        0x102bb730 => { vm.decode(0xd906a11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb734u64) }
        0x102bb734 => { vm.decode(0x431d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb738u64) }
        0x102bb738 => { vm.decode(0xd90e291u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 844i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb73cu64) }
        0x102bb73c => { vm.decode(0x230d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb740u64) }
        0x102bb740 => { vm.decode(0xd90e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb744u64) }
        0x102bb744 => { vm.decode(0x431d3022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb748u64) }
        0x102bb748 => { vm.decode(0xd906291u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 836i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb74cu64) }
        0x102bb74c => { vm.decode(0x430d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb750u64) }
        0x102bb750 => { vm.decode(0xd906211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb754u64) }
        0x102bb754 => { vm.decode(0x63351062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb758u64) }
        0x102bb758 => { vm.decode(0xd14ea91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 828i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb75cu64) }
        0x102bb75c => { vm.decode(0x630d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb760u64) }
        0x102bb760 => { vm.decode(0xd14ea11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb764u64) }
        0x102bb764 => { vm.decode(0x30d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb768u64) }
        0x102bb768 => { vm.decode(0xd146a91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 820i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb76cu64) }
        0x102bb76c => { vm.decode(0x231d3022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb770u64) }
        0x102bb770 => { vm.decode(0xd146a11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb774u64) }
        0x102bb774 => { vm.decode(0x31d3022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb778u64) }
        0x102bb778 => { vm.decode(0xd14e291u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 812i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb77cu64) }
        0x102bb77c => { vm.decode(0x231d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb780u64) }
        0x102bb780 => { vm.decode(0xd14e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb784u64) }
        0x102bb784 => { vm.decode(0x30d3022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb788u64) }
        0x102bb788 => { vm.decode(0xd146291u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 804i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb78cu64) }
        0x102bb78c => { vm.decode(0x63253062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb790u64) }
        0x102bb790 => { vm.decode(0xd146211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb794u64) }
        0x102bb794 => { vm.decode(0x63251062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb798u64) }
        0x102bb798 => { vm.decode(0x10008119u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bb79cu64) }
        0x102bb79c => { vm.decode(0x2824402fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bb7a0u64) }
        0x102bb7a0 => { vm.decode(0x9b211430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bb7a4u64) }
        0x102bb7a4 => { vm.decode(0x88cc32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bb7a8u64) }
        0x102bb7a8 => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bb7acu64) }
        0x102bb7ac => { vm.decode(0x9b1154b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bb7b0u64) }
        0x102bb7b0 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bb7b4u64) }
        0x102bb7b4 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bb7b8u64) }
        0x102bb7b8 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bb7bcu64) }
        0x102bb7bc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102bb7c0u64 + 4);
        vm.wr(37, 0x102bb7c0u64 + 4);
        Ok(0x102bb7c0u64) }
        0x102bb7c0 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bb7c4u64) }
        0x102bb7c4 => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bb7c8u64) }
        0x102bb7c8 => { vm.decode(0x2220406fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bb7ccu64) }
        0x102bb7cc => { vm.decode(0x9b2114b0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bb7d0u64) }
        0x102bb7d0 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102bb7d4u64) }
        0x102bb7d4 => { vm.decode(0x8b315570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bb7d8u64) }
        0x102bb7d8 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bb7dcu64) }
        0x102bb7dc => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bb7e0u64) }
        0x102bb7e0 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bb7e4u64) }
        0x102bb7e4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102bb7e8u64 + 4);
        vm.wr(37, 0x102bb7e8u64 + 4);
        Ok(0x102bb7e8u64) }
        0x102bb7e8 => { vm.decode(0x100036u32); let d = 16i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bb7ecu64) }
        0x102bb7ec => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bb7f0u64) }
        0x102bb7f0 => { vm.decode(0x5422a7d9u32); let d = 17i16; let s = 1i16; let s2 = -1i16; let imm = 8133i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bb7f4u64) }
        0x102bb7f4 => { vm.decode(0x10109019u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bb7f8u64) }
        0x102bb7f8 => { vm.decode(0x4002406u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102bb7fcu64) }
        0x102bb7fc => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102bb800u64).wrapping_add((imm * 4) as u64) } else { 0x102bb800u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bb800u64) }
        0x102bb800 => { vm.decode(0x6102036u32); let d = 21i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bb804u64) }
        0x102bb804 => { vm.decode(0x2f2c901du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 218476i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102bb808u64) }
        0x102bb80c => { vm.decode(0xa624735u32); let d = 1i16; let s = 4i16; let s2 = 23i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bb810u64) }
        0x102bb810 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102bb814u64) }
        0x102bb814 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bb818u64) }
        0x102bb818 => { vm.decode(0x74314675u32); let d = 3i16; let s = 3i16; let s2 = 23i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bb81cu64) }
        0x102bb81c => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bb820u64) }
        0x102bb820 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bb824u64) }
        0x102bb824 => { vm.decode(0x4b21a4f5u32); let d = 2i16; let s = 22i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102bb828u64) }
        0x102bb828 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb82cu64) }
        0x102bb82c => { vm.decode(0x8900036u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bb830u64) }
        0x102bb830 => { vm.decode(0x4002406u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102bb834u64) }
        0x102bb834 => { vm.decode(0x4848227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 45i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bb838u64).wrapping_add((imm * 4) as u64) } else { 0x102bb838u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bb838u64) }
        0x102bb838 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bb83cu64) } }
        0x102bb83c => { vm.decode(0x2084c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bb840u64) }
        0x102bb840 => { vm.decode(0xa14004f5u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102bb844u64) }
        0x102bb844 => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bb848u64) }
        0x102bb848 => { vm.decode(0x82c202fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bb84cu64) }
        0x102bb84c => { vm.decode(0x9b191530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bb850u64) }
        0x102bb850 => { vm.decode(0xb091530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bb854u64) }
        0x102bb854 => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bb858u64) }
        0x102bb858 => { vm.decode(0x40020bb5u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bb85cu64) }
        0x102bb85c => { vm.decode(0xc000bd75u32); let d = 22i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102bb860u64) }
        0x102bb860 => { vm.decode(0x228043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bb864u64) }
        0x102bb864 => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102bb868u64) }
        0x102bb868 => { vm.decode(0x5c000111u32); let d = 23i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb86cu64) }
        0x102bb86c => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bb870u64) }
        0x102bb870 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bb874u64) }
        0x102bb874 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102bb878u64 + 4);
        vm.wr(37, 0x102bb878u64 + 4);
        Ok(0x102bb878u64) }
        0x102bb878 => { vm.decode(0x660d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bb87cu64) }
        0x102bb87c => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb880u64) }
        0x102bb880 => { vm.decode(0x28c48b35u32); let d = 4i16; let s = 3i16; let s2 = 21i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bb884u64) }
        0x102bb884 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bb888u64) }
        0x102bb888 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bb88cu64) }
        0x102bb88c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bb890u64).wrapping_add((imm * 4) as u64) } else { 0x102bb890u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bb890u64) }
        0x102bb890 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bb894u64) }
        0x102bb894 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bb898u64) }
        0x102bb898 => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bb89cu64) }
        0x102bb89c => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102bb8a0u64) }
        0x102bb8a0 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bb8a4u64) }
        0x102bb8a4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102bb8a8u64).wrapping_add((imm * 4) as u64) } else { 0x102bb8a8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bb8a8u64) }
        0x102bb8a8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bb8acu64) } }
        0x102bb8ac => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bb8b0u64) }
        0x102bb8b0 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bb8b4u64) }
        0x102bb8b4 => { vm.decode(0x739cb4a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -43i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bb8b8u64).wrapping_add((imm * 4) as u64) } else { 0x102bb8b8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bb8b8u64) }
        0x102bb8b8 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bb8bcu64) }
        0x102bb8bc => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bb8c0u64) }
        0x102bb8c0 => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bb8c4u64) }
        0x102bb8c4 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102bb8c8u64) }
        0x102bb8c8 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bb8ccu64) }
        0x102bb8cc => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102bb8d0u64).wrapping_add((imm * 4) as u64) } else { 0x102bb8d0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bb8d0u64) }
        0x102bb8d0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bb8d4u64) } }
        0x102bb8d4 => { vm.decode(0xae0c801du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 218252i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102bb8d8u64) }
        0x102bb8d8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bb8dcu64) } }
        0x102bb8dc => { vm.decode(0xa728851du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 222452i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102bb8e0u64) }
        0x102bb8ec => { vm.decode(0xd9060d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1348i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb8f0u64) }
        0x102bb8f0 => { vm.decode(0x3253062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb8f4u64) }
        0x102bb8f4 => { vm.decode(0xd906051u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb8f8u64) }
        0x102bb8f8 => { vm.decode(0x3251062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb8fcu64) }
        0x102bb8fc => { vm.decode(0xd14e8d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1340i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb900u64) }
        0x102bb900 => { vm.decode(0x63153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb904u64) }
        0x102bb904 => { vm.decode(0xd14e851u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb908u64) }
        0x102bb908 => { vm.decode(0x63053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb90cu64) }
        0x102bb90c => { vm.decode(0xd1468d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1332i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb910u64) }
        0x102bb910 => { vm.decode(0x63151062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb914u64) }
        0x102bb914 => { vm.decode(0xd146851u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb918u64) }
        0x102bb918 => { vm.decode(0x63051062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb91cu64) }
        0x102bb91c => { vm.decode(0xd14e0d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1324i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb920u64) }
        0x102bb920 => { vm.decode(0x43153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb924u64) }
        0x102bb924 => { vm.decode(0xd14e051u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb928u64) }
        0x102bb928 => { vm.decode(0x43053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb92cu64) }
        0x102bb92c => { vm.decode(0xd1460d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1316i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb930u64) }
        0x102bb930 => { vm.decode(0x43151062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb934u64) }
        0x102bb934 => { vm.decode(0xd146051u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb938u64) }
        0x102bb938 => { vm.decode(0x43051062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb93cu64) }
        0x102bb93c => { vm.decode(0xd10e8d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1308i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb940u64) }
        0x102bb940 => { vm.decode(0x23153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb944u64) }
        0x102bb944 => { vm.decode(0xd10e851u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb948u64) }
        0x102bb948 => { vm.decode(0x23053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb94cu64) }
        0x102bb94c => { vm.decode(0xd1068d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1300i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb950u64) }
        0x102bb950 => { vm.decode(0x23151062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb954u64) }
        0x102bb954 => { vm.decode(0xd106851u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb958u64) }
        0x102bb958 => { vm.decode(0x3153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bb95cu64) }
        0x102bb95c => { vm.decode(0x5510f1d1u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 1292i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb960u64) }
        0x102bb960 => { vm.decode(0x5d10e151u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 1288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb964u64) }
        0x102bb964 => { vm.decode(0x4a8a63afu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bb968u64) }
        0x102bb968 => { vm.decode(0x28a4402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bb96cu64) }
        0x102bb96c => { vm.decode(0x12008119u32); let d = 21i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bb970u64) }
        0x102bb970 => { vm.decode(0x9b215430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bb974u64) }
        0x102bb974 => { vm.decode(0x88ce32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bb978u64) }
        0x102bb978 => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bb97cu64) }
        0x102bb97c => { vm.decode(0x704b0419u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2192i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bb980u64) }
        0x102bb980 => { vm.decode(0x8b1114b0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bb984u64) }
        0x102bb984 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bb988u64) }
        0x102bb988 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bb98cu64) }
        0x102bb98c => { vm.decode(0x488c43afu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bb990u64) }
        0x102bb990 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bb994u64) }
        0x102bb994 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102bb998u64 + 4);
        vm.wr(37, 0x102bb998u64 + 4);
        Ok(0x102bb998u64) }
        0x102bb998 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bb99cu64) }
        0x102bb99c => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bb9a0u64) }
        0x102bb9a0 => { vm.decode(0x70498519u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bb9a4u64) }
        0x102bb9a4 => { vm.decode(0x22a0406fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bb9a8u64) }
        0x102bb9a8 => { vm.decode(0x9b2154b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bb9acu64) }
        0x102bb9ac => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102bb9b0u64) }
        0x102bb9b0 => { vm.decode(0x9b311430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bb9b4u64) }
        0x102bb9b4 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bb9b8u64) }
        0x102bb9b8 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bb9bcu64) }
        0x102bb9bc => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bb9c0u64) }
        0x102bb9c0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102bb9c4u64 + 4);
        vm.wr(37, 0x102bb9c4u64 + 4);
        Ok(0x102bb9c4u64) }
        0x102bb9c4 => { vm.decode(0x102236u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bb9c8u64) }
        0x102bb9c8 => { vm.decode(0xd986111u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb9ccu64) }
        0x102bb9cc => { vm.decode(0x518e911u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb9d0u64) }
        0x102bb9d0 => { vm.decode(0x2001a835u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bb9d4u64) }
        0x102bb9d4 => { vm.decode(0x130207f5u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bb9d8u64) }
        0x102bb9d8 => { vm.decode(0x40012835u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bb9dcu64) }
        0x102bb9dc => { vm.decode(0x4598e811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb9e0u64) }
        0x102bb9e0 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bb9e4u64) }
        0x102bb9e4 => { vm.decode(0xc4812fb5u32); let d = 1i16; let s = 1i16; let s2 = 23i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bb9e8u64) }
        0x102bb9e8 => { vm.decode(0x4518e011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bb9ecu64) }
        0x102bb9ec => { vm.decode(0x40210fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bb9f0u64) }
        0x102bb9f0 => { vm.decode(0x1a01cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 25i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bb9f4u64) }
        0x102bb9f4 => { vm.decode(0xc4800fb5u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bb9f8u64) }
        0x102bb9f8 => { vm.decode(0x40198835u32); let d = 2i16; let s = 5i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bb9fcu64) }
        0x102bb9fc => { vm.decode(0x440017f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bba00u64) }
        0x102bba00 => { vm.decode(0x200d0835u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bba04u64) }
        0x102bba04 => { vm.decode(0x4d144a11u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bba08u64) }
        0x102bba08 => { vm.decode(0x20212fb5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bba0cu64) }
        0x102bba0c => { vm.decode(0xb0a02fb5u32); let d = 2i16; let s = 2i16; let s2 = 30i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bba10u64) }
        0x102bba10 => { vm.decode(0x20102fb5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bba14u64) }
        0x102bba14 => { vm.decode(0x28910d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 20i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bba18u64) }
        0x102bba18 => { vm.decode(0x20210fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bba1cu64) }
        0x102bba1c => { vm.decode(0x2008b835u32); let d = 3i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bba20u64) }
        0x102bba20 => { vm.decode(0x461207f5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bba24u64) }
        0x102bba24 => { vm.decode(0x401d2835u32); let d = 3i16; let s = 3i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bba28u64) }
        0x102bba28 => { vm.decode(0x5104b11u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bba2cu64) }
        0x102bba2c => { vm.decode(0x64a00fb5u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bba30u64) }
        0x102bba30 => { vm.decode(0x510e911u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bba34u64) }
        0x102bba34 => { vm.decode(0x64a00fb5u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bba38u64) }
        0x102bba38 => { vm.decode(0x64112fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bba3cu64) }
        0x102bba3c => { vm.decode(0x3e90cd35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 15i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bba40u64) }
        0x102bba40 => { vm.decode(0x64202fb5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bba44u64) }
        0x102bba44 => { vm.decode(0x41400835u32); let d = 4i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bba48u64) }
        0x102bba48 => { vm.decode(0x1157f5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bba4cu64) }
        0x102bba4c => { vm.decode(0x21008835u32); let d = 4i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bba50u64) }
        0x102bba50 => { vm.decode(0xd14cb11u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bba54u64) }
        0x102bba54 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bba58u64) }
        0x102bba58 => { vm.decode(0xd14e111u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bba5cu64) }
        0x102bba5c => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bba60u64) }
        0x102bba60 => { vm.decode(0x9a0fb5u32); let d = 4i16; let s = 4i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bba64u64) }
        0x102bba64 => { vm.decode(0xe240d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 10i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bba68u64) }
        0x102bba68 => { vm.decode(0x132fb5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bba6cu64) }
        0x102bba6c => { vm.decode(0x21442835u32); let d = 5i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bba70u64) }
        0x102bba70 => { vm.decode(0x120347f5u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bba74u64) }
        0x102bba74 => { vm.decode(0x4140a835u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bba78u64) }
        0x102bba78 => { vm.decode(0x4590cb11u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bba7cu64) }
        0x102bba7c => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bba80u64) }
        0x102bba80 => { vm.decode(0x45146911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bba84u64) }
        0x102bba84 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bba88u64) }
        0x102bba88 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bba8cu64) }
        0x102bba8c => { vm.decode(0x1a01cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 25i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bba90u64) }
        0x102bba90 => { vm.decode(0x44800fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bba94u64) }
        0x102bba94 => { vm.decode(0x41052835u32); let d = 5i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bba98u64) }
        0x102bba98 => { vm.decode(0x20357f5u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bba9cu64) }
        0x102bba9c => { vm.decode(0x6140a835u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbaa0u64) }
        0x102bbaa0 => { vm.decode(0x4510cb11u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbaa4u64) }
        0x102bbaa4 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbaa8u64) }
        0x102bbaa8 => { vm.decode(0x4514e911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbaacu64) }
        0x102bbaac => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbab0u64) }
        0x102bbab0 => { vm.decode(0x20a10fb5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbab4u64) }
        0x102bbab4 => { vm.decode(0x28910d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 20i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbab8u64) }
        0x102bbab8 => { vm.decode(0x20210fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbabcu64) }
        0x102bbabc => { vm.decode(0x2100a835u32); let d = 5i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbac0u64) }
        0x102bbac0 => { vm.decode(0x21347f5u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbac4u64) }
        0x102bbac4 => { vm.decode(0x4101a835u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbac8u64) }
        0x102bbac8 => { vm.decode(0x459c6111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbaccu64) }
        0x102bbacc => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbad0u64) }
        0x102bbad0 => { vm.decode(0x45906111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbad4u64) }
        0x102bbad4 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbad8u64) }
        0x102bbad8 => { vm.decode(0x64a10fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbadcu64) }
        0x102bbadc => { vm.decode(0x3e90cd35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 15i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbae0u64) }
        0x102bbae0 => { vm.decode(0x64202fb5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbae4u64) }
        0x102bbae4 => { vm.decode(0x41402835u32); let d = 5i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbae8u64) }
        0x102bbae8 => { vm.decode(0x21357f5u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbaecu64) }
        0x102bbaec => { vm.decode(0x6100a835u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbaf0u64) }
        0x102bbaf0 => { vm.decode(0x4514c311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbaf4u64) }
        0x102bbaf4 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbaf8u64) }
        0x102bbaf8 => { vm.decode(0x4590e111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbafcu64) }
        0x102bbafc => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbb00u64) }
        0x102bbb00 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbb04u64) }
        0x102bbb04 => { vm.decode(0xe240d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 10i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbb08u64) }
        0x102bbb08 => { vm.decode(0x132fb5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbb0cu64) }
        0x102bbb0c => { vm.decode(0x21442835u32); let d = 5i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbb10u64) }
        0x102bbb10 => { vm.decode(0x120347f5u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbb14u64) }
        0x102bbb14 => { vm.decode(0x4140a835u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbb18u64) }
        0x102bbb18 => { vm.decode(0x459ce111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbb1cu64) }
        0x102bbb1c => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbb20u64) }
        0x102bbb20 => { vm.decode(0x45906911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbb24u64) }
        0x102bbb24 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbb28u64) }
        0x102bbb28 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbb2cu64) }
        0x102bbb2c => { vm.decode(0x1a01cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 25i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbb30u64) }
        0x102bbb30 => { vm.decode(0x44800fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbb34u64) }
        0x102bbb34 => { vm.decode(0x41052835u32); let d = 5i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbb38u64) }
        0x102bbb38 => { vm.decode(0x20357f5u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbb3cu64) }
        0x102bbb3c => { vm.decode(0x6140a835u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbb40u64) }
        0x102bbb40 => { vm.decode(0x459c6911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbb44u64) }
        0x102bbb44 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbb48u64) }
        0x102bbb48 => { vm.decode(0x4590e911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbb4cu64) }
        0x102bbb4c => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbb50u64) }
        0x102bbb50 => { vm.decode(0x20a10fb5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbb54u64) }
        0x102bbb54 => { vm.decode(0x28910d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 20i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbb58u64) }
        0x102bbb58 => { vm.decode(0x20210fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbb5cu64) }
        0x102bbb5c => { vm.decode(0x2100a835u32); let d = 5i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbb60u64) }
        0x102bbb60 => { vm.decode(0x21347f5u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbb64u64) }
        0x102bbb64 => { vm.decode(0x4101a835u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbb68u64) }
        0x102bbb68 => { vm.decode(0x45104311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbb6cu64) }
        0x102bbb6c => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbb70u64) }
        0x102bbb70 => { vm.decode(0x45946111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbb74u64) }
        0x102bbb74 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbb78u64) }
        0x102bbb78 => { vm.decode(0x64a10fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbb7cu64) }
        0x102bbb7c => { vm.decode(0x3e90cd35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 15i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbb80u64) }
        0x102bbb80 => { vm.decode(0x64202fb5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbb84u64) }
        0x102bbb84 => { vm.decode(0x41402835u32); let d = 5i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbb88u64) }
        0x102bbb88 => { vm.decode(0x21357f5u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbb8cu64) }
        0x102bbb8c => { vm.decode(0x6100a835u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbb90u64) }
        0x102bbb90 => { vm.decode(0x45904311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbb94u64) }
        0x102bbb94 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbb98u64) }
        0x102bbb98 => { vm.decode(0x4594e111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbb9cu64) }
        0x102bbb9c => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbba0u64) }
        0x102bbba0 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbba4u64) }
        0x102bbba4 => { vm.decode(0xe240d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 10i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbba8u64) }
        0x102bbba8 => { vm.decode(0x132fb5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbbacu64) }
        0x102bbbac => { vm.decode(0x21442835u32); let d = 5i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbbb0u64) }
        0x102bbbb0 => { vm.decode(0x120347f5u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbbb4u64) }
        0x102bbbb4 => { vm.decode(0x4140a835u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbbb8u64) }
        0x102bbbb8 => { vm.decode(0x45944311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbbbcu64) }
        0x102bbbbc => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbbc0u64) }
        0x102bbbc0 => { vm.decode(0x45946911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbbc4u64) }
        0x102bbbc4 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbbc8u64) }
        0x102bbbc8 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbbccu64) }
        0x102bbbcc => { vm.decode(0x1a01cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 25i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbbd0u64) }
        0x102bbbd0 => { vm.decode(0x24820fb5u32); let d = 6i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbbd4u64) }
        0x102bbbd4 => { vm.decode(0x40052835u32); let d = 1i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbbd8u64) }
        0x102bbbd8 => { vm.decode(0x121207f5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbbdcu64) }
        0x102bbbdc => { vm.decode(0x60402835u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbbe0u64) }
        0x102bbbe0 => { vm.decode(0xd144311u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbbe4u64) }
        0x102bbbe4 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbbe8u64) }
        0x102bbbe8 => { vm.decode(0xd94e911u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbbecu64) }
        0x102bbbec => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbbf0u64) }
        0x102bbbf0 => { vm.decode(0x40210fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbbf4u64) }
        0x102bbbf4 => { vm.decode(0x18814d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 20i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbbf8u64) }
        0x102bbbf8 => { vm.decode(0x44822fb5u32); let d = 5i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbbfcu64) }
        0x102bbbfc => { vm.decode(0x41a835u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbc00u64) }
        0x102bbc00 => { vm.decode(0x120217f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbc04u64) }
        0x102bbc04 => { vm.decode(0x40012835u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbc08u64) }
        0x102bbc08 => { vm.decode(0x4590c211u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbc0cu64) }
        0x102bbc0c => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbc10u64) }
        0x102bbc10 => { vm.decode(0x45186011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbc14u64) }
        0x102bbc14 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbc18u64) }
        0x102bbc18 => { vm.decode(0x44210fb5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbc1cu64) }
        0x102bbc1c => { vm.decode(0x1e80cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbc20u64) }
        0x102bbc20 => { vm.decode(0x25870fb5u32); let d = 30i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbc24u64) }
        0x102bbc24 => { vm.decode(0x3651062u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bbc28u64) }
        0x102bbc28 => { vm.decode(0x2005a835u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbc2cu64) }
        0x102bbc2c => { vm.decode(0x131a07f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbc30u64) }
        0x102bbc30 => { vm.decode(0x2755062u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bbc34u64) }
        0x102bbc34 => { vm.decode(0x40412835u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbc38u64) }
        0x102bbc38 => { vm.decode(0x459ce811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbc3cu64) }
        0x102bbc3c => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbc40u64) }
        0x102bbc40 => { vm.decode(0x45186811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbc44u64) }
        0x102bbc44 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbc48u64) }
        0x102bbc48 => { vm.decode(0x40110fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbc4cu64) }
        0x102bbc4c => { vm.decode(0x1e004d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 10i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbc50u64) }
        0x102bbc50 => { vm.decode(0xd4802fb5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbc54u64) }
        0x102bbc54 => { vm.decode(0x3253062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bbc58u64) }
        0x102bbc58 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bbc5cu64) }
        0x102bbc5c => { vm.decode(0xd622a759u32); let d = 21i16; let s = 1i16; let s2 = -1i16; let imm = 8134i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bbc60u64) }
        0x102bbc60 => { vm.decode(0x12101019u32); let d = 22i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bbc64u64) }
        0x102bbc64 => { vm.decode(0x26002406u32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102bbc68u64) }
        0x102bbc68 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102bbc6cu64).wrapping_add((imm * 4) as u64) } else { 0x102bbc6cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bbc6cu64) }
        0x102bbc6c => { vm.decode(0x6102236u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bbc70u64) }
        0x102bbc70 => { vm.decode(0xa22cd01du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 219588i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102bbc74u64) }
        0x102bbc78 => { vm.decode(0x8224735u32); let d = 1i16; let s = 4i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbc7cu64) }
        0x102bbc7c => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102bbc80u64) }
        0x102bbc80 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbc84u64) }
        0x102bbc84 => { vm.decode(0x64304675u32); let d = 3i16; let s = 3i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbc88u64) }
        0x102bbc88 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbc8cu64) }
        0x102bbc8c => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbc90u64) }
        0x102bbc90 => { vm.decode(0x690104f5u32); let d = 2i16; let s = 17i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102bbc94u64) }
        0x102bbc94 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bbc98u64) }
        0x102bbc98 => { vm.decode(0x8b03236u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bbc9cu64) }
        0x102bbc9c => { vm.decode(0x26002406u32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102bbca0u64) }
        0x102bbca0 => { vm.decode(0x4040227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 40i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bbca4u64).wrapping_add((imm * 4) as u64) } else { 0x102bbca4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bbca4u64) }
        0x102bbca4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bbca8u64) } }
        0x102bbca8 => { vm.decode(0x2084c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bbcacu64) }
        0x102bbcac => { vm.decode(0xa16004f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102bbcb0u64) }
        0x102bbcb0 => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bbcb4u64) }
        0x102bbcb4 => { vm.decode(0x82c202fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bbcb8u64) }
        0x102bbcb8 => { vm.decode(0x8b195530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bbcbcu64) }
        0x102bbcbc => { vm.decode(0xb091530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bbcc0u64) }
        0x102bbcc0 => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bbcc4u64) }
        0x102bbcc4 => { vm.decode(0x40830bb5u32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbcc8u64) }
        0x102bbcc8 => { vm.decode(0xc0003d75u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102bbcccu64) }
        0x102bbccc => { vm.decode(0x414104f5u32); let d = 1i16; let s = 16i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102bbcd0u64) }
        0x102bbcd0 => { vm.decode(0x14000111u32); let d = 20i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbcd4u64) }
        0x102bbcd4 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bbcd8u64) }
        0x102bbcd8 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bbcdcu64) }
        0x102bbcdc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102bbce0u64 + 4);
        vm.wr(37, 0x102bbce0u64 + 4);
        Ok(0x102bbce0u64) }
        0x102bbce0 => { vm.decode(0x660d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bbce4u64) }
        0x102bbce4 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbce8u64) }
        0x102bbce8 => { vm.decode(0x28cc8b35u32); let d = 4i16; let s = 3i16; let s2 = 23i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbcecu64) }
        0x102bbcec => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bbcf0u64) }
        0x102bbcf0 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bbcf4u64) }
        0x102bbcf4 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bbcf8u64).wrapping_add((imm * 4) as u64) } else { 0x102bbcf8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bbcf8u64) }
        0x102bbcf8 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bbcfcu64) }
        0x102bbcfc => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bbd00u64) }
        0x102bbd00 => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbd04u64) }
        0x102bbd04 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102bbd08u64) }
        0x102bbd08 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bbd0cu64) }
        0x102bbd0c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102bbd10u64).wrapping_add((imm * 4) as u64) } else { 0x102bbd10u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bbd10u64) }
        0x102bbd10 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bbd14u64) } }
        0x102bbd14 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bbd18u64) }
        0x102bbd18 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bbd1cu64) }
        0x102bbd1c => { vm.decode(0x739c3ca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -42i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bbd20u64).wrapping_add((imm * 4) as u64) } else { 0x102bbd20u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bbd20u64) }
        0x102bbd20 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bbd24u64) }
        0x102bbd24 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bbd28u64) }
        0x102bbd28 => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbd2cu64) }
        0x102bbd2c => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102bbd30u64) }
        0x102bbd30 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bbd34u64) }
        0x102bbd34 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102bbd38u64).wrapping_add((imm * 4) as u64) } else { 0x102bbd38u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bbd38u64) }
        0x102bbd38 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bbd3cu64) } }
        0x102bbd3c => { vm.decode(0x8b2cc41du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 219384i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102bbd40u64) }
        0x102bbd40 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bbd44u64) } }
        0x102bbd44 => { vm.decode(0xf18c091u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2188i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbd48u64) }
        0x102bbd48 => { vm.decode(0x43251062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bbd4cu64) }
        0x102bbd4c => { vm.decode(0xf18c011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbd50u64) }
        0x102bbd50 => { vm.decode(0x3353062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bbd54u64) }
        0x102bbd54 => { vm.decode(0xf184091u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2180i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbd58u64) }
        0x102bbd58 => { vm.decode(0x63153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bbd5cu64) }
        0x102bbd5c => { vm.decode(0xf184011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbd60u64) }
        0x102bbd60 => { vm.decode(0x63053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bbd64u64) }
        0x102bbd64 => { vm.decode(0xf94c891u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2172i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbd68u64) }
        0x102bbd68 => { vm.decode(0x63151062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bbd6cu64) }
        0x102bbd6c => { vm.decode(0xf94c811u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbd70u64) }
        0x102bbd70 => { vm.decode(0x63051062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bbd74u64) }
        0x102bbd74 => { vm.decode(0xf944891u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2164i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbd78u64) }
        0x102bbd78 => { vm.decode(0x43153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bbd7cu64) }
        0x102bbd7c => { vm.decode(0xf944811u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbd80u64) }
        0x102bbd80 => { vm.decode(0x43053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bbd84u64) }
        0x102bbd84 => { vm.decode(0xf94c091u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2156i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbd88u64) }
        0x102bbd88 => { vm.decode(0x43151062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bbd8cu64) }
        0x102bbd8c => { vm.decode(0xf94c011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbd90u64) }
        0x102bbd90 => { vm.decode(0x43051062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bbd94u64) }
        0x102bbd94 => { vm.decode(0xf944091u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2148i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbd98u64) }
        0x102bbd98 => { vm.decode(0x23153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bbd9cu64) }
        0x102bbd9c => { vm.decode(0xf944011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbda0u64) }
        0x102bbda0 => { vm.decode(0x23053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bbda4u64) }
        0x102bbda4 => { vm.decode(0xf90c891u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2140i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbda8u64) }
        0x102bbda8 => { vm.decode(0x23151062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bbdacu64) }
        0x102bbdac => { vm.decode(0x1790c911u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbdb0u64) }
        0x102bbdb0 => { vm.decode(0x5f904891u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2132i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbdb4u64) }
        0x102bbdb4 => { vm.decode(0x1f904911u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbdb8u64) }
        0x102bbdb8 => { vm.decode(0x4a8863afu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bbdbcu64) }
        0x102bbdbc => { vm.decode(0x2824402fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bbdc0u64) }
        0x102bbdc0 => { vm.decode(0x12001119u32); let d = 22i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bbdc4u64) }
        0x102bbdc4 => { vm.decode(0x8b215530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bbdc8u64) }
        0x102bbdc8 => { vm.decode(0xa8cc32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bbdccu64) }
        0x102bbdcc => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bbdd0u64) }
        0x102bbdd0 => { vm.decode(0x88863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bbdd4u64) }
        0x102bbdd4 => { vm.decode(0xb1115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bbdd8u64) }
        0x102bbdd8 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bbddcu64) }
        0x102bbddc => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bbde0u64) }
        0x102bbde0 => { vm.decode(0x688e43afu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bbde4u64) }
        0x102bbde4 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbde8u64) }
        0x102bbde8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102bbdecu64 + 4);
        vm.wr(37, 0x102bbdecu64 + 4);
        Ok(0x102bbdecu64) }
        0x102bbdec => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bbdf0u64) }
        0x102bbdf0 => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bbdf4u64) }
        0x102bbdf4 => { vm.decode(0x746b0159u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2000i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bbdf8u64) }
        0x102bbdf8 => { vm.decode(0x2220406fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bbdfcu64) }
        0x102bbdfc => { vm.decode(0x8b2155b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bbe00u64) }
        0x102bbe00 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102bbe04u64) }
        0x102bbe04 => { vm.decode(0x8b311430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bbe08u64) }
        0x102bbe08 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bbe0cu64) }
        0x102bbe0c => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bbe10u64) }
        0x102bbe10 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbe14u64) }
        0x102bbe14 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102bbe18u64 + 4);
        vm.wr(37, 0x102bbe18u64 + 4);
        Ok(0x102bbe18u64) }
        0x102bbe18 => { vm.decode(0x102036u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bbe1cu64) }
        0x102bbe1c => { vm.decode(0x45186911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbe20u64) }
        0x102bbe20 => { vm.decode(0x1045b835u32); let d = 1i16; let s = 6i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbe24u64) }
        0x102bbe24 => { vm.decode(0x5186111u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbe28u64) }
        0x102bbe28 => { vm.decode(0x120207f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbe2cu64) }
        0x102bbe2c => { vm.decode(0x40532835u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbe30u64) }
        0x102bbe30 => { vm.decode(0x4598e811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbe34u64) }
        0x102bbe34 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbe38u64) }
        0x102bbe38 => { vm.decode(0xc4810fb5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbe3cu64) }
        0x102bbe3c => { vm.decode(0x4518e011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbe40u64) }
        0x102bbe40 => { vm.decode(0x40210fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbe44u64) }
        0x102bbe44 => { vm.decode(0x1e01cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbe48u64) }
        0x102bbe48 => { vm.decode(0x44802fb5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbe4cu64) }
        0x102bbe4c => { vm.decode(0x200c8835u32); let d = 2i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbe50u64) }
        0x102bbe50 => { vm.decode(0x551807f5u32); let d = 2i16; let s = 2i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbe54u64) }
        0x102bbe54 => { vm.decode(0x4d0835u32); let d = 2i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbe58u64) }
        0x102bbe58 => { vm.decode(0x4d104211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbe5cu64) }
        0x102bbe5c => { vm.decode(0x20212fb5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbe60u64) }
        0x102bbe60 => { vm.decode(0xa0212fb5u32); let d = 2i16; let s = 2i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbe64u64) }
        0x102bbe64 => { vm.decode(0x20102fb5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbe68u64) }
        0x102bbe68 => { vm.decode(0x2c918d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 23i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbe6cu64) }
        0x102bbe6c => { vm.decode(0x20210fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbe70u64) }
        0x102bbe70 => { vm.decode(0x40482835u32); let d = 3i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbe74u64) }
        0x102bbe74 => { vm.decode(0x561207f5u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbe78u64) }
        0x102bbe78 => { vm.decode(0x600c2835u32); let d = 3i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbe7cu64) }
        0x102bbe7c => { vm.decode(0x510cb11u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbe80u64) }
        0x102bbe80 => { vm.decode(0x64a00fb5u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbe84u64) }
        0x102bbe84 => { vm.decode(0xe4a00fb5u32); let d = 3i16; let s = 3i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbe88u64) }
        0x102bbe88 => { vm.decode(0x60396fb5u32); let d = 3i16; let s = 3i16; let s2 = 30i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbe8cu64) }
        0x102bbe8c => { vm.decode(0x3c114d35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 18i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbe90u64) }
        0x102bbe90 => { vm.decode(0x64202fb5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbe94u64) }
        0x102bbe94 => { vm.decode(0x21440835u32); let d = 4i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbe98u64) }
        0x102bbe98 => { vm.decode(0x157f5u32); let d = 4i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbe9cu64) }
        0x102bbe9c => { vm.decode(0x1408835u32); let d = 4i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbea0u64) }
        0x102bbea0 => { vm.decode(0xd90c311u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbea4u64) }
        0x102bbea4 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbea8u64) }
        0x102bbea8 => { vm.decode(0xd14e111u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbeacu64) }
        0x102bbeac => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbeb0u64) }
        0x102bbeb0 => { vm.decode(0xb20fb5u32); let d = 4i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbeb4u64) }
        0x102bbeb4 => { vm.decode(0xaa40d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 12i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbeb8u64) }
        0x102bbeb8 => { vm.decode(0x132fb5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbebcu64) }
        0x102bbebc => { vm.decode(0x41052835u32); let d = 5i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbec0u64) }
        0x102bbec0 => { vm.decode(0x21347f5u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbec4u64) }
        0x102bbec4 => { vm.decode(0x6140a835u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbec8u64) }
        0x102bbec8 => { vm.decode(0x4514cb11u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbeccu64) }
        0x102bbecc => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbed0u64) }
        0x102bbed0 => { vm.decode(0x45146911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbed4u64) }
        0x102bbed4 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbed8u64) }
        0x102bbed8 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbedcu64) }
        0x102bbedc => { vm.decode(0x1e01cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbee0u64) }
        0x102bbee0 => { vm.decode(0x44800fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbee4u64) }
        0x102bbee4 => { vm.decode(0x2100a835u32); let d = 5i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbee8u64) }
        0x102bbee8 => { vm.decode(0x21357f5u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbeecu64) }
        0x102bbeec => { vm.decode(0x4101a835u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbef0u64) }
        0x102bbef0 => { vm.decode(0x4590cb11u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbef4u64) }
        0x102bbef4 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbef8u64) }
        0x102bbef8 => { vm.decode(0x4514e911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbefcu64) }
        0x102bbefc => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbf00u64) }
        0x102bbf00 => { vm.decode(0x20a10fb5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbf04u64) }
        0x102bbf04 => { vm.decode(0x2c918d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 23i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbf08u64) }
        0x102bbf08 => { vm.decode(0x20210fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbf0cu64) }
        0x102bbf0c => { vm.decode(0x41402835u32); let d = 5i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbf10u64) }
        0x102bbf10 => { vm.decode(0x120347f5u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbf14u64) }
        0x102bbf14 => { vm.decode(0x6100a835u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbf18u64) }
        0x102bbf18 => { vm.decode(0x459ce911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbf1cu64) }
        0x102bbf1c => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbf20u64) }
        0x102bbf20 => { vm.decode(0x45906111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbf24u64) }
        0x102bbf24 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbf28u64) }
        0x102bbf28 => { vm.decode(0x64a10fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbf2cu64) }
        0x102bbf2c => { vm.decode(0x3c114d35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 18i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbf30u64) }
        0x102bbf30 => { vm.decode(0x64202fb5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbf34u64) }
        0x102bbf34 => { vm.decode(0x21442835u32); let d = 5i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbf38u64) }
        0x102bbf38 => { vm.decode(0x20357f5u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbf3cu64) }
        0x102bbf3c => { vm.decode(0x4140a835u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbf40u64) }
        0x102bbf40 => { vm.decode(0x45144311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbf44u64) }
        0x102bbf44 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbf48u64) }
        0x102bbf48 => { vm.decode(0x4590e111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbf4cu64) }
        0x102bbf4c => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbf50u64) }
        0x102bbf50 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbf54u64) }
        0x102bbf54 => { vm.decode(0xaa40d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 12i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbf58u64) }
        0x102bbf58 => { vm.decode(0x132fb5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbf5cu64) }
        0x102bbf5c => { vm.decode(0x41052835u32); let d = 5i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbf60u64) }
        0x102bbf60 => { vm.decode(0x21347f5u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbf64u64) }
        0x102bbf64 => { vm.decode(0x6140a835u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbf68u64) }
        0x102bbf68 => { vm.decode(0x459c6111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbf6cu64) }
        0x102bbf6c => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbf70u64) }
        0x102bbf70 => { vm.decode(0x45906911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbf74u64) }
        0x102bbf74 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbf78u64) }
        0x102bbf78 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbf7cu64) }
        0x102bbf7c => { vm.decode(0x1e01cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbf80u64) }
        0x102bbf80 => { vm.decode(0x44800fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbf84u64) }
        0x102bbf84 => { vm.decode(0x2100a835u32); let d = 5i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbf88u64) }
        0x102bbf88 => { vm.decode(0x21357f5u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbf8cu64) }
        0x102bbf8c => { vm.decode(0x4101a835u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbf90u64) }
        0x102bbf90 => { vm.decode(0x45144b11u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbf94u64) }
        0x102bbf94 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbf98u64) }
        0x102bbf98 => { vm.decode(0x4590e911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbf9cu64) }
        0x102bbf9c => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbfa0u64) }
        0x102bbfa0 => { vm.decode(0x20a10fb5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbfa4u64) }
        0x102bbfa4 => { vm.decode(0x2c918d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 23i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbfa8u64) }
        0x102bbfa8 => { vm.decode(0x20210fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbfacu64) }
        0x102bbfac => { vm.decode(0x41402835u32); let d = 5i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbfb0u64) }
        0x102bbfb0 => { vm.decode(0x120347f5u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbfb4u64) }
        0x102bbfb4 => { vm.decode(0x6100a835u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbfb8u64) }
        0x102bbfb8 => { vm.decode(0x45944311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbfbcu64) }
        0x102bbfbc => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbfc0u64) }
        0x102bbfc0 => { vm.decode(0x45946111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbfc4u64) }
        0x102bbfc4 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbfc8u64) }
        0x102bbfc8 => { vm.decode(0x64a10fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbfccu64) }
        0x102bbfcc => { vm.decode(0x3c114d35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 18i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbfd0u64) }
        0x102bbfd0 => { vm.decode(0x64202fb5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbfd4u64) }
        0x102bbfd4 => { vm.decode(0x21442835u32); let d = 5i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbfd8u64) }
        0x102bbfd8 => { vm.decode(0x20357f5u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbfdcu64) }
        0x102bbfdc => { vm.decode(0x4140a835u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bbfe0u64) }
        0x102bbfe0 => { vm.decode(0x459ce111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbfe4u64) }
        0x102bbfe4 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbfe8u64) }
        0x102bbfe8 => { vm.decode(0x4594e111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bbfecu64) }
        0x102bbfec => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbff0u64) }
        0x102bbff0 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbff4u64) }
        0x102bbff4 => { vm.decode(0xaa40d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 12i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbff8u64) }
        0x102bbff8 => { vm.decode(0x132fb5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bbffcu64) }
        0x102bbffc => { vm.decode(0x41052835u32); let d = 5i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc000u64) }
        0x102bc000 => { vm.decode(0x21347f5u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc004u64) }
        0x102bc004 => { vm.decode(0x6140a835u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc008u64) }
        0x102bc008 => { vm.decode(0x459c6911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc00cu64) }
        0x102bc00c => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc010u64) }
        0x102bc010 => { vm.decode(0x45946911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc014u64) }
        0x102bc014 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc018u64) }
        0x102bc018 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc01cu64) }
        0x102bc01c => { vm.decode(0x1e01cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc020u64) }
        0x102bc020 => { vm.decode(0x24840fb5u32); let d = 18i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc024u64) }
        0x102bc024 => { vm.decode(0x50a835u32); let d = 1i16; let s = 4i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc028u64) }
        0x102bc028 => { vm.decode(0x21217f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc02cu64) }
        0x102bc02c => { vm.decode(0x40012835u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc030u64) }
        0x102bc030 => { vm.decode(0xd14c311u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc034u64) }
        0x102bc034 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc038u64) }
        0x102bc038 => { vm.decode(0xd94e911u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc03cu64) }
        0x102bc03c => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc040u64) }
        0x102bc040 => { vm.decode(0x40210fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc044u64) }
        0x102bc044 => { vm.decode(0x1c81cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc048u64) }
        0x102bc048 => { vm.decode(0xe4042fb5u32); let d = 19i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc04cu64) }
        0x102bc04c => { vm.decode(0x20543835u32); let d = 1i16; let s = 18i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc050u64) }
        0x102bc050 => { vm.decode(0x120207f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc054u64) }
        0x102bc054 => { vm.decode(0x40502835u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc058u64) }
        0x102bc058 => { vm.decode(0x45904211u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc05cu64) }
        0x102bc05c => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc060u64) }
        0x102bc060 => { vm.decode(0x4518e811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc064u64) }
        0x102bc064 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc068u64) }
        0x102bc068 => { vm.decode(0x44210fb5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc06cu64) }
        0x102bc06c => { vm.decode(0x1c014d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 18i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc070u64) }
        0x102bc070 => { vm.decode(0xe4012fb5u32); let d = 3i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc074u64) }
        0x102bc074 => { vm.decode(0x60443835u32); let d = 1i16; let s = 3i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc078u64) }
        0x102bc078 => { vm.decode(0x31207f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc07cu64) }
        0x102bc07c => { vm.decode(0x60502835u32); let d = 1i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc080u64) }
        0x102bc080 => { vm.decode(0x45104a11u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc084u64) }
        0x102bc084 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc088u64) }
        0x102bc088 => { vm.decode(0x45986011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc08cu64) }
        0x102bc08c => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc090u64) }
        0x102bc090 => { vm.decode(0x40110fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc094u64) }
        0x102bc094 => { vm.decode(0x1a804d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 12i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc098u64) }
        0x102bc098 => { vm.decode(0x3357062u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bc09cu64) }
        0x102bc09c => { vm.decode(0x44012fb5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc0a0u64) }
        0x102bc0a0 => { vm.decode(0x3253062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bc0a4u64) }
        0x102bc0a4 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bc0a8u64) }
        0x102bc0a8 => { vm.decode(0xd62237d9u32); let d = 22i16; let s = 1i16; let s2 = -1i16; let imm = 8135i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bc0acu64) }
        0x102bc0ac => { vm.decode(0x12109019u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bc0b0u64) }
        0x102bc0b0 => { vm.decode(0x24002406u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102bc0b4u64) }
        0x102bc0b4 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102bc0b8u64).wrapping_add((imm * 4) as u64) } else { 0x102bc0b8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bc0b8u64) }
        0x102bc0b8 => { vm.decode(0x7100236u32); let d = 30i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bc0bcu64) }
        0x102bc0bc => { vm.decode(0x268cc41du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 220692i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102bc0c0u64) }
        0x102bc0c4 => { vm.decode(0xa224735u32); let d = 1i16; let s = 4i16; let s2 = 21i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc0c8u64) }
        0x102bc0c8 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102bc0ccu64) }
        0x102bc0cc => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc0d0u64) }
        0x102bc0d0 => { vm.decode(0x64314675u32); let d = 3i16; let s = 3i16; let s2 = 21i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc0d4u64) }
        0x102bc0d4 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc0d8u64) }
        0x102bc0d8 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc0dcu64) }
        0x102bc0dc => { vm.decode(0x92104f5u32); let d = 2i16; let s = 16i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102bc0e0u64) }
        0x102bc0e0 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bc0e4u64) }
        0x102bc0e4 => { vm.decode(0x8903036u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bc0e8u64) }
        0x102bc0e8 => { vm.decode(0x24002406u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102bc0ecu64) }
        0x102bc0ec => { vm.decode(0x4048227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 41i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bc0f0u64).wrapping_add((imm * 4) as u64) } else { 0x102bc0f0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bc0f0u64) }
        0x102bc0f0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bc0f4u64) } }
        0x102bc0f4 => { vm.decode(0x2084c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bc0f8u64) }
        0x102bc0f8 => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102bc0fcu64) }
        0x102bc0fc => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bc100u64) }
        0x102bc100 => { vm.decode(0x82c202fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bc104u64) }
        0x102bc104 => { vm.decode(0x9b195530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bc108u64) }
        0x102bc108 => { vm.decode(0xb091530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bc10cu64) }
        0x102bc10c => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bc110u64) }
        0x102bc110 => { vm.decode(0x40820bb5u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc114u64) }
        0x102bc114 => { vm.decode(0xc000ad75u32); let d = 20i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102bc118u64) }
        0x102bc118 => { vm.decode(0x288063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bc11cu64) }
        0x102bc11c => { vm.decode(0x216004f5u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102bc120u64) }
        0x102bc120 => { vm.decode(0x1c000111u32); let d = 21i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc124u64) }
        0x102bc124 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bc128u64) }
        0x102bc128 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bc12cu64) }
        0x102bc12c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102bc130u64 + 4);
        vm.wr(37, 0x102bc130u64 + 4);
        Ok(0x102bc130u64) }
        0x102bc130 => { vm.decode(0x660d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bc134u64) }
        0x102bc134 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc138u64) }
        0x102bc138 => { vm.decode(0x2acc0b35u32); let d = 4i16; let s = 3i16; let s2 = 30i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc13cu64) }
        0x102bc13c => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bc140u64) }
        0x102bc140 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bc144u64) }
        0x102bc144 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bc148u64).wrapping_add((imm * 4) as u64) } else { 0x102bc148u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bc148u64) }
        0x102bc148 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bc14cu64) }
        0x102bc14c => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bc150u64) }
        0x102bc150 => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc154u64) }
        0x102bc154 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102bc158u64) }
        0x102bc158 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bc15cu64) }
        0x102bc15c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102bc160u64).wrapping_add((imm * 4) as u64) } else { 0x102bc160u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bc160u64) }
        0x102bc160 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bc164u64) } }
        0x102bc164 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bc168u64) }
        0x102bc168 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bc16cu64) }
        0x102bc16c => { vm.decode(0x739cb4a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -43i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bc170u64).wrapping_add((imm * 4) as u64) } else { 0x102bc170u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bc170u64) }
        0x102bc170 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bc174u64) }
        0x102bc174 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bc178u64) }
        0x102bc178 => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc17cu64) }
        0x102bc17c => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102bc180u64) }
        0x102bc180 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bc184u64) }
        0x102bc184 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102bc188u64).wrapping_add((imm * 4) as u64) } else { 0x102bc188u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bc188u64) }
        0x102bc188 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bc18cu64) } }
        0x102bc18c => { vm.decode(0x262cd01du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 220484i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102bc190u64) }
        0x102bc190 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bc194u64) } }
        0x102bc194 => { vm.decode(0xf10c091u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2060i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc198u64) }
        0x102bc198 => { vm.decode(0x43251062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bc19cu64) }
        0x102bc19c => { vm.decode(0xf10c011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc1a0u64) }
        0x102bc1a0 => { vm.decode(0x3351062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bc1a4u64) }
        0x102bc1a4 => { vm.decode(0xf104091u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2052i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc1a8u64) }
        0x102bc1a8 => { vm.decode(0x3251062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bc1acu64) }
        0x102bc1ac => { vm.decode(0xf104011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2048i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc1b0u64) }
        0x102bc1b0 => { vm.decode(0x63153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bc1b4u64) }
        0x102bc1b4 => { vm.decode(0xd9cead1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2044i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc1b8u64) }
        0x102bc1b8 => { vm.decode(0x63053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bc1bcu64) }
        0x102bc1bc => { vm.decode(0xd9cea51u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc1c0u64) }
        0x102bc1c0 => { vm.decode(0x63151062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bc1c4u64) }
        0x102bc1c4 => { vm.decode(0xd9c6ad1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2036i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc1c8u64) }
        0x102bc1c8 => { vm.decode(0x63051062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bc1ccu64) }
        0x102bc1cc => { vm.decode(0xd9c6a51u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2032i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc1d0u64) }
        0x102bc1d0 => { vm.decode(0x43153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bc1d4u64) }
        0x102bc1d4 => { vm.decode(0xd9ce2d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2028i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc1d8u64) }
        0x102bc1d8 => { vm.decode(0x43053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bc1dcu64) }
        0x102bc1dc => { vm.decode(0xd9ce251u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc1e0u64) }
        0x102bc1e0 => { vm.decode(0x43151062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bc1e4u64) }
        0x102bc1e4 => { vm.decode(0xd9c62d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2020i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc1e8u64) }
        0x102bc1e8 => { vm.decode(0x43051062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bc1ecu64) }
        0x102bc1ec => { vm.decode(0xd9c6251u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc1f0u64) }
        0x102bc1f0 => { vm.decode(0x23153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bc1f4u64) }
        0x102bc1f4 => { vm.decode(0xd98ead1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2012i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc1f8u64) }
        0x102bc1f8 => { vm.decode(0x23053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bc1fcu64) }
        0x102bc1fc => { vm.decode(0x5598fb51u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc200u64) }
        0x102bc200 => { vm.decode(0x1d986ad1u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2004i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc204u64) }
        0x102bc204 => { vm.decode(0x55986b51u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc208u64) }
        0x102bc208 => { vm.decode(0x6a8263afu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bc20cu64) }
        0x102bc20c => { vm.decode(0x2884412fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bc210u64) }
        0x102bc210 => { vm.decode(0x12009119u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bc214u64) }
        0x102bc214 => { vm.decode(0x9b215530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bc218u64) }
        0x102bc218 => { vm.decode(0xa8ce32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bc21cu64) }
        0x102bc21c => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bc220u64) }
        0x102bc220 => { vm.decode(0x88843afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bc224u64) }
        0x102bc224 => { vm.decode(0xb1115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bc228u64) }
        0x102bc228 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bc22cu64) }
        0x102bc22c => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bc230u64) }
        0x102bc230 => { vm.decode(0x488643afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bc234u64) }
        0x102bc234 => { vm.decode(0xa10c875u32); let d = 25i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc238u64) }
        0x102bc238 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102bc23cu64 + 4);
        vm.wr(37, 0x102bc23cu64 + 4);
        Ok(0x102bc23cu64) }
        0x102bc23c => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bc240u64) }
        0x102bc240 => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bc244u64) }
        0x102bc244 => { vm.decode(0x2280416fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bc248u64) }
        0x102bc248 => { vm.decode(0x9b2155b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bc24cu64) }
        0x102bc24c => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102bc250u64) }
        0x102bc250 => { vm.decode(0x28c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bc254u64) }
        0x102bc254 => { vm.decode(0xb311530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bc258u64) }
        0x102bc258 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bc25cu64) }
        0x102bc25c => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bc260u64) }
        0x102bc260 => { vm.decode(0xa10c875u32); let d = 25i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc264u64) }
        0x102bc264 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102bc268u64 + 4);
        vm.wr(37, 0x102bc268u64 + 4);
        Ok(0x102bc268u64) }
        0x102bc268 => { vm.decode(0x100036u32); let d = 16i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bc26cu64) }
        0x102bc26c => { vm.decode(0x518e911u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc270u64) }
        0x102bc270 => { vm.decode(0x45186911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc274u64) }
        0x102bc274 => { vm.decode(0x41a835u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc278u64) }
        0x102bc278 => { vm.decode(0x60043835u32); let d = 1i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc27cu64) }
        0x102bc27c => { vm.decode(0x459ce011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc280u64) }
        0x102bc280 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc284u64) }
        0x102bc284 => { vm.decode(0xc4802fb5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc288u64) }
        0x102bc288 => { vm.decode(0x40290fb5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc28cu64) }
        0x102bc28c => { vm.decode(0x1a814d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 28i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc290u64) }
        0x102bc290 => { vm.decode(0x44802fb5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc294u64) }
        0x102bc294 => { vm.decode(0x200c8835u32); let d = 2i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc298u64) }
        0x102bc298 => { vm.decode(0x488835u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc29cu64) }
        0x102bc29c => { vm.decode(0x4d104211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc2a0u64) }
        0x102bc2a0 => { vm.decode(0x20212fb5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc2a4u64) }
        0x102bc2a4 => { vm.decode(0xa0210fb5u32); let d = 2i16; let s = 2i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc2a8u64) }
        0x102bc2a8 => { vm.decode(0x24282fb5u32); let d = 2i16; let s = 2i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc2acu64) }
        0x102bc2ac => { vm.decode(0x28918d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 21i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc2b0u64) }
        0x102bc2b0 => { vm.decode(0x20210fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc2b4u64) }
        0x102bc2b4 => { vm.decode(0x40482835u32); let d = 3i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc2b8u64) }
        0x102bc2b8 => { vm.decode(0x204ca835u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc2bcu64) }
        0x102bc2bc => { vm.decode(0x4d14cb11u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc2c0u64) }
        0x102bc2c0 => { vm.decode(0x64a12fb5u32); let d = 3i16; let s = 3i16; let s2 = 7i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc2c4u64) }
        0x102bc2c4 => { vm.decode(0xf4a02fb5u32); let d = 3i16; let s = 3i16; let s2 = 30i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc2c8u64) }
        0x102bc2c8 => { vm.decode(0x60112fb5u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc2ccu64) }
        0x102bc2cc => { vm.decode(0x38114d35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 16i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc2d0u64) }
        0x102bc2d0 => { vm.decode(0x64202fb5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc2d4u64) }
        0x102bc2d4 => { vm.decode(0x21440835u32); let d = 4i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc2d8u64) }
        0x102bc2d8 => { vm.decode(0x41010835u32); let d = 4i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc2dcu64) }
        0x102bc2dc => { vm.decode(0xd144b11u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc2e0u64) }
        0x102bc2e0 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc2e4u64) }
        0x102bc2e4 => { vm.decode(0xd146911u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc2e8u64) }
        0x102bc2e8 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc2ecu64) }
        0x102bc2ec => { vm.decode(0xb20fb5u32); let d = 4i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc2f0u64) }
        0x102bc2f0 => { vm.decode(0xa248d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 9i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc2f4u64) }
        0x102bc2f4 => { vm.decode(0x132fb5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc2f8u64) }
        0x102bc2f8 => { vm.decode(0x41052835u32); let d = 5i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc2fcu64) }
        0x102bc2fc => { vm.decode(0x21052835u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc300u64) }
        0x102bc300 => { vm.decode(0x4590c311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc304u64) }
        0x102bc304 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc308u64) }
        0x102bc308 => { vm.decode(0x4514e911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc30cu64) }
        0x102bc30c => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc310u64) }
        0x102bc310 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc314u64) }
        0x102bc314 => { vm.decode(0x1a814d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 28i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc318u64) }
        0x102bc318 => { vm.decode(0x44800fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc31cu64) }
        0x102bc31c => { vm.decode(0x2100a835u32); let d = 5i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc320u64) }
        0x102bc320 => { vm.decode(0x61052835u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc324u64) }
        0x102bc324 => { vm.decode(0x44932fb5u32); let d = 5i16; let s = 5i16; let s2 = 7i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc328u64) }
        0x102bc328 => { vm.decode(0x45906111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc32cu64) }
        0x102bc32c => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc330u64) }
        0x102bc330 => { vm.decode(0x20a10fb5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc334u64) }
        0x102bc334 => { vm.decode(0x28918d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 21i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc338u64) }
        0x102bc338 => { vm.decode(0x20210fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc33cu64) }
        0x102bc33c => { vm.decode(0x41402835u32); let d = 5i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc340u64) }
        0x102bc340 => { vm.decode(0x2101a835u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc344u64) }
        0x102bc344 => { vm.decode(0x4514c311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc348u64) }
        0x102bc348 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc34cu64) }
        0x102bc34c => { vm.decode(0x4590e111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc350u64) }
        0x102bc350 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc354u64) }
        0x102bc354 => { vm.decode(0x64a10fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc358u64) }
        0x102bc358 => { vm.decode(0x38114d35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 16i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc35cu64) }
        0x102bc35c => { vm.decode(0x64202fb5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc360u64) }
        0x102bc360 => { vm.decode(0x21442835u32); let d = 5i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc364u64) }
        0x102bc364 => { vm.decode(0x61012835u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc368u64) }
        0x102bc368 => { vm.decode(0x45104b11u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc36cu64) }
        0x102bc36c => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc370u64) }
        0x102bc370 => { vm.decode(0x45906911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc374u64) }
        0x102bc374 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc378u64) }
        0x102bc378 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc37cu64) }
        0x102bc37c => { vm.decode(0xa248d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 9i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc380u64) }
        0x102bc380 => { vm.decode(0x132fb5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc384u64) }
        0x102bc384 => { vm.decode(0x41052835u32); let d = 5i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc388u64) }
        0x102bc388 => { vm.decode(0x21052835u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc38cu64) }
        0x102bc38c => { vm.decode(0x4510cb11u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc390u64) }
        0x102bc390 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc394u64) }
        0x102bc394 => { vm.decode(0x4590e911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc398u64) }
        0x102bc398 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc39cu64) }
        0x102bc39c => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc3a0u64) }
        0x102bc3a0 => { vm.decode(0x1a814d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 28i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc3a4u64) }
        0x102bc3a4 => { vm.decode(0x44800fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc3a8u64) }
        0x102bc3a8 => { vm.decode(0x2100a835u32); let d = 5i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc3acu64) }
        0x102bc3ac => { vm.decode(0x61052835u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc3b0u64) }
        0x102bc3b0 => { vm.decode(0x4598e911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc3b4u64) }
        0x102bc3b4 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc3b8u64) }
        0x102bc3b8 => { vm.decode(0x45946111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc3bcu64) }
        0x102bc3bc => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc3c0u64) }
        0x102bc3c0 => { vm.decode(0x20a10fb5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc3c4u64) }
        0x102bc3c4 => { vm.decode(0x28918d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 21i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc3c8u64) }
        0x102bc3c8 => { vm.decode(0x20210fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc3ccu64) }
        0x102bc3cc => { vm.decode(0x41402835u32); let d = 5i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc3d0u64) }
        0x102bc3d0 => { vm.decode(0x2101a835u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc3d4u64) }
        0x102bc3d4 => { vm.decode(0x45144311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc3d8u64) }
        0x102bc3d8 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc3dcu64) }
        0x102bc3dc => { vm.decode(0x4594e111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc3e0u64) }
        0x102bc3e0 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc3e4u64) }
        0x102bc3e4 => { vm.decode(0x64a10fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc3e8u64) }
        0x102bc3e8 => { vm.decode(0x38114d35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 16i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc3ecu64) }
        0x102bc3ec => { vm.decode(0x64202fb5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc3f0u64) }
        0x102bc3f0 => { vm.decode(0x21442835u32); let d = 5i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc3f4u64) }
        0x102bc3f4 => { vm.decode(0x61012835u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc3f8u64) }
        0x102bc3f8 => { vm.decode(0x459c6911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc3fcu64) }
        0x102bc3fc => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc400u64) }
        0x102bc400 => { vm.decode(0x45946911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc404u64) }
        0x102bc404 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc408u64) }
        0x102bc408 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc40cu64) }
        0x102bc40c => { vm.decode(0xa248d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 9i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc410u64) }
        0x102bc410 => { vm.decode(0x132fb5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc414u64) }
        0x102bc414 => { vm.decode(0x41052835u32); let d = 5i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc418u64) }
        0x102bc418 => { vm.decode(0x21052835u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc41cu64) }
        0x102bc41c => { vm.decode(0x45904311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc420u64) }
        0x102bc420 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc424u64) }
        0x102bc424 => { vm.decode(0x4594e911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc428u64) }
        0x102bc428 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc42cu64) }
        0x102bc42c => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc430u64) }
        0x102bc430 => { vm.decode(0x1a814d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 28i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc434u64) }
        0x102bc434 => { vm.decode(0x4860fb5u32); let d = 20i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc438u64) }
        0x102bc438 => { vm.decode(0x11a835u32); let d = 1i16; let s = 4i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc43cu64) }
        0x102bc43c => { vm.decode(0x60042835u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc440u64) }
        0x102bc440 => { vm.decode(0xd90cb11u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc444u64) }
        0x102bc444 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc448u64) }
        0x102bc448 => { vm.decode(0xd186111u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc44cu64) }
        0x102bc44c => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc450u64) }
        0x102bc450 => { vm.decode(0x40210fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc454u64) }
        0x102bc454 => { vm.decode(0x1881cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 21i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc458u64) }
        0x102bc458 => { vm.decode(0xe4840fb5u32); let d = 19i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc45cu64) }
        0x102bc45c => { vm.decode(0x2050b835u32); let d = 1i16; let s = 19i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc460u64) }
        0x102bc460 => { vm.decode(0x2000a835u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc464u64) }
        0x102bc464 => { vm.decode(0x459ce811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc468u64) }
        0x102bc468 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc46cu64) }
        0x102bc46c => { vm.decode(0x4518e011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc470u64) }
        0x102bc470 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc474u64) }
        0x102bc474 => { vm.decode(0x44210fb5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc478u64) }
        0x102bc478 => { vm.decode(0x18014d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc47cu64) }
        0x102bc47c => { vm.decode(0xa4052fb5u32); let d = 18i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc480u64) }
        0x102bc480 => { vm.decode(0x40543835u32); let d = 1i16; let s = 18i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc484u64) }
        0x102bc484 => { vm.decode(0x2000b835u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc488u64) }
        0x102bc488 => { vm.decode(0x45944211u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc48cu64) }
        0x102bc48c => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc490u64) }
        0x102bc490 => { vm.decode(0x45986011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc494u64) }
        0x102bc494 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc498u64) }
        0x102bc498 => { vm.decode(0x40110fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc49cu64) }
        0x102bc49c => { vm.decode(0x1a00cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 9i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc4a0u64) }
        0x102bc4a0 => { vm.decode(0xc4002fb5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc4a4u64) }
        0x102bc4a4 => { vm.decode(0x43353062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bc4a8u64) }
        0x102bc4a8 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bc4acu64) }
        0x102bc4ac => { vm.decode(0x56268759u32); let d = 21i16; let s = 1i16; let s2 = -1i16; let imm = 8136i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bc4b0u64) }
        0x102bc4b0 => { vm.decode(0x12101019u32); let d = 22i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bc4b4u64) }
        0x102bc4b4 => { vm.decode(0x4002406u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102bc4b8u64) }
        0x102bc4b8 => { vm.decode(0x2200031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102bc4bcu64).wrapping_add((imm * 4) as u64) } else { 0x102bc4bcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bc4bcu64) }
        0x102bc4bc => { vm.decode(0x6102236u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bc4c0u64) }
        0x102bc4c0 => { vm.decode(0x2a88851du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 221724i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102bc4c4u64) }
        0x102bc4c8 => { vm.decode(0xa024735u32); let d = 1i16; let s = 4i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc4ccu64) }
        0x102bc4cc => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102bc4d0u64) }
        0x102bc4d0 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc4d4u64) }
        0x102bc4d4 => { vm.decode(0x64114675u32); let d = 3i16; let s = 3i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc4d8u64) }
        0x102bc4d8 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc4dcu64) }
        0x102bc4dc => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102bc4e0u64) }
        0x102bc4e0 => { vm.decode(0x28c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bc4e4u64) }
        0x102bc4e4 => { vm.decode(0x4b2014f5u32); let d = 2i16; let s = 2i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102bc4e8u64) }
        0x102bc4e8 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102bc4ecu64) }
        0x102bc4ec => { vm.decode(0x8900036u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bc4f0u64) }
        0x102bc4f0 => { vm.decode(0x4002406u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102bc4f4u64) }
        0x102bc4f4 => { vm.decode(0x4048227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 41i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bc4f8u64).wrapping_add((imm * 4) as u64) } else { 0x102bc4f8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bc4f8u64) }
        0x102bc4f8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bc4fcu64) } }
        0x102bc4fc => { vm.decode(0x2084c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bc500u64) }
        0x102bc500 => { vm.decode(0xa16004f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102bc504u64) }
        0x102bc504 => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bc508u64) }
        0x102bc508 => { vm.decode(0x82c202fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bc50cu64) }
        0x102bc50c => { vm.decode(0x8b195530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bc510u64) }
        0x102bc510 => { vm.decode(0xb091530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bc514u64) }
        0x102bc514 => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102bc518u64) }
        0x102bc518 => { vm.decode(0x40020bb5u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc51cu64) }
        0x102bc51c => { vm.decode(0xc008bd75u32); let d = 30i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102bc520u64) }
        0x102bc520 => { vm.decode(0x288043afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bc524u64) }
        0x102bc524 => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102bc528u64) }
        0x102bc528 => { vm.decode(0x1c000011u32); let d = 17i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc52cu64) }
        0x102bc52c => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bc530u64) }
        0x102bc530 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102bc534u64) }
        0x102bc534 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102bc538u64 + 4);
        vm.wr(37, 0x102bc538u64 + 4);
        Ok(0x102bc538u64) }
        0x102bc538 => { vm.decode(0x660d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102bc53cu64) }
        0x102bc53c => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102bc540u64) }
        0x102bc540 => { vm.decode(0x28cc8b35u32); let d = 4i16; let s = 3i16; let s2 = 23i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc544u64) }
        0x102bc544 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bc548u64) }
        0x102bc548 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bc54cu64) }
        0x102bc54c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bc550u64).wrapping_add((imm * 4) as u64) } else { 0x102bc550u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bc550u64) }
        0x102bc550 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bc554u64) }
        0x102bc554 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bc558u64) }
        0x102bc558 => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc55cu64) }
        0x102bc55c => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102bc560u64) }
        0x102bc560 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bc564u64) }
        0x102bc564 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102bc568u64).wrapping_add((imm * 4) as u64) } else { 0x102bc568u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bc568u64) }
        0x102bc568 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bc56cu64) } }
        0x102bc56c => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bc570u64) }
        0x102bc570 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bc574u64) }
        0x102bc574 => { vm.decode(0x739c34a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -44i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102bc578u64).wrapping_add((imm * 4) as u64) } else { 0x102bc578u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bc578u64) }
        0x102bc578 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bc57cu64) }
        0x102bc57c => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102bc580u64) }
        0x102bc580 => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102bc584u64) }
        0x102bc584 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102bc588u64) }
        0x102bc588 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102bc58cu64) }
        0x102bc58c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102bc590u64).wrapping_add((imm * 4) as u64) } else { 0x102bc590u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102bc590u64) }
        0x102bc590 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bc594u64) } }
        0x102bc594 => { vm.decode(0xa28911du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 221512i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102bc598u64) }
        0x102bc598 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102bc59cu64) } }
        _ => Err(Error::UnknownPc(pc)),
    }
}
