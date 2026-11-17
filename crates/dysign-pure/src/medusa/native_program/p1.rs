#![cfg_attr(rustfmt, rustfmt::skip)]
// 由 scripts/medusa-vm/gen_native.py 生成，勿手改。
use super::super::Vm;
use super::super::Error;

#[allow(clippy::all, unused_parens, unused_variables, unused_mut)]
pub fn exec(vm: &mut Vm, pc: u64) -> Result<u64, Error> {
    match pc {
        0x10273b80 => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273b84u64) }
        0x10273b84 => { vm.decode(0x9099d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273b88u64) }
        0x10273b88 => { vm.decode(0x19019c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273b8cu64) }
        0x10273b8c => { vm.decode(0x620b8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2432i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273b90u64) }
        0x10273b90 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273b94u64) }
        0x10273b94 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273b98u64 + 4);
        vm.wr(37, 0x10273b98u64 + 4);
        Ok(0x10273b98u64) }
        0x10273b98 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273b9cu64) }
        0x10273b9c => { vm.decode(0x9d4c011u32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10273ba0u64) }
        0x10273ba0 => { vm.decode(0x8002236u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10273ba4u64) }
        0x10273ba4 => { vm.decode(0x208a071u32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10273ba8u64).wrapping_add((imm * 4) as u64) } else { 0x10273ba8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10273ba8u64) }
        0x10273ba8 => { vm.decode(0xa8843efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273bacu64) }
        0x10273bac => { vm.decode(0x8099u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273bb0u64) }
        0x10273bc0 => { vm.decode(0x8099u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273bc4u64) }
        0x10273bc4 => { vm.decode(0x9935u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10273bc8u64) }
        0x10273bc8 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273bccu64) }
        0x10273bcc => { vm.decode(0x1b309430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273bd0u64) }
        0x10273bd0 => { vm.decode(0x2800612fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273bd4u64) }
        0x10273bd4 => { vm.decode(0x19199c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273bd8u64) }
        0x10273bd8 => { vm.decode(0x28263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273bdcu64) }
        0x10273bdc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273be0u64) }
        0x10273be0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273be4u64 + 4);
        vm.wr(37, 0x10273be4u64 + 4);
        Ok(0x10273be4u64) }
        0x10273be4 => { vm.decode(0x624f8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2456i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273be8u64) }
        0x10273be8 => { vm.decode(0x616d0417u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = u32::from_le_bytes(b.try_into().unwrap());
        vm.st(d, v as u64);
        Ok(0x10273becu64) }
        0x10273bec => { vm.decode(0xa8843efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273bf0u64) }
        0x10273bf0 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273bf4u64) }
        0x10273bf4 => { vm.decode(0x74491519u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273bf8u64) }
        0x10273bf8 => { vm.decode(0x644f3119u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 732i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273bfcu64) }
        0x10273bfc => { vm.decode(0x8002236u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10273c00u64) }
        0x10273c00 => { vm.decode(0x1b309430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273c04u64) }
        0x10273c04 => { vm.decode(0x2a00616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273c08u64) }
        0x10273c08 => { vm.decode(0x6b1d5022u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10273c0cu64) }
        0x10273c0c => { vm.decode(0xb2115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273c10u64) }
        0x10273c10 => { vm.decode(0x9b395430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273c14u64) }
        0x10273c14 => { vm.decode(0x1b311430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273c18u64) }
        0x10273c18 => { vm.decode(0x628e63afu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273c1cu64) }
        0x10273c1c => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273c20u64) }
        0x10273c20 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273c24u64) }
        0x10273c24 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273c28u64 + 4);
        vm.wr(37, 0x10273c28u64 + 4);
        Ok(0x10273c28u64) }
        0x10273c28 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273c2cu64) }
        0x10273c2c => { vm.decode(0x644b8119u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273c30u64) }
        0x10273c30 => { vm.decode(0x10236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10273c34u64) }
        0x10273c34 => { vm.decode(0x2a08616fu32); let d = 3i16; let s = 17i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273c38u64) }
        0x10273c38 => { vm.decode(0x6a1d5022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10273c3cu64) }
        0x10273c3c => { vm.decode(0x1b2114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273c40u64) }
        0x10273c40 => { vm.decode(0x4a8ae32fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273c44u64) }
        0x10273c44 => { vm.decode(0x8b395530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273c48u64) }
        0x10273c48 => { vm.decode(0x1b311530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273c4cu64) }
        0x10273c4c => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273c50u64) }
        0x10273c50 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273c54u64) }
        0x10273c54 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273c58u64 + 4);
        vm.wr(37, 0x10273c58u64 + 4);
        Ok(0x10273c58u64) }
        0x10273c58 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273c5cu64) }
        0x10273c5c => { vm.decode(0x721f0419u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2216i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273c60u64) }
        0x10273c60 => { vm.decode(0x2000416fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273c64u64) }
        0x10273c64 => { vm.decode(0x8921d530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273c68u64) }
        0x10273c68 => { vm.decode(0x9919d4b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273c6cu64) }
        0x10273c6c => { vm.decode(0x9b0054b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273c70u64) }
        0x10273c70 => { vm.decode(0x8843efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273c74u64) }
        0x10273c74 => { vm.decode(0x91195b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273c78u64) }
        0x10273c78 => { vm.decode(0x48009875u32); let d = 19i16; let s = 0i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273c7cu64) }
        0x10273c7c => { vm.decode(0x190994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273c80u64) }
        0x10273c80 => { vm.decode(0x628243afu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273c84u64) }
        0x10273c84 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273c88u64) }
        0x10273c88 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273c8cu64) }
        0x10273c8c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273c90u64 + 4);
        vm.wr(37, 0x10273c90u64 + 4);
        Ok(0x10273c90u64) }
        0x10273c90 => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273c94u64) }
        0x10273c94 => { vm.decode(0x2000416fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273c98u64) }
        0x10273c98 => { vm.decode(0x8863efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273c9cu64) }
        0x10273c9c => { vm.decode(0x9219530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273ca0u64) }
        0x10273ca0 => { vm.decode(0x991995b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273ca4u64) }
        0x10273ca4 => { vm.decode(0x628863efu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273ca8u64) }
        0x10273ca8 => { vm.decode(0x991195b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273cacu64) }
        0x10273cac => { vm.decode(0x190994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273cb0u64) }
        0x10273cb0 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273cb4u64) }
        0x10273cb4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273cb8u64) }
        0x10273cb8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273cbcu64 + 4);
        vm.wr(37, 0x10273cbcu64 + 4);
        Ok(0x10273cbcu64) }
        0x10273cbc => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273cc0u64) }
        0x10273cc0 => { vm.decode(0x770f1019u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273cc4u64) }
        0x10273cc4 => { vm.decode(0x645d1119u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273cc8u64) }
        0x10273cc8 => { vm.decode(0x22236u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10273cccu64) }
        0x10273ccc => { vm.decode(0xe2a04d23u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 2)?;
        let v = u16::from_le_bytes(b.try_into().unwrap()) as u64;
        vm.st(d, v);
        Ok(0x10273cd0u64) }
        0x10273cd0 => { vm.decode(0x2025ad45u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u16;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10273cd4u64) }
        0x10273cd4 => { vm.decode(0x2a00616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273cd8u64) }
        0x10273cd8 => { vm.decode(0x6b1d5022u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10273cdcu64) }
        0x10273cdc => { vm.decode(0xb181530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273ce0u64) }
        0x10273ce0 => { vm.decode(0xb2115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273ce4u64) }
        0x10273ce4 => { vm.decode(0x8b395570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273ce8u64) }
        0x10273ce8 => { vm.decode(0x1b311430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273cecu64) }
        0x10273cec => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273cf0u64) }
        0x10273cf0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273cf4u64) }
        0x10273cf4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273cf8u64 + 4);
        vm.wr(37, 0x10273cf8u64 + 4);
        Ok(0x10273cf8u64) }
        0x10273cf8 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273cfcu64) }
        0x10273cfc => { vm.decode(0x2000416fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273d00u64) }
        0x10273d00 => { vm.decode(0x8921d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273d04u64) }
        0x10273d04 => { vm.decode(0x991995b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273d08u64) }
        0x10273d08 => { vm.decode(0x608e63efu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273d0cu64) }
        0x10273d0c => { vm.decode(0x9911d5b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273d10u64) }
        0x10273d10 => { vm.decode(0x190994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273d14u64) }
        0x10273d14 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273d18u64) }
        0x10273d18 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273d1cu64) }
        0x10273d1c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273d20u64 + 4);
        vm.wr(37, 0x10273d20u64 + 4);
        Ok(0x10273d20u64) }
        0x10273d20 => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273d24u64) }
        0x10273d24 => { vm.decode(0x744b1519u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273d28u64) }
        0x10273d28 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273d2cu64) }
        0x10273d2c => { vm.decode(0x8b39d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273d30u64) }
        0x10273d30 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273d34u64) }
        0x10273d34 => { vm.decode(0x428643efu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273d38u64) }
        0x10273d38 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273d3cu64) }
        0x10273d3c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273d40u64) }
        0x10273d40 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273d44u64 + 4);
        vm.wr(37, 0x10273d44u64 + 4);
        Ok(0x10273d44u64) }
        0x10273d44 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273d48u64) }
        0x10273d48 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273d4cu64) }
        0x10273d4c => { vm.decode(0x28863efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273d50u64) }
        0x10273d50 => { vm.decode(0xb399530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273d54u64) }
        0x10273d54 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273d58u64) }
        0x10273d58 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273d5cu64) }
        0x10273d5c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273d60u64) }
        0x10273d60 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273d64u64 + 4);
        vm.wr(37, 0x10273d64u64 + 4);
        Ok(0x10273d64u64) }
        0x10273d64 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273d68u64) }
        0x10273d68 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273d6cu64) }
        0x10273d6c => { vm.decode(0x608843efu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273d70u64) }
        0x10273d70 => { vm.decode(0x9b399530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273d74u64) }
        0x10273d74 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273d78u64) }
        0x10273d78 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273d7cu64) }
        0x10273d7c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273d80u64) }
        0x10273d80 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273d84u64 + 4);
        vm.wr(37, 0x10273d84u64 + 4);
        Ok(0x10273d84u64) }
        0x10273d84 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273d88u64) }
        0x10273d88 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273d8cu64) }
        0x10273d8c => { vm.decode(0x8b39d530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273d90u64) }
        0x10273d90 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273d94u64) }
        0x10273d94 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273d98u64) }
        0x10273d98 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273d9cu64) }
        0x10273d9c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273da0u64 + 4);
        vm.wr(37, 0x10273da0u64 + 4);
        Ok(0x10273da0u64) }
        0x10273da0 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273da4u64) }
        0x10273da4 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273da8u64) }
        0x10273da8 => { vm.decode(0x608643afu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273dacu64) }
        0x10273dac => { vm.decode(0x9b39d430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273db0u64) }
        0x10273db0 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273db4u64) }
        0x10273db4 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273db8u64) }
        0x10273db8 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273dbcu64) }
        0x10273dbc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273dc0u64 + 4);
        vm.wr(37, 0x10273dc0u64 + 4);
        Ok(0x10273dc0u64) }
        0x10273dc0 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273dc4u64) }
        0x10273dc4 => { vm.decode(0x8002036u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10273dc8u64) }
        0x10273dc8 => { vm.decode(0x232d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10273dccu64) }
        0x10273dcc => { vm.decode(0x2200412fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273dd0u64) }
        0x10273dd0 => { vm.decode(0xab20dd30u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 1984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273dd4u64) }
        0x10273dd4 => { vm.decode(0x88cc32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273dd8u64) }
        0x10273dd8 => { vm.decode(0x2b189db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273ddcu64) }
        0x10273ddc => { vm.decode(0x28c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273de0u64) }
        0x10273de0 => { vm.decode(0x2b109db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273de4u64) }
        0x10273de4 => { vm.decode(0xbb08ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 1960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273de8u64) }
        0x10273de8 => { vm.decode(0x428863efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273decu64) }
        0x10273dec => { vm.decode(0xbb00dcb0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 1952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273df0u64) }
        0x10273df0 => { vm.decode(0x3b189c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273df4u64) }
        0x10273df4 => { vm.decode(0x886436fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273df8u64) }
        0x10273df8 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273dfcu64) }
        0x10273dfc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273e00u64 + 4);
        vm.wr(37, 0x10273e00u64 + 4);
        Ok(0x10273e00u64) }
        0x10273e00 => { vm.decode(0x666f8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1944i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273e04u64) }
        0x10273e04 => { vm.decode(0x2a00416fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273e08u64) }
        0x10273e08 => { vm.decode(0xa8c63afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273e0cu64) }
        0x10273e0c => { vm.decode(0x9191530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273e10u64) }
        0x10273e10 => { vm.decode(0x99111530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273e14u64) }
        0x10273e14 => { vm.decode(0x19091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273e18u64) }
        0x10273e18 => { vm.decode(0x8663afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273e1cu64) }
        0x10273e1c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273e20u64) }
        0x10273e20 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273e24u64 + 4);
        vm.wr(37, 0x10273e24u64 + 4);
        Ok(0x10273e24u64) }
        0x10273e24 => { vm.decode(0x620d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2056i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273e28u64) }
        0x10273e28 => { vm.decode(0x2200e12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273e2cu64) }
        0x10273e2c => { vm.decode(0xa8c63afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273e30u64) }
        0x10273e30 => { vm.decode(0x9399db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273e34u64) }
        0x10273e34 => { vm.decode(0x89319db0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273e38u64) }
        0x10273e38 => { vm.decode(0x19299cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273e3cu64) }
        0x10273e3c => { vm.decode(0x28243efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273e40u64) }
        0x10273e40 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273e44u64) }
        0x10273e44 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273e48u64 + 4);
        vm.wr(37, 0x10273e48u64 + 4);
        Ok(0x10273e48u64) }
        0x10273e48 => { vm.decode(0x621f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273e4cu64) }
        0x10273e4c => { vm.decode(0x88843efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273e50u64) }
        0x10273e50 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10273e54u64).wrapping_add((imm * 4) as u64) } else { 0x10273e54u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10273e54u64) }
        0x10273e54 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10273e58u64) } }
        0x10273e58 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10273e5cu64) }
        0x10273e5c => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10273e60u64) }
        0x10273e60 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10273e64u64) }
        0x10273e64 => { vm.decode(0x88843efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273e68u64) }
        0x10273e68 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10273e6cu64) }
        0x10273e6c => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x10273e70u64).wrapping_add((imm * 4) as u64) } else { 0x10273e70u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10273e70u64) }
        0x10273e70 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10273e74u64) } }
        0x10273e74 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10273e78u64).wrapping_add((imm * 4) as u64) } else { 0x10273e78u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10273e78u64) }
        0x10273e78 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10273e7cu64) } }
        0x10273e7c => { vm.decode(0x2200c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273e80u64) }
        0x10273e80 => { vm.decode(0x9119db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273e84u64) }
        0x10273e84 => { vm.decode(0x19099cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273e88u64) }
        0x10273e88 => { vm.decode(0xa82e36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273e8cu64) }
        0x10273e8c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273e90u64) }
        0x10273e90 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273e94u64 + 4);
        vm.wr(37, 0x10273e94u64 + 4);
        Ok(0x10273e94u64) }
        0x10273e94 => { vm.decode(0x621f8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2472i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273e98u64) }
        0x10273e98 => { vm.decode(0xb209430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273e9cu64) }
        0x10273e9c => { vm.decode(0xa8c63afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273ea0u64) }
        0x10273ea0 => { vm.decode(0x8000a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10273ea4u64).wrapping_add((imm * 4) as u64) } else { 0x10273ea4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10273ea4u64) }
        0x10273ea4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10273ea8u64) } }
        0x10273ea8 => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273eacu64) }
        0x10273eac => { vm.decode(0x2000212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273eb0u64) }
        0x10273eb0 => { vm.decode(0xb399530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273eb4u64) }
        0x10273eb4 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273eb8u64) }
        0x10273eb8 => { vm.decode(0x28643efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273ebcu64) }
        0x10273ebc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273ec0u64) }
        0x10273ec0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273ec4u64 + 4);
        vm.wr(37, 0x10273ec4u64 + 4);
        Ok(0x10273ec4u64) }
        0x10273ec4 => { vm.decode(0x664b8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273ec8u64) }
        0x10273ec8 => { vm.decode(0xb1894b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273eccu64) }
        0x10273ecc => { vm.decode(0x2004c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273ed0u64) }
        0x10273ed0 => { vm.decode(0x89219db0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273ed4u64) }
        0x10273ed4 => { vm.decode(0x19399c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273ed8u64) }
        0x10273ed8 => { vm.decode(0x8643efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273edcu64) }
        0x10273edc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273ee0u64) }
        0x10273ee0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273ee4u64 + 4);
        vm.wr(37, 0x10273ee4u64 + 4);
        Ok(0x10273ee4u64) }
        0x10273ee4 => { vm.decode(0x624f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273ee8u64) }
        0x10273ee8 => { vm.decode(0xd146011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10273eecu64) }
        0x10273eec => { vm.decode(0xa0258d45u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u16;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10273ef0u64) }
        0x10273ef0 => { vm.decode(0x2a00616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273ef4u64) }
        0x10273ef4 => { vm.decode(0x20236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10273ef8u64) }
        0x10273ef8 => { vm.decode(0x6a1d5022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10273efcu64) }
        0x10273efc => { vm.decode(0x8b2115b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273f00u64) }
        0x10273f00 => { vm.decode(0x8b395570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273f04u64) }
        0x10273f04 => { vm.decode(0x1b311430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273f08u64) }
        0x10273f08 => { vm.decode(0x28663afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273f0cu64) }
        0x10273f0c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273f10u64) }
        0x10273f10 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273f14u64 + 4);
        vm.wr(37, 0x10273f14u64 + 4);
        Ok(0x10273f14u64) }
        0x10273f14 => { vm.decode(0x66498519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273f18u64) }
        0x10273f18 => { vm.decode(0x2000416fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273f1cu64) }
        0x10273f1c => { vm.decode(0x8843efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273f20u64) }
        0x10273f20 => { vm.decode(0x9219530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273f24u64) }
        0x10273f24 => { vm.decode(0x8919d5f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273f28u64) }
        0x10273f28 => { vm.decode(0x6a88e32fu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273f2cu64) }
        0x10273f2c => { vm.decode(0x991195b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273f30u64) }
        0x10273f30 => { vm.decode(0x190994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273f34u64) }
        0x10273f34 => { vm.decode(0x28243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273f38u64) }
        0x10273f38 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273f3cu64) }
        0x10273f3c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273f40u64 + 4);
        vm.wr(37, 0x10273f40u64 + 4);
        Ok(0x10273f40u64) }
        0x10273f40 => { vm.decode(0x621f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2216i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273f44u64) }
        0x10273f44 => { vm.decode(0x2200612fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273f48u64) }
        0x10273f48 => { vm.decode(0x991115b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273f4cu64) }
        0x10273f4c => { vm.decode(0x28863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273f50u64) }
        0x10273f50 => { vm.decode(0x90915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273f54u64) }
        0x10273f54 => { vm.decode(0x190114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273f58u64) }
        0x10273f58 => { vm.decode(0xa86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273f5cu64) }
        0x10273f5c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273f60u64) }
        0x10273f60 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273f64u64 + 4);
        vm.wr(37, 0x10273f64u64 + 4);
        Ok(0x10273f64u64) }
        0x10273f64 => { vm.decode(0x62198419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2080i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273f68u64) }
        0x10273f68 => { vm.decode(0x2a00416fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273f6cu64) }
        0x10273f6c => { vm.decode(0x28c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273f70u64) }
        0x10273f70 => { vm.decode(0x9191530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273f74u64) }
        0x10273f74 => { vm.decode(0x8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273f78u64) }
        0x10273f78 => { vm.decode(0x9111530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273f7cu64) }
        0x10273f7c => { vm.decode(0x19091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273f80u64) }
        0x10273f80 => { vm.decode(0x8663afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273f84u64) }
        0x10273f84 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273f88u64) }
        0x10273f88 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273f8cu64 + 4);
        vm.wr(37, 0x10273f8cu64 + 4);
        Ok(0x10273f8cu64) }
        0x10273f8c => { vm.decode(0x620d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2056i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273f90u64) }
        0x10273f90 => { vm.decode(0x2200e12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273f94u64) }
        0x10273f94 => { vm.decode(0x28c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273f98u64) }
        0x10273f98 => { vm.decode(0x9399db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273f9cu64) }
        0x10273f9c => { vm.decode(0x89319db0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273fa0u64) }
        0x10273fa0 => { vm.decode(0x19299cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273fa4u64) }
        0x10273fa4 => { vm.decode(0x28243efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273fa8u64) }
        0x10273fa8 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273facu64) }
        0x10273fac => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273fb0u64 + 4);
        vm.wr(37, 0x10273fb0u64 + 4);
        Ok(0x10273fb0u64) }
        0x10273fb0 => { vm.decode(0x621f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10273fb4u64) }
        0x10273fb4 => { vm.decode(0x88843afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273fb8u64) }
        0x10273fb8 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10273fbcu64).wrapping_add((imm * 4) as u64) } else { 0x10273fbcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10273fbcu64) }
        0x10273fbc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10273fc0u64) } }
        0x10273fc0 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10273fc4u64) }
        0x10273fc4 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10273fc8u64) }
        0x10273fc8 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10273fccu64) }
        0x10273fcc => { vm.decode(0x88843afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273fd0u64) }
        0x10273fd0 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10273fd4u64) }
        0x10273fd4 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x10273fd8u64).wrapping_add((imm * 4) as u64) } else { 0x10273fd8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10273fd8u64) }
        0x10273fd8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10273fdcu64) } }
        0x10273fdc => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10273fe0u64).wrapping_add((imm * 4) as u64) } else { 0x10273fe0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10273fe0u64) }
        0x10273fe0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10273fe4u64) } }
        0x10273fe4 => { vm.decode(0x2200c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273fe8u64) }
        0x10273fe8 => { vm.decode(0x9119db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273fecu64) }
        0x10273fec => { vm.decode(0x19099cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10273ff0u64) }
        0x10273ff0 => { vm.decode(0xa82e36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10273ff4u64) }
        0x10273ff4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10273ff8u64) }
        0x10273ff8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10273ffcu64 + 4);
        vm.wr(37, 0x10273ffcu64 + 4);
        Ok(0x10273ffcu64) }
        0x10273ffc => { vm.decode(0x621f8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2472i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274000u64) }
        0x10274000 => { vm.decode(0xb009430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274004u64) }
        0x10274004 => { vm.decode(0x28c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274008u64) }
        0x10274008 => { vm.decode(0x8000a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027400cu64).wrapping_add((imm * 4) as u64) } else { 0x1027400cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027400cu64) }
        0x1027400c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10274010u64) } }
        0x10274010 => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274014u64) }
        0x10274014 => { vm.decode(0x2000212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274018u64) }
        0x10274018 => { vm.decode(0xb399530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027401cu64) }
        0x1027401c => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274020u64) }
        0x10274020 => { vm.decode(0x28643efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274024u64) }
        0x10274024 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274028u64) }
        0x10274028 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027402cu64 + 4);
        vm.wr(37, 0x1027402cu64 + 4);
        Ok(0x1027402cu64) }
        0x1027402c => { vm.decode(0x664b8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274030u64) }
        0x10274030 => { vm.decode(0xb3814b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274034u64) }
        0x10274034 => { vm.decode(0x2004c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274038u64) }
        0x10274038 => { vm.decode(0x89219db0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027403cu64) }
        0x1027403c => { vm.decode(0x19399c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274040u64) }
        0x10274040 => { vm.decode(0x8643efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274044u64) }
        0x10274044 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274048u64) }
        0x10274048 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027404cu64 + 4);
        vm.wr(37, 0x1027404cu64 + 4);
        Ok(0x1027404cu64) }
        0x1027404c => { vm.decode(0x624f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274050u64) }
        0x10274050 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274054u64) }
        0x10274054 => { vm.decode(0x764b0519u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274058u64) }
        0x10274058 => { vm.decode(0x9b399530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027405cu64) }
        0x1027405c => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274060u64) }
        0x10274060 => { vm.decode(0x628e43efu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274064u64) }
        0x10274064 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274068u64) }
        0x10274068 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027406cu64) }
        0x1027406c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274070u64 + 4);
        vm.wr(37, 0x10274070u64 + 4);
        Ok(0x10274070u64) }
        0x10274070 => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274074u64) }
        0x10274074 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274078u64) }
        0x10274078 => { vm.decode(0x8b39d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027407cu64) }
        0x1027407c => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274080u64) }
        0x10274080 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274084u64) }
        0x10274084 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274088u64) }
        0x10274088 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027408cu64 + 4);
        vm.wr(37, 0x1027408cu64 + 4);
        Ok(0x1027408cu64) }
        0x1027408c => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274090u64) }
        0x10274090 => { vm.decode(0x741b1519u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2784i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274094u64) }
        0x10274094 => { vm.decode(0x4a019u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 12i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274098u64) }
        0x10274098 => { vm.decode(0x208c16fu32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027409cu64) }
        0x1027409c => { vm.decode(0x1b3994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102740a0u64) }
        0x102740a0 => { vm.decode(0x2284c32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102740a4u64) }
        0x102740a4 => { vm.decode(0x1b3194b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102740a8u64) }
        0x102740a8 => { vm.decode(0x228063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102740acu64) }
        0x102740ac => { vm.decode(0x1b2994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102740b0u64) }
        0x102740b0 => { vm.decode(0xb2195b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102740b4u64) }
        0x102740b4 => { vm.decode(0x4c901875u32); let d = 22i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102740b8u64) }
        0x102740b8 => { vm.decode(0x608263afu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102740bcu64) }
        0x102740bc => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102740c0u64) }
        0x102740c0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102740c4u64) }
        0x102740c4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102740c8u64 + 4);
        vm.wr(37, 0x102740c8u64 + 4);
        Ok(0x102740c8u64) }
        0x102740c8 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102740ccu64) }
        0x102740cc => { vm.decode(0x90009099u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102740d0u64) }
        0x102740d0 => { vm.decode(0x2200c16fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102740d4u64) }
        0x102740d4 => { vm.decode(0x9b3995b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102740d8u64) }
        0x102740d8 => { vm.decode(0x28ce32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102740dcu64) }
        0x102740dc => { vm.decode(0xb3195b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102740e0u64) }
        0x102740e0 => { vm.decode(0x8b29d5f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102740e4u64) }
        0x102740e4 => { vm.decode(0x1b2194b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102740e8u64) }
        0x102740e8 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102740ecu64) }
        0x102740ec => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102740f0u64) }
        0x102740f0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102740f4u64 + 4);
        vm.wr(37, 0x102740f4u64 + 4);
        Ok(0x102740f4u64) }
        0x102740f4 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102740f8u64) }
        0x102740f8 => { vm.decode(0x2200c16fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102740fcu64) }
        0x102740fc => { vm.decode(0x9b3995b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274100u64) }
        0x10274100 => { vm.decode(0x88c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274104u64) }
        0x10274104 => { vm.decode(0xb3195b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274108u64) }
        0x10274108 => { vm.decode(0x628863afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027410cu64) }
        0x1027410c => { vm.decode(0x9b2995b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274110u64) }
        0x10274110 => { vm.decode(0x1b2194b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274114u64) }
        0x10274114 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274118u64) }
        0x10274118 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027411cu64) }
        0x1027411c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274120u64 + 4);
        vm.wr(37, 0x10274120u64 + 4);
        Ok(0x10274120u64) }
        0x10274120 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274124u64) }
        0x10274124 => { vm.decode(0x48019u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274128u64) }
        0x10274128 => { vm.decode(0x208c16fu32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027412cu64) }
        0x1027412c => { vm.decode(0x1b3994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274130u64) }
        0x10274130 => { vm.decode(0x2080e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274134u64) }
        0x10274134 => { vm.decode(0x1b3194b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274138u64) }
        0x10274138 => { vm.decode(0x8b29d5b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027413cu64) }
        0x1027413c => { vm.decode(0xb2195b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274140u64) }
        0x10274140 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274144u64) }
        0x10274144 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274148u64) }
        0x10274148 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027414cu64 + 4);
        vm.wr(37, 0x1027414cu64 + 4);
        Ok(0x1027414cu64) }
        0x1027414c => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274150u64) }
        0x10274150 => { vm.decode(0x701f1519u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2280i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274154u64) }
        0x10274154 => { vm.decode(0x8020236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10274158u64) }
        0x10274158 => { vm.decode(0x2a04616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027415cu64) }
        0x1027415c => { vm.decode(0x6a357022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10274160u64) }
        0x10274160 => { vm.decode(0x993195b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274164u64) }
        0x10274164 => { vm.decode(0x192994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274168u64) }
        0x10274168 => { vm.decode(0x8b1855f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027416cu64) }
        0x1027416c => { vm.decode(0x4a8ecb6fu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274170u64) }
        0x10274170 => { vm.decode(0xb05875u32); let d = 4i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274174u64) }
        0x10274174 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274178u64) }
        0x10274178 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027417cu64 + 4);
        vm.wr(37, 0x1027417cu64 + 4);
        Ok(0x1027417cu64) }
        0x1027417c => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274180u64) }
        0x10274180 => { vm.decode(0x12036u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10274184u64) }
        0x10274184 => { vm.decode(0x8c12fu32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274188u64) }
        0x10274188 => { vm.decode(0x6b353022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027418cu64) }
        0x1027418c => { vm.decode(0x8931d5b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274190u64) }
        0x10274190 => { vm.decode(0x92995b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274194u64) }
        0x10274194 => { vm.decode(0x40cac32fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274198u64) }
        0x10274198 => { vm.decode(0xb05875u32); let d = 4i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027419cu64) }
        0x1027419c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102741a0u64) }
        0x102741a0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102741a4u64 + 4);
        vm.wr(37, 0x102741a4u64 + 4);
        Ok(0x102741a4u64) }
        0x102741a4 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102741a8u64) }
        0x102741a8 => { vm.decode(0x76798159u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 1904i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102741acu64) }
        0x102741ac => { vm.decode(0x2a04416fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102741b0u64) }
        0x102741b0 => { vm.decode(0xc8c32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102741b4u64) }
        0x102741b4 => { vm.decode(0x2b089d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102741b8u64) }
        0x102741b8 => { vm.decode(0xab00dd30u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 1920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102741bcu64) }
        0x102741bc => { vm.decode(0x4c905875u32); let d = 22i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102741c0u64) }
        0x102741c0 => { vm.decode(0x628863efu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102741c4u64) }
        0x102741c4 => { vm.decode(0x288e36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102741c8u64) }
        0x102741c8 => { vm.decode(0x2b381db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102741ccu64) }
        0x102741cc => { vm.decode(0x3b301cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102741d0u64) }
        0x102741d0 => { vm.decode(0x608ee32fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102741d4u64) }
        0x102741d4 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102741d8u64) }
        0x102741d8 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102741dcu64) }
        0x102741dc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102741e0u64 + 4);
        vm.wr(37, 0x102741e0u64 + 4);
        Ok(0x102741e0u64) }
        0x102741e0 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102741e4u64) }
        0x102741e4 => { vm.decode(0x2000e12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102741e8u64) }
        0x102741e8 => { vm.decode(0x8020236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102741ecu64) }
        0x102741ec => { vm.decode(0x6a357022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102741f0u64) }
        0x102741f0 => { vm.decode(0x28c63afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102741f4u64) }
        0x102741f4 => { vm.decode(0x93195b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102741f8u64) }
        0x102741f8 => { vm.decode(0x192994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102741fcu64) }
        0x102741fc => { vm.decode(0xb05875u32); let d = 4i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274200u64) }
        0x10274200 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274204u64) }
        0x10274204 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274208u64 + 4);
        vm.wr(37, 0x10274208u64 + 4);
        Ok(0x10274208u64) }
        0x10274208 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027420cu64) }
        0x1027420c => { vm.decode(0x2a04416fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274210u64) }
        0x10274210 => { vm.decode(0x88863efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274214u64) }
        0x10274214 => { vm.decode(0x2b089d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274218u64) }
        0x10274218 => { vm.decode(0xc8c32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027421cu64) }
        0x1027421c => { vm.decode(0x2b009d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274220u64) }
        0x10274220 => { vm.decode(0x8cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274224u64) }
        0x10274224 => { vm.decode(0x2b381db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274228u64) }
        0x10274228 => { vm.decode(0x3b301cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027422cu64) }
        0x1027422c => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274230u64) }
        0x10274230 => { vm.decode(0x4c909875u32); let d = 23i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274234u64) }
        0x10274234 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274238u64) }
        0x10274238 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027423cu64 + 4);
        vm.wr(37, 0x1027423cu64 + 4);
        Ok(0x1027423cu64) }
        0x1027423c => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274240u64) }
        0x10274240 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274244u64) }
        0x10274244 => { vm.decode(0x8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274248u64) }
        0x10274248 => { vm.decode(0xb399530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027424cu64) }
        0x1027424c => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274250u64) }
        0x10274250 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274254u64) }
        0x10274254 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274258u64) }
        0x10274258 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027425cu64 + 4);
        vm.wr(37, 0x1027425cu64 + 4);
        Ok(0x1027425cu64) }
        0x1027425c => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274260u64) }
        0x10274260 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274264u64) }
        0x10274264 => { vm.decode(0x8843efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274268u64) }
        0x10274268 => { vm.decode(0xb399530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027426cu64) }
        0x1027426c => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274270u64) }
        0x10274270 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274274u64) }
        0x10274274 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274278u64) }
        0x10274278 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027427cu64 + 4);
        vm.wr(37, 0x1027427cu64 + 4);
        Ok(0x1027427cu64) }
        0x1027427c => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274280u64) }
        0x10274280 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274284u64) }
        0x10274284 => { vm.decode(0x8c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274288u64) }
        0x10274288 => { vm.decode(0xb399530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027428cu64) }
        0x1027428c => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274290u64) }
        0x10274290 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274294u64) }
        0x10274294 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274298u64) }
        0x10274298 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027429cu64 + 4);
        vm.wr(37, 0x1027429cu64 + 4);
        Ok(0x1027429cu64) }
        0x1027429c => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102742a0u64) }
        0x102742a0 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102742a4u64) }
        0x102742a4 => { vm.decode(0x8863efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102742a8u64) }
        0x102742a8 => { vm.decode(0xb399530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102742acu64) }
        0x102742ac => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102742b0u64) }
        0x102742b0 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102742b4u64) }
        0x102742b4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102742b8u64) }
        0x102742b8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102742bcu64 + 4);
        vm.wr(37, 0x102742bcu64 + 4);
        Ok(0x102742bcu64) }
        0x102742bc => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102742c0u64) }
        0x102742c0 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102742c4u64) }
        0x102742c4 => { vm.decode(0x88cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102742c8u64) }
        0x102742c8 => { vm.decode(0xb399530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102742ccu64) }
        0x102742cc => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102742d0u64) }
        0x102742d0 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102742d4u64) }
        0x102742d4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102742d8u64) }
        0x102742d8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102742dcu64 + 4);
        vm.wr(37, 0x102742dcu64 + 4);
        Ok(0x102742dcu64) }
        0x102742dc => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102742e0u64) }
        0x102742e0 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102742e4u64) }
        0x102742e4 => { vm.decode(0xa88c32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102742e8u64) }
        0x102742e8 => { vm.decode(0xb399530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102742ecu64) }
        0x102742ec => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102742f0u64) }
        0x102742f0 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102742f4u64) }
        0x102742f4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102742f8u64) }
        0x102742f8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102742fcu64 + 4);
        vm.wr(37, 0x102742fcu64 + 4);
        Ok(0x102742fcu64) }
        0x102742fc => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274300u64) }
        0x10274300 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274304u64) }
        0x10274304 => { vm.decode(0x88cc32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274308u64) }
        0x10274308 => { vm.decode(0xb399530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027430cu64) }
        0x1027430c => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274310u64) }
        0x10274310 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274314u64) }
        0x10274314 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274318u64) }
        0x10274318 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027431cu64 + 4);
        vm.wr(37, 0x1027431cu64 + 4);
        Ok(0x1027431cu64) }
        0x1027431c => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274320u64) }
        0x10274320 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274324u64) }
        0x10274324 => { vm.decode(0x28c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274328u64) }
        0x10274328 => { vm.decode(0xb399530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027432cu64) }
        0x1027432c => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274330u64) }
        0x10274330 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274334u64) }
        0x10274334 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274338u64) }
        0x10274338 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027433cu64 + 4);
        vm.wr(37, 0x1027433cu64 + 4);
        Ok(0x1027433cu64) }
        0x1027433c => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274340u64) }
        0x10274340 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274344u64) }
        0x10274344 => { vm.decode(0xa8ce32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274348u64) }
        0x10274348 => { vm.decode(0xb399530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027434cu64) }
        0x1027434c => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274350u64) }
        0x10274350 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274354u64) }
        0x10274354 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274358u64) }
        0x10274358 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027435cu64 + 4);
        vm.wr(37, 0x1027435cu64 + 4);
        Ok(0x1027435cu64) }
        0x1027435c => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274360u64) }
        0x10274360 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274364u64) }
        0x10274364 => { vm.decode(0x88ce36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274368u64) }
        0x10274368 => { vm.decode(0xb399530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027436cu64) }
        0x1027436c => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274370u64) }
        0x10274370 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274374u64) }
        0x10274374 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274378u64) }
        0x10274378 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027437cu64 + 4);
        vm.wr(37, 0x1027437cu64 + 4);
        Ok(0x1027437cu64) }
        0x1027437c => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274380u64) }
        0x10274380 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274384u64) }
        0x10274384 => { vm.decode(0x28cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274388u64) }
        0x10274388 => { vm.decode(0xb399530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027438cu64) }
        0x1027438c => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274390u64) }
        0x10274390 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274394u64) }
        0x10274394 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274398u64) }
        0x10274398 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027439cu64 + 4);
        vm.wr(37, 0x1027439cu64 + 4);
        Ok(0x1027439cu64) }
        0x1027439c => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102743a0u64) }
        0x102743a0 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102743a4u64) }
        0x102743a4 => { vm.decode(0x28ce36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102743a8u64) }
        0x102743a8 => { vm.decode(0xb399530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102743acu64) }
        0x102743ac => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102743b0u64) }
        0x102743b0 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102743b4u64) }
        0x102743b4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102743b8u64) }
        0x102743b8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102743bcu64 + 4);
        vm.wr(37, 0x102743bcu64 + 4);
        Ok(0x102743bcu64) }
        0x102743bc => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102743c0u64) }
        0x102743c0 => { vm.decode(0x621f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102743c4u64) }
        0x102743c4 => { vm.decode(0x2200e12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102743c8u64) }
        0x102743c8 => { vm.decode(0x88536fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102743ccu64) }
        0x102743cc => { vm.decode(0x9399db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102743d0u64) }
        0x102743d0 => { vm.decode(0x99319db0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102743d4u64) }
        0x102743d4 => { vm.decode(0x19299cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102743d8u64) }
        0x102743d8 => { vm.decode(0x428843efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102743dcu64) }
        0x102743dc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102743e0u64) }
        0x102743e0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102743e4u64 + 4);
        vm.wr(37, 0x102743e4u64 + 4);
        Ok(0x102743e4u64) }
        0x102743e4 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102743e8u64) }
        0x102743e8 => { vm.decode(0x88736fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102743ecu64) }
        0x102743ec => { vm.decode(0x608ce36fu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102743f0u64) }
        0x102743f0 => { vm.decode(0x4882e36fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102743f4u64) }
        0x102743f4 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102743f8u64).wrapping_add((imm * 4) as u64) } else { 0x102743f8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102743f8u64) }
        0x102743f8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102743fcu64) } }
        0x102743fc => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10274400u64) }
        0x10274400 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10274404u64) }
        0x10274404 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10274408u64) }
        0x10274408 => { vm.decode(0x88736fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027440cu64) }
        0x1027440c => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10274410u64) }
        0x10274410 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x10274414u64).wrapping_add((imm * 4) as u64) } else { 0x10274414u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10274414u64) }
        0x10274414 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10274418u64) } }
        0x10274418 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027441cu64).wrapping_add((imm * 4) as u64) } else { 0x1027441cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027441cu64) }
        0x1027441c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10274420u64) } }
        0x10274420 => { vm.decode(0x2200c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274424u64) }
        0x10274424 => { vm.decode(0x9119db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274428u64) }
        0x10274428 => { vm.decode(0x19099cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027442cu64) }
        0x1027442c => { vm.decode(0xa82e36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274430u64) }
        0x10274430 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274434u64) }
        0x10274434 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274438u64 + 4);
        vm.wr(37, 0x10274438u64 + 4);
        Ok(0x10274438u64) }
        0x10274438 => { vm.decode(0x621f8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2472i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027443cu64) }
        0x1027443c => { vm.decode(0x29281430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274440u64) }
        0x10274440 => { vm.decode(0x88536fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274444u64) }
        0x10274444 => { vm.decode(0x8000a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10274448u64).wrapping_add((imm * 4) as u64) } else { 0x10274448u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10274448u64) }
        0x10274448 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027444cu64) } }
        0x1027444c => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274450u64) }
        0x10274450 => { vm.decode(0x2000212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274454u64) }
        0x10274454 => { vm.decode(0xb399530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274458u64) }
        0x10274458 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027445cu64) }
        0x1027445c => { vm.decode(0x664b8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274460u64) }
        0x10274460 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274464u64) }
        0x10274464 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274468u64 + 4);
        vm.wr(37, 0x10274468u64 + 4);
        Ok(0x10274468u64) }
        0x10274468 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027446cu64) }
        0x1027446c => { vm.decode(0x29201430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274470u64) }
        0x10274470 => { vm.decode(0x2004c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274474u64) }
        0x10274474 => { vm.decode(0x9921ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274478u64) }
        0x10274478 => { vm.decode(0x19399c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027447cu64) }
        0x1027447c => { vm.decode(0x624f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274480u64) }
        0x10274480 => { vm.decode(0x608643efu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274484u64) }
        0x10274484 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274488u64) }
        0x10274488 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027448cu64 + 4);
        vm.wr(37, 0x1027448cu64 + 4);
        Ok(0x1027448cu64) }
        0x1027448c => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274490u64) }
        0x10274490 => { vm.decode(0x621f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274494u64) }
        0x10274494 => { vm.decode(0x2200e12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274498u64) }
        0x10274498 => { vm.decode(0x288536fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027449cu64) }
        0x1027449c => { vm.decode(0x9399db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102744a0u64) }
        0x102744a0 => { vm.decode(0x9931ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102744a4u64) }
        0x102744a4 => { vm.decode(0x19299cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102744a8u64) }
        0x102744a8 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102744acu64) }
        0x102744ac => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102744b0u64 + 4);
        vm.wr(37, 0x102744b0u64 + 4);
        Ok(0x102744b0u64) }
        0x102744b0 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102744b4u64) }
        0x102744b4 => { vm.decode(0x288736fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102744b8u64) }
        0x102744b8 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102744bcu64).wrapping_add((imm * 4) as u64) } else { 0x102744bcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102744bcu64) }
        0x102744bc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102744c0u64) } }
        0x102744c0 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102744c4u64) }
        0x102744c4 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102744c8u64) }
        0x102744c8 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102744ccu64) }
        0x102744cc => { vm.decode(0x288736fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102744d0u64) }
        0x102744d0 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102744d4u64) }
        0x102744d4 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x102744d8u64).wrapping_add((imm * 4) as u64) } else { 0x102744d8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102744d8u64) }
        0x102744d8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102744dcu64) } }
        0x102744dc => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102744e0u64).wrapping_add((imm * 4) as u64) } else { 0x102744e0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102744e0u64) }
        0x10274534 => { vm.decode(0x2004c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274538u64) }
        0x10274538 => { vm.decode(0x9921ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027453cu64) }
        0x1027453c => { vm.decode(0x19399c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274540u64) }
        0x10274540 => { vm.decode(0x624f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274544u64) }
        0x10274544 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274548u64) }
        0x10274548 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027454cu64 + 4);
        vm.wr(37, 0x1027454cu64 + 4);
        Ok(0x1027454cu64) }
        0x1027454c => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274550u64) }
        0x10274550 => { vm.decode(0x664b8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274554u64) }
        0x10274554 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274558u64) }
        0x10274558 => { vm.decode(0x288c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027455cu64) }
        0x1027455c => { vm.decode(0xb399530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274560u64) }
        0x10274560 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274564u64) }
        0x10274564 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274568u64) }
        0x10274568 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027456cu64 + 4);
        vm.wr(37, 0x1027456cu64 + 4);
        Ok(0x1027456cu64) }
        0x1027456c => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274570u64) }
        0x10274570 => { vm.decode(0x621f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274574u64) }
        0x10274574 => { vm.decode(0x2200e12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274578u64) }
        0x10274578 => { vm.decode(0x88c732fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027457cu64) }
        0x1027457c => { vm.decode(0x9399db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274580u64) }
        0x10274580 => { vm.decode(0x9931ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274584u64) }
        0x10274584 => { vm.decode(0x19299cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274588u64) }
        0x10274588 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027458cu64) }
        0x1027458c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274590u64 + 4);
        vm.wr(37, 0x10274590u64 + 4);
        Ok(0x10274590u64) }
        0x10274590 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274594u64) }
        0x10274594 => { vm.decode(0xa8c532fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274598u64) }
        0x10274598 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027459cu64).wrapping_add((imm * 4) as u64) } else { 0x1027459cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027459cu64) }
        0x1027459c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102745a0u64) } }
        0x102745a0 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102745a4u64) }
        0x102745a4 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102745a8u64) }
        0x102745a8 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102745acu64) }
        0x102745ac => { vm.decode(0xa8c532fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102745b0u64) }
        0x102745b0 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102745b4u64) }
        0x102745b4 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x102745b8u64).wrapping_add((imm * 4) as u64) } else { 0x102745b8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102745b8u64) }
        0x102745b8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102745bcu64) } }
        0x102745bc => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102745c0u64).wrapping_add((imm * 4) as u64) } else { 0x102745c0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102745c0u64) }
        0x10274614 => { vm.decode(0x2004c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274618u64) }
        0x10274618 => { vm.decode(0x9921ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027461cu64) }
        0x1027461c => { vm.decode(0x19399c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274620u64) }
        0x10274620 => { vm.decode(0x624f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274624u64) }
        0x10274624 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274628u64) }
        0x10274628 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027462cu64 + 4);
        vm.wr(37, 0x1027462cu64 + 4);
        Ok(0x1027462cu64) }
        0x1027462c => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274630u64) }
        0x10274630 => { vm.decode(0x621f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274634u64) }
        0x10274634 => { vm.decode(0x2200e12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274638u64) }
        0x10274638 => { vm.decode(0xa8c732fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027463cu64) }
        0x1027463c => { vm.decode(0x9399db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274640u64) }
        0x10274640 => { vm.decode(0x9931ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274644u64) }
        0x10274644 => { vm.decode(0x19299cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274648u64) }
        0x10274648 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027464cu64) }
        0x1027464c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274650u64 + 4);
        vm.wr(37, 0x10274650u64 + 4);
        Ok(0x10274650u64) }
        0x10274650 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274654u64) }
        0x10274654 => { vm.decode(0x888536fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274658u64) }
        0x10274658 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027465cu64).wrapping_add((imm * 4) as u64) } else { 0x1027465cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027465cu64) }
        0x1027465c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10274660u64) } }
        0x10274660 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10274664u64) }
        0x10274664 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10274668u64) }
        0x10274668 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027466cu64) }
        0x1027466c => { vm.decode(0x888536fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274670u64) }
        0x10274670 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10274674u64) }
        0x10274674 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x10274678u64).wrapping_add((imm * 4) as u64) } else { 0x10274678u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10274678u64) }
        0x10274678 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027467cu64) } }
        0x1027467c => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10274680u64).wrapping_add((imm * 4) as u64) } else { 0x10274680u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10274680u64) }
        0x102746d4 => { vm.decode(0x2004c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102746d8u64) }
        0x102746d8 => { vm.decode(0x9921ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102746dcu64) }
        0x102746dc => { vm.decode(0x19399c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102746e0u64) }
        0x102746e0 => { vm.decode(0x624f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102746e4u64) }
        0x102746e4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102746e8u64) }
        0x102746e8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102746ecu64 + 4);
        vm.wr(37, 0x102746ecu64 + 4);
        Ok(0x102746ecu64) }
        0x102746ec => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102746f0u64) }
        0x102746f0 => { vm.decode(0x621f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102746f4u64) }
        0x102746f4 => { vm.decode(0x2200e12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102746f8u64) }
        0x102746f8 => { vm.decode(0x888736fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102746fcu64) }
        0x102746fc => { vm.decode(0x9399db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274700u64) }
        0x10274700 => { vm.decode(0x9931ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274704u64) }
        0x10274704 => { vm.decode(0x19299cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274708u64) }
        0x10274708 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027470cu64) }
        0x1027470c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274710u64 + 4);
        vm.wr(37, 0x10274710u64 + 4);
        Ok(0x10274710u64) }
        0x10274710 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274714u64) }
        0x10274714 => { vm.decode(0xa88536fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274718u64) }
        0x10274718 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027471cu64).wrapping_add((imm * 4) as u64) } else { 0x1027471cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027471cu64) }
        0x1027471c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10274720u64) } }
        0x10274720 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10274724u64) }
        0x10274724 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10274728u64) }
        0x10274728 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027472cu64) }
        0x1027472c => { vm.decode(0xa88536fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274730u64) }
        0x10274730 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10274734u64) }
        0x10274734 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x10274738u64).wrapping_add((imm * 4) as u64) } else { 0x10274738u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10274738u64) }
        0x10274738 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027473cu64) } }
        0x1027473c => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10274740u64).wrapping_add((imm * 4) as u64) } else { 0x10274740u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10274740u64) }
        0x10274794 => { vm.decode(0x2004c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274798u64) }
        0x10274798 => { vm.decode(0x9921ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027479cu64) }
        0x1027479c => { vm.decode(0x19399c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102747a0u64) }
        0x102747a0 => { vm.decode(0x624f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102747a4u64) }
        0x102747a4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102747a8u64) }
        0x102747a8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102747acu64 + 4);
        vm.wr(37, 0x102747acu64 + 4);
        Ok(0x102747acu64) }
        0x102747ac => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102747b0u64) }
        0x102747b0 => { vm.decode(0x621f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102747b4u64) }
        0x102747b4 => { vm.decode(0x2200e12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102747b8u64) }
        0x102747b8 => { vm.decode(0x88c736fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102747bcu64) }
        0x102747bc => { vm.decode(0x9399db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102747c0u64) }
        0x102747c0 => { vm.decode(0x9931ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102747c4u64) }
        0x102747c4 => { vm.decode(0x19299cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102747c8u64) }
        0x102747c8 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102747ccu64) }
        0x102747cc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102747d0u64 + 4);
        vm.wr(37, 0x102747d0u64 + 4);
        Ok(0x102747d0u64) }
        0x102747d0 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102747d4u64) }
        0x102747d4 => { vm.decode(0xa8c536fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102747d8u64) }
        0x102747d8 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102747dcu64).wrapping_add((imm * 4) as u64) } else { 0x102747dcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102747dcu64) }
        0x102747dc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102747e0u64) } }
        0x102747e0 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102747e4u64) }
        0x102747e4 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102747e8u64) }
        0x102747e8 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102747ecu64) }
        0x102747ec => { vm.decode(0xa8c536fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102747f0u64) }
        0x102747f0 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102747f4u64) }
        0x102747f4 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x102747f8u64).wrapping_add((imm * 4) as u64) } else { 0x102747f8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102747f8u64) }
        0x102747f8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102747fcu64) } }
        0x102747fc => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10274800u64).wrapping_add((imm * 4) as u64) } else { 0x10274800u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10274800u64) }
        0x10274854 => { vm.decode(0x2004c12fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274858u64) }
        0x10274858 => { vm.decode(0x9921ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027485cu64) }
        0x1027485c => { vm.decode(0x19399c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274860u64) }
        0x10274860 => { vm.decode(0x624f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274864u64) }
        0x10274864 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274868u64) }
        0x10274868 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027486cu64 + 4);
        vm.wr(37, 0x1027486cu64 + 4);
        Ok(0x1027486cu64) }
        0x1027486c => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274870u64) }
        0x10274870 => { vm.decode(0x744b1519u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274874u64) }
        0x10274874 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274878u64) }
        0x10274878 => { vm.decode(0x888c32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027487cu64) }
        0x1027487c => { vm.decode(0xb399530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274880u64) }
        0x10274880 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274884u64) }
        0x10274884 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274888u64) }
        0x10274888 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027488cu64) }
        0x1027488c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274890u64 + 4);
        vm.wr(37, 0x10274890u64 + 4);
        Ok(0x10274890u64) }
        0x10274890 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274894u64) }
        0x10274894 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274898u64) }
        0x10274898 => { vm.decode(0x8b39d430u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027489cu64) }
        0x1027489c => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102748a0u64) }
        0x102748a0 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102748a4u64) }
        0x102748a4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102748a8u64) }
        0x102748a8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102748acu64 + 4);
        vm.wr(37, 0x102748acu64 + 4);
        Ok(0x102748acu64) }
        0x102748ac => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102748b0u64) }
        0x102748b0 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102748b4u64) }
        0x102748b4 => { vm.decode(0x9b399530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102748b8u64) }
        0x102748b8 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102748bcu64) }
        0x102748bc => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102748c0u64) }
        0x102748c0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102748c4u64) }
        0x102748c4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102748c8u64 + 4);
        vm.wr(37, 0x102748c8u64 + 4);
        Ok(0x102748c8u64) }
        0x102748c8 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102748ccu64) }
        0x102748cc => { vm.decode(0x40c0c3afu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102748d0u64) }
        0x102748d0 => { vm.decode(0x60c0e3afu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102748d4u64) }
        0x102748d4 => { vm.decode(0x42c8c3afu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102748d8u64) }
        0x102748d8 => { vm.decode(0x62c8e3afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102748dcu64) }
        0x102748dc => { vm.decode(0x40c6c3afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102748e0u64) }
        0x102748e0 => { vm.decode(0x60c6e3afu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102748e4u64) }
        0x102748e4 => { vm.decode(0x42cec3afu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102748e8u64) }
        0x102748e8 => { vm.decode(0x62cee3afu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102748ecu64) }
        0x102748ec => { vm.decode(0x40cacbefu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102748f0u64) }
        0x102748f0 => { vm.decode(0x60caebefu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 2888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102748f4u64) }
        0x102748f4 => { vm.decode(0x54104cf5u32); let d = -1i16; let s = -1i16; let s2 = -1i16; let imm = 0i64;
        vm.jump_flag = 2;
        vm.wr(36, vm.slots[31]);
        vm.wr(35, 2);
        Ok(0x102748f8u64) }
        0x102748f8 => { vm.decode(0x77498559u32); let d = 29i16; let s = 29i16; let s2 = -1i16; let imm = 2896i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102748fcu64) }
        0x102748fc => { vm.decode(0x2000c16fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274900u64) }
        0x10274900 => { vm.decode(0x28c73efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274904u64) }
        0x10274904 => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274908u64) }
        0x10274908 => { vm.decode(0x9099d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027490cu64) }
        0x1027490c => { vm.decode(0x19019c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274910u64) }
        0x10274910 => { vm.decode(0x620b8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2432i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274914u64) }
        0x10274914 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274918u64) }
        0x10274918 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027491cu64 + 4);
        vm.wr(37, 0x1027491cu64 + 4);
        Ok(0x1027491cu64) }
        0x1027491c => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274920u64) }
        0x10274920 => { vm.decode(0x9d4c011u32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10274924u64) }
        0x10274924 => { vm.decode(0x278308f1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -987i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10274928u64).wrapping_add((imm * 4) as u64) } else { 0x10274928u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10274928u64) }
        0x10274928 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027492cu64) } }
        0x1027492c => { vm.decode(0x2200c16fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274930u64) }
        0x10274930 => { vm.decode(0x401019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274934u64) }
        0x10274934 => { vm.decode(0xb3995b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274938u64) }
        0x10274938 => { vm.decode(0xa8c636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027493cu64) }
        0x1027493c => { vm.decode(0xb3195b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274940u64) }
        0x10274940 => { vm.decode(0x8c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274944u64) }
        0x10274944 => { vm.decode(0xb2995b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274948u64) }
        0x10274948 => { vm.decode(0x1b2194b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027494cu64) }
        0x1027494c => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274950u64) }
        0x10274950 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274954u64) }
        0x10274954 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274958u64 + 4);
        vm.wr(37, 0x10274958u64 + 4);
        Ok(0x10274958u64) }
        0x10274958 => { vm.decode(0x661b8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2784i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027495cu64) }
        0x1027495c => { vm.decode(0x2004612fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274960u64) }
        0x10274960 => { vm.decode(0x744b1519u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274964u64) }
        0x10274964 => { vm.decode(0x9b39d530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274968u64) }
        0x10274968 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027496cu64) }
        0x1027496c => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274970u64) }
        0x10274970 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274974u64) }
        0x10274974 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274978u64 + 4);
        vm.wr(37, 0x10274978u64 + 4);
        Ok(0x10274978u64) }
        0x10274978 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027497cu64) }
        0x1027497c => { vm.decode(0x721f0419u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2216i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274980u64) }
        0x10274980 => { vm.decode(0x2804616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274984u64) }
        0x10274984 => { vm.decode(0xa88c32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274988u64) }
        0x10274988 => { vm.decode(0x9219530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027498cu64) }
        0x1027498c => { vm.decode(0x9919d5b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274990u64) }
        0x10274990 => { vm.decode(0x88ce36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274994u64) }
        0x10274994 => { vm.decode(0x91195b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274998u64) }
        0x10274998 => { vm.decode(0x190994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027499cu64) }
        0x1027499c => { vm.decode(0x882636fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102749a0u64) }
        0x102749a0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102749a4u64) }
        0x102749a4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102749a8u64 + 4);
        vm.wr(37, 0x102749a8u64 + 4);
        Ok(0x102749a8u64) }
        0x102749a8 => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102749acu64) }
        0x102749ac => { vm.decode(0x2000416fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102749b0u64) }
        0x102749b0 => { vm.decode(0x9921d530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102749b4u64) }
        0x102749b4 => { vm.decode(0x888e32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102749b8u64) }
        0x102749b8 => { vm.decode(0x91995b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102749bcu64) }
        0x102749bc => { vm.decode(0x408a63efu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102749c0u64) }
        0x102749c0 => { vm.decode(0x8911d5b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102749c4u64) }
        0x102749c4 => { vm.decode(0x190994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102749c8u64) }
        0x102749c8 => { vm.decode(0x28243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102749ccu64) }
        0x102749cc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102749d0u64) }
        0x102749d0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102749d4u64 + 4);
        vm.wr(37, 0x102749d4u64 + 4);
        Ok(0x102749d4u64) }
        0x102749d4 => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102749d8u64) }
        0x102749d8 => { vm.decode(0x2204416fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102749dcu64) }
        0x102749dc => { vm.decode(0x8b215530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102749e0u64) }
        0x102749e0 => { vm.decode(0x88cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102749e4u64) }
        0x102749e4 => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102749e8u64) }
        0x102749e8 => { vm.decode(0x1b1114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102749ecu64) }
        0x102749ec => { vm.decode(0x8643afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102749f0u64) }
        0x102749f0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102749f4u64) }
        0x102749f4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102749f8u64 + 4);
        vm.wr(37, 0x102749f8u64 + 4);
        Ok(0x102749f8u64) }
        0x102749f8 => { vm.decode(0x66598419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102749fcu64) }
        0x102749fc => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274a00u64) }
        0x10274a00 => { vm.decode(0x8b39d530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274a04u64) }
        0x10274a04 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274a08u64) }
        0x10274a08 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274a0cu64) }
        0x10274a0c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274a10u64) }
        0x10274a10 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274a14u64 + 4);
        vm.wr(37, 0x10274a14u64 + 4);
        Ok(0x10274a14u64) }
        0x10274a14 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274a18u64) }
        0x10274a18 => { vm.decode(0x2204616fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274a1cu64) }
        0x10274a1c => { vm.decode(0x9b39d530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274a20u64) }
        0x10274a20 => { vm.decode(0x1b319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274a24u64) }
        0x10274a24 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274a28u64) }
        0x10274a28 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274a2cu64) }
        0x10274a2c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274a30u64 + 4);
        vm.wr(37, 0x10274a30u64 + 4);
        Ok(0x10274a30u64) }
        0x10274a30 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274a34u64) }
        0x10274a34 => { vm.decode(0x2880411du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 10764i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10274a38u64) }
        0x10274a38 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10274a3cu64) } }
        0x10274a3c => { vm.decode(0x0u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x10274a40u64) }
        0x10274a40 => { vm.decode(0x7f0bcf59u32); let d = 29i16; let s = 29i16; let s2 = -1i16; let imm = -1088i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274a44u64) }
        0x10274a44 => { vm.decode(0xb91855f0u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274a48u64) }
        0x10274a48 => { vm.decode(0xa91055f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274a4cu64) }
        0x10274a4c => { vm.decode(0xb90855b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274a50u64) }
        0x10274a50 => { vm.decode(0xa90055b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274a54u64) }
        0x10274a54 => { vm.decode(0xb9185430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 1048i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274a58u64) }
        0x10274a58 => { vm.decode(0xa9105430u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274a5cu64) }
        0x10274a5c => { vm.decode(0xb9081530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 1032i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274a60u64) }
        0x10274a60 => { vm.decode(0xa9001530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274a64u64) }
        0x10274a64 => { vm.decode(0x9b389cb0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274a68u64) }
        0x10274a68 => { vm.decode(0x8b309cb0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 1008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274a6cu64) }
        0x10274a6c => { vm.decode(0xc005875u32); let d = 16i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274a70u64) }
        0x10274a70 => { vm.decode(0x1395b78eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -8781824i64;
        vm.st(d, imm as u64);
        Ok(0x10274a74u64) }
        0x10274a74 => { vm.decode(0x20590b18u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 57512i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10274a78u64) }
        0x10274a78 => { vm.decode(0x1f95b78eu32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = -8781824i64;
        vm.st(d, imm as u64);
        Ok(0x10274a7cu64) }
        0x10274a7c => { vm.decode(0x4082012fu32); let d = 20i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274a80u64) }
        0x10274a80 => { vm.decode(0x884524f5u32); let d = 19i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10274a84u64) }
        0x10274a84 => { vm.decode(0x2084212fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274a88u64) }
        0x10274a88 => { vm.decode(0x19281430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274a8cu64) }
        0x10274a8c => { vm.decode(0x2284012fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274a90u64) }
        0x10274a90 => { vm.decode(0x19189430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274a94u64) }
        0x10274a94 => { vm.decode(0x2284212fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274a98u64) }
        0x10274a98 => { vm.decode(0x19109430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274a9cu64) }
        0x10274a9c => { vm.decode(0x2080016fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274aa0u64) }
        0x10274aa0 => { vm.decode(0x192814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274aa4u64) }
        0x10274aa4 => { vm.decode(0x2080216fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274aa8u64) }
        0x10274aa8 => { vm.decode(0x192014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274aacu64) }
        0x10274aac => { vm.decode(0x20a4202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274ab0u64) }
        0x10274ab0 => { vm.decode(0x19201430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274ab4u64) }
        0x10274ab4 => { vm.decode(0x22a4002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274ab8u64) }
        0x10274ab8 => { vm.decode(0x191814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274abcu64) }
        0x10274abc => { vm.decode(0x22a4202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274ac0u64) }
        0x10274ac0 => { vm.decode(0x19309430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274ac4u64) }
        0x10274ac4 => { vm.decode(0x20a0006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274ac8u64) }
        0x10274ac8 => { vm.decode(0x191014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274accu64) }
        0x10274acc => { vm.decode(0x20a0206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274ad0u64) }
        0x10274ad0 => { vm.decode(0x190814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274ad4u64) }
        0x10274ad4 => { vm.decode(0x22a0006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274ad8u64) }
        0x10274ad8 => { vm.decode(0x19181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274adcu64) }
        0x10274adc => { vm.decode(0x22a0206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274ae0u64) }
        0x10274ae0 => { vm.decode(0x19101430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274ae4u64) }
        0x10274ae4 => { vm.decode(0x20a4006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274ae8u64) }
        0x10274ae8 => { vm.decode(0x19289430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274aecu64) }
        0x10274aec => { vm.decode(0x20a4206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274af0u64) }
        0x10274af0 => { vm.decode(0x19209430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274af4u64) }
        0x10274af4 => { vm.decode(0x22a4006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274af8u64) }
        0x10274af8 => { vm.decode(0x191894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274afcu64) }
        0x10274afc => { vm.decode(0x22a4206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274b00u64) }
        0x10274b00 => { vm.decode(0x19381430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274b04u64) }
        0x10274b04 => { vm.decode(0x28a0002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274b08u64) }
        0x10274b08 => { vm.decode(0x19301430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274b0cu64) }
        0x10274b0c => { vm.decode(0x20a4002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274b10u64) }
        0x10274b10 => { vm.decode(0x190094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274b14u64) }
        0x10274b14 => { vm.decode(0x22a0202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274b18u64) }
        0x10274b18 => { vm.decode(0x190894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274b1cu64) }
        0x10274b1c => { vm.decode(0x22a0002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274b20u64) }
        0x10274b20 => { vm.decode(0x19089430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274b24u64) }
        0x10274b24 => { vm.decode(0x20a0202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274b28u64) }
        0x10274b28 => { vm.decode(0x19001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274b2cu64) }
        0x10274b2c => { vm.decode(0x6082002fu32); let d = 21i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274b30u64) }
        0x10274b30 => { vm.decode(0x20a0002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274b34u64) }
        0x10274b34 => { vm.decode(0x8c012fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274b38u64) }
        0x10274b38 => { vm.decode(0x91095b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274b3cu64) }
        0x10274b3c => { vm.decode(0x6280212fu32); let d = 17i16; let s = 5i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274b40u64) }
        0x10274b40 => { vm.decode(0x628a012fu32); let d = 23i16; let s = 5i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274b44u64) }
        0x10274b44 => { vm.decode(0x88212fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274b48u64) }
        0x10274b48 => { vm.decode(0x2028412fu32); let d = 3i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274b4cu64) }
        0x10274b4c => { vm.decode(0x82100019u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = 34i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274b50u64) }
        0x10274b50 => { vm.decode(0xb28dcb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274b54u64) }
        0x10274b54 => { vm.decode(0x8e309b18u32); let d = 30i16; let s = 7i16; let s2 = -1i16; let imm = 57360i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10274b58u64) }
        0x10274b58 => { vm.decode(0x4b2014f5u32); let d = 2i16; let s = 2i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10274b5cu64) }
        0x10274b5c => { vm.decode(0xb209db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274b60u64) }
        0x10274b60 => { vm.decode(0x605f3159u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 508i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274b64u64) }
        0x10274b64 => { vm.decode(0xb389d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274b68u64) }
        0x10274b68 => { vm.decode(0x1b309d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274b6cu64) }
        0x10274b6c => { vm.decode(0x692614f5u32); let d = 22i16; let s = 30i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10274b70u64) }
        0x10274b70 => { vm.decode(0x744b1159u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274b74u64) }
        0x10274b74 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274b78u64) }
        0x10274b78 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274b7cu64) }
        0x10274b7c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274b80u64 + 4);
        vm.wr(37, 0x10274b80u64 + 4);
        Ok(0x10274b80u64) }
        0x10274b80 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274b84u64) }
        0x10274b84 => { vm.decode(0x636134f5u32); let d = 1i16; let s = 30i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10274b88u64) }
        0x10274b88 => { vm.decode(0x28412fu32); let d = 2i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274b8cu64) }
        0x10274b8c => { vm.decode(0x1b209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274b90u64) }
        0x10274b90 => { vm.decode(0x604ba159u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 468i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274b94u64) }
        0x10274b94 => { vm.decode(0x1b389c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274b98u64) }
        0x10274b98 => { vm.decode(0xb309d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274b9cu64) }
        0x10274b9c => { vm.decode(0x12149019u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 40i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274ba0u64) }
        0x10274ba0 => { vm.decode(0x9b28ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274ba4u64) }
        0x10274ba4 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274ba8u64) }
        0x10274ba8 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274bacu64) }
        0x10274bac => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274bb0u64 + 4);
        vm.wr(37, 0x10274bb0u64 + 4);
        Ok(0x10274bb0u64) }
        0x10274bb0 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274bb4u64) }
        0x10274bb4 => { vm.decode(0x616114f5u32); let d = 1i16; let s = 17i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10274bb8u64) }
        0x10274bb8 => { vm.decode(0x28412fu32); let d = 2i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274bbcu64) }
        0x10274bbc => { vm.decode(0x1b209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274bc0u64) }
        0x10274bc0 => { vm.decode(0x601fa059u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 428i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274bc4u64) }
        0x10274bc4 => { vm.decode(0x1b389c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274bc8u64) }
        0x10274bc8 => { vm.decode(0xb309d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274bccu64) }
        0x10274bcc => { vm.decode(0x9b28ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274bd0u64) }
        0x10274bd0 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274bd4u64) }
        0x10274bd4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274bd8u64) }
        0x10274bd8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274bdcu64 + 4);
        vm.wr(37, 0x10274bdcu64 + 4);
        Ok(0x10274bdcu64) }
        0x10274bdc => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274be0u64) }
        0x10274be0 => { vm.decode(0x2020412fu32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274be4u64) }
        0x10274be4 => { vm.decode(0x9b28ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274be8u64) }
        0x10274be8 => { vm.decode(0xa8c432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274becu64) }
        0x10274bec => { vm.decode(0x4b2014f5u32); let d = 2i16; let s = 2i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10274bf0u64) }
        0x10274bf0 => { vm.decode(0xb209db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274bf4u64) }
        0x10274bf4 => { vm.decode(0x600b3059u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 388i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274bf8u64) }
        0x10274bf8 => { vm.decode(0xb389d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274bfcu64) }
        0x10274bfc => { vm.decode(0x1b309c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274c00u64) }
        0x10274c00 => { vm.decode(0x893055b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274c04u64) }
        0x10274c04 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274c08u64) }
        0x10274c08 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274c0cu64) }
        0x10274c0c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274c10u64 + 4);
        vm.wr(37, 0x10274c10u64 + 4);
        Ok(0x10274c10u64) }
        0x10274c10 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274c14u64) }
        0x10274c14 => { vm.decode(0x19d5b78eu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = -8781824i64;
        vm.st(d, imm as u64);
        Ok(0x10274c18u64) }
        0x10274c18 => { vm.decode(0x69100a18u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 57408i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10274c1cu64) }
        0x10274c1c => { vm.decode(0x804124f5u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10274c20u64) }
        0x10274c20 => { vm.decode(0x8c612fu32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274c24u64) }
        0x10274c24 => { vm.decode(0x9009530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274c28u64) }
        0x10274c28 => { vm.decode(0x8c412fu32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274c2cu64) }
        0x10274c2c => { vm.decode(0x93815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274c30u64) }
        0x10274c30 => { vm.decode(0x288612fu32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274c34u64) }
        0x10274c34 => { vm.decode(0x90015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274c38u64) }
        0x10274c38 => { vm.decode(0x88612fu32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274c3cu64) }
        0x10274c3c => { vm.decode(0x91095b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274c40u64) }
        0x10274c40 => { vm.decode(0x88412fu32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274c44u64) }
        0x10274c44 => { vm.decode(0x9081530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274c48u64) }
        0x10274c48 => { vm.decode(0x6280412fu32); let d = 17i16; let s = 21i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274c4cu64) }
        0x10274c4c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274c50u64) }
        0x10274c50 => { vm.decode(0x1b209c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274c54u64) }
        0x10274c54 => { vm.decode(0x2080432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274c58u64) }
        0x10274c58 => { vm.decode(0x612214f5u32); let d = 4i16; let s = 30i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10274c5cu64) }
        0x10274c5c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274c60u64) }
        0x10274c60 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274c64u64 + 4);
        vm.wr(37, 0x10274c64u64 + 4);
        Ok(0x10274c64u64) }
        0x10274c64 => { vm.decode(0x660b8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 960i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274c68u64) }
        0x10274c68 => { vm.decode(0xd98e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10274c6cu64) }
        0x10274c6c => { vm.decode(0x3251062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10274c70u64) }
        0x10274c70 => { vm.decode(0x6a110a18u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 57392i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10274c74u64) }
        0x10274c74 => { vm.decode(0x19d5b78eu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = -8781824i64;
        vm.st(d, imm as u64);
        Ok(0x10274c78u64) }
        0x10274c78 => { vm.decode(0x804124f5u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10274c7cu64) }
        0x10274c7c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274c80u64) }
        0x10274c80 => { vm.decode(0x9b189cb0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274c84u64) }
        0x10274c84 => { vm.decode(0x72498159u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274c88u64) }
        0x10274c88 => { vm.decode(0x9b10dcb0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274c8cu64) }
        0x10274c8c => { vm.decode(0x1b089cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274c90u64) }
        0x10274c90 => { vm.decode(0x2880632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274c94u64) }
        0x10274c94 => { vm.decode(0x612214f5u32); let d = 4i16; let s = 30i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10274c98u64) }
        0x10274c98 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274c9cu64) }
        0x10274c9c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274ca0u64 + 4);
        vm.wr(37, 0x10274ca0u64 + 4);
        Ok(0x10274ca0u64) }
        0x10274ca0 => { vm.decode(0x661f8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274ca4u64) }
        0x10274ca4 => { vm.decode(0x6b500a18u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 57432i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10274ca8u64) }
        0x10274ca8 => { vm.decode(0x804124f5u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10274cacu64) }
        0x10274cac => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274cb0u64) }
        0x10274cb0 => { vm.decode(0x288c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274cb4u64) }
        0x10274cb4 => { vm.decode(0xb009db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274cb8u64) }
        0x10274cb8 => { vm.decode(0x601d1159u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274cbcu64) }
        0x10274cbc => { vm.decode(0x9089530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274cc0u64) }
        0x10274cc0 => { vm.decode(0xb189d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274cc4u64) }
        0x10274cc4 => { vm.decode(0x1b109c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274cc8u64) }
        0x10274cc8 => { vm.decode(0x2884632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274cccu64) }
        0x10274ccc => { vm.decode(0x612214f5u32); let d = 4i16; let s = 30i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10274cd0u64) }
        0x10274cd0 => { vm.decode(0x664b8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274cd4u64) }
        0x10274cd4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274cd8u64) }
        0x10274cd8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274cdcu64 + 4);
        vm.wr(37, 0x10274cdcu64 + 4);
        Ok(0x10274cdcu64) }
        0x10274cdc => { vm.decode(0x908d4b0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274ce0u64) }
        0x10274ce0 => { vm.decode(0x6a580a18u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 57496i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10274ce4u64) }
        0x10274ce4 => { vm.decode(0x804124f5u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10274ce8u64) }
        0x10274ce8 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274cecu64) }
        0x10274cec => { vm.decode(0x288c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274cf0u64) }
        0x10274cf0 => { vm.decode(0xb089d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274cf4u64) }
        0x10274cf4 => { vm.decode(0x76199019u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274cf8u64) }
        0x10274cf8 => { vm.decode(0x9b00dd30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274cfcu64) }
        0x10274cfc => { vm.decode(0x1b381cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274d00u64) }
        0x10274d00 => { vm.decode(0x2884432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274d04u64) }
        0x10274d04 => { vm.decode(0x612214f5u32); let d = 4i16; let s = 30i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10274d08u64) }
        0x10274d08 => { vm.decode(0x665d8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274d0cu64) }
        0x10274d0c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274d10u64) }
        0x10274d10 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274d14u64 + 4);
        vm.wr(37, 0x10274d14u64 + 4);
        Ok(0x10274d14u64) }
        0x10274d14 => { vm.decode(0x900d4b0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274d18u64) }
        0x10274d18 => { vm.decode(0x2280432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274d1cu64) }
        0x10274d1c => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10274d20u64) }
        0x10274d20 => { vm.decode(0x19101430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274d24u64) }
        0x10274d24 => { vm.decode(0x2280632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274d28u64) }
        0x10274d28 => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10274d2cu64) }
        0x10274d2c => { vm.decode(0x19181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274d30u64) }
        0x10274d30 => { vm.decode(0x2084632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274d34u64) }
        0x10274d34 => { vm.decode(0x692614f5u32); let d = 22i16; let s = 30i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10274d38u64) }
        0x10274d38 => { vm.decode(0x2284432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274d3cu64) }
        0x10274d3c => { vm.decode(0x696414f5u32); let d = 19i16; let s = 30i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10274d40u64) }
        0x10274d40 => { vm.decode(0x2a80436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274d44u64) }
        0x10274d44 => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10274d48u64) }
        0x10274d48 => { vm.decode(0x19309430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274d4cu64) }
        0x10274d4c => { vm.decode(0x2284632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274d50u64) }
        0x10274d50 => { vm.decode(0x692414f5u32); let d = 18i16; let s = 30i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10274d54u64) }
        0x10274d54 => { vm.decode(0x2080436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274d58u64) }
        0x10274d58 => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10274d5cu64) }
        0x10274d5c => { vm.decode(0x19001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274d60u64) }
        0x10274d60 => { vm.decode(0x2080636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274d64u64) }
        0x10274d64 => { vm.decode(0x616414f5u32); let d = 17i16; let s = 30i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10274d68u64) }
        0x10274d68 => { vm.decode(0x288e36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274d6cu64) }
        0x10274d6c => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10274d70u64).wrapping_add((imm * 4) as u64) } else { 0x10274d70u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10274d70u64) }
        0x10274d70 => { vm.decode(0x412734f5u32); let d = 20i16; let s = 30i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10274d74u64) }
        0x10274d74 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10274d78u64) }
        0x10274d78 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10274d7cu64) }
        0x10274d7c => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10274d80u64) }
        0x10274d80 => { vm.decode(0x288e36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274d84u64) }
        0x10274d84 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10274d88u64) }
        0x10274d88 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x10274d8cu64).wrapping_add((imm * 4) as u64) } else { 0x10274d8cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10274d8cu64) }
        0x10274d8c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10274d90u64) } }
        0x10274d90 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10274d94u64).wrapping_add((imm * 4) as u64) } else { 0x10274d94u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10274d94u64) }
        0x10274de8 => { vm.decode(0x2a84632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274decu64) }
        0x10274dec => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10274df0u64) }
        0x10274df0 => { vm.decode(0x19201430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274df4u64) }
        0x10274df4 => { vm.decode(0x2880436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274df8u64) }
        0x10274df8 => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10274dfcu64) }
        0x10274dfc => { vm.decode(0x191814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274e00u64) }
        0x10274e00 => { vm.decode(0x2880636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274e04u64) }
        0x10274e04 => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10274e08u64) }
        0x10274e08 => { vm.decode(0x191014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274e0cu64) }
        0x10274e0c => { vm.decode(0x2a80402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274e10u64) }
        0x10274e10 => { vm.decode(0x9b305d30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274e14u64) }
        0x10274e14 => { vm.decode(0x1b281c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274e18u64) }
        0x10274e18 => { vm.decode(0xa82436fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274e1cu64) }
        0x10274e1c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274e20u64) }
        0x10274e20 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274e24u64 + 4);
        vm.wr(37, 0x10274e24u64 + 4);
        Ok(0x10274e24u64) }
        0x10274e24 => { vm.decode(0x660d8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274e28u64) }
        0x10274e28 => { vm.decode(0x664d8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274e2cu64) }
        0x10274e2c => { vm.decode(0x2080602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274e30u64) }
        0x10274e30 => { vm.decode(0x9b205cb0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274e34u64) }
        0x10274e34 => { vm.decode(0x1b381c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274e38u64) }
        0x10274e38 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274e3cu64) }
        0x10274e3c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274e40u64 + 4);
        vm.wr(37, 0x10274e40u64 + 4);
        Ok(0x10274e40u64) }
        0x10274e40 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274e44u64) }
        0x10274e44 => { vm.decode(0x2080632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274e48u64) }
        0x10274e48 => { vm.decode(0x820bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10274e4cu64) }
        0x10274e4c => { vm.decode(0x664d8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274e50u64) }
        0x10274e50 => { vm.decode(0x8c606fu32); let d = 2i16; let s = 20i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274e54u64) }
        0x10274e54 => { vm.decode(0x208c432fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274e58u64) }
        0x10274e58 => { vm.decode(0x2228012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274e5cu64) }
        0x10274e5c => { vm.decode(0x286e36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274e60u64) }
        0x10274e60 => { vm.decode(0xb205c30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274e64u64) }
        0x10274e64 => { vm.decode(0x1b181db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274e68u64) }
        0x10274e68 => { vm.decode(0x230d3062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10274e6cu64) }
        0x10274e6c => { vm.decode(0x9b081cb0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274e70u64) }
        0x10274e70 => { vm.decode(0x9b005cb0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274e74u64) }
        0x10274e74 => { vm.decode(0xb181d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274e78u64) }
        0x10274e78 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274e7cu64) }
        0x10274e7c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274e80u64 + 4);
        vm.wr(37, 0x10274e80u64 + 4);
        Ok(0x10274e80u64) }
        0x10274e80 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274e84u64) }
        0x10274e84 => { vm.decode(0x665f8119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 760i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274e88u64) }
        0x10274e88 => { vm.decode(0x700d9059u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274e8cu64) }
        0x10274e8c => { vm.decode(0x700b8059u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274e90u64) }
        0x10274e90 => { vm.decode(0x8236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10274e94u64) }
        0x10274e94 => { vm.decode(0x2280406fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274e98u64) }
        0x10274e98 => { vm.decode(0x20d7062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10274e9cu64) }
        0x10274e9c => { vm.decode(0x9b081c30u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274ea0u64) }
        0x10274ea0 => { vm.decode(0x9b001d30u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274ea4u64) }
        0x10274ea4 => { vm.decode(0x1b3894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 760i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274ea8u64) }
        0x10274ea8 => { vm.decode(0x89285530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274eacu64) }
        0x10274eac => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274eb0u64) }
        0x10274eb0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274eb4u64 + 4);
        vm.wr(37, 0x10274eb4u64 + 4);
        Ok(0x10274eb4u64) }
        0x10274eb4 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274eb8u64) }
        0x10274eb8 => { vm.decode(0x72191059u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274ebcu64) }
        0x10274ebc => { vm.decode(0x2280602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274ec0u64) }
        0x10274ec0 => { vm.decode(0xa8c432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274ec4u64) }
        0x10274ec4 => { vm.decode(0xb3095b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274ec8u64) }
        0x10274ec8 => { vm.decode(0x99289530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274eccu64) }
        0x10274ecc => { vm.decode(0x9b2895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274ed0u64) }
        0x10274ed0 => { vm.decode(0x8b20d5b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274ed4u64) }
        0x10274ed4 => { vm.decode(0x1b389430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274ed8u64) }
        0x10274ed8 => { vm.decode(0x282632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274edcu64) }
        0x10274edc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274ee0u64) }
        0x10274ee0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274ee4u64 + 4);
        vm.wr(37, 0x10274ee4u64 + 4);
        Ok(0x10274ee4u64) }
        0x10274ee4 => { vm.decode(0x664f8119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 728i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274ee8u64) }
        0x10274ee8 => { vm.decode(0x605b9119u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274eecu64) }
        0x10274eec => { vm.decode(0x8002036u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10274ef0u64) }
        0x10274ef0 => { vm.decode(0x288402fu32); let d = 2i16; let s = 20i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274ef4u64) }
        0x10274ef4 => { vm.decode(0x53022a73u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10274ef8u64) }
        0x10274ef8 => { vm.decode(0x8920d530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274efcu64) }
        0x10274efc => { vm.decode(0x8b28d530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274f00u64) }
        0x10274f00 => { vm.decode(0x191095b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274f04u64) }
        0x10274f04 => { vm.decode(0x1b209530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274f08u64) }
        0x10274f08 => { vm.decode(0xb1895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274f0cu64) }
        0x10274f0c => { vm.decode(0x282432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274f10u64) }
        0x10274f10 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274f14u64) }
        0x10274f14 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274f18u64 + 4);
        vm.wr(37, 0x10274f18u64 + 4);
        Ok(0x10274f18u64) }
        0x10274f18 => { vm.decode(0x665f8019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 696i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274f1cu64) }
        0x10274f1c => { vm.decode(0x605d9059u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274f20u64) }
        0x10274f20 => { vm.decode(0x2080606fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274f24u64) }
        0x10274f24 => { vm.decode(0xa8c436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274f28u64) }
        0x10274f28 => { vm.decode(0xb009db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274f2cu64) }
        0x10274f2c => { vm.decode(0x191895b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274f30u64) }
        0x10274f30 => { vm.decode(0x1b189d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274f34u64) }
        0x10274f34 => { vm.decode(0x1b109c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274f38u64) }
        0x10274f38 => { vm.decode(0x886632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274f3cu64) }
        0x10274f3c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274f40u64) }
        0x10274f40 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274f44u64 + 4);
        vm.wr(37, 0x10274f44u64 + 4);
        Ok(0x10274f44u64) }
        0x10274f44 => { vm.decode(0x664b8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274f48u64) }
        0x10274f48 => { vm.decode(0x2880602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274f4cu64) }
        0x10274f4c => { vm.decode(0xa8c436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274f50u64) }
        0x10274f50 => { vm.decode(0xb089d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274f54u64) }
        0x10274f54 => { vm.decode(0x9b00dd30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274f58u64) }
        0x10274f58 => { vm.decode(0x1b381cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274f5cu64) }
        0x10274f5c => { vm.decode(0x886432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274f60u64) }
        0x10274f60 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274f64u64) }
        0x10274f64 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274f68u64 + 4);
        vm.wr(37, 0x10274f68u64 + 4);
        Ok(0x10274f68u64) }
        0x10274f68 => { vm.decode(0x665d8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274f6cu64) }
        0x10274f6c => { vm.decode(0xa8c636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274f70u64) }
        0x10274f70 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10274f74u64).wrapping_add((imm * 4) as u64) } else { 0x10274f74u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10274f74u64) }
        0x10274f74 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10274f78u64) } }
        0x10274f78 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10274f7cu64) }
        0x10274f7c => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10274f80u64) }
        0x10274f80 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10274f84u64) }
        0x10274f84 => { vm.decode(0xa8c636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274f88u64) }
        0x10274f88 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10274f8cu64) }
        0x10274f8c => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x10274f90u64).wrapping_add((imm * 4) as u64) } else { 0x10274f90u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10274f90u64) }
        0x10274f90 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10274f94u64) } }
        0x10274f94 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10274f98u64).wrapping_add((imm * 4) as u64) } else { 0x10274f98u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10274f98u64) }
        0x10274f98 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10274f9cu64) } }
        0x10274f9c => { vm.decode(0x2880402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274fa0u64) }
        0x10274fa0 => { vm.decode(0xb301db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274fa4u64) }
        0x10274fa4 => { vm.decode(0x1b281cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274fa8u64) }
        0x10274fa8 => { vm.decode(0x82432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274facu64) }
        0x10274fac => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274fb0u64) }
        0x10274fb0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274fb4u64 + 4);
        vm.wr(37, 0x10274fb4u64 + 4);
        Ok(0x10274fb4u64) }
        0x10274fb4 => { vm.decode(0x661d8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274fb8u64) }
        0x10274fb8 => { vm.decode(0x93894b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274fbcu64) }
        0x10274fbc => { vm.decode(0xa8c436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274fc0u64) }
        0x10274fc0 => { vm.decode(0x8000a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10274fc4u64).wrapping_add((imm * 4) as u64) } else { 0x10274fc4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10274fc4u64) }
        0x10274fc4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10274fc8u64) } }
        0x10274fc8 => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274fccu64) }
        0x10274fcc => { vm.decode(0x2000212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274fd0u64) }
        0x10274fd0 => { vm.decode(0xb201db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274fd4u64) }
        0x10274fd4 => { vm.decode(0x1b381c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274fd8u64) }
        0x10274fd8 => { vm.decode(0x664d8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10274fdcu64) }
        0x10274fdc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274fe0u64) }
        0x10274fe0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10274fe4u64 + 4);
        vm.wr(37, 0x10274fe4u64 + 4);
        Ok(0x10274fe4u64) }
        0x10274fe4 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10274fe8u64) }
        0x10274fe8 => { vm.decode(0x93094b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274fecu64) }
        0x10274fec => { vm.decode(0x2280436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274ff0u64) }
        0x10274ff0 => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10274ff4u64) }
        0x10274ff4 => { vm.decode(0x19301430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10274ff8u64) }
        0x10274ff8 => { vm.decode(0x2280636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10274ffcu64) }
        0x10274ffc => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10275000u64) }
        0x10275000 => { vm.decode(0x19381430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275004u64) }
        0x10275004 => { vm.decode(0x2084436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275008u64) }
        0x10275008 => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027500cu64) }
        0x1027500c => { vm.decode(0x192014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275010u64) }
        0x10275010 => { vm.decode(0x2084636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275014u64) }
        0x10275014 => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10275018u64) }
        0x10275018 => { vm.decode(0x192814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027501cu64) }
        0x1027501c => { vm.decode(0x2a80432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275020u64) }
        0x10275020 => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10275024u64) }
        0x10275024 => { vm.decode(0x190814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275028u64) }
        0x10275028 => { vm.decode(0x2a80632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027502cu64) }
        0x1027502c => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10275030u64) }
        0x10275030 => { vm.decode(0x190014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275034u64) }
        0x10275034 => { vm.decode(0x2a80402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275038u64) }
        0x10275038 => { vm.decode(0x9b305d30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027503cu64) }
        0x1027503c => { vm.decode(0x1b281c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275040u64) }
        0x10275040 => { vm.decode(0xa82436fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275044u64) }
        0x10275044 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275048u64) }
        0x10275048 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027504cu64 + 4);
        vm.wr(37, 0x1027504cu64 + 4);
        Ok(0x1027504cu64) }
        0x1027504c => { vm.decode(0x660d8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275050u64) }
        0x10275050 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275054u64) }
        0x10275054 => { vm.decode(0x744d9159u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275058u64) }
        0x10275058 => { vm.decode(0x488a436fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027505cu64) }
        0x1027505c => { vm.decode(0x8b205db0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275060u64) }
        0x10275060 => { vm.decode(0x1b381c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275064u64) }
        0x10275064 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275068u64) }
        0x10275068 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027506cu64) }
        0x1027506c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275070u64 + 4);
        vm.wr(37, 0x10275070u64 + 4);
        Ok(0x10275070u64) }
        0x10275070 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275074u64) }
        0x10275074 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275078u64) }
        0x10275078 => { vm.decode(0x488a6b6fu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027507cu64) }
        0x1027507c => { vm.decode(0x8b205df0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275080u64) }
        0x10275080 => { vm.decode(0x1b381c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275084u64) }
        0x10275084 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275088u64) }
        0x10275088 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027508cu64) }
        0x1027508c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275090u64 + 4);
        vm.wr(37, 0x10275090u64 + 4);
        Ok(0x10275090u64) }
        0x10275090 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275094u64) }
        0x10275094 => { vm.decode(0x2280406fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275098u64) }
        0x10275098 => { vm.decode(0x8236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027509cu64) }
        0x1027509c => { vm.decode(0x20d7062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102750a0u64) }
        0x102750a0 => { vm.decode(0x9b081c30u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102750a4u64) }
        0x102750a4 => { vm.decode(0x8b005d30u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102750a8u64) }
        0x102750a8 => { vm.decode(0x1b3894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 760i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102750acu64) }
        0x102750ac => { vm.decode(0x82636fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102750b0u64) }
        0x102750b0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102750b4u64) }
        0x102750b4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102750b8u64 + 4);
        vm.wr(37, 0x102750b8u64 + 4);
        Ok(0x102750b8u64) }
        0x102750b8 => { vm.decode(0x665f8119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 760i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102750bcu64) }
        0x102750bc => { vm.decode(0x2080602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102750c0u64) }
        0x102750c0 => { vm.decode(0x8b205df0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102750c4u64) }
        0x102750c4 => { vm.decode(0x1b381c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102750c8u64) }
        0x102750c8 => { vm.decode(0x89189530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102750ccu64) }
        0x102750cc => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102750d0u64) }
        0x102750d0 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102750d4u64) }
        0x102750d4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102750d8u64 + 4);
        vm.wr(37, 0x102750d8u64 + 4);
        Ok(0x102750d8u64) }
        0x102750d8 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102750dcu64) }
        0x102750dc => { vm.decode(0x2284406fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102750e0u64) }
        0x102750e0 => { vm.decode(0xa8c632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102750e4u64) }
        0x102750e4 => { vm.decode(0xb3095b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102750e8u64) }
        0x102750e8 => { vm.decode(0x8b28d5f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102750ecu64) }
        0x102750ec => { vm.decode(0x9910d430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102750f0u64) }
        0x102750f0 => { vm.decode(0x9b20d4b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102750f4u64) }
        0x102750f4 => { vm.decode(0x1b389430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102750f8u64) }
        0x102750f8 => { vm.decode(0x286432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102750fcu64) }
        0x102750fc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275100u64) }
        0x10275100 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275104u64 + 4);
        vm.wr(37, 0x10275104u64 + 4);
        Ok(0x10275104u64) }
        0x10275104 => { vm.decode(0x664f8119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 728i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275108u64) }
        0x10275108 => { vm.decode(0x2284606fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027510cu64) }
        0x1027510c => { vm.decode(0x8b08d5f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275110u64) }
        0x10275110 => { vm.decode(0x8b00d5b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275114u64) }
        0x10275114 => { vm.decode(0x1b189430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275118u64) }
        0x10275118 => { vm.decode(0x286632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027511cu64) }
        0x1027511c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275120u64) }
        0x10275120 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275124u64 + 4);
        vm.wr(37, 0x10275124u64 + 4);
        Ok(0x10275124u64) }
        0x10275124 => { vm.decode(0x664f8019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275128u64) }
        0x10275128 => { vm.decode(0x731b1119u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027512cu64) }
        0x1027512c => { vm.decode(0x2080402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275130u64) }
        0x10275130 => { vm.decode(0x8b10d530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275134u64) }
        0x10275134 => { vm.decode(0x8b08d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275138u64) }
        0x10275138 => { vm.decode(0x1b009430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027513cu64) }
        0x1027513c => { vm.decode(0x82436fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275140u64) }
        0x10275140 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275144u64) }
        0x10275144 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275148u64 + 4);
        vm.wr(37, 0x10275148u64 + 4);
        Ok(0x10275148u64) }
        0x10275148 => { vm.decode(0x660b8019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027514cu64) }
        0x1027514c => { vm.decode(0x2080606fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275150u64) }
        0x10275150 => { vm.decode(0x88c436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275154u64) }
        0x10275154 => { vm.decode(0xb009db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275158u64) }
        0x10275158 => { vm.decode(0xa8c432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027515cu64) }
        0x1027515c => { vm.decode(0xb189d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275160u64) }
        0x10275160 => { vm.decode(0x1b109c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275164u64) }
        0x10275164 => { vm.decode(0x886632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275168u64) }
        0x10275168 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027516cu64) }
        0x1027516c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275170u64 + 4);
        vm.wr(37, 0x10275170u64 + 4);
        Ok(0x10275170u64) }
        0x10275170 => { vm.decode(0x664b8059u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275174u64) }
        0x10275174 => { vm.decode(0x2880602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275178u64) }
        0x10275178 => { vm.decode(0x88c436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027517cu64) }
        0x1027517c => { vm.decode(0xb089d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275180u64) }
        0x10275180 => { vm.decode(0x9b00dd30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275184u64) }
        0x10275184 => { vm.decode(0x1b381cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275188u64) }
        0x10275188 => { vm.decode(0x886432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027518cu64) }
        0x1027518c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275190u64) }
        0x10275190 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275194u64 + 4);
        vm.wr(37, 0x10275194u64 + 4);
        Ok(0x10275194u64) }
        0x10275194 => { vm.decode(0x665d8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275198u64) }
        0x10275198 => { vm.decode(0x88c636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027519cu64) }
        0x1027519c => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102751a0u64).wrapping_add((imm * 4) as u64) } else { 0x102751a0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102751a0u64) }
        0x102751a0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102751a4u64) } }
        0x102751a4 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102751a8u64) }
        0x102751a8 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102751acu64) }
        0x102751ac => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102751b0u64) }
        0x102751b0 => { vm.decode(0x88c636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102751b4u64) }
        0x102751b4 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102751b8u64) }
        0x102751b8 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x102751bcu64).wrapping_add((imm * 4) as u64) } else { 0x102751bcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102751bcu64) }
        0x102751bc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102751c0u64) } }
        0x102751c0 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102751c4u64).wrapping_add((imm * 4) as u64) } else { 0x102751c4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102751c4u64) }
        0x102751c4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102751c8u64) } }
        0x102751c8 => { vm.decode(0x2880402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102751ccu64) }
        0x102751cc => { vm.decode(0xb301db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102751d0u64) }
        0x102751d0 => { vm.decode(0x1b281cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102751d4u64) }
        0x102751d4 => { vm.decode(0x82432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102751d8u64) }
        0x102751d8 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102751dcu64) }
        0x102751dc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102751e0u64 + 4);
        vm.wr(37, 0x102751e0u64 + 4);
        Ok(0x102751e0u64) }
        0x102751e0 => { vm.decode(0x661d8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102751e4u64) }
        0x102751e4 => { vm.decode(0x92894b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102751e8u64) }
        0x102751e8 => { vm.decode(0x88c436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102751ecu64) }
        0x102751ec => { vm.decode(0x8000a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102751f0u64).wrapping_add((imm * 4) as u64) } else { 0x102751f0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102751f0u64) }
        0x102751f0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102751f4u64) } }
        0x102751f4 => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102751f8u64) }
        0x102751f8 => { vm.decode(0x2000212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102751fcu64) }
        0x102751fc => { vm.decode(0xb201db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275200u64) }
        0x10275200 => { vm.decode(0x1b381c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275204u64) }
        0x10275204 => { vm.decode(0xa82632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275208u64) }
        0x10275208 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027520cu64) }
        0x1027520c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275210u64 + 4);
        vm.wr(37, 0x10275210u64 + 4);
        Ok(0x10275210u64) }
        0x10275210 => { vm.decode(0x664d8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275214u64) }
        0x10275214 => { vm.decode(0x92094b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275218u64) }
        0x10275218 => { vm.decode(0x2a80402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027521cu64) }
        0x1027521c => { vm.decode(0x9b305d30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275220u64) }
        0x10275220 => { vm.decode(0x1b281c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275224u64) }
        0x10275224 => { vm.decode(0xa82436fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275228u64) }
        0x10275228 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027522cu64) }
        0x1027522c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275230u64 + 4);
        vm.wr(37, 0x10275230u64 + 4);
        Ok(0x10275230u64) }
        0x10275230 => { vm.decode(0x660d8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275234u64) }
        0x10275234 => { vm.decode(0x744b9159u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275238u64) }
        0x10275238 => { vm.decode(0x4a019u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 12i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027523cu64) }
        0x1027523c => { vm.decode(0xa88602fu32); let d = 2i16; let s = 20i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275240u64) }
        0x10275240 => { vm.decode(0x1b289cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275244u64) }
        0x10275244 => { vm.decode(0x2084432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275248u64) }
        0x10275248 => { vm.decode(0x1b209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027524cu64) }
        0x1027524c => { vm.decode(0x9b38dd30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275250u64) }
        0x10275250 => { vm.decode(0xb309d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275254u64) }
        0x10275254 => { vm.decode(0x428e436fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275258u64) }
        0x10275258 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027525cu64) }
        0x1027525c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275260u64) }
        0x10275260 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275264u64 + 4);
        vm.wr(37, 0x10275264u64 + 4);
        Ok(0x10275264u64) }
        0x10275264 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275268u64) }
        0x10275268 => { vm.decode(0x724fb119u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 220i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027526cu64) }
        0x1027526c => { vm.decode(0x90008099u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275270u64) }
        0x10275270 => { vm.decode(0x2a80602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275274u64) }
        0x10275274 => { vm.decode(0x9b289cb0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275278u64) }
        0x10275278 => { vm.decode(0x8c632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027527cu64) }
        0x1027527c => { vm.decode(0xb209db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275280u64) }
        0x10275280 => { vm.decode(0x9b38dd30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275284u64) }
        0x10275284 => { vm.decode(0x1b309c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275288u64) }
        0x10275288 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027528cu64) }
        0x1027528c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275290u64) }
        0x10275290 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275294u64 + 4);
        vm.wr(37, 0x10275294u64 + 4);
        Ok(0x10275294u64) }
        0x10275294 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275298u64) }
        0x10275298 => { vm.decode(0x724f8119u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027529cu64) }
        0x1027529c => { vm.decode(0x2a80602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102752a0u64) }
        0x102752a0 => { vm.decode(0x9b289cb0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102752a4u64) }
        0x102752a4 => { vm.decode(0x8c636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102752a8u64) }
        0x102752a8 => { vm.decode(0xb209db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102752acu64) }
        0x102752ac => { vm.decode(0x9b38dc30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102752b0u64) }
        0x102752b0 => { vm.decode(0x1b309c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102752b4u64) }
        0x102752b4 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102752b8u64) }
        0x102752b8 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102752bcu64) }
        0x102752bc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102752c0u64 + 4);
        vm.wr(37, 0x102752c0u64 + 4);
        Ok(0x102752c0u64) }
        0x102752c0 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102752c4u64) }
        0x102752c4 => { vm.decode(0x48019u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102752c8u64) }
        0x102752c8 => { vm.decode(0xa88602fu32); let d = 2i16; let s = 20i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102752ccu64) }
        0x102752cc => { vm.decode(0x1b289cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102752d0u64) }
        0x102752d0 => { vm.decode(0x2084436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102752d4u64) }
        0x102752d4 => { vm.decode(0x1b209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102752d8u64) }
        0x102752d8 => { vm.decode(0x8b38dd70u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102752dcu64) }
        0x102752dc => { vm.decode(0xb309d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102752e0u64) }
        0x102752e0 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102752e4u64) }
        0x102752e4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102752e8u64) }
        0x102752e8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102752ecu64 + 4);
        vm.wr(37, 0x102752ecu64 + 4);
        Ok(0x102752ecu64) }
        0x102752ec => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102752f0u64) }
        0x102752f0 => { vm.decode(0x74198119u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102752f4u64) }
        0x102752f4 => { vm.decode(0xa120236u32); let d = 22i16; let s = 0i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102752f8u64) }
        0x102752f8 => { vm.decode(0x2284602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102752fcu64) }
        0x102752fc => { vm.decode(0x624d7122u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10275300u64) }
        0x10275300 => { vm.decode(0x9b2854b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275304u64) }
        0x10275304 => { vm.decode(0x1b2014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275308u64) }
        0x10275308 => { vm.decode(0x4288636fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027530cu64) }
        0x1027530c => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275310u64) }
        0x10275310 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275314u64) }
        0x10275314 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275318u64 + 4);
        vm.wr(37, 0x10275318u64 + 4);
        Ok(0x10275318u64) }
        0x10275318 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027531cu64) }
        0x1027531c => { vm.decode(0x12036u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10275320u64) }
        0x10275320 => { vm.decode(0x88406fu32); let d = 2i16; let s = 20i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275324u64) }
        0x10275324 => { vm.decode(0x630d3022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10275328u64) }
        0x10275328 => { vm.decode(0x8b2855f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027532cu64) }
        0x1027532c => { vm.decode(0xb2015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275330u64) }
        0x10275330 => { vm.decode(0x628663efu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275334u64) }
        0x10275334 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275338u64) }
        0x10275338 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027533cu64) }
        0x1027533c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275340u64 + 4);
        vm.wr(37, 0x10275340u64 + 4);
        Ok(0x10275340u64) }
        0x10275340 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275344u64) }
        0x10275344 => { vm.decode(0x745d9019u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275348u64) }
        0x10275348 => { vm.decode(0x2280606fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027534cu64) }
        0x1027534c => { vm.decode(0x28c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275350u64) }
        0x10275350 => { vm.decode(0xb301530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275354u64) }
        0x10275354 => { vm.decode(0x9b285430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275358u64) }
        0x10275358 => { vm.decode(0x28c636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027535cu64) }
        0x1027535c => { vm.decode(0xb201530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275360u64) }
        0x10275360 => { vm.decode(0x1b1814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275364u64) }
        0x10275364 => { vm.decode(0x6282436fu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275368u64) }
        0x10275368 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027536cu64) }
        0x1027536c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275370u64) }
        0x10275370 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275374u64 + 4);
        vm.wr(37, 0x10275374u64 + 4);
        Ok(0x10275374u64) }
        0x10275374 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275378u64) }
        0x10275378 => { vm.decode(0x2084406fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027537cu64) }
        0x1027537c => { vm.decode(0x624d7122u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10275380u64) }
        0x10275380 => { vm.decode(0x9b2855b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275384u64) }
        0x10275384 => { vm.decode(0x1b2014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275388u64) }
        0x10275388 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027538cu64) }
        0x1027538c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275390u64) }
        0x10275390 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275394u64 + 4);
        vm.wr(37, 0x10275394u64 + 4);
        Ok(0x10275394u64) }
        0x10275394 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275398u64) }
        0x10275398 => { vm.decode(0x2280606fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027539cu64) }
        0x1027539c => { vm.decode(0x88c32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102753a0u64) }
        0x102753a0 => { vm.decode(0xb301530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102753a4u64) }
        0x102753a4 => { vm.decode(0x28c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102753a8u64) }
        0x102753a8 => { vm.decode(0xb281530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102753acu64) }
        0x102753ac => { vm.decode(0x888432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102753b0u64) }
        0x102753b0 => { vm.decode(0xb201530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102753b4u64) }
        0x102753b4 => { vm.decode(0x1b1814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102753b8u64) }
        0x102753b8 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102753bcu64) }
        0x102753bc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102753c0u64) }
        0x102753c0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102753c4u64 + 4);
        vm.wr(37, 0x102753c4u64 + 4);
        Ok(0x102753c4u64) }
        0x102753c4 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102753c8u64) }
        0x102753c8 => { vm.decode(0x744d8159u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102753ccu64) }
        0x102753cc => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102753d0u64) }
        0x102753d0 => { vm.decode(0xa8c432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102753d4u64) }
        0x102753d4 => { vm.decode(0xb201db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102753d8u64) }
        0x102753d8 => { vm.decode(0x1b381c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102753dcu64) }
        0x102753dc => { vm.decode(0x4a88632fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102753e0u64) }
        0x102753e0 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102753e4u64) }
        0x102753e4 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102753e8u64) }
        0x102753e8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102753ecu64 + 4);
        vm.wr(37, 0x102753ecu64 + 4);
        Ok(0x102753ecu64) }
        0x102753ec => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102753f0u64) }
        0x102753f0 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102753f4u64) }
        0x102753f4 => { vm.decode(0x888636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102753f8u64) }
        0x102753f8 => { vm.decode(0xb201db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102753fcu64) }
        0x102753fc => { vm.decode(0x1b381c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275400u64) }
        0x10275400 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275404u64) }
        0x10275404 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275408u64) }
        0x10275408 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027540cu64 + 4);
        vm.wr(37, 0x1027540cu64 + 4);
        Ok(0x1027540cu64) }
        0x1027540c => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275410u64) }
        0x10275410 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275414u64) }
        0x10275414 => { vm.decode(0x888436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275418u64) }
        0x10275418 => { vm.decode(0xb201db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027541cu64) }
        0x1027541c => { vm.decode(0x1b381c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275420u64) }
        0x10275420 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275424u64) }
        0x10275424 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275428u64) }
        0x10275428 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027542cu64 + 4);
        vm.wr(37, 0x1027542cu64 + 4);
        Ok(0x1027542cu64) }
        0x1027542c => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275430u64) }
        0x10275430 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275434u64) }
        0x10275434 => { vm.decode(0xa8c632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275438u64) }
        0x10275438 => { vm.decode(0xb201db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027543cu64) }
        0x1027543c => { vm.decode(0x1b381c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275440u64) }
        0x10275440 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275444u64) }
        0x10275444 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275448u64) }
        0x10275448 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027544cu64 + 4);
        vm.wr(37, 0x1027544cu64 + 4);
        Ok(0x1027544cu64) }
        0x1027544c => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275450u64) }
        0x10275450 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275454u64) }
        0x10275454 => { vm.decode(0xa88432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275458u64) }
        0x10275458 => { vm.decode(0xb201db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027545cu64) }
        0x1027545c => { vm.decode(0x1b381c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275460u64) }
        0x10275460 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275464u64) }
        0x10275464 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275468u64) }
        0x10275468 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027546cu64 + 4);
        vm.wr(37, 0x1027546cu64 + 4);
        Ok(0x1027546cu64) }
        0x1027546c => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275470u64) }
        0x10275470 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275474u64) }
        0x10275474 => { vm.decode(0x888632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275478u64) }
        0x10275478 => { vm.decode(0xb201db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027547cu64) }
        0x1027547c => { vm.decode(0x1b381c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275480u64) }
        0x10275480 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275484u64) }
        0x10275484 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275488u64) }
        0x10275488 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027548cu64 + 4);
        vm.wr(37, 0x1027548cu64 + 4);
        Ok(0x1027548cu64) }
        0x1027548c => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275490u64) }
        0x10275490 => { vm.decode(0x4a84c3efu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 1008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275494u64) }
        0x10275494 => { vm.decode(0x6a84e3efu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275498u64) }
        0x10275498 => { vm.decode(0x4088532fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027549cu64) }
        0x1027549c => { vm.decode(0x6088732fu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 1032i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102754a0u64) }
        0x102754a0 => { vm.decode(0x4282532fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102754a4u64) }
        0x102754a4 => { vm.decode(0x6282732fu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 1048i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102754a8u64) }
        0x102754a8 => { vm.decode(0x408e532fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102754acu64) }
        0x102754ac => { vm.decode(0x608e732fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102754b0u64) }
        0x102754b0 => { vm.decode(0x428e5b2fu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102754b4u64) }
        0x102754b4 => { vm.decode(0x628e7b2fu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102754b8u64) }
        0x102754b8 => { vm.decode(0x54104cf5u32); let d = -1i16; let s = -1i16; let s2 = -1i16; let imm = 0i64;
        vm.jump_flag = 2;
        vm.wr(36, vm.slots[31]);
        vm.wr(35, 2);
        Ok(0x102754bcu64) }
        0x102754bc => { vm.decode(0x73298119u32); let d = 29i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102754c0u64) }
        0x102754c0 => { vm.decode(0x7f2bcc59u32); let d = 29i16; let s = 29i16; let s2 = -1i16; let imm = -4224i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102754c4u64) }
        0x102754c4 => { vm.decode(0x9d3855f0u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 4216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102754c8u64) }
        0x102754c8 => { vm.decode(0x8d3055f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 4208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102754ccu64) }
        0x102754cc => { vm.decode(0x9d2855b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 4200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102754d0u64) }
        0x102754d0 => { vm.decode(0x8d2055b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 4192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102754d4u64) }
        0x102754d4 => { vm.decode(0x9d385430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 4184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102754d8u64) }
        0x102754d8 => { vm.decode(0x8d305430u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 4176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102754dcu64) }
        0x102754dc => { vm.decode(0x9d281530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 4168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102754e0u64) }
        0x102754e0 => { vm.decode(0x8d201530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 4160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102754e4u64) }
        0x102754e4 => { vm.decode(0x9d1814b0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 4152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102754e8u64) }
        0x102754e8 => { vm.decode(0x8d1014b0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 4144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102754ecu64) }
        0x102754ec => { vm.decode(0x4c005875u32); let d = 18i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102754f0u64) }
        0x102754f0 => { vm.decode(0x130db28eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -14614528i64;
        vm.st(d, imm as u64);
        Ok(0x102754f4u64) }
        0x102754f4 => { vm.decode(0x200d0bd8u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 26528i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102754f8u64) }
        0x102754f8 => { vm.decode(0x88012fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102754fcu64) }
        0x102754fc => { vm.decode(0xb001d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275500u64) }
        0x10275500 => { vm.decode(0x820604f5u32); let d = 20i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10275504u64) }
        0x10275504 => { vm.decode(0x2a84a12fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275508u64) }
        0x10275508 => { vm.decode(0x1b201cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027550cu64) }
        0x1027550c => { vm.decode(0x2084012fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275510u64) }
        0x10275510 => { vm.decode(0x1b081430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275514u64) }
        0x10275514 => { vm.decode(0x2084212fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275518u64) }
        0x10275518 => { vm.decode(0x39201430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027551cu64) }
        0x1027551c => { vm.decode(0x2284212fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275520u64) }
        0x10275520 => { vm.decode(0x1b189cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275524u64) }
        0x10275524 => { vm.decode(0x2080016fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275528u64) }
        0x10275528 => { vm.decode(0x1b109cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027552cu64) }
        0x1027552c => { vm.decode(0x2080216fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275530u64) }
        0x10275530 => { vm.decode(0x1b009cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275534u64) }
        0x10275534 => { vm.decode(0x2280016fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275538u64) }
        0x10275538 => { vm.decode(0x1b189c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027553cu64) }
        0x1027553c => { vm.decode(0x2280216fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275540u64) }
        0x10275540 => { vm.decode(0x1b089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275544u64) }
        0x10275544 => { vm.decode(0x2084016fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275548u64) }
        0x10275548 => { vm.decode(0x1b0894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027554cu64) }
        0x1027554c => { vm.decode(0x2084216fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275550u64) }
        0x10275550 => { vm.decode(0x1b001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275554u64) }
        0x10275554 => { vm.decode(0x2284016fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275558u64) }
        0x10275558 => { vm.decode(0x191894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027555cu64) }
        0x1027555c => { vm.decode(0x2284216fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275560u64) }
        0x10275560 => { vm.decode(0x191094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275564u64) }
        0x10275564 => { vm.decode(0x2880012fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275568u64) }
        0x10275568 => { vm.decode(0x190894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027556cu64) }
        0x1027556c => { vm.decode(0x2880212fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275570u64) }
        0x10275570 => { vm.decode(0x190094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275574u64) }
        0x10275574 => { vm.decode(0x2a80012fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275578u64) }
        0x10275578 => { vm.decode(0x19189430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027557cu64) }
        0x1027557c => { vm.decode(0x2a80212fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275580u64) }
        0x10275580 => { vm.decode(0x19109430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275584u64) }
        0x10275584 => { vm.decode(0x2884012fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275588u64) }
        0x10275588 => { vm.decode(0x19209430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027558cu64) }
        0x1027558c => { vm.decode(0x2884212fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275590u64) }
        0x10275590 => { vm.decode(0x1b101cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275594u64) }
        0x10275594 => { vm.decode(0x2a84012fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275598u64) }
        0x10275598 => { vm.decode(0x19201430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027559cu64) }
        0x1027559c => { vm.decode(0x2a84212fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102755a0u64) }
        0x102755a0 => { vm.decode(0x1b381c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102755a4u64) }
        0x102755a4 => { vm.decode(0x2880016fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102755a8u64) }
        0x102755a8 => { vm.decode(0x192894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102755acu64) }
        0x102755ac => { vm.decode(0x2880216fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102755b0u64) }
        0x102755b0 => { vm.decode(0x19201cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102755b4u64) }
        0x102755b4 => { vm.decode(0x2a80016fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102755b8u64) }
        0x102755b8 => { vm.decode(0x19381c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102755bcu64) }
        0x102755bc => { vm.decode(0x2a80216fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102755c0u64) }
        0x102755c0 => { vm.decode(0x19301c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102755c4u64) }
        0x102755c4 => { vm.decode(0x2884016fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102755c8u64) }
        0x102755c8 => { vm.decode(0x19281c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102755ccu64) }
        0x102755cc => { vm.decode(0x2884216fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102755d0u64) }
        0x102755d0 => { vm.decode(0x19201c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102755d4u64) }
        0x102755d4 => { vm.decode(0x2a84016fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102755d8u64) }
        0x102755d8 => { vm.decode(0x19181cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102755dcu64) }
        0x102755dc => { vm.decode(0x2a84216fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102755e0u64) }
        0x102755e0 => { vm.decode(0x19101cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102755e4u64) }
        0x102755e4 => { vm.decode(0x2080812fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102755e8u64) }
        0x102755e8 => { vm.decode(0x19081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102755ecu64) }
        0x102755ec => { vm.decode(0x2080a12fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102755f0u64) }
        0x102755f0 => { vm.decode(0x19001cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102755f4u64) }
        0x102755f4 => { vm.decode(0x2280812fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102755f8u64) }
        0x102755f8 => { vm.decode(0x19181c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102755fcu64) }
        0x102755fc => { vm.decode(0x2280a12fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275600u64) }
        0x10275600 => { vm.decode(0x19101c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275604u64) }
        0x10275604 => { vm.decode(0x2084812fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275608u64) }
        0x10275608 => { vm.decode(0x19189c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027560cu64) }
        0x1027560c => { vm.decode(0x2284812fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275610u64) }
        0x10275610 => { vm.decode(0x19109c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275614u64) }
        0x10275614 => { vm.decode(0x2284a12fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275618u64) }
        0x10275618 => { vm.decode(0x19089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027561cu64) }
        0x1027561c => { vm.decode(0x2080816fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275620u64) }
        0x10275620 => { vm.decode(0x19209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275624u64) }
        0x10275624 => { vm.decode(0x2080a16fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275628u64) }
        0x10275628 => { vm.decode(0x19309c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027562cu64) }
        0x1027562c => { vm.decode(0x2280816fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275630u64) }
        0x10275630 => { vm.decode(0x19289c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275634u64) }
        0x10275634 => { vm.decode(0x2280a16fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275638u64) }
        0x10275638 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027563cu64) }
        0x1027563c => { vm.decode(0x2084816fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275640u64) }
        0x10275640 => { vm.decode(0x1b089430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275644u64) }
        0x10275644 => { vm.decode(0x2084a16fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275648u64) }
        0x10275648 => { vm.decode(0x1b009430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027564cu64) }
        0x1027564c => { vm.decode(0x2284816fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275650u64) }
        0x10275650 => { vm.decode(0x1b3814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275654u64) }
        0x10275654 => { vm.decode(0x2284a16fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275658u64) }
        0x10275658 => { vm.decode(0x1b3014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027565cu64) }
        0x1027565c => { vm.decode(0x2880812fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275660u64) }
        0x10275660 => { vm.decode(0x1b2814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275664u64) }
        0x10275664 => { vm.decode(0x2880a12fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275668u64) }
        0x10275668 => { vm.decode(0x1b2014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027566cu64) }
        0x1027566c => { vm.decode(0x2a80812fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275670u64) }
        0x10275670 => { vm.decode(0x1b381430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275674u64) }
        0x10275674 => { vm.decode(0x2a80a12fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275678u64) }
        0x10275678 => { vm.decode(0x1b301430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027567cu64) }
        0x1027567c => { vm.decode(0x2884812fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275680u64) }
        0x10275680 => { vm.decode(0x1b281430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275684u64) }
        0x10275684 => { vm.decode(0x2884a12fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275688u64) }
        0x10275688 => { vm.decode(0x1b201430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027568cu64) }
        0x1027568c => { vm.decode(0x2a84812fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275690u64) }
        0x10275690 => { vm.decode(0x1b1814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275694u64) }
        0x10275694 => { vm.decode(0x2284012fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275698u64) }
        0x10275698 => { vm.decode(0x390814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027569cu64) }
        0x1027569c => { vm.decode(0x2084a12fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102756a0u64) }
        0x102756a0 => { vm.decode(0x19009c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102756a4u64) }
        0x102756a4 => { vm.decode(0x2280212fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102756a8u64) }
        0x102756a8 => { vm.decode(0x390014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102756acu64) }
        0x102756ac => { vm.decode(0x2280012fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102756b0u64) }
        0x102756b0 => { vm.decode(0x1b289cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102756b4u64) }
        0x102756b4 => { vm.decode(0x2080212fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102756b8u64) }
        0x102756b8 => { vm.decode(0x1b289c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102756bcu64) }
        0x102756bc => { vm.decode(0x22a0202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102756c0u64) }
        0x102756c0 => { vm.decode(0x19209c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102756c4u64) }
        0x102756c4 => { vm.decode(0x20a4002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102756c8u64) }
        0x102756c8 => { vm.decode(0x1b389cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102756ccu64) }
        0x102756cc => { vm.decode(0x20a4202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102756d0u64) }
        0x102756d0 => { vm.decode(0x1b309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102756d4u64) }
        0x102756d4 => { vm.decode(0x22a4002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102756d8u64) }
        0x102756d8 => { vm.decode(0x1b209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102756dcu64) }
        0x102756dc => { vm.decode(0x22a4202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102756e0u64) }
        0x102756e0 => { vm.decode(0x1b009c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102756e4u64) }
        0x102756e4 => { vm.decode(0x20a0006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102756e8u64) }
        0x102756e8 => { vm.decode(0x1b209c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102756ecu64) }
        0x102756ec => { vm.decode(0x20a0206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102756f0u64) }
        0x102756f0 => { vm.decode(0x1b089cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102756f4u64) }
        0x102756f4 => { vm.decode(0x22a0006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102756f8u64) }
        0x102756f8 => { vm.decode(0x1b001cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102756fcu64) }
        0x102756fc => { vm.decode(0x22a0206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275700u64) }
        0x10275700 => { vm.decode(0x19389430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275704u64) }
        0x10275704 => { vm.decode(0x20a4006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275708u64) }
        0x10275708 => { vm.decode(0x39181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1048i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027570cu64) }
        0x1027570c => { vm.decode(0x20a4206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275710u64) }
        0x10275710 => { vm.decode(0x1b181c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275714u64) }
        0x10275714 => { vm.decode(0x22a4006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275718u64) }
        0x10275718 => { vm.decode(0x39001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027571cu64) }
        0x1027571c => { vm.decode(0x22a4206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275720u64) }
        0x10275720 => { vm.decode(0x391814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275724u64) }
        0x10275724 => { vm.decode(0x28a0002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275728u64) }
        0x10275728 => { vm.decode(0x1b109c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027572cu64) }
        0x1027572c => { vm.decode(0x28a0202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275730u64) }
        0x10275730 => { vm.decode(0x39081430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1032i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275734u64) }
        0x10275734 => { vm.decode(0x2aa0002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275738u64) }
        0x10275738 => { vm.decode(0x1b389c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027573cu64) }
        0x1027573c => { vm.decode(0x2aa0202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275740u64) }
        0x10275740 => { vm.decode(0x1b309c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275744u64) }
        0x10275744 => { vm.decode(0x28a4002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275748u64) }
        0x10275748 => { vm.decode(0x1b301cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027574cu64) }
        0x1027574c => { vm.decode(0x28a4202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275750u64) }
        0x10275750 => { vm.decode(0x192014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275754u64) }
        0x10275754 => { vm.decode(0x2aa4002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275758u64) }
        0x10275758 => { vm.decode(0x19009430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027575cu64) }
        0x1027575c => { vm.decode(0x2aa4202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275760u64) }
        0x10275760 => { vm.decode(0x193814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275764u64) }
        0x10275764 => { vm.decode(0x28a0006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275768u64) }
        0x10275768 => { vm.decode(0x193014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027576cu64) }
        0x1027576c => { vm.decode(0x28a0206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275770u64) }
        0x10275770 => { vm.decode(0x192814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275774u64) }
        0x10275774 => { vm.decode(0x2aa0006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275778u64) }
        0x10275778 => { vm.decode(0x39101430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027577cu64) }
        0x1027577c => { vm.decode(0x2aa0206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275780u64) }
        0x10275780 => { vm.decode(0x1b081c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275784u64) }
        0x10275784 => { vm.decode(0x28a4006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275788u64) }
        0x10275788 => { vm.decode(0x190814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027578cu64) }
        0x1027578c => { vm.decode(0x28a4206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275790u64) }
        0x10275790 => { vm.decode(0x19309430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275794u64) }
        0x10275794 => { vm.decode(0x2aa4006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275798u64) }
        0x10275798 => { vm.decode(0x19289430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027579cu64) }
        0x1027579c => { vm.decode(0x2aa4206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102757a0u64) }
        0x102757a0 => { vm.decode(0x1b301c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102757a4u64) }
        0x102757a4 => { vm.decode(0x20a0802fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102757a8u64) }
        0x102757a8 => { vm.decode(0x1b201c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102757acu64) }
        0x102757ac => { vm.decode(0x20a0a02fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102757b0u64) }
        0x102757b0 => { vm.decode(0x1b181cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102757b4u64) }
        0x102757b4 => { vm.decode(0x22a0802fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102757b8u64) }
        0x102757b8 => { vm.decode(0x192094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102757bcu64) }
        0x102757bc => { vm.decode(0x22a0a02fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102757c0u64) }
        0x102757c0 => { vm.decode(0x19081c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102757c4u64) }
        0x102757c4 => { vm.decode(0x20a4802fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102757c8u64) }
        0x102757c8 => { vm.decode(0x19001c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102757ccu64) }
        0x102757cc => { vm.decode(0x20a4a02fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102757d0u64) }
        0x102757d0 => { vm.decode(0x193894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102757d4u64) }
        0x102757d4 => { vm.decode(0x22a4802fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102757d8u64) }
        0x102757d8 => { vm.decode(0x193094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102757dcu64) }
        0x102757dc => { vm.decode(0x22a4a02fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102757e0u64) }
        0x102757e0 => { vm.decode(0x19101430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102757e4u64) }
        0x102757e4 => { vm.decode(0x20a0806fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102757e8u64) }
        0x102757e8 => { vm.decode(0x19081430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102757ecu64) }
        0x102757ec => { vm.decode(0x20a0a06fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102757f0u64) }
        0x102757f0 => { vm.decode(0x19381cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102757f4u64) }
        0x102757f4 => { vm.decode(0x22a0806fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102757f8u64) }
        0x102757f8 => { vm.decode(0x19301cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102757fcu64) }
        0x102757fc => { vm.decode(0x22a0a06fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275800u64) }
        0x10275800 => { vm.decode(0x19281cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275804u64) }
        0x10275804 => { vm.decode(0x20a4806fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275808u64) }
        0x10275808 => { vm.decode(0x19189cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027580cu64) }
        0x1027580c => { vm.decode(0x20a4a06fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275810u64) }
        0x10275810 => { vm.decode(0x19109cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275814u64) }
        0x10275814 => { vm.decode(0x22a4806fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275818u64) }
        0x10275818 => { vm.decode(0x19089cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027581cu64) }
        0x1027581c => { vm.decode(0x22a4a06fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275820u64) }
        0x10275820 => { vm.decode(0x19001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275824u64) }
        0x10275824 => { vm.decode(0x28a0802fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275828u64) }
        0x10275828 => { vm.decode(0x19389cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027582cu64) }
        0x1027582c => { vm.decode(0x28a0a02fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275830u64) }
        0x10275830 => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275834u64) }
        0x10275834 => { vm.decode(0x2aa0802fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275838u64) }
        0x10275838 => { vm.decode(0x19289cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027583cu64) }
        0x1027583c => { vm.decode(0x2aa0a02fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275840u64) }
        0x10275840 => { vm.decode(0x19389c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275844u64) }
        0x10275844 => { vm.decode(0x28a4802fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275848u64) }
        0x10275848 => { vm.decode(0x1b1014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027584cu64) }
        0x1027584c => { vm.decode(0x28a4a02fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275850u64) }
        0x10275850 => { vm.decode(0x1b0814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275854u64) }
        0x10275854 => { vm.decode(0x2aa4802fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275858u64) }
        0x10275858 => { vm.decode(0x1b0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027585cu64) }
        0x1027585c => { vm.decode(0x2aa4a02fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275860u64) }
        0x10275860 => { vm.decode(0x1b181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275864u64) }
        0x10275864 => { vm.decode(0x28a0a06fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275868u64) }
        0x10275868 => { vm.decode(0x1b101430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027586cu64) }
        0x1027586c => { vm.decode(0x22a0002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275870u64) }
        0x10275870 => { vm.decode(0x1b101c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275874u64) }
        0x10275874 => { vm.decode(0x60aa202fu32); let d = 23i16; let s = 6i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275878u64) }
        0x10275878 => { vm.decode(0x20a0002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027587cu64) }
        0x1027587c => { vm.decode(0x6088002fu32); let d = 19i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275880u64) }
        0x10275880 => { vm.decode(0x228416fu32); let d = 2i16; let s = 19i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275884u64) }
        0x10275884 => { vm.decode(0xb1895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275888u64) }
        0x10275888 => { vm.decode(0x28616fu32); let d = 2i16; let s = 19i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027588cu64) }
        0x1027588c => { vm.decode(0xb1095b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275890u64) }
        0x10275890 => { vm.decode(0x2c412fu32); let d = 2i16; let s = 19i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275894u64) }
        0x10275894 => { vm.decode(0x91815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275898u64) }
        0x10275898 => { vm.decode(0x28612fu32); let d = 2i16; let s = 19i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027589cu64) }
        0x1027589c => { vm.decode(0xb281db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102758a0u64) }
        0x102758a0 => { vm.decode(0x28412fu32); let d = 2i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102758a4u64) }
        0x102758a4 => { vm.decode(0xb3895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 760i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102758a8u64) }
        0x102758a8 => { vm.decode(0x88402fu32); let d = 2i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102758acu64) }
        0x102758ac => { vm.decode(0xd0015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102758b0u64) }
        0x102758b0 => { vm.decode(0x706f0019u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 1176i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102758b4u64) }
        0x102758b4 => { vm.decode(0x20101019u32); let d = 2i16; let s = 16i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102758b8u64) }
        0x102758b8 => { vm.decode(0xb209530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102758bcu64) }
        0x102758bc => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102758c0u64) }
        0x102758c0 => { vm.decode(0x1d4fb28eu32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = -14614528i64;
        vm.st(d, imm as u64);
        Ok(0x102758c4u64) }
        0x102758c4 => { vm.decode(0xcd671ad8u32); let d = 22i16; let s = 30i16; let s2 = -1i16; let imm = 26472i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102758c8u64) }
        0x102758c8 => { vm.decode(0x616604f5u32); let d = 21i16; let s = 22i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102758ccu64) }
        0x102758cc => { vm.decode(0x70198219u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102758d0u64) }
        0x102758d0 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102758d4u64) }
        0x102758d4 => { vm.decode(0xb91054b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102758d8u64) }
        0x102758d8 => { vm.decode(0xe00d875u32); let d = 25i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102758dcu64) }
        0x102758dc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102758e0u64 + 4);
        vm.wr(37, 0x102758e0u64 + 4);
        Ok(0x102758e0u64) }
        0x102758e0 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102758e4u64) }
        0x102758e4 => { vm.decode(0x2080402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102758e8u64) }
        0x102758e8 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102758ecu64) }
        0x102758ec => { vm.decode(0x20548019u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 56i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102758f0u64) }
        0x102758f0 => { vm.decode(0x1b381cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102758f4u64) }
        0x102758f4 => { vm.decode(0x1d0814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102758f8u64) }
        0x102758f8 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102758fcu64) }
        0x102758fc => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275900u64) }
        0x10275900 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275904u64 + 4);
        vm.wr(37, 0x10275904u64 + 4);
        Ok(0x10275904u64) }
        0x10275904 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275908u64) }
        0x10275908 => { vm.decode(0x8b081cb0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027590cu64) }
        0x1027590c => { vm.decode(0x20408119u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 80i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275910u64) }
        0x10275910 => { vm.decode(0x88402fu32); let d = 2i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275914u64) }
        0x10275914 => { vm.decode(0x1d0814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275918u64) }
        0x10275918 => { vm.decode(0xd0015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027591cu64) }
        0x1027591c => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275920u64) }
        0x10275920 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275924u64) }
        0x10275924 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275928u64 + 4);
        vm.wr(37, 0x10275928u64 + 4);
        Ok(0x10275928u64) }
        0x10275928 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027592cu64) }
        0x1027592c => { vm.decode(0x2880e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275930u64) }
        0x10275930 => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10275934u64) }
        0x10275934 => { vm.decode(0xc90b3a58u32); let d = 2i16; let s = 30i16; let s2 = -1i16; let imm = 27104i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10275938u64) }
        0x10275938 => { vm.decode(0x6082c3afu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027593cu64) }
        0x1027593c => { vm.decode(0x600524f5u32); let d = 16i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10275940u64) }
        0x10275940 => { vm.decode(0x8402fu32); let d = 2i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275944u64) }
        0x10275944 => { vm.decode(0x1d101430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275948u64) }
        0x10275948 => { vm.decode(0xd001530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027594cu64) }
        0x1027594c => { vm.decode(0x6ba724f5u32); let d = 30i16; let s = 22i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10275950u64) }
        0x10275950 => { vm.decode(0x12400019u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275954u64) }
        0x10275954 => { vm.decode(0x8d185430u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 4120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275958u64) }
        0x10275958 => { vm.decode(0x722f9019u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 1160i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027595cu64) }
        0x1027595c => { vm.decode(0x9d085530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 4104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275960u64) }
        0x10275960 => { vm.decode(0x70098219u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 4096i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275964u64) }
        0x10275964 => { vm.decode(0xb05875u32); let d = 4i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275968u64) }
        0x10275968 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027596cu64) }
        0x1027596c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275970u64 + 4);
        vm.wr(37, 0x10275970u64 + 4);
        Ok(0x10275970u64) }
        0x10275970 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275974u64) }
        0x10275974 => { vm.decode(0xb900d5b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 1184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275978u64) }
        0x10275978 => { vm.decode(0x48909875u32); let d = 23i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027597cu64) }
        0x1027597c => { vm.decode(0xa918d430u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275980u64) }
        0x10275980 => { vm.decode(0x108099u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 33i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275984u64) }
        0x10275984 => { vm.decode(0x8402fu32); let d = 2i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275988u64) }
        0x10275988 => { vm.decode(0x1d181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027598cu64) }
        0x1027598c => { vm.decode(0x2884e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275990u64) }
        0x10275990 => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10275994u64) }
        0x10275994 => { vm.decode(0x1d101430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275998u64) }
        0x10275998 => { vm.decode(0x6039a119u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1124i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027599cu64) }
        0x1027599c => { vm.decode(0x1d081430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102759a0u64) }
        0x102759a0 => { vm.decode(0xd001530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102759a4u64) }
        0x102759a4 => { vm.decode(0xb05875u32); let d = 4i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102759a8u64) }
        0x102759a8 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102759acu64) }
        0x102759ac => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102759b0u64 + 4);
        vm.wr(37, 0x102759b0u64 + 4);
        Ok(0x102759b0u64) }
        0x102759b0 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102759b4u64) }
        0x102759b4 => { vm.decode(0xa8663efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 760i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102759b8u64) }
        0x102759b8 => { vm.decode(0x220bb5u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102759bcu64) }
        0x102759bc => { vm.decode(0x405c4349u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102759c0u64) }
        0x102759c0 => { vm.decode(0x23d7422u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 668i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102759c4u64) }
        0x102759c4 => { vm.decode(0x20410bb5u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102759c8u64) }
        0x102759c8 => { vm.decode(0x40930d35u32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102759ccu64) }
        0x102759cc => { vm.decode(0x33d7022u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102759d0u64) }
        0x102759d0 => { vm.decode(0x20c34bb5u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102759d4u64) }
        0x102759d4 => { vm.decode(0x44401875u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102759d8u64) }
        0x102759d8 => { vm.decode(0x42d84349u32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102759dcu64) }
        0x102759dc => { vm.decode(0x32d7422u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 660i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102759e0u64) }
        0x102759e0 => { vm.decode(0xc34bb5u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102759e4u64) }
        0x102759e4 => { vm.decode(0x40481875u32); let d = 2i16; let s = 3i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102759e8u64) }
        0x102759e8 => { vm.decode(0xa236u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102759ecu64) }
        0x102759ec => { vm.decode(0x23355822u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102759f0u64) }
        0x102759f0 => { vm.decode(0x63055822u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102759f4u64) }
        0x102759f4 => { vm.decode(0x1b0db28eu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = -14614528i64;
        vm.st(d, imm as u64);
        Ok(0x102759f8u64) }
        0x102759f8 => { vm.decode(0xa8082b18u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 26752i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102759fcu64) }
        0x102759fc => { vm.decode(0xe84124f5u32); let d = 3i16; let s = 3i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10275a00u64) }
        0x10275a00 => { vm.decode(0x4c104875u32); let d = 18i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275a04u64) }
        0x10275a04 => { vm.decode(0xd2ca812du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10275a08u64) }
        0x10275a08 => { vm.decode(0x32d3022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10275a0cu64) }
        0x10275a0c => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275a10u64) }
        0x10275a10 => { vm.decode(0x2910d4b0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275a14u64) }
        0x10275a14 => { vm.decode(0x23053422u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 52i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10275a18u64) }
        0x10275a18 => { vm.decode(0x23351c22u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1196i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10275a1cu64) }
        0x10275a1c => { vm.decode(0x2020012fu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275a20u64) }
        0x10275a20 => { vm.decode(0x3b319cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275a24u64) }
        0x10275a24 => { vm.decode(0x2280c3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275a28u64) }
        0x10275a28 => { vm.decode(0x612204f5u32); let d = 4i16; let s = 22i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10275a2cu64) }
        0x10275a2c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275a30u64) }
        0x10275a30 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275a34u64 + 4);
        vm.wr(37, 0x10275a34u64 + 4);
        Ok(0x10275a34u64) }
        0x10275a34 => { vm.decode(0x667b8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 4080i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275a38u64) }
        0x10275a38 => { vm.decode(0x2ac4f3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275a3cu64) }
        0x10275a3c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275a40u64) }
        0x10275a40 => { vm.decode(0x1b3094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275a44u64) }
        0x10275a44 => { vm.decode(0x21c80a7eu32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 92i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10275a48u64) }
        0x10275a48 => { vm.decode(0x23251462u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 420i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10275a4cu64) }
        0x10275a4c => { vm.decode(0x15c08911u32); let d = 20i16; let s = 19i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10275a50u64) }
        0x10275a50 => { vm.decode(0x2224612fu32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275a54u64) }
        0x10275a54 => { vm.decode(0x1b289430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275a58u64) }
        0x10275a58 => { vm.decode(0x2224412fu32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275a5cu64) }
        0x10275a5c => { vm.decode(0x1b309430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275a60u64) }
        0x10275a60 => { vm.decode(0x2024612fu32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275a64u64) }
        0x10275a64 => { vm.decode(0x1b2894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275a68u64) }
        0x10275a68 => { vm.decode(0x2024412fu32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275a6cu64) }
        0x10275a6c => { vm.decode(0x19089430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275a70u64) }
        0x10275a70 => { vm.decode(0x6222612fu32); let d = 21i16; let s = 19i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275a74u64) }
        0x10275a74 => { vm.decode(0x2220412fu32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275a78u64) }
        0x10275a78 => { vm.decode(0x19281430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275a7cu64) }
        0x10275a7c => { vm.decode(0x2020612fu32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275a80u64) }
        0x10275a80 => { vm.decode(0x1b289cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275a84u64) }
        0x10275a84 => { vm.decode(0xd1cc0d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1196i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10275a88u64) }
        0x10275a88 => { vm.decode(0x6b253462u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2548i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10275a8cu64) }
        0x10275a8c => { vm.decode(0x2000402fu32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275a90u64) }
        0x10275a90 => { vm.decode(0x561119u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 248i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275a94u64) }
        0x10275a94 => { vm.decode(0xd181530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275a98u64) }
        0x10275a98 => { vm.decode(0x8c532fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275a9cu64) }
        0x10275a9c => { vm.decode(0x4b2004f5u32); let d = 2i16; let s = 22i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10275aa0u64) }
        0x10275aa0 => { vm.decode(0xd101530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275aa4u64) }
        0x10275aa4 => { vm.decode(0x705f0519u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2296i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275aa8u64) }
        0x10275aa8 => { vm.decode(0x8d081430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 4104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275aacu64) }
        0x10275aac => { vm.decode(0x1d001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275ab0u64) }
        0x10275ab0 => { vm.decode(0x8b28dd70u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275ab4u64) }
        0x10275ab4 => { vm.decode(0xb05875u32); let d = 4i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275ab8u64) }
        0x10275ab8 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275abcu64) }
        0x10275abc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275ac0u64 + 4);
        vm.wr(37, 0x10275ac0u64 + 4);
        Ok(0x10275ac0u64) }
        0x10275ac0 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275ac4u64) }
        0x10275ac4 => { vm.decode(0x2080532fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275ac8u64) }
        0x10275ac8 => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10275accu64) }
        0x10275acc => { vm.decode(0x39001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275ad0u64) }
        0x10275ad0 => { vm.decode(0x2280e3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275ad4u64) }
        0x10275ad4 => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10275ad8u64) }
        0x10275ad8 => { vm.decode(0x390014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275adcu64) }
        0x10275adc => { vm.decode(0x2280732fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1048i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275ae0u64) }
        0x10275ae0 => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10275ae4u64) }
        0x10275ae4 => { vm.decode(0x39181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1048i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275ae8u64) }
        0x10275ae8 => { vm.decode(0x2084c3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275aecu64) }
        0x10275aec => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10275af0u64) }
        0x10275af0 => { vm.decode(0x1b101c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275af4u64) }
        0x10275af4 => { vm.decode(0x2884e3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275af8u64) }
        0x10275af8 => { vm.decode(0x696004f5u32); let d = 3i16; let s = 22i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10275afcu64) }
        0x10275afc => { vm.decode(0x2880c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275b00u64) }
        0x10275b00 => { vm.decode(0x696204f5u32); let d = 7i16; let s = 22i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10275b04u64) }
        0x10275b04 => { vm.decode(0x2880c3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275b08u64) }
        0x10275b08 => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10275b0cu64) }
        0x10275b0c => { vm.decode(0x1b001cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275b10u64) }
        0x10275b10 => { vm.decode(0x2884c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275b14u64) }
        0x10275b14 => { vm.decode(0x692204f5u32); let d = 6i16; let s = 22i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10275b18u64) }
        0x10275b18 => { vm.decode(0x2a84c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275b1cu64) }
        0x10275b1c => { vm.decode(0x61a004f5u32); let d = 8i16; let s = 22i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10275b20u64) }
        0x10275b20 => { vm.decode(0x2a84e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275b24u64) }
        0x10275b24 => { vm.decode(0x612204f5u32); let d = 4i16; let s = 22i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10275b28u64) }
        0x10275b28 => { vm.decode(0x2080536fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275b2cu64) }
        0x10275b2c => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10275b30u64) }
        0x10275b30 => { vm.decode(0x1b009c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275b34u64) }
        0x10275b34 => { vm.decode(0x4b2504f5u32); let d = 18i16; let s = 22i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10275b38u64) }
        0x10275b38 => { vm.decode(0x8002036u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10275b3cu64) }
        0x10275b3c => { vm.decode(0x73291059u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 1280i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275b40u64) }
        0x10275b40 => { vm.decode(0x647d1059u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1848i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275b44u64) }
        0x10275b44 => { vm.decode(0x625ba559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2548i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275b48u64) }
        0x10275b48 => { vm.decode(0x743d8019u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275b4cu64) }
        0x10275b4c => { vm.decode(0xb9285530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275b50u64) }
        0x10275b50 => { vm.decode(0x9b209cb0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275b54u64) }
        0x10275b54 => { vm.decode(0xb389db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275b58u64) }
        0x10275b58 => { vm.decode(0x1b089db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275b5cu64) }
        0x10275b5c => { vm.decode(0x1b185c30u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275b60u64) }
        0x10275b60 => { vm.decode(0x1b20dd30u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275b64u64) }
        0x10275b64 => { vm.decode(0xb30ddb0u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 1008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275b68u64) }
        0x10275b68 => { vm.decode(0xb001c70u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275b6cu64) }
        0x10275b6c => { vm.decode(0x626d1122u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10275b70u64) }
        0x10275b70 => { vm.decode(0xb38d430u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275b74u64) }
        0x10275b74 => { vm.decode(0x99185430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275b78u64) }
        0x10275b78 => { vm.decode(0x902a6031u32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 27i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10275b7cu64).wrapping_add((imm * 4) as u64) } else { 0x10275b7cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10275b7cu64) }
        0x10275b7c => { vm.decode(0x9b281d30u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275b80u64) }
        0x10275b80 => { vm.decode(0x2880c36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275b84u64) }
        0x10275be8 => { vm.decode(0x2a80636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275becu64) }
        0x10275bec => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10275bf0u64) }
        0x10275bf0 => { vm.decode(0x1b081430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275bf4u64) }
        0x10275bf4 => { vm.decode(0x319900eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 372310016i64;
        vm.st(d, imm as u64);
        Ok(0x10275bf8u64) }
        0x10275bf8 => { vm.decode(0xc016c699u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 14507i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275bfcu64) }
        0x10275bfc => { vm.decode(0x45108091u32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10275c00u64) }
        0x10275c00 => { vm.decode(0x2288412fu32); let d = 3i16; let s = 21i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275c04u64) }
        0x10275c04 => { vm.decode(0x422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10275c08u64) }
        0x10275c08 => { vm.decode(0x688844eu32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 7143424i64;
        vm.st(d, imm as u64);
        Ok(0x10275c0cu64) }
        0x10275c0c => { vm.decode(0xc22188d9u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 17667i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275c10u64) }
        0x10275c10 => { vm.decode(0xb842ad75u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 23i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10275c14u64) }
        0x10275c14 => { vm.decode(0x880d875u32); let d = 21i16; let s = 0i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275c18u64) }
        0x10275c18 => { vm.decode(0x422711d9u32); let d = 6i16; let s = 5i16; let s2 = -1i16; let imm = 1481i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275c1cu64) }
        0x10275c1c => { vm.decode(0x1691a54eu32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1484652544i64;
        vm.st(d, imm as u64);
        Ok(0x10275c20u64) }
        0x10275c20 => { vm.decode(0x92429d75u32); let d = 7i16; let s = 6i16; let s2 = -1i16; let imm = 18i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10275c24u64) }
        0x10275c24 => { vm.decode(0xc814e5d9u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -22161i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275c28u64) }
        0x10275c28 => { vm.decode(0xce31a0d9u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = -30937i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275c2cu64) }
        0x10275c2c => { vm.decode(0x422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10275c30u64) }
        0x10275c30 => { vm.decode(0xc10410eu32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 9568256i64;
        vm.st(d, imm as u64);
        Ok(0x10275c34u64) }
        0x10275c34 => { vm.decode(0x29175d9u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 10597i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275c38u64) }
        0x10275c38 => { vm.decode(0x12029d75u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10275c3cu64) }
        0x10275c3c => { vm.decode(0x86937b99u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 29415i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275c40u64) }
        0x10275c40 => { vm.decode(0xba029d75u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 23i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10275c44u64) }
        0x10275c44 => { vm.decode(0xc6c3b999u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 17111i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275c48u64) }
        0x10275c48 => { vm.decode(0x4056e219u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 12476i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275c4cu64) }
        0x10275c4c => { vm.decode(0x1042ad75u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10275c50u64) }
        0x10275c50 => { vm.decode(0xc203ebd9u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 29127i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275c54u64) }
        0x10275c54 => { vm.decode(0x3042ad75u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 17i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10275c58u64) }
        0x10275c58 => { vm.decode(0x4e25ed99u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = -4531i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275c5cu64) }
        0x10275c5c => { vm.decode(0x29001c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275c60u64) }
        0x10275c60 => { vm.decode(0x39005cb0u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275c64u64) }
        0x10275c64 => { vm.decode(0x39181c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275c68u64) }
        0x10275c68 => { vm.decode(0x39105d30u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 1296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275c6cu64) }
        0x10275c6c => { vm.decode(0x84b5a399u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 5743i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275c70u64) }
        0x10275c70 => { vm.decode(0x39081c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275c74u64) }
        0x10275c74 => { vm.decode(0x667b8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 4016i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275c78u64) }
        0x10275c78 => { vm.decode(0x2224c06fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275c7cu64) }
        0x10275c7c => { vm.decode(0xab19ddf0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 4024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275c80u64) }
        0x10275c80 => { vm.decode(0x3b219d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 4032i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275c84u64) }
        0x10275c84 => { vm.decode(0x4a3d5862u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10275c88u64) }
        0x10275c88 => { vm.decode(0x3b119cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275c8cu64) }
        0x10275c8c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275c90u64) }
        0x10275c90 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275c94u64 + 4);
        vm.wr(37, 0x10275c94u64 + 4);
        Ok(0x10275c94u64) }
        0x10275c94 => { vm.decode(0x8a00875u32); let d = 20i16; let s = 0i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275c98u64) }
        0x10275c98 => { vm.decode(0x666b8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3984i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275c9cu64) }
        0x10275c9c => { vm.decode(0x2224e06fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275ca0u64) }
        0x10275ca0 => { vm.decode(0xbb019cb0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 4000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275ca4u64) }
        0x10275ca4 => { vm.decode(0xab19dd70u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275ca8u64) }
        0x10275ca8 => { vm.decode(0x3b119c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275cacu64) }
        0x10275cac => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275cb0u64) }
        0x10275cb0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275cb4u64 + 4);
        vm.wr(37, 0x10275cb4u64 + 4);
        Ok(0x10275cb4u64) }
        0x10275cb4 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275cb8u64) }
        0x10275cb8 => { vm.decode(0x66798559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3952i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275cbcu64) }
        0x10275cbc => { vm.decode(0x4002036u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10275cc0u64) }
        0x10275cc0 => { vm.decode(0xa28606fu32); let d = 2i16; let s = 18i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275cc4u64) }
        0x10275cc4 => { vm.decode(0xb3d1862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10275cc8u64) }
        0x10275cc8 => { vm.decode(0xbb019c30u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 3968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275cccu64) }
        0x10275ccc => { vm.decode(0x4a86e3efu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275cd0u64) }
        0x10275cd0 => { vm.decode(0xab395cb0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 3960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275cd4u64) }
        0x10275cd4 => { vm.decode(0x2b311db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275cd8u64) }
        0x10275cd8 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275cdcu64) }
        0x10275cdc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275ce0u64 + 4);
        vm.wr(37, 0x10275ce0u64 + 4);
        Ok(0x10275ce0u64) }
        0x10275ce0 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275ce4u64) }
        0x10275ce4 => { vm.decode(0x66698559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3920i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275ce8u64) }
        0x10275ce8 => { vm.decode(0x1920dc30u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275cecu64) }
        0x10275cec => { vm.decode(0x2224606fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275cf0u64) }
        0x10275cf0 => { vm.decode(0xab215cb0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 3936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275cf4u64) }
        0x10275cf4 => { vm.decode(0x88843efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275cf8u64) }
        0x10275cf8 => { vm.decode(0x2b391d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275cfcu64) }
        0x10275cfc => { vm.decode(0x3b311c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275d00u64) }
        0x10275d00 => { vm.decode(0x86c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275d04u64) }
        0x10275d04 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275d08u64) }
        0x10275d08 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275d0cu64 + 4);
        vm.wr(37, 0x10275d0cu64 + 4);
        Ok(0x10275d0cu64) }
        0x10275d0c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10275d10u64) } }
        0x10275d10 => { vm.decode(0x62198219u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275d14u64) }
        0x10275d14 => { vm.decode(0x1b20d430u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275d18u64) }
        0x10275d18 => { vm.decode(0x2820402fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275d1cu64) }
        0x10275d1c => { vm.decode(0x8d0854b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275d20u64) }
        0x10275d20 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275d24u64) }
        0x10275d24 => { vm.decode(0x6284532fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275d28u64) }
        0x10275d28 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275d2cu64) }
        0x10275d2c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275d30u64 + 4);
        vm.wr(37, 0x10275d30u64 + 4);
        Ok(0x10275d30u64) }
        0x10275d30 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275d34u64) }
        0x10275d34 => { vm.decode(0x4082636fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275d38u64) }
        0x10275d38 => { vm.decode(0x9108091u32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10275d3cu64) }
        0x10275d3c => { vm.decode(0x2c406fu32); let d = 2i16; let s = 18i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275d40u64) }
        0x10275d40 => { vm.decode(0x52184b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 3912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10275d44u64) }
        0x10275d44 => { vm.decode(0xab215d70u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275d48u64) }
        0x10275d48 => { vm.decode(0x2b191db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275d4cu64) }
        0x10275d4c => { vm.decode(0x459840d1u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1220i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10275d50u64) }
        0x10275d50 => { vm.decode(0x40210fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10275d54u64) }
        0x10275d54 => { vm.decode(0xb036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10275d58u64) }
        0x10275d58 => { vm.decode(0x4b1d1c62u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3916i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10275d5cu64) }
        0x10275d5c => { vm.decode(0x882c3efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275d60u64) }
        0x10275d60 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275d64u64) }
        0x10275d64 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275d68u64 + 4);
        vm.wr(37, 0x10275d68u64 + 4);
        Ok(0x10275d68u64) }
        0x10275d68 => { vm.decode(0x667d8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3896i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275d6cu64) }
        0x10275d6c => { vm.decode(0x76698459u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 3856i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275d70u64) }
        0x10275d70 => { vm.decode(0x2020406fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275d74u64) }
        0x10275d74 => { vm.decode(0x459840d1u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1220i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10275d78u64) }
        0x10275d78 => { vm.decode(0x2b091db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275d7cu64) }
        0x10275d7c => { vm.decode(0x888736fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275d80u64) }
        0x10275d80 => { vm.decode(0x2b011db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275d84u64) }
        0x10275d84 => { vm.decode(0x288d32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275d88u64) }
        0x10275d88 => { vm.decode(0x2b191d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275d8cu64) }
        0x10275d8c => { vm.decode(0x3b111c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275d90u64) }
        0x10275d90 => { vm.decode(0x688ee3afu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275d94u64) }
        0x10275d94 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275d98u64) }
        0x10275d98 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275d9cu64) }
        0x10275d9c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275da0u64 + 4);
        vm.wr(37, 0x10275da0u64 + 4);
        Ok(0x10275da0u64) }
        0x10275da0 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275da4u64) }
        0x10275da4 => { vm.decode(0x2020406fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275da8u64) }
        0x10275da8 => { vm.decode(0x288402fu32); let d = 2i16; let s = 20i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275dacu64) }
        0x10275dac => { vm.decode(0x49108091u32); let d = 3i16; let s = 20i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10275db0u64) }
        0x10275db0 => { vm.decode(0x3b091db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 3880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275db4u64) }
        0x10275db4 => { vm.decode(0x2b011db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275db8u64) }
        0x10275db8 => { vm.decode(0x3b111c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275dbcu64) }
        0x10275dbc => { vm.decode(0xd9840d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1220i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10275dc0u64) }
        0x10275dc0 => { vm.decode(0x288d32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275dc4u64) }
        0x10275dc4 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10275dc8u64) }
        0x10275dc8 => { vm.decode(0x3b191c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275dccu64) }
        0x10275dcc => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275dd0u64) }
        0x10275dd0 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275dd4u64) }
        0x10275dd4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275dd8u64 + 4);
        vm.wr(37, 0x10275dd8u64 + 4);
        Ok(0x10275dd8u64) }
        0x10275dd8 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275ddcu64) }
        0x10275ddc => { vm.decode(0xa019u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275de0u64) }
        0x10275de0 => { vm.decode(0x41108091u32); let d = 2i16; let s = 20i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10275de4u64) }
        0x10275de4 => { vm.decode(0x2028406fu32); let d = 3i16; let s = 18i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275de8u64) }
        0x10275de8 => { vm.decode(0x3b091cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275decu64) }
        0x10275dec => { vm.decode(0x2280e3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275df0u64) }
        0x10275df0 => { vm.decode(0x3b011cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275df4u64) }
        0x10275df4 => { vm.decode(0x3b111d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 3856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275df8u64) }
        0x10275df8 => { vm.decode(0xd9840d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1220i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10275dfcu64) }
        0x10275dfc => { vm.decode(0x2288d32fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275e00u64) }
        0x10275e00 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10275e04u64) }
        0x10275e04 => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10275e08u64) }
        0x10275e08 => { vm.decode(0x3b191c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275e0cu64) }
        0x10275e0c => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275e10u64) }
        0x10275e10 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275e14u64) }
        0x10275e14 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275e18u64 + 4);
        vm.wr(37, 0x10275e18u64 + 4);
        Ok(0x10275e18u64) }
        0x10275e18 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275e1cu64) }
        0x10275e1c => { vm.decode(0x667f8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3832i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275e20u64) }
        0x10275e20 => { vm.decode(0x2824c02fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275e24u64) }
        0x10275e24 => { vm.decode(0xab015d70u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275e28u64) }
        0x10275e28 => { vm.decode(0x3b3994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275e2cu64) }
        0x10275e2c => { vm.decode(0x4282c3afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275e30u64) }
        0x10275e30 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275e34u64) }
        _ => Err(Error::UnknownPc(pc)),
    }
}
