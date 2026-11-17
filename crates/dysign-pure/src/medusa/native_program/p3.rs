#![cfg_attr(rustfmt, rustfmt::skip)]
// 由 scripts/medusa-vm/gen_native.py 生成，勿手改。
use super::super::Vm;
use super::super::Error;

#[allow(clippy::all, unused_parens, unused_variables, unused_mut)]
pub fn exec(vm: &mut Vm, pc: u64) -> Result<u64, Error> {
    match pc {
        0x10278c84 => { vm.decode(0x62398519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3168i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10278c88u64) }
        0x10278c88 => { vm.decode(0x22a460afu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278c8cu64) }
        0x10278c8c => { vm.decode(0xa93915b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 3192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278c90u64) }
        0x10278c90 => { vm.decode(0xb93154b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 3184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278c94u64) }
        0x10278c94 => { vm.decode(0xa92955f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278c98u64) }
        0x10278c98 => { vm.decode(0x392114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278c9cu64) }
        0x10278c9c => { vm.decode(0xa86636fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278ca0u64) }
        0x10278ca0 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278ca4u64) }
        0x10278ca4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278ca8u64 + 4);
        vm.wr(37, 0x10278ca8u64 + 4);
        Ok(0x10278ca8u64) }
        0x10278ca8 => { vm.decode(0x8909875u32); let d = 21i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278cacu64) }
        0x10278cac => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278cb0u64) }
        0x10278cb0 => { vm.decode(0x8d0815b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278cb4u64) }
        0x10278cb4 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278cb8u64) }
        0x10278cb8 => { vm.decode(0x4286532fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278cbcu64) }
        0x10278cbc => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278cc0u64) }
        0x10278cc0 => { vm.decode(0x4a88c32fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278cc4u64) }
        0x10278cc4 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278cc8u64) }
        0x10278cc8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278cccu64 + 4);
        vm.wr(37, 0x10278cccu64 + 4);
        Ok(0x10278cccu64) }
        0x10278ccc => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278cd0u64) }
        0x10278cd0 => { vm.decode(0x20a040efu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278cd4u64) }
        0x10278cd4 => { vm.decode(0x288d32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278cd8u64) }
        0x10278cd8 => { vm.decode(0x29311530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278cdcu64) }
        0x10278cdc => { vm.decode(0x288e32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278ce0u64) }
        0x10278ce0 => { vm.decode(0x29291530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278ce4u64) }
        0x10278ce4 => { vm.decode(0x88853afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278ce8u64) }
        0x10278ce8 => { vm.decode(0x29211530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278cecu64) }
        0x10278cec => { vm.decode(0x391914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278cf0u64) }
        0x10278cf0 => { vm.decode(0xa86436fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278cf4u64) }
        0x10278cf4 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278cf8u64) }
        0x10278cf8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278cfcu64 + 4);
        vm.wr(37, 0x10278cfcu64 + 4);
        Ok(0x10278cfcu64) }
        0x10278cfc => { vm.decode(0x627d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10278d00u64) }
        0x10278d00 => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278d04u64) }
        0x10278d04 => { vm.decode(0x8d0855f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278d08u64) }
        0x10278d08 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278d0cu64) }
        0x10278d0c => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278d10u64) }
        0x10278d10 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278d14u64) }
        0x10278d14 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278d18u64 + 4);
        vm.wr(37, 0x10278d18u64 + 4);
        Ok(0x10278d18u64) }
        0x10278d18 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278d1cu64) }
        0x10278d1c => { vm.decode(0xd98c811u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10278d20u64) }
        0x10278d20 => { vm.decode(0x840bb5u32); let d = 4i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10278d24u64) }
        0x10278d24 => { vm.decode(0x2420001au32); let d = 2i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = ((vm.ld(s) as u32) as i32) as f64;
        vm.st(d, v.to_bits());
        Ok(0x10278d28u64) }
        0x10278d28 => { vm.decode(0x22a040afu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278d2cu64) }
        0x10278d2c => { vm.decode(0x29199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278d30u64) }
        0x10278d30 => { vm.decode(0x88843efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278d34u64) }
        0x10278d34 => { vm.decode(0x29119530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278d38u64) }
        0x10278d38 => { vm.decode(0x88853afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278d3cu64) }
        0x10278d3c => { vm.decode(0x29099530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278d40u64) }
        0x10278d40 => { vm.decode(0x39019430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278d44u64) }
        0x10278d44 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278d48u64) }
        0x10278d48 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278d4cu64) }
        0x10278d4c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278d50u64 + 4);
        vm.wr(37, 0x10278d50u64 + 4);
        Ok(0x10278d50u64) }
        0x10278d50 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278d54u64) }
        0x10278d54 => { vm.decode(0x488c63efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278d58u64) }
        0x10278d58 => { vm.decode(0xc8a0227u32); let d = 0i16; let s = 18i16; let s2 = -1i16; let imm = 60i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10278d5cu64).wrapping_add((imm * 4) as u64) } else { 0x10278d5cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10278d5cu64) }
        0x10278d5c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10278d60u64) } }
        0x10278d60 => { vm.decode(0x9408091u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10278d64u64) }
        0x10278d64 => { vm.decode(0x80804e1u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 58i64;
        vm.wr(36, if (vm.ld(s) as i64) < 1 { (0x10278d68u64).wrapping_add((imm * 4) as u64) } else { 0x10278d68u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10278d68u64) }
        0x10278d68 => { vm.decode(0xa100036u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10278d6cu64) }
        0x10278d6c => { vm.decode(0x9408091u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10278d70u64) }
        0x10278e50 => { vm.decode(0x2084732fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278e54u64) }
        0x10278e54 => { vm.decode(0x88ce32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278e58u64) }
        0x10278e58 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10278e5cu64) }
        0x10278e5c => { vm.decode(0x290015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278e60u64) }
        0x10278e60 => { vm.decode(0xa8cc32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278e64u64) }
        0x10278e64 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10278e68u64) }
        0x10278e68 => { vm.decode(0xb2895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278e6cu64) }
        0x10278e6c => { vm.decode(0xa8ce32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278e70u64) }
        0x10278e70 => { vm.decode(0x8a4404f5u32); let d = 19i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10278e74u64) }
        0x10278e74 => { vm.decode(0x22a040efu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278e78u64) }
        0x10278e78 => { vm.decode(0x5b50eb73u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 3064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10278e7cu64) }
        0x10278e7c => { vm.decode(0xa88e32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278e80u64) }
        0x10278e80 => { vm.decode(0xb319db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278e84u64) }
        0x10278e84 => { vm.decode(0x88853afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278e88u64) }
        0x10278e88 => { vm.decode(0xb299db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3048i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278e8cu64) }
        0x10278e8c => { vm.decode(0x1b219cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278e90u64) }
        0x10278e90 => { vm.decode(0x286e36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278e94u64) }
        0x10278e94 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278e98u64) }
        0x10278e98 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278e9cu64 + 4);
        vm.wr(37, 0x10278e9cu64 + 4);
        Ok(0x10278e9cu64) }
        0x10278e9c => { vm.decode(0x661b8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3040i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10278ea0u64) }
        0x10278ea0 => { vm.decode(0x22a060efu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278ea4u64) }
        0x10278ea4 => { vm.decode(0x88853afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278ea8u64) }
        0x10278ea8 => { vm.decode(0xb399d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3032i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278eacu64) }
        0x10278eac => { vm.decode(0x88c3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278eb0u64) }
        0x10278eb0 => { vm.decode(0xb319d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278eb4u64) }
        0x10278eb4 => { vm.decode(0x1b299c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278eb8u64) }
        0x10278eb8 => { vm.decode(0x882c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278ebcu64) }
        0x10278ebc => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278ec0u64) }
        0x10278ec0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278ec4u64 + 4);
        vm.wr(37, 0x10278ec4u64 + 4);
        Ok(0x10278ec4u64) }
        0x10278ec4 => { vm.decode(0x660f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3016i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10278ec8u64) }
        0x10278ec8 => { vm.decode(0xa00de9b6u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = -244i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10278eccu64) }
        0x10278ecc => { vm.decode(0x8c73efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278ed0u64) }
        0x10278ed0 => { vm.decode(0x228002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278ed4u64) }
        0x10278ed4 => { vm.decode(0x9299db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278ed8u64) }
        0x10278ed8 => { vm.decode(0x4a8c43efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278edcu64) }
        0x10278edc => { vm.decode(0xa28602fu32); let d = 2i16; let s = 18i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278ee0u64) }
        0x10278ee0 => { vm.decode(0x433d3822u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10278ee4u64) }
        0x10278ee4 => { vm.decode(0x2b309530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278ee8u64) }
        0x10278ee8 => { vm.decode(0x2280532fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278eecu64) }
        0x10278eec => { vm.decode(0x3b289430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278ef0u64) }
        0x10278ef0 => { vm.decode(0x22a4e02fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278ef4u64) }
        0x10278ef4 => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10278ef8u64) }
        0x10278ef8 => { vm.decode(0xb199db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278efcu64) }
        0x10278efc => { vm.decode(0x1b119cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278f00u64) }
        0x10278f00 => { vm.decode(0x88643efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278f04u64) }
        0x10278f04 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278f08u64) }
        0x10278f08 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278f0cu64 + 4);
        vm.wr(37, 0x10278f0cu64 + 4);
        Ok(0x10278f0cu64) }
        0x10278f0c => { vm.decode(0x665b8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2992i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10278f10u64) }
        0x10278f10 => { vm.decode(0x72090219u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 4096i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10278f14u64) }
        0x10278f14 => { vm.decode(0x8119u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10278f18u64) }
        0x10278f18 => { vm.decode(0x47986211u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10278f1cu64) }
        0x10278f1c => { vm.decode(0x423d7822u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10278f20u64) }
        0x10278f20 => { vm.decode(0x2ac60efu32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278f24u64) }
        0x10278f24 => { vm.decode(0x1d181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278f28u64) }
        0x10278f28 => { vm.decode(0x288063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278f2cu64) }
        0x10278f2c => { vm.decode(0x1d101430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278f30u64) }
        0x10278f30 => { vm.decode(0x9d085430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 4104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278f34u64) }
        0x10278f34 => { vm.decode(0xd001530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278f38u64) }
        0x10278f38 => { vm.decode(0xc10c875u32); let d = 17i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278f3cu64) }
        0x10278f3c => { vm.decode(0x6882e3efu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278f40u64) }
        0x10278f40 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278f44u64) }
        0x10278f44 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278f48u64) }
        0x10278f48 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278f4cu64 + 4);
        vm.wr(37, 0x10278f4cu64 + 4);
        Ok(0x10278f4cu64) }
        0x10278f4c => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278f50u64) }
        0x10278f50 => { vm.decode(0x148059u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 296i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10278f54u64) }
        0x10278f54 => { vm.decode(0x82c602fu32); let d = 2i16; let s = 18i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278f58u64) }
        0x10278f58 => { vm.decode(0x2b201530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278f5cu64) }
        0x10278f5c => { vm.decode(0x414c0811u32); let d = 2i16; let s = 18i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10278f60u64) }
        0x10278f60 => { vm.decode(0x421d5822u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10278f64u64) }
        0x10278f64 => { vm.decode(0xa2c602fu32); let d = 2i16; let s = 18i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278f68u64) }
        0x10278f68 => { vm.decode(0x228002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278f6cu64) }
        0x10278f6c => { vm.decode(0x2b301530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278f70u64) }
        0x10278f70 => { vm.decode(0x41c88011u32); let d = 2i16; let s = 18i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10278f74u64) }
        0x10278f74 => { vm.decode(0x421d7822u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10278f78u64) }
        0x10278f78 => { vm.decode(0x41c88091u32); let d = 2i16; let s = 18i16; let s2 = -1i16; let imm = 204i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10278f7cu64) }
        0x10278f7c => { vm.decode(0x421d7c22u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1628i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10278f80u64) }
        0x10278f80 => { vm.decode(0x41c80811u32); let d = 2i16; let s = 18i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10278f84u64) }
        0x10278f84 => { vm.decode(0xbb089cb0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 1960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278f88u64) }
        0x10278f88 => { vm.decode(0x620d5822u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10278f8cu64) }
        0x10278f8c => { vm.decode(0x2b009cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278f90u64) }
        0x10278f90 => { vm.decode(0x2ac60efu32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278f94u64) }
        0x10278f94 => { vm.decode(0x1d181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278f98u64) }
        0x10278f98 => { vm.decode(0x2a8043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278f9cu64) }
        0x10278f9c => { vm.decode(0x1d101430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278fa0u64) }
        0x10278fa0 => { vm.decode(0x8d085570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 4104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278fa4u64) }
        0x10278fa4 => { vm.decode(0xd001530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278fa8u64) }
        0x10278fa8 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278facu64) }
        0x10278fac => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278fb0u64) }
        0x10278fb0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278fb4u64 + 4);
        vm.wr(37, 0x10278fb4u64 + 4);
        Ok(0x10278fb4u64) }
        0x10278fb4 => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278fb8u64) }
        0x10278fb8 => { vm.decode(0xd1c6091u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 420i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10278fbcu64) }
        0x10278fbc => { vm.decode(0x80812du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10278fc0u64) }
        0x10278fc0 => { vm.decode(0x100c0a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 90i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10278fc4u64).wrapping_add((imm * 4) as u64) } else { 0x10278fc4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10278fc4u64) }
        0x10278fc4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10278fc8u64) } }
        0x10278fc8 => { vm.decode(0x8002036u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10278fccu64) }
        0x10278fcc => { vm.decode(0x3153862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10278fd0u64) }
        0x10278fd0 => { vm.decode(0x94c0011u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10278fd4u64) }
        0x10278fd4 => { vm.decode(0xacc436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278fd8u64) }
        0x10278fd8 => { vm.decode(0x29001db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278fdcu64) }
        0x10278fdc => { vm.decode(0x3153c62u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1308i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10278fe0u64) }
        0x10278fe0 => { vm.decode(0x28c0632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278fe4u64) }
        0x10278fe4 => { vm.decode(0x39081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278fe8u64) }
        0x10278fe8 => { vm.decode(0x2824606fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278fecu64) }
        0x10278fec => { vm.decode(0x39101cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278ff0u64) }
        0x10278ff0 => { vm.decode(0x2020c02fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278ff4u64) }
        0x10278ff4 => { vm.decode(0x39181cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278ff8u64) }
        0x10278ff8 => { vm.decode(0x940a011u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10278ffcu64) }
        0x10278ffc => { vm.decode(0x43051862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10279000u64) }
        0x10279000 => { vm.decode(0x940a091u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 268i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10279004u64) }
        0x10279004 => { vm.decode(0x43051c62u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1348i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10279008u64) }
        0x10279008 => { vm.decode(0x9402811u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027900cu64) }
        0x1027900c => { vm.decode(0x43151862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10279010u64) }
        0x10279010 => { vm.decode(0x9402891u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 276i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10279014u64) }
        0x10279014 => { vm.decode(0x43151c62u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1356i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10279018u64) }
        0x10279018 => { vm.decode(0x2024e02fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027901cu64) }
        0x1027901c => { vm.decode(0x39301c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279020u64) }
        0x10279020 => { vm.decode(0x2020c06fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279024u64) }
        0x10279024 => { vm.decode(0x39381c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279028u64) }
        0x10279028 => { vm.decode(0x2220e06fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027902cu64) }
        0x1027902c => { vm.decode(0x39201cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279030u64) }
        0x10279030 => { vm.decode(0x9c42011u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10279034u64) }
        0x10279034 => { vm.decode(0x63151862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10279038u64) }
        0x10279038 => { vm.decode(0x2224e06fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027903cu64) }
        0x1027903c => { vm.decode(0x39301cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279040u64) }
        0x10279040 => { vm.decode(0x9482011u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10279044u64) }
        0x10279044 => { vm.decode(0x63153862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10279048u64) }
        0x10279048 => { vm.decode(0x9482091u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 388i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027904cu64) }
        0x1027904c => { vm.decode(0x63153c62u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1404i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10279050u64) }
        0x10279050 => { vm.decode(0x948a011u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10279054u64) }
        0x10279054 => { vm.decode(0x3251862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10279058u64) }
        0x10279058 => { vm.decode(0x948a091u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 396i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027905cu64) }
        0x1027905c => { vm.decode(0x3251c62u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1412i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10279060u64) }
        0x10279060 => { vm.decode(0x9482811u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10279064u64) }
        0x10279064 => { vm.decode(0x3351862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10279068u64) }
        0x10279068 => { vm.decode(0x9482891u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 404i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027906cu64) }
        0x1027906c => { vm.decode(0x3351c62u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1420i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10279070u64) }
        0x10279070 => { vm.decode(0x2824e02fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279074u64) }
        0x10279074 => { vm.decode(0x39109c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279078u64) }
        0x10279078 => { vm.decode(0x94c2811u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027907cu64) }
        0x1027907c => { vm.decode(0x3353862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10279080u64) }
        0x10279080 => { vm.decode(0x94c2891u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 436i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10279084u64) }
        0x10279084 => { vm.decode(0x3353c62u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1436i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10279088u64) }
        0x10279088 => { vm.decode(0x2a24e02fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027908cu64) }
        0x1027908c => { vm.decode(0x39009cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279090u64) }
        0x10279090 => { vm.decode(0x2020602fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279094u64) }
        0x10279094 => { vm.decode(0x39089cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279098u64) }
        0x10279098 => { vm.decode(0x2820c06fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027909cu64) }
        0x1027909c => { vm.decode(0x39109cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102790a0u64) }
        0x102790a0 => { vm.decode(0x20a440efu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102790a4u64) }
        0x102790a4 => { vm.decode(0x39019c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102790a8u64) }
        0x102790a8 => { vm.decode(0x82e3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102790acu64) }
        0x102790ac => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102790b0u64) }
        0x102790b0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102790b4u64 + 4);
        vm.wr(37, 0x102790b4u64 + 4);
        Ok(0x102790b4u64) }
        0x102790b4 => { vm.decode(0x622b8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3456i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102790b8u64) }
        0x102790b8 => { vm.decode(0x28c0f32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102790bcu64) }
        0x102790bc => { vm.decode(0x39189cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102790c0u64) }
        0x102790c0 => { vm.decode(0x9c8a011u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102790c4u64) }
        0x102790c4 => { vm.decode(0x43251862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102790c8u64) }
        0x102790c8 => { vm.decode(0x2824c06fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102790ccu64) }
        0x102790cc => { vm.decode(0x39289c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102790d0u64) }
        0x102790d0 => { vm.decode(0x2a24e06fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102790d4u64) }
        0x102790d4 => { vm.decode(0x39309c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102790d8u64) }
        0x102790d8 => { vm.decode(0x222040afu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102790dcu64) }
        0x102790dc => { vm.decode(0x39389c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102790e0u64) }
        0x102790e0 => { vm.decode(0x202460afu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102790e4u64) }
        0x102790e4 => { vm.decode(0x39209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102790e8u64) }
        0x102790e8 => { vm.decode(0x202040efu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102790ecu64) }
        0x102790ec => { vm.decode(0x39289cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102790f0u64) }
        0x102790f0 => { vm.decode(0x222060efu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102790f4u64) }
        0x102790f4 => { vm.decode(0x39309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102790f8u64) }
        0x102790f8 => { vm.decode(0x222440efu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102790fcu64) }
        0x102790fc => { vm.decode(0x39389cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279100u64) }
        0x10279100 => { vm.decode(0x282060afu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279104u64) }
        0x10279104 => { vm.decode(0x3b001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279108u64) }
        0x10279108 => { vm.decode(0x2a2040afu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027910cu64) }
        0x1027910c => { vm.decode(0x3b081430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279110u64) }
        0x10279110 => { vm.decode(0x282460afu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279114u64) }
        0x10279114 => { vm.decode(0x414c0291u32); let d = 2i16; let s = 18i16; let s2 = -1i16; let imm = 676i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10279118u64) }
        0x10279118 => { vm.decode(0x2b101530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027911cu64) }
        0x1027911c => { vm.decode(0x3b181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279120u64) }
        0x10279120 => { vm.decode(0x2a2440afu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279124u64) }
        0x10279124 => { vm.decode(0x3b0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279128u64) }
        0x10279128 => { vm.decode(0xab00ddf0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 1952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027912cu64) }
        0x1027912c => { vm.decode(0x282060efu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279130u64) }
        0x10279130 => { vm.decode(0x3b109cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279134u64) }
        0x10279134 => { vm.decode(0x20a460efu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279138u64) }
        0x10279138 => { vm.decode(0x8b019cb0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027913cu64) }
        0x1027913c => { vm.decode(0x1b199c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279140u64) }
        0x10279140 => { vm.decode(0x664f8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2968i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279144u64) }
        0x10279144 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279148u64) }
        0x10279148 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027914cu64 + 4);
        vm.wr(37, 0x1027914cu64 + 4);
        Ok(0x1027914cu64) }
        0x1027914c => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279150u64) }
        0x10279150 => { vm.decode(0x2084e3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279154u64) }
        0x10279154 => { vm.decode(0x40408119u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 80i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279158u64) }
        0x10279158 => { vm.decode(0xac402fu32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027915cu64) }
        0x1027915c => { vm.decode(0x43106e73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2964i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10279160u64) }
        0x10279160 => { vm.decode(0x4f1ce211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10279164u64) }
        0x10279164 => { vm.decode(0xb2d7062u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10279168u64) }
        0x10279168 => { vm.decode(0x1b099c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027916cu64) }
        0x1027916c => { vm.decode(0xb019d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279170u64) }
        0x10279170 => { vm.decode(0x88663efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279174u64) }
        0x10279174 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279178u64) }
        0x10279178 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027917cu64 + 4);
        vm.wr(37, 0x1027917cu64 + 4);
        Ok(0x1027917cu64) }
        0x1027917c => { vm.decode(0x660b8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2944i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279180u64) }
        0x10279180 => { vm.decode(0x22a4c02fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279184u64) }
        0x10279184 => { vm.decode(0xa8c736fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279188u64) }
        0x10279188 => { vm.decode(0xb311db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027918cu64) }
        0x1027918c => { vm.decode(0x8b291cb0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279190u64) }
        0x10279190 => { vm.decode(0x1b211cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279194u64) }
        0x10279194 => { vm.decode(0x86532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279198u64) }
        0x10279198 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027919cu64) }
        0x1027919c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102791a0u64 + 4);
        vm.wr(37, 0x102791a0u64 + 4);
        Ok(0x102791a0u64) }
        0x102791a0 => { vm.decode(0x66198559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2912i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102791a4u64) }
        0x102791a4 => { vm.decode(0x2280532fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102791a8u64) }
        0x102791a8 => { vm.decode(0x3b289430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102791acu64) }
        0x102791ac => { vm.decode(0xd98ca51u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102791b0u64) }
        0x102791b0 => { vm.decode(0x4088532fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102791b4u64) }
        0x102791b4 => { vm.decode(0x82002031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102791b8u64).wrapping_add((imm * 4) as u64) } else { 0x102791b8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102791b8u64) }
        0x102791b8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102791bcu64) } }
        0x102791bc => { vm.decode(0x20a0c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102791c0u64) }
        0x102791c0 => { vm.decode(0xa8853efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102791c4u64) }
        0x102791c4 => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102791c8u64) }
        0x102791c8 => { vm.decode(0xb199db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102791ccu64) }
        0x102791cc => { vm.decode(0x1b119cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102791d0u64) }
        0x102791d0 => { vm.decode(0x88643efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102791d4u64) }
        0x102791d4 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102791d8u64) }
        0x102791d8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102791dcu64 + 4);
        vm.wr(37, 0x102791dcu64 + 4);
        Ok(0x102791dcu64) }
        0x102791dc => { vm.decode(0x665b8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2992i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102791e0u64) }
        0x102791e0 => { vm.decode(0x20a4c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102791e4u64) }
        0x102791e4 => { vm.decode(0x8c73efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102791e8u64) }
        0x102791e8 => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102791ecu64) }
        0x102791ec => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102791f0u64) }
        0x102791f0 => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102791f4u64) }
        0x102791f4 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102791f8u64) }
        0x102791f8 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102791fcu64) }
        0x102791fc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279200u64 + 4);
        vm.wr(37, 0x10279200u64 + 4);
        Ok(0x10279200u64) }
        0x10279200 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279204u64) }
        0x10279204 => { vm.decode(0x28c53efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279208u64) }
        0x10279208 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027920cu64).wrapping_add((imm * 4) as u64) } else { 0x1027920cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027920cu64) }
        0x1027920c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10279210u64) } }
        0x10279210 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10279214u64) }
        0x10279214 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10279218u64) }
        0x10279218 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027921cu64) }
        0x1027921c => { vm.decode(0x28c53efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279220u64) }
        0x10279220 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10279224u64) }
        0x10279224 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x10279228u64).wrapping_add((imm * 4) as u64) } else { 0x10279228u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10279228u64) }
        0x10279228 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027922cu64) } }
        0x1027922c => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10279230u64).wrapping_add((imm * 4) as u64) } else { 0x10279230u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10279230u64) }
        0x10279230 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10279234u64) } }
        0x10279234 => { vm.decode(0x22a0c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279238u64) }
        0x10279238 => { vm.decode(0x2b311530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027923cu64) }
        0x1027923c => { vm.decode(0x3b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279240u64) }
        0x10279240 => { vm.decode(0xa82c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279244u64) }
        0x10279244 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279248u64) }
        0x10279248 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027924cu64 + 4);
        vm.wr(37, 0x1027924cu64 + 4);
        Ok(0x1027924cu64) }
        0x1027924c => { vm.decode(0x662d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3656i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279250u64) }
        0x10279250 => { vm.decode(0x2b3014b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279254u64) }
        0x10279254 => { vm.decode(0x8c73efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279258u64) }
        0x10279258 => { vm.decode(0x8000a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027925cu64).wrapping_add((imm * 4) as u64) } else { 0x1027925cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027925cu64) }
        0x1027925c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10279260u64) } }
        0x10279260 => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279264u64) }
        0x10279264 => { vm.decode(0x2000212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279268u64) }
        0x10279268 => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027926cu64) }
        0x1027926c => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279270u64) }
        0x10279270 => { vm.decode(0x286532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279274u64) }
        0x10279274 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279278u64) }
        0x10279278 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027927cu64 + 4);
        vm.wr(37, 0x1027927cu64 + 4);
        Ok(0x1027927cu64) }
        0x1027927c => { vm.decode(0x62198219u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279280u64) }
        0x10279280 => { vm.decode(0x2b2814b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279284u64) }
        0x10279284 => { vm.decode(0x20a4e06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279288u64) }
        0x10279288 => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027928cu64) }
        0x1027928c => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279290u64) }
        0x10279290 => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279294u64) }
        0x10279294 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279298u64) }
        0x10279298 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027929cu64 + 4);
        vm.wr(37, 0x1027929cu64 + 4);
        Ok(0x1027929cu64) }
        0x1027929c => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102792a0u64) }
        0x102792a0 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102792a4u64) }
        0x102792a4 => { vm.decode(0x20a4c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102792a8u64) }
        0x102792a8 => { vm.decode(0x88853afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102792acu64) }
        0x102792ac => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102792b0u64) }
        0x102792b0 => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102792b4u64) }
        0x102792b4 => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102792b8u64) }
        0x102792b8 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102792bcu64) }
        0x102792bc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102792c0u64 + 4);
        vm.wr(37, 0x102792c0u64 + 4);
        Ok(0x102792c0u64) }
        0x102792c0 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102792c4u64) }
        0x102792c4 => { vm.decode(0x88873afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102792c8u64) }
        0x102792c8 => { vm.decode(0xc000027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 32i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102792ccu64).wrapping_add((imm * 4) as u64) } else { 0x102792ccu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102792ccu64) }
        0x102792cc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102792d0u64) } }
        0x102792d0 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102792d4u64) }
        0x102792d4 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102792d8u64) }
        0x102792d8 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102792dcu64) }
        0x102792dc => { vm.decode(0x88873afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102792e0u64) }
        0x102792e0 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102792e4u64) }
        0x102792e4 => { vm.decode(0x10300084u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 25i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x102792e8u64).wrapping_add((imm * 4) as u64) } else { 0x102792e8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102792e8u64) }
        0x102792e8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102792ecu64) } }
        0x102792ec => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102792f0u64).wrapping_add((imm * 4) as u64) } else { 0x102792f0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102792f0u64) }
        0x102792f0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102792f4u64) } }
        0x102792f4 => { vm.decode(0x22a0c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102792f8u64) }
        0x102792f8 => { vm.decode(0x2b311530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102792fcu64) }
        0x102792fc => { vm.decode(0x3b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279300u64) }
        0x10279300 => { vm.decode(0xa82c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279304u64) }
        0x10279304 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279308u64) }
        0x10279308 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027930cu64 + 4);
        vm.wr(37, 0x1027930cu64 + 4);
        Ok(0x1027930cu64) }
        0x1027930c => { vm.decode(0x662d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3656i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279310u64) }
        0x10279310 => { vm.decode(0x2b089430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279314u64) }
        0x10279314 => { vm.decode(0x88853afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279318u64) }
        0x10279318 => { vm.decode(0x8800227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 12i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027931cu64).wrapping_add((imm * 4) as u64) } else { 0x1027931cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027931cu64) }
        0x1027931c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10279320u64) } }
        0x10279320 => { vm.decode(0x2084732fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279324u64) }
        0x10279324 => { vm.decode(0x2088432fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279328u64) }
        0x10279328 => { vm.decode(0xa20204f5u32); let d = 4i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027932cu64) }
        0x1027932c => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279330u64) }
        0x10279330 => { vm.decode(0x2000212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279334u64) }
        0x10279334 => { vm.decode(0xb391d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279338u64) }
        0x10279338 => { vm.decode(0x1b311c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027933cu64) }
        0x1027933c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279340u64) }
        0x10279340 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279344u64 + 4);
        vm.wr(37, 0x10279344u64 + 4);
        Ok(0x10279344u64) }
        0x10279344 => { vm.decode(0x66498559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2896i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279348u64) }
        0x10279348 => { vm.decode(0x2b009430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027934cu64) }
        0x1027934c => { vm.decode(0x20a4e06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279350u64) }
        0x10279350 => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279354u64) }
        0x10279354 => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279358u64) }
        0x10279358 => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027935cu64) }
        0x1027935c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279360u64) }
        0x10279360 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279364u64 + 4);
        vm.wr(37, 0x10279364u64 + 4);
        Ok(0x10279364u64) }
        0x10279364 => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279368u64) }
        0x10279368 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027936cu64) }
        0x1027936c => { vm.decode(0x20a4c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279370u64) }
        0x10279370 => { vm.decode(0xa8873afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279374u64) }
        0x10279374 => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279378u64) }
        0x10279378 => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027937cu64) }
        0x1027937c => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279380u64) }
        0x10279380 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279384u64) }
        0x10279384 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279388u64 + 4);
        vm.wr(37, 0x10279388u64 + 4);
        Ok(0x10279388u64) }
        0x10279388 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027938cu64) }
        0x1027938c => { vm.decode(0x88c53afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279390u64) }
        0x10279390 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10279394u64).wrapping_add((imm * 4) as u64) } else { 0x10279394u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10279394u64) }
        0x10279394 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10279398u64) } }
        0x10279398 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027939cu64) }
        0x1027939c => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102793a0u64) }
        0x102793a0 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102793a4u64) }
        0x102793a4 => { vm.decode(0x88c53afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102793a8u64) }
        0x102793a8 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102793acu64) }
        0x102793ac => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x102793b0u64).wrapping_add((imm * 4) as u64) } else { 0x102793b0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102793b0u64) }
        0x102793b0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102793b4u64) } }
        0x102793b4 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102793b8u64).wrapping_add((imm * 4) as u64) } else { 0x102793b8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102793b8u64) }
        0x102793b8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102793bcu64) } }
        0x102793bc => { vm.decode(0x22a0c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102793c0u64) }
        0x102793c0 => { vm.decode(0x2b311530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102793c4u64) }
        0x102793c4 => { vm.decode(0x3b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102793c8u64) }
        0x102793c8 => { vm.decode(0xa82c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102793ccu64) }
        0x102793cc => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102793d0u64) }
        0x102793d0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102793d4u64 + 4);
        vm.wr(37, 0x102793d4u64 + 4);
        Ok(0x102793d4u64) }
        0x102793d4 => { vm.decode(0x662d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3656i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102793d8u64) }
        0x102793d8 => { vm.decode(0x2b0094b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102793dcu64) }
        0x102793dc => { vm.decode(0xa8873afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102793e0u64) }
        0x102793e0 => { vm.decode(0x8000a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102793e4u64).wrapping_add((imm * 4) as u64) } else { 0x102793e4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102793e4u64) }
        0x102793e4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102793e8u64) } }
        0x102793e8 => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102793ecu64) }
        0x102793ec => { vm.decode(0x2000212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102793f0u64) }
        0x102793f0 => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102793f4u64) }
        0x102793f4 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102793f8u64) }
        0x102793f8 => { vm.decode(0x286532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102793fcu64) }
        0x102793fc => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279400u64) }
        0x10279400 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279404u64 + 4);
        vm.wr(37, 0x10279404u64 + 4);
        Ok(0x10279404u64) }
        0x10279404 => { vm.decode(0x62198219u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279408u64) }
        0x10279408 => { vm.decode(0x2b189430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027940cu64) }
        0x1027940c => { vm.decode(0x20a4e06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279410u64) }
        0x10279410 => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279414u64) }
        0x10279414 => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279418u64) }
        0x10279418 => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027941cu64) }
        0x1027941c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279420u64) }
        0x10279420 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279424u64 + 4);
        vm.wr(37, 0x10279424u64 + 4);
        Ok(0x10279424u64) }
        0x10279424 => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279428u64) }
        0x10279428 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027942cu64) }
        0x1027942c => { vm.decode(0x20a4c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279430u64) }
        0x10279430 => { vm.decode(0xa8c53afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279434u64) }
        0x10279434 => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279438u64) }
        0x10279438 => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027943cu64) }
        0x1027943c => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279440u64) }
        0x10279440 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279444u64) }
        0x10279444 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279448u64 + 4);
        vm.wr(37, 0x10279448u64 + 4);
        Ok(0x10279448u64) }
        0x10279448 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027944cu64) }
        0x1027944c => { vm.decode(0xa8c73afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279450u64) }
        0x10279450 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10279454u64).wrapping_add((imm * 4) as u64) } else { 0x10279454u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10279454u64) }
        0x10279454 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10279458u64) } }
        0x10279458 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027945cu64) }
        0x1027945c => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10279460u64) }
        0x10279460 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10279464u64) }
        0x10279464 => { vm.decode(0xa8c73afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279468u64) }
        0x10279468 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027946cu64) }
        0x1027946c => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x10279470u64).wrapping_add((imm * 4) as u64) } else { 0x10279470u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10279470u64) }
        0x10279470 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10279474u64) } }
        0x10279474 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10279478u64).wrapping_add((imm * 4) as u64) } else { 0x10279478u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10279478u64) }
        0x102794cc => { vm.decode(0x20a4e06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102794d0u64) }
        0x102794d0 => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102794d4u64) }
        0x102794d4 => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102794d8u64) }
        0x102794d8 => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102794dcu64) }
        0x102794dc => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102794e0u64) }
        0x102794e0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102794e4u64 + 4);
        vm.wr(37, 0x102794e4u64 + 4);
        Ok(0x102794e4u64) }
        0x102794e4 => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102794e8u64) }
        0x102794e8 => { vm.decode(0x70198219u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102794ecu64) }
        0x102794ec => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102794f0u64) }
        0x102794f0 => { vm.decode(0x8cc3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102794f4u64) }
        0x102794f4 => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102794f8u64) }
        0x102794f8 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102794fcu64) }
        0x102794fc => { vm.decode(0x628c532fu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279500u64) }
        0x10279500 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279504u64) }
        0x10279504 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279508u64) }
        0x10279508 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027950cu64 + 4);
        vm.wr(37, 0x1027950cu64 + 4);
        Ok(0x1027950cu64) }
        0x1027950c => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279510u64) }
        0x10279510 => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279514u64) }
        0x10279514 => { vm.decode(0xa8ce3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279518u64) }
        0x10279518 => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027951cu64) }
        0x1027951c => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279520u64) }
        0x10279520 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279524u64) }
        0x10279524 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279528u64) }
        0x10279528 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027952cu64 + 4);
        vm.wr(37, 0x1027952cu64 + 4);
        Ok(0x1027952cu64) }
        0x1027952c => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279530u64) }
        0x10279530 => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279534u64) }
        0x10279534 => { vm.decode(0x88e3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279538u64) }
        0x10279538 => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027953cu64) }
        0x1027953c => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279540u64) }
        0x10279540 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279544u64) }
        0x10279544 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279548u64) }
        0x10279548 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027954cu64 + 4);
        vm.wr(37, 0x1027954cu64 + 4);
        Ok(0x1027954cu64) }
        0x1027954c => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279550u64) }
        0x10279550 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279554u64) }
        0x10279554 => { vm.decode(0x20a4c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279558u64) }
        0x10279558 => { vm.decode(0xcc632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027955cu64) }
        0x1027955c => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279560u64) }
        0x10279560 => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279564u64) }
        0x10279564 => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279568u64) }
        0x10279568 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027956cu64) }
        0x1027956c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279570u64 + 4);
        vm.wr(37, 0x10279570u64 + 4);
        Ok(0x10279570u64) }
        0x10279570 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279574u64) }
        0x10279574 => { vm.decode(0x2cc432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279578u64) }
        0x10279578 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027957cu64).wrapping_add((imm * 4) as u64) } else { 0x1027957cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027957cu64) }
        0x1027957c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10279580u64) } }
        0x10279580 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10279584u64) }
        0x10279584 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10279588u64) }
        0x10279588 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027958cu64) }
        0x1027958c => { vm.decode(0x2cc432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279590u64) }
        0x10279590 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10279594u64) }
        0x10279594 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x10279598u64).wrapping_add((imm * 4) as u64) } else { 0x10279598u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10279598u64) }
        0x10279598 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027959cu64) } }
        0x1027959c => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102795a0u64).wrapping_add((imm * 4) as u64) } else { 0x102795a0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102795a0u64) }
        0x102795a0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102795a4u64) } }
        0x102795a4 => { vm.decode(0x22a0c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102795a8u64) }
        0x102795a8 => { vm.decode(0x2b311530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102795acu64) }
        0x102795ac => { vm.decode(0x3b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102795b0u64) }
        0x102795b0 => { vm.decode(0xa82c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102795b4u64) }
        0x102795b4 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102795b8u64) }
        0x102795b8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102795bcu64 + 4);
        vm.wr(37, 0x102795bcu64 + 4);
        Ok(0x102795bcu64) }
        0x102795bc => { vm.decode(0x662d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3656i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102795c0u64) }
        0x102795c0 => { vm.decode(0x91114b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102795c4u64) }
        0x102795c4 => { vm.decode(0xcc632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102795c8u64) }
        0x102795c8 => { vm.decode(0x8000a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102795ccu64).wrapping_add((imm * 4) as u64) } else { 0x102795ccu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102795ccu64) }
        0x102795cc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102795d0u64) } }
        0x102795d0 => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102795d4u64) }
        0x102795d4 => { vm.decode(0x2000212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102795d8u64) }
        0x102795d8 => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102795dcu64) }
        0x102795dc => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102795e0u64) }
        0x102795e0 => { vm.decode(0x286532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102795e4u64) }
        0x102795e4 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102795e8u64) }
        0x102795e8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102795ecu64 + 4);
        vm.wr(37, 0x102795ecu64 + 4);
        Ok(0x102795ecu64) }
        0x102795ec => { vm.decode(0x62198219u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102795f0u64) }
        0x102795f0 => { vm.decode(0x90914b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102795f4u64) }
        0x102795f4 => { vm.decode(0x20a4e06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102795f8u64) }
        0x102795f8 => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102795fcu64) }
        0x102795fc => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279600u64) }
        0x10279600 => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279604u64) }
        0x10279604 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279608u64) }
        0x10279608 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027960cu64 + 4);
        vm.wr(37, 0x1027960cu64 + 4);
        Ok(0x1027960cu64) }
        0x1027960c => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279610u64) }
        0x10279610 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279614u64) }
        0x10279614 => { vm.decode(0x20a4c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279618u64) }
        0x10279618 => { vm.decode(0x2cc632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027961cu64) }
        0x1027961c => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279620u64) }
        0x10279620 => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279624u64) }
        0x10279624 => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279628u64) }
        0x10279628 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027962cu64) }
        0x1027962c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279630u64 + 4);
        vm.wr(37, 0x10279630u64 + 4);
        Ok(0x10279630u64) }
        0x10279630 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279634u64) }
        0x10279634 => { vm.decode(0xc8436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279638u64) }
        0x10279638 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027963cu64).wrapping_add((imm * 4) as u64) } else { 0x1027963cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027963cu64) }
        0x1027963c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10279640u64) } }
        0x10279640 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10279644u64) }
        0x10279644 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10279648u64) }
        0x10279648 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027964cu64) }
        0x1027964c => { vm.decode(0xc8436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279650u64) }
        0x10279650 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10279654u64) }
        0x10279654 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x10279658u64).wrapping_add((imm * 4) as u64) } else { 0x10279658u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10279658u64) }
        0x10279658 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027965cu64) } }
        0x1027965c => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10279660u64).wrapping_add((imm * 4) as u64) } else { 0x10279660u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10279660u64) }
        0x102796b4 => { vm.decode(0x20a4e06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102796b8u64) }
        0x102796b8 => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102796bcu64) }
        0x102796bc => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102796c0u64) }
        0x102796c0 => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102796c4u64) }
        0x102796c4 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102796c8u64) }
        0x102796c8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102796ccu64 + 4);
        vm.wr(37, 0x102796ccu64 + 4);
        Ok(0x102796ccu64) }
        0x102796cc => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102796d0u64) }
        0x102796d0 => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102796d4u64) }
        0x102796d4 => { vm.decode(0x28cc3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102796d8u64) }
        0x102796d8 => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102796dcu64) }
        0x102796dc => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102796e0u64) }
        0x102796e0 => { vm.decode(0x286532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102796e4u64) }
        0x102796e4 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102796e8u64) }
        0x102796e8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102796ecu64 + 4);
        vm.wr(37, 0x102796ecu64 + 4);
        Ok(0x102796ecu64) }
        0x102796ec => { vm.decode(0x62198219u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102796f0u64) }
        0x102796f0 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102796f4u64) }
        0x102796f4 => { vm.decode(0x20a4c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102796f8u64) }
        0x102796f8 => { vm.decode(0xac8432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102796fcu64) }
        0x102796fc => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279700u64) }
        0x10279700 => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279704u64) }
        0x10279704 => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279708u64) }
        0x10279708 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027970cu64) }
        0x1027970c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279710u64 + 4);
        vm.wr(37, 0x10279710u64 + 4);
        Ok(0x10279710u64) }
        0x10279710 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279714u64) }
        0x10279714 => { vm.decode(0xac8632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279718u64) }
        0x10279718 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027971cu64).wrapping_add((imm * 4) as u64) } else { 0x1027971cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027971cu64) }
        0x1027971c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10279720u64) } }
        0x10279720 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10279724u64) }
        0x10279724 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10279728u64) }
        0x10279728 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027972cu64) }
        0x1027972c => { vm.decode(0xac8632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279730u64) }
        0x10279730 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10279734u64) }
        0x10279734 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x10279738u64).wrapping_add((imm * 4) as u64) } else { 0x10279738u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10279738u64) }
        0x10279738 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027973cu64) } }
        0x1027973c => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10279740u64).wrapping_add((imm * 4) as u64) } else { 0x10279740u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10279740u64) }
        0x10279794 => { vm.decode(0x20a4e06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279798u64) }
        0x10279798 => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027979cu64) }
        0x1027979c => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102797a0u64) }
        0x102797a0 => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102797a4u64) }
        0x102797a4 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102797a8u64) }
        0x102797a8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102797acu64 + 4);
        vm.wr(37, 0x102797acu64 + 4);
        Ok(0x102797acu64) }
        0x102797ac => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102797b0u64) }
        0x102797b0 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102797b4u64) }
        0x102797b4 => { vm.decode(0x20a4c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102797b8u64) }
        0x102797b8 => { vm.decode(0x8cc432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102797bcu64) }
        0x102797bc => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102797c0u64) }
        0x102797c0 => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102797c4u64) }
        0x102797c4 => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102797c8u64) }
        0x102797c8 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102797ccu64) }
        0x102797cc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102797d0u64 + 4);
        vm.wr(37, 0x102797d0u64 + 4);
        Ok(0x102797d0u64) }
        0x102797d0 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102797d4u64) }
        0x102797d4 => { vm.decode(0x8cc632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102797d8u64) }
        0x102797d8 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102797dcu64).wrapping_add((imm * 4) as u64) } else { 0x102797dcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102797dcu64) }
        0x102797dc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102797e0u64) } }
        0x102797e0 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102797e4u64) }
        0x102797e4 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102797e8u64) }
        0x102797e8 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102797ecu64) }
        0x102797ec => { vm.decode(0x8cc632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102797f0u64) }
        0x102797f0 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102797f4u64) }
        0x102797f4 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x102797f8u64).wrapping_add((imm * 4) as u64) } else { 0x102797f8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102797f8u64) }
        0x102797f8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102797fcu64) } }
        0x102797fc => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10279800u64).wrapping_add((imm * 4) as u64) } else { 0x10279800u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10279800u64) }
        0x10279854 => { vm.decode(0x20a4e06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279858u64) }
        0x10279858 => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027985cu64) }
        0x1027985c => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279860u64) }
        0x10279860 => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279864u64) }
        0x10279864 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279868u64) }
        0x10279868 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027986cu64 + 4);
        vm.wr(37, 0x1027986cu64 + 4);
        Ok(0x1027986cu64) }
        0x1027986c => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279870u64) }
        0x10279870 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279874u64) }
        0x10279874 => { vm.decode(0x20a4c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279878u64) }
        0x10279878 => { vm.decode(0xacc432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027987cu64) }
        0x1027987c => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279880u64) }
        0x10279880 => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279884u64) }
        0x10279884 => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279888u64) }
        0x10279888 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027988cu64) }
        0x1027988c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279890u64 + 4);
        vm.wr(37, 0x10279890u64 + 4);
        Ok(0x10279890u64) }
        0x10279890 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279894u64) }
        0x10279894 => { vm.decode(0xacc632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279898u64) }
        0x10279898 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027989cu64).wrapping_add((imm * 4) as u64) } else { 0x1027989cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027989cu64) }
        0x1027989c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102798a0u64) } }
        0x102798a0 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102798a4u64) }
        0x102798a4 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102798a8u64) }
        0x102798a8 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102798acu64) }
        0x102798ac => { vm.decode(0xacc632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102798b0u64) }
        0x102798b0 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102798b4u64) }
        0x102798b4 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x102798b8u64).wrapping_add((imm * 4) as u64) } else { 0x102798b8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102798b8u64) }
        0x102798b8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102798bcu64) } }
        0x102798bc => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102798c0u64).wrapping_add((imm * 4) as u64) } else { 0x102798c0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102798c0u64) }
        0x10279914 => { vm.decode(0x2084732fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279918u64) }
        0x10279918 => { vm.decode(0x888e36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027991cu64) }
        0x1027991c => { vm.decode(0x820604f5u32); let d = 20i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10279920u64) }
        0x10279920 => { vm.decode(0xa88c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279924u64) }
        0x10279924 => { vm.decode(0x824604f5u32); let d = 21i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10279928u64) }
        0x10279928 => { vm.decode(0x810d875u32); let d = 17i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027992cu64) }
        0x1027992c => { vm.decode(0x88cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279930u64) }
        0x10279930 => { vm.decode(0x8a0604f5u32); let d = 22i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10279934u64) }
        0x10279934 => { vm.decode(0x2004e16fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279938u64) }
        0x10279938 => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027993cu64) }
        0x1027993c => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279940u64) }
        0x10279940 => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279944u64) }
        0x10279944 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279948u64) }
        0x10279948 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027994cu64 + 4);
        vm.wr(37, 0x1027994cu64 + 4);
        Ok(0x1027994cu64) }
        0x1027994c => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279950u64) }
        0x10279950 => { vm.decode(0x2004c16fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279954u64) }
        0x10279954 => { vm.decode(0xac8436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279958u64) }
        0x10279958 => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027995cu64) }
        0x1027995c => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279960u64) }
        0x10279960 => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279964u64) }
        0x10279964 => { vm.decode(0x82532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279968u64) }
        0x10279968 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027996cu64) }
        0x1027996c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279970u64 + 4);
        vm.wr(37, 0x10279970u64 + 4);
        Ok(0x10279970u64) }
        0x10279970 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279974u64) }
        0x10279974 => { vm.decode(0xac8636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279978u64) }
        0x10279978 => { vm.decode(0x8888a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 31i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027997cu64).wrapping_add((imm * 4) as u64) } else { 0x1027997cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027997cu64) }
        0x1027997c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10279980u64) } }
        0x10279980 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10279984u64) }
        0x10279984 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10279988u64) }
        0x10279988 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027998cu64) }
        0x1027998c => { vm.decode(0xac8636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279990u64) }
        0x10279990 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10279994u64) }
        0x10279994 => { vm.decode(0x10200084u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x10279998u64).wrapping_add((imm * 4) as u64) } else { 0x10279998u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10279998u64) }
        0x10279998 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027999cu64) } }
        0x1027999c => { vm.decode(0x8008227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 9i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102799a0u64).wrapping_add((imm * 4) as u64) } else { 0x102799a0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102799a0u64) }
        0x102799f8 => { vm.decode(0x8c732fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102799fcu64) }
        0x102799fc => { vm.decode(0x2a80e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279a00u64) }
        0x10279a00 => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10279a04u64) }
        0x10279a04 => { vm.decode(0x1b109cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279a08u64) }
        0x10279a08 => { vm.decode(0x2884e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279a0cu64) }
        0x10279a0c => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10279a10u64) }
        0x10279a10 => { vm.decode(0x1b081c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279a14u64) }
        0x10279a14 => { vm.decode(0x2a84c36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279a18u64) }
        0x10279a18 => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10279a1cu64) }
        0x10279a1c => { vm.decode(0x39101430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279a20u64) }
        0x10279a20 => { vm.decode(0x2a84e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279a24u64) }
        0x10279a24 => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10279a28u64) }
        0x10279a28 => { vm.decode(0x1b101c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279a2cu64) }
        0x10279a2c => { vm.decode(0x4088536fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279a30u64) }
        0x10279a30 => { vm.decode(0x2024e06fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279a34u64) }
        0x10279a34 => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279a38u64) }
        0x10279a38 => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279a3cu64) }
        0x10279a3c => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279a40u64) }
        0x10279a40 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279a44u64) }
        0x10279a44 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279a48u64 + 4);
        vm.wr(37, 0x10279a48u64 + 4);
        Ok(0x10279a48u64) }
        0x10279a48 => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279a4cu64) }
        0x10279a4c => { vm.decode(0x2820402fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279a50u64) }
        0x10279a50 => { vm.decode(0x28ce3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279a54u64) }
        0x10279a54 => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279a58u64) }
        0x10279a58 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279a5cu64) }
        0x10279a5c => { vm.decode(0x286532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279a60u64) }
        0x10279a60 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279a64u64) }
        0x10279a64 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279a68u64 + 4);
        vm.wr(37, 0x10279a68u64 + 4);
        Ok(0x10279a68u64) }
        0x10279a68 => { vm.decode(0x62198219u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279a6cu64) }
        0x10279a6c => { vm.decode(0x70099219u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 4096i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279a70u64) }
        0x10279a70 => { vm.decode(0x1004a099u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 13i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279a74u64) }
        0x10279a74 => { vm.decode(0x222460efu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279a78u64) }
        0x10279a78 => { vm.decode(0x9d181430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 4120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279a7cu64) }
        0x10279a7c => { vm.decode(0x8d105530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 4112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279a80u64) }
        0x10279a80 => { vm.decode(0x48901875u32); let d = 22i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279a84u64) }
        0x10279a84 => { vm.decode(0x888c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279a88u64) }
        0x10279a88 => { vm.decode(0xd081530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279a8cu64) }
        0x10279a8c => { vm.decode(0x1d001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279a90u64) }
        0x10279a90 => { vm.decode(0x4888e3efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279a94u64) }
        0x10279a94 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279a98u64) }
        0x10279a98 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279a9cu64) }
        0x10279a9c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279aa0u64 + 4);
        vm.wr(37, 0x10279aa0u64 + 4);
        Ok(0x10279aa0u64) }
        0x10279aa0 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279aa4u64) }
        0x10279aa4 => { vm.decode(0x600b8519u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2240i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279aa8u64) }
        0x10279aa8 => { vm.decode(0x2ac60efu32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279aacu64) }
        0x10279aac => { vm.decode(0x9d181430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 4120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279ab0u64) }
        0x10279ab0 => { vm.decode(0x9d105430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 4112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279ab4u64) }
        0x10279ab4 => { vm.decode(0x1d081430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279ab8u64) }
        0x10279ab8 => { vm.decode(0xd001530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279abcu64) }
        0x10279abc => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279ac0u64) }
        0x10279ac0 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279ac4u64) }
        0x10279ac4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279ac8u64 + 4);
        vm.wr(37, 0x10279ac8u64 + 4);
        Ok(0x10279ac8u64) }
        0x10279ac8 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279accu64) }
        0x10279acc => { vm.decode(0x725b8419u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2224i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279ad0u64) }
        0x10279ad0 => { vm.decode(0x48099u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 9i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279ad4u64) }
        0x10279ad4 => { vm.decode(0x2ac60efu32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279ad8u64) }
        0x10279ad8 => { vm.decode(0x1d181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279adcu64) }
        0x10279adc => { vm.decode(0x8d105430u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 4112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279ae0u64) }
        0x10279ae0 => { vm.decode(0x9d085430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 4104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279ae4u64) }
        0x10279ae4 => { vm.decode(0xd001530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279ae8u64) }
        0x10279ae8 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279aecu64) }
        0x10279aec => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279af0u64) }
        0x10279af0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279af4u64 + 4);
        vm.wr(37, 0x10279af4u64 + 4);
        Ok(0x10279af4u64) }
        0x10279af4 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279af8u64) }
        0x10279af8 => { vm.decode(0x2aa0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279afcu64) }
        0x10279afc => { vm.decode(0x3b1914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279b00u64) }
        0x10279b00 => { vm.decode(0xa82e3efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279b04u64) }
        0x10279b04 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279b08u64) }
        0x10279b08 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279b0cu64 + 4);
        vm.wr(37, 0x10279b0cu64 + 4);
        Ok(0x10279b0cu64) }
        0x10279b0c => { vm.decode(0x667d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3640i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279b10u64) }
        0x10279b10 => { vm.decode(0xf904251u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10279b14u64) }
        0x10279b14 => { vm.decode(0x23051822u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10279b18u64) }
        0x10279b18 => { vm.decode(0x920d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10279b1cu64) }
        0x10279b1c => { vm.decode(0x2424a905u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u16;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10279b20u64) }
        0x10279b20 => { vm.decode(0x92994b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279b24u64) }
        0x10279b24 => { vm.decode(0x92194b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279b28u64) }
        0x10279b28 => { vm.decode(0x20a4c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279b2cu64) }
        0x10279b2c => { vm.decode(0x6084e3efu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279b30u64) }
        0x10279b30 => { vm.decode(0x80c412fu32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279b34u64) }
        0x10279b34 => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279b38u64) }
        0x10279b38 => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279b3cu64) }
        0x10279b3c => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279b40u64) }
        0x10279b40 => { vm.decode(0x82532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279b44u64) }
        0x10279b44 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279b48u64) }
        0x10279b48 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279b4cu64 + 4);
        vm.wr(37, 0x10279b4cu64 + 4);
        Ok(0x10279b4cu64) }
        0x10279b4c => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279b50u64) }
        0x10279b50 => { vm.decode(0x2804412fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279b54u64) }
        0x10279b54 => { vm.decode(0x192194b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279b58u64) }
        0x10279b58 => { vm.decode(0x80c612fu32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279b5cu64) }
        0x10279b5c => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10279b60u64).wrapping_add((imm * 4) as u64) } else { 0x10279b60u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10279b60u64) }
        0x10279b60 => { vm.decode(0x92995b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279b64u64) }
        0x10279b64 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10279b68u64) }
        0x10279b68 => { vm.decode(0x8003036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10279b6cu64) }
        0x10279b6c => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10279b70u64) }
        0x10279b70 => { vm.decode(0x20a4e06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279b74u64) }
        0x10279b74 => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279b78u64) }
        0x10279b78 => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279b7cu64) }
        0x10279b7c => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279b80u64) }
        0x10279b80 => { vm.decode(0x4082736fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279b84u64) }
        0x10279b84 => { vm.decode(0xa10c875u32); let d = 25i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279b88u64) }
        0x10279b88 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279b8cu64 + 4);
        vm.wr(37, 0x10279b8cu64 + 4);
        Ok(0x10279b8cu64) }
        0x10279b8c => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279b90u64) }
        0x10279b90 => { vm.decode(0x68cc436fu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279b94u64) }
        0x10279b94 => { vm.decode(0x8012036u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 9i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10279b98u64) }
        0x10279b98 => { vm.decode(0xa8c02fu32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279b9cu64) }
        0x10279b9c => { vm.decode(0x4b0d1062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10279ba0u64) }
        0x10279ba0 => { vm.decode(0x9b195cb0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279ba4u64) }
        0x10279ba4 => { vm.decode(0xb111db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279ba8u64) }
        0x10279ba8 => { vm.decode(0x282c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279bacu64) }
        0x10279bac => { vm.decode(0xa10c875u32); let d = 25i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279bb0u64) }
        0x10279bb0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279bb4u64 + 4);
        vm.wr(37, 0x10279bb4u64 + 4);
        Ok(0x10279bb4u64) }
        0x10279bb4 => { vm.decode(0x66598459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2864i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279bb8u64) }
        0x10279bb8 => { vm.decode(0x28a0602fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279bbcu64) }
        0x10279bbc => { vm.decode(0xc8e3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279bc0u64) }
        0x10279bc0 => { vm.decode(0x2b299530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279bc4u64) }
        0x10279bc4 => { vm.decode(0xab219430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 3776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279bc8u64) }
        0x10279bc8 => { vm.decode(0x3b1994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279bccu64) }
        0x10279bcc => { vm.decode(0x282732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1048i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279bd0u64) }
        0x10279bd0 => { vm.decode(0xa10c875u32); let d = 25i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279bd4u64) }
        0x10279bd4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279bd8u64 + 4);
        vm.wr(37, 0x10279bd8u64 + 4);
        Ok(0x10279bd8u64) }
        0x10279bd8 => { vm.decode(0x667f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3768i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279bdcu64) }
        0x10279bdc => { vm.decode(0x28a0406fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279be0u64) }
        0x10279be0 => { vm.decode(0x8b091cb0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279be4u64) }
        0x10279be4 => { vm.decode(0x9b011db0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279be8u64) }
        0x10279be8 => { vm.decode(0x88cc3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279becu64) }
        0x10279bec => { vm.decode(0xb191d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279bf0u64) }
        0x10279bf0 => { vm.decode(0x1b111c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279bf4u64) }
        0x10279bf4 => { vm.decode(0x282532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279bf8u64) }
        0x10279bf8 => { vm.decode(0xa10c875u32); let d = 25i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279bfcu64) }
        0x10279bfc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279c00u64 + 4);
        vm.wr(37, 0x10279c00u64 + 4);
        Ok(0x10279c00u64) }
        0x10279c00 => { vm.decode(0x66498459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2832i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279c04u64) }
        0x10279c04 => { vm.decode(0x20a0606fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279c08u64) }
        0x10279c08 => { vm.decode(0x8c73afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279c0cu64) }
        0x10279c0c => { vm.decode(0xb091d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279c10u64) }
        0x10279c10 => { vm.decode(0xa8ce3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279c14u64) }
        0x10279c14 => { vm.decode(0xb011d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279c18u64) }
        0x10279c18 => { vm.decode(0x1b3994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279c1cu64) }
        0x10279c1c => { vm.decode(0x82e3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279c20u64) }
        0x10279c20 => { vm.decode(0xa10c875u32); let d = 25i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279c24u64) }
        0x10279c24 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279c28u64 + 4);
        vm.wr(37, 0x10279c28u64 + 4);
        Ok(0x10279c28u64) }
        0x10279c28 => { vm.decode(0x665f8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2808i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279c2cu64) }
        0x10279c2c => { vm.decode(0x2aa0406fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279c30u64) }
        0x10279c30 => { vm.decode(0x28cf3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279c34u64) }
        0x10279c34 => { vm.decode(0xb3195b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279c38u64) }
        0x10279c38 => { vm.decode(0x888e3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279c3cu64) }
        0x10279c3c => { vm.decode(0xb2995b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279c40u64) }
        0x10279c40 => { vm.decode(0x1b2194b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279c44u64) }
        0x10279c44 => { vm.decode(0xa86c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279c48u64) }
        0x10279c48 => { vm.decode(0xa10c875u32); let d = 25i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279c4cu64) }
        0x10279c4c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279c50u64 + 4);
        vm.wr(37, 0x10279c50u64 + 4);
        Ok(0x10279c50u64) }
        0x10279c50 => { vm.decode(0x661b8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2784i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279c54u64) }
        0x10279c54 => { vm.decode(0x20a4c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279c58u64) }
        0x10279c58 => { vm.decode(0x28cf3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279c5cu64) }
        0x10279c5c => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279c60u64) }
        0x10279c60 => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279c64u64) }
        0x10279c64 => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279c68u64) }
        0x10279c68 => { vm.decode(0x82532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279c6cu64) }
        0x10279c6c => { vm.decode(0xa10c875u32); let d = 25i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279c70u64) }
        0x10279c70 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279c74u64 + 4);
        vm.wr(37, 0x10279c74u64 + 4);
        Ok(0x10279c74u64) }
        0x10279c74 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279c78u64) }
        0x10279c78 => { vm.decode(0x88d3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279c7cu64) }
        0x10279c7c => { vm.decode(0x628e532fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279c80u64) }
        0x10279c80 => { vm.decode(0x4a86c3efu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279c84u64) }
        0x10279c84 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10279c88u64).wrapping_add((imm * 4) as u64) } else { 0x10279c88u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10279c88u64) }
        0x10279c88 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10279c8cu64) } }
        0x10279c8c => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10279c90u64) }
        0x10279c90 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10279c94u64) }
        0x10279c94 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10279c98u64) }
        0x10279c98 => { vm.decode(0x88d3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279c9cu64) }
        0x10279c9c => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10279ca0u64) }
        0x10279ca0 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x10279ca4u64).wrapping_add((imm * 4) as u64) } else { 0x10279ca4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10279ca4u64) }
        0x10279ca4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10279ca8u64) } }
        0x10279ca8 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10279cacu64).wrapping_add((imm * 4) as u64) } else { 0x10279cacu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10279cacu64) }
        0x10279cac => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10279cb0u64) } }
        0x10279cb0 => { vm.decode(0x22a0c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279cb4u64) }
        0x10279cb4 => { vm.decode(0x2b311530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279cb8u64) }
        0x10279cb8 => { vm.decode(0x3b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279cbcu64) }
        0x10279cbc => { vm.decode(0xa82c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279cc0u64) }
        0x10279cc0 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279cc4u64) }
        0x10279cc4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279cc8u64 + 4);
        vm.wr(37, 0x10279cc8u64 + 4);
        Ok(0x10279cc8u64) }
        0x10279cc8 => { vm.decode(0x662d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3656i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279cccu64) }
        0x10279ccc => { vm.decode(0x2b201c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279cd0u64) }
        0x10279cd0 => { vm.decode(0x28cf3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279cd4u64) }
        0x10279cd4 => { vm.decode(0x8000a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10279cd8u64).wrapping_add((imm * 4) as u64) } else { 0x10279cd8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10279cd8u64) }
        0x10279cd8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10279cdcu64) } }
        0x10279cdc => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279ce0u64) }
        0x10279ce0 => { vm.decode(0x2000212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279ce4u64) }
        0x10279ce4 => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279ce8u64) }
        0x10279ce8 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279cecu64) }
        0x10279cec => { vm.decode(0x62198219u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279cf0u64) }
        0x10279cf0 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279cf4u64) }
        0x10279cf4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279cf8u64 + 4);
        vm.wr(37, 0x10279cf8u64 + 4);
        Ok(0x10279cf8u64) }
        0x10279cf8 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279cfcu64) }
        0x10279cfc => { vm.decode(0x2b181cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279d00u64) }
        0x10279d00 => { vm.decode(0x20a4e06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279d04u64) }
        0x10279d04 => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279d08u64) }
        0x10279d08 => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279d0cu64) }
        0x10279d0c => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279d10u64) }
        0x10279d10 => { vm.decode(0x6080736fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279d14u64) }
        0x10279d14 => { vm.decode(0xe108875u32); let d = 25i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279d18u64) }
        0x10279d18 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279d1cu64 + 4);
        vm.wr(37, 0x10279d1cu64 + 4);
        Ok(0x10279d1cu64) }
        0x10279d1c => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279d20u64) }
        0x10279d20 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279d24u64) }
        0x10279d24 => { vm.decode(0x20a4c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279d28u64) }
        0x10279d28 => { vm.decode(0x8c73afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279d2cu64) }
        0x10279d2c => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279d30u64) }
        0x10279d30 => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279d34u64) }
        0x10279d34 => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279d38u64) }
        0x10279d38 => { vm.decode(0x4088532fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279d3cu64) }
        0x10279d3c => { vm.decode(0xe108875u32); let d = 25i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279d40u64) }
        0x10279d40 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279d44u64 + 4);
        vm.wr(37, 0x10279d44u64 + 4);
        Ok(0x10279d44u64) }
        0x10279d44 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279d48u64) }
        0x10279d48 => { vm.decode(0x28c53afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279d4cu64) }
        0x10279d4c => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10279d50u64).wrapping_add((imm * 4) as u64) } else { 0x10279d50u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10279d50u64) }
        0x10279d50 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10279d54u64) } }
        0x10279d54 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10279d58u64) }
        0x10279d58 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10279d5cu64) }
        0x10279d5c => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10279d60u64) }
        0x10279d60 => { vm.decode(0x28c53afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279d64u64) }
        0x10279d64 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10279d68u64) }
        0x10279d68 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x10279d6cu64).wrapping_add((imm * 4) as u64) } else { 0x10279d6cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10279d6cu64) }
        0x10279d6c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10279d70u64) } }
        0x10279d70 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10279d74u64).wrapping_add((imm * 4) as u64) } else { 0x10279d74u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10279d74u64) }
        0x10279d74 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10279d78u64) } }
        0x10279d78 => { vm.decode(0x22a0c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279d7cu64) }
        0x10279d7c => { vm.decode(0x2b311530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279d80u64) }
        0x10279d80 => { vm.decode(0x3b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279d84u64) }
        0x10279d84 => { vm.decode(0xa82c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279d88u64) }
        0x10279d88 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279d8cu64) }
        0x10279d8c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279d90u64 + 4);
        vm.wr(37, 0x10279d90u64 + 4);
        Ok(0x10279d90u64) }
        0x10279d90 => { vm.decode(0x662d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3656i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279d94u64) }
        0x10279d94 => { vm.decode(0x2b1014b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279d98u64) }
        0x10279d98 => { vm.decode(0x8c73afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279d9cu64) }
        0x10279d9c => { vm.decode(0x8000a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10279da0u64).wrapping_add((imm * 4) as u64) } else { 0x10279da0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10279da0u64) }
        0x10279da0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10279da4u64) } }
        0x10279da4 => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279da8u64) }
        0x10279da8 => { vm.decode(0x2000212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279dacu64) }
        0x10279dac => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279db0u64) }
        0x10279db0 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279db4u64) }
        0x10279db4 => { vm.decode(0x62198219u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279db8u64) }
        0x10279db8 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279dbcu64) }
        0x10279dbc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279dc0u64 + 4);
        vm.wr(37, 0x10279dc0u64 + 4);
        Ok(0x10279dc0u64) }
        0x10279dc0 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279dc4u64) }
        0x10279dc4 => { vm.decode(0x2b0814b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279dc8u64) }
        0x10279dc8 => { vm.decode(0x20a4e06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279dccu64) }
        0x10279dcc => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279dd0u64) }
        0x10279dd0 => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279dd4u64) }
        0x10279dd4 => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279dd8u64) }
        0x10279dd8 => { vm.decode(0x6080736fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279ddcu64) }
        0x10279ddc => { vm.decode(0xe108875u32); let d = 25i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279de0u64) }
        0x10279de0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279de4u64 + 4);
        vm.wr(37, 0x10279de4u64 + 4);
        Ok(0x10279de4u64) }
        0x10279de4 => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279de8u64) }
        0x10279de8 => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279decu64) }
        0x10279dec => { vm.decode(0x62198219u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279df0u64) }
        0x10279df0 => { vm.decode(0x8d0814b0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279df4u64) }
        0x10279df4 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279df8u64) }
        0x10279df8 => { vm.decode(0xe108875u32); let d = 25i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279dfcu64) }
        0x10279dfc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279e00u64 + 4);
        vm.wr(37, 0x10279e00u64 + 4);
        Ok(0x10279e00u64) }
        0x10279e00 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279e04u64) }
        0x10279e04 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279e08u64) }
        0x10279e08 => { vm.decode(0x20a4c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279e0cu64) }
        0x10279e0c => { vm.decode(0x8cc436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279e10u64) }
        0x10279e10 => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279e14u64) }
        0x10279e14 => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279e18u64) }
        0x10279e18 => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279e1cu64) }
        0x10279e1c => { vm.decode(0xe108875u32); let d = 25i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279e20u64) }
        0x10279e20 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279e24u64 + 4);
        vm.wr(37, 0x10279e24u64 + 4);
        Ok(0x10279e24u64) }
        0x10279e24 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279e28u64) }
        0x10279e28 => { vm.decode(0x8cc636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279e2cu64) }
        0x10279e2c => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10279e30u64).wrapping_add((imm * 4) as u64) } else { 0x10279e30u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10279e30u64) }
        0x10279e30 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10279e34u64) } }
        0x10279e34 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10279e38u64) }
        0x10279e38 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10279e3cu64) }
        0x10279e3c => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10279e40u64) }
        0x10279e40 => { vm.decode(0x8cc636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279e44u64) }
        0x10279e44 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10279e48u64) }
        0x10279e48 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x10279e4cu64).wrapping_add((imm * 4) as u64) } else { 0x10279e4cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10279e4cu64) }
        0x10279e4c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10279e50u64) } }
        0x10279e50 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10279e54u64).wrapping_add((imm * 4) as u64) } else { 0x10279e54u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10279e54u64) }
        0x10279ea8 => { vm.decode(0x20a4e06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279eacu64) }
        0x10279eac => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279eb0u64) }
        0x10279eb0 => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279eb4u64) }
        0x10279eb4 => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279eb8u64) }
        0x10279eb8 => { vm.decode(0x6088736fu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279ebcu64) }
        0x10279ebc => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279ec0u64) }
        0x10279ec0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279ec4u64 + 4);
        vm.wr(37, 0x10279ec4u64 + 4);
        Ok(0x10279ec4u64) }
        0x10279ec4 => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279ec8u64) }
        0x10279ec8 => { vm.decode(0x66798559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3952i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279eccu64) }
        0x10279ecc => { vm.decode(0x20c0632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279ed0u64) }
        0x10279ed0 => { vm.decode(0x102436u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10279ed4u64) }
        0x10279ed4 => { vm.decode(0xaa8606fu32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279ed8u64) }
        0x10279ed8 => { vm.decode(0xb3d1962u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 3976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10279edcu64) }
        0x10279edc => { vm.decode(0x3b019c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279ee0u64) }
        0x10279ee0 => { vm.decode(0xab391cb0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 3960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279ee4u64) }
        0x10279ee4 => { vm.decode(0x2b311db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279ee8u64) }
        0x10279ee8 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279eecu64) }
        0x10279eec => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279ef0u64 + 4);
        vm.wr(37, 0x10279ef0u64 + 4);
        Ok(0x10279ef0u64) }
        0x10279ef0 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279ef4u64) }
        0x10279ef4 => { vm.decode(0x22a0406fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279ef8u64) }
        0x10279ef8 => { vm.decode(0x4a186873u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 3376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10279efcu64) }
        0x10279efc => { vm.decode(0xa9091cb0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 3368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279f00u64) }
        0x10279f00 => { vm.decode(0xa8ce3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279f04u64) }
        0x10279f04 => { vm.decode(0x29011db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279f08u64) }
        0x10279f08 => { vm.decode(0x39191c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279f0cu64) }
        0x10279f0c => { vm.decode(0x286e3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279f10u64) }
        0x10279f10 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279f14u64) }
        0x10279f14 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279f18u64 + 4);
        vm.wr(37, 0x10279f18u64 + 4);
        Ok(0x10279f18u64) }
        0x10279f18 => { vm.decode(0x626d8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3352i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279f1cu64) }
        0x10279f1c => { vm.decode(0x2aa0406fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279f20u64) }
        0x10279f20 => { vm.decode(0x28cf3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279f24u64) }
        0x10279f24 => { vm.decode(0xb3195b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279f28u64) }
        0x10279f28 => { vm.decode(0x28ce3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279f2cu64) }
        0x10279f2c => { vm.decode(0xb2995b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279f30u64) }
        0x10279f30 => { vm.decode(0x1b2194b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279f34u64) }
        0x10279f34 => { vm.decode(0xa86c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279f38u64) }
        0x10279f38 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279f3cu64) }
        0x10279f3c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279f40u64 + 4);
        vm.wr(37, 0x10279f40u64 + 4);
        Ok(0x10279f40u64) }
        0x10279f40 => { vm.decode(0x661b8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2784i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279f44u64) }
        0x10279f44 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279f48u64) }
        0x10279f48 => { vm.decode(0x20a4c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279f4cu64) }
        0x10279f4c => { vm.decode(0x28cf3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279f50u64) }
        0x10279f50 => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279f54u64) }
        0x10279f54 => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279f58u64) }
        0x10279f58 => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279f5cu64) }
        0x10279f5c => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279f60u64) }
        0x10279f60 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279f64u64 + 4);
        vm.wr(37, 0x10279f64u64 + 4);
        Ok(0x10279f64u64) }
        0x10279f64 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279f68u64) }
        0x10279f68 => { vm.decode(0x88d3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279f6cu64) }
        0x10279f6c => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10279f70u64).wrapping_add((imm * 4) as u64) } else { 0x10279f70u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10279f70u64) }
        0x10279f70 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10279f74u64) } }
        0x10279f74 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10279f78u64) }
        0x10279f78 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10279f7cu64) }
        0x10279f7c => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10279f80u64) }
        0x10279f80 => { vm.decode(0x88d3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279f84u64) }
        0x10279f84 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10279f88u64) }
        0x10279f88 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x10279f8cu64).wrapping_add((imm * 4) as u64) } else { 0x10279f8cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10279f8cu64) }
        0x10279f8c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10279f90u64) } }
        0x10279f90 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10279f94u64).wrapping_add((imm * 4) as u64) } else { 0x10279f94u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10279f94u64) }
        0x10279f94 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10279f98u64) } }
        0x10279f98 => { vm.decode(0x22a0c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279f9cu64) }
        0x10279f9c => { vm.decode(0x2b311530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279fa0u64) }
        0x10279fa0 => { vm.decode(0x3b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279fa4u64) }
        0x10279fa4 => { vm.decode(0xa82c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279fa8u64) }
        0x10279fa8 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279facu64) }
        0x10279fac => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279fb0u64 + 4);
        vm.wr(37, 0x10279fb0u64 + 4);
        Ok(0x10279fb0u64) }
        0x10279fb0 => { vm.decode(0x662d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3656i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279fb4u64) }
        0x10279fb4 => { vm.decode(0x2b201c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279fb8u64) }
        0x10279fb8 => { vm.decode(0x28cf3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279fbcu64) }
        0x10279fbc => { vm.decode(0x8000a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10279fc0u64).wrapping_add((imm * 4) as u64) } else { 0x10279fc0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10279fc0u64) }
        0x10279fc0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10279fc4u64) } }
        0x10279fc4 => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279fc8u64) }
        0x10279fc8 => { vm.decode(0x2000212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279fccu64) }
        0x10279fcc => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279fd0u64) }
        0x10279fd0 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279fd4u64) }
        0x10279fd4 => { vm.decode(0x62198219u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10279fd8u64) }
        0x10279fd8 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279fdcu64) }
        0x10279fdc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10279fe0u64 + 4);
        vm.wr(37, 0x10279fe0u64 + 4);
        Ok(0x10279fe0u64) }
        0x10279fe0 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10279fe4u64) }
        0x10279fe4 => { vm.decode(0x2b181cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279fe8u64) }
        0x10279fe8 => { vm.decode(0x20a4e06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279fecu64) }
        0x10279fec => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279ff0u64) }
        0x10279ff0 => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10279ff4u64) }
        0x10279ff4 => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279ff8u64) }
        0x10279ff8 => { vm.decode(0x6088736fu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10279ffcu64) }
        0x10279ffc => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a000u64) }
        0x1027a000 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027a004u64 + 4);
        vm.wr(37, 0x1027a004u64 + 4);
        Ok(0x1027a004u64) }
        0x1027a004 => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a008u64) }
        0x1027a008 => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a00cu64) }
        0x1027a00c => { vm.decode(0x62198219u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a010u64) }
        0x1027a010 => { vm.decode(0x8d0814b0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a014u64) }
        0x1027a014 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a018u64) }
        0x1027a018 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a01cu64) }
        0x1027a01c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027a020u64 + 4);
        vm.wr(37, 0x1027a020u64 + 4);
        Ok(0x1027a020u64) }
        0x1027a020 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a024u64) }
        0x1027a024 => { vm.decode(0x66798559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3952i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a028u64) }
        0x1027a028 => { vm.decode(0x2aa0606fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a02cu64) }
        0x1027a02c => { vm.decode(0xb3d1962u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 3976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027a030u64) }
        0x1027a030 => { vm.decode(0xc8632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a034u64) }
        0x1027a034 => { vm.decode(0xc01019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a038u64) }
        0x1027a038 => { vm.decode(0x2b019d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a03cu64) }
        0x1027a03c => { vm.decode(0xab391cb0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 3960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a040u64) }
        0x1027a040 => { vm.decode(0x3b311cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a044u64) }
        0x1027a044 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a048u64) }
        0x1027a048 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027a04cu64 + 4);
        vm.wr(37, 0x1027a04cu64 + 4);
        Ok(0x1027a04cu64) }
        0x1027a04c => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a050u64) }
        0x1027a050 => { vm.decode(0x22a0406fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a054u64) }
        0x1027a054 => { vm.decode(0x4a186873u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 3376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1027a058u64) }
        0x1027a058 => { vm.decode(0xa9091cb0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 3368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a05cu64) }
        0x1027a05c => { vm.decode(0xa8ce3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a060u64) }
        0x1027a060 => { vm.decode(0x29011db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a064u64) }
        0x1027a064 => { vm.decode(0x39191c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a068u64) }
        0x1027a068 => { vm.decode(0x286e3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a06cu64) }
        0x1027a06c => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a070u64) }
        0x1027a070 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027a074u64 + 4);
        vm.wr(37, 0x1027a074u64 + 4);
        Ok(0x1027a074u64) }
        0x1027a074 => { vm.decode(0x626d8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3352i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a078u64) }
        0x1027a078 => { vm.decode(0x2aa0406fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a07cu64) }
        0x1027a07c => { vm.decode(0x28cf3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a080u64) }
        0x1027a080 => { vm.decode(0xb3195b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a084u64) }
        0x1027a084 => { vm.decode(0x28cc3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a088u64) }
        0x1027a088 => { vm.decode(0xb2995b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a08cu64) }
        0x1027a08c => { vm.decode(0x1b2194b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a090u64) }
        0x1027a090 => { vm.decode(0xa86c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a094u64) }
        0x1027a094 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a098u64) }
        0x1027a098 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027a09cu64 + 4);
        vm.wr(37, 0x1027a09cu64 + 4);
        Ok(0x1027a09cu64) }
        0x1027a09c => { vm.decode(0x661b8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2784i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a0a0u64) }
        0x1027a0a0 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a0a4u64) }
        0x1027a0a4 => { vm.decode(0x20a4c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a0a8u64) }
        0x1027a0a8 => { vm.decode(0x28cf3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a0acu64) }
        0x1027a0ac => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a0b0u64) }
        0x1027a0b0 => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a0b4u64) }
        0x1027a0b4 => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a0b8u64) }
        0x1027a0b8 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a0bcu64) }
        0x1027a0bc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027a0c0u64 + 4);
        vm.wr(37, 0x1027a0c0u64 + 4);
        Ok(0x1027a0c0u64) }
        0x1027a0c0 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a0c4u64) }
        0x1027a0c4 => { vm.decode(0x88d3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a0c8u64) }
        0x1027a0c8 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027a0ccu64).wrapping_add((imm * 4) as u64) } else { 0x1027a0ccu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027a0ccu64) }
        0x1027a0cc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027a0d0u64) } }
        0x1027a0d0 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027a0d4u64) }
        0x1027a0d4 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027a0d8u64) }
        0x1027a0d8 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027a0dcu64) }
        0x1027a0dc => { vm.decode(0x88d3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a0e0u64) }
        0x1027a0e0 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027a0e4u64) }
        0x1027a0e4 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x1027a0e8u64).wrapping_add((imm * 4) as u64) } else { 0x1027a0e8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027a0e8u64) }
        0x1027a0e8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027a0ecu64) } }
        0x1027a0ec => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027a0f0u64).wrapping_add((imm * 4) as u64) } else { 0x1027a0f0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027a0f0u64) }
        0x1027a0f0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027a0f4u64) } }
        0x1027a0f4 => { vm.decode(0x22a0c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a0f8u64) }
        0x1027a0f8 => { vm.decode(0x2b311530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a0fcu64) }
        0x1027a0fc => { vm.decode(0x3b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a100u64) }
        0x1027a100 => { vm.decode(0xa82c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a104u64) }
        0x1027a104 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a108u64) }
        0x1027a108 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027a10cu64 + 4);
        vm.wr(37, 0x1027a10cu64 + 4);
        Ok(0x1027a10cu64) }
        0x1027a10c => { vm.decode(0x662d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3656i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a110u64) }
        0x1027a110 => { vm.decode(0x2b201c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a114u64) }
        0x1027a114 => { vm.decode(0x28cf3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a118u64) }
        0x1027a118 => { vm.decode(0x8000a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027a11cu64).wrapping_add((imm * 4) as u64) } else { 0x1027a11cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027a11cu64) }
        0x1027a11c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027a120u64) } }
        0x1027a120 => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a124u64) }
        0x1027a124 => { vm.decode(0x2000212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a128u64) }
        0x1027a128 => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a12cu64) }
        0x1027a12c => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a130u64) }
        0x1027a130 => { vm.decode(0x62198219u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a134u64) }
        0x1027a134 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a138u64) }
        0x1027a138 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027a13cu64 + 4);
        vm.wr(37, 0x1027a13cu64 + 4);
        Ok(0x1027a13cu64) }
        0x1027a13c => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a140u64) }
        0x1027a140 => { vm.decode(0x2b181cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a144u64) }
        0x1027a144 => { vm.decode(0x8c732fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a148u64) }
        0x1027a148 => { vm.decode(0x228043afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a14cu64) }
        0x1027a14c => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027a150u64) }
        0x1027a150 => { vm.decode(0x1b3094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a154u64) }
        0x1027a154 => { vm.decode(0x228063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a158u64) }
        0x1027a158 => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027a15cu64) }
        0x1027a15c => { vm.decode(0x1b181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a160u64) }
        0x1027a160 => { vm.decode(0x208443afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a164u64) }
        0x1027a164 => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027a168u64) }
        0x1027a168 => { vm.decode(0x1b0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a16cu64) }
        0x1027a16c => { vm.decode(0x208463afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a170u64) }
        0x1027a170 => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027a174u64) }
        0x1027a174 => { vm.decode(0x1b181cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a178u64) }
        0x1027a178 => { vm.decode(0x228443afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a17cu64) }
        0x1027a17c => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027a180u64) }
        0x1027a180 => { vm.decode(0x1b1014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a184u64) }
        0x1027a184 => { vm.decode(0x228463afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a188u64) }
        0x1027a188 => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027a18cu64) }
        0x1027a18c => { vm.decode(0x1b2094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a190u64) }
        0x1027a190 => { vm.decode(0x208043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a194u64) }
        0x1027a194 => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027a198u64) }
        0x1027a198 => { vm.decode(0x1b309430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a19cu64) }
        0x1027a19c => { vm.decode(0x208063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a1a0u64) }
        0x1027a1a0 => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027a1a4u64) }
        0x1027a1a4 => { vm.decode(0x1b289430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a1a8u64) }
        0x1027a1a8 => { vm.decode(0x228043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a1acu64) }
        0x1027a1ac => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027a1b0u64) }
        0x1027a1b0 => { vm.decode(0x1b209430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a1b4u64) }
        0x1027a1b4 => { vm.decode(0x228063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a1b8u64) }
        0x1027a1b8 => { vm.decode(0x684404f5u32); let d = 19i16; let s = 2i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027a1bcu64) }
        0x1027a1bc => { vm.decode(0x208443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a1c0u64) }
        0x1027a1c0 => { vm.decode(0x604604f5u32); let d = 21i16; let s = 2i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027a1c4u64) }
        0x1027a1c4 => { vm.decode(0x810d875u32); let d = 17i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a1c8u64) }
        0x1027a1c8 => { vm.decode(0x208463efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a1ccu64) }
        0x1027a1cc => { vm.decode(0x680604f5u32); let d = 22i16; let s = 2i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027a1d0u64) }
        0x1027a1d0 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a1d4u64) }
        0x1027a1d4 => { vm.decode(0x600604f5u32); let d = 20i16; let s = 2i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027a1d8u64) }
        0x1027a1d8 => { vm.decode(0x228463efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a1dcu64) }
        0x1027a1dc => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027a1e0u64) }
        0x1027a1e0 => { vm.decode(0x1b2814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a1e4u64) }
        0x1027a1e4 => { vm.decode(0x288043afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a1e8u64) }
        0x1027a1e8 => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027a1ecu64) }
        0x1027a1ec => { vm.decode(0x1b009430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a1f0u64) }
        0x1027a1f0 => { vm.decode(0x288063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a1f4u64) }
        0x1027a1f4 => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027a1f8u64) }
        0x1027a1f8 => { vm.decode(0x1b3814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a1fcu64) }
        0x1027a1fc => { vm.decode(0x288443afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a200u64) }
        0x1027a200 => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027a204u64) }
        0x1027a204 => { vm.decode(0x1b3014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a208u64) }
        0x1027a208 => { vm.decode(0x2084e3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a20cu64) }
        0x1027a20c => { vm.decode(0x40408119u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 80i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a210u64) }
        0x1027a210 => { vm.decode(0x1b089430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a214u64) }
        0x1027a214 => { vm.decode(0xd144051u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027a218u64) }
        0x1027a218 => { vm.decode(0xf7cbaf2du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 65535i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1027a21cu64) }
        0x1027a21c => { vm.decode(0x232d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027a220u64) }
        0x1027a220 => { vm.decode(0x2004e16fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a224u64) }
        0x1027a224 => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a228u64) }
        0x1027a228 => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a22cu64) }
        0x1027a22c => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a230u64) }
        0x1027a230 => { vm.decode(0x4088736fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a234u64) }
        0x1027a234 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a238u64) }
        0x1027a238 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027a23cu64 + 4);
        vm.wr(37, 0x1027a23cu64 + 4);
        Ok(0x1027a23cu64) }
        0x1027a23c => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a240u64) }
        0x1027a240 => { vm.decode(0x2800412fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a244u64) }
        0x1027a244 => { vm.decode(0x62198219u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a248u64) }
        0x1027a248 => { vm.decode(0x390854b0u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a24cu64) }
        0x1027a24c => { vm.decode(0x8d0814b0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a250u64) }
        0x1027a250 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a254u64) }
        0x1027a254 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a258u64) }
        0x1027a258 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027a25cu64 + 4);
        vm.wr(37, 0x1027a25cu64 + 4);
        Ok(0x1027a25cu64) }
        0x1027a25c => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a260u64) }
        0x1027a260 => { vm.decode(0x66798559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3952i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a264u64) }
        0x1027a264 => { vm.decode(0x39105430u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a268u64) }
        0x1027a268 => { vm.decode(0xa036u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027a26cu64) }
        0x1027a26c => { vm.decode(0x45144051u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027a270u64) }
        0x1027a270 => { vm.decode(0x2055862u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027a274u64) }
        0x1027a274 => { vm.decode(0x4c908875u32); let d = 23i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a278u64) }
        0x1027a278 => { vm.decode(0xa08616fu32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a27cu64) }
        0x1027a27c => { vm.decode(0xb3d1862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027a280u64) }
        0x1027a280 => { vm.decode(0xab01dd70u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a284u64) }
        0x1027a284 => { vm.decode(0x2a84e3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a288u64) }
        0x1027a288 => { vm.decode(0x3b391cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a28cu64) }
        0x1027a28c => { vm.decode(0x2b311db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a290u64) }
        0x1027a290 => { vm.decode(0xa86c3efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a294u64) }
        0x1027a294 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a298u64) }
        0x1027a298 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027a29cu64 + 4);
        vm.wr(37, 0x1027a29cu64 + 4);
        Ok(0x1027a29cu64) }
        0x1027a29c => { vm.decode(0xc008875u32); let d = 17i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a2a0u64) }
        0x1027a2a0 => { vm.decode(0x627f8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3320i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a2a4u64) }
        0x1027a2a4 => { vm.decode(0x390054b0u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a2a8u64) }
        0x1027a2a8 => { vm.decode(0x22a0616fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a2acu64) }
        0x1027a2ac => { vm.decode(0xb9111c30u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 3344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a2b0u64) }
        0x1027a2b0 => { vm.decode(0x6880e3afu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a2b4u64) }
        0x1027a2b4 => { vm.decode(0xb9091c30u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 3336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a2b8u64) }
        0x1027a2b8 => { vm.decode(0x6a8ee3efu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a2bcu64) }
        0x1027a2bc => { vm.decode(0xb9015d30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 3328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a2c0u64) }
        0x1027a2c0 => { vm.decode(0x393994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a2c4u64) }
        0x1027a2c4 => { vm.decode(0x886e3efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a2c8u64) }
        0x1027a2c8 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a2ccu64) }
        0x1027a2cc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027a2d0u64 + 4);
        vm.wr(37, 0x1027a2d0u64 + 4);
        Ok(0x1027a2d0u64) }
        0x1027a2d0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027a2d4u64) } }
        0x1027a2d4 => { vm.decode(0x2080536fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a2d8u64) }
        0x1027a2d8 => { vm.decode(0x2200216fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a2dcu64) }
        0x1027a2dc => { vm.decode(0xb9111c30u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 3344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a2e0u64) }
        0x1027a2e0 => { vm.decode(0xb9095d30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 3336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a2e4u64) }
        0x1027a2e4 => { vm.decode(0xa9011c30u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 3328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a2e8u64) }
        0x1027a2e8 => { vm.decode(0x393994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a2ecu64) }
        0x1027a2ec => { vm.decode(0x886e3efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a2f0u64) }
        0x1027a2f0 => { vm.decode(0x2086532fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a2f4u64) }
        0x1027a2f4 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a2f8u64) }
        0x1027a2f8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027a2fcu64 + 4);
        vm.wr(37, 0x1027a2fcu64 + 4);
        Ok(0x1027a2fcu64) }
        0x1027a2fc => { vm.decode(0x48909875u32); let d = 23i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a300u64) }
        0x1027a300 => { vm.decode(0x319900eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 372310016i64;
        vm.st(d, imm as u64);
        Ok(0x1027a304u64) }
        0x1027a304 => { vm.decode(0xc016c699u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 14507i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a308u64) }
        0x1027a308 => { vm.decode(0x422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1027a30cu64) }
        0x1027a30c => { vm.decode(0x888844eu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 7143424i64;
        vm.st(d, imm as u64);
        Ok(0x1027a310u64) }
        0x1027a310 => { vm.decode(0x80a018d9u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 17667i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a314u64) }
        0x1027a314 => { vm.decode(0xaa021d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 23i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1027a318u64) }
        0x1027a318 => { vm.decode(0xa611d9u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1481i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a31cu64) }
        0x1027a31c => { vm.decode(0xc814e5d9u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -22161i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a320u64) }
        0x1027a320 => { vm.decode(0x1a91a54eu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 1484652544i64;
        vm.st(d, imm as u64);
        Ok(0x1027a324u64) }
        0x1027a324 => { vm.decode(0xccb0b0d9u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = -30937i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a328u64) }
        0x1027a328 => { vm.decode(0x2423d75u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1027a32cu64) }
        0x1027a32c => { vm.decode(0xc082fbd9u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 29127i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a330u64) }
        0x1027a330 => { vm.decode(0x22423d75u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 17i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1027a334u64) }
        0x1027a334 => { vm.decode(0x422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1027a338u64) }
        0x1027a338 => { vm.decode(0x82021d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 18i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1027a33cu64) }
        0x1027a33c => { vm.decode(0x410410eu32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 9568256i64;
        vm.st(d, imm as u64);
        Ok(0x1027a340u64) }
        0x1027a340 => { vm.decode(0x21165d9u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 10597i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a344u64) }
        0x1027a344 => { vm.decode(0x10028d75u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1027a348u64) }
        0x1027a348 => { vm.decode(0x86136b99u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 29415i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a34cu64) }
        0x1027a34c => { vm.decode(0xb8028d75u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 23i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1027a350u64) }
        0x1027a350 => { vm.decode(0x86352399u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 5743i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a354u64) }
        0x1027a354 => { vm.decode(0x84c23999u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 17111i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a358u64) }
        0x1027a358 => { vm.decode(0x4056e219u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 12476i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a35cu64) }
        0x1027a35c => { vm.decode(0x4ca4fd99u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = -4531i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a360u64) }
        0x1027a360 => { vm.decode(0x47106191u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 2308i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027a364u64) }
        0x1027a364 => { vm.decode(0x20cae32fu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 2312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a368u64) }
        0x1027a368 => { vm.decode(0x39001db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a36cu64) }
        0x1027a36c => { vm.decode(0x39181c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a370u64) }
        0x1027a370 => { vm.decode(0x29101d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a374u64) }
        0x1027a374 => { vm.decode(0x29085c30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a378u64) }
        0x1027a378 => { vm.decode(0x29001c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a37cu64) }
        0x1027a37c => { vm.decode(0x4088536fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a380u64) }
        0x1027a380 => { vm.decode(0x2224c06fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a384u64) }
        0x1027a384 => { vm.decode(0xab19ddf0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 4024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a388u64) }
        0x1027a388 => { vm.decode(0x3b21dd30u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 4032i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a38cu64) }
        0x1027a38c => { vm.decode(0x4a7d5862u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 4040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027a390u64) }
        0x1027a390 => { vm.decode(0x3b119cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a394u64) }
        0x1027a394 => { vm.decode(0xa8263efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a398u64) }
        0x1027a398 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a39cu64) }
        0x1027a39c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027a3a0u64 + 4);
        vm.wr(37, 0x1027a3a0u64 + 4);
        Ok(0x1027a3a0u64) }
        0x1027a3a0 => { vm.decode(0x667b8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 4016i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a3a4u64) }
        0x1027a3a4 => { vm.decode(0x2224e06fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a3a8u64) }
        0x1027a3a8 => { vm.decode(0x6884c3efu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a3acu64) }
        0x1027a3ac => { vm.decode(0xbb019cb0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 4000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a3b0u64) }
        0x1027a3b0 => { vm.decode(0xab19dd70u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a3b4u64) }
        0x1027a3b4 => { vm.decode(0x3b119c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a3b8u64) }
        0x1027a3b8 => { vm.decode(0x82c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a3bcu64) }
        0x1027a3bc => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a3c0u64) }
        0x1027a3c0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027a3c4u64 + 4);
        vm.wr(37, 0x1027a3c4u64 + 4);
        Ok(0x1027a3c4u64) }
        0x1027a3c4 => { vm.decode(0x666b8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3984i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a3c8u64) }
        0x1027a3c8 => { vm.decode(0x4002036u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027a3ccu64) }
        0x1027a3cc => { vm.decode(0xa28606fu32); let d = 2i16; let s = 18i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a3d0u64) }
        0x1027a3d0 => { vm.decode(0x48909875u32); let d = 23i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a3d4u64) }
        0x1027a3d4 => { vm.decode(0xb3d1862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027a3d8u64) }
        0x1027a3d8 => { vm.decode(0xbb019c30u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 3968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a3dcu64) }
        0x1027a3dc => { vm.decode(0x2080e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a3e0u64) }
        0x1027a3e0 => { vm.decode(0x3b391cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a3e4u64) }
        0x1027a3e4 => { vm.decode(0x2b311db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a3e8u64) }
        0x1027a3e8 => { vm.decode(0xa86c3efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a3ecu64) }
        0x1027a3ec => { vm.decode(0x2282532fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a3f0u64) }
        0x1027a3f0 => { vm.decode(0x4088736fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a3f4u64) }
        0x1027a3f4 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a3f8u64) }
        0x1027a3f8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027a3fcu64 + 4);
        vm.wr(37, 0x1027a3fcu64 + 4);
        Ok(0x1027a3fcu64) }
        0x1027a3fc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027a400u64) } }
        0x1027a400 => { vm.decode(0x28a0412fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a404u64) }
        0x1027a404 => { vm.decode(0x8d0814b0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a408u64) }
        0x1027a408 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a40cu64) }
        0x1027a40c => { vm.decode(0x628e532fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a410u64) }
        0x1027a410 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a414u64) }
        0x1027a414 => { vm.decode(0x6084732fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a418u64) }
        0x1027a418 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a41cu64) }
        0x1027a41c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027a420u64 + 4);
        vm.wr(37, 0x1027a420u64 + 4);
        Ok(0x1027a420u64) }
        0x1027a420 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a424u64) }
        0x1027a424 => { vm.decode(0x4088536fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a428u64) }
        0x1027a428 => { vm.decode(0x2820402fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a42cu64) }
        0x1027a42c => { vm.decode(0xa8ce3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a430u64) }
        0x1027a430 => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a434u64) }
        0x1027a434 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a438u64) }
        0x1027a438 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a43cu64) }
        0x1027a43c => { vm.decode(0x6080736fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a440u64) }
        0x1027a440 => { vm.decode(0xe108875u32); let d = 25i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a444u64) }
        0x1027a444 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027a448u64 + 4);
        vm.wr(37, 0x1027a448u64 + 4);
        Ok(0x1027a448u64) }
        0x1027a448 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a44cu64) }
        0x1027a44c => { vm.decode(0x2820402fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a450u64) }
        0x1027a450 => { vm.decode(0xa8ce3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a454u64) }
        0x1027a454 => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a458u64) }
        0x1027a458 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a45cu64) }
        0x1027a45c => { vm.decode(0x2086732fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a460u64) }
        0x1027a460 => { vm.decode(0xe108875u32); let d = 25i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a464u64) }
        0x1027a464 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027a468u64 + 4);
        vm.wr(37, 0x1027a468u64 + 4);
        Ok(0x1027a468u64) }
        0x1027a468 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a46cu64) }
        0x1027a46c => { vm.decode(0x4f104091u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2052i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027a470u64) }
        0x1027a470 => { vm.decode(0xc2632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 2056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a474u64) }
        0x1027a474 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027a478u64) }
        0x1027a478 => { vm.decode(0x2500c375u32); let d = 1i16; let s = 5i16; let s2 = 3i16; let imm = 0i64;
        let v = ((vm.ld(s) as i64) < (vm.ld(s2) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1027a47cu64) }
        0x1027a47c => { vm.decode(0x82020071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 13i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1027a480u64).wrapping_add((imm * 4) as u64) } else { 0x1027a480u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027a480u64) }
        0x1027a480 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027a484u64) }
        0x1027a484 => { vm.decode(0x2904801du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 20524i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1027a488u64) }
        0x1027a48c => { vm.decode(0x2000103eu32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1027a490u64) }
        0x1027a490 => { vm.decode(0x4558bb5u32); let d = 6i16; let s = 2i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1027a494u64) }
        0x1027a494 => { vm.decode(0x40412835u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a498u64) }
        0x1027a498 => { vm.decode(0x20258d35u32); let d = 6i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1027a49cu64) }
        0x1027a49c => { vm.decode(0x2004a835u32); let d = 1i16; let s = 6i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a4a0u64) }
        0x1027a4a0 => { vm.decode(0x40480835u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a4a4u64) }
        0x1027a4a4 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027a4a8u64) }
        0x1027a4a8 => { vm.decode(0x2500c375u32); let d = 1i16; let s = 5i16; let s2 = 3i16; let imm = 0i64;
        let v = ((vm.ld(s) as i64) < (vm.ld(s2) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1027a4acu64) }
        0x1027a4ac => { vm.decode(0x848a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027a4b0u64).wrapping_add((imm * 4) as u64) } else { 0x1027a4b0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027a4b0u64) }
        0x1027a4b0 => { vm.decode(0x2010099u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a4b4u64) }
        0x1027a4b4 => { vm.decode(0x80816du32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1027a4b8u64) }
        0x1027a4b8 => { vm.decode(0x779c34a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -12i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027a4bcu64).wrapping_add((imm * 4) as u64) } else { 0x1027a4bcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027a4bcu64) }
        0x1027a4bc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027a4c0u64) } }
        0x1027a4c0 => { vm.decode(0x2000103eu32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1027a4c4u64) }
        0x1027a4c4 => { vm.decode(0x445cbb5u32); let d = 6i16; let s = 2i16; let s2 = -1i16; let imm = 11i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1027a4c8u64) }
        0x1027a4c8 => { vm.decode(0x8d875u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a4ccu64) }
        0x1027a4cc => { vm.decode(0x30058d35u32); let d = 6i16; let s = 2i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1027a4d0u64) }
        0x1027a4d0 => { vm.decode(0x2004a835u32); let d = 1i16; let s = 6i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a4d4u64) }
        0x1027a4d4 => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a4d8u64) }
        0x1027a4d8 => { vm.decode(0x10109b5u32); let d = 2i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x1027a4dcu64) }
        0x1027a4dc => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027a4e0u64) }
        0x1027a4e0 => { vm.decode(0x2500c375u32); let d = 1i16; let s = 5i16; let s2 = 3i16; let imm = 0i64;
        let v = ((vm.ld(s) as i64) < (vm.ld(s2) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1027a4e4u64) }
        0x1027a4e4 => { vm.decode(0x27f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -13i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1027a4e8u64).wrapping_add((imm * 4) as u64) } else { 0x1027a4e8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027a4e8u64) }
        0x1027a4e8 => { vm.decode(0x2010099u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a4ecu64) }
        0x1027a4ec => { vm.decode(0x6a102973u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1027a4f0u64) }
        0x1027a4f0 => { vm.decode(0x6080536fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a4f4u64) }
        0x1027a4f4 => { vm.decode(0x2204612fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a4f8u64) }
        0x1027a4f8 => { vm.decode(0x88cc3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a4fcu64) }
        0x1027a4fc => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a500u64) }
        0x1027a500 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a504u64) }
        0x1027a504 => { vm.decode(0x286532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a508u64) }
        0x1027a508 => { vm.decode(0x608a736fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a50cu64) }
        0x1027a50c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a510u64) }
        0x1027a510 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027a514u64 + 4);
        vm.wr(37, 0x1027a514u64 + 4);
        Ok(0x1027a514u64) }
        0x1027a514 => { vm.decode(0x62198219u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a518u64) }
        0x1027a518 => { vm.decode(0xbc0fedf6u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027a51cu64) }
        0x1027a51c => { vm.decode(0x66398419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a520u64) }
        0x1027a520 => { vm.decode(0x8ce3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a524u64) }
        0x1027a524 => { vm.decode(0x28802fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a528u64) }
        0x1027a528 => { vm.decode(0x3053862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027a52cu64) }
        0x1027a52c => { vm.decode(0x29081d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a530u64) }
        0x1027a530 => { vm.decode(0x4088732fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1032i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a534u64) }
        0x1027a534 => { vm.decode(0xa9001d30u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a538u64) }
        0x1027a538 => { vm.decode(0x2800c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a53cu64) }
        0x1027a53c => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a540u64) }
        0x1027a540 => { vm.decode(0x2b0915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a544u64) }
        0x1027a544 => { vm.decode(0x3b0114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a548u64) }
        0x1027a548 => { vm.decode(0x6a80c3efu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a54cu64) }
        0x1027a54c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a550u64) }
        0x1027a550 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027a554u64 + 4);
        vm.wr(37, 0x1027a554u64 + 4);
        Ok(0x1027a554u64) }
        0x1027a554 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a558u64) }
        0x1027a558 => { vm.decode(0xa9001d30u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a55cu64) }
        0x1027a55c => { vm.decode(0xf144a51u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027a560u64) }
        0x1027a560 => { vm.decode(0x82022031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 11i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1027a564u64).wrapping_add((imm * 4) as u64) } else { 0x1027a564u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027a564u64) }
        0x1027a564 => { vm.decode(0x3053862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027a568u64) }
        0x1027a568 => { vm.decode(0x2080536fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a56cu64) }
        0x1027a56c => { vm.decode(0x2800a12fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a570u64) }
        0x1027a570 => { vm.decode(0x88f32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a574u64) }
        0x1027a574 => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a578u64) }
        0x1027a578 => { vm.decode(0x2b0915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a57cu64) }
        0x1027a57c => { vm.decode(0x3b0114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a580u64) }
        0x1027a580 => { vm.decode(0x66398419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a584u64) }
        0x1027a584 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a588u64) }
        0x1027a588 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027a58cu64 + 4);
        vm.wr(37, 0x1027a58cu64 + 4);
        Ok(0x1027a58cu64) }
        0x1027a58c => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a590u64) }
        0x1027a590 => { vm.decode(0x608ee3efu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a594u64) }
        0x1027a594 => { vm.decode(0xdd48011u32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027a598u64) }
        0x1027a598 => { vm.decode(0x8000236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027a59cu64) }
        0x1027a59c => { vm.decode(0x240a171u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 198i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1027a5a0u64).wrapping_add((imm * 4) as u64) } else { 0x1027a5a0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027a5a0u64) }
        0x1027a5a0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027a5a4u64) } }
        0x1027a5a4 => { vm.decode(0x6080536fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a5a8u64) }
        0x1027a8b8 => { vm.decode(0x20a0c12fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a8bcu64) }
        0x1027a8bc => { vm.decode(0xbc0fcff6u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027a8c0u64) }
        0x1027a8c0 => { vm.decode(0x2057862u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027a8c4u64) }
        0x1027a8c4 => { vm.decode(0x39081c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a8c8u64) }
        0x1027a8c8 => { vm.decode(0x4088732fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1032i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a8ccu64) }
        0x1027a8cc => { vm.decode(0xa9001d30u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a8d0u64) }
        0x1027a8d0 => { vm.decode(0x88536fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a8d4u64) }
        0x1027a8d4 => { vm.decode(0x828802fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a8d8u64) }
        0x1027a8d8 => { vm.decode(0x40048019u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a8dcu64) }
        0x1027a8dc => { vm.decode(0x3b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a8e0u64) }
        0x1027a8e0 => { vm.decode(0x2b0115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a8e4u64) }
        0x1027a8e4 => { vm.decode(0x66398419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a8e8u64) }
        0x1027a8e8 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a8ecu64) }
        0x1027a8ec => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027a8f0u64 + 4);
        vm.wr(37, 0x1027a8f0u64 + 4);
        Ok(0x1027a8f0u64) }
        0x1027a8f0 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a8f4u64) }
        0x1027a8f4 => { vm.decode(0xa9001d30u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a8f8u64) }
        0x1027a8f8 => { vm.decode(0xf144a51u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027a8fcu64) }
        0x1027a8fc => { vm.decode(0x6848aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 687i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027a900u64).wrapping_add((imm * 4) as u64) } else { 0x1027a900u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027a900u64) }
        0x1027a900 => { vm.decode(0x3053862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027a904u64) }
        0x1027a904 => { vm.decode(0xdd48011u32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027a908u64) }
        0x1027a910 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027a914u64) }
        0x1027a914 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027a918u64) }
        0x1027a918 => { vm.decode(0x882c3efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a91cu64) }
        0x1027a91c => { vm.decode(0x3020406u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 2i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1027a920u64) }
        0x1027a920 => { vm.decode(0x82020071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 13i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1027a924u64).wrapping_add((imm * 4) as u64) } else { 0x1027a924u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027a924u64) }
        0x1027a924 => { vm.decode(0x600d9519u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2120i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a928u64) }
        0x1027a928 => { vm.decode(0x8424841du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 21712i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1027a92cu64) }
        0x1027a930 => { vm.decode(0x2008023eu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1027a934u64) }
        0x1027a934 => { vm.decode(0x4178bb5u32); let d = 5i16; let s = 2i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1027a938u64) }
        0x1027a938 => { vm.decode(0x60012835u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a93cu64) }
        0x1027a93c => { vm.decode(0x20348d35u32); let d = 5i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1027a940u64) }
        0x1027a940 => { vm.decode(0x6000a835u32); let d = 1i16; let s = 5i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a944u64) }
        0x1027a944 => { vm.decode(0x40480835u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a948u64) }
        0x1027a948 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027a94cu64) }
        0x1027a94c => { vm.decode(0x3020406u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 2i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1027a950u64) }
        0x1027a950 => { vm.decode(0x848a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027a954u64).wrapping_add((imm * 4) as u64) } else { 0x1027a954u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027a954u64) }
        0x1027a954 => { vm.decode(0x40809099u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a958u64) }
        0x1027a958 => { vm.decode(0x80416du32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1027a95cu64) }
        0x1027a95c => { vm.decode(0x779c34a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -12i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027a960u64).wrapping_add((imm * 4) as u64) } else { 0x1027a960u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027a960u64) }
        0x1027a960 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027a964u64) } }
        0x1027a964 => { vm.decode(0x2008023eu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1027a968u64) }
        0x1027a968 => { vm.decode(0x407cbb5u32); let d = 5i16; let s = 2i16; let s2 = -1i16; let imm = 11i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1027a96cu64) }
        0x1027a96c => { vm.decode(0x408c875u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a970u64) }
        0x1027a970 => { vm.decode(0x30148d35u32); let d = 5i16; let s = 2i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1027a974u64) }
        0x1027a974 => { vm.decode(0x6000a835u32); let d = 1i16; let s = 5i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a978u64) }
        0x1027a978 => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027a97cu64) }
        0x1027a97c => { vm.decode(0x10109b5u32); let d = 2i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x1027a980u64) }
        0x1027a980 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027a984u64) }
        0x1027a984 => { vm.decode(0x3020406u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 2i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1027a988u64) }
        0x1027a988 => { vm.decode(0x27f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -13i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1027a98cu64).wrapping_add((imm * 4) as u64) } else { 0x1027a98cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027a98cu64) }
        0x1027a98c => { vm.decode(0x40809099u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a990u64) }
        0x1027a990 => { vm.decode(0xe31028f3u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2195i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1027a994u64) }
        0x1027a994 => { vm.decode(0x60100d35u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1027a998u64) }
        0x1027a998 => { vm.decode(0x431228f3u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2194i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1027a99cu64) }
        0x1027a99c => { vm.decode(0x20120d35u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1027a9a0u64) }
        0x1027a9a0 => { vm.decode(0xc3122873u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2193i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1027a9a4u64) }
        0x1027a9a4 => { vm.decode(0x60120d35u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1027a9a8u64) }
        0x1027a9a8 => { vm.decode(0x43122873u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1027a9acu64) }
        0x1027a9ac => { vm.decode(0x2080536fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a9b0u64) }
        0x1027a9b0 => { vm.decode(0x2004016fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a9b4u64) }
        0x1027a9b4 => { vm.decode(0x45144ad1u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1588i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027a9b8u64) }
        0x1027a9b8 => { vm.decode(0x4a1d5c62u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3916i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027a9bcu64) }
        0x1027a9bc => { vm.decode(0x102036u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027a9c0u64) }
        0x1027a9c0 => { vm.decode(0x52184b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 3912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1027a9c4u64) }
        0x1027a9c4 => { vm.decode(0xa8ce3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a9c8u64) }
        0x1027a9c8 => { vm.decode(0x2b211d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a9ccu64) }
        0x1027a9cc => { vm.decode(0x3b191cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027a9d0u64) }
        0x1027a9d0 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a9d4u64) }
        0x1027a9d4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027a9d8u64 + 4);
        vm.wr(37, 0x1027a9d8u64 + 4);
        Ok(0x1027a9d8u64) }
        0x1027a9d8 => { vm.decode(0x667d8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3896i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a9dcu64) }
        0x1027a9dc => { vm.decode(0x1019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027a9e0u64) }
        0x1027a9e0 => { vm.decode(0x4d144ad1u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1588i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027a9e4u64) }
        0x1027a9e4 => { vm.decode(0xbe2fddf6u32); let d = 4i16; let s = 3i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027a9e8u64) }
        0x1027a9e8 => { vm.decode(0x806bafu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027a9ecu64) }
        0x1027a9ec => { vm.decode(0x500e375u32); let d = 1i16; let s = 17i16; let s2 = 3i16; let imm = 0i64;
        let v = ((vm.ld(s) as i64) < (vm.ld(s2) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1027a9f0u64) }
        0x1027a9f0 => { vm.decode(0x8c8027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 21i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027a9f4u64).wrapping_add((imm * 4) as u64) } else { 0x1027a9f4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027a9f4u64) }
        0x1027a9f4 => { vm.decode(0xb60dedf6u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = -4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027a9f8u64) }
        0x1027a9f8 => { vm.decode(0x7818c7b5u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 31i64;
        let v = ((vm.ld(s) as i32) >> (imm as u32)) as i64;
        vm.st(d, v as u64);
        Ok(0x1027a9fcu64) }
        0x1027a9fc => { vm.decode(0x50b20d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 30i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1027aa00u64) }
        0x1027aa00 => { vm.decode(0x44090fb5u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1027aa04u64) }
        0x1027aa04 => { vm.decode(0x20317f5u32); let d = 1i16; let s = 5i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027aa08u64) }
        0x1027aa08 => { vm.decode(0xa8008b35u32); let d = 1i16; let s = 17i16; let s2 = 1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_sub(vm.ld(s2) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1027aa0cu64) }
        0x1027aa0c => { vm.decode(0x640bb5u32); let d = 6i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1027aa10u64) }
        0x1027aa10 => { vm.decode(0x228e73afu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 1592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027aa14u64) }
        0x1027aa14 => { vm.decode(0x6a2224f5u32); let d = 6i16; let s = 6i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027aa18u64) }
        0x1027aa18 => { vm.decode(0x208af3afu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 1800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027aa1cu64) }
        0x1027aa1c => { vm.decode(0x6a4224f5u32); let d = 7i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027aa20u64) }
        0x1027aa20 => { vm.decode(0xa08103eu32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1027aa24u64) }
        0x1027aa24 => { vm.decode(0x804084f5u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027aa28u64) }
        0x1027aa28 => { vm.decode(0xbe4fcdf6u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027aa2cu64) }
        0x1027aa2c => { vm.decode(0x8903036u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027aa30u64) }
        0x1027aa30 => { vm.decode(0x2000023eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1027aa34u64) }
        0x1027aa34 => { vm.decode(0x40412835u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027aa38u64) }
        0x1027aa38 => { vm.decode(0x40220833u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1027aa3cu64) }
        0x1027aa3c => { vm.decode(0x500e375u32); let d = 1i16; let s = 17i16; let s2 = 3i16; let imm = 0i64;
        let v = ((vm.ld(s) as i64) < (vm.ld(s2) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1027aa40u64) }
        0x1027aa40 => { vm.decode(0xa7d30df1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -19i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1027aa44u64).wrapping_add((imm * 4) as u64) } else { 0x1027aa44u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027aa44u64) }
        0x1027aa44 => { vm.decode(0x801099u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027aa48u64) }
        0x1027aa48 => { vm.decode(0x80008099u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027aa4cu64) }
        0x1027aa4c => { vm.decode(0x45b935u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1027aa50u64) }
        0x1027aa50 => { vm.decode(0x190194b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027aa54u64) }
        0x1027aa54 => { vm.decode(0x888736fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027aa58u64) }
        0x1027aa58 => { vm.decode(0x208003eu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1027aa5cu64) }
        0x1027aa5c => { vm.decode(0x505a412du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 63i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1027aa60u64) }
        0x1027aa60 => { vm.decode(0x85402935u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 46i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1027aa64u64) }
        0x1027aa64 => { vm.decode(0x288c536fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027aa68u64) }
        0x1027aa68 => { vm.decode(0x66398419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027aa6cu64) }
        0x1027aa6c => { vm.decode(0xbe0fcdf6u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027aa70u64) }
        0x1027aa70 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027aa74u64) }
        0x1027aa74 => { vm.decode(0x190194b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027aa78u64) }
        0x1027aa78 => { vm.decode(0x208023eu32); let d = 2i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1027aa7cu64) }
        0x1027aa7c => { vm.decode(0x505a412du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 63i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1027aa80u64) }
        0x1027aa80 => { vm.decode(0x81400935u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 40i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1027aa84u64) }
        0x1027aa84 => { vm.decode(0x4408875u32); let d = 1i16; let s = 2i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027aa88u64) }
        0x1027aa88 => { vm.decode(0x190194b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027aa8cu64) }
        0x1027aa8c => { vm.decode(0x20a0c12fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027aa90u64) }
        0x1027aa90 => { vm.decode(0x2453862u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027aa94u64) }
        0x1027aa94 => { vm.decode(0x39081c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027aa98u64) }
        0x1027aa98 => { vm.decode(0x4088732fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1032i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027aa9cu64) }
        0x1027aa9c => { vm.decode(0xa9001d30u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027aaa0u64) }
        0x1027aaa0 => { vm.decode(0x88536fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027aaa4u64) }
        0x1027aaa4 => { vm.decode(0x828802fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027aaa8u64) }
        0x1027aaa8 => { vm.decode(0x40048019u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027aaacu64) }
        0x1027aaac => { vm.decode(0x3b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027aab0u64) }
        0x1027aab0 => { vm.decode(0x2b0115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027aab4u64) }
        0x1027aab4 => { vm.decode(0x6a80c3efu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027aab8u64) }
        0x1027aab8 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027aabcu64) }
        0x1027aabc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027aac0u64 + 4);
        vm.wr(37, 0x1027aac0u64 + 4);
        Ok(0x1027aac0u64) }
        0x1027aac0 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027aac4u64) }
        0x1027aac4 => { vm.decode(0xa9001d30u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027aac8u64) }
        0x1027aac8 => { vm.decode(0xf144a51u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027aaccu64) }
        0x1027aacc => { vm.decode(0x82022031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 11i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1027aad0u64).wrapping_add((imm * 4) as u64) } else { 0x1027aad0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027aad0u64) }
        0x1027aad0 => { vm.decode(0x3053862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027aad4u64) }
        0x1027aad4 => { vm.decode(0x2080536fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027aad8u64) }
        0x1027aad8 => { vm.decode(0x2800a12fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027aadcu64) }
        0x1027aadc => { vm.decode(0x88f32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027aae0u64) }
        0x1027aae0 => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027aae4u64) }
        0x1027aae4 => { vm.decode(0x2b0915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027aae8u64) }
        0x1027aae8 => { vm.decode(0x3b0114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027aaecu64) }
        0x1027aaec => { vm.decode(0x66398419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027aaf0u64) }
        0x1027aaf0 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027aaf4u64) }
        0x1027aaf4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027aaf8u64 + 4);
        vm.wr(37, 0x1027aaf8u64 + 4);
        Ok(0x1027aaf8u64) }
        0x1027aaf8 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027aafcu64) }
        0x1027aafc => { vm.decode(0xdd48011u32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027ab00u64) }
        0x1027ab00 => { vm.decode(0x8002236u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027ab04u64) }
        0x1027ab04 => { vm.decode(0x208a071u32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1027ab08u64).wrapping_add((imm * 4) as u64) } else { 0x1027ab08u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027ab08u64) }
        0x1027ab08 => { vm.decode(0x8cc432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ab0cu64) }
        0x1027ab0c => { vm.decode(0x8099u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027ab10u64) }
        0x1027ab20 => { vm.decode(0x8099u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027ab24u64) }
        0x1027ab24 => { vm.decode(0x9935u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1027ab28u64) }
        0x1027ab28 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027ab2cu64) }
        0x1027ab2c => { vm.decode(0x190194b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027ab30u64) }
        0x1027ab30 => { vm.decode(0x2080536fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ab34u64) }
        0x1027ab34 => { vm.decode(0x2a00012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ab38u64) }
        0x1027ab38 => { vm.decode(0x3b1914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027ab3cu64) }
        0x1027ab3c => { vm.decode(0xa82e3efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ab40u64) }
        0x1027ab40 => { vm.decode(0x6080736fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ab44u64) }
        0x1027ab44 => { vm.decode(0xe108875u32); let d = 25i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027ab48u64) }
        0x1027ab48 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027ab4cu64 + 4);
        vm.wr(37, 0x1027ab4cu64 + 4);
        Ok(0x1027ab4cu64) }
        0x1027ab4c => { vm.decode(0x667d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3640i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027ab50u64) }
        0x1027ab50 => { vm.decode(0x71411617u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = u32::from_le_bytes(b.try_into().unwrap());
        vm.st(d, v as u64);
        Ok(0x1027ab54u64) }
        0x1027ab54 => { vm.decode(0x8cc432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ab58u64) }
        0x1027ab58 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027ab5cu64) }
        0x1027ab5c => { vm.decode(0x4518c851u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027ab60u64) }
        0x1027ab60 => { vm.decode(0x308236u32); let d = 18i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027ab64u64) }
        0x1027ab64 => { vm.decode(0x667d8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3896i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027ab68u64) }
        0x1027ab68 => { vm.decode(0xe108875u32); let d = 25i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027ab6cu64) }
        0x1027ab6c => { vm.decode(0x190194b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027ab70u64) }
        0x1027ab70 => { vm.decode(0x6080536fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ab74u64) }
        0x1027ab74 => { vm.decode(0x2004416fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ab78u64) }
        0x1027ab78 => { vm.decode(0x4a1d5d62u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 3916i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027ab7cu64) }
        0x1027ab7c => { vm.decode(0x52184b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 3912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1027ab80u64) }
        0x1027ab80 => { vm.decode(0x8cc3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ab84u64) }
        0x1027ab84 => { vm.decode(0x2b211d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027ab88u64) }
        0x1027ab88 => { vm.decode(0x3b191cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027ab8cu64) }
        0x1027ab8c => { vm.decode(0x882c3efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ab90u64) }
        0x1027ab90 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027ab94u64 + 4);
        vm.wr(37, 0x1027ab94u64 + 4);
        Ok(0x1027ab94u64) }
        0x1027ab94 => { vm.decode(0x2102236u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027ab98u64) }
        0x1027ab98 => { vm.decode(0x66698459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3856i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027ab9cu64) }
        0x1027ab9c => { vm.decode(0x390854b0u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027aba0u64) }
        0x1027aba0 => { vm.decode(0x60398119u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1120i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027aba4u64) }
        0x1027aba4 => { vm.decode(0x3019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027aba8u64) }
        0x1027aba8 => { vm.decode(0x2008416fu32); let d = 3i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027abacu64) }
        0x1027abac => { vm.decode(0x2b091db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027abb0u64) }
        0x1027abb0 => { vm.decode(0x3b011cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027abb4u64) }
        0x1027abb4 => { vm.decode(0x2a8453efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027abb8u64) }
        0x1027abb8 => { vm.decode(0x3b191c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027abbcu64) }
        0x1027abbc => { vm.decode(0x3b111d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 3856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027abc0u64) }
        0x1027abc0 => { vm.decode(0x6884e3afu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027abc4u64) }
        0x1027abc4 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027abc8u64) }
        0x1027abc8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027abccu64 + 4);
        vm.wr(37, 0x1027abccu64 + 4);
        Ok(0x1027abccu64) }
        0x1027abcc => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027abd0u64) }
        0x1027abd0 => { vm.decode(0x2080536fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027abd4u64) }
        0x1027abd4 => { vm.decode(0x2000016fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027abd8u64) }
        0x1027abd8 => { vm.decode(0xa88732fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027abdcu64) }
        0x1027abdc => { vm.decode(0x2b091db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027abe0u64) }
        0x1027abe0 => { vm.decode(0x88c532fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027abe4u64) }
        0x1027abe4 => { vm.decode(0x2b011db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027abe8u64) }
        0x1027abe8 => { vm.decode(0x3b111c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027abecu64) }
        0x1027abec => { vm.decode(0x2a8453efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027abf0u64) }
        0x1027abf0 => { vm.decode(0x4000a019u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027abf4u64) }
        0x1027abf4 => { vm.decode(0x3b191c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027abf8u64) }
        0x1027abf8 => { vm.decode(0x2086732fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027abfcu64) }
        0x1027abfc => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ac00u64) }
        0x1027ac00 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027ac04u64 + 4);
        vm.wr(37, 0x1027ac04u64 + 4);
        Ok(0x1027ac04u64) }
        0x1027ac04 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027ac08u64) }
        0x1027ac08 => { vm.decode(0xd18cb91u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 668i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027ac0cu64) }
        0x1027ac0c => { vm.decode(0x4518cb11u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027ac10u64) }
        0x1027ac10 => { vm.decode(0x4d184b91u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 660i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027ac14u64) }
        0x1027ac14 => { vm.decode(0x5185a11u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027ac18u64) }
        0x1027ac18 => { vm.decode(0x25186375u32); let d = 1i16; let s = 23i16; let s2 = 18i16; let imm = 0i64;
        let v = ((vm.ld(s) as i64) < (vm.ld(s2) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1027ac1cu64) }
        0x1027ac1c => { vm.decode(0xc0a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 26i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027ac20u64).wrapping_add((imm * 4) as u64) } else { 0x1027ac20u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027ac20u64) }
        0x1027ac20 => { vm.decode(0x1019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027ac24u64) }
        0x1027ac24 => { vm.decode(0x2a8453efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ac28u64) }
        0x1027ac28 => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027ac2cu64) }
        0x1027ac2c => { vm.decode(0x2200023eu32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1027ac30u64) }
        0x1027ac30 => { vm.decode(0x30482835u32); let d = 3i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027ac34u64) }
        0x1027ac34 => { vm.decode(0x20020833u32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1027ac38u64) }
        0x1027ac38 => { vm.decode(0x2a8453efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ac3cu64) }
        0x1027ac3c => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027ac40u64) }
        0x1027ac40 => { vm.decode(0x80b019u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027ac44u64) }
        0x1027ac44 => { vm.decode(0x2f0b236u32); let d = 23i16; let s = 23i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027ac48u64) }
        0x1027ac48 => { vm.decode(0x80002beu32); let d = 4i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1027ac4cu64) }
        0x1027ac4c => { vm.decode(0x41058835u32); let d = 4i16; let s = 7i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027ac50u64) }
        0x1027ac50 => { vm.decode(0x80400833u32); let d = 4i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1027ac54u64) }
        0x1027ac54 => { vm.decode(0x2a8453efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ac58u64) }
        0x1027ac58 => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027ac5cu64) }
        0x1027ac5c => { vm.decode(0x1800023eu32); let d = 4i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1027ac60u64) }
        0x1027ac60 => { vm.decode(0x41018835u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027ac64u64) }
        0x1027ac64 => { vm.decode(0x4008b3u32); let d = 4i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1027ac68u64) }
        0x1027ac68 => { vm.decode(0x2a8453efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ac6cu64) }
        0x1027ac6c => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027ac70u64) }
        0x1027ac70 => { vm.decode(0x120002beu32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1027ac74u64) }
        0x1027ac74 => { vm.decode(0x4c8835u32); let d = 2i16; let s = 6i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027ac78u64) }
        0x1027ac78 => { vm.decode(0xa00008b3u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1027ac7cu64) }
        0x1027ac7c => { vm.decode(0x25186375u32); let d = 1i16; let s = 23i16; let s2 = 18i16; let imm = 0i64;
        let v = ((vm.ld(s) as i64) < (vm.ld(s2) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1027ac80u64) }
        0x1027ac80 => { vm.decode(0xa7d10db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -24i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1027ac84u64).wrapping_add((imm * 4) as u64) } else { 0x1027ac84u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027ac84u64) }
        0x1027ac84 => { vm.decode(0x44001875u32); let d = 2i16; let s = 0i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027ac88u64) }
        0x1027ac88 => { vm.decode(0xd1062d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1796i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027ac8cu64) }
        0x1027ac8c => { vm.decode(0x4088536fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ac90u64) }
        0x1027ac90 => { vm.decode(0x2c406fu32); let d = 2i16; let s = 18i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ac94u64) }
        0x1027ac94 => { vm.decode(0x8033036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 11i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027ac98u64) }
        0x1027ac98 => { vm.decode(0x23151822u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027ac9cu64) }
        0x1027ac9c => { vm.decode(0x4b1d1c62u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3916i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027aca0u64) }
        0x1027aca0 => { vm.decode(0x52184b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 3912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1027aca4u64) }
        0x1027aca4 => { vm.decode(0x2a80e3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027aca8u64) }
        0x1027aca8 => { vm.decode(0x3b211c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027acacu64) }
        0x1027acac => { vm.decode(0x2b191db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027acb0u64) }
        0x1027acb0 => { vm.decode(0x667d8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3896i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027acb4u64) }
        0x1027acb4 => { vm.decode(0x39085430u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1032i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027acb8u64) }
        0x1027acb8 => { vm.decode(0x882c3efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027acbcu64) }
        0x1027acbc => { vm.decode(0x608a736fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027acc0u64) }
        0x1027acc0 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027acc4u64) }
        0x1027acc4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027acc8u64 + 4);
        vm.wr(37, 0x1027acc8u64 + 4);
        Ok(0x1027acc8u64) }
        0x1027acc8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027acccu64) } }
        0x1027accc => { vm.decode(0x2020406fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027acd0u64) }
        0x1027acd0 => { vm.decode(0x66698459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3856i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027acd4u64) }
        0x1027acd4 => { vm.decode(0x39105430u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027acd8u64) }
        0x1027acd8 => { vm.decode(0x1099u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027acdcu64) }
        0x1027acdc => { vm.decode(0x2b091db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027ace0u64) }
        0x1027ace0 => { vm.decode(0x8843afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ace4u64) }
        0x1027ace4 => { vm.decode(0x2b011db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027ace8u64) }
        0x1027ace8 => { vm.decode(0xa8873efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027acecu64) }
        0x1027acec => { vm.decode(0x290015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027acf0u64) }
        0x1027acf0 => { vm.decode(0x2b191d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027acf4u64) }
        0x1027acf4 => { vm.decode(0x3b111c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027acf8u64) }
        0x1027acf8 => { vm.decode(0x6884e3afu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027acfcu64) }
        0x1027acfc => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027ad00u64) }
        0x1027ad00 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027ad04u64 + 4);
        vm.wr(37, 0x1027ad04u64 + 4);
        Ok(0x1027ad04u64) }
        0x1027ad04 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027ad08u64) }
        0x1027ad08 => { vm.decode(0x601b8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2208i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027ad0cu64) }
        0x1027ad0c => { vm.decode(0x41019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027ad10u64) }
        0x1027ad10 => { vm.decode(0x2028406fu32); let d = 3i16; let s = 18i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ad14u64) }
        0x1027ad14 => { vm.decode(0x2b091db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027ad18u64) }
        0x1027ad18 => { vm.decode(0x3b011cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027ad1cu64) }
        0x1027ad1c => { vm.decode(0x2084532fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ad20u64) }
        0x1027ad20 => { vm.decode(0x40008099u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027ad24u64) }
        0x1027ad24 => { vm.decode(0x3b191c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027ad28u64) }
        0x1027ad28 => { vm.decode(0x3b111d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 3856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027ad2cu64) }
        0x1027ad2c => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027ad30u64) }
        0x1027ad30 => { vm.decode(0x6280532fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 1040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ad34u64) }
        0x1027ad34 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027ad38u64) }
        0x1027ad38 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027ad3cu64 + 4);
        vm.wr(37, 0x1027ad3cu64 + 4);
        Ok(0x1027ad3cu64) }
        0x1027ad3c => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027ad40u64) }
        0x1027ad40 => { vm.decode(0x2084532fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ad44u64) }
        0x1027ad44 => { vm.decode(0x40049099u32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 9i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027ad48u64) }
        0x1027ad48 => { vm.decode(0x390015b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027ad4cu64) }
        0x1027ad4c => { vm.decode(0x2020406fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ad50u64) }
        0x1027ad50 => { vm.decode(0x451062d1u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1796i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027ad54u64) }
        0x1027ad54 => { vm.decode(0x2b091db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027ad58u64) }
        0x1027ad58 => { vm.decode(0x88f3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ad5cu64) }
        0x1027ad5c => { vm.decode(0x2b011db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027ad60u64) }
        0x1027ad60 => { vm.decode(0x3b191d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 3864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027ad64u64) }
        0x1027ad64 => { vm.decode(0x3b111c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027ad68u64) }
        0x1027ad68 => { vm.decode(0x488ce3afu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ad6cu64) }
        0x1027ad6c => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027ad70u64) }
        0x1027ad70 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027ad74u64) }
        0x1027ad74 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027ad78u64 + 4);
        vm.wr(37, 0x1027ad78u64 + 4);
        Ok(0x1027ad78u64) }
        0x1027ad78 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027ad7cu64) }
        0x1027ad7c => { vm.decode(0x600d8519u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2120i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027ad80u64) }
        0x1027ad80 => { vm.decode(0x80001019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027ad84u64) }
        0x1027ad84 => { vm.decode(0x6080536fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ad88u64) }
        0x1027ad88 => { vm.decode(0x2008416fu32); let d = 3i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ad8cu64) }
        0x1027ad8c => { vm.decode(0x2b091db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027ad90u64) }
        0x1027ad90 => { vm.decode(0x3b011cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027ad94u64) }
        0x1027ad94 => { vm.decode(0x3b111d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 3856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027ad98u64) }
        0x1027ad98 => { vm.decode(0xd1062d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1796i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027ad9cu64) }
        0x1027ad9c => { vm.decode(0x8c532fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ada0u64) }
        0x1027ada0 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027ada4u64) }
        0x1027ada4 => { vm.decode(0x3b191c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027ada8u64) }
        0x1027ada8 => { vm.decode(0x2282532fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027adacu64) }
        0x1027adac => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027adb0u64) }
        0x1027adb0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027adb4u64 + 4);
        vm.wr(37, 0x1027adb4u64 + 4);
        Ok(0x1027adb4u64) }
        0x1027adb4 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027adb8u64) }
        0x1027adb8 => { vm.decode(0x803a436u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 31i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027adbcu64) }
        0x1027adbc => { vm.decode(0xa8873efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027adc0u64) }
        0x1027adc0 => { vm.decode(0x290015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027adc4u64) }
        0x1027adc4 => { vm.decode(0xc416fu32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027adc8u64) }
        0x1027adc8 => { vm.decode(0x4b1d1c62u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3916i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027adccu64) }
        0x1027adcc => { vm.decode(0x52184b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 3912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1027add0u64) }
        0x1027add0 => { vm.decode(0x2280e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027add4u64) }
        0x1027add4 => { vm.decode(0x3b211c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027add8u64) }
        0x1027add8 => { vm.decode(0x2b191db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027addcu64) }
        0x1027addc => { vm.decode(0x882c3efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ade0u64) }
        0x1027ade0 => { vm.decode(0x2082732fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1032i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ade4u64) }
        0x1027ade4 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027ade8u64) }
        0x1027ade8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027adecu64 + 4);
        vm.wr(37, 0x1027adecu64 + 4);
        Ok(0x1027adecu64) }
        0x1027adec => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027adf0u64) } }
        0x1027adf0 => { vm.decode(0xd14c051u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027adf4u64) }
        0x1027adf4 => { vm.decode(0x30000200u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1027adf8u64) }
        0x1027adf8 => { vm.decode(0x83022131u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 107i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1027adfcu64).wrapping_add((imm * 4) as u64) } else { 0x1027adfcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027adfcu64) }
        0x1027adfc => { vm.decode(0x3053822u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027ae00u64) }
        0x1027ae00 => { vm.decode(0x88853efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ae04u64) }
        0x1027ae04 => { vm.decode(0x2236u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027ae08u64) }
        0x1027ae08 => { vm.decode(0x202a036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027ae0cu64) }
        0x1027ae0c => { vm.decode(0xa028236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027ae10u64) }
        0x1027ae10 => { vm.decode(0xa022236u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027ae14u64) }
        0x1027ae14 => { vm.decode(0x9000036u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027ae18u64) }
        0x1027ae18 => { vm.decode(0x1022036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027ae1cu64) }
        0x1027ae1c => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027ae20u64) }
        0x1027ae20 => { vm.decode(0xb0102640u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 31i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1027ae24u64) }
        0x1027ae24 => { vm.decode(0x82022031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 11i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1027ae28u64).wrapping_add((imm * 4) as u64) } else { 0x1027ae28u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027ae28u64) }
        0x1027ae28 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027ae2cu64) }
        0x1027ae2c => { vm.decode(0x89a4c01du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 23272i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1027ae30u64) }
        0x1027ae40 => { vm.decode(0x801099u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027ae44u64) }
        0x1027ae44 => { vm.decode(0x8203236u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027ae48u64) }
        0x1027ae48 => { vm.decode(0xb0102640u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 31i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1027ae4cu64) }
        0x1027ae4c => { vm.decode(0x108c0827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 86i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027ae50u64).wrapping_add((imm * 4) as u64) } else { 0x1027ae50u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027ae50u64) }
        0x1027ae50 => { vm.decode(0x42a04875u32); let d = 14i16; let s = 0i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027ae54u64) }
        0x1027ae54 => { vm.decode(0x4020b036u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027ae58u64) }
        0x1027ae58 => { vm.decode(0xa008036u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027ae5cu64) }
        0x1027ae5c => { vm.decode(0x82cae849u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 1908483i64;
        let src = vm.ld(s) as u32;
        let dst = vm.ld(d) as u32;
        let v = (src << 3) | (dst & 7);
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1027ae60u64) }
        0x1027ae60 => { vm.decode(0xf82d4909u32); let d = 12i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1027ae64u64) }
        0x1027ae64 => { vm.decode(0x8c5b2fu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ae68u64) }
        0x1027ae68 => { vm.decode(0x2a294f5u32); let d = 12i16; let s = 12i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027ae6cu64) }
        0x1027ae6c => { vm.decode(0x208a0a2fu32); let d = 15i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027ae70u64) }
        0x1027ae70 => { vm.decode(0x14852175u32); let d = 1i16; let s = 15i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shr(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x1027ae74u64) }
        0x1027ae74 => { vm.decode(0x6a01875u32); let d = 12i16; let s = 0i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027ae78u64) }
        0x1027ae78 => { vm.decode(0xadaf849u32); let d = 12i16; let s = 5i16; let s2 = -1i16; let imm = 1908483i64;
        let src = vm.ld(s) as u32;
        let dst = vm.ld(d) as u32;
        let v = (src << 3) | (dst & 7);
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1027ae7cu64) }
        0x1027ae7c => { vm.decode(0x1e80e175u32); let d = 24i16; let s = 15i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shr(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x1027ae80u64) }
        0x1027ae80 => { vm.decode(0x4b209036u32); let d = 12i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027ae84u64) }
        0x1027ae84 => { vm.decode(0xaeaf849u32); let d = 12i16; let s = 6i16; let s2 = -1i16; let imm = 1908483i64;
        let src = vm.ld(s) as u32;
        let dst = vm.ld(d) as u32;
        let v = (src << 3) | (dst & 7);
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1027ae88u64) }
        0x1027ae88 => { vm.decode(0x4132b036u32); let d = 25i16; let s = 11i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027ae8cu64) }
        0x1027ae8c => { vm.decode(0x1e846175u32); let d = 17i16; let s = 15i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shr(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x1027ae90u64) }
        0x1027ae90 => { vm.decode(0xcafae849u32); let d = 25i16; let s = 7i16; let s2 = -1i16; let imm = 1908483i64;
        let src = vm.ld(s) as u32;
        let dst = vm.ld(d) as u32;
        let v = (src << 3) | (dst & 7);
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1027ae94u64) }
        0x1027ae94 => { vm.decode(0x4a323236u32); let d = 23i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027ae98u64) }
        0x1027ae98 => { vm.decode(0x300a036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027ae9cu64) }
        0x1027ae9c => { vm.decode(0x820bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1027aea0u64) }
        0x1027aea0 => { vm.decode(0x43610036u32); let d = 12i16; let s = 14i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027aea4u64) }
        0x1027aea4 => { vm.decode(0x43702636u32); let d = 31i16; let s = 14i16; let s2 = -1i16; let imm = 16i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027aea8u64) }
        0x1027aea8 => { vm.decode(0x4b201236u32); let d = 14i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027aeacu64) }
        0x1027aeac => { vm.decode(0xfc3fdff6u32); let d = 18i16; let s = 11i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027aeb0u64) }
        0x1027aeb0 => { vm.decode(0xe2def849u32); let d = 23i16; let s = 13i16; let s2 = -1i16; let imm = 1908483i64;
        let src = vm.ld(s) as u32;
        let dst = vm.ld(d) as u32;
        let v = (src << 3) | (dst & 7);
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1027aeb4u64) }
        0x1027aeb4 => { vm.decode(0xc1002bb5u32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1027aeb8u64) }
        0x1027aeb8 => { vm.decode(0x3685e175u32); let d = 25i16; let s = 15i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shr(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x1027aebcu64) }
        0x1027aebc => { vm.decode(0x1812516du32); let d = 10i16; let s = 14i16; let s2 = -1i16; let imm = 7i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1027aec0u64) }
        0x1027aec0 => { vm.decode(0x514c151u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027aec4u64) }
        0x1027aec4 => { vm.decode(0x6ba5ae35u32); let d = 31i16; let s = 4i16; let s2 = 31i16; let imm = 0i64;
        let v = (vm.ld(s) < vm.ld(s2)) as u64;
        vm.st(d, v);
        Ok(0x1027aec8u64) }
        0x1027aec8 => { vm.decode(0x80812du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1027aeccu64) }
        0x1027aecc => { vm.decode(0x4920bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1027aed0u64) }
        0x1027aed0 => { vm.decode(0x62c6e849u32); let d = 18i16; let s = 8i16; let s2 = -1i16; let imm = 1908483i64;
        let src = vm.ld(s) as u32;
        let dst = vm.ld(d) as u32;
        let v = (src << 3) | (dst & 7);
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1027aed4u64) }
        0x1027aed4 => { vm.decode(0xb6a02175u32); let d = 18i16; let s = 15i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shr(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x1027aed8u64) }
        0x1027aed8 => { vm.decode(0xc0410bb5u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1027aedcu64) }
        0x1027aedc => { vm.decode(0x3041adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1027aee0u64) }
        0x1027aee0 => { vm.decode(0xc0510bb5u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1027aee4u64) }
        0x1027aee4 => { vm.decode(0x189875u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027aee8u64) }
        0x1027aee8 => { vm.decode(0xbea12175u32); let d = 18i16; let s = 15i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shr(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x1027aeecu64) }
        0x1027aeec => { vm.decode(0xc0410bb5u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1027aef0u64) }
        0x1027aef0 => { vm.decode(0x3041adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1027aef4u64) }
        0x1027aef4 => { vm.decode(0xc0418bb5u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1027aef8u64) }
        0x1027aef8 => { vm.decode(0x440a875u32); let d = 1i16; let s = 18i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027aefcu64) }
        0x1027aefc => { vm.decode(0x94a0e175u32); let d = 10i16; let s = 15i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shr(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x1027af00u64) }
        0x1027af00 => { vm.decode(0x1412bb5u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1027af04u64) }
        0x1027af04 => { vm.decode(0x810512du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1027af08u64) }
        0x1027af08 => { vm.decode(0x151abb5u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1027af0cu64) }
        0x1027af0c => { vm.decode(0x5408875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027af10u64) }
        0x1027af10 => { vm.decode(0x41c02bb5u32); let d = 10i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1027af14u64) }
        0x1027af14 => { vm.decode(0x810512du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1027af18u64) }
        0x1027af18 => { vm.decode(0x5412bb5u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1027af1cu64) }
        0x1027af1c => { vm.decode(0x5408875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027af20u64) }
        0x1027af20 => { vm.decode(0x41c00bb5u32); let d = 10i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1027af24u64) }
        0x1027af24 => { vm.decode(0x551abb5u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1027af28u64) }
        0x1027af28 => { vm.decode(0x5408875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027af2cu64) }
        0x1027af2c => { vm.decode(0x830112du32); let d = 10i16; let s = 24i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1027af30u64) }
        0x1027af30 => { vm.decode(0x41321036u32); let d = 24i16; let s = 11i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027af34u64) }
        0x1027af34 => { vm.decode(0x6ad2e849u32); let d = 24i16; let s = 9i16; let s2 = -1i16; let imm = 1908483i64;
        let src = vm.ld(s) as u32;
        let dst = vm.ld(d) as u32;
        let v = (src << 3) | (dst & 7);
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1027af38u64) }
        0x1027af38 => { vm.decode(0x74a4e175u32); let d = 15i16; let s = 15i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shr(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x1027af3cu64) }
        0x1027af3c => { vm.decode(0x1e72bb5u32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1027af40u64) }
        0x1027af40 => { vm.decode(0x894d16du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1027af44u64) }
        0x1027af44 => { vm.decode(0x15488875u32); let d = 1i16; let s = 15i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027af48u64) }
        0x1027af48 => { vm.decode(0x541abb5u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1027af4cu64) }
        0x1027af4c => { vm.decode(0x5408875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027af50u64) }
        0x1027af50 => { vm.decode(0x8878927u32); let d = 0i16; let s = 31i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027af54u64).wrapping_add((imm * 4) as u64) } else { 0x1027af54u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027af54u64) }
        0x1027af54 => { vm.decode(0x220033u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1027af58u64) }
        0x1027af58 => { vm.decode(0xa1008200u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1027af5cu64) }
        0x1027af70 => { vm.decode(0xa1000446u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1027af74u64) }
        0x1027af74 => { vm.decode(0xa7d32cf1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -81i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1027af78u64).wrapping_add((imm * 4) as u64) } else { 0x1027af78u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027af78u64) }
        0x1027af78 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027af7cu64) } }
        0x1027af7c => { vm.decode(0x88a4c41du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 23256i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1027af80u64) }
        0x1027af80 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027af84u64) } }
        0x1027af84 => { vm.decode(0x3002036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027af88u64) }
        0x1027af98 => { vm.decode(0x608a736fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027af9cu64) }
        0x1027af9c => { vm.decode(0xa036u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027afa0u64) }
        0x1027afa0 => { vm.decode(0x6795fda7u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = -89i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027afa4u64).wrapping_add((imm * 4) as u64) } else { 0x1027afa4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027afa4u64) }
        0x1027afa4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027afa8u64) } }
        0x1027afa8 => { vm.decode(0x2080536fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027afacu64) }
        0x1027afac => { vm.decode(0x2a00812fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027afb0u64) }
        0x1027afb0 => { vm.decode(0x3019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027afb4u64) }
        0x1027afb4 => { vm.decode(0xb2115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027afb8u64) }
        0x1027afb8 => { vm.decode(0xa8c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 760i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027afbcu64) }
        0x1027afbc => { vm.decode(0xb391530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027afc0u64) }
        0x1027afc0 => { vm.decode(0x1b311430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027afc4u64) }
        0x1027afc4 => { vm.decode(0x28263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027afc8u64) }
        0x1027afc8 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027afccu64) }
        0x1027afcc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027afd0u64 + 4);
        vm.wr(37, 0x1027afd0u64 + 4);
        Ok(0x1027afd0u64) }
        0x1027afd0 => { vm.decode(0x66498519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027afd4u64) }
        0x1027afd4 => { vm.decode(0xcc63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027afd8u64) }
        0x1027afd8 => { vm.decode(0x2b0094b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027afdcu64) }
        0x1027afdc => { vm.decode(0x2020406u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 2i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1027afe0u64) }
        0x1027afe0 => { vm.decode(0x10840827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 70i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027afe4u64).wrapping_add((imm * 4) as u64) } else { 0x1027afe4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027afe4u64) }
        0x1027afe4 => { vm.decode(0x2b189430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027afe8u64) }
        0x1027afe8 => { vm.decode(0x440880a1u32); let d = -1i16; let s = 2i16; let s2 = -1i16; let imm = 344i64;
        vm.wr(36, if (vm.ld(s) as i64) < 1 { (0x1027afecu64).wrapping_add((imm * 4) as u64) } else { 0x1027afecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027afecu64) }
        0x1027afec => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027aff0u64) } }
        0x1027aff0 => { vm.decode(0xf9cc091u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2284i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027aff4u64) }
        0x1027b3bc => { vm.decode(0x2080536fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b3c0u64) }
        0x1027b3c0 => { vm.decode(0x2800a12fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b3c4u64) }
        0x1027b3c4 => { vm.decode(0x88f32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b3c8u64) }
        0x1027b3c8 => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027b3ccu64) }
        0x1027b3cc => { vm.decode(0x2b0915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027b3d0u64) }
        0x1027b3d0 => { vm.decode(0x3b0114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027b3d4u64) }
        0x1027b3d4 => { vm.decode(0x66398419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027b3d8u64) }
        0x1027b3d8 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b3dcu64) }
        0x1027b3dc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027b3e0u64 + 4);
        vm.wr(37, 0x1027b3e0u64 + 4);
        Ok(0x1027b3e0u64) }
        0x1027b3e0 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027b3e4u64) }
        0x1027b3e4 => { vm.decode(0xdd48011u32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027b3e8u64) }
        0x1027b3e8 => { vm.decode(0xa6830db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -695i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1027b3ecu64).wrapping_add((imm * 4) as u64) } else { 0x1027b3ecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027b3ecu64) }
        0x1027b3ec => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027b3f0u64) } }
        0x1027b3f0 => { vm.decode(0x608a536fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b3f4u64) }
        0x1027b3f4 => { vm.decode(0x22a461efu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b3f8u64) }
        0x1027b3f8 => { vm.decode(0x401019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027b3fcu64) }
        0x1027b3fc => { vm.decode(0xd181530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027b400u64) }
        0x1027b400 => { vm.decode(0x8c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b404u64) }
        0x1027b404 => { vm.decode(0xd101530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027b408u64) }
        0x1027b408 => { vm.decode(0xa8ce3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b40cu64) }
        0x1027b40c => { vm.decode(0xd081530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027b410u64) }
        0x1027b410 => { vm.decode(0x1d001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027b414u64) }
        0x1027b414 => { vm.decode(0x882e3efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b418u64) }
        0x1027b418 => { vm.decode(0x4088736fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b41cu64) }
        0x1027b41c => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027b420u64) }
        0x1027b420 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027b424u64 + 4);
        vm.wr(37, 0x1027b424u64 + 4);
        Ok(0x1027b424u64) }
        0x1027b424 => { vm.decode(0x62098219u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 4096i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027b428u64) }
        0x1027b428 => { vm.decode(0x22a4612fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b42cu64) }
        0x1027b42c => { vm.decode(0x62198219u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027b430u64) }
        0x1027b430 => { vm.decode(0x390854b0u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027b434u64) }
        0x1027b434 => { vm.decode(0x8d0855f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027b438u64) }
        0x1027b438 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027b43cu64) }
        0x1027b43c => { vm.decode(0x6284532fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b440u64) }
        0x1027b440 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027b444u64) }
        0x1027b444 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027b448u64 + 4);
        vm.wr(37, 0x1027b448u64 + 4);
        Ok(0x1027b448u64) }
        0x1027b448 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027b44cu64) }
        0x1027b44c => { vm.decode(0x4002036u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027b450u64) }
        0x1027b450 => { vm.decode(0xac412fu32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b454u64) }
        0x1027b454 => { vm.decode(0x43126e73u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2964i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1027b458u64) }
        0x1027b458 => { vm.decode(0xd9c48d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1268i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027b45cu64) }
        0x1027b45c => { vm.decode(0xb2d3062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027b460u64) }
        0x1027b460 => { vm.decode(0x8b09dd70u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027b464u64) }
        0x1027b464 => { vm.decode(0xb019d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027b468u64) }
        0x1027b468 => { vm.decode(0x88663efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b46cu64) }
        0x1027b46c => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027b470u64) }
        0x1027b470 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027b474u64 + 4);
        vm.wr(37, 0x1027b474u64 + 4);
        Ok(0x1027b474u64) }
        0x1027b474 => { vm.decode(0x660b8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2944i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027b478u64) }
        0x1027b478 => { vm.decode(0x22a0612fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b47cu64) }
        0x1027b47c => { vm.decode(0x8cd3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b480u64) }
        0x1027b480 => { vm.decode(0x4d10ead1u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1820i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027b484u64) }
        0x1027b484 => { vm.decode(0xb3d7022u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027b488u64) }
        0x1027b488 => { vm.decode(0xb119530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027b48cu64) }
        0x1027b48c => { vm.decode(0x459c48d1u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1268i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027b490u64) }
        0x1027b490 => { vm.decode(0xa3d5022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027b494u64) }
        0x1027b494 => { vm.decode(0xa8c736fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b498u64) }
        0x1027b498 => { vm.decode(0xb019530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027b49cu64) }
        0x1027b49c => { vm.decode(0x288d32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b4a0u64) }
        0x1027b4a0 => { vm.decode(0xb3915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027b4a4u64) }
        0x1027b4a4 => { vm.decode(0x1b3114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027b4a8u64) }
        0x1027b4a8 => { vm.decode(0x8643afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b4acu64) }
        0x1027b4ac => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027b4b0u64) }
        0x1027b4b0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027b4b4u64 + 4);
        vm.wr(37, 0x1027b4b4u64 + 4);
        Ok(0x1027b4b4u64) }
        0x1027b4b4 => { vm.decode(0x66598519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2672i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027b4b8u64) }
        0x1027b4b8 => { vm.decode(0x22a0616fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b4bcu64) }
        0x1027b4bc => { vm.decode(0xa9115d70u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027b4c0u64) }
        0x1027b4c0 => { vm.decode(0x28c632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b4c4u64) }
        0x1027b4c4 => { vm.decode(0x29091d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027b4c8u64) }
        0x1027b4c8 => { vm.decode(0xa9011c30u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 3328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027b4ccu64) }
        0x1027b4cc => { vm.decode(0x393994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027b4d0u64) }
        0x1027b4d0 => { vm.decode(0x886e3efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b4d4u64) }
        0x1027b4d4 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027b4d8u64) }
        0x1027b4d8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027b4dcu64 + 4);
        vm.wr(37, 0x1027b4dcu64 + 4);
        Ok(0x1027b4dcu64) }
        0x1027b4dc => { vm.decode(0x627f8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3320i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027b4e0u64) }
        0x1027b4e0 => { vm.decode(0x22a4616fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b4e4u64) }
        0x1027b4e4 => { vm.decode(0xab211cb0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 3936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027b4e8u64) }
        0x1027b4e8 => { vm.decode(0x88cc3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b4ecu64) }
        0x1027b4ec => { vm.decode(0x2b391d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027b4f0u64) }
        0x1027b4f0 => { vm.decode(0x3b311c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027b4f4u64) }
        0x1027b4f4 => { vm.decode(0x86c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b4f8u64) }
        0x1027b4f8 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027b4fcu64) }
        0x1027b4fc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027b500u64 + 4);
        vm.wr(37, 0x1027b500u64 + 4);
        Ok(0x1027b500u64) }
        0x1027b500 => { vm.decode(0x66698559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3920i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027b504u64) }
        0x1027b504 => { vm.decode(0x28a0412fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b508u64) }
        0x1027b508 => { vm.decode(0x8d0814b0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027b50cu64) }
        0x1027b50c => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027b510u64) }
        0x1027b510 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027b514u64) }
        0x1027b514 => { vm.decode(0x6084732fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b518u64) }
        0x1027b518 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027b51cu64) }
        0x1027b51c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027b520u64 + 4);
        vm.wr(37, 0x1027b520u64 + 4);
        Ok(0x1027b520u64) }
        0x1027b520 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027b524u64) }
        0x1027b524 => { vm.decode(0x28a0412fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b528u64) }
        0x1027b528 => { vm.decode(0x8d0855f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027b52cu64) }
        0x1027b52c => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027b530u64) }
        0x1027b530 => { vm.decode(0x286532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b534u64) }
        0x1027b534 => { vm.decode(0x608ee3efu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b538u64) }
        0x1027b538 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027b53cu64) }
        0x1027b53c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027b540u64 + 4);
        vm.wr(37, 0x1027b540u64 + 4);
        Ok(0x1027b540u64) }
        0x1027b540 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027b544u64) }
        0x1027b544 => { vm.decode(0x424841du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 21584i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1027b548u64) }
        0x1027b548 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027b54cu64) } }
        0x1027b54c => { vm.decode(0x408a536fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027b550u64) }
        0x1027b550 => { vm.decode(0x3e1000a7u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 3808i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027b554u64).wrapping_add((imm * 4) as u64) } else { 0x1027b554u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027b554u64) }
        0x1027b554 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027b558u64) } }
        0x1027b558 => { vm.decode(0xf9cc091u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2284i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027b55cu64) }
        0x1027c644 => { vm.decode(0x2b201c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027c648u64) }
        0x1027c648 => { vm.decode(0x2b181cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027c64cu64) }
        0x1027c64c => { vm.decode(0x20a4c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027c650u64) }
        0x1027c650 => { vm.decode(0xa8873afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027c654u64) }
        0x1027c654 => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027c658u64) }
        0x1027c658 => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027c65cu64) }
        0x1027c65c => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027c660u64) }
        0x1027c660 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027c664u64) }
        0x1027c664 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027c668u64) }
        0x1027c668 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027c66cu64 + 4);
        vm.wr(37, 0x1027c66cu64 + 4);
        Ok(0x1027c66cu64) }
        0x1027c66c => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027c670u64) }
        0x1027c670 => { vm.decode(0x2a8073afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027c674u64) }
        0x1027c674 => { vm.decode(0x3b181cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027c678u64) }
        0x1027c678 => { vm.decode(0x88c53afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027c67cu64) }
        0x1027c67c => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027c680u64).wrapping_add((imm * 4) as u64) } else { 0x1027c680u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027c680u64) }
        0x1027c680 => { vm.decode(0x2b201d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027c684u64) }
        0x1027c684 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027c688u64) }
        0x1027c688 => { vm.decode(0x8003036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027c68cu64) }
        0x1027c68c => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027c690u64) }
        0x1027c690 => { vm.decode(0x20a4e06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027c694u64) }
        0x1027c694 => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027c698u64) }
        0x1027c698 => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027c69cu64) }
        0x1027c69c => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027c6a0u64) }
        0x1027c6a0 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027c6a4u64) }
        0x1027c6a4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027c6a8u64 + 4);
        vm.wr(37, 0x1027c6a8u64 + 4);
        Ok(0x1027c6a8u64) }
        0x1027c6a8 => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027c6acu64) }
        0x1027c6ac => { vm.decode(0xa040101du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 33156i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1027c6b0u64) }
        0x1027c6b0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027c6b4u64) } }
        0x1027c6b4 => { vm.decode(0x2ac4436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027c6b8u64) }
        0x1027d644 => { vm.decode(0x20a4c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027d648u64) }
        0x1027d648 => { vm.decode(0xa8873afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027d64cu64) }
        0x1027d64c => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027d650u64) }
        0x1027d650 => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027d654u64) }
        0x1027d654 => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027d658u64) }
        0x1027d658 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027d65cu64) }
        0x1027d65c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d660u64) }
        0x1027d660 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027d664u64 + 4);
        vm.wr(37, 0x1027d664u64 + 4);
        Ok(0x1027d664u64) }
        0x1027d664 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d668u64) }
        0x1027d668 => { vm.decode(0x88c53afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027d66cu64) }
        0x1027d66c => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027d670u64).wrapping_add((imm * 4) as u64) } else { 0x1027d670u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027d670u64) }
        0x1027d670 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027d674u64) } }
        0x1027d674 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027d678u64) }
        0x1027d678 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027d67cu64) }
        0x1027d67c => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027d680u64) }
        0x1027d680 => { vm.decode(0x88c53afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027d684u64) }
        0x1027d684 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027d688u64) }
        0x1027d688 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x1027d68cu64).wrapping_add((imm * 4) as u64) } else { 0x1027d68cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027d68cu64) }
        0x1027d68c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027d690u64) } }
        0x1027d690 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027d694u64).wrapping_add((imm * 4) as u64) } else { 0x1027d694u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027d694u64) }
        0x1027d6e8 => { vm.decode(0x20a4e06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027d6ecu64) }
        0x1027d6ec => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027d6f0u64) }
        0x1027d6f0 => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027d6f4u64) }
        0x1027d6f4 => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027d6f8u64) }
        0x1027d6f8 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d6fcu64) }
        0x1027d6fc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027d700u64 + 4);
        vm.wr(37, 0x1027d700u64 + 4);
        Ok(0x1027d700u64) }
        0x1027d700 => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027d704u64) }
        0x1027d704 => { vm.decode(0x2aa0406fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027d708u64) }
        0x1027d708 => { vm.decode(0x28cf3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027d70cu64) }
        0x1027d70c => { vm.decode(0xb3195b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027d710u64) }
        0x1027d710 => { vm.decode(0x8b2994b0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027d714u64) }
        0x1027d714 => { vm.decode(0x1b2194b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027d718u64) }
        0x1027d718 => { vm.decode(0xa86c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027d71cu64) }
        0x1027d71c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d720u64) }
        0x1027d720 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027d724u64 + 4);
        vm.wr(37, 0x1027d724u64 + 4);
        Ok(0x1027d724u64) }
        0x1027d724 => { vm.decode(0x661b8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2784i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027d728u64) }
        0x1027d728 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027d72cu64) }
        0x1027d72c => { vm.decode(0x20a4c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027d730u64) }
        0x1027d730 => { vm.decode(0x28cf3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027d734u64) }
        0x1027d734 => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027d738u64) }
        0x1027d738 => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027d73cu64) }
        0x1027d73c => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027d740u64) }
        0x1027d740 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d744u64) }
        0x1027d744 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027d748u64 + 4);
        vm.wr(37, 0x1027d748u64 + 4);
        Ok(0x1027d748u64) }
        0x1027d748 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d74cu64) }
        0x1027d74c => { vm.decode(0x88d3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027d750u64) }
        0x1027d750 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027d754u64).wrapping_add((imm * 4) as u64) } else { 0x1027d754u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027d754u64) }
        0x1027d754 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027d758u64) } }
        0x1027d758 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027d75cu64) }
        0x1027d75c => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027d760u64) }
        0x1027d760 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027d764u64) }
        0x1027d764 => { vm.decode(0x88d3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027d768u64) }
        0x1027d768 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027d76cu64) }
        0x1027d76c => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x1027d770u64).wrapping_add((imm * 4) as u64) } else { 0x1027d770u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027d770u64) }
        0x1027d770 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027d774u64) } }
        0x1027d774 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027d778u64).wrapping_add((imm * 4) as u64) } else { 0x1027d778u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027d778u64) }
        0x1027d778 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027d77cu64) } }
        0x1027d77c => { vm.decode(0x22a0c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027d780u64) }
        0x1027d780 => { vm.decode(0x2b311530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027d784u64) }
        0x1027d784 => { vm.decode(0x3b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027d788u64) }
        0x1027d788 => { vm.decode(0xa82c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027d78cu64) }
        0x1027d78c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d790u64) }
        0x1027d790 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027d794u64 + 4);
        vm.wr(37, 0x1027d794u64 + 4);
        Ok(0x1027d794u64) }
        0x1027d794 => { vm.decode(0x662d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3656i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027d798u64) }
        0x1027d798 => { vm.decode(0x2b201c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027d79cu64) }
        0x1027d79c => { vm.decode(0x28cf3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027d7a0u64) }
        0x1027d7a0 => { vm.decode(0x8000a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027d7a4u64).wrapping_add((imm * 4) as u64) } else { 0x1027d7a4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027d7a4u64) }
        0x1027d7a4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027d7a8u64) } }
        0x1027d7a8 => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027d7acu64) }
        0x1027d7ac => { vm.decode(0x2000212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027d7b0u64) }
        0x1027d7b0 => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027d7b4u64) }
        0x1027d7b4 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027d7b8u64) }
        0x1027d7b8 => { vm.decode(0x286532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027d7bcu64) }
        0x1027d7bc => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d7c0u64) }
        0x1027d7c0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027d7c4u64 + 4);
        vm.wr(37, 0x1027d7c4u64 + 4);
        Ok(0x1027d7c4u64) }
        0x1027d7c4 => { vm.decode(0x62198219u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027d7c8u64) }
        0x1027d7c8 => { vm.decode(0x2b181cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027d7ccu64) }
        0x1027d7cc => { vm.decode(0x20a4e06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027d7d0u64) }
        0x1027d7d0 => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027d7d4u64) }
        0x1027d7d4 => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027d7d8u64) }
        0x1027d7d8 => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027d7dcu64) }
        0x1027d7dc => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d7e0u64) }
        0x1027d7e0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027d7e4u64 + 4);
        vm.wr(37, 0x1027d7e4u64 + 4);
        Ok(0x1027d7e4u64) }
        0x1027d7e4 => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027d7e8u64) }
        0x1027d7e8 => { vm.decode(0xc90d875u32); let d = 21i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027d7ecu64) }
        0x1027d7ec => { vm.decode(0xd104851u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027d7f0u64) }
        _ => Err(Error::UnknownPc(pc)),
    }
}
