#![cfg_attr(rustfmt, rustfmt::skip)]
// 由 scripts/medusa-vm/gen_native.py 生成，勿手改。
use super::super::Vm;
use super::super::Error;

#[allow(clippy::all, unused_parens, unused_variables, unused_mut)]
pub fn exec(vm: &mut Vm, pc: u64) -> Result<u64, Error> {
    match pc {
        0x102818f0 => { vm.decode(0x8b3855f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102818f4u64) }
        0x102818f4 => { vm.decode(0xb3015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102818f8u64) }
        0x102818f8 => { vm.decode(0x688263afu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102818fcu64) }
        0x102818fc => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281900u64) }
        0x10281900 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281904u64) }
        0x10281904 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10281908u64 + 4);
        vm.wr(37, 0x10281908u64 + 4);
        Ok(0x10281908u64) }
        0x10281908 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028190cu64) }
        0x1028190c => { vm.decode(0x740d9119u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10281910u64) }
        0x10281910 => { vm.decode(0x2280606fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281914u64) }
        0x10281914 => { vm.decode(0x88863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281918u64) }
        0x10281918 => { vm.decode(0xb2015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028191cu64) }
        0x1028191c => { vm.decode(0x9b385430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281920u64) }
        0x10281920 => { vm.decode(0x888632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281924u64) }
        0x10281924 => { vm.decode(0xb301530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281928u64) }
        0x10281928 => { vm.decode(0x1b281430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028192cu64) }
        0x1028192c => { vm.decode(0x6086436fu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281930u64) }
        0x10281930 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281934u64) }
        0x10281934 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281938u64) }
        0x10281938 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028193cu64 + 4);
        vm.wr(37, 0x1028193cu64 + 4);
        Ok(0x1028193cu64) }
        0x1028193c => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281940u64) }
        0x10281940 => { vm.decode(0x2080406fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281944u64) }
        0x10281944 => { vm.decode(0x26d5122u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10281948u64) }
        0x10281948 => { vm.decode(0x9b3855b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028194cu64) }
        0x1028194c => { vm.decode(0x1b3014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281950u64) }
        0x10281950 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281954u64) }
        0x10281954 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281958u64) }
        0x10281958 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028195cu64 + 4);
        vm.wr(37, 0x1028195cu64 + 4);
        Ok(0x1028195cu64) }
        0x1028195c => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281960u64) }
        0x10281960 => { vm.decode(0x2280606fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281964u64) }
        0x10281964 => { vm.decode(0x88e32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281968u64) }
        0x10281968 => { vm.decode(0xb2015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028196cu64) }
        0x1028196c => { vm.decode(0x88863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281970u64) }
        0x10281970 => { vm.decode(0xb381530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281974u64) }
        0x10281974 => { vm.decode(0xa88632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281978u64) }
        0x10281978 => { vm.decode(0xb301530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028197cu64) }
        0x1028197c => { vm.decode(0x1b281430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281980u64) }
        0x10281980 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281984u64) }
        0x10281984 => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281988u64) }
        0x10281988 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028198cu64 + 4);
        vm.wr(37, 0x1028198cu64 + 4);
        Ok(0x1028198cu64) }
        0x1028198c => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281990u64) }
        0x10281990 => { vm.decode(0x741d8159u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10281994u64) }
        0x10281994 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281998u64) }
        0x10281998 => { vm.decode(0x888636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028199cu64) }
        0x1028199c => { vm.decode(0xb301db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102819a0u64) }
        0x102819a0 => { vm.decode(0x1b281cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102819a4u64) }
        0x102819a4 => { vm.decode(0x4a8c432fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102819a8u64) }
        0x102819a8 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102819acu64) }
        0x102819ac => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102819b0u64) }
        0x102819b0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102819b4u64 + 4);
        vm.wr(37, 0x102819b4u64 + 4);
        Ok(0x102819b4u64) }
        0x102819b4 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102819b8u64) }
        0x102819b8 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102819bcu64) }
        0x102819bc => { vm.decode(0x888432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102819c0u64) }
        0x102819c0 => { vm.decode(0xb301db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102819c4u64) }
        0x102819c4 => { vm.decode(0x1b281cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102819c8u64) }
        0x102819c8 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102819ccu64) }
        0x102819cc => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102819d0u64) }
        0x102819d0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102819d4u64 + 4);
        vm.wr(37, 0x102819d4u64 + 4);
        Ok(0x102819d4u64) }
        0x102819d4 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102819d8u64) }
        0x102819d8 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102819dcu64) }
        0x102819dc => { vm.decode(0xa88636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102819e0u64) }
        0x102819e0 => { vm.decode(0xb301db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102819e4u64) }
        0x102819e4 => { vm.decode(0x1b281cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102819e8u64) }
        0x102819e8 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102819ecu64) }
        0x102819ec => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102819f0u64) }
        0x102819f0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102819f4u64 + 4);
        vm.wr(37, 0x102819f4u64 + 4);
        Ok(0x102819f4u64) }
        0x102819f4 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102819f8u64) }
        0x102819f8 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102819fcu64) }
        0x102819fc => { vm.decode(0xa88436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281a00u64) }
        0x10281a00 => { vm.decode(0xb301db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281a04u64) }
        0x10281a04 => { vm.decode(0x1b281cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281a08u64) }
        0x10281a08 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281a0cu64) }
        0x10281a0c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281a10u64) }
        0x10281a10 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10281a14u64 + 4);
        vm.wr(37, 0x10281a14u64 + 4);
        Ok(0x10281a14u64) }
        0x10281a14 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281a18u64) }
        0x10281a18 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281a1cu64) }
        0x10281a1c => { vm.decode(0x88c632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281a20u64) }
        0x10281a20 => { vm.decode(0xb301db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281a24u64) }
        0x10281a24 => { vm.decode(0x1b281cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281a28u64) }
        0x10281a28 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281a2cu64) }
        0x10281a2c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281a30u64) }
        0x10281a30 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10281a34u64 + 4);
        vm.wr(37, 0x10281a34u64 + 4);
        Ok(0x10281a34u64) }
        0x10281a34 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281a38u64) }
        0x10281a38 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281a3cu64) }
        0x10281a3c => { vm.decode(0x88c432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281a40u64) }
        0x10281a40 => { vm.decode(0xb301db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281a44u64) }
        0x10281a44 => { vm.decode(0x1b281cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10281a48u64) }
        0x10281a48 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281a4cu64) }
        0x10281a4c => { vm.decode(0xa108875u32); let d = 25i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281a50u64) }
        0x10281a50 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10281a54u64 + 4);
        vm.wr(37, 0x10281a54u64 + 4);
        Ok(0x10281a54u64) }
        0x10281a54 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10281a58u64) }
        0x10281a58 => { vm.decode(0x4080532fu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 1024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281a5cu64) }
        0x10281a5c => { vm.decode(0x6080732fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 1032i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281a60u64) }
        0x10281a60 => { vm.decode(0x4288532fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281a64u64) }
        0x10281a64 => { vm.decode(0x6288732fu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 1048i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281a68u64) }
        0x10281a68 => { vm.decode(0x4086532fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281a6cu64) }
        0x10281a6c => { vm.decode(0x6086732fu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281a70u64) }
        0x10281a70 => { vm.decode(0x428e532fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281a74u64) }
        0x10281a74 => { vm.decode(0x628e732fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281a78u64) }
        0x10281a78 => { vm.decode(0x408a5b6fu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281a7cu64) }
        0x10281a7c => { vm.decode(0x608a7b6fu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10281a80u64) }
        0x10281a80 => { vm.decode(0x54104cf5u32); let d = -1i16; let s = -1i16; let s2 = -1i16; let imm = 0i64;
        vm.jump_flag = 2;
        vm.wr(36, vm.slots[31]);
        vm.wr(35, 2);
        Ok(0x10281a84u64) }
        0x10281a84 => { vm.decode(0x73698119u32); let d = 29i16; let s = 29i16; let s2 = -1i16; let imm = 1104i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10281a88u64) }
        0x10281a88 => { vm.decode(0x0u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x10281a8cu64) }
        0x10286380 => { vm.decode(0x7b79ca59u32); let d = 29i16; let s = 29i16; let s2 = -1i16; let imm = -2768i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286384u64) }
        0x10286384 => { vm.decode(0x9b29d570u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 2760i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286388u64) }
        0x10286388 => { vm.decode(0x8b21d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028638cu64) }
        0x1028638c => { vm.decode(0x9b19d5b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286390u64) }
        0x10286390 => { vm.decode(0x8b11d5b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286394u64) }
        0x10286394 => { vm.decode(0x9b09d4b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286398u64) }
        0x10286398 => { vm.decode(0x8b01d4b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028639cu64) }
        0x1028639c => { vm.decode(0x9b199530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102863a0u64) }
        0x102863a0 => { vm.decode(0x8b119530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102863a4u64) }
        0x102863a4 => { vm.decode(0x9b099430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102863a8u64) }
        0x102863a8 => { vm.decode(0x8b019430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102863acu64) }
        0x102863ac => { vm.decode(0x4c80d875u32); let d = 23i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102863b0u64) }
        0x102863b0 => { vm.decode(0x2a84a16fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102863b4u64) }
        0x102863b4 => { vm.decode(0x190094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102863b8u64) }
        0x102863b8 => { vm.decode(0x2a84816fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102863bcu64) }
        0x102863bc => { vm.decode(0x193814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102863c0u64) }
        0x102863c0 => { vm.decode(0x2884a16fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102863c4u64) }
        0x102863c4 => { vm.decode(0x19189430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102863c8u64) }
        0x102863c8 => { vm.decode(0x2280202fu32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102863ccu64) }
        0x102863cc => { vm.decode(0x1b281430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102863d0u64) }
        0x102863d0 => { vm.decode(0x2080202fu32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102863d4u64) }
        0x102863d4 => { vm.decode(0x1b201430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102863d8u64) }
        0x102863d8 => { vm.decode(0x18100811u32); let d = 17i16; let s = 4i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102863dcu64) }
        0x102863dc => { vm.decode(0x2080212fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102863e0u64) }
        0x102863e0 => { vm.decode(0x1b301430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102863e4u64) }
        0x102863e4 => { vm.decode(0x2280012fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102863e8u64) }
        0x102863e8 => { vm.decode(0x1b1814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102863ecu64) }
        0x102863ec => { vm.decode(0x2280212fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102863f0u64) }
        0x102863f0 => { vm.decode(0x1b1014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102863f4u64) }
        0x102863f4 => { vm.decode(0x2084012fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102863f8u64) }
        0x102863f8 => { vm.decode(0x1b0814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102863fcu64) }
        0x102863fc => { vm.decode(0x2084212fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286400u64) }
        0x10286400 => { vm.decode(0x1b0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286404u64) }
        0x10286404 => { vm.decode(0x22a0002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286408u64) }
        0x10286408 => { vm.decode(0x1b089430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028640cu64) }
        0x1028640c => { vm.decode(0x22a0202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286410u64) }
        0x10286410 => { vm.decode(0x1b109430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286414u64) }
        0x10286414 => { vm.decode(0x20a4002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286418u64) }
        0x10286418 => { vm.decode(0x1b009430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028641cu64) }
        0x1028641c => { vm.decode(0x20a4202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286420u64) }
        0x10286420 => { vm.decode(0x1b3814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286424u64) }
        0x10286424 => { vm.decode(0x22a4202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286428u64) }
        0x10286428 => { vm.decode(0x1b3014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028642cu64) }
        0x1028642c => { vm.decode(0x20a0006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286430u64) }
        0x10286430 => { vm.decode(0x1b2814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286434u64) }
        0x10286434 => { vm.decode(0x20a0206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286438u64) }
        0x10286438 => { vm.decode(0x1b2014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028643cu64) }
        0x1028643c => { vm.decode(0x22a0006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286440u64) }
        0x10286440 => { vm.decode(0x190814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286444u64) }
        0x10286444 => { vm.decode(0x22a0206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286448u64) }
        0x10286448 => { vm.decode(0x19109430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028644cu64) }
        0x1028644c => { vm.decode(0x20a4006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286450u64) }
        0x10286450 => { vm.decode(0x19089430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286454u64) }
        0x10286454 => { vm.decode(0x20a4206fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286458u64) }
        0x10286458 => { vm.decode(0x19009430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028645cu64) }
        0x1028645c => { vm.decode(0x22a4006fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286460u64) }
        0x10286460 => { vm.decode(0x193014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286464u64) }
        0x10286464 => { vm.decode(0x20a0202fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286468u64) }
        0x10286468 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028646cu64) }
        0x1028646c => { vm.decode(0x20a0002fu32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286470u64) }
        0x10286470 => { vm.decode(0x4080002fu32); let d = 16i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286474u64) }
        0x10286474 => { vm.decode(0x28c012fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286478u64) }
        0x10286478 => { vm.decode(0xb181530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028647cu64) }
        0x1028647c => { vm.decode(0x28c212fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286480u64) }
        0x10286480 => { vm.decode(0xb101530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286484u64) }
        0x10286484 => { vm.decode(0x88016fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286488u64) }
        0x10286488 => { vm.decode(0xb001530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028648cu64) }
        0x1028648c => { vm.decode(0x88216fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286490u64) }
        0x10286490 => { vm.decode(0x9389db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286494u64) }
        0x10286494 => { vm.decode(0x288016fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286498u64) }
        0x10286498 => { vm.decode(0x9309db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028649cu64) }
        0x1028649c => { vm.decode(0x288216fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102864a0u64) }
        0x102864a0 => { vm.decode(0x9289db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102864a4u64) }
        0x102864a4 => { vm.decode(0x8c016fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102864a8u64) }
        0x102864a8 => { vm.decode(0x9209db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102864acu64) }
        0x102864ac => { vm.decode(0x8c216fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102864b0u64) }
        0x102864b0 => { vm.decode(0x9389d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102864b4u64) }
        0x102864b4 => { vm.decode(0x28c016fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102864b8u64) }
        0x102864b8 => { vm.decode(0x9309d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102864bcu64) }
        0x102864bc => { vm.decode(0x28c216fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102864c0u64) }
        0x102864c0 => { vm.decode(0x9109db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102864c4u64) }
        0x102864c4 => { vm.decode(0x888012fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102864c8u64) }
        0x102864c8 => { vm.decode(0x9189db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102864ccu64) }
        0x102864cc => { vm.decode(0x888212fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102864d0u64) }
        0x102864d0 => { vm.decode(0x9209d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102864d4u64) }
        0x102864d4 => { vm.decode(0xa88012fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102864d8u64) }
        0x102864d8 => { vm.decode(0x9289d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102864dcu64) }
        0x102864dc => { vm.decode(0xa88212fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102864e0u64) }
        0x102864e0 => { vm.decode(0x9089db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102864e4u64) }
        0x102864e4 => { vm.decode(0x88c012fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102864e8u64) }
        0x102864e8 => { vm.decode(0x9009db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102864ecu64) }
        0x102864ec => { vm.decode(0x88c212fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102864f0u64) }
        0x102864f0 => { vm.decode(0x9189d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102864f4u64) }
        0x102864f4 => { vm.decode(0xa8c012fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102864f8u64) }
        0x102864f8 => { vm.decode(0x9109d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102864fcu64) }
        0x102864fc => { vm.decode(0xa8c212fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286500u64) }
        0x10286500 => { vm.decode(0x9089d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286504u64) }
        0x10286504 => { vm.decode(0x888016fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286508u64) }
        0x10286508 => { vm.decode(0x9009d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028650cu64) }
        0x1028650c => { vm.decode(0x888216fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286510u64) }
        0x10286510 => { vm.decode(0x9381db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286514u64) }
        0x10286514 => { vm.decode(0xa88016fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286518u64) }
        0x10286518 => { vm.decode(0x9301db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028651cu64) }
        0x1028651c => { vm.decode(0xa88216fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286520u64) }
        0x10286520 => { vm.decode(0x9281db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286524u64) }
        0x10286524 => { vm.decode(0x88c016fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286528u64) }
        0x10286528 => { vm.decode(0x9201db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028652cu64) }
        0x1028652c => { vm.decode(0x88c216fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286530u64) }
        0x10286530 => { vm.decode(0x9381d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286534u64) }
        0x10286534 => { vm.decode(0xa8c016fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286538u64) }
        0x10286538 => { vm.decode(0x9301d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028653cu64) }
        0x1028653c => { vm.decode(0xa8c216fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286540u64) }
        0x10286540 => { vm.decode(0x9281d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286544u64) }
        0x10286544 => { vm.decode(0x88812fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286548u64) }
        0x10286548 => { vm.decode(0x9201d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028654cu64) }
        0x1028654c => { vm.decode(0x88a12fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286550u64) }
        0x10286550 => { vm.decode(0x9181db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286554u64) }
        0x10286554 => { vm.decode(0x288812fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286558u64) }
        0x10286558 => { vm.decode(0x9101db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028655cu64) }
        0x1028655c => { vm.decode(0x288a12fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286560u64) }
        0x10286560 => { vm.decode(0x9081db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286564u64) }
        0x10286564 => { vm.decode(0x8c812fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286568u64) }
        0x10286568 => { vm.decode(0x9181d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028656cu64) }
        0x1028656c => { vm.decode(0x8ca12fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286570u64) }
        0x10286570 => { vm.decode(0x9101d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286574u64) }
        0x10286574 => { vm.decode(0x28c812fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286578u64) }
        0x10286578 => { vm.decode(0x9081d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028657cu64) }
        0x1028657c => { vm.decode(0x28ca12fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286580u64) }
        0x10286580 => { vm.decode(0x9001d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286584u64) }
        0x10286584 => { vm.decode(0x88816fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286588u64) }
        0x10286588 => { vm.decode(0x93895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028658cu64) }
        0x1028658c => { vm.decode(0x88a16fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286590u64) }
        0x10286590 => { vm.decode(0x93095b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286594u64) }
        0x10286594 => { vm.decode(0x288816fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286598u64) }
        0x10286598 => { vm.decode(0x9309530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028659cu64) }
        0x1028659c => { vm.decode(0x288a16fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102865a0u64) }
        0x102865a0 => { vm.decode(0x9209530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102865a4u64) }
        0x102865a4 => { vm.decode(0x8c816fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102865a8u64) }
        0x102865a8 => { vm.decode(0x92895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102865acu64) }
        0x102865ac => { vm.decode(0x8ca16fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102865b0u64) }
        0x102865b0 => { vm.decode(0x92095b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102865b4u64) }
        0x102865b4 => { vm.decode(0x28c816fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102865b8u64) }
        0x102865b8 => { vm.decode(0x91095b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102865bcu64) }
        0x102865bc => { vm.decode(0x28ca16fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102865c0u64) }
        0x102865c0 => { vm.decode(0x9381530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102865c4u64) }
        0x102865c4 => { vm.decode(0x888812fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102865c8u64) }
        0x102865c8 => { vm.decode(0x9301530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102865ccu64) }
        0x102865cc => { vm.decode(0x888a12fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102865d0u64) }
        0x102865d0 => { vm.decode(0x9281530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102865d4u64) }
        0x102865d4 => { vm.decode(0xa88812fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102865d8u64) }
        0x102865d8 => { vm.decode(0x91815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102865dcu64) }
        0x102865dc => { vm.decode(0xa88a12fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102865e0u64) }
        0x102865e0 => { vm.decode(0x9101530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102865e4u64) }
        0x102865e4 => { vm.decode(0x88c812fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102865e8u64) }
        0x102865e8 => { vm.decode(0x9081530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102865ecu64) }
        0x102865ec => { vm.decode(0x88ca12fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102865f0u64) }
        0x102865f0 => { vm.decode(0x9001530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102865f4u64) }
        0x102865f4 => { vm.decode(0xa8c812fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102865f8u64) }
        0x102865f8 => { vm.decode(0x92815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102865fcu64) }
        0x102865fc => { vm.decode(0xa8ca12fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286600u64) }
        0x10286600 => { vm.decode(0x9389530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286604u64) }
        0x10286604 => { vm.decode(0x888816fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286608u64) }
        0x10286608 => { vm.decode(0x90015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028660cu64) }
        0x1028660c => { vm.decode(0x488aa16fu32); let d = 22i16; let s = 5i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286610u64) }
        0x10286610 => { vm.decode(0x4a82816fu32); let d = 20i16; let s = 5i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286614u64) }
        0x10286614 => { vm.decode(0x6a88a16fu32); let d = 19i16; let s = 5i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286618u64) }
        0x10286618 => { vm.decode(0x88c816fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028661cu64) }
        0x1028661c => { vm.decode(0x9181530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286620u64) }
        0x10286620 => { vm.decode(0x6082012fu32); let d = 21i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286624u64) }
        0x10286624 => { vm.decode(0x1d47778eu32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = -4063232i64;
        vm.st(d, imm as u64);
        Ok(0x10286628u64) }
        0x10286628 => { vm.decode(0xcc5f3418u32); let d = 18i16; let s = 30i16; let s2 = -1i16; let imm = 39592i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x1028662cu64) }
        0x1028662c => { vm.decode(0x690124f5u32); let d = 2i16; let s = 18i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286630u64) }
        0x10286630 => { vm.decode(0xb189530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286634u64) }
        0x10286634 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286638u64) }
        0x10286638 => { vm.decode(0xa636u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 20i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028663cu64) }
        0x1028663c => { vm.decode(0x6b1d7422u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2684i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10286640u64) }
        0x10286640 => { vm.decode(0xb2915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286644u64) }
        0x10286644 => { vm.decode(0x5a102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10286648u64) }
        0x10286648 => { vm.decode(0x8b3114b0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028664cu64) }
        0x1028664c => { vm.decode(0x610204f5u32); let d = 4i16; let s = 18i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286650u64) }
        0x10286650 => { vm.decode(0xe00d875u32); let d = 25i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10286654u64) }
        0x10286654 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10286658u64 + 4);
        vm.wr(37, 0x10286658u64 + 4);
        Ok(0x10286658u64) }
        0x10286658 => { vm.decode(0x661d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2664i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028665cu64) }
        0x1028665c => { vm.decode(0x6b5f3418u32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 39672i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10286660u64) }
        0x10286660 => { vm.decode(0xa04124f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286664u64) }
        0x10286664 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286668u64) }
        0x10286668 => { vm.decode(0x101019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028666cu64) }
        0x1028666c => { vm.decode(0xb2115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286670u64) }
        0x10286670 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10286674u64) }
        0x10286674 => { vm.decode(0x641f1019u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286678u64) }
        0x10286678 => { vm.decode(0x9201530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028667cu64) }
        0x1028667c => { vm.decode(0xb311530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286680u64) }
        0x10286680 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286684u64) }
        0x10286684 => { vm.decode(0x288443afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286688u64) }
        0x10286688 => { vm.decode(0x610204f5u32); let d = 4i16; let s = 18i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028668cu64) }
        0x1028668c => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286690u64) }
        0x10286690 => { vm.decode(0x9005cb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286694u64) }
        0x10286694 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10286698u64) }
        0x10286698 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028669cu64 + 4);
        vm.wr(37, 0x1028669cu64 + 4);
        Ok(0x1028669cu64) }
        0x1028669c => { vm.decode(0x9b00d5b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102866a0u64) }
        0x102866a0 => { vm.decode(0x208443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102866a4u64) }
        0x102866a4 => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102866a8u64) }
        0x102866a8 => { vm.decode(0x192014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102866acu64) }
        0x102866ac => { vm.decode(0x208463efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102866b0u64) }
        0x102866b0 => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102866b4u64) }
        0x102866b4 => { vm.decode(0x190894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102866b8u64) }
        0x102866b8 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102866bcu64) }
        0x102866bc => { vm.decode(0x690004f5u32); let d = 2i16; let s = 18i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102866c0u64) }
        0x102866c0 => { vm.decode(0x228463efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102866c4u64) }
        0x102866c4 => { vm.decode(0x690204f5u32); let d = 6i16; let s = 18i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102866c8u64) }
        0x102866c8 => { vm.decode(0x288043afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102866ccu64) }
        0x102866cc => { vm.decode(0x610204f5u32); let d = 4i16; let s = 18i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102866d0u64) }
        0x102866d0 => { vm.decode(0x2a8043afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102866d4u64) }
        0x102866d4 => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102866d8u64) }
        0x102866d8 => { vm.decode(0x1b109430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102866dcu64) }
        0x102866dc => { vm.decode(0x891895b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102866e0u64) }
        0x102866e0 => { vm.decode(0x288063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102866e4u64) }
        0x102866e4 => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102866e8u64) }
        0x102866e8 => { vm.decode(0x1b081430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102866ecu64) }
        0x102866ec => { vm.decode(0x24000506u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102866f0u64) }
        0x102866f0 => { vm.decode(0x2208402fu32); let d = 3i16; let s = 16i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102866f4u64) }
        0x102866f4 => { vm.decode(0x19289530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102866f8u64) }
        0x102866f8 => { vm.decode(0x60499419u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2064i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102866fcu64) }
        0x102866fc => { vm.decode(0x1b089530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286700u64) }
        0x10286700 => { vm.decode(0x732d1059u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 1288i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286704u64) }
        0x10286704 => { vm.decode(0x642d9059u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1800i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286708u64) }
        0x10286708 => { vm.decode(0x1b381530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028670cu64) }
        0x1028670c => { vm.decode(0x646b9059u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1936i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286710u64) }
        0x10286710 => { vm.decode(0x1b009530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286714u64) }
        0x10286714 => { vm.decode(0x64699159u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1872i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286718u64) }
        0x10286718 => { vm.decode(0x1b3815b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028671cu64) }
        0x1028671c => { vm.decode(0x8b2855f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286720u64) }
        0x10286720 => { vm.decode(0x91054b0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286724u64) }
        0x10286724 => { vm.decode(0xb3055b0u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286728u64) }
        0x10286728 => { vm.decode(0x8c00a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 148i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028672cu64).wrapping_add((imm * 4) as u64) } else { 0x1028672cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028672cu64) }
        0x1028672c => { vm.decode(0xb2015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286730u64) }
        0x10286730 => { vm.decode(0x8900dd30u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286734u64) }
        0x10286734 => { vm.decode(0x2a88636fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286738u64) }
        0x10286738 => { vm.decode(0x888e32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028673cu64) }
        0x1028673c => { vm.decode(0x2a82c32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286740u64) }
        0x10286740 => { vm.decode(0x688ecb2fu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286744u64) }
        0x10286744 => { vm.decode(0x6a8ae32fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286748u64) }
        0x10286748 => { vm.decode(0x488ee32fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028674cu64) }
        0x1028674c => { vm.decode(0x6882e36fu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286750u64) }
        0x10286750 => { vm.decode(0x4882c36fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286754u64) }
        0x10286754 => { vm.decode(0x6a88e36fu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286758u64) }
        0x10286758 => { vm.decode(0x4a8ce32fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028675cu64) }
        0x1028675c => { vm.decode(0x4884c36fu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286760u64) }
        0x10286760 => { vm.decode(0x6a84cb2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286764u64) }
        0x10286764 => { vm.decode(0x4a84cb6fu32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286768u64) }
        0x10286768 => { vm.decode(0x288eeb6fu32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028676cu64) }
        0x1028676c => { vm.decode(0xa8acb6fu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286770u64) }
        0x10286770 => { vm.decode(0x2a86eb6fu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286774u64) }
        0x10286774 => { vm.decode(0x824bafu32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286778u64) }
        0x10286778 => { vm.decode(0x22884bafu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028677cu64) }
        0x1028677c => { vm.decode(0x2886bafu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286780u64) }
        0x10286780 => { vm.decode(0x22844bafu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286784u64) }
        0x10286784 => { vm.decode(0x844bafu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286788u64) }
        0x10286788 => { vm.decode(0x228e63afu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028678cu64) }
        0x1028678c => { vm.decode(0x8e63afu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286790u64) }
        0x10286790 => { vm.decode(0x28243efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286794u64) }
        0x10286794 => { vm.decode(0x24000446u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10286798u64) }
        0x10286798 => { vm.decode(0x408c8027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 277i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028679cu64).wrapping_add((imm * 4) as u64) } else { 0x1028679cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028679cu64) }
        0x1028679c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102867a0u64) } }
        0x102867a0 => { vm.decode(0x24020406u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 2i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102867a4u64) }
        0x102867a4 => { vm.decode(0x54040aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 490i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102867a8u64).wrapping_add((imm * 4) as u64) } else { 0x102867a8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102867a8u64) }
        0x102867a8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102867acu64) } }
        0x102867ac => { vm.decode(0x59ac04e1u32); let d = -1i16; let s = 17i16; let s2 = -1i16; let imm = 7418i64;
        vm.wr(36, if (vm.ld(s) as i64) < 1 { (0x102867b0u64).wrapping_add((imm * 4) as u64) } else { 0x102867b0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102867b0u64) }
        0x102867b0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102867b4u64) } }
        0x102867b4 => { vm.decode(0x2a84632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102867b8u64) }
        0x102867b8 => { vm.decode(0xa24404f5u32); let d = 17i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102867bcu64) }
        0x102867bc => { vm.decode(0x208043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102867c0u64) }
        0x102867c0 => { vm.decode(0x4c008191u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102867c4u64) }
        0x102867c4 => { vm.decode(0x200092fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102867c8u64) }
        0x102867c8 => { vm.decode(0x42004b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102867ccu64) }
        0x102867cc => { vm.decode(0x1099u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102867d0u64) }
        0x102867d0 => { vm.decode(0x660d1059u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102867d4u64) }
        0x102867d4 => { vm.decode(0x9119u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102867d8u64) }
        0x102867d8 => { vm.decode(0x424024f5u32); let d = 1i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102867dcu64) }
        0x102867dc => { vm.decode(0x801099u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102867e0u64) }
        0x102867e0 => { vm.decode(0xadfb8df1u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = -3i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102867e4u64).wrapping_add((imm * 4) as u64) } else { 0x102867e4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102867e4u64) }
        0x102867e4 => { vm.decode(0x833u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102867e8u64) }
        0x102867e8 => { vm.decode(0xb281c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102867ecu64) }
        0x102867ec => { vm.decode(0x41119u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 72i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102867f0u64) }
        0x102867f0 => { vm.decode(0x109119u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 96i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102867f4u64) }
        0x102867f4 => { vm.decode(0x28643efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102867f8u64) }
        0x102867f8 => { vm.decode(0x424024f5u32); let d = 1i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102867fcu64) }
        0x102867fc => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286800u64) }
        0x10286800 => { vm.decode(0xadfb8df1u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = -3i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10286804u64).wrapping_add((imm * 4) as u64) } else { 0x10286804u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286804u64) }
        0x10286804 => { vm.decode(0x430u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286808u64) }
        0x10286808 => { vm.decode(0x9209470u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028680cu64) }
        0x1028680c => { vm.decode(0x43657022u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10286810u64) }
        0x10286810 => { vm.decode(0xb301cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286814u64) }
        0x10286814 => { vm.decode(0x501119u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 112i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286818u64) }
        0x10286818 => { vm.decode(0x169019u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 168i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028681cu64) }
        0x1028681c => { vm.decode(0x424024f5u32); let d = 1i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286820u64) }
        0x10286820 => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286824u64) }
        0x10286824 => { vm.decode(0xadfb8df1u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = -3i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10286828u64).wrapping_add((imm * 4) as u64) } else { 0x10286828u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286828u64) }
        0x10286828 => { vm.decode(0x430u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028682cu64) }
        0x1028682c => { vm.decode(0xb289cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286830u64) }
        0x10286830 => { vm.decode(0xb209cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286834u64) }
        0x10286834 => { vm.decode(0xb389c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286838u64) }
        0x10286838 => { vm.decode(0xb309c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028683cu64) }
        0x1028683c => { vm.decode(0xb289c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286840u64) }
        0x10286840 => { vm.decode(0xb209c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286844u64) }
        0x10286844 => { vm.decode(0xb189cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286848u64) }
        0x10286848 => { vm.decode(0xb109cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028684cu64) }
        0x1028684c => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286850u64) }
        0x10286850 => { vm.decode(0x2004016fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286854u64) }
        0x10286854 => { vm.decode(0x931dd30u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 2512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286858u64) }
        0x10286858 => { vm.decode(0x19299c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028685cu64) }
        0x1028685c => { vm.decode(0x620f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2504i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286860u64) }
        0x10286860 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286864u64) }
        0x10286864 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10286868u64 + 4);
        vm.wr(37, 0x10286868u64 + 4);
        Ok(0x10286868u64) }
        0x10286868 => { vm.decode(0x938d530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028686cu64) }
        0x1028686c => { vm.decode(0x208063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286870u64) }
        0x10286870 => { vm.decode(0x39281430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286874u64) }
        0x10286874 => { vm.decode(0x2a80636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286878u64) }
        0x10286878 => { vm.decode(0x50160119u32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = 232i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028687cu64) }
        0x1028687c => { vm.decode(0x100236u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10286880u64) }
        0x10286880 => { vm.decode(0x10009119u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286884u64) }
        0x10286884 => { vm.decode(0x5288d9u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 16817i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286888u64) }
        0x10286888 => { vm.decode(0x20422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 17i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1028688cu64) }
        0x1028688c => { vm.decode(0x5e700cd9u32); let d = 20i16; let s = 1i16; let s2 = -1i16; let imm = -12495i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286890u64) }
        0x10286890 => { vm.decode(0x760d9419u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286894u64) }
        0x10286894 => { vm.decode(0x6120506u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 11i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10286898u64) }
        0x10286898 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028689cu64).wrapping_add((imm * 4) as u64) } else { 0x1028689cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028689cu64) }
        0x1028689c => { vm.decode(0x9e76efd9u32); let d = 21i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102868a0u64) }
        0x102868a0 => { vm.decode(0x84141du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 4880i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102868a4u64) }
        0x102868a8 => { vm.decode(0x6041b835u32); let d = 1i16; let s = 7i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102868acu64) }
        0x102868ac => { vm.decode(0x4e081ff5u32); let d = 4i16; let s = 30i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shl(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x102868b0u64) }
        0x102868b0 => { vm.decode(0x120207f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102868b4u64) }
        0x102868b4 => { vm.decode(0x94352175u32); let d = 3i16; let s = 30i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shr(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x102868b8u64) }
        0x102868b8 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102868bcu64) }
        0x102868bc => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102868c0u64) }
        0x102868c0 => { vm.decode(0x492104f5u32); let d = 2i16; let s = 16i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102868c4u64) }
        0x102868c4 => { vm.decode(0x10400030u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102868c8u64) }
        0x102868c8 => { vm.decode(0x8b00236u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102868ccu64) }
        0x102868cc => { vm.decode(0x6120506u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 11i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102868d0u64) }
        0x102868d0 => { vm.decode(0x56848a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 879i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102868d4u64).wrapping_add((imm * 4) as u64) } else { 0x102868d4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102868d4u64) }
        0x102868d4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102868d8u64) } }
        0x102868d8 => { vm.decode(0x40030bb5u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102868dcu64) }
        0x102868dc => { vm.decode(0xe000bd75u32); let d = 22i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102868e0u64) }
        0x102868e0 => { vm.decode(0x616104f5u32); let d = 1i16; let s = 17i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102868e4u64) }
        0x102868e4 => { vm.decode(0x400a092fu32); let d = 30i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102868e8u64) }
        0x102868e8 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102868ecu64) }
        0x102868ec => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102868f0u64) }
        0x102868f0 => { vm.decode(0x9b191530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102868f4u64) }
        0x102868f4 => { vm.decode(0x88736fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102868f8u64) }
        0x102868f8 => { vm.decode(0xb2004f5u32); let d = 2i16; let s = 2i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102868fcu64) }
        0x102868fc => { vm.decode(0xb111530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286900u64) }
        0x10286900 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286904u64) }
        0x10286904 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286908u64) }
        0x10286908 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028690cu64) }
        0x1028690c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10286910u64 + 4);
        vm.wr(37, 0x10286910u64 + 4);
        Ok(0x10286910u64) }
        0x10286910 => { vm.decode(0x490d875u32); let d = 5i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10286914u64) }
        0x10286914 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10286918u64) }
        0x10286918 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028691cu64) }
        0x1028691c => { vm.decode(0x830bb5u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10286920u64) }
        0x10286920 => { vm.decode(0x92a0b75u32); let d = 4i16; let s = 19i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_sub(vm.ld(s2));
        vm.st(d, v);
        Ok(0x10286924u64) }
        0x10286924 => { vm.decode(0x260bb5u32); let d = 5i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10286928u64) }
        0x10286928 => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1028692cu64) }
        0x1028692c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10286930u64).wrapping_add((imm * 4) as u64) } else { 0x10286930u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286930u64) }
        0x10286930 => { vm.decode(0x1019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286934u64) }
        0x10286934 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10286938u64) }
        0x10286938 => { vm.decode(0x22400d75u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1028693cu64) }
        0x1028693c => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10286940u64) }
        0x10286940 => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10286944u64) }
        0x10286944 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10286948u64).wrapping_add((imm * 4) as u64) } else { 0x10286948u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286948u64) }
        0x10286948 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028694cu64) } }
        0x1028694c => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10286950u64) }
        0x10286950 => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10286954u64) }
        0x10286954 => { vm.decode(0x739c34a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -44i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10286958u64).wrapping_add((imm * 4) as u64) } else { 0x10286958u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286958u64) }
        0x10286958 => { vm.decode(0x2009019u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028695cu64) }
        0x1028695c => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10286960u64) }
        0x10286960 => { vm.decode(0x32402d75u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10286964u64) }
        0x10286964 => { vm.decode(0xa0204118u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10286968u64) }
        0x10286968 => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1028696cu64) }
        0x1028696c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10286970u64).wrapping_add((imm * 4) as u64) } else { 0x10286970u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286970u64) }
        0x10286970 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10286974u64) } }
        0x10286974 => { vm.decode(0xd00101du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 1320i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10286978u64) }
        0x10286978 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028697cu64) } }
        0x1028697c => { vm.decode(0x9b1815b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286980u64) }
        0x10286980 => { vm.decode(0x8b305430u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286984u64) }
        0x10286984 => { vm.decode(0x282636fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286988u64) }
        0x10286988 => { vm.decode(0x2882436fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028698cu64) }
        0x1028698c => { vm.decode(0xa8c4b2fu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286990u64) }
        0x10286990 => { vm.decode(0x6a8a4b6fu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286994u64) }
        0x10286994 => { vm.decode(0x6a86636fu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286998u64) }
        0x10286998 => { vm.decode(0x688e436fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028699cu64) }
        0x1028699c => { vm.decode(0x4a8e436fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102869a0u64) }
        0x102869a0 => { vm.decode(0x6088c32fu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102869a4u64) }
        0x102869a4 => { vm.decode(0x4886636fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102869a8u64) }
        0x102869a8 => { vm.decode(0x4088e32fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102869acu64) }
        0x102869ac => { vm.decode(0x4280c32fu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102869b0u64) }
        0x102869b0 => { vm.decode(0x6284eb2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102869b4u64) }
        0x102869b4 => { vm.decode(0x4280eb2fu32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102869b8u64) }
        0x102869b8 => { vm.decode(0x208acb6fu32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102869bcu64) }
        0x102869bc => { vm.decode(0x8eeb2fu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102869c0u64) }
        0x102869c0 => { vm.decode(0x2282cb6fu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102869c4u64) }
        0x102869c4 => { vm.decode(0x2088e36fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102869c8u64) }
        0x102869c8 => { vm.decode(0x286cb2fu32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102869ccu64) }
        0x102869cc => { vm.decode(0x2288eb6fu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102869d0u64) }
        0x102869d0 => { vm.decode(0x2084cb6fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102869d4u64) }
        0x102869d4 => { vm.decode(0x8ce36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102869d8u64) }
        0x102869d8 => { vm.decode(0x284cb6fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102869dcu64) }
        0x102869dc => { vm.decode(0x228ee36fu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102869e0u64) }
        0x102869e0 => { vm.decode(0x88ac32fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102869e4u64) }
        0x102869e4 => { vm.decode(0x24000546u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 12i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102869e8u64) }
        0x102869e8 => { vm.decode(0x140c8aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 251i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102869ecu64).wrapping_add((imm * 4) as u64) } else { 0x102869ecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102869ecu64) }
        0x102869ec => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102869f0u64) } }
        0x102869f0 => { vm.decode(0x24020506u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 10i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102869f4u64) }
        0x102869f4 => { vm.decode(0x500c88a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 467i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102869f8u64).wrapping_add((imm * 4) as u64) } else { 0x102869f8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102869f8u64) }
        0x102869f8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102869fcu64) } }
        0x102869fc => { vm.decode(0x24100506u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 9i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10286a00u64) }
        0x10286a00 => { vm.decode(0x428488a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 903i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10286a04u64).wrapping_add((imm * 4) as u64) } else { 0x10286a04u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286a04u64) }
        0x10286a04 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10286a08u64) } }
        0x10286a08 => { vm.decode(0x2a84632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286a0cu64) }
        0x10286a0c => { vm.decode(0xa840a4f5u32); let d = 3i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286a10u64) }
        0x10286a10 => { vm.decode(0x19309530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286a14u64) }
        0x10286a14 => { vm.decode(0xaa4084f5u32); let d = 3i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286a18u64) }
        0x10286a18 => { vm.decode(0x19209530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286a1cu64) }
        0x10286a1c => { vm.decode(0xa84084f5u32); let d = 3i16; let s = 9i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286a20u64) }
        0x10286a20 => { vm.decode(0x19189db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286a24u64) }
        0x10286a24 => { vm.decode(0x8a4004f5u32); let d = 3i16; let s = 2i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286a28u64) }
        0x10286a28 => { vm.decode(0x19109db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286a2cu64) }
        0x10286a2c => { vm.decode(0x884084f5u32); let d = 3i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286a30u64) }
        0x10286a30 => { vm.decode(0x19089db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286a34u64) }
        0x10286a34 => { vm.decode(0xaa4024f5u32); let d = 3i16; let s = 7i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286a38u64) }
        0x10286a38 => { vm.decode(0x19009db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286a3cu64) }
        0x10286a3c => { vm.decode(0x824424f5u32); let d = 17i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286a40u64) }
        0x10286a40 => { vm.decode(0x208043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286a44u64) }
        0x10286a44 => { vm.decode(0x620a012fu32); let d = 23i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286a48u64) }
        0x10286a48 => { vm.decode(0x14008091u32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10286a4cu64) }
        0x10286a4c => { vm.decode(0x4a8263afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286a50u64) }
        0x10286a50 => { vm.decode(0x2a80402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286a54u64) }
        0x10286a54 => { vm.decode(0x109119u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 96i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286a58u64) }
        0x10286a58 => { vm.decode(0x19099d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286a5cu64) }
        0x10286a5c => { vm.decode(0x640d9059u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286a60u64) }
        0x10286a60 => { vm.decode(0x19389530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286a64u64) }
        0x10286a64 => { vm.decode(0x19391db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286a68u64) }
        0x10286a68 => { vm.decode(0x19311cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286a6cu64) }
        0x10286a6c => { vm.decode(0xa251062u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10286a70u64) }
        0x10286a70 => { vm.decode(0x8643efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286a74u64) }
        0x10286a74 => { vm.decode(0x688c43afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286a78u64) }
        0x10286a78 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10286a7cu64) }
        0x10286a7c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10286a80u64 + 4);
        vm.wr(37, 0x10286a80u64 + 4);
        Ok(0x10286a80u64) }
        0x10286a80 => { vm.decode(0x62598559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2416i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286a84u64) }
        0x10286a84 => { vm.decode(0x762d8019u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286a88u64) }
        0x10286a88 => { vm.decode(0x10401019u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286a8cu64) }
        0x10286a8c => { vm.decode(0x208063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286a90u64) }
        0x10286a90 => { vm.decode(0x1b201cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286a94u64) }
        0x10286a94 => { vm.decode(0x2884402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286a98u64) }
        0x10286a98 => { vm.decode(0x8b211530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286a9cu64) }
        0x10286a9c => { vm.decode(0x9b1914b0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286aa0u64) }
        0x10286aa0 => { vm.decode(0x9b1154b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286aa4u64) }
        0x10286aa4 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286aa8u64) }
        0x10286aa8 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286aacu64) }
        0x10286aac => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10286ab0u64) }
        0x10286ab0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10286ab4u64 + 4);
        vm.wr(37, 0x10286ab4u64 + 4);
        Ok(0x10286ab4u64) }
        0x10286ab4 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286ab8u64) }
        0x10286ab8 => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286abcu64) }
        0x10286abc => { vm.decode(0x2280406fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286ac0u64) }
        0x10286ac0 => { vm.decode(0x8b2115b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286ac4u64) }
        0x10286ac4 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10286ac8u64) }
        0x10286ac8 => { vm.decode(0x8b315570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286accu64) }
        0x10286acc => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286ad0u64) }
        0x10286ad0 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286ad4u64) }
        0x10286ad4 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10286ad8u64) }
        0x10286ad8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10286adcu64 + 4);
        vm.wr(37, 0x10286adcu64 + 4);
        Ok(0x10286adcu64) }
        0x10286adc => { vm.decode(0x102036u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10286ae0u64) }
        0x10286ae0 => { vm.decode(0x40020bb5u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10286ae4u64) }
        0x10286ae4 => { vm.decode(0x19209c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286ae8u64) }
        0x10286ae8 => { vm.decode(0x21d54a58u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 58219i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10286aecu64) }
        0x10286aec => { vm.decode(0x422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10286af0u64) }
        0x10286af0 => { vm.decode(0x1b3014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286af4u64) }
        0x10286af4 => { vm.decode(0x54223759u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = 8132i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286af8u64) }
        0x10286af8 => { vm.decode(0x10109019u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286afcu64) }
        0x10286afc => { vm.decode(0x770d1419u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286b00u64) }
        0x10286b00 => { vm.decode(0x6100036u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10286b04u64) }
        0x10286b04 => { vm.decode(0x24000446u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10286b08u64) }
        0x10286b08 => { vm.decode(0x2220031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 17i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10286b0cu64).wrapping_add((imm * 4) as u64) } else { 0x10286b0cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286b0cu64) }
        0x10286b0c => { vm.decode(0x9b305430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286b10u64) }
        0x10286b10 => { vm.decode(0xad24141du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 5628i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10286b14u64) }
        0x10286b18 => { vm.decode(0x8624735u32); let d = 1i16; let s = 4i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10286b1cu64) }
        0x10286b1c => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x10286b20u64) }
        0x10286b20 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10286b24u64) }
        0x10286b24 => { vm.decode(0x74304675u32); let d = 3i16; let s = 3i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10286b28u64) }
        0x10286b28 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10286b2cu64) }
        0x10286b2c => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10286b30u64) }
        0x10286b30 => { vm.decode(0x8c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286b34u64) }
        0x10286b34 => { vm.decode(0x8b2004f5u32); let d = 2i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286b38u64) }
        0x10286b38 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10286b3cu64) }
        0x10286b3c => { vm.decode(0x8903036u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10286b40u64) }
        0x10286b40 => { vm.decode(0x628243efu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286b44u64) }
        0x10286b44 => { vm.decode(0x24000446u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10286b48u64) }
        0x10286b48 => { vm.decode(0x428402a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 908i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10286b4cu64).wrapping_add((imm * 4) as u64) } else { 0x10286b4cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286b4cu64) }
        0x10286b4c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10286b50u64) } }
        0x10286b50 => { vm.decode(0x2084c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286b54u64) }
        0x10286b54 => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286b58u64) }
        0x10286b58 => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286b5cu64) }
        0x10286b5c => { vm.decode(0x82c202fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286b60u64) }
        0x10286b60 => { vm.decode(0x9b191530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286b64u64) }
        0x10286b64 => { vm.decode(0xb091530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286b68u64) }
        0x10286b68 => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286b6cu64) }
        0x10286b6c => { vm.decode(0x40820bb5u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10286b70u64) }
        0x10286b70 => { vm.decode(0x44104875u32); let d = 2i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10286b74u64) }
        0x10286b74 => { vm.decode(0xc040ad75u32); let d = 21i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10286b78u64) }
        0x10286b78 => { vm.decode(0x836004f5u32); let d = 1i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286b7cu64) }
        0x10286b7c => { vm.decode(0x54000111u32); let d = 22i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10286b80u64) }
        0x10286b80 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286b84u64) }
        0x10286b84 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286b88u64) }
        0x10286b88 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10286b8cu64 + 4);
        vm.wr(37, 0x10286b8cu64 + 4);
        Ok(0x10286b8cu64) }
        0x10286b8c => { vm.decode(0xb0d875u32); let d = 5i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10286b90u64) }
        0x10286b90 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10286b94u64) }
        0x10286b94 => { vm.decode(0x28c40b35u32); let d = 4i16; let s = 3i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10286b98u64) }
        0x10286b98 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10286b9cu64) }
        0x10286b9c => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10286ba0u64) }
        0x10286ba0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10286ba4u64).wrapping_add((imm * 4) as u64) } else { 0x10286ba4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286ba4u64) }
        0x10286ba4 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10286ba8u64) }
        0x10286ba8 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10286bacu64) }
        0x10286bac => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10286bb0u64) }
        0x10286bb0 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10286bb4u64) }
        0x10286bb4 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10286bb8u64) }
        0x10286bb8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10286bbcu64).wrapping_add((imm * 4) as u64) } else { 0x10286bbcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286bbcu64) }
        0x10286bbc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10286bc0u64) } }
        0x10286bc0 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10286bc4u64) }
        0x10286bc4 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10286bc8u64) }
        0x10286bc8 => { vm.decode(0x731cbca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -45i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10286bccu64).wrapping_add((imm * 4) as u64) } else { 0x10286bccu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286bccu64) }
        0x10286bcc => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10286bd0u64) }
        0x10286bd0 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10286bd4u64) }
        0x10286bd4 => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10286bd8u64) }
        0x10286bd8 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10286bdcu64) }
        0x10286bdc => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10286be0u64) }
        0x10286be0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10286be4u64).wrapping_add((imm * 4) as u64) } else { 0x10286be4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286be4u64) }
        0x10286be4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10286be8u64) } }
        0x10286be8 => { vm.decode(0x8c80141du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 1944i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10286becu64) }
        0x10286bec => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10286bf0u64) } }
        0x10286bf0 => { vm.decode(0x24020446u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 6i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10286bf4u64) }
        0x10286bf4 => { vm.decode(0x50040aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 458i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10286bf8u64).wrapping_add((imm * 4) as u64) } else { 0x10286bf8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286bf8u64) }
        0x10286bf8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10286bfcu64) } }
        0x10286bfc => { vm.decode(0x24100446u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10286c00u64) }
        0x10286c00 => { vm.decode(0x940227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1036i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10286c04u64).wrapping_add((imm * 4) as u64) } else { 0x10286c04u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286c04u64) }
        0x10286c04 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10286c08u64) } }
        0x10286c08 => { vm.decode(0x2a84632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286c0cu64) }
        0x10286c0c => { vm.decode(0x8c432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286c10u64) }
        0x10286c10 => { vm.decode(0x824404f5u32); let d = 17i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286c14u64) }
        0x10286c14 => { vm.decode(0x208043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286c18u64) }
        0x10286c18 => { vm.decode(0x4c008191u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10286c1cu64) }
        0x10286c1c => { vm.decode(0x200092fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286c20u64) }
        0x10286c20 => { vm.decode(0x42004b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10286c24u64) }
        0x10286c24 => { vm.decode(0x1099u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286c28u64) }
        0x10286c28 => { vm.decode(0x660d1059u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286c2cu64) }
        0x10286c2c => { vm.decode(0x9119u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286c30u64) }
        0x10286c30 => { vm.decode(0x424024f5u32); let d = 1i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286c34u64) }
        0x10286c34 => { vm.decode(0x801099u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286c38u64) }
        0x10286c38 => { vm.decode(0xadfb8df1u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = -3i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10286c3cu64).wrapping_add((imm * 4) as u64) } else { 0x10286c3cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286c3cu64) }
        0x10286c3c => { vm.decode(0x833u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10286c40u64) }
        0x10286c40 => { vm.decode(0xb281c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286c44u64) }
        0x10286c44 => { vm.decode(0x41119u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 72i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286c48u64) }
        0x10286c48 => { vm.decode(0x109119u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 96i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286c4cu64) }
        0x10286c4c => { vm.decode(0x28643efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286c50u64) }
        0x10286c50 => { vm.decode(0x424024f5u32); let d = 1i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286c54u64) }
        0x10286c54 => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286c58u64) }
        0x10286c58 => { vm.decode(0xadfb8df1u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = -3i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10286c5cu64).wrapping_add((imm * 4) as u64) } else { 0x10286c5cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286c5cu64) }
        0x10286c5c => { vm.decode(0x430u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286c60u64) }
        0x10286c60 => { vm.decode(0x9209470u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286c64u64) }
        0x10286c64 => { vm.decode(0x43657022u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10286c68u64) }
        0x10286c68 => { vm.decode(0xb301cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286c6cu64) }
        0x10286c6c => { vm.decode(0x501119u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 112i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286c70u64) }
        0x10286c70 => { vm.decode(0x169019u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 168i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286c74u64) }
        0x10286c74 => { vm.decode(0x424024f5u32); let d = 1i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286c78u64) }
        0x10286c78 => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286c7cu64) }
        0x10286c7c => { vm.decode(0xadfb8df1u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = -3i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10286c80u64).wrapping_add((imm * 4) as u64) } else { 0x10286c80u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286c80u64) }
        0x10286c80 => { vm.decode(0x430u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286c84u64) }
        0x10286c84 => { vm.decode(0xb289cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286c88u64) }
        0x10286c88 => { vm.decode(0xb209cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286c8cu64) }
        0x10286c8c => { vm.decode(0xb389c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286c90u64) }
        0x10286c90 => { vm.decode(0xb309c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286c94u64) }
        0x10286c94 => { vm.decode(0xb289c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286c98u64) }
        0x10286c98 => { vm.decode(0xb209c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286c9cu64) }
        0x10286c9c => { vm.decode(0xb189cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286ca0u64) }
        0x10286ca0 => { vm.decode(0xb109cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286ca4u64) }
        0x10286ca4 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286ca8u64) }
        0x10286ca8 => { vm.decode(0x2004216fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286cacu64) }
        0x10286cac => { vm.decode(0x931dd30u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 2512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286cb0u64) }
        0x10286cb0 => { vm.decode(0x19299c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286cb4u64) }
        0x10286cb4 => { vm.decode(0x620f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2504i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286cb8u64) }
        0x10286cb8 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286cbcu64) }
        0x10286cbc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10286cc0u64 + 4);
        vm.wr(37, 0x10286cc0u64 + 4);
        Ok(0x10286cc0u64) }
        0x10286cc0 => { vm.decode(0x938d530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286cc4u64) }
        0x10286cc4 => { vm.decode(0x208063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286cc8u64) }
        0x10286cc8 => { vm.decode(0x39281430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286cccu64) }
        0x10286ccc => { vm.decode(0x2a80636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286cd0u64) }
        0x10286cd0 => { vm.decode(0x50161119u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = 232i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286cd4u64) }
        0x10286cd4 => { vm.decode(0x102236u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10286cd8u64) }
        0x10286cd8 => { vm.decode(0x12000119u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286cdcu64) }
        0x10286cdc => { vm.decode(0x30d14458u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 37167i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10286ce0u64) }
        0x10286ce0 => { vm.decode(0x422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10286ce4u64) }
        0x10286ce4 => { vm.decode(0x1b089430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286ce8u64) }
        0x10286ce8 => { vm.decode(0x5a54eb19u32); let d = 21i16; let s = 1i16; let s2 = -1i16; let imm = -3972i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286cecu64) }
        0x10286cec => { vm.decode(0x760d9419u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286cf0u64) }
        0x10286cf0 => { vm.decode(0x26120506u32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 11i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10286cf4u64) }
        0x10286cf4 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10286cf8u64).wrapping_add((imm * 4) as u64) } else { 0x10286cf8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286cf8u64) }
        0x10286cf8 => { vm.decode(0x9e767fd9u32); let d = 22i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286cfcu64) }
        0x10286d04 => { vm.decode(0x2045b835u32); let d = 1i16; let s = 7i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10286d08u64) }
        0x10286d08 => { vm.decode(0x44001ff5u32); let d = 4i16; let s = 4i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_shl(vm.ld(s) as u32);
        vm.st(d, v);
        Ok(0x10286d0cu64) }
        0x10286d0c => { vm.decode(0x120207f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10286d10u64) }
        0x10286d10 => { vm.decode(0x80350175u32); let d = 3i16; let s = 3i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_shr(vm.ld(s) as u32);
        vm.st(d, v);
        Ok(0x10286d14u64) }
        0x10286d14 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10286d18u64) }
        0x10286d18 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10286d1cu64) }
        0x10286d1c => { vm.decode(0x4b2114f5u32); let d = 2i16; let s = 18i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286d20u64) }
        0x10286d20 => { vm.decode(0x10400030u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286d24u64) }
        0x10286d24 => { vm.decode(0x8b03236u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10286d28u64) }
        0x10286d28 => { vm.decode(0x26120506u32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 11i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10286d2cu64) }
        0x10286d2c => { vm.decode(0x41c8227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1081i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10286d30u64).wrapping_add((imm * 4) as u64) } else { 0x10286d30u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286d30u64) }
        0x10286d30 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10286d34u64) } }
        0x10286d34 => { vm.decode(0x40830bb5u32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10286d38u64) }
        0x10286d38 => { vm.decode(0xe008bd75u32); let d = 30i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10286d3cu64) }
        0x10286d3c => { vm.decode(0x616114f5u32); let d = 1i16; let s = 17i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286d40u64) }
        0x10286d40 => { vm.decode(0x4000012fu32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286d44u64) }
        0x10286d44 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286d48u64) }
        0x10286d48 => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286d4cu64) }
        0x10286d4c => { vm.decode(0x8b195430u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286d50u64) }
        0x10286d50 => { vm.decode(0x88736fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286d54u64) }
        0x10286d54 => { vm.decode(0x8b2004f5u32); let d = 2i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286d58u64) }
        0x10286d58 => { vm.decode(0xb111530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286d5cu64) }
        0x10286d5c => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286d60u64) }
        0x10286d60 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286d64u64) }
        0x10286d64 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286d68u64) }
        0x10286d68 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10286d6cu64 + 4);
        vm.wr(37, 0x10286d6cu64 + 4);
        Ok(0x10286d6cu64) }
        0x10286d6c => { vm.decode(0x490d875u32); let d = 5i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10286d70u64) }
        0x10286d70 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10286d74u64) }
        0x10286d74 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10286d78u64) }
        0x10286d78 => { vm.decode(0x830bb5u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10286d7cu64) }
        0x10286d7c => { vm.decode(0x89020b75u32); let d = 4i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_sub(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286d80u64) }
        0x10286d80 => { vm.decode(0x260bb5u32); let d = 5i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10286d84u64) }
        0x10286d84 => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10286d88u64) }
        0x10286d88 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10286d8cu64).wrapping_add((imm * 4) as u64) } else { 0x10286d8cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286d8cu64) }
        0x10286d8c => { vm.decode(0x1019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286d90u64) }
        0x10286d90 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10286d94u64) }
        0x10286d94 => { vm.decode(0x22400d75u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10286d98u64) }
        0x10286d98 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10286d9cu64) }
        0x10286d9c => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10286da0u64) }
        0x10286da0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10286da4u64).wrapping_add((imm * 4) as u64) } else { 0x10286da4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286da4u64) }
        0x10286da4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10286da8u64) } }
        0x10286da8 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10286dacu64) }
        0x10286dac => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10286db0u64) }
        0x10286db0 => { vm.decode(0x739c34a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -44i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10286db4u64).wrapping_add((imm * 4) as u64) } else { 0x10286db4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286db4u64) }
        0x10286db4 => { vm.decode(0x2009019u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286db8u64) }
        0x10286db8 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10286dbcu64) }
        0x10286dbc => { vm.decode(0x32402d75u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10286dc0u64) }
        0x10286dc0 => { vm.decode(0xa0204118u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10286dc4u64) }
        0x10286dc4 => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10286dc8u64) }
        0x10286dc8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10286dccu64).wrapping_add((imm * 4) as u64) } else { 0x10286dccu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286dccu64) }
        0x10286dcc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10286dd0u64) } }
        0x10286dd0 => { vm.decode(0xa000501du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 2436i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10286dd4u64) }
        0x10286dd4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10286dd8u64) } }
        0x10286dd8 => { vm.decode(0x762d8019u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286ddcu64) }
        0x10286ddc => { vm.decode(0x24020546u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 14i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10286de0u64) }
        0x10286de0 => { vm.decode(0x440482a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 425i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10286de4u64).wrapping_add((imm * 4) as u64) } else { 0x10286de4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286de4u64) }
        0x10286de4 => { vm.decode(0x9938d430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286de8u64) }
        0x10286de8 => { vm.decode(0x24100546u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 13i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10286decu64) }
        0x10286dec => { vm.decode(0x10940227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1100i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10286df0u64).wrapping_add((imm * 4) as u64) } else { 0x10286df0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286df0u64) }
        0x10286df0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10286df4u64) } }
        0x10286df4 => { vm.decode(0x2a84632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286df8u64) }
        0x10286df8 => { vm.decode(0xa80024f5u32); let d = 2i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286dfcu64) }
        0x10286dfc => { vm.decode(0xb101530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286e00u64) }
        0x10286e00 => { vm.decode(0xaa01a4f5u32); let d = 2i16; let s = 1i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286e04u64) }
        0x10286e04 => { vm.decode(0xb3815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286e08u64) }
        0x10286e08 => { vm.decode(0x824124f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286e0cu64) }
        0x10286e0c => { vm.decode(0x8843efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286e10u64) }
        0x10286e10 => { vm.decode(0x10408191u32); let d = 20i16; let s = 2i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10286e14u64) }
        0x10286e14 => { vm.decode(0x228002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286e18u64) }
        0x10286e18 => { vm.decode(0xb009530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286e1cu64) }
        0x10286e1c => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286e20u64) }
        0x10286e20 => { vm.decode(0x82c002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286e24u64) }
        0x10286e24 => { vm.decode(0x10108019u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286e28u64) }
        0x10286e28 => { vm.decode(0x9b211430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286e2cu64) }
        0x10286e2c => { vm.decode(0x1b1914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286e30u64) }
        0x10286e30 => { vm.decode(0x288063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286e34u64) }
        0x10286e34 => { vm.decode(0x1b1114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286e38u64) }
        0x10286e38 => { vm.decode(0xb0915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286e3cu64) }
        0x10286e3c => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286e40u64) }
        0x10286e40 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286e44u64) }
        0x10286e44 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10286e48u64 + 4);
        vm.wr(37, 0x10286e48u64 + 4);
        Ok(0x10286e48u64) }
        0x10286e48 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286e4cu64) }
        0x10286e4c => { vm.decode(0x100236u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10286e50u64) }
        0x10286e50 => { vm.decode(0x23516818u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 43129i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10286e54u64) }
        0x10286e54 => { vm.decode(0x422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10286e58u64) }
        0x10286e58 => { vm.decode(0x1b201430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286e5cu64) }
        0x10286e5c => { vm.decode(0xc006a019u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 142i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286e60u64) }
        0x10286e60 => { vm.decode(0x8863efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286e64u64) }
        0x10286e64 => { vm.decode(0x824604f5u32); let d = 21i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286e68u64) }
        0x10286e68 => { vm.decode(0x760d1419u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286e6cu64) }
        0x10286e6c => { vm.decode(0x6102236u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10286e70u64) }
        0x10286e70 => { vm.decode(0x6000506u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10286e74u64) }
        0x10286e74 => { vm.decode(0x2200031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10286e78u64).wrapping_add((imm * 4) as u64) } else { 0x10286e78u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286e78u64) }
        0x10286e78 => { vm.decode(0x70499519u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286e7cu64) }
        0x10286e7c => { vm.decode(0x2d24501du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 7532i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10286e80u64) }
        0x10286e84 => { vm.decode(0x8024735u32); let d = 1i16; let s = 4i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10286e88u64) }
        0x10286e88 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x10286e8cu64) }
        0x10286e8c => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10286e90u64) }
        0x10286e90 => { vm.decode(0x64104675u32); let d = 3i16; let s = 3i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10286e94u64) }
        0x10286e94 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10286e98u64) }
        0x10286e98 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10286e9cu64) }
        0x10286e9c => { vm.decode(0x6b2114f5u32); let d = 2i16; let s = 19i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286ea0u64) }
        0x10286ea0 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10286ea4u64) }
        0x10286ea4 => { vm.decode(0x8b00236u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10286ea8u64) }
        0x10286ea8 => { vm.decode(0x408e6befu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286eacu64) }
        0x10286eac => { vm.decode(0x6000506u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10286eb0u64) }
        0x10286eb0 => { vm.decode(0x940aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1166i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10286eb4u64).wrapping_add((imm * 4) as u64) } else { 0x10286eb4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286eb4u64) }
        0x10286eb4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10286eb8u64) } }
        0x10286eb8 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286ebcu64) }
        0x10286ebc => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286ec0u64) }
        0x10286ec0 => { vm.decode(0x9b191430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286ec4u64) }
        0x10286ec4 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286ec8u64) }
        0x10286ec8 => { vm.decode(0x9b115430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286eccu64) }
        0x10286ecc => { vm.decode(0x40030bb5u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10286ed0u64) }
        0x10286ed0 => { vm.decode(0xc008bd75u32); let d = 30i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10286ed4u64) }
        0x10286ed4 => { vm.decode(0x288063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286ed8u64) }
        0x10286ed8 => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286edcu64) }
        0x10286edc => { vm.decode(0x14000011u32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10286ee0u64) }
        0x10286ee0 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286ee4u64) }
        0x10286ee4 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286ee8u64) }
        0x10286ee8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10286eecu64 + 4);
        vm.wr(37, 0x10286eecu64 + 4);
        Ok(0x10286eecu64) }
        0x10286eec => { vm.decode(0x90d875u32); let d = 5i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10286ef0u64) }
        0x10286ef0 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10286ef4u64) }
        0x10286ef4 => { vm.decode(0x28cc8b35u32); let d = 4i16; let s = 3i16; let s2 = 23i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10286ef8u64) }
        0x10286ef8 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10286efcu64) }
        0x10286efc => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10286f00u64) }
        0x10286f00 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10286f04u64).wrapping_add((imm * 4) as u64) } else { 0x10286f04u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286f04u64) }
        0x10286f04 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10286f08u64) }
        0x10286f08 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10286f0cu64) }
        0x10286f0c => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10286f10u64) }
        0x10286f10 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10286f14u64) }
        0x10286f14 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10286f18u64) }
        0x10286f18 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10286f1cu64).wrapping_add((imm * 4) as u64) } else { 0x10286f1cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286f1cu64) }
        0x10286f1c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10286f20u64) } }
        0x10286f20 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10286f24u64) }
        0x10286f24 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10286f28u64) }
        0x10286f28 => { vm.decode(0x739c3ca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -42i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10286f2cu64).wrapping_add((imm * 4) as u64) } else { 0x10286f2cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286f2cu64) }
        0x10286f2c => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10286f30u64) }
        0x10286f30 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10286f34u64) }
        0x10286f34 => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10286f38u64) }
        0x10286f38 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10286f3cu64) }
        0x10286f3c => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10286f40u64) }
        0x10286f40 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10286f44u64).wrapping_add((imm * 4) as u64) } else { 0x10286f44u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286f44u64) }
        0x10286f44 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10286f48u64) } }
        0x10286f48 => { vm.decode(0x2080501du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 2820i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10286f4cu64) }
        0x10286f4c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10286f50u64) } }
        0x10286f50 => { vm.decode(0x24120406u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 3i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10286f54u64) }
        0x10286f54 => { vm.decode(0x49c02a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1212i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10286f58u64).wrapping_add((imm * 4) as u64) } else { 0x10286f58u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10286f58u64) }
        0x10286f58 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10286f5cu64) } }
        0x10286f5c => { vm.decode(0x2a84632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286f60u64) }
        0x10286f60 => { vm.decode(0x884184f5u32); let d = 3i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286f64u64) }
        0x10286f64 => { vm.decode(0x19309530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286f68u64) }
        0x10286f68 => { vm.decode(0xa840a4f5u32); let d = 3i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286f6cu64) }
        0x10286f6c => { vm.decode(0x19209530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286f70u64) }
        0x10286f70 => { vm.decode(0x8840a4f5u32); let d = 3i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286f74u64) }
        0x10286f74 => { vm.decode(0x19189db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286f78u64) }
        0x10286f78 => { vm.decode(0xaa4084f5u32); let d = 3i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286f7cu64) }
        0x10286f7c => { vm.decode(0x19109db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286f80u64) }
        0x10286f80 => { vm.decode(0x8a4084f5u32); let d = 3i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286f84u64) }
        0x10286f84 => { vm.decode(0x19089db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286f88u64) }
        0x10286f88 => { vm.decode(0x884084f5u32); let d = 3i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286f8cu64) }
        0x10286f8c => { vm.decode(0x19009db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286f90u64) }
        0x10286f90 => { vm.decode(0x824424f5u32); let d = 17i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10286f94u64) }
        0x10286f94 => { vm.decode(0x208043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286f98u64) }
        0x10286f98 => { vm.decode(0x620a012fu32); let d = 23i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286f9cu64) }
        0x10286f9c => { vm.decode(0x14008091u32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10286fa0u64) }
        0x10286fa0 => { vm.decode(0x4a8263afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286fa4u64) }
        0x10286fa4 => { vm.decode(0x2a80402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286fa8u64) }
        0x10286fa8 => { vm.decode(0x109119u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 96i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286facu64) }
        0x10286fac => { vm.decode(0x19099d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286fb0u64) }
        0x10286fb0 => { vm.decode(0x640d9059u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286fb4u64) }
        0x10286fb4 => { vm.decode(0x19389530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286fb8u64) }
        0x10286fb8 => { vm.decode(0x19391db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286fbcu64) }
        0x10286fbc => { vm.decode(0x19311cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286fc0u64) }
        0x10286fc0 => { vm.decode(0xa251062u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10286fc4u64) }
        0x10286fc4 => { vm.decode(0x8643efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286fc8u64) }
        0x10286fc8 => { vm.decode(0x688c43afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286fccu64) }
        0x10286fcc => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10286fd0u64) }
        0x10286fd0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10286fd4u64 + 4);
        vm.wr(37, 0x10286fd4u64 + 4);
        Ok(0x10286fd4u64) }
        0x10286fd4 => { vm.decode(0x62598559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2416i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286fd8u64) }
        0x10286fd8 => { vm.decode(0x762d8019u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286fdcu64) }
        0x10286fdc => { vm.decode(0x10401019u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10286fe0u64) }
        0x10286fe0 => { vm.decode(0x208063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286fe4u64) }
        0x10286fe4 => { vm.decode(0x1b201cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286fe8u64) }
        0x10286fe8 => { vm.decode(0x2884402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10286fecu64) }
        0x10286fec => { vm.decode(0x8b211530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286ff0u64) }
        0x10286ff0 => { vm.decode(0x9b1914b0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286ff4u64) }
        0x10286ff4 => { vm.decode(0x9b1154b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286ff8u64) }
        0x10286ff8 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10286ffcu64) }
        0x10286ffc => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287000u64) }
        0x10287000 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10287004u64) }
        0x10287004 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10287008u64 + 4);
        vm.wr(37, 0x10287008u64 + 4);
        Ok(0x10287008u64) }
        0x10287008 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028700cu64) }
        0x1028700c => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287010u64) }
        0x10287010 => { vm.decode(0x2280406fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287014u64) }
        0x10287014 => { vm.decode(0x8b2115b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287018u64) }
        0x10287018 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028701cu64) }
        0x1028701c => { vm.decode(0x8b315570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287020u64) }
        0x10287020 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287024u64) }
        0x10287024 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287028u64) }
        0x10287028 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028702cu64) }
        0x1028702c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10287030u64 + 4);
        vm.wr(37, 0x10287030u64 + 4);
        Ok(0x10287030u64) }
        0x10287030 => { vm.decode(0x102036u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287034u64) }
        0x10287034 => { vm.decode(0x40020bb5u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10287038u64) }
        0x10287038 => { vm.decode(0x19209c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028703cu64) }
        0x1028703c => { vm.decode(0x30596098u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 36013i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10287040u64) }
        0x10287040 => { vm.decode(0x422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10287044u64) }
        0x10287044 => { vm.decode(0x1b3014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287048u64) }
        0x10287048 => { vm.decode(0x50567359u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = 12796i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028704cu64) }
        0x1028704c => { vm.decode(0x10109019u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287050u64) }
        0x10287050 => { vm.decode(0x770d1419u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287054u64) }
        0x10287054 => { vm.decode(0x6100036u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287058u64) }
        0x10287058 => { vm.decode(0x24000446u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028705cu64) }
        0x1028705c => { vm.decode(0x2220031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 17i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10287060u64).wrapping_add((imm * 4) as u64) } else { 0x10287060u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287060u64) }
        0x10287060 => { vm.decode(0x9b305430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287064u64) }
        0x1028706c => { vm.decode(0x8624735u32); let d = 1i16; let s = 4i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10287070u64) }
        0x10287070 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x10287074u64) }
        0x10287074 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10287078u64) }
        0x10287078 => { vm.decode(0x74304675u32); let d = 3i16; let s = 3i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028707cu64) }
        0x1028707c => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10287080u64) }
        0x10287080 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10287084u64) }
        0x10287084 => { vm.decode(0x8c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287088u64) }
        0x10287088 => { vm.decode(0x8b2004f5u32); let d = 2i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028708cu64) }
        0x1028708c => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10287090u64) }
        0x10287090 => { vm.decode(0x8903036u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287094u64) }
        0x10287094 => { vm.decode(0x628243efu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287098u64) }
        0x10287098 => { vm.decode(0x24000446u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028709cu64) }
        0x1028709c => { vm.decode(0x101480a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1217i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102870a0u64).wrapping_add((imm * 4) as u64) } else { 0x102870a0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102870a0u64) }
        0x102870a0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102870a4u64) } }
        0x102870a4 => { vm.decode(0x2084c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102870a8u64) }
        0x102870a8 => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102870acu64) }
        0x102870ac => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102870b0u64) }
        0x102870b0 => { vm.decode(0x82c202fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102870b4u64) }
        0x102870b4 => { vm.decode(0x9b191530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102870b8u64) }
        0x102870b8 => { vm.decode(0xb091530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102870bcu64) }
        0x102870bc => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102870c0u64) }
        0x102870c0 => { vm.decode(0x40820bb5u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102870c4u64) }
        0x102870c4 => { vm.decode(0x44104875u32); let d = 2i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102870c8u64) }
        0x102870c8 => { vm.decode(0xc040ad75u32); let d = 21i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102870ccu64) }
        0x102870cc => { vm.decode(0x836004f5u32); let d = 1i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102870d0u64) }
        0x102870d0 => { vm.decode(0x54000111u32); let d = 22i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102870d4u64) }
        0x102870d4 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102870d8u64) }
        0x102870d8 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102870dcu64) }
        0x102870dc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102870e0u64 + 4);
        vm.wr(37, 0x102870e0u64 + 4);
        Ok(0x102870e0u64) }
        0x102870e0 => { vm.decode(0xb0d875u32); let d = 5i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102870e4u64) }
        0x102870e4 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102870e8u64) }
        0x102870e8 => { vm.decode(0x28c40b35u32); let d = 4i16; let s = 3i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102870ecu64) }
        0x102870ec => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102870f0u64) }
        0x102870f0 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102870f4u64) }
        0x102870f4 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102870f8u64).wrapping_add((imm * 4) as u64) } else { 0x102870f8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102870f8u64) }
        0x102870f8 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102870fcu64) }
        0x102870fc => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287100u64) }
        0x10287100 => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10287104u64) }
        0x10287104 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10287108u64) }
        0x10287108 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1028710cu64) }
        0x1028710c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10287110u64).wrapping_add((imm * 4) as u64) } else { 0x10287110u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287110u64) }
        0x10287110 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10287114u64) } }
        0x10287114 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287118u64) }
        0x10287118 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1028711cu64) }
        0x1028711c => { vm.decode(0x731cbca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -45i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10287120u64).wrapping_add((imm * 4) as u64) } else { 0x10287120u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287120u64) }
        0x10287120 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287124u64) }
        0x10287124 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287128u64) }
        0x10287128 => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028712cu64) }
        0x1028712c => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10287130u64) }
        0x10287130 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10287134u64) }
        0x10287134 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10287138u64).wrapping_add((imm * 4) as u64) } else { 0x10287138u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287138u64) }
        0x10287138 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028713cu64) } }
        0x1028713c => { vm.decode(0xad20401du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 3308i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10287140u64) }
        0x10287140 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10287144u64) } }
        0x10287144 => { vm.decode(0x24120506u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 11i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10287148u64) }
        0x10287148 => { vm.decode(0x50940027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1348i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028714cu64).wrapping_add((imm * 4) as u64) } else { 0x1028714cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028714cu64) }
        0x1028714c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10287150u64) } }
        0x10287150 => { vm.decode(0x2a84632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287154u64) }
        0x10287154 => { vm.decode(0x28843efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287158u64) }
        0x10287158 => { vm.decode(0x824404f5u32); let d = 17i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028715cu64) }
        0x1028715c => { vm.decode(0x208043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287160u64) }
        0x10287160 => { vm.decode(0x4c008191u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10287164u64) }
        0x10287164 => { vm.decode(0x200092fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287168u64) }
        0x10287168 => { vm.decode(0x42004b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028716cu64) }
        0x1028716c => { vm.decode(0x1099u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287170u64) }
        0x10287170 => { vm.decode(0x660d1059u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287174u64) }
        0x10287174 => { vm.decode(0x9119u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287178u64) }
        0x10287178 => { vm.decode(0x424024f5u32); let d = 1i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028717cu64) }
        0x1028717c => { vm.decode(0x801099u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287180u64) }
        0x10287180 => { vm.decode(0xadfb8df1u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = -3i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10287184u64).wrapping_add((imm * 4) as u64) } else { 0x10287184u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287184u64) }
        0x10287184 => { vm.decode(0x833u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10287188u64) }
        0x10287188 => { vm.decode(0xb281c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028718cu64) }
        0x1028718c => { vm.decode(0x41119u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 72i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287190u64) }
        0x10287190 => { vm.decode(0x109119u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 96i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287194u64) }
        0x10287194 => { vm.decode(0x22844befu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287198u64) }
        0x10287198 => { vm.decode(0x424024f5u32); let d = 1i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028719cu64) }
        0x1028719c => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102871a0u64) }
        0x102871a0 => { vm.decode(0xadfb8df1u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = -3i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102871a4u64).wrapping_add((imm * 4) as u64) } else { 0x102871a4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102871a4u64) }
        0x102871a4 => { vm.decode(0x430u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102871a8u64) }
        0x102871a8 => { vm.decode(0x9209470u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102871acu64) }
        0x102871ac => { vm.decode(0x43657022u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102871b0u64) }
        0x102871b0 => { vm.decode(0xb301cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102871b4u64) }
        0x102871b4 => { vm.decode(0x501119u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 112i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102871b8u64) }
        0x102871b8 => { vm.decode(0x169019u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 168i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102871bcu64) }
        0x102871bc => { vm.decode(0x424024f5u32); let d = 1i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102871c0u64) }
        0x102871c0 => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102871c4u64) }
        0x102871c4 => { vm.decode(0xadfb8df1u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = -3i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102871c8u64).wrapping_add((imm * 4) as u64) } else { 0x102871c8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102871c8u64) }
        0x102871c8 => { vm.decode(0x430u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102871ccu64) }
        0x102871cc => { vm.decode(0xb289cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102871d0u64) }
        0x102871d0 => { vm.decode(0xb209cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102871d4u64) }
        0x102871d4 => { vm.decode(0xb389c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102871d8u64) }
        0x102871d8 => { vm.decode(0xb309c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102871dcu64) }
        0x102871dc => { vm.decode(0xb289c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102871e0u64) }
        0x102871e0 => { vm.decode(0xb209c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102871e4u64) }
        0x102871e4 => { vm.decode(0xb189cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102871e8u64) }
        0x102871e8 => { vm.decode(0xb109cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102871ecu64) }
        0x102871ec => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102871f0u64) }
        0x102871f0 => { vm.decode(0x2204216fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102871f4u64) }
        0x102871f4 => { vm.decode(0x931dd30u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 2512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102871f8u64) }
        0x102871f8 => { vm.decode(0x19299c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102871fcu64) }
        0x102871fc => { vm.decode(0x620f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2504i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287200u64) }
        0x10287200 => { vm.decode(0x4a00875u32); let d = 4i16; let s = 0i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10287204u64) }
        0x10287204 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287208u64) }
        0x10287208 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028720cu64 + 4);
        vm.wr(37, 0x1028720cu64 + 4);
        Ok(0x1028720cu64) }
        0x1028720c => { vm.decode(0x938d530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287210u64) }
        0x10287210 => { vm.decode(0x208063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287214u64) }
        0x10287214 => { vm.decode(0x39281430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287218u64) }
        0x10287218 => { vm.decode(0x2a80636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028721cu64) }
        0x1028721c => { vm.decode(0x50160119u32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = 232i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287220u64) }
        0x10287220 => { vm.decode(0x100236u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287224u64) }
        0x10287224 => { vm.decode(0x10009119u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287228u64) }
        0x10287228 => { vm.decode(0x8056e0d9u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 8639i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028722cu64) }
        0x1028722c => { vm.decode(0x80422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 18i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10287230u64) }
        0x10287230 => { vm.decode(0xde564fd9u32); let d = 20i16; let s = 1i16; let s2 = -1i16; let imm = -1029i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287234u64) }
        0x10287234 => { vm.decode(0x760d9419u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287238u64) }
        0x10287238 => { vm.decode(0x6120506u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 11i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028723cu64) }
        0x1028723c => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10287240u64).wrapping_add((imm * 4) as u64) } else { 0x10287240u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287240u64) }
        0x10287240 => { vm.decode(0x9e76efd9u32); let d = 21i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287244u64) }
        0x10287244 => { vm.decode(0xa420011du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 9412i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10287248u64) }
        0x1028724c => { vm.decode(0x6041b835u32); let d = 1i16; let s = 7i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10287250u64) }
        0x10287250 => { vm.decode(0x4e081ff5u32); let d = 4i16; let s = 30i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shl(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x10287254u64) }
        0x10287254 => { vm.decode(0x120207f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10287258u64) }
        0x10287258 => { vm.decode(0x94352175u32); let d = 3i16; let s = 30i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shr(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x1028725cu64) }
        0x1028725c => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10287260u64) }
        0x10287260 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10287264u64) }
        0x10287264 => { vm.decode(0x492104f5u32); let d = 2i16; let s = 16i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10287268u64) }
        0x10287268 => { vm.decode(0x10400030u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028726cu64) }
        0x1028726c => { vm.decode(0x8b00236u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287270u64) }
        0x10287270 => { vm.decode(0x6120506u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 11i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10287274u64) }
        0x10287274 => { vm.decode(0x541c8827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1395i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10287278u64).wrapping_add((imm * 4) as u64) } else { 0x10287278u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287278u64) }
        0x10287278 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028727cu64) } }
        0x1028727c => { vm.decode(0x40030bb5u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10287280u64) }
        0x10287280 => { vm.decode(0xe000bd75u32); let d = 22i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10287284u64) }
        0x10287284 => { vm.decode(0x616104f5u32); let d = 1i16; let s = 17i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10287288u64) }
        0x10287288 => { vm.decode(0x400a092fu32); let d = 30i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028728cu64) }
        0x1028728c => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287290u64) }
        0x10287290 => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287294u64) }
        0x10287294 => { vm.decode(0x9b191530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287298u64) }
        0x10287298 => { vm.decode(0x88736fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028729cu64) }
        0x1028729c => { vm.decode(0xb2004f5u32); let d = 2i16; let s = 2i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102872a0u64) }
        0x102872a0 => { vm.decode(0xb111530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102872a4u64) }
        0x102872a4 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102872a8u64) }
        0x102872a8 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102872acu64) }
        0x102872ac => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102872b0u64) }
        0x102872b0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102872b4u64 + 4);
        vm.wr(37, 0x102872b4u64 + 4);
        Ok(0x102872b4u64) }
        0x102872b4 => { vm.decode(0x490d875u32); let d = 5i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102872b8u64) }
        0x102872b8 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102872bcu64) }
        0x102872bc => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102872c0u64) }
        0x102872c0 => { vm.decode(0x830bb5u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102872c4u64) }
        0x102872c4 => { vm.decode(0x92a0b75u32); let d = 4i16; let s = 19i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_sub(vm.ld(s2));
        vm.st(d, v);
        Ok(0x102872c8u64) }
        0x102872c8 => { vm.decode(0x260bb5u32); let d = 5i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102872ccu64) }
        0x102872cc => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102872d0u64) }
        0x102872d0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102872d4u64).wrapping_add((imm * 4) as u64) } else { 0x102872d4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102872d4u64) }
        0x102872d4 => { vm.decode(0x1019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102872d8u64) }
        0x102872d8 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102872dcu64) }
        0x102872dc => { vm.decode(0x22400d75u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102872e0u64) }
        0x102872e0 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102872e4u64) }
        0x102872e4 => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102872e8u64) }
        0x102872e8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102872ecu64).wrapping_add((imm * 4) as u64) } else { 0x102872ecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102872ecu64) }
        0x102872ec => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102872f0u64) } }
        0x102872f0 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102872f4u64) }
        0x102872f4 => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102872f8u64) }
        0x102872f8 => { vm.decode(0x739c34a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -44i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102872fcu64).wrapping_add((imm * 4) as u64) } else { 0x102872fcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102872fcu64) }
        0x102872fc => { vm.decode(0x2009019u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287300u64) }
        0x10287300 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287304u64) }
        0x10287304 => { vm.decode(0x32402d75u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10287308u64) }
        0x10287308 => { vm.decode(0xa0204118u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x1028730cu64) }
        0x1028730c => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10287310u64) }
        0x10287310 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10287314u64).wrapping_add((imm * 4) as u64) } else { 0x10287314u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287314u64) }
        0x10287314 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10287318u64) } }
        0x10287318 => { vm.decode(0xaca0401du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 3788i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1028731cu64) }
        0x1028731c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10287320u64) } }
        0x10287320 => { vm.decode(0x24120446u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 7i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10287324u64) }
        0x10287324 => { vm.decode(0x40148aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1419i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10287328u64).wrapping_add((imm * 4) as u64) } else { 0x10287328u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287328u64) }
        0x10287328 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028732cu64) } }
        0x1028732c => { vm.decode(0x2a84632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287330u64) }
        0x10287330 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10287334u64) }
        0x10287334 => { vm.decode(0xb101530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287338u64) }
        0x10287338 => { vm.decode(0xa00624f5u32); let d = 20i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028733cu64) }
        0x1028733c => { vm.decode(0xa24124f5u32); let d = 1i16; let s = 1i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10287340u64) }
        0x10287340 => { vm.decode(0x208843efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287344u64) }
        0x10287344 => { vm.decode(0x44408091u32); let d = 2i16; let s = 3i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10287348u64) }
        0x10287348 => { vm.decode(0x22d5022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028734cu64) }
        0x1028734c => { vm.decode(0x228012fu32); let d = 2i16; let s = 3i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287350u64) }
        0x10287350 => { vm.decode(0xb3815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287354u64) }
        0x10287354 => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287358u64) }
        0x10287358 => { vm.decode(0x82c002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028735cu64) }
        0x1028735c => { vm.decode(0x10108019u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287360u64) }
        0x10287360 => { vm.decode(0x9b211430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287364u64) }
        0x10287364 => { vm.decode(0x1b1914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287368u64) }
        0x10287368 => { vm.decode(0x288063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028736cu64) }
        0x1028736c => { vm.decode(0x1b1114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287370u64) }
        0x10287370 => { vm.decode(0xb0915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287374u64) }
        0x10287374 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287378u64) }
        0x10287378 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028737cu64) }
        0x1028737c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10287380u64 + 4);
        vm.wr(37, 0x10287380u64 + 4);
        Ok(0x10287380u64) }
        0x10287380 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287384u64) }
        0x10287384 => { vm.decode(0x100236u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287388u64) }
        0x10287388 => { vm.decode(0x8424a6d9u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 7951i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028738cu64) }
        0x1028738c => { vm.decode(0xa0422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 19i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10287390u64) }
        0x10287390 => { vm.decode(0xcc12afd9u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -9241i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287394u64) }
        0x10287394 => { vm.decode(0x8863efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287398u64) }
        0x10287398 => { vm.decode(0x8a4404f5u32); let d = 19i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028739cu64) }
        0x1028739c => { vm.decode(0x760d1419u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102873a0u64) }
        0x102873a0 => { vm.decode(0x60498519u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102873a4u64) }
        0x102873a4 => { vm.decode(0x1b2014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102873a8u64) }
        0x102873a8 => { vm.decode(0x6000506u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102873acu64) }
        0x102873ac => { vm.decode(0x2200031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102873b0u64).wrapping_add((imm * 4) as u64) } else { 0x102873b0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102873b0u64) }
        0x102873b0 => { vm.decode(0x6102036u32); let d = 21i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102873b4u64) }
        0x102873bc => { vm.decode(0x8024735u32); let d = 1i16; let s = 4i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102873c0u64) }
        0x102873c0 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102873c4u64) }
        0x102873c4 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102873c8u64) }
        0x102873c8 => { vm.decode(0x64104675u32); let d = 3i16; let s = 3i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102873ccu64) }
        0x102873cc => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102873d0u64) }
        0x102873d0 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102873d4u64) }
        0x102873d4 => { vm.decode(0x8c43efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102873d8u64) }
        0x102873d8 => { vm.decode(0xcb2004f5u32); let d = 2i16; let s = 2i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102873dcu64) }
        0x102873dc => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102873e0u64) }
        0x102873e0 => { vm.decode(0x8b00236u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102873e4u64) }
        0x102873e4 => { vm.decode(0x6000506u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102873e8u64) }
        0x102873e8 => { vm.decode(0x509482a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1485i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102873ecu64).wrapping_add((imm * 4) as u64) } else { 0x102873ecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102873ecu64) }
        0x102873ec => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102873f0u64) } }
        0x102873f0 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102873f4u64) }
        0x102873f4 => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102873f8u64) }
        0x102873f8 => { vm.decode(0x9b191430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102873fcu64) }
        0x102873fc => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287400u64) }
        0x10287400 => { vm.decode(0x9b111530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287404u64) }
        0x10287404 => { vm.decode(0x40030bb5u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10287408u64) }
        0x10287408 => { vm.decode(0xc040bd75u32); let d = 23i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1028740cu64) }
        0x1028740c => { vm.decode(0x288063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287410u64) }
        0x10287410 => { vm.decode(0xe16004f5u32); let d = 1i16; let s = 1i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10287414u64) }
        0x10287414 => { vm.decode(0x14000011u32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10287418u64) }
        0x10287418 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028741cu64) }
        0x1028741c => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287420u64) }
        0x10287420 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10287424u64 + 4);
        vm.wr(37, 0x10287424u64 + 4);
        Ok(0x10287424u64) }
        0x10287424 => { vm.decode(0x90d875u32); let d = 5i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10287428u64) }
        0x10287428 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028742cu64) }
        0x1028742c => { vm.decode(0x28c48b35u32); let d = 4i16; let s = 3i16; let s2 = 21i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10287430u64) }
        0x10287430 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287434u64) }
        0x10287434 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10287438u64) }
        0x10287438 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028743cu64).wrapping_add((imm * 4) as u64) } else { 0x1028743cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028743cu64) }
        0x1028743c => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287440u64) }
        0x10287440 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287444u64) }
        0x10287444 => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10287448u64) }
        0x10287448 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x1028744cu64) }
        0x1028744c => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10287450u64) }
        0x10287450 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10287454u64).wrapping_add((imm * 4) as u64) } else { 0x10287454u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287454u64) }
        0x10287454 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10287458u64) } }
        0x10287458 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028745cu64) }
        0x1028745c => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10287460u64) }
        0x10287460 => { vm.decode(0x739c3ca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -42i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10287464u64).wrapping_add((imm * 4) as u64) } else { 0x10287464u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287464u64) }
        0x10287464 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287468u64) }
        0x10287468 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028746cu64) }
        0x1028746c => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10287470u64) }
        0x10287470 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10287474u64) }
        0x10287474 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10287478u64) }
        0x10287478 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028747cu64).wrapping_add((imm * 4) as u64) } else { 0x1028747cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028747cu64) }
        0x1028747c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10287480u64) } }
        0x10287480 => { vm.decode(0x2904041du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 4156i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10287484u64) }
        0x10287484 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10287488u64) } }
        0x10287488 => { vm.decode(0x640f8119u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028748cu64) }
        0x1028748c => { vm.decode(0x1b301430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287490u64) }
        0x10287490 => { vm.decode(0x24120546u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 15i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10287494u64) }
        0x10287494 => { vm.decode(0x541c02a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1528i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10287498u64).wrapping_add((imm * 4) as u64) } else { 0x10287498u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287498u64) }
        0x10287498 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028749cu64) } }
        0x1028749c => { vm.decode(0x2a84632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102874a0u64) }
        0x102874a0 => { vm.decode(0x28c632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102874a4u64) }
        0x102874a4 => { vm.decode(0x8a4004f5u32); let d = 3i16; let s = 2i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102874a8u64) }
        0x102874a8 => { vm.decode(0x19201530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102874acu64) }
        0x102874ac => { vm.decode(0x88636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102874b0u64) }
        0x102874b0 => { vm.decode(0x8a4004f5u32); let d = 3i16; let s = 2i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102874b4u64) }
        0x102874b4 => { vm.decode(0x19281530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102874b8u64) }
        0x102874b8 => { vm.decode(0x288436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102874bcu64) }
        0x102874bc => { vm.decode(0x8a4004f5u32); let d = 3i16; let s = 2i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102874c0u64) }
        0x102874c0 => { vm.decode(0x19189db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102874c4u64) }
        0x102874c4 => { vm.decode(0x884024f5u32); let d = 3i16; let s = 4i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102874c8u64) }
        0x102874c8 => { vm.decode(0x19109db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102874ccu64) }
        0x102874cc => { vm.decode(0x8a4084f5u32); let d = 3i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102874d0u64) }
        0x102874d0 => { vm.decode(0x19089db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102874d4u64) }
        0x102874d4 => { vm.decode(0xaa4124f5u32); let d = 3i16; let s = 23i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102874d8u64) }
        0x102874d8 => { vm.decode(0x19009db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102874dcu64) }
        0x102874dc => { vm.decode(0x804524f5u32); let d = 17i16; let s = 20i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102874e0u64) }
        0x102874e0 => { vm.decode(0x642d8159u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1864i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102874e4u64) }
        0x102874e4 => { vm.decode(0x19289c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102874e8u64) }
        0x102874e8 => { vm.decode(0x208843efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102874ecu64) }
        0x102874ec => { vm.decode(0x2220012fu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102874f0u64) }
        0x102874f0 => { vm.decode(0x19189c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102874f4u64) }
        0x102874f4 => { vm.decode(0x14408091u32); let d = 16i16; let s = 3i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102874f8u64) }
        0x102874f8 => { vm.decode(0x4a8263afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102874fcu64) }
        0x102874fc => { vm.decode(0x2a80402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287500u64) }
        0x10287500 => { vm.decode(0x109119u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 96i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287504u64) }
        0x10287504 => { vm.decode(0x19099d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287508u64) }
        0x10287508 => { vm.decode(0x640d9059u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028750cu64) }
        0x1028750c => { vm.decode(0x19209530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287510u64) }
        0x10287510 => { vm.decode(0x19391db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287514u64) }
        0x10287514 => { vm.decode(0x19311cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287518u64) }
        0x10287518 => { vm.decode(0xa251062u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1028751cu64) }
        0x1028751c => { vm.decode(0x8643efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287520u64) }
        0x10287520 => { vm.decode(0x688c43afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287524u64) }
        0x10287524 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10287528u64) }
        0x10287528 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028752cu64 + 4);
        vm.wr(37, 0x1028752cu64 + 4);
        Ok(0x1028752cu64) }
        0x1028752c => { vm.decode(0x62598559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2416i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287530u64) }
        0x10287530 => { vm.decode(0x10401019u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287534u64) }
        0x10287534 => { vm.decode(0x208063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287538u64) }
        0x10287538 => { vm.decode(0x1b201cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028753cu64) }
        0x1028753c => { vm.decode(0x2884402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287540u64) }
        0x10287540 => { vm.decode(0x8b211530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287544u64) }
        0x10287544 => { vm.decode(0x9b1914b0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287548u64) }
        0x10287548 => { vm.decode(0x9b1154b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028754cu64) }
        0x1028754c => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287550u64) }
        0x10287550 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287554u64) }
        0x10287554 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10287558u64) }
        0x10287558 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028755cu64 + 4);
        vm.wr(37, 0x1028755cu64 + 4);
        Ok(0x1028755cu64) }
        0x1028755c => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287560u64) }
        0x10287560 => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287564u64) }
        0x10287564 => { vm.decode(0x2280406fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287568u64) }
        0x10287568 => { vm.decode(0x8b2115b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028756cu64) }
        0x1028756c => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10287570u64) }
        0x10287570 => { vm.decode(0x8b315570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287574u64) }
        0x10287574 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287578u64) }
        0x10287578 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028757cu64) }
        0x1028757c => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10287580u64) }
        0x10287580 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10287584u64 + 4);
        vm.wr(37, 0x10287584u64 + 4);
        Ok(0x10287584u64) }
        0x10287584 => { vm.decode(0x102036u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287588u64) }
        0x10287588 => { vm.decode(0x40020bb5u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028758cu64) }
        0x1028758c => { vm.decode(0x19109c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287590u64) }
        0x10287590 => { vm.decode(0x32156a58u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 60213i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10287594u64) }
        0x10287594 => { vm.decode(0x422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10287598u64) }
        0x10287598 => { vm.decode(0x1b3014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028759cu64) }
        0x1028759c => { vm.decode(0xdc607699u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = -16873i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102875a0u64) }
        0x102875a0 => { vm.decode(0x10109019u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102875a4u64) }
        0x102875a4 => { vm.decode(0x760d1419u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102875a8u64) }
        0x102875a8 => { vm.decode(0x24000446u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102875acu64) }
        0x102875ac => { vm.decode(0x2200031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102875b0u64).wrapping_add((imm * 4) as u64) } else { 0x102875b0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102875b0u64) }
        0x102875b0 => { vm.decode(0x6100036u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102875b4u64) }
        0x102875b4 => { vm.decode(0xa880411du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 10892i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102875b8u64) }
        0x102875bc => { vm.decode(0xa624735u32); let d = 1i16; let s = 4i16; let s2 = 23i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102875c0u64) }
        0x102875c0 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102875c4u64) }
        0x102875c4 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102875c8u64) }
        0x102875c8 => { vm.decode(0x74314675u32); let d = 3i16; let s = 3i16; let s2 = 23i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102875ccu64) }
        0x102875cc => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102875d0u64) }
        0x102875d0 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102875d4u64) }
        0x102875d4 => { vm.decode(0x8b21a4f5u32); let d = 2i16; let s = 21i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102875d8u64) }
        0x102875d8 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102875dcu64) }
        0x102875dc => { vm.decode(0x8903036u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102875e0u64) }
        0x102875e0 => { vm.decode(0x6a82636fu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102875e4u64) }
        0x102875e4 => { vm.decode(0x24000446u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102875e8u64) }
        0x102875e8 => { vm.decode(0x2140227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1544i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102875ecu64).wrapping_add((imm * 4) as u64) } else { 0x102875ecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102875ecu64) }
        0x102875ec => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102875f0u64) } }
        0x102875f0 => { vm.decode(0x2084c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102875f4u64) }
        0x102875f4 => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102875f8u64) }
        0x102875f8 => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102875fcu64) }
        0x102875fc => { vm.decode(0x82c202fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287600u64) }
        0x10287600 => { vm.decode(0x9b191530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287604u64) }
        0x10287604 => { vm.decode(0xb091530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287608u64) }
        0x10287608 => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028760cu64) }
        0x1028760c => { vm.decode(0x40820bb5u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10287610u64) }
        0x10287610 => { vm.decode(0x44104875u32); let d = 2i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10287614u64) }
        0x10287614 => { vm.decode(0xc040ad75u32); let d = 21i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10287618u64) }
        0x10287618 => { vm.decode(0x836004f5u32); let d = 1i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028761cu64) }
        0x1028761c => { vm.decode(0x5c000111u32); let d = 23i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10287620u64) }
        0x10287620 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287624u64) }
        0x10287624 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287628u64) }
        0x10287628 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028762cu64 + 4);
        vm.wr(37, 0x1028762cu64 + 4);
        Ok(0x1028762cu64) }
        0x1028762c => { vm.decode(0x90d875u32); let d = 5i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10287630u64) }
        0x10287630 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10287634u64) }
        0x10287634 => { vm.decode(0x28c40b35u32); let d = 4i16; let s = 3i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10287638u64) }
        0x10287638 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028763cu64) }
        0x1028763c => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10287640u64) }
        0x10287640 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10287644u64).wrapping_add((imm * 4) as u64) } else { 0x10287644u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287644u64) }
        0x10287644 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287648u64) }
        0x10287648 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028764cu64) }
        0x1028764c => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10287650u64) }
        0x10287650 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10287654u64) }
        0x10287654 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10287658u64) }
        0x10287658 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028765cu64).wrapping_add((imm * 4) as u64) } else { 0x1028765cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028765cu64) }
        0x1028765c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10287660u64) } }
        0x10287660 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287664u64) }
        0x10287664 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10287668u64) }
        0x10287668 => { vm.decode(0x739c34a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -44i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028766cu64).wrapping_add((imm * 4) as u64) } else { 0x1028766cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028766cu64) }
        0x1028766c => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287670u64) }
        0x10287670 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287674u64) }
        0x10287674 => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10287678u64) }
        0x10287678 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x1028767cu64) }
        0x1028767c => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10287680u64) }
        0x10287680 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10287684u64).wrapping_add((imm * 4) as u64) } else { 0x10287684u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287684u64) }
        0x10287684 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10287688u64) } }
        0x10287688 => { vm.decode(0x2984041du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 4668i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1028768cu64) }
        0x1028768c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10287690u64) } }
        0x10287690 => { vm.decode(0xb381c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287694u64) }
        0x10287694 => { vm.decode(0xb201cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287698u64) }
        0x10287698 => { vm.decode(0xb301c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028769cu64) }
        0x1028769c => { vm.decode(0xb281c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102876a0u64) }
        0x102876a0 => { vm.decode(0x621d1062u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102876a4u64) }
        0x102876a4 => { vm.decode(0x100036u32); let d = 16i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102876a8u64) }
        0x102876a8 => { vm.decode(0x10008119u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102876acu64) }
        0x102876ac => { vm.decode(0x5288d9u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 16817i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102876b0u64) }
        0x102876b0 => { vm.decode(0x20422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 17i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102876b4u64) }
        0x102876b4 => { vm.decode(0x5c701c59u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = -12496i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102876b8u64) }
        0x102876b8 => { vm.decode(0x760d9419u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102876bcu64) }
        0x102876bc => { vm.decode(0x20020200u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102876c0u64) }
        0x102876c0 => { vm.decode(0x2202031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 18i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102876c4u64).wrapping_add((imm * 4) as u64) } else { 0x102876c4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102876c4u64) }
        0x102876c4 => { vm.decode(0x9c76ffd9u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102876c8u64) }
        0x102876c8 => { vm.decode(0x504001du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 5152i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102876ccu64) }
        0x102876d0 => { vm.decode(0x60453835u32); let d = 1i16; let s = 7i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102876d4u64) }
        0x102876d4 => { vm.decode(0x44081ff5u32); let d = 4i16; let s = 4i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_shl(vm.ld(s) as u32);
        vm.st(d, v);
        Ok(0x102876d8u64) }
        0x102876d8 => { vm.decode(0x120207f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102876dcu64) }
        0x102876dc => { vm.decode(0x90350175u32); let d = 3i16; let s = 3i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_shr(vm.ld(s) as u32);
        vm.st(d, v);
        Ok(0x102876e0u64) }
        0x102876e0 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102876e4u64) }
        0x102876e4 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102876e8u64) }
        0x102876e8 => { vm.decode(0x8b214909u32); let d = 2i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102876ecu64) }
        0x102876ec => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102876f0u64) }
        0x102876f0 => { vm.decode(0x2a88636fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102876f4u64) }
        0x102876f4 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102876f8u64) }
        0x102876f8 => { vm.decode(0x106800b0u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102876fcu64) }
        0x102876fc => { vm.decode(0x8900036u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287700u64) }
        0x10287700 => { vm.decode(0x20020200u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x10287704u64) }
        0x10287704 => { vm.decode(0x4840827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 38i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10287708u64).wrapping_add((imm * 4) as u64) } else { 0x10287708u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287708u64) }
        0x10287708 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028770cu64) } }
        0x1028770c => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287710u64) }
        0x10287710 => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287714u64) }
        0x10287714 => { vm.decode(0x9b191430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287718u64) }
        0x10287718 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028771cu64) }
        0x1028771c => { vm.decode(0x4082736fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287720u64) }
        0x10287720 => { vm.decode(0x414124f5u32); let d = 1i16; let s = 18i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10287724u64) }
        0x10287724 => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287728u64) }
        0x10287728 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028772cu64) }
        0x1028772c => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287730u64) }
        0x10287730 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10287734u64 + 4);
        vm.wr(37, 0x10287734u64 + 4);
        Ok(0x10287734u64) }
        0x10287734 => { vm.decode(0x490d875u32); let d = 5i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10287738u64) }
        0x10287738 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028773cu64) }
        0x1028773c => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10287740u64) }
        0x10287740 => { vm.decode(0x830bb5u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10287744u64) }
        0x10287744 => { vm.decode(0x90a0b75u32); let d = 4i16; let s = 17i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_sub(vm.ld(s2));
        vm.st(d, v);
        Ok(0x10287748u64) }
        0x10287748 => { vm.decode(0x260bb5u32); let d = 5i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028774cu64) }
        0x1028774c => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10287750u64) }
        0x10287750 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10287754u64).wrapping_add((imm * 4) as u64) } else { 0x10287754u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287754u64) }
        0x10287754 => { vm.decode(0x1019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287758u64) }
        0x10287758 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028775cu64) }
        0x1028775c => { vm.decode(0x22400d75u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10287760u64) }
        0x10287760 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10287764u64) }
        0x10287764 => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10287768u64) }
        0x10287768 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028776cu64).wrapping_add((imm * 4) as u64) } else { 0x1028776cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028776cu64) }
        0x1028776c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10287770u64) } }
        0x10287770 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287774u64) }
        0x10287774 => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10287778u64) }
        0x10287778 => { vm.decode(0x739cb4a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -43i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1028777cu64).wrapping_add((imm * 4) as u64) } else { 0x1028777cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028777cu64) }
        0x1028777c => { vm.decode(0x2009019u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287780u64) }
        0x10287780 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287784u64) }
        0x10287784 => { vm.decode(0x32402d75u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10287788u64) }
        0x10287788 => { vm.decode(0xa0204118u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x1028778cu64) }
        0x1028778c => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10287790u64) }
        0x10287790 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10287794u64).wrapping_add((imm * 4) as u64) } else { 0x10287794u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287794u64) }
        0x10287794 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10287798u64) } }
        0x10287798 => { vm.decode(0xa4141du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 4944i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1028779cu64) }
        0x1028779c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102877a0u64) } }
        0x102877a0 => { vm.decode(0x4d94e211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102877a4u64) }
        0x102877a4 => { vm.decode(0x2a1c1027u32); let d = 0i16; let s = 3i16; let s2 = -1i16; let imm = 7696i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102877a8u64).wrapping_add((imm * 4) as u64) } else { 0x102877a8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102877a8u64) }
        0x102877a8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102877acu64) } }
        0x102877ac => { vm.decode(0x9823c909u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102877b0u64) }
        0x10287820 => { vm.decode(0x2a84632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287824u64) }
        0x10287824 => { vm.decode(0xa80184f5u32); let d = 2i16; let s = 1i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10287828u64) }
        0x10287828 => { vm.decode(0xb101530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028782cu64) }
        0x1028782c => { vm.decode(0xaa00a4f5u32); let d = 2i16; let s = 1i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10287830u64) }
        0x10287830 => { vm.decode(0xb3815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287834u64) }
        0x10287834 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10287838u64) }
        0x10287838 => { vm.decode(0x8843efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028783cu64) }
        0x1028783c => { vm.decode(0x10408191u32); let d = 20i16; let s = 2i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10287840u64) }
        0x10287840 => { vm.decode(0x228002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287844u64) }
        0x10287844 => { vm.decode(0xb009530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287848u64) }
        0x10287848 => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028784cu64) }
        0x1028784c => { vm.decode(0x82c002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287850u64) }
        0x10287850 => { vm.decode(0x10108019u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287854u64) }
        0x10287854 => { vm.decode(0x9b211430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287858u64) }
        0x10287858 => { vm.decode(0x1b1914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028785cu64) }
        0x1028785c => { vm.decode(0x288063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287860u64) }
        0x10287860 => { vm.decode(0x1b1114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287864u64) }
        0x10287864 => { vm.decode(0xb0915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287868u64) }
        0x10287868 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028786cu64) }
        0x1028786c => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287870u64) }
        0x10287870 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10287874u64 + 4);
        vm.wr(37, 0x10287874u64 + 4);
        Ok(0x10287874u64) }
        0x10287874 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287878u64) }
        0x10287878 => { vm.decode(0x100236u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028787cu64) }
        0x1028787c => { vm.decode(0x20156c18u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 47649i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10287880u64) }
        0x10287880 => { vm.decode(0x422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10287884u64) }
        0x10287884 => { vm.decode(0x1b201430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287888u64) }
        0x10287888 => { vm.decode(0x40708c19u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 19504i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028788cu64) }
        0x1028788c => { vm.decode(0x8863efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287890u64) }
        0x10287890 => { vm.decode(0x824604f5u32); let d = 21i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10287894u64) }
        0x10287894 => { vm.decode(0x760d1419u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287898u64) }
        0x10287898 => { vm.decode(0x6102236u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028789cu64) }
        0x1028789c => { vm.decode(0x6000506u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102878a0u64) }
        0x102878a0 => { vm.decode(0x2200031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102878a4u64).wrapping_add((imm * 4) as u64) } else { 0x102878a4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102878a4u64) }
        0x102878a4 => { vm.decode(0x70499519u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102878a8u64) }
        0x102878a8 => { vm.decode(0x2d00551du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 11580i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102878acu64) }
        0x102878b0 => { vm.decode(0x8024735u32); let d = 1i16; let s = 4i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102878b4u64) }
        0x102878b4 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102878b8u64) }
        0x102878b8 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102878bcu64) }
        0x102878bc => { vm.decode(0x64104675u32); let d = 3i16; let s = 3i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102878c0u64) }
        0x102878c0 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102878c4u64) }
        0x102878c4 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102878c8u64) }
        0x102878c8 => { vm.decode(0x6b2114f5u32); let d = 2i16; let s = 19i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102878ccu64) }
        0x102878cc => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102878d0u64) }
        0x102878d0 => { vm.decode(0x8b00236u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102878d4u64) }
        0x102878d4 => { vm.decode(0x408e6befu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102878d8u64) }
        0x102878d8 => { vm.decode(0x6000506u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102878dcu64) }
        0x102878dc => { vm.decode(0x549c88a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1527i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102878e0u64).wrapping_add((imm * 4) as u64) } else { 0x102878e0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102878e0u64) }
        0x102878e0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102878e4u64) } }
        0x102878e4 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102878e8u64) }
        0x102878e8 => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102878ecu64) }
        0x102878ec => { vm.decode(0x9b191430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102878f0u64) }
        0x102878f0 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102878f4u64) }
        0x102878f4 => { vm.decode(0x9b115430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102878f8u64) }
        0x102878f8 => { vm.decode(0x40030bb5u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102878fcu64) }
        0x102878fc => { vm.decode(0xc008bd75u32); let d = 30i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10287900u64) }
        0x10287900 => { vm.decode(0x288063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287904u64) }
        0x10287904 => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10287908u64) }
        0x10287908 => { vm.decode(0x14000011u32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028790cu64) }
        0x1028790c => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287910u64) }
        0x10287910 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287914u64) }
        0x10287914 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10287918u64 + 4);
        vm.wr(37, 0x10287918u64 + 4);
        Ok(0x10287918u64) }
        0x10287918 => { vm.decode(0x90d875u32); let d = 5i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028791cu64) }
        0x1028791c => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10287920u64) }
        0x10287920 => { vm.decode(0x28cc8b35u32); let d = 4i16; let s = 3i16; let s2 = 23i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10287924u64) }
        0x10287924 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287928u64) }
        0x10287928 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1028792cu64) }
        0x1028792c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10287930u64).wrapping_add((imm * 4) as u64) } else { 0x10287930u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287930u64) }
        0x10287930 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287934u64) }
        0x10287934 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287938u64) }
        0x10287938 => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028793cu64) }
        0x1028793c => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10287940u64) }
        0x10287940 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10287944u64) }
        0x10287944 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10287948u64).wrapping_add((imm * 4) as u64) } else { 0x10287948u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287948u64) }
        0x10287948 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028794cu64) } }
        0x1028794c => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287950u64) }
        0x10287950 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10287954u64) }
        0x10287954 => { vm.decode(0x739c3ca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -42i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10287958u64).wrapping_add((imm * 4) as u64) } else { 0x10287958u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287958u64) }
        0x10287958 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028795cu64) }
        0x1028795c => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287960u64) }
        0x10287960 => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10287964u64) }
        0x10287964 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10287968u64) }
        0x10287968 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1028796cu64) }
        0x1028796c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10287970u64).wrapping_add((imm * 4) as u64) } else { 0x10287970u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287970u64) }
        0x10287970 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10287974u64) } }
        0x10287974 => { vm.decode(0x504141du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 5424i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10287978u64) }
        0x10287978 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028797cu64) } }
        0x1028797c => { vm.decode(0xb081c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287980u64) }
        0x10287980 => { vm.decode(0xe08bd04du32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1295i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let w0 = vm.mem.r64(a & !7u64)?;
        let sh = ((a & 7) * 8) as u32;
        let v = w0.wrapping_shl(56 - sh) | (vm.ld(d) & 0x00ff_ffff_ffff_ffffu64.wrapping_shr(sh));
        vm.st(d, v);
        Ok(0x10287984u64) }
        0x10287984 => { vm.decode(0x51148302u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let w0 = vm.mem.r64(a & !7u64)?;
        let sh = ((a & 7) * 8) as u32;
        let v = if sh == 0 { w0 } else { w0.wrapping_shr(sh) | (vm.ld(d) & u64::MAX.wrapping_shl(64 - sh)) };
        vm.st(d, v);
        Ok(0x10287988u64) }
        0x10287988 => { vm.decode(0x1b101c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028798cu64) }
        0x1028798c => { vm.decode(0xe88bc04du32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1303i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let w0 = vm.mem.r64(a & !7u64)?;
        let sh = ((a & 7) * 8) as u32;
        let v = w0.wrapping_shl(56 - sh) | (vm.ld(d) & 0x00ff_ffff_ffff_ffffu64.wrapping_shr(sh));
        vm.st(d, v);
        Ok(0x10287990u64) }
        0x10287990 => { vm.decode(0x41168302u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let w0 = vm.mem.r64(a & !7u64)?;
        let sh = ((a & 7) * 8) as u32;
        let v = if sh == 0 { w0 } else { w0.wrapping_shr(sh) | (vm.ld(d) & u64::MAX.wrapping_shl(64 - sh)) };
        vm.st(d, v);
        Ok(0x10287994u64) }
        0x10287994 => { vm.decode(0x1b181c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287998u64) }
        0x10287998 => { vm.decode(0x6882c36fu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028799cu64) }
        0x1028799c => { vm.decode(0x12026031u32); let d = 0i16; let s = 21i16; let s2 = -1i16; let imm = 3i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102879a0u64).wrapping_add((imm * 4) as u64) } else { 0x102879a0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102879a0u64) }
        0x102879a0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102879a4u64) } }
        0x102879a4 => { vm.decode(0x8644001du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 169088i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102879a8u64) }
        0x102879ac => { vm.decode(0xd10e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102879b0u64) }
        0x102879b0 => { vm.decode(0xa4000fb5u32); let d = 2i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102879b4u64) }
        0x102879b4 => { vm.decode(0x21d5062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102879b8u64) }
        0x102879b8 => { vm.decode(0x20082e35u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) < vm.ld(s)) as u64;
        vm.st(d, v);
        Ok(0x102879bcu64) }
        0x102879bc => { vm.decode(0x428043efu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102879c0u64) }
        0x102879c0 => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102879c4u64).wrapping_add((imm * 4) as u64) } else { 0x102879c4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102879c4u64) }
        0x102879c4 => { vm.decode(0x50da812du32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 63i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102879c8u64) }
        0x102879c8 => { vm.decode(0xd10e291u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 780i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102879ccu64) }
        0x102879d4 => { vm.decode(0x408e6befu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102879d8u64) }
        0x102879d8 => { vm.decode(0xa022031u32); let d = 0i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102879dcu64).wrapping_add((imm * 4) as u64) } else { 0x102879dcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102879dcu64) }
        0x102879dc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102879e0u64) } }
        0x102879e0 => { vm.decode(0xa684551du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 147348i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102879e4u64) }
        0x102879e4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102879e8u64) } }
        0x102879e8 => { vm.decode(0x2076u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102879ecu64) }
        0x10287c34 => { vm.decode(0x2a84632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287c38u64) }
        0x10287c38 => { vm.decode(0xaa41a4f5u32); let d = 3i16; let s = 1i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10287c3cu64) }
        0x10287c3c => { vm.decode(0x19389530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287c40u64) }
        0x10287c40 => { vm.decode(0x8a4124f5u32); let d = 3i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10287c44u64) }
        0x10287c44 => { vm.decode(0x19309530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287c48u64) }
        0x10287c48 => { vm.decode(0xa84124f5u32); let d = 3i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10287c4cu64) }
        0x10287c4c => { vm.decode(0x19289d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287c50u64) }
        0x10287c50 => { vm.decode(0x884124f5u32); let d = 3i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10287c54u64) }
        0x10287c54 => { vm.decode(0x19209d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287c58u64) }
        0x10287c58 => { vm.decode(0x8a4104f5u32); let d = 3i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10287c5cu64) }
        0x10287c5c => { vm.decode(0x19189db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287c60u64) }
        0x10287c60 => { vm.decode(0xa84184f5u32); let d = 3i16; let s = 1i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10287c64u64) }
        0x10287c64 => { vm.decode(0x19109db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287c68u64) }
        0x10287c68 => { vm.decode(0x8244a4f5u32); let d = 17i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10287c6cu64) }
        0x10287c6c => { vm.decode(0x208843efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287c70u64) }
        0x10287c70 => { vm.decode(0x2220012fu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287c74u64) }
        0x10287c74 => { vm.decode(0x19089cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287c78u64) }
        0x10287c78 => { vm.decode(0x14408091u32); let d = 16i16; let s = 3i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10287c7cu64) }
        0x10287c7c => { vm.decode(0x4a8263afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287c80u64) }
        0x10287c80 => { vm.decode(0x2a80402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287c84u64) }
        0x10287c84 => { vm.decode(0x109119u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 96i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287c88u64) }
        0x10287c88 => { vm.decode(0x19099d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287c8cu64) }
        0x10287c8c => { vm.decode(0x640d9059u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287c90u64) }
        0x10287c90 => { vm.decode(0x192095b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287c94u64) }
        0x10287c94 => { vm.decode(0x19391db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287c98u64) }
        0x10287c98 => { vm.decode(0x19311cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287c9cu64) }
        0x10287c9c => { vm.decode(0xa251062u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10287ca0u64) }
        0x10287ca0 => { vm.decode(0x8643efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287ca4u64) }
        0x10287ca4 => { vm.decode(0x688c43afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287ca8u64) }
        0x10287ca8 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10287cacu64) }
        0x10287cac => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10287cb0u64 + 4);
        vm.wr(37, 0x10287cb0u64 + 4);
        Ok(0x10287cb0u64) }
        0x10287cb0 => { vm.decode(0x62598559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2416i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287cb4u64) }
        0x10287cb4 => { vm.decode(0x642d1019u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287cb8u64) }
        0x10287cb8 => { vm.decode(0x10401019u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287cbcu64) }
        0x10287cbc => { vm.decode(0x208063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287cc0u64) }
        0x10287cc0 => { vm.decode(0x1b201cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287cc4u64) }
        0x10287cc4 => { vm.decode(0x2884402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287cc8u64) }
        0x10287cc8 => { vm.decode(0x8b211530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287cccu64) }
        0x10287ccc => { vm.decode(0x9b1914b0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287cd0u64) }
        0x10287cd0 => { vm.decode(0xb2015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287cd4u64) }
        0x10287cd4 => { vm.decode(0xb1115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287cd8u64) }
        0x10287cd8 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287cdcu64) }
        0x10287cdc => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287ce0u64) }
        0x10287ce0 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10287ce4u64) }
        0x10287ce4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10287ce8u64 + 4);
        vm.wr(37, 0x10287ce8u64 + 4);
        Ok(0x10287ce8u64) }
        0x10287ce8 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287cecu64) }
        0x10287cec => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287cf0u64) }
        0x10287cf0 => { vm.decode(0x2280406fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287cf4u64) }
        0x10287cf4 => { vm.decode(0x8b2115b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287cf8u64) }
        0x10287cf8 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10287cfcu64) }
        0x10287cfc => { vm.decode(0x8b315570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287d00u64) }
        0x10287d00 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287d04u64) }
        0x10287d04 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287d08u64) }
        0x10287d08 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10287d0cu64) }
        0x10287d0c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10287d10u64 + 4);
        vm.wr(37, 0x10287d10u64 + 4);
        Ok(0x10287d10u64) }
        0x10287d10 => { vm.decode(0x102036u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287d14u64) }
        0x10287d14 => { vm.decode(0x40020bb5u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10287d18u64) }
        0x10287d18 => { vm.decode(0x19009cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287d1cu64) }
        0x10287d1c => { vm.decode(0x8004e0d9u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 8463i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287d20u64) }
        0x10287d20 => { vm.decode(0x80422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 18i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10287d24u64) }
        0x10287d24 => { vm.decode(0x5c207d19u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = -4540i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287d28u64) }
        0x10287d28 => { vm.decode(0x10109019u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287d2cu64) }
        0x10287d2c => { vm.decode(0x760d1419u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287d30u64) }
        0x10287d30 => { vm.decode(0x24000446u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10287d34u64) }
        0x10287d34 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10287d38u64).wrapping_add((imm * 4) as u64) } else { 0x10287d38u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287d38u64) }
        0x10287d38 => { vm.decode(0x6100036u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287d3cu64) }
        0x10287d44 => { vm.decode(0xa624735u32); let d = 1i16; let s = 4i16; let s2 = 23i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10287d48u64) }
        0x10287d48 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x10287d4cu64) }
        0x10287d4c => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10287d50u64) }
        0x10287d50 => { vm.decode(0x74314675u32); let d = 3i16; let s = 3i16; let s2 = 23i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10287d54u64) }
        0x10287d54 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10287d58u64) }
        0x10287d58 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10287d5cu64) }
        0x10287d5c => { vm.decode(0x8b21a4f5u32); let d = 2i16; let s = 21i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10287d60u64) }
        0x10287d60 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10287d64u64) }
        0x10287d64 => { vm.decode(0x8903036u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287d68u64) }
        0x10287d68 => { vm.decode(0x24000446u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10287d6cu64) }
        0x10287d6c => { vm.decode(0x44140a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1322i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10287d70u64).wrapping_add((imm * 4) as u64) } else { 0x10287d70u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287d70u64) }
        0x10287d70 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10287d74u64) } }
        0x10287d74 => { vm.decode(0x2084c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287d78u64) }
        0x10287d78 => { vm.decode(0x614004f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10287d7cu64) }
        0x10287d7c => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287d80u64) }
        0x10287d80 => { vm.decode(0x82c202fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287d84u64) }
        0x10287d84 => { vm.decode(0x9b191530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287d88u64) }
        0x10287d88 => { vm.decode(0xb091530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287d8cu64) }
        0x10287d8c => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287d90u64) }
        0x10287d90 => { vm.decode(0x40820bb5u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10287d94u64) }
        0x10287d94 => { vm.decode(0xc040ad75u32); let d = 21i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10287d98u64) }
        0x10287d98 => { vm.decode(0x208443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287d9cu64) }
        0x10287d9c => { vm.decode(0xa16004f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10287da0u64) }
        0x10287da0 => { vm.decode(0x5c000111u32); let d = 23i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10287da4u64) }
        0x10287da4 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287da8u64) }
        0x10287da8 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287dacu64) }
        0x10287dac => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10287db0u64 + 4);
        vm.wr(37, 0x10287db0u64 + 4);
        Ok(0x10287db0u64) }
        0x10287db0 => { vm.decode(0x90d875u32); let d = 5i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10287db4u64) }
        0x10287db4 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10287db8u64) }
        0x10287db8 => { vm.decode(0x28c40b35u32); let d = 4i16; let s = 3i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10287dbcu64) }
        0x10287dbc => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287dc0u64) }
        0x10287dc0 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10287dc4u64) }
        0x10287dc4 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10287dc8u64).wrapping_add((imm * 4) as u64) } else { 0x10287dc8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287dc8u64) }
        0x10287dc8 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287dccu64) }
        0x10287dcc => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287dd0u64) }
        0x10287dd0 => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10287dd4u64) }
        0x10287dd4 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10287dd8u64) }
        0x10287dd8 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10287ddcu64) }
        0x10287ddc => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10287de0u64).wrapping_add((imm * 4) as u64) } else { 0x10287de0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287de0u64) }
        0x10287de0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10287de4u64) } }
        0x10287de4 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287de8u64) }
        0x10287de8 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10287decu64) }
        0x10287dec => { vm.decode(0x739cb4a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -43i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10287df0u64).wrapping_add((imm * 4) as u64) } else { 0x10287df0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287df0u64) }
        0x10287df0 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287df4u64) }
        0x10287df4 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287df8u64) }
        0x10287df8 => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10287dfcu64) }
        0x10287dfc => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10287e00u64) }
        0x10287e00 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10287e04u64) }
        0x10287e04 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10287e08u64).wrapping_add((imm * 4) as u64) } else { 0x10287e08u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287e08u64) }
        0x10287e08 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10287e0cu64) } }
        0x10287e0c => { vm.decode(0xa024501du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 6596i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10287e10u64) }
        0x10287e10 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10287e14u64) } }
        0x10287e14 => { vm.decode(0xb381c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287e18u64) }
        0x10287e18 => { vm.decode(0xb201cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287e1cu64) }
        0x10287e1c => { vm.decode(0xb301c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287e20u64) }
        0x10287e20 => { vm.decode(0xb281c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287e24u64) }
        0x10287e24 => { vm.decode(0x621d1062u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10287e28u64) }
        0x10287e28 => { vm.decode(0x102036u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287e2cu64) }
        0x10287e2c => { vm.decode(0x10001119u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287e30u64) }
        0x10287e30 => { vm.decode(0x288063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287e34u64) }
        0x10287e34 => { vm.decode(0xd8544b99u32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = -3973i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287e38u64) }
        0x10287e38 => { vm.decode(0x760d9419u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287e3cu64) }
        0x10287e3c => { vm.decode(0x30020200u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x10287e40u64) }
        0x10287e40 => { vm.decode(0x2202031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 18i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10287e44u64).wrapping_add((imm * 4) as u64) } else { 0x10287e44u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287e44u64) }
        0x10287e44 => { vm.decode(0x9c76ffd9u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287e48u64) }
        0x10287e50 => { vm.decode(0x60453835u32); let d = 1i16; let s = 7i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10287e54u64) }
        0x10287e54 => { vm.decode(0x44081ff5u32); let d = 4i16; let s = 4i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_shl(vm.ld(s) as u32);
        vm.st(d, v);
        Ok(0x10287e58u64) }
        0x10287e58 => { vm.decode(0x120207f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10287e5cu64) }
        0x10287e5c => { vm.decode(0x90350175u32); let d = 3i16; let s = 3i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_shr(vm.ld(s) as u32);
        vm.st(d, v);
        Ok(0x10287e60u64) }
        0x10287e60 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10287e64u64) }
        0x10287e64 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10287e68u64) }
        0x10287e68 => { vm.decode(0x8b21c909u32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10287e6cu64) }
        0x10287e6c => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10287e70u64) }
        0x10287e70 => { vm.decode(0x2a88636fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287e74u64) }
        0x10287e74 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10287e78u64) }
        0x10287e78 => { vm.decode(0x106800b0u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287e7cu64) }
        0x10287e7c => { vm.decode(0x8903036u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287e80u64) }
        0x10287e80 => { vm.decode(0x30020200u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x10287e84u64) }
        0x10287e84 => { vm.decode(0x401c08a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1426i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10287e88u64).wrapping_add((imm * 4) as u64) } else { 0x10287e88u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287e88u64) }
        0x10287e88 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10287e8cu64) } }
        0x10287e8c => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287e90u64) }
        0x10287e90 => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287e94u64) }
        0x10287e94 => { vm.decode(0x8b191530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287e98u64) }
        0x10287e98 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287e9cu64) }
        0x10287e9c => { vm.decode(0x4082736fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287ea0u64) }
        0x10287ea0 => { vm.decode(0x14124f5u32); let d = 1i16; let s = 16i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10287ea4u64) }
        0x10287ea4 => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287ea8u64) }
        0x10287ea8 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287eacu64) }
        0x10287eac => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287eb0u64) }
        0x10287eb0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10287eb4u64 + 4);
        vm.wr(37, 0x10287eb4u64 + 4);
        Ok(0x10287eb4u64) }
        0x10287eb4 => { vm.decode(0x490d875u32); let d = 5i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10287eb8u64) }
        0x10287eb8 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287ebcu64) }
        0x10287ebc => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10287ec0u64) }
        0x10287ec0 => { vm.decode(0x830bb5u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10287ec4u64) }
        0x10287ec4 => { vm.decode(0x9220b75u32); let d = 4i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_sub(vm.ld(s));
        vm.st(d, v);
        Ok(0x10287ec8u64) }
        0x10287ec8 => { vm.decode(0x260bb5u32); let d = 5i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10287eccu64) }
        0x10287ecc => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10287ed0u64) }
        0x10287ed0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10287ed4u64).wrapping_add((imm * 4) as u64) } else { 0x10287ed4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287ed4u64) }
        0x10287ed4 => { vm.decode(0x1019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287ed8u64) }
        0x10287ed8 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287edcu64) }
        0x10287edc => { vm.decode(0x22400d75u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10287ee0u64) }
        0x10287ee0 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10287ee4u64) }
        0x10287ee4 => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10287ee8u64) }
        0x10287ee8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10287eecu64).wrapping_add((imm * 4) as u64) } else { 0x10287eecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287eecu64) }
        0x10287eec => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10287ef0u64) } }
        0x10287ef0 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287ef4u64) }
        0x10287ef4 => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10287ef8u64) }
        0x10287ef8 => { vm.decode(0x739cb4a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -43i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10287efcu64).wrapping_add((imm * 4) as u64) } else { 0x10287efcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287efcu64) }
        0x10287efc => { vm.decode(0x2009019u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287f00u64) }
        0x10287f00 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287f04u64) }
        0x10287f04 => { vm.decode(0x32402d75u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10287f08u64) }
        0x10287f08 => { vm.decode(0xa0204118u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10287f0cu64) }
        0x10287f0c => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10287f10u64) }
        0x10287f10 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10287f14u64).wrapping_add((imm * 4) as u64) } else { 0x10287f14u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287f14u64) }
        0x10287f14 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10287f18u64) } }
        0x10287f18 => { vm.decode(0x80a4441du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 6864i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10287f1cu64) }
        0x10287f1c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10287f20u64) } }
        0x10287f20 => { vm.decode(0x2a84632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287f24u64) }
        0x10287f24 => { vm.decode(0x28c63afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287f28u64) }
        0x10287f28 => { vm.decode(0x824404f5u32); let d = 17i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10287f2cu64) }
        0x10287f2c => { vm.decode(0x208043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287f30u64) }
        0x10287f30 => { vm.decode(0x44008191u32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10287f34u64) }
        0x10287f34 => { vm.decode(0x220a012fu32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287f38u64) }
        0x10287f38 => { vm.decode(0x42004b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10287f3cu64) }
        0x10287f3c => { vm.decode(0x1099u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287f40u64) }
        0x10287f40 => { vm.decode(0x740d0059u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287f44u64) }
        0x10287f44 => { vm.decode(0x9119u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287f48u64) }
        0x10287f48 => { vm.decode(0x404104f5u32); let d = 1i16; let s = 2i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10287f4cu64) }
        0x10287f4c => { vm.decode(0x801099u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287f50u64) }
        0x10287f50 => { vm.decode(0xadfb8df1u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = -3i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10287f54u64).wrapping_add((imm * 4) as u64) } else { 0x10287f54u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287f54u64) }
        0x10287f54 => { vm.decode(0x833u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10287f58u64) }
        0x10287f58 => { vm.decode(0xb281c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287f5cu64) }
        0x10287f5c => { vm.decode(0x41119u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 72i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287f60u64) }
        0x10287f60 => { vm.decode(0x109119u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 96i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287f64u64) }
        0x10287f64 => { vm.decode(0x2844befu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287f68u64) }
        0x10287f68 => { vm.decode(0x404104f5u32); let d = 1i16; let s = 2i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10287f6cu64) }
        0x10287f6c => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287f70u64) }
        0x10287f70 => { vm.decode(0xadfb8df1u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = -3i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10287f74u64).wrapping_add((imm * 4) as u64) } else { 0x10287f74u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287f74u64) }
        0x10287f74 => { vm.decode(0x430u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287f78u64) }
        0x10287f78 => { vm.decode(0x1910d5b0u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287f7cu64) }
        0x10287f7c => { vm.decode(0x42655022u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10287f80u64) }
        0x10287f80 => { vm.decode(0xb301cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287f84u64) }
        0x10287f84 => { vm.decode(0x501119u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 112i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287f88u64) }
        0x10287f88 => { vm.decode(0x169019u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 168i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287f8cu64) }
        0x10287f8c => { vm.decode(0x404104f5u32); let d = 1i16; let s = 2i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10287f90u64) }
        0x10287f90 => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287f94u64) }
        0x10287f94 => { vm.decode(0xadfb8df1u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = -3i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10287f98u64).wrapping_add((imm * 4) as u64) } else { 0x10287f98u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10287f98u64) }
        0x10287f98 => { vm.decode(0x430u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287f9cu64) }
        0x10287f9c => { vm.decode(0xb289cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287fa0u64) }
        0x10287fa0 => { vm.decode(0xb209cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287fa4u64) }
        0x10287fa4 => { vm.decode(0xb389c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287fa8u64) }
        0x10287fa8 => { vm.decode(0xb309c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287facu64) }
        0x10287fac => { vm.decode(0xb289c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287fb0u64) }
        0x10287fb0 => { vm.decode(0xb209c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287fb4u64) }
        0x10287fb4 => { vm.decode(0xb189cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287fb8u64) }
        0x10287fb8 => { vm.decode(0xb109cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287fbcu64) }
        0x10287fbc => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287fc0u64) }
        0x10287fc0 => { vm.decode(0x2800012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287fc4u64) }
        0x10287fc4 => { vm.decode(0x89319c30u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287fc8u64) }
        0x10287fc8 => { vm.decode(0x19299c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287fccu64) }
        0x10287fcc => { vm.decode(0x620f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2504i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287fd0u64) }
        0x10287fd0 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287fd4u64) }
        0x10287fd4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10287fd8u64 + 4);
        vm.wr(37, 0x10287fd8u64 + 4);
        Ok(0x10287fd8u64) }
        0x10287fd8 => { vm.decode(0xa00875u32); let d = 4i16; let s = 0i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10287fdcu64) }
        0x10287fdc => { vm.decode(0x208063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10287fe0u64) }
        0x10287fe0 => { vm.decode(0x39281430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287fe4u64) }
        0x10287fe4 => { vm.decode(0x89309430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10287fe8u64) }
        0x10287fe8 => { vm.decode(0x30160119u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 232i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287fecu64) }
        0x10287fec => { vm.decode(0x100236u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10287ff0u64) }
        0x10287ff0 => { vm.decode(0x10009119u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287ff4u64) }
        0x10287ff4 => { vm.decode(0x8450a999u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 17015i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10287ff8u64) }
        0x10287ff8 => { vm.decode(0x20422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 17i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10287ffcu64) }
        0x10287ffc => { vm.decode(0xda326419u32); let d = 20i16; let s = 1i16; let s2 = -1i16; let imm = -21338i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288000u64) }
        0x10288000 => { vm.decode(0x760d9419u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288004u64) }
        0x10288004 => { vm.decode(0x6120506u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 11i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10288008u64) }
        0x10288008 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028800cu64).wrapping_add((imm * 4) as u64) } else { 0x1028800cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028800cu64) }
        0x1028800c => { vm.decode(0x9e76efd9u32); let d = 21i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288010u64) }
        0x10288010 => { vm.decode(0xa024151du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 12756i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10288014u64) }
        0x10288018 => { vm.decode(0x6041b835u32); let d = 1i16; let s = 7i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028801cu64) }
        0x1028801c => { vm.decode(0x4e081ff5u32); let d = 4i16; let s = 30i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shl(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x10288020u64) }
        0x10288020 => { vm.decode(0x120207f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288024u64) }
        0x10288024 => { vm.decode(0x94352175u32); let d = 3i16; let s = 30i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shr(vm.ld(s2) as u32);
        vm.st(d, v);
        Ok(0x10288028u64) }
        0x10288028 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028802cu64) }
        0x1028802c => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288030u64) }
        0x10288030 => { vm.decode(0x492104f5u32); let d = 2i16; let s = 16i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10288034u64) }
        0x10288034 => { vm.decode(0x10400030u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288038u64) }
        0x10288038 => { vm.decode(0x8b00236u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028803cu64) }
        0x1028803c => { vm.decode(0x6120506u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 11i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10288040u64) }
        0x10288040 => { vm.decode(0x50940027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1348i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10288044u64).wrapping_add((imm * 4) as u64) } else { 0x10288044u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288044u64) }
        0x10288044 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10288048u64) } }
        0x10288048 => { vm.decode(0x40030bb5u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028804cu64) }
        0x1028804c => { vm.decode(0xe000bd75u32); let d = 22i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10288050u64) }
        0x10288050 => { vm.decode(0x616104f5u32); let d = 1i16; let s = 17i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10288054u64) }
        0x10288054 => { vm.decode(0x400a092fu32); let d = 30i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288058u64) }
        0x10288058 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028805cu64) }
        0x1028805c => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288060u64) }
        0x10288060 => { vm.decode(0x9b191530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288064u64) }
        0x10288064 => { vm.decode(0x88736fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288068u64) }
        0x10288068 => { vm.decode(0xb2004f5u32); let d = 2i16; let s = 2i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028806cu64) }
        0x1028806c => { vm.decode(0xb111530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288070u64) }
        0x10288070 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288074u64) }
        0x10288074 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288078u64) }
        0x10288078 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028807cu64) }
        0x1028807c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10288080u64 + 4);
        vm.wr(37, 0x10288080u64 + 4);
        Ok(0x10288080u64) }
        0x10288080 => { vm.decode(0x490d875u32); let d = 5i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288084u64) }
        0x10288084 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288088u64) }
        0x10288088 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1028808cu64) }
        0x1028808c => { vm.decode(0x830bb5u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10288090u64) }
        0x10288090 => { vm.decode(0x92a0b75u32); let d = 4i16; let s = 19i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_sub(vm.ld(s2));
        vm.st(d, v);
        Ok(0x10288094u64) }
        0x10288094 => { vm.decode(0x260bb5u32); let d = 5i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10288098u64) }
        0x10288098 => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1028809cu64) }
        0x1028809c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102880a0u64).wrapping_add((imm * 4) as u64) } else { 0x102880a0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102880a0u64) }
        0x102880a0 => { vm.decode(0x1019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102880a4u64) }
        0x102880a4 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102880a8u64) }
        0x102880a8 => { vm.decode(0x22400d75u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102880acu64) }
        0x102880ac => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102880b0u64) }
        0x102880b0 => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102880b4u64) }
        0x102880b4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102880b8u64).wrapping_add((imm * 4) as u64) } else { 0x102880b8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102880b8u64) }
        0x102880b8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102880bcu64) } }
        0x102880bc => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102880c0u64) }
        0x102880c0 => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102880c4u64) }
        0x102880c4 => { vm.decode(0x739c34a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -44i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102880c8u64).wrapping_add((imm * 4) as u64) } else { 0x102880c8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102880c8u64) }
        0x102880c8 => { vm.decode(0x2009019u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102880ccu64) }
        0x102880cc => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102880d0u64) }
        0x102880d0 => { vm.decode(0x32402d75u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102880d4u64) }
        0x102880d4 => { vm.decode(0xa0204118u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102880d8u64) }
        0x102880d8 => { vm.decode(0x21288375u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102880dcu64) }
        0x102880dc => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102880e0u64).wrapping_add((imm * 4) as u64) } else { 0x102880e0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102880e0u64) }
        0x102880e0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102880e4u64) } }
        0x102880e4 => { vm.decode(0x8c04441du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 7320i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102880e8u64) }
        0x102880e8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102880ecu64) } }
        0x102880ec => { vm.decode(0x9b1015b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102880f0u64) }
        0x102880f0 => { vm.decode(0x6a8063afu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102880f4u64) }
        0x102880f4 => { vm.decode(0x2804412fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102880f8u64) }
        0x102880f8 => { vm.decode(0x10001059u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 256i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102880fcu64) }
        0x102880fc => { vm.decode(0x8b211530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288100u64) }
        0x10288100 => { vm.decode(0x28c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288104u64) }
        0x10288104 => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288108u64) }
        0x10288108 => { vm.decode(0x742d0019u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028810cu64) }
        0x1028810c => { vm.decode(0x8b1114b0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288110u64) }
        0x10288110 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288114u64) }
        0x10288114 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288118u64) }
        0x10288118 => { vm.decode(0x688c43afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028811cu64) }
        0x1028811c => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288120u64) }
        0x10288120 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10288124u64 + 4);
        vm.wr(37, 0x10288124u64 + 4);
        Ok(0x10288124u64) }
        0x10288124 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288128u64) }
        0x10288128 => { vm.decode(0x2200416fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028812cu64) }
        0x1028812c => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288130u64) }
        0x10288130 => { vm.decode(0x8b2115b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288134u64) }
        0x10288134 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10288138u64) }
        0x10288138 => { vm.decode(0x8b315570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028813cu64) }
        0x1028813c => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288140u64) }
        0x10288140 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288144u64) }
        0x10288144 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288148u64) }
        0x10288148 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028814cu64 + 4);
        vm.wr(37, 0x1028814cu64 + 4);
        Ok(0x1028814cu64) }
        0x1028814c => { vm.decode(0x102036u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288150u64) }
        0x10288150 => { vm.decode(0x208043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288154u64) }
        0x10288154 => { vm.decode(0xc006a099u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 143i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288158u64) }
        0x10288158 => { vm.decode(0x8863efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028815cu64) }
        0x1028815c => { vm.decode(0x8a0404f5u32); let d = 18i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10288160u64) }
        0x10288160 => { vm.decode(0x12108019u32); let d = 21i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288164u64) }
        0x10288164 => { vm.decode(0x760d1419u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288168u64) }
        0x10288168 => { vm.decode(0x6102236u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028816cu64) }
        0x1028816c => { vm.decode(0x24000486u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10288170u64) }
        0x10288170 => { vm.decode(0x2200031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10288174u64).wrapping_add((imm * 4) as u64) } else { 0x10288174u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288174u64) }
        0x10288174 => { vm.decode(0x4010c875u32); let d = 3i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288178u64) }
        0x10288178 => { vm.decode(0x89a4011du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 13032i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1028817cu64) }
        0x10288180 => { vm.decode(0xa424735u32); let d = 1i16; let s = 4i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10288184u64) }
        0x10288184 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x10288188u64) }
        0x10288188 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028818cu64) }
        0x1028818c => { vm.decode(0x74114675u32); let d = 3i16; let s = 3i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10288190u64) }
        0x10288190 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288194u64) }
        0x10288194 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288198u64) }
        0x10288198 => { vm.decode(0x8c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028819cu64) }
        0x1028819c => { vm.decode(0x4b2014f5u32); let d = 2i16; let s = 2i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102881a0u64) }
        0x102881a0 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102881a4u64) }
        0x102881a4 => { vm.decode(0x8903036u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102881a8u64) }
        0x102881a8 => { vm.decode(0x24000486u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102881acu64) }
        0x102881ac => { vm.decode(0x44940a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1326i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102881b0u64).wrapping_add((imm * 4) as u64) } else { 0x102881b0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102881b0u64) }
        0x102881b0 => { vm.decode(0x4010c875u32); let d = 3i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102881b4u64) }
        0x102881b4 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102881b8u64) }
        0x102881b8 => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102881bcu64) }
        0x102881bc => { vm.decode(0x9b195430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102881c0u64) }
        0x102881c0 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102881c4u64) }
        0x102881c4 => { vm.decode(0x8b111530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102881c8u64) }
        0x102881c8 => { vm.decode(0x40820bb5u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102881ccu64) }
        0x102881cc => { vm.decode(0xc008bd75u32); let d = 30i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102881d0u64) }
        0x102881d0 => { vm.decode(0x416114f5u32); let d = 1i16; let s = 16i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102881d4u64) }
        0x102881d4 => { vm.decode(0x5c000011u32); let d = 19i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102881d8u64) }
        0x102881d8 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102881dcu64) }
        0x102881dc => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102881e0u64) }
        0x102881e0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102881e4u64 + 4);
        vm.wr(37, 0x102881e4u64 + 4);
        Ok(0x102881e4u64) }
        0x102881e4 => { vm.decode(0x90d875u32); let d = 5i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102881e8u64) }
        0x102881e8 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102881ecu64) }
        0x102881ec => { vm.decode(0x28cc8b35u32); let d = 4i16; let s = 3i16; let s2 = 23i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102881f0u64) }
        0x102881f0 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102881f4u64) }
        0x102881f4 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102881f8u64) }
        0x102881f8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102881fcu64).wrapping_add((imm * 4) as u64) } else { 0x102881fcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102881fcu64) }
        0x102881fc => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288200u64) }
        0x10288200 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288204u64) }
        0x10288204 => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10288208u64) }
        0x10288208 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x1028820cu64) }
        0x1028820c => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10288210u64) }
        0x10288210 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10288214u64).wrapping_add((imm * 4) as u64) } else { 0x10288214u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288214u64) }
        0x10288214 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10288218u64) } }
        0x10288218 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028821cu64) }
        0x1028821c => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10288220u64) }
        0x10288220 => { vm.decode(0x739cbca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -41i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10288224u64).wrapping_add((imm * 4) as u64) } else { 0x10288224u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288224u64) }
        0x10288224 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288228u64) }
        0x10288228 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028822cu64) }
        0x1028822c => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10288230u64) }
        0x10288230 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10288234u64) }
        0x10288234 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10288238u64) }
        0x10288238 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1028823cu64).wrapping_add((imm * 4) as u64) } else { 0x1028823cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1028823cu64) }
        0x1028823c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10288240u64) } }
        0x10288240 => { vm.decode(0x484401du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 7680i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10288244u64) }
        0x10288244 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10288248u64) } }
        0x10288248 => { vm.decode(0x2a84632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028824cu64) }
        0x1028824c => { vm.decode(0xaa0104f5u32); let d = 2i16; let s = 19i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10288250u64) }
        0x10288250 => { vm.decode(0xb101530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288254u64) }
        0x10288254 => { vm.decode(0x800704f5u32); let d = 20i16; let s = 16i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10288258u64) }
        0x10288258 => { vm.decode(0xa240a4f5u32); let d = 1i16; let s = 15i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028825cu64) }
        0x1028825c => { vm.decode(0x208843efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288260u64) }
        0x10288260 => { vm.decode(0x44408091u32); let d = 2i16; let s = 3i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10288264u64) }
        0x10288264 => { vm.decode(0x22d5022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10288268u64) }
        0x10288268 => { vm.decode(0x228012fu32); let d = 2i16; let s = 3i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028826cu64) }
        0x1028826c => { vm.decode(0xb3815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288270u64) }
        0x10288270 => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288274u64) }
        0x10288274 => { vm.decode(0x82c002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288278u64) }
        0x10288278 => { vm.decode(0x10108019u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028827cu64) }
        0x1028827c => { vm.decode(0x9b211430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288280u64) }
        0x10288280 => { vm.decode(0x1b1914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288284u64) }
        0x10288284 => { vm.decode(0x288063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288288u64) }
        0x10288288 => { vm.decode(0x1b1114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028828cu64) }
        0x1028828c => { vm.decode(0xb0915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288290u64) }
        0x10288290 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288294u64) }
        0x10288294 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288298u64) }
        0x10288298 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1028829cu64 + 4);
        vm.wr(37, 0x1028829cu64 + 4);
        Ok(0x1028829cu64) }
        0x1028829c => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102882a0u64) }
        0x102882a0 => { vm.decode(0x100236u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102882a4u64) }
        0x102882a4 => { vm.decode(0x84028cd9u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 19331i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102882a8u64) }
        0x102882a8 => { vm.decode(0x20422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 17i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102882acu64) }
        0x102882ac => { vm.decode(0xc050e959u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24950i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102882b0u64) }
        0x102882b0 => { vm.decode(0x8863efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102882b4u64) }
        0x102882b4 => { vm.decode(0x8a4404f5u32); let d = 19i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102882b8u64) }
        0x102882b8 => { vm.decode(0x760d1419u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102882bcu64) }
        0x102882bc => { vm.decode(0x60498519u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102882c0u64) }
        0x102882c0 => { vm.decode(0x1b2014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102882c4u64) }
        0x102882c4 => { vm.decode(0x6000506u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102882c8u64) }
        0x102882c8 => { vm.decode(0x2200031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102882ccu64).wrapping_add((imm * 4) as u64) } else { 0x102882ccu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102882ccu64) }
        0x102882cc => { vm.decode(0x6102036u32); let d = 21i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102882d0u64) }
        0x102882d0 => { vm.decode(0x184551du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 15152i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102882d4u64) }
        0x102882d8 => { vm.decode(0x8024735u32); let d = 1i16; let s = 4i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102882dcu64) }
        0x102882dc => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102882e0u64) }
        0x102882e0 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102882e4u64) }
        0x102882e4 => { vm.decode(0x64104675u32); let d = 3i16; let s = 3i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102882e8u64) }
        0x102882e8 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102882ecu64) }
        0x102882ec => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102882f0u64) }
        0x102882f0 => { vm.decode(0x8c43efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102882f4u64) }
        0x102882f4 => { vm.decode(0xcb2004f5u32); let d = 2i16; let s = 2i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102882f8u64) }
        0x102882f8 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102882fcu64) }
        0x102882fc => { vm.decode(0x8b00236u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288300u64) }
        0x10288300 => { vm.decode(0x6000506u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x10288304u64) }
        0x10288304 => { vm.decode(0x16140aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1770i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10288308u64).wrapping_add((imm * 4) as u64) } else { 0x10288308u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288308u64) }
        0x10288308 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028830cu64) } }
        0x1028830c => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288310u64) }
        0x10288310 => { vm.decode(0x2804212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288314u64) }
        0x10288314 => { vm.decode(0x9b191430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288318u64) }
        0x10288318 => { vm.decode(0x1b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028831cu64) }
        0x1028831c => { vm.decode(0x9b111530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288320u64) }
        0x10288320 => { vm.decode(0x40030bb5u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10288324u64) }
        0x10288324 => { vm.decode(0xc040bd75u32); let d = 23i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10288328u64) }
        0x10288328 => { vm.decode(0x288063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028832cu64) }
        0x1028832c => { vm.decode(0xe16004f5u32); let d = 1i16; let s = 1i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10288330u64) }
        0x10288330 => { vm.decode(0x14000011u32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10288334u64) }
        0x10288334 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288338u64) }
        0x10288338 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028833cu64) }
        0x1028833c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10288340u64 + 4);
        vm.wr(37, 0x10288340u64 + 4);
        Ok(0x10288340u64) }
        0x10288340 => { vm.decode(0x90d875u32); let d = 5i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288344u64) }
        0x10288344 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10288348u64) }
        0x10288348 => { vm.decode(0x28c48b35u32); let d = 4i16; let s = 3i16; let s2 = 21i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028834cu64) }
        0x1028834c => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288350u64) }
        0x10288350 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10288354u64) }
        0x10288354 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10288358u64).wrapping_add((imm * 4) as u64) } else { 0x10288358u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288358u64) }
        0x10288358 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1028835cu64) }
        0x1028835c => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288360u64) }
        0x10288360 => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10288364u64) }
        0x10288364 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10288368u64) }
        0x10288368 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1028836cu64) }
        0x1028836c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10288370u64).wrapping_add((imm * 4) as u64) } else { 0x10288370u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288370u64) }
        0x10288370 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10288374u64) } }
        0x10288374 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288378u64) }
        0x10288378 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x1028837cu64) }
        0x1028837c => { vm.decode(0x739c3ca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -42i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10288380u64).wrapping_add((imm * 4) as u64) } else { 0x10288380u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288380u64) }
        0x10288380 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288384u64) }
        0x10288384 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288388u64) }
        0x10288388 => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028838cu64) }
        0x1028838c => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10288390u64) }
        0x10288390 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x10288394u64) }
        0x10288394 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10288398u64).wrapping_add((imm * 4) as u64) } else { 0x10288398u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288398u64) }
        0x10288398 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1028839cu64) } }
        0x1028839c => { vm.decode(0xca4541du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 8024i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102883a0u64) }
        0x102883a0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102883a4u64) } }
        0x102883a4 => { vm.decode(0xb081c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102883a8u64) }
        0x102883a8 => { vm.decode(0xe08bd04du32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1295i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let w0 = vm.mem.r64(a & !7u64)?;
        let sh = ((a & 7) * 8) as u32;
        let v = w0.wrapping_shl(56 - sh) | (vm.ld(d) & 0x00ff_ffff_ffff_ffffu64.wrapping_shr(sh));
        vm.st(d, v);
        Ok(0x102883acu64) }
        0x102883ac => { vm.decode(0x51148302u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let w0 = vm.mem.r64(a & !7u64)?;
        let sh = ((a & 7) * 8) as u32;
        let v = if sh == 0 { w0 } else { w0.wrapping_shr(sh) | (vm.ld(d) & u64::MAX.wrapping_shl(64 - sh)) };
        vm.st(d, v);
        Ok(0x102883b0u64) }
        0x102883b0 => { vm.decode(0x1b101c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102883b4u64) }
        0x102883b4 => { vm.decode(0xe88bc04du32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1303i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let w0 = vm.mem.r64(a & !7u64)?;
        let sh = ((a & 7) * 8) as u32;
        let v = w0.wrapping_shl(56 - sh) | (vm.ld(d) & 0x00ff_ffff_ffff_ffffu64.wrapping_shr(sh));
        vm.st(d, v);
        Ok(0x102883b8u64) }
        0x102883b8 => { vm.decode(0x41168302u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let w0 = vm.mem.r64(a & !7u64)?;
        let sh = ((a & 7) * 8) as u32;
        let v = if sh == 0 { w0 } else { w0.wrapping_shr(sh) | (vm.ld(d) & u64::MAX.wrapping_shl(64 - sh)) };
        vm.st(d, v);
        Ok(0x102883bcu64) }
        0x102883bc => { vm.decode(0x1b181c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102883c0u64) }
        0x102883c0 => { vm.decode(0x6882c36fu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102883c4u64) }
        0x102883c4 => { vm.decode(0x12026031u32); let d = 0i16; let s = 21i16; let s2 = -1i16; let imm = 3i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102883c8u64).wrapping_add((imm * 4) as u64) } else { 0x102883c8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102883c8u64) }
        0x102883c8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102883ccu64) } }
        0x102883d4 => { vm.decode(0xd10e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102883d8u64) }
        0x102883d8 => { vm.decode(0xa4000fb5u32); let d = 2i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102883dcu64) }
        0x102883dc => { vm.decode(0x21d5062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102883e0u64) }
        0x102883e0 => { vm.decode(0x20082e35u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) < vm.ld(s)) as u64;
        vm.st(d, v);
        Ok(0x102883e4u64) }
        0x102883e4 => { vm.decode(0x428043efu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102883e8u64) }
        0x102883e8 => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102883ecu64).wrapping_add((imm * 4) as u64) } else { 0x102883ecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102883ecu64) }
        0x102883ec => { vm.decode(0x50da812du32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 63i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102883f0u64) }
        0x102883fc => { vm.decode(0x408e6befu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288400u64) }
        0x10288400 => { vm.decode(0xa022031u32); let d = 0i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10288404u64).wrapping_add((imm * 4) as u64) } else { 0x10288404u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288404u64) }
        0x10288404 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10288408u64) } }
        0x10288408 => { vm.decode(0xae84551du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 147356i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1028840cu64) }
        0x1028840c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10288410u64) } }
        0x1028865c => { vm.decode(0x2a84632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288660u64) }
        0x10288660 => { vm.decode(0xa84124f5u32); let d = 3i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10288664u64) }
        0x10288664 => { vm.decode(0x19309530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288668u64) }
        0x10288668 => { vm.decode(0xaa4104f5u32); let d = 3i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028866cu64) }
        0x1028866c => { vm.decode(0x19209530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288670u64) }
        0x10288670 => { vm.decode(0x8a4104f5u32); let d = 3i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10288674u64) }
        0x10288674 => { vm.decode(0x19189db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288678u64) }
        0x10288678 => { vm.decode(0x884104f5u32); let d = 3i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028867cu64) }
        0x1028867c => { vm.decode(0x19109db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288680u64) }
        0x10288680 => { vm.decode(0x884184f5u32); let d = 3i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10288684u64) }
        0x10288684 => { vm.decode(0x19089db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288688u64) }
        0x10288688 => { vm.decode(0x8a40a4f5u32); let d = 3i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028868cu64) }
        0x1028868c => { vm.decode(0x19009db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288690u64) }
        0x10288690 => { vm.decode(0x8044a4f5u32); let d = 17i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10288694u64) }
        0x10288694 => { vm.decode(0x208843efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288698u64) }
        0x10288698 => { vm.decode(0x2220012fu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1028869cu64) }
        0x1028869c => { vm.decode(0x19209c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102886a0u64) }
        0x102886a0 => { vm.decode(0x14408091u32); let d = 16i16; let s = 3i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102886a4u64) }
        0x102886a4 => { vm.decode(0x4a8263afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102886a8u64) }
        0x102886a8 => { vm.decode(0x2a80402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102886acu64) }
        0x102886ac => { vm.decode(0x109119u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 96i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102886b0u64) }
        0x102886b0 => { vm.decode(0x19099d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102886b4u64) }
        0x102886b4 => { vm.decode(0x760d9059u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102886b8u64) }
        0x102886b8 => { vm.decode(0x99395db0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102886bcu64) }
        0x102886bc => { vm.decode(0x19311cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102886c0u64) }
        0x102886c0 => { vm.decode(0xa251062u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102886c4u64) }
        0x102886c4 => { vm.decode(0x8643efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102886c8u64) }
        0x102886c8 => { vm.decode(0x688c43afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102886ccu64) }
        0x102886cc => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102886d0u64) }
        0x102886d0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102886d4u64 + 4);
        vm.wr(37, 0x102886d4u64 + 4);
        Ok(0x102886d4u64) }
        0x102886d4 => { vm.decode(0x62598559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2416i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102886d8u64) }
        0x102886d8 => { vm.decode(0x762d8019u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102886dcu64) }
        0x102886dc => { vm.decode(0x10401019u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102886e0u64) }
        0x102886e0 => { vm.decode(0x208063efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102886e4u64) }
        0x102886e4 => { vm.decode(0x1b201cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102886e8u64) }
        0x102886e8 => { vm.decode(0x2884402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102886ecu64) }
        0x102886ec => { vm.decode(0x8b211530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102886f0u64) }
        0x102886f0 => { vm.decode(0x9b1914b0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102886f4u64) }
        0x102886f4 => { vm.decode(0x9b1154b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102886f8u64) }
        0x102886f8 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102886fcu64) }
        0x102886fc => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288700u64) }
        0x10288700 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288704u64) }
        0x10288704 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10288708u64 + 4);
        vm.wr(37, 0x10288708u64 + 4);
        Ok(0x10288708u64) }
        0x10288708 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028870cu64) }
        0x1028870c => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288710u64) }
        0x10288710 => { vm.decode(0x2280406fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288714u64) }
        0x10288714 => { vm.decode(0x8b2115b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288718u64) }
        0x10288718 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1028871cu64) }
        0x1028871c => { vm.decode(0x8b315570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288720u64) }
        0x10288720 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288724u64) }
        0x10288724 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288728u64) }
        0x10288728 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1028872cu64) }
        0x1028872c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10288730u64 + 4);
        vm.wr(37, 0x10288730u64 + 4);
        Ok(0x10288730u64) }
        0x10288730 => { vm.decode(0x102036u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288734u64) }
        0x10288734 => { vm.decode(0x40020bb5u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10288738u64) }
        0x10288738 => { vm.decode(0x19189c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1028873cu64) }
        0x1028873c => { vm.decode(0x31916218u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 51303i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10288740u64) }
        0x10288740 => { vm.decode(0x422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10288744u64) }
        0x10288744 => { vm.decode(0x1b3014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288748u64) }
        0x10288748 => { vm.decode(0x5c4070d9u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = -23787i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1028874cu64) }
        0x1028874c => { vm.decode(0x10109019u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288750u64) }
        0x10288750 => { vm.decode(0x770d1419u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10288754u64) }
        0x10288754 => { vm.decode(0x6100036u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288758u64) }
        0x10288758 => { vm.decode(0x24000446u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028875cu64) }
        0x1028875c => { vm.decode(0x2220031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 17i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10288760u64).wrapping_add((imm * 4) as u64) } else { 0x10288760u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10288760u64) }
        0x10288760 => { vm.decode(0x9b305430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10288764u64) }
        0x10288764 => { vm.decode(0x8c04411du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 15496i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10288768u64) }
        0x1028876c => { vm.decode(0x8624735u32); let d = 1i16; let s = 4i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10288770u64) }
        0x10288770 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x10288774u64) }
        0x10288774 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288778u64) }
        0x10288778 => { vm.decode(0x74304675u32); let d = 3i16; let s = 3i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1028877cu64) }
        0x1028877c => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288780u64) }
        0x10288780 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10288784u64) }
        0x10288784 => { vm.decode(0x8c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288788u64) }
        0x10288788 => { vm.decode(0x8b2004f5u32); let d = 2i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1028878cu64) }
        0x1028878c => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10288790u64) }
        0x10288790 => { vm.decode(0x8903036u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10288794u64) }
        0x10288794 => { vm.decode(0x628243efu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10288798u64) }
        0x10288798 => { vm.decode(0x24000446u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x1028879cu64) }
        0x1028879c => { vm.decode(0x21c0a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1562i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102887a0u64).wrapping_add((imm * 4) as u64) } else { 0x102887a0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102887a0u64) }
        0x102887a0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102887a4u64) } }
        _ => Err(Error::UnknownPc(pc)),
    }
}
