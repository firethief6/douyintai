#![cfg_attr(rustfmt, rustfmt::skip)]
// 由 scripts/medusa-vm/gen_native.py 生成，勿手改。
use super::super::Vm;
use super::super::Error;

#[allow(clippy::all, unused_parens, unused_variables, unused_mut)]
pub fn exec(vm: &mut Vm, pc: u64) -> Result<u64, Error> {
    match pc {
        0x1026d390 => { vm.decode(0x7f29cf19u32); let d = 29i16; let s = 29i16; let s2 = -1i16; let imm = -448i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1026d394u64) }
        0x1026d394 => { vm.decode(0x9918ddf0u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1026d398u64) }
        0x1026d398 => { vm.decode(0x8910ddf0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1026d39cu64) }
        0x1026d39c => { vm.decode(0x9908ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1026d3a0u64) }
        0x1026d3a0 => { vm.decode(0x8900ddb0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1026d3a4u64) }
        0x1026d3a4 => { vm.decode(0x9918dc30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1026d3a8u64) }
        0x1026d3a8 => { vm.decode(0x8910dc30u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1026d3acu64) }
        0x1026d3ac => { vm.decode(0x99089d30u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1026d3b0u64) }
        0x1026d3b0 => { vm.decode(0x89009d30u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1026d3b4u64) }
        0x1026d3b4 => { vm.decode(0x99381cb0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1026d3b8u64) }
        0x1026d3b8 => { vm.decode(0x89301cb0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1026d3bcu64) }
        0x1026d3bc => { vm.decode(0x288212fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1026d3c0u64) }
        0x1026d3c0 => { vm.decode(0x1385738eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -7733248i64;
        vm.st(d, imm as u64);
        Ok(0x1026d3c4u64) }
        0x1026d3c4 => { vm.decode(0x273525d8u32); let d = 21i16; let s = 1i16; let s2 = -1i16; let imm = 40816i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x1026d3c8u64) }
        0x1026d3c8 => { vm.decode(0x8b2604f5u32); let d = 22i16; let s = 21i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1026d3ccu64) }
        0x1026d3cc => { vm.decode(0x9500011u32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1026d3d0u64) }
        0x1026d3d0 => { vm.decode(0x82002431u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 266i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1026d3d4u64).wrapping_add((imm * 4) as u64) } else { 0x1026d3d4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1026d3d4u64) }
        0x1026d3d4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1026d3d8u64) } }
        0x1026d3d8 => { vm.decode(0xc00d875u32); let d = 17i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1026d3dcu64) }
        0x1026d7fc => { vm.decode(0x41500011u32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1026d800u64) }
        0x1026d800 => { vm.decode(0x5022u32); let d = 2i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1026d804u64) }
        0x1026d804 => { vm.decode(0x4284c36fu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1026d808u64) }
        0x1026d808 => { vm.decode(0x6284e36fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1026d80cu64) }
        0x1026d80c => { vm.decode(0x4888c32fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1026d810u64) }
        0x1026d810 => { vm.decode(0x6888e32fu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1026d814u64) }
        0x1026d814 => { vm.decode(0x4a82c32fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1026d818u64) }
        0x1026d818 => { vm.decode(0x6a82e32fu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1026d81cu64) }
        0x1026d81c => { vm.decode(0x488ec32fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1026d820u64) }
        0x1026d820 => { vm.decode(0x688ee32fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1026d824u64) }
        0x1026d824 => { vm.decode(0x4a8ecb2fu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1026d828u64) }
        0x1026d828 => { vm.decode(0x6a8eeb2fu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1026d82cu64) }
        0x1026d82c => { vm.decode(0x54104cf5u32); let d = -1i16; let s = -1i16; let s2 = -1i16; let imm = 0i64;
        vm.jump_flag = 2;
        vm.wr(36, vm.slots[31]);
        vm.wr(35, 2);
        Ok(0x1026d830u64) }
        0x1026d830 => { vm.decode(0x730b8159u32); let d = 29i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1026d834u64) }
        0x1026d834 => { vm.decode(0x0u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1026d838u64) }
        0x10270fb0 => { vm.decode(0x7b7bca19u32); let d = 29i16; let s = 29i16; let s2 = -1i16; let imm = -2896i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10270fb4u64) }
        0x10270fb4 => { vm.decode(0x9b295d70u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 2888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10270fb8u64) }
        0x10270fb8 => { vm.decode(0x8b215d70u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10270fbcu64) }
        0x10270fbc => { vm.decode(0x9b195db0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10270fc0u64) }
        0x10270fc0 => { vm.decode(0x8b115db0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10270fc4u64) }
        0x10270fc4 => { vm.decode(0x9b095cb0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10270fc8u64) }
        0x10270fc8 => { vm.decode(0x8b015cb0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10270fccu64) }
        0x10270fcc => { vm.decode(0x9b191d30u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10270fd0u64) }
        0x10270fd0 => { vm.decode(0x8b111d30u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10270fd4u64) }
        0x10270fd4 => { vm.decode(0x9b091c30u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10270fd8u64) }
        0x10270fd8 => { vm.decode(0x8b011c30u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10270fdcu64) }
        0x10270fdc => { vm.decode(0xc005875u32); let d = 16i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10270fe0u64) }
        0x10270fe0 => { vm.decode(0x1381f6ceu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -18087936i64;
        vm.st(d, imm as u64);
        Ok(0x10270fe4u64) }
        0x10270fe4 => { vm.decode(0x21502358u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 51528i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10270fe8u64) }
        0x10270fe8 => { vm.decode(0x1f81f6ceu32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = -18087936i64;
        vm.st(d, imm as u64);
        Ok(0x10270fecu64) }
        0x10270fec => { vm.decode(0x88012fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10270ff0u64) }
        0x10270ff0 => { vm.decode(0x9009d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10270ff4u64) }
        0x10270ff4 => { vm.decode(0x8a0404f5u32); let d = 18i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10270ff8u64) }
        0x10270ff8 => { vm.decode(0x22a0002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10270ffcu64) }
        0x10270ffc => { vm.decode(0x191814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271000u64) }
        0x10271000 => { vm.decode(0x22a0202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271004u64) }
        0x10271004 => { vm.decode(0x1b2814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271008u64) }
        0x10271008 => { vm.decode(0x20a4002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027100cu64) }
        0x1027100c => { vm.decode(0x1b381430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271010u64) }
        0x10271010 => { vm.decode(0x20a4202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271014u64) }
        0x10271014 => { vm.decode(0x1b281430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271018u64) }
        0x10271018 => { vm.decode(0x22a4002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027101cu64) }
        0x1027101c => { vm.decode(0x1b1814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271020u64) }
        0x10271020 => { vm.decode(0x22a4202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271024u64) }
        0x10271024 => { vm.decode(0x1b1014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271028u64) }
        0x10271028 => { vm.decode(0x20a0006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027102cu64) }
        0x1027102c => { vm.decode(0x1b2014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271030u64) }
        0x10271030 => { vm.decode(0x20a0206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271034u64) }
        0x10271034 => { vm.decode(0x1b301430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271038u64) }
        0x10271038 => { vm.decode(0x22a0006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027103cu64) }
        0x1027103c => { vm.decode(0x19109cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271040u64) }
        0x10271040 => { vm.decode(0x22a0206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271044u64) }
        0x10271044 => { vm.decode(0x1b201430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271048u64) }
        0x10271048 => { vm.decode(0x20a4006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027104cu64) }
        0x1027104c => { vm.decode(0x19389c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271050u64) }
        0x10271050 => { vm.decode(0x20a4206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271054u64) }
        0x10271054 => { vm.decode(0x1b181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271058u64) }
        0x10271058 => { vm.decode(0x22a4006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027105cu64) }
        0x1027105c => { vm.decode(0x19209c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271060u64) }
        0x10271060 => { vm.decode(0x22a4206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271064u64) }
        0x10271064 => { vm.decode(0x19281430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271068u64) }
        0x10271068 => { vm.decode(0x28a0002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027106cu64) }
        0x1027106c => { vm.decode(0x19289cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271070u64) }
        0x10271070 => { vm.decode(0x28a0202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271074u64) }
        0x10271074 => { vm.decode(0x19301c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271078u64) }
        0x10271078 => { vm.decode(0x2aa0002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027107cu64) }
        0x1027107c => { vm.decode(0x19389430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271080u64) }
        0x10271080 => { vm.decode(0x2aa0202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271084u64) }
        0x10271084 => { vm.decode(0x1b101430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271088u64) }
        0x10271088 => { vm.decode(0x28a4002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027108cu64) }
        0x1027108c => { vm.decode(0x19309430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271090u64) }
        0x10271090 => { vm.decode(0x28a4202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271094u64) }
        0x10271094 => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271098u64) }
        0x10271098 => { vm.decode(0x2aa4002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027109cu64) }
        0x1027109c => { vm.decode(0x192014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102710a0u64) }
        0x102710a0 => { vm.decode(0x408a082fu32); let d = 30i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102710a4u64) }
        0x102710a4 => { vm.decode(0x2aa4202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102710a8u64) }
        0x102710a8 => { vm.decode(0x19209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102710acu64) }
        0x102710ac => { vm.decode(0x28a0006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102710b0u64) }
        0x102710b0 => { vm.decode(0x19381430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102710b4u64) }
        0x102710b4 => { vm.decode(0x28a0206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102710b8u64) }
        0x102710b8 => { vm.decode(0x19089cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102710bcu64) }
        0x102710bc => { vm.decode(0x2aa0006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102710c0u64) }
        0x102710c0 => { vm.decode(0x19301430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102710c4u64) }
        0x102710c4 => { vm.decode(0x2aa0206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102710c8u64) }
        0x102710c8 => { vm.decode(0x19089430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102710ccu64) }
        0x102710cc => { vm.decode(0x28a4006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102710d0u64) }
        0x102710d0 => { vm.decode(0x19009430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102710d4u64) }
        0x102710d4 => { vm.decode(0x28a4206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102710d8u64) }
        0x102710d8 => { vm.decode(0x193814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102710dcu64) }
        0x102710dc => { vm.decode(0x2aa4006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102710e0u64) }
        0x102710e0 => { vm.decode(0x19281c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102710e4u64) }
        0x102710e4 => { vm.decode(0x2aa4206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102710e8u64) }
        0x102710e8 => { vm.decode(0x19201c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102710ecu64) }
        0x102710ec => { vm.decode(0x20a0802fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102710f0u64) }
        0x102710f0 => { vm.decode(0x19181cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102710f4u64) }
        0x102710f4 => { vm.decode(0x20a0a02fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102710f8u64) }
        0x102710f8 => { vm.decode(0x19101cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102710fcu64) }
        0x102710fc => { vm.decode(0x22a0802fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271100u64) }
        0x10271100 => { vm.decode(0x19081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271104u64) }
        0x10271104 => { vm.decode(0x20a0202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271108u64) }
        0x10271108 => { vm.decode(0x1b3014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027110cu64) }
        0x1027110c => { vm.decode(0x20a0002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271110u64) }
        0x10271110 => { vm.decode(0x288212fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271114u64) }
        0x10271114 => { vm.decode(0x91015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271118u64) }
        0x10271118 => { vm.decode(0x8c012fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027111cu64) }
        0x1027111c => { vm.decode(0xb0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271120u64) }
        0x10271120 => { vm.decode(0x28c012fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271124u64) }
        0x10271124 => { vm.decode(0x9289530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271128u64) }
        0x10271128 => { vm.decode(0x28c212fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027112cu64) }
        0x1027112c => { vm.decode(0x9209530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271130u64) }
        0x10271130 => { vm.decode(0x88016fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271134u64) }
        0x10271134 => { vm.decode(0x91895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271138u64) }
        0x10271138 => { vm.decode(0x88216fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027113cu64) }
        0x1027113c => { vm.decode(0x91095b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271140u64) }
        0x10271140 => { vm.decode(0x288016fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271144u64) }
        0x10271144 => { vm.decode(0x90895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271148u64) }
        0x10271148 => { vm.decode(0x288216fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027114cu64) }
        0x1027114c => { vm.decode(0x90095b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271150u64) }
        0x10271150 => { vm.decode(0x8c016fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271154u64) }
        0x10271154 => { vm.decode(0x9189530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271158u64) }
        0x10271158 => { vm.decode(0x8c216fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027115cu64) }
        0x1027115c => { vm.decode(0x9109530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271160u64) }
        0x10271160 => { vm.decode(0x28c016fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271164u64) }
        0x10271164 => { vm.decode(0x9009db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271168u64) }
        0x10271168 => { vm.decode(0x28c216fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027116cu64) }
        0x1027116c => { vm.decode(0x92095b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271170u64) }
        0x10271170 => { vm.decode(0x888012fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271174u64) }
        0x10271174 => { vm.decode(0x9001db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271178u64) }
        0x10271178 => { vm.decode(0x888212fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027117cu64) }
        0x1027117c => { vm.decode(0x9181d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271180u64) }
        0x10271180 => { vm.decode(0xa88012fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271184u64) }
        0x10271184 => { vm.decode(0x9101d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271188u64) }
        0x10271188 => { vm.decode(0xa88212fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027118cu64) }
        0x1027118c => { vm.decode(0x9081d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271190u64) }
        0x10271190 => { vm.decode(0x88c012fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271194u64) }
        0x10271194 => { vm.decode(0x9001d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271198u64) }
        0x10271198 => { vm.decode(0x88c212fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027119cu64) }
        0x1027119c => { vm.decode(0x93895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102711a0u64) }
        0x102711a0 => { vm.decode(0xa8c012fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102711a4u64) }
        0x102711a4 => { vm.decode(0x93095b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102711a8u64) }
        0x102711a8 => { vm.decode(0xa8c212fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102711acu64) }
        0x102711ac => { vm.decode(0x92895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102711b0u64) }
        0x102711b0 => { vm.decode(0x8c212fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102711b4u64) }
        0x102711b4 => { vm.decode(0xb001530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102711b8u64) }
        0x102711b8 => { vm.decode(0x628a012fu32); let d = 23i16; let s = 5i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102711bcu64) }
        0x102711bc => { vm.decode(0x88212fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102711c0u64) }
        0x102711c0 => { vm.decode(0x2028402fu32); let d = 3i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102711c4u64) }
        0x102711c4 => { vm.decode(0x82142019u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = 46i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102711c8u64) }
        0x102711c8 => { vm.decode(0xb39d4b0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 2808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102711ccu64) }
        0x102711cc => { vm.decode(0x2f7d13d8u32); let d = 21i16; let s = 7i16; let s2 = -1i16; let imm = 51192i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102711d0u64) }
        0x102711d0 => { vm.decode(0x8b2004f5u32); let d = 2i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102711d4u64) }
        0x102711d4 => { vm.decode(0xb3195b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102711d8u64) }
        0x102711d8 => { vm.decode(0x64391019u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102711dcu64) }
        0x102711dc => { vm.decode(0xb2995b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102711e0u64) }
        0x102711e0 => { vm.decode(0x1b2195b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102711e4u64) }
        0x102711e4 => { vm.decode(0xa92604f5u32); let d = 22i16; let s = 21i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102711e8u64) }
        0x102711e8 => { vm.decode(0x741b8519u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2784i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102711ecu64) }
        0x102711ec => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102711f0u64) }
        0x102711f0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102711f4u64) }
        0x102711f4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102711f8u64 + 4);
        vm.wr(37, 0x102711f8u64 + 4);
        Ok(0x102711f8u64) }
        0x102711f8 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102711fcu64) }
        0x102711fc => { vm.decode(0x22a0426fu32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271200u64) }
        0x10271200 => { vm.decode(0x19201cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271204u64) }
        0x10271204 => { vm.decode(0x20a0626fu32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271208u64) }
        0x10271208 => { vm.decode(0x19381c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027120cu64) }
        0x1027120c => { vm.decode(0x59d0c811u32); let d = 19i16; let s = 30i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10271210u64) }
        0x10271210 => { vm.decode(0x22a4622fu32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271214u64) }
        0x10271214 => { vm.decode(0x192814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271218u64) }
        0x10271218 => { vm.decode(0x22a4422fu32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027121cu64) }
        0x1027121c => { vm.decode(0x19189cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271220u64) }
        0x10271220 => { vm.decode(0x20a4622fu32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271224u64) }
        0x10271224 => { vm.decode(0x19301cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271228u64) }
        0x10271228 => { vm.decode(0x20a4422fu32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027122cu64) }
        0x1027122c => { vm.decode(0x19089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271230u64) }
        0x10271230 => { vm.decode(0x42a2622fu32); let d = 20i16; let s = 30i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271234u64) }
        0x10271234 => { vm.decode(0x22a0422fu32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271238u64) }
        0x10271238 => { vm.decode(0x19201430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027123cu64) }
        0x1027123c => { vm.decode(0x20a0622fu32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271240u64) }
        0x10271240 => { vm.decode(0x19389cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271244u64) }
        0x10271244 => { vm.decode(0x89385df0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271248u64) }
        0x10271248 => { vm.decode(0x20a0422fu32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027124cu64) }
        0x1027124c => { vm.decode(0x19309c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271250u64) }
        0x10271250 => { vm.decode(0x2020402fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271254u64) }
        0x10271254 => { vm.decode(0x121119u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 224i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271258u64) }
        0x10271258 => { vm.decode(0xb3995b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027125cu64) }
        0x1027125c => { vm.decode(0xab2124f5u32); let d = 2i16; let s = 21i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10271260u64) }
        0x10271260 => { vm.decode(0xb3195b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271264u64) }
        0x10271264 => { vm.decode(0x60291159u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1344i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271268u64) }
        0x10271268 => { vm.decode(0x93015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027126cu64) }
        0x1027126c => { vm.decode(0xb2995b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271270u64) }
        0x10271270 => { vm.decode(0x1b2194b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271274u64) }
        0x10271274 => { vm.decode(0x8b085530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271278u64) }
        0x10271278 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027127cu64) }
        0x1027127c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271280u64) }
        0x10271280 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10271284u64 + 4);
        vm.wr(37, 0x10271284u64 + 4);
        Ok(0x10271284u64) }
        0x10271284 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271288u64) }
        0x10271288 => { vm.decode(0x1381f6ceu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -18087936i64;
        vm.st(d, imm as u64);
        Ok(0x1027128cu64) }
        0x1027128c => { vm.decode(0x20112318u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 51232i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10271290u64) }
        0x10271290 => { vm.decode(0x488ac32fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271294u64) }
        0x10271294 => { vm.decode(0x824524f5u32); let d = 17i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10271298u64) }
        0x10271298 => { vm.decode(0x733d1059u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 1320i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027129cu64) }
        0x1027129c => { vm.decode(0x2000412fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102712a0u64) }
        0x102712a0 => { vm.decode(0x8b39d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102712a4u64) }
        0x102712a4 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102712a8u64) }
        0x102712a8 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102712acu64) }
        0x102712ac => { vm.decode(0xa96604f5u32); let d = 23i16; let s = 21i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102712b0u64) }
        0x102712b0 => { vm.decode(0x744b1519u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102712b4u64) }
        0x102712b4 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102712b8u64) }
        0x102712b8 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102712bcu64) }
        0x102712bc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102712c0u64 + 4);
        vm.wr(37, 0x102712c0u64 + 4);
        Ok(0x102712c0u64) }
        0x102712c0 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102712c4u64) }
        0x102712c4 => { vm.decode(0x2000412fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102712c8u64) }
        0x102712c8 => { vm.decode(0x60691059u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1296i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102712ccu64) }
        0x102712cc => { vm.decode(0x9289d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102712d0u64) }
        0x102712d0 => { vm.decode(0xb399530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102712d4u64) }
        0x102712d4 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102712d8u64) }
        0x102712d8 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102712dcu64) }
        0x102712dc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102712e0u64) }
        0x102712e0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102712e4u64 + 4);
        vm.wr(37, 0x102712e4u64 + 4);
        Ok(0x102712e4u64) }
        0x102712e4 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102712e8u64) }
        0x102712e8 => { vm.decode(0x228043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102712ecu64) }
        0x102712ec => { vm.decode(0xa16004f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102712f0u64) }
        0x102712f0 => { vm.decode(0x1b301430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102712f4u64) }
        0x102712f4 => { vm.decode(0x208443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102712f8u64) }
        0x102712f8 => { vm.decode(0xa16004f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102712fcu64) }
        0x102712fc => { vm.decode(0x1b2014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271300u64) }
        0x10271300 => { vm.decode(0x228443afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271304u64) }
        0x10271304 => { vm.decode(0xa16004f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10271308u64) }
        0x10271308 => { vm.decode(0x1b1014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027130cu64) }
        0x1027130c => { vm.decode(0x228463afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271310u64) }
        0x10271310 => { vm.decode(0xa92004f5u32); let d = 2i16; let s = 21i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10271314u64) }
        0x10271314 => { vm.decode(0x208063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271318u64) }
        0x10271318 => { vm.decode(0xa96004f5u32); let d = 3i16; let s = 21i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027131cu64) }
        0x1027131c => { vm.decode(0x228063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271320u64) }
        0x10271320 => { vm.decode(0xa16204f5u32); let d = 5i16; let s = 21i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10271324u64) }
        0x10271324 => { vm.decode(0x208463efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271328u64) }
        0x10271328 => { vm.decode(0xa12204f5u32); let d = 4i16; let s = 21i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027132cu64) }
        0x1027132c => { vm.decode(0x208463afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271330u64) }
        0x10271330 => { vm.decode(0xa16004f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10271334u64) }
        0x10271334 => { vm.decode(0x1b0814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271338u64) }
        0x10271338 => { vm.decode(0x836524f5u32); let d = 17i16; let s = 21i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027133cu64) }
        0x1027133c => { vm.decode(0x8002036u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10271340u64) }
        0x10271340 => { vm.decode(0x65790119u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 1648i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271344u64) }
        0x10271344 => { vm.decode(0x741f1159u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271348u64) }
        0x10271348 => { vm.decode(0x76691119u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 1616i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027134cu64) }
        0x1027134c => { vm.decode(0x9b3055b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271350u64) }
        0x10271350 => { vm.decode(0x8b285530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271354u64) }
        0x10271354 => { vm.decode(0x1b1815b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271358u64) }
        0x10271358 => { vm.decode(0xb0015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027135cu64) }
        0x1027135c => { vm.decode(0x3255162u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10271360u64) }
        0x10271360 => { vm.decode(0x89285df0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271364u64) }
        0x10271364 => { vm.decode(0x1918dc30u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271368u64) }
        0x10271368 => { vm.decode(0x910dc30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027136cu64) }
        0x1027136c => { vm.decode(0x890854b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271370u64) }
        0x10271370 => { vm.decode(0x8a286031u32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 26i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10271374u64).wrapping_add((imm * 4) as u64) } else { 0x10271374u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10271374u64) }
        0x10271374 => { vm.decode(0x8b2815b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271378u64) }
        0x10271378 => { vm.decode(0x2284632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027137cu64) }
        0x102713dc => { vm.decode(0x319900eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 372310016i64;
        vm.st(d, imm as u64);
        Ok(0x102713e0u64) }
        0x102713e0 => { vm.decode(0xc016c699u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 14507i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102713e4u64) }
        0x102713e4 => { vm.decode(0x41108091u32); let d = 2i16; let s = 20i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102713e8u64) }
        0x102713e8 => { vm.decode(0x4c009875u32); let d = 19i16; let s = 0i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102713ecu64) }
        0x102713ec => { vm.decode(0x2288402fu32); let d = 3i16; let s = 20i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102713f0u64) }
        0x102713f0 => { vm.decode(0x422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102713f4u64) }
        0x102713f4 => { vm.decode(0xc804875u32); let d = 20i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102713f8u64) }
        0x102713f8 => { vm.decode(0x688844eu32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 7143424i64;
        vm.st(d, imm as u64);
        Ok(0x102713fcu64) }
        0x102713fc => { vm.decode(0xc22188d9u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 17667i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271400u64) }
        0x10271400 => { vm.decode(0xb842ad75u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 23i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10271404u64) }
        0x10271404 => { vm.decode(0x422711d9u32); let d = 6i16; let s = 5i16; let s2 = -1i16; let imm = 1481i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271408u64) }
        0x10271408 => { vm.decode(0x1691a54eu32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1484652544i64;
        vm.st(d, imm as u64);
        Ok(0x1027140cu64) }
        0x1027140c => { vm.decode(0x92429d75u32); let d = 7i16; let s = 6i16; let s2 = -1i16; let imm = 18i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10271410u64) }
        0x10271410 => { vm.decode(0xc814e5d9u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -22161i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271414u64) }
        0x10271414 => { vm.decode(0xce31a0d9u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = -30937i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271418u64) }
        0x10271418 => { vm.decode(0x422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1027141cu64) }
        0x1027141c => { vm.decode(0xc10410eu32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 9568256i64;
        vm.st(d, imm as u64);
        Ok(0x10271420u64) }
        0x10271420 => { vm.decode(0x29175d9u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 10597i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271424u64) }
        0x10271424 => { vm.decode(0x12029d75u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10271428u64) }
        0x10271428 => { vm.decode(0x86937b99u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 29415i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027142cu64) }
        0x1027142c => { vm.decode(0xba029d75u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 23i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10271430u64) }
        0x10271430 => { vm.decode(0xc6c3b999u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 17111i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271434u64) }
        0x10271434 => { vm.decode(0x191855b0u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271438u64) }
        0x10271438 => { vm.decode(0x4156e219u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 12476i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027143cu64) }
        0x1027143c => { vm.decode(0x191014f0u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271440u64) }
        0x10271440 => { vm.decode(0x10422d75u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10271444u64) }
        0x10271444 => { vm.decode(0xc002ebd9u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 29127i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271448u64) }
        0x10271448 => { vm.decode(0x20422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 17i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1027144cu64) }
        0x1027144c => { vm.decode(0x4c24ed99u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -4531i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271450u64) }
        0x10271450 => { vm.decode(0x190014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271454u64) }
        0x10271454 => { vm.decode(0x2b3014b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271458u64) }
        0x10271458 => { vm.decode(0x3b109430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027145cu64) }
        0x1027145c => { vm.decode(0x3b089470u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 1672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271460u64) }
        0x10271460 => { vm.decode(0x3b00d530u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 1664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271464u64) }
        0x10271464 => { vm.decode(0x84b5a399u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 5743i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271468u64) }
        0x10271468 => { vm.decode(0x19101430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027146cu64) }
        0x1027146c => { vm.decode(0x3b3814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271470u64) }
        0x10271470 => { vm.decode(0x664b8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271474u64) }
        0x10271474 => { vm.decode(0x19185430u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271478u64) }
        0x10271478 => { vm.decode(0x2004e12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027147cu64) }
        0x1027147c => { vm.decode(0xb199470u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 2712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271480u64) }
        0x10271480 => { vm.decode(0x1b0195b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271484u64) }
        0x10271484 => { vm.decode(0x2a3d5022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10271488u64) }
        0x10271488 => { vm.decode(0x1b119430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027148cu64) }
        0x1027148c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271490u64) }
        0x10271490 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10271494u64 + 4);
        vm.wr(37, 0x10271494u64 + 4);
        Ok(0x10271494u64) }
        0x10271494 => { vm.decode(0x48a08875u32); let d = 23i16; let s = 0i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271498u64) }
        0x10271498 => { vm.decode(0x66598519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2672i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027149cu64) }
        0x1027149c => { vm.decode(0x19085430u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102714a0u64) }
        0x102714a0 => { vm.decode(0x2204c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102714a4u64) }
        0x102714a4 => { vm.decode(0x8b01d530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102714a8u64) }
        0x102714a8 => { vm.decode(0x9b3955b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102714acu64) }
        0x102714ac => { vm.decode(0x1b3114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102714b0u64) }
        0x102714b0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102714b4u64) }
        0x102714b4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102714b8u64 + 4);
        vm.wr(37, 0x102714b8u64 + 4);
        Ok(0x102714b8u64) }
        0x102714b8 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102714bcu64) }
        0x102714bc => { vm.decode(0x66498519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102714c0u64) }
        0x102714c0 => { vm.decode(0x19005430u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102714c4u64) }
        0x102714c4 => { vm.decode(0x4000236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102714c8u64) }
        0x102714c8 => { vm.decode(0x2a00616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102714ccu64) }
        0x102714cc => { vm.decode(0x6a1d5022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102714d0u64) }
        0x102714d0 => { vm.decode(0x8b2155b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102714d4u64) }
        0x102714d4 => { vm.decode(0x8b391530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102714d8u64) }
        0x102714d8 => { vm.decode(0x1b311430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102714dcu64) }
        0x102714dc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102714e0u64) }
        0x102714e0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102714e4u64 + 4);
        vm.wr(37, 0x102714e4u64 + 4);
        Ok(0x102714e4u64) }
        0x102714e4 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102714e8u64) }
        0x102714e8 => { vm.decode(0x76590419u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102714ecu64) }
        0x102714ec => { vm.decode(0x2204416fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102714f0u64) }
        0x102714f0 => { vm.decode(0x8b211530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102714f4u64) }
        0x102714f4 => { vm.decode(0x8b1955f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102714f8u64) }
        0x102714f8 => { vm.decode(0x1b1114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102714fcu64) }
        0x102714fc => { vm.decode(0x8643afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271500u64) }
        0x10271500 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271504u64) }
        0x10271504 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10271508u64 + 4);
        vm.wr(37, 0x10271508u64 + 4);
        Ok(0x10271508u64) }
        0x10271508 => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027150cu64) }
        0x1027150c => { vm.decode(0x774b1519u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271510u64) }
        0x10271510 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271514u64) }
        0x10271514 => { vm.decode(0x8b399530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271518u64) }
        0x10271518 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027151cu64) }
        0x1027151c => { vm.decode(0x628c43efu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271520u64) }
        0x10271520 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271524u64) }
        0x10271524 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271528u64) }
        0x10271528 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027152cu64 + 4);
        vm.wr(37, 0x1027152cu64 + 4);
        Ok(0x1027152cu64) }
        0x1027152c => { vm.decode(0xb0d875u32); let d = 5i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271530u64) }
        0x10271530 => { vm.decode(0x88436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271534u64) }
        0x10271534 => { vm.decode(0x8408091u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10271538u64) }
        0x10271538 => { vm.decode(0x228002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027153cu64) }
        0x1027153c => { vm.decode(0x208c432fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271540u64) }
        0x10271540 => { vm.decode(0x3b109530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271544u64) }
        0x10271544 => { vm.decode(0x228c432fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271548u64) }
        0x10271548 => { vm.decode(0x3b089530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027154cu64) }
        0x1027154c => { vm.decode(0x228c632fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271550u64) }
        0x10271550 => { vm.decode(0x3b009530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271554u64) }
        0x10271554 => { vm.decode(0x2288432fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271558u64) }
        0x10271558 => { vm.decode(0x3b3815b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027155cu64) }
        0x1027155c => { vm.decode(0x2b3014b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271560u64) }
        0x10271560 => { vm.decode(0x200ce12fu32); let d = 3i16; let s = 17i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271564u64) }
        0x10271564 => { vm.decode(0x9b19d530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271568u64) }
        0x10271568 => { vm.decode(0xb0195b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027156cu64) }
        0x1027156c => { vm.decode(0x2b3d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10271570u64) }
        0x10271570 => { vm.decode(0x1b119530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271574u64) }
        0x10271574 => { vm.decode(0xa82c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271578u64) }
        0x10271578 => { vm.decode(0x2282632fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027157cu64) }
        0x1027157c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271580u64) }
        0x10271580 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10271584u64 + 4);
        vm.wr(37, 0x10271584u64 + 4);
        Ok(0x10271584u64) }
        0x10271584 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10271588u64) } }
        0x10271588 => { vm.decode(0x2204c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027158cu64) }
        0x1027158c => { vm.decode(0x8b01d530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271590u64) }
        0x10271590 => { vm.decode(0x9b3955b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271594u64) }
        0x10271594 => { vm.decode(0x1b3114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271598u64) }
        0x10271598 => { vm.decode(0xa82e32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027159cu64) }
        0x1027159c => { vm.decode(0x2082632fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102715a0u64) }
        0x102715a0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102715a4u64) }
        0x102715a4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102715a8u64 + 4);
        vm.wr(37, 0x102715a8u64 + 4);
        Ok(0x102715a8u64) }
        0x102715a8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102715acu64) } }
        0x102715ac => { vm.decode(0x2a00616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102715b0u64) }
        0x102715b0 => { vm.decode(0x4000236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102715b4u64) }
        0x102715b4 => { vm.decode(0x6a1d5022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102715b8u64) }
        0x102715b8 => { vm.decode(0x8b2155b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102715bcu64) }
        0x102715bc => { vm.decode(0x8b391530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102715c0u64) }
        0x102715c0 => { vm.decode(0x1b311430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102715c4u64) }
        0x102715c4 => { vm.decode(0x28663afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102715c8u64) }
        0x102715c8 => { vm.decode(0x2082432fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102715ccu64) }
        0x102715cc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102715d0u64) }
        0x102715d0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102715d4u64 + 4);
        vm.wr(37, 0x102715d4u64 + 4);
        Ok(0x102715d4u64) }
        0x102715d4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102715d8u64) } }
        0x102715d8 => { vm.decode(0x2204416fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102715dcu64) }
        0x102715dc => { vm.decode(0x8b211530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102715e0u64) }
        0x102715e0 => { vm.decode(0x888e36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102715e4u64) }
        0x102715e4 => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102715e8u64) }
        0x102715e8 => { vm.decode(0x1b1114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102715ecu64) }
        0x102715ec => { vm.decode(0x8643afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102715f0u64) }
        0x102715f0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102715f4u64) }
        0x102715f4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102715f8u64 + 4);
        vm.wr(37, 0x102715f8u64 + 4);
        Ok(0x102715f8u64) }
        0x102715f8 => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102715fcu64) }
        0x102715fc => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271600u64) }
        0x10271600 => { vm.decode(0x8b399530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271604u64) }
        0x10271604 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271608u64) }
        0x10271608 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027160cu64) }
        0x1027160c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271610u64) }
        0x10271610 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10271614u64 + 4);
        vm.wr(37, 0x10271614u64 + 4);
        Ok(0x10271614u64) }
        0x10271614 => { vm.decode(0xb0d875u32); let d = 5i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271618u64) }
        0x10271618 => { vm.decode(0x2a84c32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027161cu64) }
        0x1027161c => { vm.decode(0xa92604f5u32); let d = 22i16; let s = 21i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10271620u64) }
        0x10271620 => { vm.decode(0xd186011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10271624u64) }
        0x10271624 => { vm.decode(0x31d3c22u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1564i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10271628u64) }
        0x10271628 => { vm.decode(0x200820eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 538968064i64;
        vm.st(d, imm as u64);
        Ok(0x1027162cu64) }
        0x1027162c => { vm.decode(0x20416158u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2345i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10271630u64) }
        0x10271630 => { vm.decode(0x43153862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10271634u64) }
        0x10271634 => { vm.decode(0x2800412fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271638u64) }
        0x10271638 => { vm.decode(0x8c63afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027163cu64) }
        0x1027163c => { vm.decode(0xb0915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271640u64) }
        0x10271640 => { vm.decode(0x607f1119u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1272i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271644u64) }
        0x10271644 => { vm.decode(0x9009d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271648u64) }
        0x10271648 => { vm.decode(0xb0115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027164cu64) }
        0x1027164c => { vm.decode(0x1b191430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271650u64) }
        0x10271650 => { vm.decode(0x28643afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271654u64) }
        0x10271654 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271658u64) }
        0x10271658 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027165cu64 + 4);
        vm.wr(37, 0x1027165cu64 + 4);
        Ok(0x1027165cu64) }
        0x1027165c => { vm.decode(0x664d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271660u64) }
        0x10271660 => { vm.decode(0x603f8119u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1256i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271664u64) }
        0x10271664 => { vm.decode(0xc416fu32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271668u64) }
        0x10271668 => { vm.decode(0x4a8eeb6fu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027166cu64) }
        0x1027166c => { vm.decode(0x8b115570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271670u64) }
        0x10271670 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271674u64) }
        0x10271674 => { vm.decode(0xb011530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271678u64) }
        0x10271678 => { vm.decode(0x8643efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027167cu64) }
        0x1027167c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271680u64) }
        0x10271680 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10271684u64 + 4);
        vm.wr(37, 0x10271684u64 + 4);
        Ok(0x10271684u64) }
        0x10271684 => { vm.decode(0x66098419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2560i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271688u64) }
        0x10271688 => { vm.decode(0x621f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027168cu64) }
        0x1027168c => { vm.decode(0x292094b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271690u64) }
        0x10271690 => { vm.decode(0x29389430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271694u64) }
        0x10271694 => { vm.decode(0x2200e12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271698u64) }
        0x10271698 => { vm.decode(0x88c736fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027169cu64) }
        0x1027169c => { vm.decode(0x9399db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102716a0u64) }
        0x102716a0 => { vm.decode(0x9931ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102716a4u64) }
        0x102716a4 => { vm.decode(0x19299cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102716a8u64) }
        0x102716a8 => { vm.decode(0x428243efu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102716acu64) }
        0x102716ac => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102716b0u64) }
        0x102716b0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102716b4u64 + 4);
        vm.wr(37, 0x102716b4u64 + 4);
        Ok(0x102716b4u64) }
        0x102716b4 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102716b8u64) }
        0x102716b8 => { vm.decode(0x2884736fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102716bcu64) }
        0x102716bc => { vm.decode(0x39389430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102716c0u64) }
        0x102716c0 => { vm.decode(0xa8c536fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102716c4u64) }
        0x102716c4 => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102716c8u64).wrapping_add((imm * 4) as u64) } else { 0x102716c8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102716c8u64) }
        0x102716c8 => { vm.decode(0x292095b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102716ccu64) }
        0x102716cc => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102716d0u64) }
        0x102716d0 => { vm.decode(0x8003036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102716d4u64) }
        0x102716d4 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102716d8u64) }
        0x102716d8 => { vm.decode(0x208043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102716dcu64) }
        0x102716dc => { vm.decode(0xa16004f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102716e0u64) }
        0x102716e0 => { vm.decode(0x19109cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102716e4u64) }
        0x102716e4 => { vm.decode(0x2004c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102716e8u64) }
        0x102716e8 => { vm.decode(0x9921ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102716ecu64) }
        0x102716ec => { vm.decode(0x19399c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102716f0u64) }
        0x102716f0 => { vm.decode(0x624f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102716f4u64) }
        0x102716f4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102716f8u64) }
        0x102716f8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102716fcu64 + 4);
        vm.wr(37, 0x102716fcu64 + 4);
        Ok(0x102716fcu64) }
        0x102716fc => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271700u64) }
        0x10271700 => { vm.decode(0x8102236u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10271704u64) }
        0x10271704 => { vm.decode(0x2a80736fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271708u64) }
        0x10271708 => { vm.decode(0x8c8827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027170cu64).wrapping_add((imm * 4) as u64) } else { 0x1027170cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027170cu64) }
        0x1027170c => { vm.decode(0x9b205430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271710u64) }
        0x10271710 => { vm.decode(0x2a80736fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271714u64) }
        0x10271714 => { vm.decode(0xc008091u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10271718u64) }
        0x10271718 => { vm.decode(0x8c0827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 22i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027171cu64).wrapping_add((imm * 4) as u64) } else { 0x1027171cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027171cu64) }
        0x1027171c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10271720u64) } }
        0x10271720 => { vm.decode(0x2a80736fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271724u64) }
        0x10271724 => { vm.decode(0x2200012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271728u64) }
        0x10271728 => { vm.decode(0x6082636fu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027172cu64) }
        0x1027172c => { vm.decode(0xc0827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 18i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10271730u64).wrapping_add((imm * 4) as u64) } else { 0x10271730u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10271730u64) }
        0x10271730 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10271734u64) } }
        0x10271734 => { vm.decode(0x2000e16fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271738u64) }
        0x10271738 => { vm.decode(0x8c63afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027173cu64) }
        0x1027173c => { vm.decode(0x9299d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271740u64) }
        0x10271740 => { vm.decode(0xa88736fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271744u64) }
        0x10271744 => { vm.decode(0x9219d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271748u64) }
        0x10271748 => { vm.decode(0x19199cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027174cu64) }
        0x1027174c => { vm.decode(0xa86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271750u64) }
        0x10271750 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271754u64) }
        0x10271754 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10271758u64 + 4);
        vm.wr(37, 0x10271758u64 + 4);
        Ok(0x10271758u64) }
        0x10271758 => { vm.decode(0x625f8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2488i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027175cu64) }
        0x1027175c => { vm.decode(0xe7c4323eu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10271760u64) }
        0x10271760 => { vm.decode(0x8ca0101du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 1992i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10271764u64) }
        0x10271764 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10271768u64) } }
        0x10271768 => { vm.decode(0x6082636fu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027176cu64) }
        0x10271778 => { vm.decode(0x1381f6ceu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -18087936i64;
        vm.st(d, imm as u64);
        Ok(0x1027177cu64) }
        0x1027177c => { vm.decode(0x84502318u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = 51208i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10271780u64) }
        0x10271780 => { vm.decode(0x208043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271784u64) }
        0x10271784 => { vm.decode(0xa88e36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271788u64) }
        0x10271788 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027178cu64) }
        0x1027178c => { vm.decode(0x19389c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271790u64) }
        0x10271790 => { vm.decode(0x2200e12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271794u64) }
        0x10271794 => { vm.decode(0xa88736fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271798u64) }
        0x10271798 => { vm.decode(0x9399db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027179cu64) }
        0x1027179c => { vm.decode(0x9931ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102717a0u64) }
        0x102717a0 => { vm.decode(0x19299cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102717a4u64) }
        0x102717a4 => { vm.decode(0x621f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102717a8u64) }
        0x102717a8 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102717acu64) }
        0x102717ac => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102717b0u64 + 4);
        vm.wr(37, 0x102717b0u64 + 4);
        Ok(0x102717b0u64) }
        0x102717b0 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102717b4u64) }
        0x102717b4 => { vm.decode(0x88c536fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102717b8u64) }
        0x102717b8 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102717bcu64).wrapping_add((imm * 4) as u64) } else { 0x102717bcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102717bcu64) }
        0x102717bc => { vm.decode(0x282208e0u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) ^ imm as u64;
        vm.st(d, v);
        Ok(0x102717c0u64) }
        0x102717c0 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102717c4u64) }
        0x102717c4 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102717c8u64) }
        0x102717c8 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102717ccu64) }
        0x102717cc => { vm.decode(0x88c536fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102717d0u64) }
        0x102717d0 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102717d4u64) }
        0x102717d4 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x102717d8u64).wrapping_add((imm * 4) as u64) } else { 0x102717d8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102717d8u64) }
        0x102717d8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102717dcu64) } }
        0x102717dc => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102717e0u64).wrapping_add((imm * 4) as u64) } else { 0x102717e0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102717e0u64) }
        0x10271834 => { vm.decode(0x208043afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271838u64) }
        0x10271838 => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027183cu64) }
        0x1027183c => { vm.decode(0x1b001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271840u64) }
        0x10271840 => { vm.decode(0x208043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271844u64) }
        0x10271844 => { vm.decode(0x888c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271848u64) }
        0x10271848 => { vm.decode(0x8a0404f5u32); let d = 18i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027184cu64) }
        0x1027184c => { vm.decode(0x28863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271850u64) }
        0x10271850 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10271854u64) }
        0x10271854 => { vm.decode(0x1b181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271858u64) }
        0x10271858 => { vm.decode(0x2004c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027185cu64) }
        0x1027185c => { vm.decode(0x9921ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271860u64) }
        0x10271860 => { vm.decode(0x19399c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271864u64) }
        0x10271864 => { vm.decode(0x624f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271868u64) }
        0x10271868 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027186cu64) }
        0x1027186c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10271870u64 + 4);
        vm.wr(37, 0x10271870u64 + 4);
        Ok(0x10271870u64) }
        0x10271870 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271874u64) }
        0x10271874 => { vm.decode(0xa0c12du32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10271878u64) }
        0x10271878 => { vm.decode(0x48a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 11i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027187cu64).wrapping_add((imm * 4) as u64) } else { 0x1027187cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027187cu64) }
        0x1027187c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10271880u64) } }
        0x10271880 => { vm.decode(0x2204416fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271884u64) }
        0x10271884 => { vm.decode(0x88c736fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271888u64) }
        0x10271888 => { vm.decode(0xb211530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027188cu64) }
        0x1027188c => { vm.decode(0x888c32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271890u64) }
        0x10271890 => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271894u64) }
        0x10271894 => { vm.decode(0x1b1114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271898u64) }
        0x10271898 => { vm.decode(0x8643afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027189cu64) }
        0x1027189c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102718a0u64) }
        0x102718a0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102718a4u64 + 4);
        vm.wr(37, 0x102718a4u64 + 4);
        Ok(0x102718a4u64) }
        0x102718a4 => { vm.decode(0x66598419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102718a8u64) }
        0x102718a8 => { vm.decode(0x2080f32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102718acu64) }
        0x102718ac => { vm.decode(0x39281cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102718b0u64) }
        0x102718b0 => { vm.decode(0x8002036u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102718b4u64) }
        0x102718b4 => { vm.decode(0x43153c62u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1372i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102718b8u64) }
        0x102718b8 => { vm.decode(0x2800612fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102718bcu64) }
        0x102718bc => { vm.decode(0x19199c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102718c0u64) }
        0x102718c0 => { vm.decode(0x28263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102718c4u64) }
        0x102718c4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102718c8u64) }
        0x102718c8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102718ccu64 + 4);
        vm.wr(37, 0x102718ccu64 + 4);
        Ok(0x102718ccu64) }
        0x102718cc => { vm.decode(0x624f8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2456i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102718d0u64) }
        0x102718d0 => { vm.decode(0xf1c6011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102718d4u64) }
        0x102718d4 => { vm.decode(0xbc0fcff6u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102718d8u64) }
        0x102718d8 => { vm.decode(0x620b8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2432i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102718dcu64) }
        0x102718dc => { vm.decode(0x63051862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102718e0u64) }
        0x102718e0 => { vm.decode(0x20a0c22fu32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102718e4u64) }
        0x102718e4 => { vm.decode(0x623d7062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102718e8u64) }
        0x102718e8 => { vm.decode(0x1b309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102718ecu64) }
        0x102718ec => { vm.decode(0x608843afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102718f0u64) }
        0x102718f0 => { vm.decode(0x9b289db0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102718f4u64) }
        0x102718f4 => { vm.decode(0x20ce12fu32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102718f8u64) }
        0x102718f8 => { vm.decode(0x40048019u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102718fcu64) }
        0x102718fc => { vm.decode(0x19099c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271900u64) }
        0x10271900 => { vm.decode(0x9019d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271904u64) }
        0x10271904 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271908u64) }
        0x10271908 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027190cu64 + 4);
        vm.wr(37, 0x1027190cu64 + 4);
        Ok(0x1027190cu64) }
        0x1027190c => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271910u64) }
        0x10271910 => { vm.decode(0x621f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271914u64) }
        0x10271914 => { vm.decode(0x29309430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271918u64) }
        0x10271918 => { vm.decode(0x29289430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027191cu64) }
        0x1027191c => { vm.decode(0xf186811u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10271920u64) }
        0x10271920 => { vm.decode(0x633d3062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10271924u64) }
        0x10271924 => { vm.decode(0x2200e12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271928u64) }
        0x10271928 => { vm.decode(0x2ac622fu32); let d = 2i16; let s = 30i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027192cu64) }
        0x1027192c => { vm.decode(0x9399db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271930u64) }
        0x10271930 => { vm.decode(0x9931ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271934u64) }
        0x10271934 => { vm.decode(0x19299cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271938u64) }
        0x10271938 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027193cu64) }
        0x1027193c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10271940u64 + 4);
        vm.wr(37, 0x10271940u64 + 4);
        Ok(0x10271940u64) }
        0x10271940 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271944u64) }
        0x10271944 => { vm.decode(0x22a4622fu32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271948u64) }
        0x10271948 => { vm.decode(0x39289430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027194cu64) }
        0x1027194c => { vm.decode(0xa8426fu32); let d = 2i16; let s = 30i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271950u64) }
        0x10271950 => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10271954u64).wrapping_add((imm * 4) as u64) } else { 0x10271954u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10271954u64) }
        0x10271954 => { vm.decode(0x29309530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271958u64) }
        0x10271958 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027195cu64) }
        0x1027195c => { vm.decode(0x8003036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10271960u64) }
        0x10271960 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10271964u64) }
        0x10271964 => { vm.decode(0x2004c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271968u64) }
        0x10271968 => { vm.decode(0x9921ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027196cu64) }
        0x1027196c => { vm.decode(0x19399c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271970u64) }
        0x10271970 => { vm.decode(0x624f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271974u64) }
        0x10271974 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271978u64) }
        0x10271978 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027197cu64 + 4);
        vm.wr(37, 0x1027197cu64 + 4);
        Ok(0x1027197cu64) }
        0x1027197c => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271980u64) }
        0x10271980 => { vm.decode(0x9b289db0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271984u64) }
        0x10271984 => { vm.decode(0xd9cea11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10271988u64) }
        0x10271988 => { vm.decode(0x82002031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1027198cu64).wrapping_add((imm * 4) as u64) } else { 0x1027198cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027198cu64) }
        0x1027198c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10271990u64) } }
        0x10271990 => { vm.decode(0x2000c16fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271994u64) }
        0x10271994 => { vm.decode(0xa8cc3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271998u64) }
        0x10271998 => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027199cu64) }
        0x1027199c => { vm.decode(0x9099d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102719a0u64) }
        0x102719a0 => { vm.decode(0x19019c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102719a4u64) }
        0x102719a4 => { vm.decode(0x620b8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2432i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102719a8u64) }
        0x102719a8 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102719acu64) }
        0x102719ac => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102719b0u64 + 4);
        vm.wr(37, 0x102719b0u64 + 4);
        Ok(0x102719b0u64) }
        0x102719b0 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102719b4u64) }
        0x102719b4 => { vm.decode(0x20a0c22fu32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102719b8u64) }
        0x102719b8 => { vm.decode(0xbc0fcff6u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102719bcu64) }
        0x102719bc => { vm.decode(0x623d7062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102719c0u64) }
        0x102719c0 => { vm.decode(0x1b309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102719c4u64) }
        0x102719c4 => { vm.decode(0x9b289db0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102719c8u64) }
        0x102719c8 => { vm.decode(0x20ce12fu32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102719ccu64) }
        0x102719cc => { vm.decode(0x40048019u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102719d0u64) }
        0x102719d0 => { vm.decode(0x19099c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102719d4u64) }
        0x102719d4 => { vm.decode(0x9019d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102719d8u64) }
        0x102719d8 => { vm.decode(0x620b8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2432i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102719dcu64) }
        0x102719dc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102719e0u64) }
        0x102719e0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102719e4u64 + 4);
        vm.wr(37, 0x102719e4u64 + 4);
        Ok(0x102719e4u64) }
        0x102719e4 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102719e8u64) }
        0x102719e8 => { vm.decode(0x621f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102719ecu64) }
        0x102719ec => { vm.decode(0x29209430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102719f0u64) }
        0x102719f0 => { vm.decode(0x291894b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102719f4u64) }
        0x102719f4 => { vm.decode(0xf186811u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102719f8u64) }
        0x102719f8 => { vm.decode(0x633d3062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102719fcu64) }
        0x102719fc => { vm.decode(0x2200e12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271a00u64) }
        0x10271a00 => { vm.decode(0x2ac426fu32); let d = 2i16; let s = 30i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271a04u64) }
        0x10271a04 => { vm.decode(0x9399db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271a08u64) }
        0x10271a08 => { vm.decode(0x9931ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271a0cu64) }
        0x10271a0c => { vm.decode(0x19299cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271a10u64) }
        0x10271a10 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271a14u64) }
        0x10271a14 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10271a18u64 + 4);
        vm.wr(37, 0x10271a18u64 + 4);
        Ok(0x10271a18u64) }
        0x10271a18 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271a1cu64) }
        0x10271a1c => { vm.decode(0x22a4426fu32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271a20u64) }
        0x10271a20 => { vm.decode(0x391894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271a24u64) }
        0x10271a24 => { vm.decode(0x2ac626fu32); let d = 2i16; let s = 30i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271a28u64) }
        0x10271a28 => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10271a2cu64).wrapping_add((imm * 4) as u64) } else { 0x10271a2cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10271a2cu64) }
        0x10271a2c => { vm.decode(0x29209530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271a30u64) }
        0x10271a30 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10271a34u64) }
        0x10271a34 => { vm.decode(0x8003036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10271a38u64) }
        0x10271a38 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10271a3cu64) }
        0x10271a3c => { vm.decode(0x2004c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271a40u64) }
        0x10271a40 => { vm.decode(0x9921ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271a44u64) }
        0x10271a44 => { vm.decode(0x19399c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271a48u64) }
        0x10271a48 => { vm.decode(0x624f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271a4cu64) }
        0x10271a4c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271a50u64) }
        0x10271a50 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10271a54u64 + 4);
        vm.wr(37, 0x10271a54u64 + 4);
        Ok(0x10271a54u64) }
        0x10271a54 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271a58u64) }
        0x10271a58 => { vm.decode(0x9b289db0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271a5cu64) }
        0x10271a5c => { vm.decode(0xd9cea11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10271a60u64) }
        0x10271a60 => { vm.decode(0x82002031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10271a64u64).wrapping_add((imm * 4) as u64) } else { 0x10271a64u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10271a64u64) }
        0x10271a64 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10271a68u64) } }
        0x10271a68 => { vm.decode(0x2000c16fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271a6cu64) }
        0x10271a6c => { vm.decode(0xa8cc3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271a70u64) }
        0x10271a70 => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271a74u64) }
        0x10271a74 => { vm.decode(0x9099d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271a78u64) }
        0x10271a78 => { vm.decode(0x19019c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271a7cu64) }
        0x10271a7c => { vm.decode(0x620b8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2432i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271a80u64) }
        0x10271a80 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271a84u64) }
        0x10271a84 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10271a88u64 + 4);
        vm.wr(37, 0x10271a88u64 + 4);
        Ok(0x10271a88u64) }
        0x10271a88 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271a8cu64) }
        0x10271a8c => { vm.decode(0x89209d30u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271a90u64) }
        0x10271a90 => { vm.decode(0x2a00412fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271a94u64) }
        0x10271a94 => { vm.decode(0x8b115570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271a98u64) }
        0x10271a98 => { vm.decode(0x603f1019u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1192i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271a9cu64) }
        0x10271a9c => { vm.decode(0xb091530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271aa0u64) }
        0x10271aa0 => { vm.decode(0x1b011430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271aa4u64) }
        0x10271aa4 => { vm.decode(0x8643efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271aa8u64) }
        0x10271aa8 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271aacu64) }
        0x10271aac => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10271ab0u64 + 4);
        vm.wr(37, 0x10271ab0u64 + 4);
        Ok(0x10271ab0u64) }
        0x10271ab0 => { vm.decode(0x66098419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2560i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271ab4u64) }
        0x10271ab4 => { vm.decode(0x706b9019u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 1168i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271ab8u64) }
        0x10271ab8 => { vm.decode(0x2800412fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271abcu64) }
        0x10271abc => { vm.decode(0x8c63afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271ac0u64) }
        0x10271ac0 => { vm.decode(0xb0915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271ac4u64) }
        0x10271ac4 => { vm.decode(0x9b0115b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271ac8u64) }
        0x10271ac8 => { vm.decode(0x1b191430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271accu64) }
        0x10271acc => { vm.decode(0x28643afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271ad0u64) }
        0x10271ad0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271ad4u64) }
        0x10271ad4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10271ad8u64 + 4);
        vm.wr(37, 0x10271ad8u64 + 4);
        Ok(0x10271ad8u64) }
        0x10271ad8 => { vm.decode(0x664d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271adcu64) }
        0x10271adc => { vm.decode(0x621f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271ae0u64) }
        0x10271ae0 => { vm.decode(0x29089430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271ae4u64) }
        0x10271ae4 => { vm.decode(0x29009430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271ae8u64) }
        0x10271ae8 => { vm.decode(0x2200e12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271aecu64) }
        0x10271aec => { vm.decode(0x888736fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271af0u64) }
        0x10271af0 => { vm.decode(0x9399db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271af4u64) }
        0x10271af4 => { vm.decode(0x9931ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271af8u64) }
        0x10271af8 => { vm.decode(0x19299cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271afcu64) }
        0x10271afc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271b00u64) }
        0x10271b00 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10271b04u64 + 4);
        vm.wr(37, 0x10271b04u64 + 4);
        Ok(0x10271b04u64) }
        0x10271b04 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271b08u64) }
        0x10271b08 => { vm.decode(0x2880736fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271b0cu64) }
        0x10271b0c => { vm.decode(0x39009430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271b10u64) }
        0x10271b10 => { vm.decode(0xa88536fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271b14u64) }
        0x10271b14 => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10271b18u64).wrapping_add((imm * 4) as u64) } else { 0x10271b18u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10271b18u64) }
        0x10271b18 => { vm.decode(0x29089530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271b1cu64) }
        0x10271b1c => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10271b20u64) }
        0x10271b20 => { vm.decode(0x8003036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10271b24u64) }
        0x10271b24 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10271b28u64) }
        0x10271b28 => { vm.decode(0x2004c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271b2cu64) }
        0x10271b2c => { vm.decode(0x9921ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271b30u64) }
        0x10271b30 => { vm.decode(0x19399c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271b34u64) }
        0x10271b34 => { vm.decode(0x624f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271b38u64) }
        0x10271b38 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271b3cu64) }
        0x10271b3c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10271b40u64 + 4);
        vm.wr(37, 0x10271b40u64 + 4);
        Ok(0x10271b40u64) }
        0x10271b40 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271b44u64) }
        0x10271b44 => { vm.decode(0x2880532fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271b48u64) }
        0x10271b48 => { vm.decode(0x8c0027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 20i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10271b4cu64).wrapping_add((imm * 4) as u64) } else { 0x10271b4cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10271b4cu64) }
        0x10271b4c => { vm.decode(0x8100236u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10271b50u64) }
        0x10271b50 => { vm.decode(0x2880532fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271b54u64) }
        0x10271b54 => { vm.decode(0xc008091u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10271b58u64) }
        0x10271b58 => { vm.decode(0xc0027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10271b5cu64).wrapping_add((imm * 4) as u64) } else { 0x10271b5cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10271b5cu64) }
        0x10271b5c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10271b60u64) } }
        0x10271b60 => { vm.decode(0x2880532fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271b64u64) }
        0x10271b64 => { vm.decode(0x2200012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271b68u64) }
        0x10271b68 => { vm.decode(0x840227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 12i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10271b6cu64).wrapping_add((imm * 4) as u64) } else { 0x10271b6cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10271b6cu64) }
        0x10271b6c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10271b70u64) } }
        0x10271b70 => { vm.decode(0x2000e16fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271b74u64) }
        0x10271b74 => { vm.decode(0x8c63afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271b78u64) }
        0x10271b78 => { vm.decode(0x9299d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271b7cu64) }
        0x10271b7c => { vm.decode(0x888532fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271b80u64) }
        0x10271b80 => { vm.decode(0x9219d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271b84u64) }
        0x10271b84 => { vm.decode(0x19199cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271b88u64) }
        0x10271b88 => { vm.decode(0xa86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271b8cu64) }
        0x10271b8c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271b90u64) }
        0x10271b90 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10271b94u64 + 4);
        vm.wr(37, 0x10271b94u64 + 4);
        Ok(0x10271b94u64) }
        0x10271b94 => { vm.decode(0x625f8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2488i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271b98u64) }
        0x10271b98 => { vm.decode(0xc7c4323eu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10271b9cu64) }
        0x10271b9c => { vm.decode(0x2200e12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271ba0u64) }
        0x10271ba0 => { vm.decode(0x888532fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271ba4u64) }
        0x10271ba4 => { vm.decode(0x9399db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271ba8u64) }
        0x10271ba8 => { vm.decode(0x9931ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271bacu64) }
        0x10271bac => { vm.decode(0x19299cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271bb0u64) }
        0x10271bb0 => { vm.decode(0x621f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271bb4u64) }
        0x10271bb4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271bb8u64) }
        0x10271bb8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10271bbcu64 + 4);
        vm.wr(37, 0x10271bbcu64 + 4);
        Ok(0x10271bbcu64) }
        0x10271bbc => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271bc0u64) }
        0x10271bc0 => { vm.decode(0x888732fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271bc4u64) }
        0x10271bc4 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10271bc8u64).wrapping_add((imm * 4) as u64) } else { 0x10271bc8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10271bc8u64) }
        0x10271bc8 => { vm.decode(0x8220860u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) ^ imm as u64;
        vm.st(d, v);
        Ok(0x10271bccu64) }
        0x10271bcc => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10271bd0u64) }
        0x10271bd0 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10271bd4u64) }
        0x10271bd4 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10271bd8u64) }
        0x10271bd8 => { vm.decode(0x888732fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271bdcu64) }
        0x10271bdc => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10271be0u64) }
        0x10271be0 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x10271be4u64).wrapping_add((imm * 4) as u64) } else { 0x10271be4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10271be4u64) }
        0x10271be4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10271be8u64) } }
        0x10271be8 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10271becu64).wrapping_add((imm * 4) as u64) } else { 0x10271becu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10271becu64) }
        0x10271c40 => { vm.decode(0x2004c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271c44u64) }
        0x10271c44 => { vm.decode(0x9921ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271c48u64) }
        0x10271c48 => { vm.decode(0x19399c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271c4cu64) }
        0x10271c4c => { vm.decode(0x624f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271c50u64) }
        0x10271c50 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271c54u64) }
        0x10271c54 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10271c58u64 + 4);
        vm.wr(37, 0x10271c58u64 + 4);
        Ok(0x10271c58u64) }
        0x10271c58 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271c5cu64) }
        0x10271c5c => { vm.decode(0xa0412du32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10271c60u64) }
        0x10271c60 => { vm.decode(0x848227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 13i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10271c64u64).wrapping_add((imm * 4) as u64) } else { 0x10271c64u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10271c64u64) }
        0x10271c64 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10271c68u64) } }
        0x10271c68 => { vm.decode(0x2204416fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271c6cu64) }
        0x10271c6c => { vm.decode(0x888736fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271c70u64) }
        0x10271c70 => { vm.decode(0xb211530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271c74u64) }
        0x10271c74 => { vm.decode(0x9b1915b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271c78u64) }
        0x10271c78 => { vm.decode(0x1b1114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271c7cu64) }
        0x10271c7c => { vm.decode(0x8643afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271c80u64) }
        0x10271c80 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10271c84u64) }
        0x10271c84 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10271c88u64 + 4);
        vm.wr(37, 0x10271c88u64 + 4);
        Ok(0x10271c88u64) }
        0x10271c88 => { vm.decode(0x66598419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10271c8cu64) }
        0x10271c8c => { vm.decode(0x408843efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10271c90u64) }
        0x10271c90 => { vm.decode(0x824041du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 4184i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10271c94u64) }
        0x10271c94 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10271c98u64) } }
        0x10271c98 => { vm.decode(0x293814b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10271c9cu64) }
        0x10272008 => { vm.decode(0x2884532fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027200cu64) }
        0x1027200c => { vm.decode(0x39301cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272010u64) }
        0x10272010 => { vm.decode(0x20a0622fu32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272014u64) }
        0x10272014 => { vm.decode(0xc008091u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10272018u64) }
        0x10272018 => { vm.decode(0x2000071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1027201cu64).wrapping_add((imm * 4) as u64) } else { 0x1027201cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027201cu64) }
        0x1027201c => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10272020u64) }
        0x10272020 => { vm.decode(0x28a0422fu32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272024u64) }
        0x1027202c => { vm.decode(0x8b2055b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272030u64) }
        0x10272030 => { vm.decode(0x614724f5u32); let d = 21i16; let s = 18i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10272034u64) }
        0x10272034 => { vm.decode(0x2a80c36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272038u64) }
        0x10272038 => { vm.decode(0x820bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1027203cu64) }
        0x1027203c => { vm.decode(0x800202f5u32); let d = 1i16; let s = 0i16; let s2 = 2i16; let imm = 0i64;
        if vm.ld(s2) != 0 {
            let v = vm.ld(s);
            vm.st(d, v);
        }
        Ok(0x10272040u64) }
        0x10272040 => { vm.decode(0x63351862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10272044u64) }
        0x10272044 => { vm.decode(0x621f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272048u64) }
        0x10272048 => { vm.decode(0xb309cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027204cu64) }
        0x1027204c => { vm.decode(0xb289cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272050u64) }
        0x10272050 => { vm.decode(0x2200e12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272054u64) }
        0x10272054 => { vm.decode(0x8ac422fu32); let d = 2i16; let s = 30i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272058u64) }
        0x10272058 => { vm.decode(0x9399db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027205cu64) }
        0x1027205c => { vm.decode(0x9931ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272060u64) }
        0x10272060 => { vm.decode(0x19299cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272064u64) }
        0x10272064 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272068u64) }
        0x10272068 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027206cu64 + 4);
        vm.wr(37, 0x1027206cu64 + 4);
        Ok(0x1027206cu64) }
        0x1027206c => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272070u64) }
        0x10272070 => { vm.decode(0x28a4422fu32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272074u64) }
        0x10272074 => { vm.decode(0x1b289cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272078u64) }
        0x10272078 => { vm.decode(0x8ac622fu32); let d = 2i16; let s = 30i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027207cu64) }
        0x1027207c => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10272080u64).wrapping_add((imm * 4) as u64) } else { 0x10272080u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10272080u64) }
        0x10272080 => { vm.decode(0xb309db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272084u64) }
        0x10272084 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10272088u64) }
        0x10272088 => { vm.decode(0x8003036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027208cu64) }
        0x1027208c => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10272090u64) }
        0x10272090 => { vm.decode(0x2a84c36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272094u64) }
        0x10272094 => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10272098u64) }
        0x10272098 => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027209cu64) }
        0x1027209c => { vm.decode(0x2a80436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102720a0u64) }
        0x102720a0 => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102720a4u64) }
        0x102720a4 => { vm.decode(0x19309430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102720a8u64) }
        0x102720a8 => { vm.decode(0x228043afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102720acu64) }
        0x102720ac => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102720b0u64) }
        0x102720b0 => { vm.decode(0x1b101430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102720b4u64) }
        0x102720b4 => { vm.decode(0x2a80636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102720b8u64) }
        0x102720b8 => { vm.decode(0x610604f5u32); let d = 20i16; let s = 18i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102720bcu64) }
        0x102720bc => { vm.decode(0x109875u32); let d = 1i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102720c0u64) }
        0x102720c0 => { vm.decode(0x288c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102720c4u64) }
        0x102720c4 => { vm.decode(0x4b0404f5u32); let d = 18i16; let s = 18i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102720c8u64) }
        0x102720c8 => { vm.decode(0x88ce36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102720ccu64) }
        0x102720cc => { vm.decode(0x8a0604f5u32); let d = 22i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102720d0u64) }
        0x102720d0 => { vm.decode(0xa88432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102720d4u64) }
        0x102720d4 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102720d8u64) }
        0x102720d8 => { vm.decode(0x9389530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102720dcu64) }
        0x102720dc => { vm.decode(0xa88632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102720e0u64) }
        0x102720e0 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102720e4u64) }
        0x102720e4 => { vm.decode(0x9189530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102720e8u64) }
        0x102720e8 => { vm.decode(0x88c432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102720ecu64) }
        0x102720ec => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102720f0u64) }
        0x102720f0 => { vm.decode(0x90095b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102720f4u64) }
        0x102720f4 => { vm.decode(0x88c632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102720f8u64) }
        0x102720f8 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102720fcu64) }
        0x102720fc => { vm.decode(0x90895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272100u64) }
        0x10272100 => { vm.decode(0xa8c432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272104u64) }
        0x10272104 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10272108u64) }
        0x10272108 => { vm.decode(0x9289db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027210cu64) }
        0x1027210c => { vm.decode(0xa8c632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272110u64) }
        0x10272110 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10272114u64) }
        0x10272114 => { vm.decode(0x91895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272118u64) }
        0x10272118 => { vm.decode(0x888436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027211cu64) }
        0x1027211c => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10272120u64) }
        0x10272120 => { vm.decode(0x9209530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272124u64) }
        0x10272124 => { vm.decode(0x888636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272128u64) }
        0x10272128 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027212cu64) }
        0x1027212c => { vm.decode(0x19289430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272130u64) }
        0x10272130 => { vm.decode(0x2004c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272134u64) }
        0x10272134 => { vm.decode(0x9921ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272138u64) }
        0x10272138 => { vm.decode(0x19399c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027213cu64) }
        0x1027213c => { vm.decode(0x8643efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272140u64) }
        0x10272140 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272144u64) }
        0x10272144 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272148u64 + 4);
        vm.wr(37, 0x10272148u64 + 4);
        Ok(0x10272148u64) }
        0x10272148 => { vm.decode(0x624f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027214cu64) }
        0x1027214c => { vm.decode(0x2a00612fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272150u64) }
        0x10272150 => { vm.decode(0x621d8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2408i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272154u64) }
        0x10272154 => { vm.decode(0x60691119u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1104i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272158u64) }
        0x10272158 => { vm.decode(0x288ce3efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027215cu64) }
        0x1027215c => { vm.decode(0x19391db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272160u64) }
        0x10272160 => { vm.decode(0x9311db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272164u64) }
        0x10272164 => { vm.decode(0x19291cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272168u64) }
        0x10272168 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027216cu64) }
        0x1027216c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272170u64 + 4);
        vm.wr(37, 0x10272170u64 + 4);
        Ok(0x10272170u64) }
        0x10272170 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272174u64) }
        0x10272174 => { vm.decode(0x2200e12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272178u64) }
        0x10272178 => { vm.decode(0x88ce3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027217cu64) }
        0x1027217c => { vm.decode(0x9399db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272180u64) }
        0x10272180 => { vm.decode(0x9931ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272184u64) }
        0x10272184 => { vm.decode(0x19299cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272188u64) }
        0x10272188 => { vm.decode(0x28243efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027218cu64) }
        0x1027218c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272190u64) }
        0x10272190 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272194u64 + 4);
        vm.wr(37, 0x10272194u64 + 4);
        Ok(0x10272194u64) }
        0x10272194 => { vm.decode(0x621f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272198u64) }
        0x10272198 => { vm.decode(0xa8cc3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027219cu64) }
        0x1027219c => { vm.decode(0x408e6befu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102721a0u64) }
        0x102721a0 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102721a4u64).wrapping_add((imm * 4) as u64) } else { 0x102721a4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102721a4u64) }
        0x102721a4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102721a8u64) } }
        0x102721a8 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102721acu64) }
        0x102721ac => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102721b0u64) }
        0x102721b0 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102721b4u64) }
        0x102721b4 => { vm.decode(0xa8cc3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102721b8u64) }
        0x102721b8 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102721bcu64) }
        0x102721bc => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x102721c0u64).wrapping_add((imm * 4) as u64) } else { 0x102721c0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102721c0u64) }
        0x102721c0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102721c4u64) } }
        0x102721c4 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102721c8u64).wrapping_add((imm * 4) as u64) } else { 0x102721c8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102721c8u64) }
        0x1027221c => { vm.decode(0x8910d5b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272220u64) }
        0x10272220 => { vm.decode(0x99301d30u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272224u64) }
        0x10272224 => { vm.decode(0x2004c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272228u64) }
        0x10272228 => { vm.decode(0x9921ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027222cu64) }
        0x1027222c => { vm.decode(0x19399c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272230u64) }
        0x10272230 => { vm.decode(0x8643efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272234u64) }
        0x10272234 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272238u64) }
        0x10272238 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027223cu64 + 4);
        vm.wr(37, 0x1027223cu64 + 4);
        Ok(0x1027223cu64) }
        0x1027223c => { vm.decode(0x624f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272240u64) }
        0x10272240 => { vm.decode(0x620d8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272244u64) }
        0x10272244 => { vm.decode(0x60298119u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272248u64) }
        0x10272248 => { vm.decode(0x288536fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027224cu64) }
        0x1027224c => { vm.decode(0x228002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272250u64) }
        0x10272250 => { vm.decode(0x29381db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272254u64) }
        0x10272254 => { vm.decode(0x80c412fu32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272258u64) }
        0x10272258 => { vm.decode(0x19311c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027225cu64) }
        0x1027225c => { vm.decode(0x9291d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272260u64) }
        0x10272260 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272264u64) }
        0x10272264 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272268u64 + 4);
        vm.wr(37, 0x10272268u64 + 4);
        Ok(0x10272268u64) }
        0x10272268 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027226cu64) }
        0x1027226c => { vm.decode(0x2080536fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272270u64) }
        0x10272270 => { vm.decode(0x2200012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272274u64) }
        0x10272274 => { vm.decode(0x39009c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272278u64) }
        0x10272278 => { vm.decode(0x2804612fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027227cu64) }
        0x1027227c => { vm.decode(0x19199c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272280u64) }
        0x10272280 => { vm.decode(0x28263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272284u64) }
        0x10272284 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272288u64) }
        0x10272288 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027228cu64 + 4);
        vm.wr(37, 0x1027228cu64 + 4);
        Ok(0x1027228cu64) }
        0x1027228c => { vm.decode(0x624f8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2456i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272290u64) }
        0x10272290 => { vm.decode(0x625d8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2360i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272294u64) }
        0x10272294 => { vm.decode(0xf1c6011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10272298u64) }
        0x10272298 => { vm.decode(0x3253862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027229cu64) }
        0x1027229c => { vm.decode(0x2a04412fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102722a0u64) }
        0x102722a0 => { vm.decode(0x19191cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102722a4u64) }
        0x102722a4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102722a8u64) }
        0x102722a8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102722acu64 + 4);
        vm.wr(37, 0x102722acu64 + 4);
        Ok(0x102722acu64) }
        0x102722ac => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102722b0u64) }
        0x102722b0 => { vm.decode(0x741b1519u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2784i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102722b4u64) }
        0x102722b4 => { vm.decode(0x60798019u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102722b8u64) }
        0x102722b8 => { vm.decode(0x401019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102722bcu64) }
        0x102722bc => { vm.decode(0x20c8c36fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102722c0u64) }
        0x102722c0 => { vm.decode(0x39089d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102722c4u64) }
        0x102722c4 => { vm.decode(0x882e32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102722c8u64) }
        0x102722c8 => { vm.decode(0x48108091u32); let d = 3i16; let s = 4i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102722ccu64) }
        0x102722cc => { vm.decode(0x282002fu32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102722d0u64) }
        0x102722d0 => { vm.decode(0x2900dcb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102722d4u64) }
        0x102722d4 => { vm.decode(0x39189d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102722d8u64) }
        0x102722d8 => { vm.decode(0x2208c16fu32); let d = 3i16; let s = 17i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102722dcu64) }
        0x102722dc => { vm.decode(0xb3995b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102722e0u64) }
        0x102722e0 => { vm.decode(0x888636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102722e4u64) }
        0x102722e4 => { vm.decode(0xb3195b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102722e8u64) }
        0x102722e8 => { vm.decode(0x1b2994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102722ecu64) }
        0x102722ec => { vm.decode(0x1b2195b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102722f0u64) }
        0x102722f0 => { vm.decode(0x608863afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102722f4u64) }
        0x102722f4 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102722f8u64) }
        0x102722f8 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102722fcu64) }
        0x102722fc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272300u64 + 4);
        vm.wr(37, 0x10272300u64 + 4);
        Ok(0x10272300u64) }
        0x10272300 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272304u64) }
        0x10272304 => { vm.decode(0x8000a019u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 6i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272308u64) }
        0x10272308 => { vm.decode(0x623d0019u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027230cu64) }
        0x1027230c => { vm.decode(0x928d430u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272310u64) }
        0x10272310 => { vm.decode(0x3099u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272314u64) }
        0x10272314 => { vm.decode(0x2a88c36fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272318u64) }
        0x10272318 => { vm.decode(0x39109db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027231cu64) }
        0x1027231c => { vm.decode(0x39189cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272320u64) }
        0x10272320 => { vm.decode(0x39289c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272324u64) }
        0x10272324 => { vm.decode(0x2084d32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272328u64) }
        0x10272328 => { vm.decode(0x39209c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027232cu64) }
        0x1027232c => { vm.decode(0x2284f32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272330u64) }
        0x10272330 => { vm.decode(0x39309c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272334u64) }
        0x10272334 => { vm.decode(0x22351862u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10272338u64) }
        0x10272338 => { vm.decode(0x2200c16fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027233cu64) }
        0x1027233c => { vm.decode(0xb3995b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272340u64) }
        0x10272340 => { vm.decode(0x888436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272344u64) }
        0x10272344 => { vm.decode(0xb3195b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272348u64) }
        0x10272348 => { vm.decode(0xb29d4b0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 2792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027234cu64) }
        0x1027234c => { vm.decode(0x1b2194b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272350u64) }
        0x10272350 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272354u64) }
        0x10272354 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272358u64) }
        0x10272358 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027235cu64 + 4);
        vm.wr(37, 0x1027235cu64 + 4);
        Ok(0x1027235cu64) }
        0x1027235c => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272360u64) }
        0x10272360 => { vm.decode(0x6069a019u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1044i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272364u64) }
        0x10272364 => { vm.decode(0x403019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272368u64) }
        0x10272368 => { vm.decode(0x2208c16fu32); let d = 3i16; let s = 17i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027236cu64) }
        0x1027236c => { vm.decode(0xb3995b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272370u64) }
        0x10272370 => { vm.decode(0xa8c632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272374u64) }
        0x10272374 => { vm.decode(0xb3195b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272378u64) }
        0x10272378 => { vm.decode(0x1b2994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027237cu64) }
        0x1027237c => { vm.decode(0x1b2195b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272380u64) }
        0x10272380 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272384u64) }
        0x10272384 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272388u64) }
        0x10272388 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027238cu64 + 4);
        vm.wr(37, 0x1027238cu64 + 4);
        Ok(0x1027238cu64) }
        0x1027238c => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272390u64) }
        0x10272390 => { vm.decode(0x148019u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 40i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272394u64) }
        0x10272394 => { vm.decode(0x208c16fu32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272398u64) }
        0x10272398 => { vm.decode(0x1b3994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027239cu64) }
        0x1027239c => { vm.decode(0x2884e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102723a0u64) }
        0x102723a0 => { vm.decode(0x1b3194b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102723a4u64) }
        0x102723a4 => { vm.decode(0x8b29d5f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102723a8u64) }
        0x102723a8 => { vm.decode(0xb2195b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102723acu64) }
        0x102723ac => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102723b0u64) }
        0x102723b0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102723b4u64) }
        0x102723b4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102723b8u64 + 4);
        vm.wr(37, 0x102723b8u64 + 4);
        Ok(0x102723b8u64) }
        0x102723b8 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102723bcu64) }
        0x102723bc => { vm.decode(0x701d9459u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2344i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102723c0u64) }
        0x102723c0 => { vm.decode(0x2000412fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102723c4u64) }
        0x102723c4 => { vm.decode(0x19091cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102723c8u64) }
        0x102723c8 => { vm.decode(0x628243afu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102723ccu64) }
        0x102723cc => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102723d0u64) }
        0x102723d0 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102723d4u64) }
        0x102723d4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102723d8u64) }
        0x102723d8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102723dcu64 + 4);
        vm.wr(37, 0x102723dcu64 + 4);
        Ok(0x102723dcu64) }
        0x102723dc => { vm.decode(0x99289db0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102723e0u64) }
        0x102723e0 => { vm.decode(0x664d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102723e4u64) }
        0x102723e4 => { vm.decode(0x22c4c32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102723e8u64) }
        0x102723e8 => { vm.decode(0x4002012fu32); let d = 20i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102723ecu64) }
        0x102723ec => { vm.decode(0x2800412fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102723f0u64) }
        0x102723f0 => { vm.decode(0x88c632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102723f4u64) }
        0x102723f4 => { vm.decode(0xb0915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102723f8u64) }
        0x102723f8 => { vm.decode(0x9b0155b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102723fcu64) }
        0x102723fc => { vm.decode(0x1b191430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272400u64) }
        0x10272400 => { vm.decode(0x428c43afu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272404u64) }
        0x10272404 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272408u64) }
        0x10272408 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027240cu64) }
        0x1027240c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272410u64 + 4);
        vm.wr(37, 0x10272410u64 + 4);
        Ok(0x10272410u64) }
        0x10272410 => { vm.decode(0x4e804875u32); let d = 30i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272414u64) }
        0x10272414 => { vm.decode(0x620d8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2312i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272418u64) }
        0x10272418 => { vm.decode(0x2000612fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027241cu64) }
        0x1027241c => { vm.decode(0x99195d30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272420u64) }
        0x10272420 => { vm.decode(0x89115c30u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272424u64) }
        0x10272424 => { vm.decode(0x19091c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272428u64) }
        0x10272428 => { vm.decode(0x4a8a436fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027242cu64) }
        0x1027242c => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272430u64) }
        0x10272430 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272434u64) }
        0x10272434 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272438u64 + 4);
        vm.wr(37, 0x10272438u64 + 4);
        Ok(0x10272438u64) }
        0x10272438 => { vm.decode(0xc804875u32); let d = 20i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027243cu64) }
        0x1027243c => { vm.decode(0x664b8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272440u64) }
        0x10272440 => { vm.decode(0x1920d430u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272444u64) }
        0x10272444 => { vm.decode(0xf146011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10272448u64) }
        0x10272448 => { vm.decode(0x3051822u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027244cu64) }
        0x1027244c => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272450u64) }
        0x10272450 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272454u64) }
        0x10272454 => { vm.decode(0x9b39d530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272458u64) }
        0x10272458 => { vm.decode(0x28643efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027245cu64) }
        0x1027245c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272460u64) }
        0x10272460 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272464u64 + 4);
        vm.wr(37, 0x10272464u64 + 4);
        Ok(0x10272464u64) }
        0x10272464 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10272468u64) } }
        0x10272468 => { vm.decode(0x2000412fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027246cu64) }
        0x1027246c => { vm.decode(0x19091cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272470u64) }
        0x10272470 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272474u64) }
        0x10272474 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272478u64) }
        0x10272478 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027247cu64 + 4);
        vm.wr(37, 0x1027247cu64 + 4);
        Ok(0x1027247cu64) }
        0x1027247c => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272480u64) }
        0x10272480 => { vm.decode(0x22c4c32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272484u64) }
        0x10272484 => { vm.decode(0x6008012fu32); let d = 19i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272488u64) }
        0x10272488 => { vm.decode(0x2800412fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027248cu64) }
        0x1027248c => { vm.decode(0x88c432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272490u64) }
        0x10272490 => { vm.decode(0xb0915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272494u64) }
        0x10272494 => { vm.decode(0x9b0155b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272498u64) }
        0x10272498 => { vm.decode(0x1b191430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027249cu64) }
        0x1027249c => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102724a0u64) }
        0x102724a0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102724a4u64) }
        0x102724a4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102724a8u64 + 4);
        vm.wr(37, 0x102724a8u64 + 4);
        Ok(0x102724a8u64) }
        0x102724a8 => { vm.decode(0xb0d875u32); let d = 5i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102724acu64) }
        0x102724ac => { vm.decode(0x2000612fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102724b0u64) }
        0x102724b0 => { vm.decode(0x99195d30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102724b4u64) }
        0x102724b4 => { vm.decode(0x99111d30u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102724b8u64) }
        0x102724b8 => { vm.decode(0x19091c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102724bcu64) }
        0x102724bc => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102724c0u64) }
        0x102724c0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102724c4u64) }
        0x102724c4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102724c8u64 + 4);
        vm.wr(37, 0x102724c8u64 + 4);
        Ok(0x102724c8u64) }
        0x102724c8 => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102724ccu64) }
        0x102724cc => { vm.decode(0xf146011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102724d0u64) }
        0x102724d0 => { vm.decode(0x3151822u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1032i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102724d4u64) }
        0x102724d4 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102724d8u64) }
        0x102724d8 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102724dcu64) }
        0x102724dc => { vm.decode(0x9b39d530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102724e0u64) }
        0x102724e0 => { vm.decode(0x28643efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102724e4u64) }
        0x102724e4 => { vm.decode(0x6888436fu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102724e8u64) }
        0x102724e8 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102724ecu64) }
        0x102724ec => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102724f0u64 + 4);
        vm.wr(37, 0x102724f0u64 + 4);
        Ok(0x102724f0u64) }
        0x102724f0 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102724f4u64) }
        0x102724f4 => { vm.decode(0x2000412fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102724f8u64) }
        0x102724f8 => { vm.decode(0x19091cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102724fcu64) }
        0x102724fc => { vm.decode(0x2886e36fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272500u64) }
        0x10272500 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272504u64) }
        0x10272504 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272508u64 + 4);
        vm.wr(37, 0x10272508u64 + 4);
        Ok(0x10272508u64) }
        0x10272508 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027250cu64) }
        0x1027250c => { vm.decode(0x22c4c32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272510u64) }
        0x10272510 => { vm.decode(0x6002012fu32); let d = 21i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272514u64) }
        0x10272514 => { vm.decode(0x2800412fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272518u64) }
        0x10272518 => { vm.decode(0xa88632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027251cu64) }
        0x1027251c => { vm.decode(0xb0915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272520u64) }
        0x10272520 => { vm.decode(0x9b0155b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272524u64) }
        0x10272524 => { vm.decode(0x1b191430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272528u64) }
        0x10272528 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027252cu64) }
        0x1027252c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272530u64) }
        0x10272530 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272534u64 + 4);
        vm.wr(37, 0x10272534u64 + 4);
        Ok(0x10272534u64) }
        0x10272534 => { vm.decode(0xb0d875u32); let d = 5i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272538u64) }
        0x10272538 => { vm.decode(0x2000612fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027253cu64) }
        0x1027253c => { vm.decode(0x99195d30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272540u64) }
        0x10272540 => { vm.decode(0x99115c30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272544u64) }
        0x10272544 => { vm.decode(0x19091c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272548u64) }
        0x10272548 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027254cu64) }
        0x1027254c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272550u64) }
        0x10272550 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272554u64 + 4);
        vm.wr(37, 0x10272554u64 + 4);
        Ok(0x10272554u64) }
        0x10272554 => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272558u64) }
        0x10272558 => { vm.decode(0xf146011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027255cu64) }
        0x1027255c => { vm.decode(0x3051c22u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1028i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10272560u64) }
        0x10272560 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272564u64) }
        0x10272564 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272568u64) }
        0x10272568 => { vm.decode(0x9b39d530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027256cu64) }
        0x1027256c => { vm.decode(0x628643efu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272570u64) }
        0x10272570 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272574u64) }
        0x10272574 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272578u64) }
        0x10272578 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027257cu64) }
        0x1027257c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272580u64 + 4);
        vm.wr(37, 0x10272580u64 + 4);
        Ok(0x10272580u64) }
        0x10272580 => { vm.decode(0x4c101875u32); let d = 18i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272584u64) }
        0x10272584 => { vm.decode(0x2000412fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272588u64) }
        0x10272588 => { vm.decode(0x19091cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027258cu64) }
        0x1027258c => { vm.decode(0x28243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272590u64) }
        0x10272590 => { vm.decode(0x2886e36fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272594u64) }
        0x10272594 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272598u64) }
        0x10272598 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027259cu64 + 4);
        vm.wr(37, 0x1027259cu64 + 4);
        Ok(0x1027259cu64) }
        0x1027259c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102725a0u64) } }
        0x102725a0 => { vm.decode(0x22c4c32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102725a4u64) }
        0x102725a4 => { vm.decode(0x6008012fu32); let d = 19i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102725a8u64) }
        0x102725a8 => { vm.decode(0x2800412fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102725acu64) }
        0x102725ac => { vm.decode(0xa88636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102725b0u64) }
        0x102725b0 => { vm.decode(0xb0915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102725b4u64) }
        0x102725b4 => { vm.decode(0x9b0155b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102725b8u64) }
        0x102725b8 => { vm.decode(0x1b191430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102725bcu64) }
        0x102725bc => { vm.decode(0x28643afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102725c0u64) }
        0x102725c0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102725c4u64) }
        0x102725c4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102725c8u64 + 4);
        vm.wr(37, 0x102725c8u64 + 4);
        Ok(0x102725c8u64) }
        0x102725c8 => { vm.decode(0xb0d875u32); let d = 5i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102725ccu64) }
        0x102725cc => { vm.decode(0x2000612fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102725d0u64) }
        0x102725d0 => { vm.decode(0x99195d30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102725d4u64) }
        0x102725d4 => { vm.decode(0x99111d30u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102725d8u64) }
        0x102725d8 => { vm.decode(0x19091c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102725dcu64) }
        0x102725dc => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102725e0u64) }
        0x102725e0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102725e4u64) }
        0x102725e4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102725e8u64 + 4);
        vm.wr(37, 0x102725e8u64 + 4);
        Ok(0x102725e8u64) }
        0x102725e8 => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102725ecu64) }
        0x102725ec => { vm.decode(0xf146011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102725f0u64) }
        0x102725f0 => { vm.decode(0x3151c22u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1036i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102725f4u64) }
        0x102725f4 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102725f8u64) }
        0x102725f8 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102725fcu64) }
        0x102725fc => { vm.decode(0x9b385530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272600u64) }
        0x10272600 => { vm.decode(0x9b39d530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272604u64) }
        0x10272604 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272608u64) }
        0x10272608 => { vm.decode(0x48109875u32); let d = 19i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027260cu64) }
        0x1027260c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272610u64) }
        0x10272610 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272614u64 + 4);
        vm.wr(37, 0x10272614u64 + 4);
        Ok(0x10272614u64) }
        0x10272614 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272618u64) }
        0x10272618 => { vm.decode(0x800a036u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027261cu64) }
        0x1027261c => { vm.decode(0x8c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272620u64) }
        0x10272620 => { vm.decode(0x29389d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272624u64) }
        0x10272624 => { vm.decode(0x28cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272628u64) }
        0x10272628 => { vm.decode(0x228002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027262cu64) }
        0x1027262c => { vm.decode(0x29209db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272630u64) }
        0x10272630 => { vm.decode(0x28ce36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272634u64) }
        0x10272634 => { vm.decode(0x40c00011u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10272638u64) }
        0x10272638 => { vm.decode(0x21d7822u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027263cu64) }
        0x1027263c => { vm.decode(0xa0c612fu32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272640u64) }
        0x10272640 => { vm.decode(0x6b353022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10272644u64) }
        0x10272644 => { vm.decode(0x2880636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272648u64) }
        0x10272648 => { vm.decode(0x193194b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027264cu64) }
        0x1027264c => { vm.decode(0x92995b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272650u64) }
        0x10272650 => { vm.decode(0xa86c36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272654u64) }
        0x10272654 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272658u64) }
        0x10272658 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027265cu64 + 4);
        vm.wr(37, 0x1027265cu64 + 4);
        Ok(0x1027265cu64) }
        0x1027265c => { vm.decode(0x621f8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2280i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272660u64) }
        0x10272660 => { vm.decode(0x644b1159u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272664u64) }
        0x10272664 => { vm.decode(0x40c8c32fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272668u64) }
        0x10272668 => { vm.decode(0x2004612fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027266cu64) }
        0x1027266c => { vm.decode(0x9381db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272670u64) }
        0x10272670 => { vm.decode(0xb399530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272674u64) }
        0x10272674 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272678u64) }
        0x10272678 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027267cu64) }
        0x1027267c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272680u64) }
        0x10272680 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272684u64 + 4);
        vm.wr(37, 0x10272684u64 + 4);
        Ok(0x10272684u64) }
        0x10272684 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272688u64) }
        0x10272688 => { vm.decode(0x645f1059u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027268cu64) }
        0x1027268c => { vm.decode(0x2004612fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272690u64) }
        0x10272690 => { vm.decode(0x9301db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272694u64) }
        0x10272694 => { vm.decode(0xb399530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272698u64) }
        0x10272698 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027269cu64) }
        0x1027269c => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102726a0u64) }
        0x102726a0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102726a4u64) }
        0x102726a4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102726a8u64 + 4);
        vm.wr(37, 0x102726a8u64 + 4);
        Ok(0x102726a8u64) }
        0x102726a8 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102726acu64) }
        0x102726ac => { vm.decode(0xa86e32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102726b0u64) }
        0x102726b0 => { vm.decode(0x90a27u32); let d = 0i16; let s = 4i16; let s2 = -1i16; let imm = 26i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102726b4u64).wrapping_add((imm * 4) as u64) } else { 0x102726b4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102726b4u64) }
        0x102726b4 => { vm.decode(0x8000236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102726b8u64) }
        0x102726b8 => { vm.decode(0x8108091u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102726bcu64) }
        0x1027271c => { vm.decode(0x628a63efu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272720u64) }
        0x10272720 => { vm.decode(0x608643afu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272724u64) }
        0x10272724 => { vm.decode(0x408e636fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272728u64) }
        0x10272728 => { vm.decode(0x408e4b6fu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027272cu64) }
        0x1027272c => { vm.decode(0x6288636fu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272730u64) }
        0x10272730 => { vm.decode(0x4282436fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272734u64) }
        0x10272734 => { vm.decode(0xa584101du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 6052i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10272738u64) }
        0x10272738 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027273cu64) } }
        0x1027273c => { vm.decode(0x628a63efu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272740u64) }
        0x10272754 => { vm.decode(0x208043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272758u64) }
        0x10272758 => { vm.decode(0x884124f5u32); let d = 3i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027275cu64) }
        0x1027275c => { vm.decode(0x19389530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272760u64) }
        0x10272760 => { vm.decode(0x288ce32fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272764u64) }
        0x10272764 => { vm.decode(0xaa4004f5u32); let d = 3i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10272768u64) }
        0x10272768 => { vm.decode(0x19089db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027276cu64) }
        0x1027276c => { vm.decode(0xaa4504f5u32); let d = 19i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10272770u64) }
        0x10272770 => { vm.decode(0x288cc36fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272774u64) }
        0x10272774 => { vm.decode(0xaa4004f5u32); let d = 3i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10272778u64) }
        0x10272778 => { vm.decode(0x1b101530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027277cu64) }
        0x1027277c => { vm.decode(0x8a41a4f5u32); let d = 3i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10272780u64) }
        0x10272780 => { vm.decode(0x19209db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272784u64) }
        0x10272784 => { vm.decode(0x288c436fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272788u64) }
        0x10272788 => { vm.decode(0xaa8604f5u32); let d = 30i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027278cu64) }
        0x1027278c => { vm.decode(0x288cc32fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272790u64) }
        0x10272790 => { vm.decode(0xa20604f5u32); let d = 20i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10272794u64) }
        0x10272794 => { vm.decode(0x80412du32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10272798u64) }
        0x10272798 => { vm.decode(0x641b1059u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027279cu64) }
        0x1027279c => { vm.decode(0x9289db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102727a0u64) }
        0x102727a0 => { vm.decode(0x646d1159u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1880i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102727a4u64) }
        0x102727a4 => { vm.decode(0x2222071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102727a8u64).wrapping_add((imm * 4) as u64) } else { 0x102727a8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102727a8u64) }
        0x102727a8 => { vm.decode(0xb2815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102727acu64) }
        0x102727ac => { vm.decode(0x80b0827u32); let d = 0i16; let s = 22i16; let s2 = -1i16; let imm = 18i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102727b0u64).wrapping_add((imm * 4) as u64) } else { 0x102727b0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102727b0u64) }
        0x10272804 => { vm.decode(0xab101530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272808u64) }
        0x10272808 => { vm.decode(0x8843efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027280cu64) }
        0x1027280c => { vm.decode(0x2888632fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272810u64) }
        0x10272810 => { vm.decode(0x882432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272814u64) }
        0x10272814 => { vm.decode(0x2286636fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272818u64) }
        0x10272818 => { vm.decode(0x428c436fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027281cu64) }
        0x1027281c => { vm.decode(0x4a8a636fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272820u64) }
        0x10272820 => { vm.decode(0x8884541du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 7064i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10272824u64) }
        0x10272824 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10272828u64) } }
        0x10272828 => { vm.decode(0x22a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027282cu64) }
        0x10272b48 => { vm.decode(0x604024f5u32); let d = 1i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10272b4cu64) }
        0x10272b4c => { vm.decode(0x1b0814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272b50u64) }
        0x10272b50 => { vm.decode(0x404024f5u32); let d = 1i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10272b54u64) }
        0x10272b54 => { vm.decode(0x19109cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272b58u64) }
        0x10272b58 => { vm.decode(0x624604f5u32); let d = 21i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10272b5cu64) }
        0x10272b5c => { vm.decode(0x2000616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272b60u64) }
        0x10272b60 => { vm.decode(0x89199530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272b64u64) }
        0x10272b64 => { vm.decode(0x19119430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272b68u64) }
        0x10272b68 => { vm.decode(0x624b8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2192i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272b6cu64) }
        0x10272b6c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272b70u64) }
        0x10272b70 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272b74u64 + 4);
        vm.wr(37, 0x10272b74u64 + 4);
        Ok(0x10272b74u64) }
        0x10272b74 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272b78u64) }
        0x10272b78 => { vm.decode(0xf1c4011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10272b7cu64) }
        0x10272b7c => { vm.decode(0x208416fu32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272b80u64) }
        0x10272b80 => { vm.decode(0xb351422u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2188i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10272b84u64) }
        0x10272b84 => { vm.decode(0x43100973u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10272b88u64) }
        0x10272b88 => { vm.decode(0x2884e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272b8cu64) }
        0x10272b8c => { vm.decode(0x19019430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272b90u64) }
        0x10272b90 => { vm.decode(0x93915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272b94u64) }
        0x10272b94 => { vm.decode(0x886e32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272b98u64) }
        0x10272b98 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272b9cu64) }
        0x10272b9c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272ba0u64 + 4);
        vm.wr(37, 0x10272ba0u64 + 4);
        Ok(0x10272ba0u64) }
        0x10272ba0 => { vm.decode(0x625d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2168i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272ba4u64) }
        0x10272ba4 => { vm.decode(0x624d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2136i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272ba8u64) }
        0x10272ba8 => { vm.decode(0x2200616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272bacu64) }
        0x10272bac => { vm.decode(0xa8cc3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272bb0u64) }
        0x10272bb0 => { vm.decode(0x92915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272bb4u64) }
        0x10272bb4 => { vm.decode(0x892115b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272bb8u64) }
        0x10272bb8 => { vm.decode(0x19391430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272bbcu64) }
        0x10272bbc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272bc0u64) }
        0x10272bc0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272bc4u64 + 4);
        vm.wr(37, 0x10272bc4u64 + 4);
        Ok(0x10272bc4u64) }
        0x10272bc4 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272bc8u64) }
        0x10272bc8 => { vm.decode(0x741b1519u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2784i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272bccu64) }
        0x10272bcc => { vm.decode(0x640b8059u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272bd0u64) }
        0x10272bd0 => { vm.decode(0x43099u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 13i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272bd4u64) }
        0x10272bd4 => { vm.decode(0x2208c16fu32); let d = 3i16; let s = 17i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272bd8u64) }
        0x10272bd8 => { vm.decode(0xb3995b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272bdcu64) }
        0x10272bdc => { vm.decode(0x8b31d4b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272be0u64) }
        0x10272be0 => { vm.decode(0x1b2994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272be4u64) }
        0x10272be4 => { vm.decode(0x1b2195b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272be8u64) }
        0x10272be8 => { vm.decode(0x408263afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272becu64) }
        0x10272bec => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272bf0u64) }
        0x10272bf0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272bf4u64) }
        0x10272bf4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272bf8u64 + 4);
        vm.wr(37, 0x10272bf8u64 + 4);
        Ok(0x10272bf8u64) }
        0x10272bf8 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272bfcu64) }
        0x10272bfc => { vm.decode(0x7459b159u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 884i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272c00u64) }
        0x10272c00 => { vm.decode(0x48099u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 9i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272c04u64) }
        0x10272c04 => { vm.decode(0x208c16fu32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272c08u64) }
        0x10272c08 => { vm.decode(0x1b3994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272c0cu64) }
        0x10272c0c => { vm.decode(0x8b31d5f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272c10u64) }
        0x10272c10 => { vm.decode(0x9b2995b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272c14u64) }
        0x10272c14 => { vm.decode(0xb2195b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272c18u64) }
        0x10272c18 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272c1cu64) }
        0x10272c1c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272c20u64) }
        0x10272c20 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272c24u64 + 4);
        vm.wr(37, 0x10272c24u64 + 4);
        Ok(0x10272c24u64) }
        0x10272c24 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272c28u64) }
        0x10272c28 => { vm.decode(0x2800612fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272c2cu64) }
        0x10272c2c => { vm.decode(0x19199c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272c30u64) }
        0x10272c30 => { vm.decode(0x28263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272c34u64) }
        0x10272c34 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272c38u64) }
        0x10272c38 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272c3cu64 + 4);
        vm.wr(37, 0x10272c3cu64 + 4);
        Ok(0x10272c3cu64) }
        0x10272c3c => { vm.decode(0x624f8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2456i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272c40u64) }
        0x10272c40 => { vm.decode(0x621f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272c44u64) }
        0x10272c44 => { vm.decode(0x171c6111u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10272c48u64) }
        0x10272c48 => { vm.decode(0x522d35u32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 16i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10272c4cu64) }
        0x10272c4c => { vm.decode(0x20258f45u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u16;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10272c50u64) }
        0x10272c50 => { vm.decode(0xb101cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272c54u64) }
        0x10272c54 => { vm.decode(0xb081cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272c58u64) }
        0x10272c58 => { vm.decode(0x2200e12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272c5cu64) }
        0x10272c5c => { vm.decode(0x4a8ce36fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272c60u64) }
        0x10272c60 => { vm.decode(0x82c402fu32); let d = 2i16; let s = 18i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272c64u64) }
        0x10272c64 => { vm.decode(0x9399db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272c68u64) }
        0x10272c68 => { vm.decode(0x9931ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272c6cu64) }
        0x10272c6c => { vm.decode(0x19299cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272c70u64) }
        0x10272c70 => { vm.decode(0x428a43efu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272c74u64) }
        0x10272c74 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272c78u64) }
        0x10272c78 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272c7cu64 + 4);
        vm.wr(37, 0x10272c7cu64 + 4);
        Ok(0x10272c7cu64) }
        0x10272c7c => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272c80u64) }
        0x10272c80 => { vm.decode(0x2824402fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272c84u64) }
        0x10272c84 => { vm.decode(0x1b081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272c88u64) }
        0x10272c88 => { vm.decode(0x82c602fu32); let d = 2i16; let s = 18i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272c8cu64) }
        0x10272c8c => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10272c90u64).wrapping_add((imm * 4) as u64) } else { 0x10272c90u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10272c90u64) }
        0x10272c90 => { vm.decode(0xb101db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272c94u64) }
        0x10272c94 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10272c98u64) }
        0x10272c98 => { vm.decode(0x8003036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10272c9cu64) }
        0x10272c9c => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10272ca0u64) }
        0x10272ca0 => { vm.decode(0x2004c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272ca4u64) }
        0x10272ca4 => { vm.decode(0x9921ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272ca8u64) }
        0x10272ca8 => { vm.decode(0x19399c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272cacu64) }
        0x10272cac => { vm.decode(0x624f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272cb0u64) }
        0x10272cb0 => { vm.decode(0x408e4befu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272cb4u64) }
        0x10272cb4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272cb8u64) }
        0x10272cb8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272cbcu64 + 4);
        vm.wr(37, 0x10272cbcu64 + 4);
        Ok(0x10272cbcu64) }
        0x10272cbc => { vm.decode(0xb05875u32); let d = 4i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272cc0u64) }
        0x10272cc0 => { vm.decode(0x408ce3afu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272cc4u64) }
        0x10272cc4 => { vm.decode(0x8012036u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 9i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10272cc8u64) }
        0x10272cc8 => { vm.decode(0x808616fu32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272cccu64) }
        0x10272ccc => { vm.decode(0x6b353022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10272cd0u64) }
        0x10272cd0 => { vm.decode(0x993195b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272cd4u64) }
        0x10272cd4 => { vm.decode(0x92995b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272cd8u64) }
        0x10272cd8 => { vm.decode(0xa86c36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272cdcu64) }
        0x10272cdc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272ce0u64) }
        0x10272ce0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272ce4u64 + 4);
        vm.wr(37, 0x10272ce4u64 + 4);
        Ok(0x10272ce4u64) }
        0x10272ce4 => { vm.decode(0x621f8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2280i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272ce8u64) }
        0x10272ce8 => { vm.decode(0x2800412fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272cecu64) }
        0x10272cec => { vm.decode(0xc8c32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272cf0u64) }
        0x10272cf0 => { vm.decode(0xb0915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272cf4u64) }
        0x10272cf4 => { vm.decode(0x608863efu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272cf8u64) }
        0x10272cf8 => { vm.decode(0x9b0115b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272cfcu64) }
        0x10272cfc => { vm.decode(0x1b191430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272d00u64) }
        0x10272d00 => { vm.decode(0x28643afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272d04u64) }
        0x10272d04 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272d08u64) }
        0x10272d08 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272d0cu64 + 4);
        vm.wr(37, 0x10272d0cu64 + 4);
        Ok(0x10272d0cu64) }
        0x10272d0c => { vm.decode(0x664d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272d10u64) }
        0x10272d10 => { vm.decode(0x625d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2104i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272d14u64) }
        0x10272d14 => { vm.decode(0x64091159u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 832i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272d18u64) }
        0x10272d18 => { vm.decode(0x2800416fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272d1cu64) }
        0x10272d1c => { vm.decode(0x99311530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272d20u64) }
        0x10272d20 => { vm.decode(0x89291530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272d24u64) }
        0x10272d24 => { vm.decode(0xb1015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272d28u64) }
        0x10272d28 => { vm.decode(0x9211530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272d2cu64) }
        0x10272d2c => { vm.decode(0x191914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272d30u64) }
        0x10272d30 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272d34u64) }
        0x10272d34 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272d38u64 + 4);
        vm.wr(37, 0x10272d38u64 + 4);
        Ok(0x10272d38u64) }
        0x10272d38 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272d3cu64) }
        0x10272d3c => { vm.decode(0x2204412fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272d40u64) }
        0x10272d40 => { vm.decode(0x88c3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272d44u64) }
        0x10272d44 => { vm.decode(0x91115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272d48u64) }
        0x10272d48 => { vm.decode(0x8c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272d4cu64) }
        0x10272d4c => { vm.decode(0x90915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272d50u64) }
        0x10272d50 => { vm.decode(0x190114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272d54u64) }
        0x10272d54 => { vm.decode(0xa86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272d58u64) }
        0x10272d58 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272d5cu64) }
        0x10272d5c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272d60u64 + 4);
        vm.wr(37, 0x10272d60u64 + 4);
        Ok(0x10272d60u64) }
        0x10272d60 => { vm.decode(0x62198419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2080i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272d64u64) }
        0x10272d64 => { vm.decode(0x644d9159u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272d68u64) }
        0x10272d68 => { vm.decode(0x2a00416fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272d6cu64) }
        0x10272d6c => { vm.decode(0x288f3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272d70u64) }
        0x10272d70 => { vm.decode(0x9191530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272d74u64) }
        0x10272d74 => { vm.decode(0x19189db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272d78u64) }
        0x10272d78 => { vm.decode(0x19111530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272d7cu64) }
        0x10272d7c => { vm.decode(0x19091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272d80u64) }
        0x10272d80 => { vm.decode(0x8663afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272d84u64) }
        0x10272d84 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272d88u64) }
        0x10272d88 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272d8cu64 + 4);
        vm.wr(37, 0x10272d8cu64 + 4);
        Ok(0x10272d8cu64) }
        0x10272d8c => { vm.decode(0x620d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2056i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272d90u64) }
        0x10272d90 => { vm.decode(0x621f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272d94u64) }
        0x10272d94 => { vm.decode(0x2200e12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272d98u64) }
        0x10272d98 => { vm.decode(0x288f3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272d9cu64) }
        0x10272d9c => { vm.decode(0x9399db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272da0u64) }
        0x10272da0 => { vm.decode(0x9931ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272da4u64) }
        0x10272da4 => { vm.decode(0x19299cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272da8u64) }
        0x10272da8 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272dacu64) }
        0x10272dac => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272db0u64 + 4);
        vm.wr(37, 0x10272db0u64 + 4);
        Ok(0x10272db0u64) }
        0x10272db0 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272db4u64) }
        0x10272db4 => { vm.decode(0x8cd3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272db8u64) }
        0x10272db8 => { vm.decode(0x628663afu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272dbcu64) }
        0x10272dbc => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10272dc0u64).wrapping_add((imm * 4) as u64) } else { 0x10272dc0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10272dc0u64) }
        0x10272dc0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10272dc4u64) } }
        0x10272dc4 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10272dc8u64) }
        0x10272dc8 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10272dccu64) }
        0x10272dcc => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10272dd0u64) }
        0x10272dd0 => { vm.decode(0x8cd3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272dd4u64) }
        0x10272dd4 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10272dd8u64) }
        0x10272dd8 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x10272ddcu64).wrapping_add((imm * 4) as u64) } else { 0x10272ddcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10272ddcu64) }
        0x10272ddc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10272de0u64) } }
        0x10272de0 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10272de4u64).wrapping_add((imm * 4) as u64) } else { 0x10272de4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10272de4u64) }
        0x10272de4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10272de8u64) } }
        0x10272de8 => { vm.decode(0x2200c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272decu64) }
        0x10272dec => { vm.decode(0x9119db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272df0u64) }
        0x10272df0 => { vm.decode(0x19099cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272df4u64) }
        0x10272df4 => { vm.decode(0xa82e36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272df8u64) }
        0x10272df8 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272dfcu64) }
        0x10272dfc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272e00u64 + 4);
        vm.wr(37, 0x10272e00u64 + 4);
        Ok(0x10272e00u64) }
        0x10272e00 => { vm.decode(0x621f8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2472i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272e04u64) }
        0x10272e04 => { vm.decode(0x2b201cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272e08u64) }
        0x10272e08 => { vm.decode(0x288f3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272e0cu64) }
        0x10272e0c => { vm.decode(0x8000a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10272e10u64).wrapping_add((imm * 4) as u64) } else { 0x10272e10u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10272e10u64) }
        0x10272e10 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10272e14u64) } }
        0x10272e14 => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272e18u64) }
        0x10272e18 => { vm.decode(0x2000212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272e1cu64) }
        0x10272e1c => { vm.decode(0xb399530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272e20u64) }
        0x10272e20 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272e24u64) }
        0x10272e24 => { vm.decode(0x28643efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272e28u64) }
        0x10272e28 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272e2cu64) }
        0x10272e2c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272e30u64 + 4);
        vm.wr(37, 0x10272e30u64 + 4);
        Ok(0x10272e30u64) }
        0x10272e30 => { vm.decode(0x664b8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272e34u64) }
        0x10272e34 => { vm.decode(0x2b381c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272e38u64) }
        0x10272e38 => { vm.decode(0x2004c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272e3cu64) }
        0x10272e3c => { vm.decode(0x9921ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272e40u64) }
        0x10272e40 => { vm.decode(0x19399c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272e44u64) }
        0x10272e44 => { vm.decode(0x624f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272e48u64) }
        0x10272e48 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272e4cu64) }
        0x10272e4c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272e50u64 + 4);
        vm.wr(37, 0x10272e50u64 + 4);
        Ok(0x10272e50u64) }
        0x10272e50 => { vm.decode(0xb05875u32); let d = 4i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272e54u64) }
        0x10272e54 => { vm.decode(0x621f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272e58u64) }
        0x10272e58 => { vm.decode(0x2200e12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272e5cu64) }
        0x10272e5c => { vm.decode(0x88c3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272e60u64) }
        0x10272e60 => { vm.decode(0x9399db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272e64u64) }
        0x10272e64 => { vm.decode(0x9931ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272e68u64) }
        0x10272e68 => { vm.decode(0x19299cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272e6cu64) }
        0x10272e6c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272e70u64) }
        0x10272e70 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272e74u64 + 4);
        vm.wr(37, 0x10272e74u64 + 4);
        Ok(0x10272e74u64) }
        0x10272e74 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272e78u64) }
        0x10272e78 => { vm.decode(0x88e3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272e7cu64) }
        0x10272e7c => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10272e80u64).wrapping_add((imm * 4) as u64) } else { 0x10272e80u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10272e80u64) }
        0x10272e80 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10272e84u64) } }
        0x10272e84 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10272e88u64) }
        0x10272e88 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10272e8cu64) }
        0x10272e8c => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10272e90u64) }
        0x10272e90 => { vm.decode(0x88e3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272e94u64) }
        0x10272e94 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10272e98u64) }
        0x10272e98 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x10272e9cu64).wrapping_add((imm * 4) as u64) } else { 0x10272e9cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10272e9cu64) }
        0x10272e9c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10272ea0u64) } }
        0x10272ea0 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10272ea4u64).wrapping_add((imm * 4) as u64) } else { 0x10272ea4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10272ea4u64) }
        0x10272ea4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10272ea8u64) } }
        0x10272ea8 => { vm.decode(0x2200c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272eacu64) }
        0x10272eac => { vm.decode(0x9119db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272eb0u64) }
        0x10272eb0 => { vm.decode(0x19099cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272eb4u64) }
        0x10272eb4 => { vm.decode(0xa82e36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272eb8u64) }
        0x10272eb8 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272ebcu64) }
        0x10272ebc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272ec0u64 + 4);
        vm.wr(37, 0x10272ec0u64 + 4);
        Ok(0x10272ec0u64) }
        0x10272ec0 => { vm.decode(0x621f8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2472i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272ec4u64) }
        0x10272ec4 => { vm.decode(0xb281c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272ec8u64) }
        0x10272ec8 => { vm.decode(0x88c3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272eccu64) }
        0x10272ecc => { vm.decode(0x8000a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10272ed0u64).wrapping_add((imm * 4) as u64) } else { 0x10272ed0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10272ed0u64) }
        0x10272ed0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10272ed4u64) } }
        0x10272ed4 => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272ed8u64) }
        0x10272ed8 => { vm.decode(0x2000212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272edcu64) }
        0x10272edc => { vm.decode(0xb399530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272ee0u64) }
        0x10272ee0 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272ee4u64) }
        0x10272ee4 => { vm.decode(0x28643efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272ee8u64) }
        0x10272ee8 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272eecu64) }
        0x10272eec => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272ef0u64 + 4);
        vm.wr(37, 0x10272ef0u64 + 4);
        Ok(0x10272ef0u64) }
        0x10272ef0 => { vm.decode(0x664b8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272ef4u64) }
        0x10272ef4 => { vm.decode(0xb201c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272ef8u64) }
        0x10272ef8 => { vm.decode(0x2004c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272efcu64) }
        0x10272efc => { vm.decode(0x9921ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272f00u64) }
        0x10272f00 => { vm.decode(0x19399c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272f04u64) }
        0x10272f04 => { vm.decode(0x624f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272f08u64) }
        0x10272f08 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272f0cu64) }
        0x10272f0c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272f10u64 + 4);
        vm.wr(37, 0x10272f10u64 + 4);
        Ok(0x10272f10u64) }
        0x10272f10 => { vm.decode(0xb05875u32); let d = 4i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272f14u64) }
        0x10272f14 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272f18u64) }
        0x10272f18 => { vm.decode(0x9b399530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272f1cu64) }
        0x10272f1c => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272f20u64) }
        0x10272f20 => { vm.decode(0x28643efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272f24u64) }
        0x10272f24 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272f28u64) }
        0x10272f28 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272f2cu64 + 4);
        vm.wr(37, 0x10272f2cu64 + 4);
        Ok(0x10272f2cu64) }
        0x10272f2c => { vm.decode(0x664b8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272f30u64) }
        0x10272f30 => { vm.decode(0x621f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272f34u64) }
        0x10272f34 => { vm.decode(0x2200e12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272f38u64) }
        0x10272f38 => { vm.decode(0x8ce3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272f3cu64) }
        0x10272f3c => { vm.decode(0x9399db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272f40u64) }
        0x10272f40 => { vm.decode(0x9931ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272f44u64) }
        0x10272f44 => { vm.decode(0x19299cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272f48u64) }
        0x10272f48 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272f4cu64) }
        0x10272f4c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272f50u64 + 4);
        vm.wr(37, 0x10272f50u64 + 4);
        Ok(0x10272f50u64) }
        0x10272f50 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272f54u64) }
        0x10272f54 => { vm.decode(0x28cc3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272f58u64) }
        0x10272f58 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10272f5cu64).wrapping_add((imm * 4) as u64) } else { 0x10272f5cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10272f5cu64) }
        0x10272f5c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10272f60u64) } }
        0x10272f60 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10272f64u64) }
        0x10272f64 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10272f68u64) }
        0x10272f68 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10272f6cu64) }
        0x10272f6c => { vm.decode(0x28cc3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272f70u64) }
        0x10272f70 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10272f74u64) }
        0x10272f74 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x10272f78u64).wrapping_add((imm * 4) as u64) } else { 0x10272f78u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10272f78u64) }
        0x10272f78 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10272f7cu64) } }
        0x10272f7c => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10272f80u64).wrapping_add((imm * 4) as u64) } else { 0x10272f80u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10272f80u64) }
        0x10272fd4 => { vm.decode(0x2004c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272fd8u64) }
        0x10272fd8 => { vm.decode(0x9921ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272fdcu64) }
        0x10272fdc => { vm.decode(0x19399c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10272fe0u64) }
        0x10272fe0 => { vm.decode(0x624f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272fe4u64) }
        0x10272fe4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272fe8u64) }
        0x10272fe8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10272fecu64 + 4);
        vm.wr(37, 0x10272fecu64 + 4);
        Ok(0x10272fecu64) }
        0x10272fec => { vm.decode(0xb05875u32); let d = 4i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10272ff0u64) }
        0x10272ff0 => { vm.decode(0x66498519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10272ff4u64) }
        0x10272ff4 => { vm.decode(0x2084e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10272ff8u64) }
        0x10272ff8 => { vm.decode(0x100636u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10272ffcu64) }
        0x10272ffc => { vm.decode(0xa08616fu32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273000u64) }
        0x10273000 => { vm.decode(0x6a1d5122u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10273004u64) }
        0x10273004 => { vm.decode(0x1b2114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273008u64) }
        0x10273008 => { vm.decode(0x9b391530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027300cu64) }
        0x1027300c => { vm.decode(0xb311530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273010u64) }
        0x10273010 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273014u64) }
        0x10273014 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273018u64 + 4);
        vm.wr(37, 0x10273018u64 + 4);
        Ok(0x10273018u64) }
        0x10273018 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027301cu64) }
        0x1027301c => { vm.decode(0x2204612fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273020u64) }
        0x10273020 => { vm.decode(0x5b100873u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10273024u64) }
        0x10273024 => { vm.decode(0x99399530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273028u64) }
        0x10273028 => { vm.decode(0x8c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027302cu64) }
        0x1027302c => { vm.decode(0x9319530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273030u64) }
        0x10273030 => { vm.decode(0x19299430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273034u64) }
        0x10273034 => { vm.decode(0x886c36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273038u64) }
        0x10273038 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027303cu64) }
        0x1027303c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273040u64 + 4);
        vm.wr(37, 0x10273040u64 + 4);
        Ok(0x10273040u64) }
        0x10273040 => { vm.decode(0x620f8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2248i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273044u64) }
        0x10273044 => { vm.decode(0x2a00416fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273048u64) }
        0x10273048 => { vm.decode(0x288f3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027304cu64) }
        0x1027304c => { vm.decode(0x9191530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273050u64) }
        0x10273050 => { vm.decode(0x28c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273054u64) }
        0x10273054 => { vm.decode(0x9111530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273058u64) }
        0x10273058 => { vm.decode(0x19091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027305cu64) }
        0x1027305c => { vm.decode(0x8663afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273060u64) }
        0x10273060 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273064u64) }
        0x10273064 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273068u64 + 4);
        vm.wr(37, 0x10273068u64 + 4);
        Ok(0x10273068u64) }
        0x10273068 => { vm.decode(0x620d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2056i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027306cu64) }
        0x1027306c => { vm.decode(0x621f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273070u64) }
        0x10273070 => { vm.decode(0x2200e12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273074u64) }
        0x10273074 => { vm.decode(0x288f3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273078u64) }
        0x10273078 => { vm.decode(0x9399db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027307cu64) }
        0x1027307c => { vm.decode(0x9931ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273080u64) }
        0x10273080 => { vm.decode(0x19299cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273084u64) }
        0x10273084 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273088u64) }
        0x10273088 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027308cu64 + 4);
        vm.wr(37, 0x1027308cu64 + 4);
        Ok(0x1027308cu64) }
        0x1027308c => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273090u64) }
        0x10273090 => { vm.decode(0x8cd3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273094u64) }
        0x10273094 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10273098u64).wrapping_add((imm * 4) as u64) } else { 0x10273098u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10273098u64) }
        0x10273098 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027309cu64) } }
        0x1027309c => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102730a0u64) }
        0x102730a0 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102730a4u64) }
        0x102730a4 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102730a8u64) }
        0x102730a8 => { vm.decode(0x8cd3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102730acu64) }
        0x102730ac => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102730b0u64) }
        0x102730b0 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x102730b4u64).wrapping_add((imm * 4) as u64) } else { 0x102730b4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102730b4u64) }
        0x102730b4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102730b8u64) } }
        0x102730b8 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102730bcu64).wrapping_add((imm * 4) as u64) } else { 0x102730bcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102730bcu64) }
        0x102730bc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102730c0u64) } }
        0x102730c0 => { vm.decode(0x2200c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102730c4u64) }
        0x102730c4 => { vm.decode(0x9119db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102730c8u64) }
        0x102730c8 => { vm.decode(0x19099cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102730ccu64) }
        0x102730cc => { vm.decode(0xa82e36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102730d0u64) }
        0x102730d0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102730d4u64) }
        0x102730d4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102730d8u64 + 4);
        vm.wr(37, 0x102730d8u64 + 4);
        Ok(0x102730d8u64) }
        0x102730d8 => { vm.decode(0x621f8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2472i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102730dcu64) }
        0x102730dc => { vm.decode(0x2b201cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102730e0u64) }
        0x102730e0 => { vm.decode(0x288f3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102730e4u64) }
        0x102730e4 => { vm.decode(0x8000a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102730e8u64).wrapping_add((imm * 4) as u64) } else { 0x102730e8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102730e8u64) }
        0x102730e8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102730ecu64) } }
        0x102730ec => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102730f0u64) }
        0x102730f0 => { vm.decode(0x2000212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102730f4u64) }
        0x102730f4 => { vm.decode(0xb399530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102730f8u64) }
        0x102730f8 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102730fcu64) }
        0x102730fc => { vm.decode(0x28643efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273100u64) }
        0x10273100 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273104u64) }
        0x10273104 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273108u64 + 4);
        vm.wr(37, 0x10273108u64 + 4);
        Ok(0x10273108u64) }
        0x10273108 => { vm.decode(0x664b8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027310cu64) }
        0x1027310c => { vm.decode(0x2b381c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273110u64) }
        0x10273110 => { vm.decode(0x2004c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273114u64) }
        0x10273114 => { vm.decode(0x9921ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273118u64) }
        0x10273118 => { vm.decode(0x19399c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027311cu64) }
        0x1027311c => { vm.decode(0x624f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273120u64) }
        0x10273120 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273124u64) }
        0x10273124 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273128u64 + 4);
        vm.wr(37, 0x10273128u64 + 4);
        Ok(0x10273128u64) }
        0x10273128 => { vm.decode(0xb05875u32); let d = 4i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027312cu64) }
        0x1027312c => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273130u64) }
        0x10273130 => { vm.decode(0x9b399530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273134u64) }
        0x10273134 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273138u64) }
        0x10273138 => { vm.decode(0x28643efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027313cu64) }
        0x1027313c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273140u64) }
        0x10273140 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273144u64 + 4);
        vm.wr(37, 0x10273144u64 + 4);
        Ok(0x10273144u64) }
        0x10273144 => { vm.decode(0x664b8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273148u64) }
        0x10273148 => { vm.decode(0x66498519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027314cu64) }
        0x1027314c => { vm.decode(0x2a00616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273150u64) }
        0x10273150 => { vm.decode(0x6a1d5122u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10273154u64) }
        0x10273154 => { vm.decode(0x8ce3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273158u64) }
        0x10273158 => { vm.decode(0xc01019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027315cu64) }
        0x1027315c => { vm.decode(0xb2115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273160u64) }
        0x10273160 => { vm.decode(0x9b391530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273164u64) }
        0x10273164 => { vm.decode(0x1b311430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273168u64) }
        0x10273168 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027316cu64) }
        0x1027316c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273170u64 + 4);
        vm.wr(37, 0x10273170u64 + 4);
        Ok(0x10273170u64) }
        0x10273170 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273174u64) }
        0x10273174 => { vm.decode(0x2204612fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273178u64) }
        0x10273178 => { vm.decode(0x5b100873u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1027317cu64) }
        0x1027317c => { vm.decode(0x99399530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273180u64) }
        0x10273180 => { vm.decode(0x8c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273184u64) }
        0x10273184 => { vm.decode(0x9319530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273188u64) }
        0x10273188 => { vm.decode(0x19299430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027318cu64) }
        0x1027318c => { vm.decode(0x886c36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273190u64) }
        0x10273190 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273194u64) }
        0x10273194 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273198u64 + 4);
        vm.wr(37, 0x10273198u64 + 4);
        Ok(0x10273198u64) }
        0x10273198 => { vm.decode(0x620f8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2248i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027319cu64) }
        0x1027319c => { vm.decode(0x641d9059u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102731a0u64) }
        0x102731a0 => { vm.decode(0x2a00416fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102731a4u64) }
        0x102731a4 => { vm.decode(0x288f3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102731a8u64) }
        0x102731a8 => { vm.decode(0x9191530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102731acu64) }
        0x102731ac => { vm.decode(0x19009db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102731b0u64) }
        0x102731b0 => { vm.decode(0x19111530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102731b4u64) }
        0x102731b4 => { vm.decode(0x19091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102731b8u64) }
        0x102731b8 => { vm.decode(0x8663afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102731bcu64) }
        0x102731bc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102731c0u64) }
        0x102731c0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102731c4u64 + 4);
        vm.wr(37, 0x102731c4u64 + 4);
        Ok(0x102731c4u64) }
        0x102731c4 => { vm.decode(0x620d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2056i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102731c8u64) }
        0x102731c8 => { vm.decode(0x621f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102731ccu64) }
        0x102731cc => { vm.decode(0x2200e12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102731d0u64) }
        0x102731d0 => { vm.decode(0x288f3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102731d4u64) }
        0x102731d4 => { vm.decode(0x9399db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102731d8u64) }
        0x102731d8 => { vm.decode(0x9931ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102731dcu64) }
        0x102731dc => { vm.decode(0x19299cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102731e0u64) }
        0x102731e0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102731e4u64) }
        0x102731e4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102731e8u64 + 4);
        vm.wr(37, 0x102731e8u64 + 4);
        Ok(0x102731e8u64) }
        0x102731e8 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102731ecu64) }
        0x102731ec => { vm.decode(0x8cd3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102731f0u64) }
        0x102731f0 => { vm.decode(0x408843efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102731f4u64) }
        0x102731f4 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102731f8u64).wrapping_add((imm * 4) as u64) } else { 0x102731f8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102731f8u64) }
        0x102731f8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102731fcu64) } }
        0x102731fc => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10273200u64) }
        0x10273200 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10273204u64) }
        0x10273204 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10273208u64) }
        0x10273208 => { vm.decode(0x8cd3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027320cu64) }
        0x1027320c => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10273210u64) }
        0x10273210 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x10273214u64).wrapping_add((imm * 4) as u64) } else { 0x10273214u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10273214u64) }
        0x10273214 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10273218u64) } }
        0x10273218 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027321cu64).wrapping_add((imm * 4) as u64) } else { 0x1027321cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027321cu64) }
        0x1027321c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10273220u64) } }
        0x10273220 => { vm.decode(0x2200c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273224u64) }
        0x10273224 => { vm.decode(0x9119db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273228u64) }
        0x10273228 => { vm.decode(0x19099cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027322cu64) }
        0x1027322c => { vm.decode(0xa82e36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273230u64) }
        0x10273230 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273234u64) }
        0x10273234 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273238u64 + 4);
        vm.wr(37, 0x10273238u64 + 4);
        Ok(0x10273238u64) }
        0x10273238 => { vm.decode(0x621f8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2472i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027323cu64) }
        0x1027323c => { vm.decode(0x2b201cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273240u64) }
        0x10273240 => { vm.decode(0x288f3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273244u64) }
        0x10273244 => { vm.decode(0x8000a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10273248u64).wrapping_add((imm * 4) as u64) } else { 0x10273248u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10273248u64) }
        0x10273248 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027324cu64) } }
        0x1027324c => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273250u64) }
        0x10273250 => { vm.decode(0x2000212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273254u64) }
        0x10273254 => { vm.decode(0xb399530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273258u64) }
        0x10273258 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027325cu64) }
        0x1027325c => { vm.decode(0x28643efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273260u64) }
        0x10273260 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273264u64) }
        0x10273264 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273268u64 + 4);
        vm.wr(37, 0x10273268u64 + 4);
        Ok(0x10273268u64) }
        0x10273268 => { vm.decode(0x664b8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027326cu64) }
        0x1027326c => { vm.decode(0x2b381c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273270u64) }
        0x10273270 => { vm.decode(0x2084e32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273274u64) }
        0x10273274 => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10273278u64) }
        0x10273278 => { vm.decode(0x19081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027327cu64) }
        0x1027327c => { vm.decode(0x2284c32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273280u64) }
        0x10273280 => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10273284u64) }
        0x10273284 => { vm.decode(0x192094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273288u64) }
        0x10273288 => { vm.decode(0x2284e32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027328cu64) }
        0x1027328c => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10273290u64) }
        0x10273290 => { vm.decode(0x19289430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273294u64) }
        0x10273294 => { vm.decode(0x2080c36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273298u64) }
        0x10273298 => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027329cu64) }
        0x1027329c => { vm.decode(0x19389430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102732a0u64) }
        0x102732a0 => { vm.decode(0x2080e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102732a4u64) }
        0x102732a4 => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102732a8u64) }
        0x102732a8 => { vm.decode(0x19309430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102732acu64) }
        0x102732ac => { vm.decode(0x2884636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102732b0u64) }
        0x102732b0 => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102732b4u64) }
        0x102732b4 => { vm.decode(0x19281c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102732b8u64) }
        0x102732b8 => { vm.decode(0x2a84436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102732bcu64) }
        0x102732bc => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102732c0u64) }
        0x102732c0 => { vm.decode(0x19201c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102732c4u64) }
        0x102732c4 => { vm.decode(0x2a84636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102732c8u64) }
        0x102732c8 => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102732ccu64) }
        0x102732cc => { vm.decode(0x19181cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102732d0u64) }
        0x102732d0 => { vm.decode(0x2080c32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102732d4u64) }
        0x102732d4 => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102732d8u64) }
        0x102732d8 => { vm.decode(0x19101cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102732dcu64) }
        0x102732dc => { vm.decode(0x2080e32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102732e0u64) }
        0x102732e0 => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102732e4u64) }
        0x102732e4 => { vm.decode(0x193894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102732e8u64) }
        0x102732e8 => { vm.decode(0x2280c32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102732ecu64) }
        0x102732ec => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102732f0u64) }
        0x102732f0 => { vm.decode(0x19101c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102732f4u64) }
        0x102732f4 => { vm.decode(0x2280e32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102732f8u64) }
        0x102732f8 => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102732fcu64) }
        0x102732fc => { vm.decode(0x19081c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273300u64) }
        0x10273300 => { vm.decode(0x2084c32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273304u64) }
        0x10273304 => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10273308u64) }
        0x10273308 => { vm.decode(0x19001c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027330cu64) }
        0x1027330c => { vm.decode(0xf7eb2f6du32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 65535i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10273310u64) }
        0x10273310 => { vm.decode(0x23051062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10273314u64) }
        0x10273314 => { vm.decode(0x2004c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273318u64) }
        0x10273318 => { vm.decode(0x9921ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027331cu64) }
        0x1027331c => { vm.decode(0x19399c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273320u64) }
        0x10273320 => { vm.decode(0x8643efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273324u64) }
        0x10273324 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273328u64) }
        0x10273328 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027332cu64 + 4);
        vm.wr(37, 0x1027332cu64 + 4);
        Ok(0x1027332cu64) }
        0x1027332c => { vm.decode(0x624f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273330u64) }
        0x10273330 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273334u64) }
        0x10273334 => { vm.decode(0x664b8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273338u64) }
        0x10273338 => { vm.decode(0x19185c30u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027333cu64) }
        0x1027333c => { vm.decode(0x9b399530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273340u64) }
        0x10273340 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273344u64) }
        0x10273344 => { vm.decode(0x628c43efu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273348u64) }
        0x10273348 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027334cu64) }
        0x1027334c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273350u64 + 4);
        vm.wr(37, 0x10273350u64 + 4);
        Ok(0x10273350u64) }
        0x10273350 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273354u64) }
        0x10273354 => { vm.decode(0x66498519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273358u64) }
        0x10273358 => { vm.decode(0x1920dcb0u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027335cu64) }
        0x1027335c => { vm.decode(0x761b8119u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 736i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273360u64) }
        0x10273360 => { vm.decode(0xa036u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10273364u64) }
        0x10273364 => { vm.decode(0x624d3922u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10273368u64) }
        0x10273368 => { vm.decode(0xa08616fu32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027336cu64) }
        0x1027336c => { vm.decode(0x6b1d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10273370u64) }
        0x10273370 => { vm.decode(0x9b2155b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273374u64) }
        0x10273374 => { vm.decode(0x9b395430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273378u64) }
        0x10273378 => { vm.decode(0xb311530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027337cu64) }
        0x1027337c => { vm.decode(0x28663afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273380u64) }
        0x10273380 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273384u64) }
        0x10273384 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273388u64 + 4);
        vm.wr(37, 0x10273388u64 + 4);
        Ok(0x10273388u64) }
        0x10273388 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027338cu64) } }
        0x1027338c => { vm.decode(0x721f1419u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2216i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273390u64) }
        0x10273390 => { vm.decode(0x765f0119u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 760i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273394u64) }
        0x10273394 => { vm.decode(0x2000416fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273398u64) }
        0x10273398 => { vm.decode(0x9921d430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027339cu64) }
        0x1027339c => { vm.decode(0x4a8eeb2fu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102733a0u64) }
        0x102733a0 => { vm.decode(0x8919d5f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102733a4u64) }
        0x102733a4 => { vm.decode(0x8911d4b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102733a8u64) }
        0x102733a8 => { vm.decode(0x190994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102733acu64) }
        0x102733ac => { vm.decode(0x428843afu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102733b0u64) }
        0x102733b0 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102733b4u64) }
        0x102733b4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102733b8u64) }
        0x102733b8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102733bcu64 + 4);
        vm.wr(37, 0x102733bcu64 + 4);
        Ok(0x102733bcu64) }
        0x102733bc => { vm.decode(0x90d875u32); let d = 5i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102733c0u64) }
        0x102733c0 => { vm.decode(0x2000416fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102733c4u64) }
        0x102733c4 => { vm.decode(0x8921d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102733c8u64) }
        0x102733c8 => { vm.decode(0x8919d4b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102733ccu64) }
        0x102733cc => { vm.decode(0x408e6befu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102733d0u64) }
        0x102733d0 => { vm.decode(0x8911d5f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102733d4u64) }
        0x102733d4 => { vm.decode(0x190994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102733d8u64) }
        0x102733d8 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102733dcu64) }
        0x102733dc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102733e0u64) }
        0x102733e0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102733e4u64 + 4);
        vm.wr(37, 0x102733e4u64 + 4);
        Ok(0x102733e4u64) }
        0x102733e4 => { vm.decode(0x90d875u32); let d = 5i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102733e8u64) }
        0x102733e8 => { vm.decode(0x319900eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 372310016i64;
        vm.st(d, imm as u64);
        Ok(0x102733ecu64) }
        0x102733ec => { vm.decode(0xc016c699u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 14507i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102733f0u64) }
        0x102733f0 => { vm.decode(0x422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102733f4u64) }
        0x102733f4 => { vm.decode(0x888844eu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 7143424i64;
        vm.st(d, imm as u64);
        Ok(0x102733f8u64) }
        0x102733f8 => { vm.decode(0x80a018d9u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 17667i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102733fcu64) }
        0x102733fc => { vm.decode(0xaa021d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 23i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10273400u64) }
        0x10273400 => { vm.decode(0xa611d9u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1481i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273404u64) }
        0x10273404 => { vm.decode(0xc814e5d9u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -22161i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273408u64) }
        0x10273408 => { vm.decode(0x1a91a54eu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 1484652544i64;
        vm.st(d, imm as u64);
        Ok(0x1027340cu64) }
        0x1027340c => { vm.decode(0xccb0b0d9u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = -30937i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273410u64) }
        0x10273410 => { vm.decode(0x2423d75u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10273414u64) }
        0x10273414 => { vm.decode(0xc082fbd9u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 29127i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273418u64) }
        0x10273418 => { vm.decode(0x22423d75u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 17i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1027341cu64) }
        0x1027341c => { vm.decode(0x422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10273420u64) }
        0x10273420 => { vm.decode(0x82021d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 18i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10273424u64) }
        0x10273424 => { vm.decode(0x410410eu32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 9568256i64;
        vm.st(d, imm as u64);
        Ok(0x10273428u64) }
        0x10273428 => { vm.decode(0x21165d9u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 10597i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027342cu64) }
        0x1027342c => { vm.decode(0x10028d75u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10273430u64) }
        0x10273430 => { vm.decode(0x86136b99u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 29415i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273434u64) }
        0x10273434 => { vm.decode(0xb8028d75u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 23i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10273438u64) }
        0x10273438 => { vm.decode(0x86352399u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 5743i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027343cu64) }
        0x1027343c => { vm.decode(0x84c23999u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 17111i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273440u64) }
        0x10273440 => { vm.decode(0x4056e219u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 12476i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273444u64) }
        0x10273444 => { vm.decode(0x4ca4fd99u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = -4531i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273448u64) }
        0x10273448 => { vm.decode(0x459463d1u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 1892i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027344cu64) }
        0x1027344c => { vm.decode(0x208ef3efu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 1896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273450u64) }
        0x10273450 => { vm.decode(0x3b109530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273454u64) }
        0x10273454 => { vm.decode(0x3b089430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273458u64) }
        0x10273458 => { vm.decode(0x2b009530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027345cu64) }
        0x1027345c => { vm.decode(0x2b3854b0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273460u64) }
        0x10273460 => { vm.decode(0x2b3014b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273464u64) }
        0x10273464 => { vm.decode(0x2004e12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273468u64) }
        0x10273468 => { vm.decode(0x9b19d530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027346cu64) }
        0x1027346c => { vm.decode(0x1b01d5b0u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 2720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273470u64) }
        0x10273470 => { vm.decode(0x2a7d5022u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 2728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10273474u64) }
        0x10273474 => { vm.decode(0x1b119430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273478u64) }
        0x10273478 => { vm.decode(0xa82c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027347cu64) }
        0x1027347c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273480u64) }
        0x10273480 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273484u64 + 4);
        vm.wr(37, 0x10273484u64 + 4);
        Ok(0x10273484u64) }
        0x10273484 => { vm.decode(0x664b8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273488u64) }
        0x10273488 => { vm.decode(0x2204c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027348cu64) }
        0x1027348c => { vm.decode(0x408863efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273490u64) }
        0x10273490 => { vm.decode(0x8b019530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273494u64) }
        0x10273494 => { vm.decode(0x9b3955b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273498u64) }
        0x10273498 => { vm.decode(0x1b3114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027349cu64) }
        0x1027349c => { vm.decode(0xa82e32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102734a0u64) }
        0x102734a0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102734a4u64) }
        0x102734a4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102734a8u64 + 4);
        vm.wr(37, 0x102734a8u64 + 4);
        Ok(0x102734a8u64) }
        0x102734a8 => { vm.decode(0x66598519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2672i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102734acu64) }
        0x102734ac => { vm.decode(0x64499059u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102734b0u64) }
        0x102734b0 => { vm.decode(0x4002036u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102734b4u64) }
        0x102734b4 => { vm.decode(0xa08616fu32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102734b8u64) }
        0x102734b8 => { vm.decode(0x6b1d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102734bcu64) }
        0x102734bc => { vm.decode(0x8b2115b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102734c0u64) }
        0x102734c0 => { vm.decode(0x19109d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102734c4u64) }
        0x102734c4 => { vm.decode(0x1b391530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102734c8u64) }
        0x102734c8 => { vm.decode(0xb311530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102734ccu64) }
        0x102734cc => { vm.decode(0x28663afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102734d0u64) }
        0x102734d0 => { vm.decode(0x2886c36fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102734d4u64) }
        0x102734d4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102734d8u64) }
        0x102734d8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102734dcu64 + 4);
        vm.wr(37, 0x102734dcu64 + 4);
        Ok(0x102734dcu64) }
        0x102734dc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102734e0u64) } }
        0x102734e0 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102734e4u64) }
        0x102734e4 => { vm.decode(0x8b39d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102734e8u64) }
        0x102734e8 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102734ecu64) }
        0x102734ec => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102734f0u64) }
        0x102734f0 => { vm.decode(0x4288e32fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102734f4u64) }
        0x102734f4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102734f8u64) }
        0x102734f8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102734fcu64 + 4);
        vm.wr(37, 0x102734fcu64 + 4);
        Ok(0x102734fcu64) }
        0x102734fc => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273500u64) }
        0x10273500 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273504u64) }
        0x10273504 => { vm.decode(0x8920dcb0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273508u64) }
        0x10273508 => { vm.decode(0x8b39d430u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027350cu64) }
        0x1027350c => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273510u64) }
        0x10273510 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273514u64) }
        0x10273514 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273518u64) }
        0x10273518 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027351cu64 + 4);
        vm.wr(37, 0x1027351cu64 + 4);
        Ok(0x1027351cu64) }
        0x1027351c => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273520u64) }
        0x10273520 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273524u64) }
        0x10273524 => { vm.decode(0x9b205430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273528u64) }
        0x10273528 => { vm.decode(0x9b39d430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027352cu64) }
        0x1027352c => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273530u64) }
        0x10273530 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273534u64) }
        0x10273534 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273538u64) }
        0x10273538 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027353cu64 + 4);
        vm.wr(37, 0x1027353cu64 + 4);
        Ok(0x1027353cu64) }
        0x1027353c => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273540u64) }
        0x10273540 => { vm.decode(0x4d946291u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 868i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10273544u64) }
        0x10273544 => { vm.decode(0x86e3efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273548u64) }
        0x10273548 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027354cu64) }
        0x1027354c => { vm.decode(0x4a8eeb6fu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273550u64) }
        0x10273550 => { vm.decode(0x4888c36fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273554u64) }
        0x10273554 => { vm.decode(0x2500c375u32); let d = 1i16; let s = 5i16; let s2 = 3i16; let imm = 0i64;
        let v = ((vm.ld(s) as i64) < (vm.ld(s2) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10273558u64) }
        0x10273558 => { vm.decode(0x82020071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 13i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1027355cu64).wrapping_add((imm * 4) as u64) } else { 0x1027355cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027355cu64) }
        0x1027355c => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10273560u64) }
        0x10273560 => { vm.decode(0xc80051du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 9752i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10273564u64) }
        0x10273568 => { vm.decode(0x2000103eu32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1027356cu64) }
        0x1027356c => { vm.decode(0x4558bb5u32); let d = 6i16; let s = 2i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10273570u64) }
        0x10273570 => { vm.decode(0x40412835u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273574u64) }
        0x10273574 => { vm.decode(0x20258d35u32); let d = 6i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10273578u64) }
        0x10273578 => { vm.decode(0x2004a835u32); let d = 1i16; let s = 6i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027357cu64) }
        0x1027357c => { vm.decode(0x40480835u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273580u64) }
        0x10273580 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10273584u64) }
        0x10273584 => { vm.decode(0x2500c375u32); let d = 1i16; let s = 5i16; let s2 = 3i16; let imm = 0i64;
        let v = ((vm.ld(s) as i64) < (vm.ld(s2) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10273588u64) }
        0x10273588 => { vm.decode(0x848a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027358cu64).wrapping_add((imm * 4) as u64) } else { 0x1027358cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027358cu64) }
        0x1027358c => { vm.decode(0x2010099u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273590u64) }
        0x10273590 => { vm.decode(0x80816du32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10273594u64) }
        0x10273594 => { vm.decode(0x779c34a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -12i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10273598u64).wrapping_add((imm * 4) as u64) } else { 0x10273598u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10273598u64) }
        0x10273598 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027359cu64) } }
        0x1027359c => { vm.decode(0x2000103eu32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102735a0u64) }
        0x102735a0 => { vm.decode(0x445cbb5u32); let d = 6i16; let s = 2i16; let s2 = -1i16; let imm = 11i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102735a4u64) }
        0x102735a4 => { vm.decode(0x8d875u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102735a8u64) }
        0x102735a8 => { vm.decode(0x30058d35u32); let d = 6i16; let s = 2i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102735acu64) }
        0x102735ac => { vm.decode(0x2004a835u32); let d = 1i16; let s = 6i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102735b0u64) }
        0x102735b0 => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102735b4u64) }
        0x102735b4 => { vm.decode(0x10109b5u32); let d = 2i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102735b8u64) }
        0x102735b8 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102735bcu64) }
        0x102735bc => { vm.decode(0x2500c375u32); let d = 1i16; let s = 5i16; let s2 = 3i16; let imm = 0i64;
        let v = ((vm.ld(s) as i64) < (vm.ld(s2) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102735c0u64) }
        0x102735c0 => { vm.decode(0x27f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -13i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102735c4u64).wrapping_add((imm * 4) as u64) } else { 0x102735c4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102735c4u64) }
        0x102735c4 => { vm.decode(0x2010099u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102735c8u64) }
        0x102735c8 => { vm.decode(0x73002f73u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 732i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102735ccu64) }
        0x102735cc => { vm.decode(0x2004612fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102735d0u64) }
        0x102735d0 => { vm.decode(0x88cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102735d4u64) }
        0x102735d4 => { vm.decode(0xb399530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102735d8u64) }
        0x102735d8 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102735dcu64) }
        0x102735dc => { vm.decode(0x664b8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102735e0u64) }
        0x102735e0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102735e4u64) }
        0x102735e4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102735e8u64 + 4);
        vm.wr(37, 0x102735e8u64 + 4);
        Ok(0x102735e8u64) }
        0x102735e8 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102735ecu64) }
        0x102735ec => { vm.decode(0xbc0fedf6u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102735f0u64) }
        0x102735f0 => { vm.decode(0x620b8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2432i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102735f4u64) }
        0x102735f4 => { vm.decode(0xa8c22fu32); let d = 2i16; let s = 30i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102735f8u64) }
        0x102735f8 => { vm.decode(0x32d1822u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102735fcu64) }
        0x102735fc => { vm.decode(0x2b3815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273600u64) }
        0x10273600 => { vm.decode(0x408243afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273604u64) }
        0x10273604 => { vm.decode(0xab3054b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273608u64) }
        0x10273608 => { vm.decode(0x2204e12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027360cu64) }
        0x1027360c => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273610u64) }
        0x10273610 => { vm.decode(0x9099d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273614u64) }
        0x10273614 => { vm.decode(0x19019c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273618u64) }
        0x10273618 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027361cu64) }
        0x1027361c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273620u64 + 4);
        vm.wr(37, 0x10273620u64 + 4);
        Ok(0x10273620u64) }
        0x10273620 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273624u64) }
        0x10273624 => { vm.decode(0xab3054b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273628u64) }
        0x10273628 => { vm.decode(0xf186811u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027362cu64) }
        0x1027362c => { vm.decode(0x32d1822u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10273630u64) }
        0x10273630 => { vm.decode(0x628a63efu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273634u64) }
        0x10273634 => { vm.decode(0x82002031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10273638u64).wrapping_add((imm * 4) as u64) } else { 0x10273638u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10273638u64) }
        0x10273638 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027363cu64) } }
        0x1027363c => { vm.decode(0x2000c16fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273640u64) }
        0x10273640 => { vm.decode(0x28c73efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273644u64) }
        0x10273644 => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273648u64) }
        0x10273648 => { vm.decode(0x9099d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027364cu64) }
        0x1027364c => { vm.decode(0x19019c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273650u64) }
        0x10273650 => { vm.decode(0x620b8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2432i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273654u64) }
        0x10273654 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273658u64) }
        0x10273658 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027365cu64 + 4);
        vm.wr(37, 0x1027365cu64 + 4);
        Ok(0x1027365cu64) }
        0x1027365c => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273660u64) }
        0x10273660 => { vm.decode(0x9d4c011u32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10273664u64) }
        0x10273664 => { vm.decode(0x8000236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10273668u64) }
        0x10273668 => { vm.decode(0x761b1019u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027366cu64) }
        0x1027366c => { vm.decode(0x765f8019u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 696i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273670u64) }
        0x10273670 => { vm.decode(0x83628071u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 189i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10273674u64).wrapping_add((imm * 4) as u64) } else { 0x10273674u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10273674u64) }
        0x10273674 => { vm.decode(0x8918dd30u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273678u64) }
        0x10273678 => { vm.decode(0x2200c16fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027367cu64) }
        0x10273968 => { vm.decode(0x20a0c22fu32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027396cu64) }
        0x1027396c => { vm.decode(0xbc0fcff6u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10273970u64) }
        0x10273970 => { vm.decode(0x22d5822u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10273974u64) }
        0x10273974 => { vm.decode(0x3b3814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273978u64) }
        0x10273978 => { vm.decode(0x408243afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027397cu64) }
        0x1027397c => { vm.decode(0xab3054b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273980u64) }
        0x10273980 => { vm.decode(0x20ce12fu32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273984u64) }
        0x10273984 => { vm.decode(0x40048019u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273988u64) }
        0x10273988 => { vm.decode(0x19099c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027398cu64) }
        0x1027398c => { vm.decode(0x9019d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273990u64) }
        0x10273990 => { vm.decode(0x620b8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2432i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273994u64) }
        0x10273994 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273998u64) }
        0x10273998 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027399cu64 + 4);
        vm.wr(37, 0x1027399cu64 + 4);
        Ok(0x1027399cu64) }
        0x1027399c => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102739a0u64) }
        0x102739a0 => { vm.decode(0xab3054b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102739a4u64) }
        0x102739a4 => { vm.decode(0xf186811u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102739a8u64) }
        0x102739a8 => { vm.decode(0x528c00a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 980i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102739acu64).wrapping_add((imm * 4) as u64) } else { 0x102739acu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102739acu64) }
        0x102739ac => { vm.decode(0x32d1822u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102739b0u64) }
        0x102739b0 => { vm.decode(0x9d4c011u32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102739b4u64) }
        0x102739bc => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102739c0u64) }
        0x102739c0 => { vm.decode(0x2000036u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102739c4u64) }
        0x102739c4 => { vm.decode(0x8a63efu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102739c8u64) }
        0x102739c8 => { vm.decode(0x608843afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102739ccu64) }
        0x102739cc => { vm.decode(0x1020406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 2i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102739d0u64) }
        0x102739d0 => { vm.decode(0x82020071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 13i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102739d4u64).wrapping_add((imm * 4) as u64) } else { 0x102739d4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102739d4u64) }
        0x102739d4 => { vm.decode(0x64599159u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102739d8u64) }
        0x102739d8 => { vm.decode(0x8080451du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 10896i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102739dcu64) }
        0x102739e0 => { vm.decode(0x2008023eu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102739e4u64) }
        0x102739e4 => { vm.decode(0x4178bb5u32); let d = 5i16; let s = 2i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102739e8u64) }
        0x102739e8 => { vm.decode(0x60012835u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102739ecu64) }
        0x102739ec => { vm.decode(0x20348d35u32); let d = 5i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102739f0u64) }
        0x102739f0 => { vm.decode(0x6000a835u32); let d = 1i16; let s = 5i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102739f4u64) }
        0x102739f4 => { vm.decode(0x40480835u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102739f8u64) }
        0x102739f8 => { vm.decode(0xa400036u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102739fcu64) }
        0x102739fc => { vm.decode(0x1020406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 2i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10273a00u64) }
        0x10273a00 => { vm.decode(0x848a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10273a04u64).wrapping_add((imm * 4) as u64) } else { 0x10273a04u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10273a04u64) }
        0x10273a04 => { vm.decode(0x40809099u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273a08u64) }
        0x10273a08 => { vm.decode(0x80016du32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10273a0cu64) }
        0x10273a0c => { vm.decode(0x779c34a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -12i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10273a10u64).wrapping_add((imm * 4) as u64) } else { 0x10273a10u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10273a10u64) }
        0x10273a10 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10273a14u64) } }
        0x10273a14 => { vm.decode(0x2008023eu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10273a18u64) }
        0x10273a18 => { vm.decode(0x407cbb5u32); let d = 5i16; let s = 2i16; let s2 = -1i16; let imm = 11i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10273a1cu64) }
        0x10273a1c => { vm.decode(0x408c875u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273a20u64) }
        0x10273a20 => { vm.decode(0x30148d35u32); let d = 5i16; let s = 2i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10273a24u64) }
        0x10273a24 => { vm.decode(0x6000a835u32); let d = 1i16; let s = 5i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273a28u64) }
        0x10273a28 => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273a2cu64) }
        0x10273a2c => { vm.decode(0x10109b5u32); let d = 2i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x10273a30u64) }
        0x10273a30 => { vm.decode(0xa400036u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10273a34u64) }
        0x10273a34 => { vm.decode(0x1020406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 2i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10273a38u64) }
        0x10273a38 => { vm.decode(0x27f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -13i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10273a3cu64).wrapping_add((imm * 4) as u64) } else { 0x10273a3cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10273a3cu64) }
        0x10273a3c => { vm.decode(0x40809099u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273a40u64) }
        0x10273a40 => { vm.decode(0xf3002bf3u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 731i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10273a44u64) }
        0x10273a44 => { vm.decode(0x60100d35u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10273a48u64) }
        0x10273a48 => { vm.decode(0x53022bf3u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 730i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10273a4cu64) }
        0x10273a4c => { vm.decode(0x20120d35u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10273a50u64) }
        0x10273a50 => { vm.decode(0xd3022b73u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 729i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10273a54u64) }
        0x10273a54 => { vm.decode(0x60120d35u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10273a58u64) }
        0x10273a58 => { vm.decode(0x53022b73u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10273a5cu64) }
        0x10273a5c => { vm.decode(0x2200416fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273a60u64) }
        0x10273a60 => { vm.decode(0x45106291u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 772i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10273a64u64) }
        0x10273a64 => { vm.decode(0xa355422u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2188i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10273a68u64) }
        0x10273a68 => { vm.decode(0x100236u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10273a6cu64) }
        0x10273a6c => { vm.decode(0x43100973u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10273a70u64) }
        0x10273a70 => { vm.decode(0x901d530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 2176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273a74u64) }
        0x10273a74 => { vm.decode(0x193914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273a78u64) }
        0x10273a78 => { vm.decode(0x886e32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273a7cu64) }
        0x10273a7c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273a80u64) }
        0x10273a80 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273a84u64 + 4);
        vm.wr(37, 0x10273a84u64 + 4);
        Ok(0x10273a84u64) }
        0x10273a84 => { vm.decode(0x625d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2168i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273a88u64) }
        0x10273a88 => { vm.decode(0x1019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273a8cu64) }
        0x10273a8c => { vm.decode(0x4d106291u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 772i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10273a90u64) }
        0x10273a90 => { vm.decode(0xbe2fddf6u32); let d = 4i16; let s = 3i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10273a94u64) }
        0x10273a94 => { vm.decode(0xb60dedf6u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = -4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10273a98u64) }
        0x10273a98 => { vm.decode(0x508a375u32); let d = 1i16; let s = 18i16; let s2 = 3i16; let imm = 0i64;
        let v = ((vm.ld(s) as i64) < (vm.ld(s2) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10273a9cu64) }
        0x10273a9c => { vm.decode(0x8c8027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 21i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10273aa0u64).wrapping_add((imm * 4) as u64) } else { 0x10273aa0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10273aa0u64) }
        0x10273aa0 => { vm.decode(0x664f1119u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 728i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273aa4u64) }
        0x10273aa4 => { vm.decode(0x7018e7b5u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 31i64;
        let v = ((vm.ld(s) as i32) >> (imm as u32)) as i64;
        vm.st(d, v as u64);
        Ok(0x10273aa8u64) }
        0x10273aa8 => { vm.decode(0x50b20d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 30i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10273aacu64) }
        0x10273aac => { vm.decode(0x40290fb5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10273ab0u64) }
        0x10273ab0 => { vm.decode(0x20317f5u32); let d = 1i16; let s = 5i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273ab4u64) }
        0x10273ab4 => { vm.decode(0xa8080b35u32); let d = 1i16; let s = 18i16; let s2 = 1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_sub(vm.ld(s2) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10273ab8u64) }
        0x10273ab8 => { vm.decode(0x660bb5u32); let d = 7i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10273abcu64) }
        0x10273abc => { vm.decode(0x80ebafu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273ac0u64) }
        0x10273ac0 => { vm.decode(0xc86284f5u32); let d = 7i16; let s = 7i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10273ac4u64) }
        0x10273ac4 => { vm.decode(0x845befu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 1632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273ac8u64) }
        0x10273ac8 => { vm.decode(0x408084f5u32); let d = 8i16; let s = 2i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10273accu64) }
        0x10273acc => { vm.decode(0x2a08123eu32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10273ad0u64) }
        0x10273ad0 => { vm.decode(0x824024f5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10273ad4u64) }
        0x10273ad4 => { vm.decode(0xbe4fcdf6u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10273ad8u64) }
        0x10273ad8 => { vm.decode(0x8b00236u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10273adcu64) }
        0x10273adc => { vm.decode(0x2000023eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10273ae0u64) }
        0x10273ae0 => { vm.decode(0x60412835u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273ae4u64) }
        0x10273ae4 => { vm.decode(0x20073u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10273ae8u64) }
        0x10273ae8 => { vm.decode(0x508a375u32); let d = 1i16; let s = 18i16; let s2 = 3i16; let imm = 0i64;
        let v = ((vm.ld(s) as i64) < (vm.ld(s2) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10273aecu64) }
        0x10273aec => { vm.decode(0xa7d30df1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -19i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10273af0u64).wrapping_add((imm * 4) as u64) } else { 0x10273af0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10273af0u64) }
        0x10273af0 => { vm.decode(0x801099u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273af4u64) }
        0x10273af4 => { vm.decode(0x80008099u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273af8u64) }
        0x10273af8 => { vm.decode(0x45b935u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10273afcu64) }
        0x10273afc => { vm.decode(0x1b309430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273b00u64) }
        0x10273b00 => { vm.decode(0x28cf32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273b04u64) }
        0x10273b04 => { vm.decode(0x208003eu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10273b08u64) }
        0x10273b08 => { vm.decode(0x505a412du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 63i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10273b0cu64) }
        0x10273b0c => { vm.decode(0x85402935u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 46i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10273b10u64) }
        0x10273b10 => { vm.decode(0x208cd32fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273b14u64) }
        0x10273b14 => { vm.decode(0x620b8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2432i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273b18u64) }
        0x10273b18 => { vm.decode(0xbe0fcdf6u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10273b1cu64) }
        0x10273b1c => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273b20u64) }
        0x10273b20 => { vm.decode(0x1b309430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273b24u64) }
        0x10273b24 => { vm.decode(0x208023eu32); let d = 2i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10273b28u64) }
        0x10273b28 => { vm.decode(0x505a412du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 63i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10273b2cu64) }
        0x10273b2c => { vm.decode(0x81400935u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 40i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10273b30u64) }
        0x10273b30 => { vm.decode(0x4408875u32); let d = 1i16; let s = 2i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273b34u64) }
        0x10273b34 => { vm.decode(0x1b309430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273b38u64) }
        0x10273b38 => { vm.decode(0x20a0c22fu32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273b3cu64) }
        0x10273b3c => { vm.decode(0x26d1822u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10273b40u64) }
        0x10273b40 => { vm.decode(0x3b3814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273b44u64) }
        0x10273b44 => { vm.decode(0xbb3015b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 1648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273b48u64) }
        0x10273b48 => { vm.decode(0x20ce12fu32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273b4cu64) }
        0x10273b4c => { vm.decode(0x40048019u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273b50u64) }
        0x10273b50 => { vm.decode(0x19099c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273b54u64) }
        0x10273b54 => { vm.decode(0x9019d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273b58u64) }
        0x10273b58 => { vm.decode(0x4888c36fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273b5cu64) }
        0x10273b5c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273b60u64) }
        0x10273b60 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273b64u64 + 4);
        vm.wr(37, 0x10273b64u64 + 4);
        Ok(0x10273b64u64) }
        0x10273b64 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273b68u64) }
        0x10273b68 => { vm.decode(0xbb3015b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 1648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273b6cu64) }
        0x10273b6c => { vm.decode(0xf186811u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10273b70u64) }
        0x10273b70 => { vm.decode(0x82002031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10273b74u64).wrapping_add((imm * 4) as u64) } else { 0x10273b74u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10273b74u64) }
        0x10273b74 => { vm.decode(0x32d1822u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10273b78u64) }
        0x10273b78 => { vm.decode(0x2000c16fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273b7cu64) }
        0x10273b7c => { vm.decode(0x28c73efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273b80u64) }
        _ => Err(Error::UnknownPc(pc)),
    }
}
