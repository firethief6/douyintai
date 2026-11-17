#![cfg_attr(rustfmt, rustfmt::skip)]
// 由 scripts/medusa-vm/gen_native.py 生成，勿手改。
use super::super::Vm;
use super::super::Error;

#[allow(clippy::all, unused_parens, unused_variables, unused_mut)]
pub fn exec(vm: &mut Vm, pc: u64) -> Result<u64, Error> {
    match pc {
        0x10275e34 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275e38u64 + 4);
        vm.wr(37, 0x10275e38u64 + 4);
        Ok(0x10275e38u64) }
        0x10275e38 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275e3cu64) }
        0x10275e3c => { vm.decode(0xf10e251u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10275e40u64) }
        0x10275e40 => { vm.decode(0x82ce02fu32); let d = 2i16; let s = 18i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275e44u64) }
        0x10275e44 => { vm.decode(0x2028412fu32); let d = 3i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275e48u64) }
        0x10275e48 => { vm.decode(0x3b3195b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 3824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275e4cu64) }
        0x10275e4c => { vm.decode(0x108a812du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10275e50u64) }
        0x10275e50 => { vm.decode(0x6b3d1822u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10275e54u64) }
        0x10275e54 => { vm.decode(0xab21d5f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275e58u64) }
        0x10275e58 => { vm.decode(0x6a8ce3efu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275e5cu64) }
        0x10275e5c => { vm.decode(0xbb399530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 3800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275e60u64) }
        0x10275e60 => { vm.decode(0x2b319530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275e64u64) }
        0x10275e64 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275e68u64) }
        0x10275e68 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275e6cu64) }
        0x10275e6c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275e70u64 + 4);
        vm.wr(37, 0x10275e70u64 + 4);
        Ok(0x10275e70u64) }
        0x10275e70 => { vm.decode(0x666b8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3792i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275e74u64) }
        0x10275e74 => { vm.decode(0x2224606fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275e78u64) }
        0x10275e78 => { vm.decode(0xbb211db0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 3936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275e7cu64) }
        0x10275e7c => { vm.decode(0xc909875u32); let d = 21i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275e80u64) }
        0x10275e80 => { vm.decode(0x28ce3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275e84u64) }
        0x10275e84 => { vm.decode(0x2b391d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275e88u64) }
        0x10275e88 => { vm.decode(0x3b311c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275e8cu64) }
        0x10275e8c => { vm.decode(0x608cc3afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275e90u64) }
        0x10275e90 => { vm.decode(0x2882c36fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275e94u64) }
        0x10275e94 => { vm.decode(0x608a736fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275e98u64) }
        0x10275e98 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275e9cu64) }
        0x10275e9c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275ea0u64 + 4);
        vm.wr(37, 0x10275ea0u64 + 4);
        Ok(0x10275ea0u64) }
        0x10275ea0 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275ea4u64) }
        0x10275ea4 => { vm.decode(0x2820402fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275ea8u64) }
        0x10275ea8 => { vm.decode(0x9d0854b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275eacu64) }
        0x10275eac => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275eb0u64) }
        0x10275eb0 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275eb4u64) }
        0x10275eb4 => { vm.decode(0x688243efu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275eb8u64) }
        0x10275eb8 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275ebcu64) }
        0x10275ebc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275ec0u64 + 4);
        vm.wr(37, 0x10275ec0u64 + 4);
        Ok(0x10275ec0u64) }
        0x10275ec0 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275ec4u64) }
        0x10275ec4 => { vm.decode(0x2820402fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275ec8u64) }
        0x10275ec8 => { vm.decode(0x8d0855f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275eccu64) }
        0x10275ecc => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275ed0u64) }
        0x10275ed0 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275ed4u64) }
        0x10275ed4 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275ed8u64) }
        0x10275ed8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275edcu64 + 4);
        vm.wr(37, 0x10275edcu64 + 4);
        Ok(0x10275edcu64) }
        0x10275edc => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275ee0u64) }
        0x10275ee0 => { vm.decode(0x8909875u32); let d = 21i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275ee4u64) }
        0x10275ee4 => { vm.decode(0x2284732fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275ee8u64) }
        0x10275ee8 => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10275eecu64) }
        0x10275eec => { vm.decode(0x391814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275ef0u64) }
        0x10275ef0 => { vm.decode(0xd9c4211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10275ef4u64) }
        0x10275ef4 => { vm.decode(0x4b353462u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2524i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10275ef8u64) }
        0x10275ef8 => { vm.decode(0x2884532fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275efcu64) }
        0x10275efc => { vm.decode(0x19191c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275f00u64) }
        0x10275f00 => { vm.decode(0x2a80732fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275f04u64) }
        0x10275f04 => { vm.decode(0x19111c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275f08u64) }
        0x10275f08 => { vm.decode(0x2880612fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275f0cu64) }
        0x10275f0c => { vm.decode(0x888c3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275f10u64) }
        0x10275f10 => { vm.decode(0x2b299530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275f14u64) }
        0x10275f14 => { vm.decode(0x601b1519u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2272i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275f18u64) }
        0x10275f18 => { vm.decode(0xb381db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275f1cu64) }
        0x10275f1c => { vm.decode(0x2b219530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275f20u64) }
        0x10275f20 => { vm.decode(0x3b1994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275f24u64) }
        0x10275f24 => { vm.decode(0x282732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1048i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275f28u64) }
        0x10275f28 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275f2cu64) }
        0x10275f2c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275f30u64 + 4);
        vm.wr(37, 0x10275f30u64 + 4);
        Ok(0x10275f30u64) }
        0x10275f30 => { vm.decode(0x667f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3768i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275f34u64) }
        0x10275f34 => { vm.decode(0x604b1519u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2256i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275f38u64) }
        0x10275f38 => { vm.decode(0x2084616fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275f3cu64) }
        0x10275f3c => { vm.decode(0x288ce3efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275f40u64) }
        0x10275f40 => { vm.decode(0x3b1195b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 3760i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275f44u64) }
        0x10275f44 => { vm.decode(0x9209d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275f48u64) }
        0x10275f48 => { vm.decode(0x2b0995b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275f4cu64) }
        0x10275f4c => { vm.decode(0x3b0194b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275f50u64) }
        0x10275f50 => { vm.decode(0x86532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275f54u64) }
        0x10275f54 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275f58u64) }
        0x10275f58 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275f5cu64 + 4);
        vm.wr(37, 0x10275f5cu64 + 4);
        Ok(0x10275f5cu64) }
        0x10275f5c => { vm.decode(0x663b8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3744i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275f60u64) }
        0x10275f60 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275f64u64) }
        0x10275f64 => { vm.decode(0x9299430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275f68u64) }
        0x10275f68 => { vm.decode(0x9219430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275f6cu64) }
        0x10275f6c => { vm.decode(0x2084c16fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275f70u64) }
        0x10275f70 => { vm.decode(0xac8436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275f74u64) }
        0x10275f74 => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275f78u64) }
        0x10275f78 => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275f7cu64) }
        0x10275f7c => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275f80u64) }
        0x10275f80 => { vm.decode(0x4088532fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275f84u64) }
        0x10275f84 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275f88u64) }
        0x10275f88 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275f8cu64 + 4);
        vm.wr(37, 0x10275f8cu64 + 4);
        Ok(0x10275f8cu64) }
        0x10275f8c => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275f90u64) }
        0x10275f90 => { vm.decode(0x2ac0436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275f94u64) }
        0x10275f94 => { vm.decode(0x19219430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275f98u64) }
        0x10275f98 => { vm.decode(0xac8636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275f9cu64) }
        0x10275f9c => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10275fa0u64).wrapping_add((imm * 4) as u64) } else { 0x10275fa0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10275fa0u64) }
        0x10275fa0 => { vm.decode(0x9299530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275fa4u64) }
        0x10275fa4 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10275fa8u64) }
        0x10275fa8 => { vm.decode(0x8003036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10275facu64) }
        0x10275fac => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10275fb0u64) }
        0x10275fb0 => { vm.decode(0x2a80c3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275fb4u64) }
        0x10275fb4 => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10275fb8u64) }
        0x10275fb8 => { vm.decode(0x1b209430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275fbcu64) }
        0x10275fbc => { vm.decode(0x2084e16fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275fc0u64) }
        0x10275fc0 => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275fc4u64) }
        0x10275fc4 => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10275fc8u64) }
        0x10275fc8 => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275fccu64) }
        0x10275fcc => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10275fd0u64) }
        0x10275fd0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10275fd4u64 + 4);
        vm.wr(37, 0x10275fd4u64 + 4);
        Ok(0x10275fd4u64) }
        0x10275fd4 => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10275fd8u64) }
        0x10275fd8 => { vm.decode(0x28c0436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275fdcu64) }
        0x10275fdc => { vm.decode(0x8c0027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 20i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10275fe0u64).wrapping_add((imm * 4) as u64) } else { 0x10275fe0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10275fe0u64) }
        0x10275fe0 => { vm.decode(0x8102036u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10275fe4u64) }
        0x10275fe4 => { vm.decode(0x28c0436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275fe8u64) }
        0x10275fe8 => { vm.decode(0xc008091u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10275fecu64) }
        0x10275fec => { vm.decode(0xc0027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10275ff0u64).wrapping_add((imm * 4) as u64) } else { 0x10275ff0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10275ff0u64) }
        0x10275ff0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10275ff4u64) } }
        0x10275ff4 => { vm.decode(0x28c0436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275ff8u64) }
        0x10275ff8 => { vm.decode(0x2200012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10275ffcu64) }
        0x10275ffc => { vm.decode(0x840227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 12i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10276000u64).wrapping_add((imm * 4) as u64) } else { 0x10276000u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10276000u64) }
        0x10276000 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10276004u64) } }
        0x10276004 => { vm.decode(0x2a80e12fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276008u64) }
        0x10276008 => { vm.decode(0x888c3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027600cu64) }
        0x1027600c => { vm.decode(0x2b2915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276010u64) }
        0x10276010 => { vm.decode(0x8c8436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276014u64) }
        0x10276014 => { vm.decode(0x2b2115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276018u64) }
        0x10276018 => { vm.decode(0x3b391430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027601cu64) }
        0x1027601c => { vm.decode(0x88243efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276020u64) }
        0x10276020 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276024u64) }
        0x10276024 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276028u64 + 4);
        vm.wr(37, 0x10276028u64 + 4);
        Ok(0x10276028u64) }
        0x10276028 => { vm.decode(0x666d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3672i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027602cu64) }
        0x1027602c => { vm.decode(0xa1c5b63eu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 3696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10276030u64) }
        0x10276030 => { vm.decode(0x130db28eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -14614528i64;
        vm.st(d, imm as u64);
        Ok(0x10276034u64) }
        0x10276034 => { vm.decode(0x23450bd8u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 26488i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10276038u64) }
        0x10276038 => { vm.decode(0x1b2094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027603cu64) }
        0x1027603c => { vm.decode(0x2080732fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1032i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276040u64) }
        0x10276040 => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10276044u64) }
        0x10276044 => { vm.decode(0x1b109c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276048u64) }
        0x10276048 => { vm.decode(0x2084c16fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027604cu64) }
        0x1027604c => { vm.decode(0x8c8436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276050u64) }
        0x10276050 => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276054u64) }
        0x10276054 => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276058u64) }
        0x10276058 => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027605cu64) }
        0x1027605c => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276060u64) }
        0x10276060 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276064u64) }
        0x10276064 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276068u64 + 4);
        vm.wr(37, 0x10276068u64 + 4);
        Ok(0x10276068u64) }
        0x10276068 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027606cu64) }
        0x1027606c => { vm.decode(0x8c8636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276070u64) }
        0x10276070 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10276074u64).wrapping_add((imm * 4) as u64) } else { 0x10276074u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10276074u64) }
        0x10276074 => { vm.decode(0x200208e0u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) ^ imm as u64;
        vm.st(d, v);
        Ok(0x10276078u64) }
        0x10276078 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027607cu64) }
        0x1027607c => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10276080u64) }
        0x10276080 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10276084u64) }
        0x10276084 => { vm.decode(0x8c8636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276088u64) }
        0x10276088 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027608cu64) }
        0x1027608c => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x10276090u64).wrapping_add((imm * 4) as u64) } else { 0x10276090u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10276090u64) }
        0x10276090 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10276094u64) } }
        0x10276094 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10276098u64).wrapping_add((imm * 4) as u64) } else { 0x10276098u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10276098u64) }
        0x102760ec => { vm.decode(0x288443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102760f0u64) }
        0x102760f0 => { vm.decode(0x8c732fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102760f4u64) }
        0x102760f4 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102760f8u64) }
        0x102760f8 => { vm.decode(0x39081430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1032i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102760fcu64) }
        0x102760fc => { vm.decode(0x2a80c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276100u64) }
        0x10276100 => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10276104u64) }
        0x10276104 => { vm.decode(0x1b309c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276108u64) }
        0x10276108 => { vm.decode(0x2a80e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027610cu64) }
        0x1027610c => { vm.decode(0x612604f5u32); let d = 20i16; let s = 22i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10276110u64) }
        0x10276110 => { vm.decode(0x2084e16fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276114u64) }
        0x10276114 => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276118u64) }
        0x10276118 => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027611cu64) }
        0x1027611c => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276120u64) }
        0x10276120 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276124u64) }
        0x10276124 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276128u64 + 4);
        vm.wr(37, 0x10276128u64 + 4);
        Ok(0x10276128u64) }
        0x10276128 => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027612cu64) }
        0x1027612c => { vm.decode(0xa0812du32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10276130u64) }
        0x10276130 => { vm.decode(0x48a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 11i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10276134u64).wrapping_add((imm * 4) as u64) } else { 0x10276134u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10276134u64) }
        0x10276134 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10276138u64) } }
        0x10276138 => { vm.decode(0x2284616fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027613cu64) }
        0x1027613c => { vm.decode(0xac8436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276140u64) }
        0x10276140 => { vm.decode(0x2b211db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276144u64) }
        0x10276144 => { vm.decode(0x28ce3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276148u64) }
        0x10276148 => { vm.decode(0x2b391d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027614cu64) }
        0x1027614c => { vm.decode(0x3b311c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276150u64) }
        0x10276150 => { vm.decode(0x66698559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3920i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276154u64) }
        0x10276154 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276158u64) }
        0x10276158 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027615cu64 + 4);
        vm.wr(37, 0x1027615cu64 + 4);
        Ok(0x1027615cu64) }
        0x1027615c => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276160u64) }
        0x10276160 => { vm.decode(0xa90855b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276164u64) }
        0x10276164 => { vm.decode(0x2ac4436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276168u64) }
        0x10276168 => { vm.decode(0x19091cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027616cu64) }
        0x1027616c => { vm.decode(0xd1cc051u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10276170u64) }
        0x10276170 => { vm.decode(0x2b051062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10276174u64) }
        0x10276174 => { vm.decode(0x2a80412fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276178u64) }
        0x10276178 => { vm.decode(0x3b1914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027617cu64) }
        0x1027617c => { vm.decode(0x667d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3640i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276180u64) }
        0x10276180 => { vm.decode(0x8b38dc30u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276184u64) }
        0x10276184 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276188u64) }
        0x10276188 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027618cu64 + 4);
        vm.wr(37, 0x1027618cu64 + 4);
        Ok(0x1027618cu64) }
        0x1027618c => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276190u64) }
        0x10276190 => { vm.decode(0xf904251u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10276194u64) }
        0x10276194 => { vm.decode(0xbc0fcff6u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10276198u64) }
        0x10276198 => { vm.decode(0x66398419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027619cu64) }
        0x1027619c => { vm.decode(0x2b051462u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2340i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102761a0u64) }
        0x102761a0 => { vm.decode(0x488ee3efu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102761a4u64) }
        0x102761a4 => { vm.decode(0x20a0c02fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102761a8u64) }
        0x102761a8 => { vm.decode(0x421d5862u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102761acu64) }
        0x102761ac => { vm.decode(0x3b201c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102761b0u64) }
        0x102761b0 => { vm.decode(0x88732fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1032i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102761b4u64) }
        0x102761b4 => { vm.decode(0x2b181db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102761b8u64) }
        0x102761b8 => { vm.decode(0x888c12fu32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102761bcu64) }
        0x102761bc => { vm.decode(0x40048019u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102761c0u64) }
        0x102761c0 => { vm.decode(0x3b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102761c4u64) }
        0x102761c4 => { vm.decode(0x2b0115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102761c8u64) }
        0x102761c8 => { vm.decode(0x6a80c3efu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102761ccu64) }
        0x102761cc => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102761d0u64) }
        0x102761d0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102761d4u64 + 4);
        vm.wr(37, 0x102761d4u64 + 4);
        Ok(0x102761d4u64) }
        0x102761d4 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102761d8u64) }
        0x102761d8 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102761dcu64) }
        0x102761dc => { vm.decode(0x91994b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102761e0u64) }
        0x102761e0 => { vm.decode(0x91194b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102761e4u64) }
        0x102761e4 => { vm.decode(0xf144a51u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102761e8u64) }
        0x102761e8 => { vm.decode(0x431d1862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102761ecu64) }
        0x102761ec => { vm.decode(0x2084c16fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102761f0u64) }
        0x102761f0 => { vm.decode(0x2ac602fu32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102761f4u64) }
        0x102761f4 => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102761f8u64) }
        0x102761f8 => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102761fcu64) }
        0x102761fc => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276200u64) }
        0x10276200 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276204u64) }
        0x10276204 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276208u64 + 4);
        vm.wr(37, 0x10276208u64 + 4);
        Ok(0x10276208u64) }
        0x10276208 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027620cu64) }
        0x1027620c => { vm.decode(0x22a4602fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276210u64) }
        0x10276210 => { vm.decode(0x191194b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276214u64) }
        0x10276214 => { vm.decode(0xa8406fu32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276218u64) }
        0x10276218 => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027621cu64).wrapping_add((imm * 4) as u64) } else { 0x1027621cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027621cu64) }
        0x1027621c => { vm.decode(0x91995b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276220u64) }
        0x10276220 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10276224u64) }
        0x10276224 => { vm.decode(0x8003036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10276228u64) }
        0x10276228 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027622cu64) }
        0x1027622c => { vm.decode(0xc901875u32); let d = 20i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276230u64) }
        0x10276230 => { vm.decode(0x2084e16fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276234u64) }
        0x10276234 => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276238u64) }
        0x10276238 => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027623cu64) }
        0x1027623c => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276240u64) }
        0x10276240 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276244u64) }
        0x10276244 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276248u64 + 4);
        vm.wr(37, 0x10276248u64 + 4);
        Ok(0x10276248u64) }
        0x10276248 => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027624cu64) }
        0x1027624c => { vm.decode(0x408a732fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 1032i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276250u64) }
        0x10276250 => { vm.decode(0xab185db0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 1848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276254u64) }
        0x10276254 => { vm.decode(0xd90e251u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10276258u64) }
        0x10276258 => { vm.decode(0x82002031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x1027625cu64).wrapping_add((imm * 4) as u64) } else { 0x1027625cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027625cu64) }
        0x1027625c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10276260u64) } }
        0x10276260 => { vm.decode(0x2880e12fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276264u64) }
        0x10276264 => { vm.decode(0x88d3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276268u64) }
        0x10276268 => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027626cu64) }
        0x1027626c => { vm.decode(0x2b0915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276270u64) }
        0x10276270 => { vm.decode(0x3b0114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276274u64) }
        0x10276274 => { vm.decode(0x66398419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276278u64) }
        0x10276278 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027627cu64) }
        0x1027627c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276280u64 + 4);
        vm.wr(37, 0x10276280u64 + 4);
        Ok(0x10276280u64) }
        0x10276280 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276284u64) }
        0x10276284 => { vm.decode(0x688ce3efu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276288u64) }
        0x10276288 => { vm.decode(0x2020c12fu32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027628cu64) }
        0x1027628c => { vm.decode(0xbc0fcff6u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10276290u64) }
        0x10276290 => { vm.decode(0x421d5862u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10276294u64) }
        0x10276294 => { vm.decode(0x3b201c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276298u64) }
        0x10276298 => { vm.decode(0xab185db0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 1848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027629cu64) }
        0x1027629c => { vm.decode(0x888c12fu32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102762a0u64) }
        0x102762a0 => { vm.decode(0x40048019u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102762a4u64) }
        0x102762a4 => { vm.decode(0x3b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102762a8u64) }
        0x102762a8 => { vm.decode(0x2b0115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102762acu64) }
        0x102762ac => { vm.decode(0x66398419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102762b0u64) }
        0x102762b0 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102762b4u64) }
        0x102762b4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102762b8u64 + 4);
        vm.wr(37, 0x102762b8u64 + 4);
        Ok(0x102762b8u64) }
        0x102762b8 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102762bcu64) }
        0x102762bc => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102762c0u64) }
        0x102762c0 => { vm.decode(0x90994b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102762c4u64) }
        0x102762c4 => { vm.decode(0x90194b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102762c8u64) }
        0x102762c8 => { vm.decode(0xf144a51u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102762ccu64) }
        0x102762cc => { vm.decode(0x431d1862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102762d0u64) }
        0x102762d0 => { vm.decode(0x2084c16fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102762d4u64) }
        0x102762d4 => { vm.decode(0x22c416fu32); let d = 2i16; let s = 19i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102762d8u64) }
        0x102762d8 => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102762dcu64) }
        0x102762dc => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102762e0u64) }
        0x102762e0 => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102762e4u64) }
        0x102762e4 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102762e8u64) }
        0x102762e8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102762ecu64 + 4);
        vm.wr(37, 0x102762ecu64 + 4);
        Ok(0x102762ecu64) }
        0x102762ec => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102762f0u64) }
        0x102762f0 => { vm.decode(0x2224416fu32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102762f4u64) }
        0x102762f4 => { vm.decode(0x190194b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102762f8u64) }
        0x102762f8 => { vm.decode(0x22c616fu32); let d = 2i16; let s = 19i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102762fcu64) }
        0x102762fc => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10276300u64).wrapping_add((imm * 4) as u64) } else { 0x10276300u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10276300u64) }
        0x10276300 => { vm.decode(0x90995b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276304u64) }
        0x10276304 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10276308u64) }
        0x10276308 => { vm.decode(0x8003036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027630cu64) }
        0x1027630c => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10276310u64) }
        0x10276310 => { vm.decode(0x408e732fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276314u64) }
        0x10276314 => { vm.decode(0x2284c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276318u64) }
        0x10276318 => { vm.decode(0x616404f5u32); let d = 17i16; let s = 22i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027631cu64) }
        0x1027631c => { vm.decode(0x2084e16fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276320u64) }
        0x10276320 => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276324u64) }
        0x10276324 => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276328u64) }
        0x10276328 => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027632cu64) }
        0x1027632c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276330u64) }
        0x10276330 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276334u64 + 4);
        vm.wr(37, 0x10276334u64 + 4);
        Ok(0x10276334u64) }
        0x10276334 => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276338u64) }
        0x10276338 => { vm.decode(0x2080732fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1032i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027633cu64) }
        0x1027633c => { vm.decode(0x3b181cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276340u64) }
        0x10276340 => { vm.decode(0xd90e251u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10276344u64) }
        0x10276344 => { vm.decode(0x82002031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10276348u64).wrapping_add((imm * 4) as u64) } else { 0x10276348u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10276348u64) }
        0x10276348 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027634cu64) } }
        0x1027634c => { vm.decode(0x2880e12fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276350u64) }
        0x10276350 => { vm.decode(0x88d3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276354u64) }
        0x10276354 => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276358u64) }
        0x10276358 => { vm.decode(0x2b0915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027635cu64) }
        0x1027635c => { vm.decode(0x3b0114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276360u64) }
        0x10276360 => { vm.decode(0xa82c3efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276364u64) }
        0x10276364 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276368u64) }
        0x10276368 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027636cu64 + 4);
        vm.wr(37, 0x1027636cu64 + 4);
        Ok(0x1027636cu64) }
        0x1027636c => { vm.decode(0x66398419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276370u64) }
        0x10276370 => { vm.decode(0x2a80612fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276374u64) }
        0x10276374 => { vm.decode(0x688ce3efu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276378u64) }
        0x10276378 => { vm.decode(0xbb1195b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 3760i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027637cu64) }
        0x1027637c => { vm.decode(0x604b1419u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2192i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276380u64) }
        0x10276380 => { vm.decode(0xb081530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276384u64) }
        0x10276384 => { vm.decode(0x2b0995b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276388u64) }
        0x10276388 => { vm.decode(0x3b0194b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027638cu64) }
        0x1027638c => { vm.decode(0x86532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276390u64) }
        0x10276390 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276394u64) }
        0x10276394 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276398u64 + 4);
        vm.wr(37, 0x10276398u64 + 4);
        Ok(0x10276398u64) }
        0x10276398 => { vm.decode(0x663b8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3744i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027639cu64) }
        0x1027639c => { vm.decode(0x2a84c12fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102763a0u64) }
        0x102763a0 => { vm.decode(0x662d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3592i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102763a4u64) }
        0x102763a4 => { vm.decode(0xbb111530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 3600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102763a8u64) }
        0x102763a8 => { vm.decode(0x3b091430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102763acu64) }
        0x102763ac => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102763b0u64) }
        0x102763b0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102763b4u64 + 4);
        vm.wr(37, 0x102763b4u64 + 4);
        Ok(0x102763b4u64) }
        0x102763b4 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102763b8u64) }
        0x102763b8 => { vm.decode(0x605d1519u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2168i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102763bcu64) }
        0x102763bc => { vm.decode(0xf10ca51u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102763c0u64) }
        0x102763c0 => { vm.decode(0x43353022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102763c4u64) }
        0x102763c4 => { vm.decode(0x2880612fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102763c8u64) }
        0x102763c8 => { vm.decode(0x2888c3afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102763ccu64) }
        0x102763cc => { vm.decode(0x3b299530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 3784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102763d0u64) }
        0x102763d0 => { vm.decode(0xb301db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102763d4u64) }
        0x102763d4 => { vm.decode(0x2b219530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102763d8u64) }
        0x102763d8 => { vm.decode(0x3b1994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102763dcu64) }
        0x102763dc => { vm.decode(0x282732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1048i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102763e0u64) }
        0x102763e0 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102763e4u64) }
        0x102763e4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102763e8u64 + 4);
        vm.wr(37, 0x102763e8u64 + 4);
        Ok(0x102763e8u64) }
        0x102763e8 => { vm.decode(0x667f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3768i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102763ecu64) }
        0x102763ec => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102763f0u64) }
        0x102763f0 => { vm.decode(0x93114b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102763f4u64) }
        0x102763f4 => { vm.decode(0x92914b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102763f8u64) }
        0x102763f8 => { vm.decode(0x2084c16fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102763fcu64) }
        0x102763fc => { vm.decode(0xacc432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276400u64) }
        0x10276400 => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276404u64) }
        0x10276404 => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276408u64) }
        0x10276408 => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027640cu64) }
        0x1027640c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276410u64) }
        0x10276410 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276414u64 + 4);
        vm.wr(37, 0x10276414u64 + 4);
        Ok(0x10276414u64) }
        0x10276414 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276418u64) }
        0x10276418 => { vm.decode(0x2ac4432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027641cu64) }
        0x1027641c => { vm.decode(0x192914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276420u64) }
        0x10276420 => { vm.decode(0xacc632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276424u64) }
        0x10276424 => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10276428u64).wrapping_add((imm * 4) as u64) } else { 0x10276428u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10276428u64) }
        0x10276428 => { vm.decode(0x93115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027642cu64) }
        0x1027642c => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10276430u64) }
        0x10276430 => { vm.decode(0x8003036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10276434u64) }
        0x10276434 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10276438u64) }
        0x10276438 => { vm.decode(0x2084e16fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027643cu64) }
        0x1027643c => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276440u64) }
        0x10276440 => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276444u64) }
        0x10276444 => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276448u64) }
        0x10276448 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027644cu64) }
        0x1027644c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276450u64 + 4);
        vm.wr(37, 0x10276450u64 + 4);
        Ok(0x10276450u64) }
        0x10276450 => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276454u64) }
        0x10276454 => { vm.decode(0x8102036u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10276458u64) }
        0x10276458 => { vm.decode(0x20c4636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027645cu64) }
        0x1027645c => { vm.decode(0x8c0027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 20i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10276460u64).wrapping_add((imm * 4) as u64) } else { 0x10276460u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10276460u64) }
        0x10276460 => { vm.decode(0x4810c875u32); let d = 19i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276464u64) }
        0x10276464 => { vm.decode(0x20c4636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276468u64) }
        0x10276468 => { vm.decode(0xc008091u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027646cu64) }
        0x1027646c => { vm.decode(0xc0027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10276470u64).wrapping_add((imm * 4) as u64) } else { 0x10276470u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10276470u64) }
        0x10276470 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10276474u64) } }
        0x10276474 => { vm.decode(0x20c4636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276478u64) }
        0x10276478 => { vm.decode(0x2200012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027647cu64) }
        0x1027647c => { vm.decode(0x840227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 12i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10276480u64).wrapping_add((imm * 4) as u64) } else { 0x10276480u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10276480u64) }
        0x10276480 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10276484u64) } }
        0x10276484 => { vm.decode(0x2a80e12fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276488u64) }
        0x10276488 => { vm.decode(0x888c3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027648cu64) }
        0x1027648c => { vm.decode(0x2b2915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276490u64) }
        0x10276490 => { vm.decode(0xcc636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276494u64) }
        0x10276494 => { vm.decode(0x2b2115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276498u64) }
        0x10276498 => { vm.decode(0x3b391430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027649cu64) }
        0x1027649c => { vm.decode(0x88243efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102764a0u64) }
        0x102764a0 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102764a4u64) }
        0x102764a4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102764a8u64 + 4);
        vm.wr(37, 0x102764a8u64 + 4);
        Ok(0x102764a8u64) }
        0x102764a8 => { vm.decode(0x666d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3672i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102764acu64) }
        0x102764ac => { vm.decode(0xa1c5b63eu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 3696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102764b0u64) }
        0x102764b0 => { vm.decode(0x2084c16fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102764b4u64) }
        0x102764b4 => { vm.decode(0xcc636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102764b8u64) }
        0x102764b8 => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102764bcu64) }
        0x102764bc => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102764c0u64) }
        0x102764c0 => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102764c4u64) }
        0x102764c4 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102764c8u64) }
        0x102764c8 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102764ccu64) }
        0x102764cc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102764d0u64 + 4);
        vm.wr(37, 0x102764d0u64 + 4);
        Ok(0x102764d0u64) }
        0x102764d0 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102764d4u64) }
        0x102764d4 => { vm.decode(0x2cc436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102764d8u64) }
        0x102764d8 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102764dcu64).wrapping_add((imm * 4) as u64) } else { 0x102764dcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102764dcu64) }
        0x102764dc => { vm.decode(0x200208e0u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) ^ imm as u64;
        vm.st(d, v);
        Ok(0x102764e0u64) }
        0x102764e0 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102764e4u64) }
        0x102764e4 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102764e8u64) }
        0x102764e8 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102764ecu64) }
        0x102764ec => { vm.decode(0x2cc436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102764f0u64) }
        0x102764f0 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102764f4u64) }
        0x102764f4 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x102764f8u64).wrapping_add((imm * 4) as u64) } else { 0x102764f8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102764f8u64) }
        0x102764f8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102764fcu64) } }
        0x102764fc => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10276500u64).wrapping_add((imm * 4) as u64) } else { 0x10276500u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10276500u64) }
        0x10276554 => { vm.decode(0x2084e16fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276558u64) }
        0x10276558 => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027655cu64) }
        0x1027655c => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276560u64) }
        0x10276560 => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276564u64) }
        0x10276564 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276568u64) }
        0x10276568 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027656cu64 + 4);
        vm.wr(37, 0x1027656cu64 + 4);
        Ok(0x1027656cu64) }
        0x1027656c => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276570u64) }
        0x10276570 => { vm.decode(0xa0812du32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10276574u64) }
        0x10276574 => { vm.decode(0x848227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 13i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10276578u64).wrapping_add((imm * 4) as u64) } else { 0x10276578u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10276578u64) }
        0x10276578 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027657cu64) } }
        0x1027657c => { vm.decode(0x2284616fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276580u64) }
        0x10276580 => { vm.decode(0xacc432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276584u64) }
        0x10276584 => { vm.decode(0x2b211db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276588u64) }
        0x10276588 => { vm.decode(0x28cc3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027658cu64) }
        0x1027658c => { vm.decode(0x2b391d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276590u64) }
        0x10276590 => { vm.decode(0x3b311c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276594u64) }
        0x10276594 => { vm.decode(0x66698559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3920i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276598u64) }
        0x10276598 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027659cu64) }
        0x1027659c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102765a0u64 + 4);
        vm.wr(37, 0x102765a0u64 + 4);
        Ok(0x102765a0u64) }
        0x102765a0 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102765a4u64) }
        0x102765a4 => { vm.decode(0x2c24041du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 5212i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102765a8u64) }
        0x102765a8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102765acu64) } }
        0x102765ac => { vm.decode(0x92114b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102765b0u64) }
        0x1027691c => { vm.decode(0x28c0632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276920u64) }
        0x10276920 => { vm.decode(0x19111cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276924u64) }
        0x10276924 => { vm.decode(0x6884e3efu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276928u64) }
        0x10276928 => { vm.decode(0x2000612fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027692cu64) }
        0x1027692c => { vm.decode(0xc008091u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10276930u64) }
        0x10276930 => { vm.decode(0x2000071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10276934u64).wrapping_add((imm * 4) as u64) } else { 0x10276934u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10276934u64) }
        0x10276934 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10276938u64) }
        0x10276938 => { vm.decode(0x2800412fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027693cu64) }
        0x10276944 => { vm.decode(0x2084436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276948u64) }
        0x10276948 => { vm.decode(0x696404f5u32); let d = 19i16; let s = 22i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027694cu64) }
        0x1027694c => { vm.decode(0xf9c6891u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2548i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10276950u64) }
        0x10276950 => { vm.decode(0x800202f5u32); let d = 1i16; let s = 0i16; let s2 = 2i16; let imm = 0i64;
        if vm.ld(s2) != 0 {
            let v = vm.ld(s);
            vm.st(d, v);
        }
        Ok(0x10276954u64) }
        0x10276954 => { vm.decode(0x2b351062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10276958u64) }
        0x10276958 => { vm.decode(0x2b201c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027695cu64) }
        0x1027695c => { vm.decode(0x2b181cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276960u64) }
        0x10276960 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276964u64) }
        0x10276964 => { vm.decode(0xb9205430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276968u64) }
        0x10276968 => { vm.decode(0x2084c16fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027696cu64) }
        0x1027696c => { vm.decode(0x80c412fu32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276970u64) }
        0x10276970 => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276974u64) }
        0x10276974 => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276978u64) }
        0x10276978 => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027697cu64) }
        0x1027697c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276980u64) }
        0x10276980 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276984u64 + 4);
        vm.wr(37, 0x10276984u64 + 4);
        Ok(0x10276984u64) }
        0x10276984 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276988u64) }
        0x10276988 => { vm.decode(0x2804412fu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027698cu64) }
        0x1027698c => { vm.decode(0x3b181cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276990u64) }
        0x10276990 => { vm.decode(0x80c612fu32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276994u64) }
        0x10276994 => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10276998u64).wrapping_add((imm * 4) as u64) } else { 0x10276998u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10276998u64) }
        0x10276998 => { vm.decode(0x2b201d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027699cu64) }
        0x1027699c => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102769a0u64) }
        0x102769a0 => { vm.decode(0x8003036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102769a4u64) }
        0x102769a4 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102769a8u64) }
        0x102769a8 => { vm.decode(0x2080e3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102769acu64) }
        0x102769ac => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102769b0u64) }
        0x102769b0 => { vm.decode(0x1b081c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102769b4u64) }
        0x102769b4 => { vm.decode(0x2280532fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102769b8u64) }
        0x102769b8 => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102769bcu64) }
        0x102769bc => { vm.decode(0x39101430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102769c0u64) }
        0x102769c0 => { vm.decode(0x2084636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102769c4u64) }
        0x102769c4 => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102769c8u64) }
        0x102769c8 => { vm.decode(0x390014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102769ccu64) }
        0x102769cc => { vm.decode(0x2284436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102769d0u64) }
        0x102769d0 => { vm.decode(0x616404f5u32); let d = 17i16; let s = 22i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102769d4u64) }
        0x102769d4 => { vm.decode(0x2284636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102769d8u64) }
        0x102769d8 => { vm.decode(0x616604f5u32); let d = 21i16; let s = 22i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102769dcu64) }
        0x102769dc => { vm.decode(0x2880432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102769e0u64) }
        0x102769e0 => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102769e4u64) }
        0x102769e4 => { vm.decode(0x190014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102769e8u64) }
        0x102769e8 => { vm.decode(0x2a80432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102769ecu64) }
        0x102769ec => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102769f0u64) }
        0x102769f0 => { vm.decode(0x19109430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102769f4u64) }
        0x102769f4 => { vm.decode(0x2a80632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102769f8u64) }
        0x102769f8 => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102769fcu64) }
        0x102769fc => { vm.decode(0x19189430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276a00u64) }
        0x10276a00 => { vm.decode(0x2884432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276a04u64) }
        0x10276a04 => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10276a08u64) }
        0x10276a08 => { vm.decode(0x190094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276a0cu64) }
        0x10276a0c => { vm.decode(0x2884632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276a10u64) }
        0x10276a10 => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10276a14u64) }
        0x10276a14 => { vm.decode(0x190894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276a18u64) }
        0x10276a18 => { vm.decode(0x2a84432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276a1cu64) }
        0x10276a1c => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10276a20u64) }
        0x10276a20 => { vm.decode(0x191094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276a24u64) }
        0x10276a24 => { vm.decode(0x2a84632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276a28u64) }
        0x10276a28 => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10276a2cu64) }
        0x10276a2c => { vm.decode(0x19009430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276a30u64) }
        0x10276a30 => { vm.decode(0x208043afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276a34u64) }
        0x10276a34 => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10276a38u64) }
        0x10276a38 => { vm.decode(0x193814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276a3cu64) }
        0x10276a3c => { vm.decode(0x288463afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276a40u64) }
        0x10276a40 => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10276a44u64) }
        0x10276a44 => { vm.decode(0x193014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276a48u64) }
        0x10276a48 => { vm.decode(0x2880e3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276a4cu64) }
        0x10276a4c => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10276a50u64) }
        0x10276a50 => { vm.decode(0x192814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276a54u64) }
        0x10276a54 => { vm.decode(0x2a80e3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276a58u64) }
        0x10276a58 => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10276a5cu64) }
        0x10276a5c => { vm.decode(0x192014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276a60u64) }
        0x10276a60 => { vm.decode(0x2884c3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276a64u64) }
        0x10276a64 => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10276a68u64) }
        0x10276a68 => { vm.decode(0x19301430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276a6cu64) }
        0x10276a6c => { vm.decode(0x2a84c3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276a70u64) }
        0x10276a70 => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10276a74u64) }
        0x10276a74 => { vm.decode(0x1b009cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276a78u64) }
        0x10276a78 => { vm.decode(0x2a84e3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276a7cu64) }
        0x10276a7c => { vm.decode(0x692404f5u32); let d = 18i16; let s = 22i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10276a80u64) }
        0x10276a80 => { vm.decode(0x4082536fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276a84u64) }
        0x10276a84 => { vm.decode(0x2084e06fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276a88u64) }
        0x10276a88 => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276a8cu64) }
        0x10276a8c => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276a90u64) }
        0x10276a90 => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276a94u64) }
        0x10276a94 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276a98u64) }
        0x10276a98 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276a9cu64 + 4);
        vm.wr(37, 0x10276a9cu64 + 4);
        Ok(0x10276a9cu64) }
        0x10276a9c => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276aa0u64) }
        0x10276aa0 => { vm.decode(0x2884402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276aa4u64) }
        0x10276aa4 => { vm.decode(0x627b8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276aa8u64) }
        0x10276aa8 => { vm.decode(0x605d9419u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2104i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276aacu64) }
        0x10276aac => { vm.decode(0x28cf3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276ab0u64) }
        0x10276ab0 => { vm.decode(0x2b011530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276ab4u64) }
        0x10276ab4 => { vm.decode(0x1b001530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276ab8u64) }
        0x10276ab8 => { vm.decode(0x39399db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 3576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276abcu64) }
        0x10276abc => { vm.decode(0x39319cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276ac0u64) }
        0x10276ac0 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276ac4u64) }
        0x10276ac4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276ac8u64 + 4);
        vm.wr(37, 0x10276ac8u64 + 4);
        Ok(0x10276ac8u64) }
        0x10276ac8 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276accu64) }
        0x10276acc => { vm.decode(0x2084c06fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276ad0u64) }
        0x10276ad0 => { vm.decode(0x28cf3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276ad4u64) }
        0x10276ad4 => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276ad8u64) }
        0x10276ad8 => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276adcu64) }
        0x10276adc => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276ae0u64) }
        0x10276ae0 => { vm.decode(0x82532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276ae4u64) }
        0x10276ae4 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276ae8u64) }
        0x10276ae8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276aecu64 + 4);
        vm.wr(37, 0x10276aecu64 + 4);
        Ok(0x10276aecu64) }
        0x10276aec => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276af0u64) }
        0x10276af0 => { vm.decode(0x88d3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276af4u64) }
        0x10276af4 => { vm.decode(0x6a88e3efu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276af8u64) }
        0x10276af8 => { vm.decode(0x8888a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 31i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10276afcu64).wrapping_add((imm * 4) as u64) } else { 0x10276afcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10276afcu64) }
        0x10276afc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10276b00u64) } }
        0x10276b00 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10276b04u64) }
        0x10276b04 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10276b08u64) }
        0x10276b08 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10276b0cu64) }
        0x10276b0c => { vm.decode(0x88d3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276b10u64) }
        0x10276b10 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10276b14u64) }
        0x10276b14 => { vm.decode(0x10200084u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x10276b18u64).wrapping_add((imm * 4) as u64) } else { 0x10276b18u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10276b18u64) }
        0x10276b18 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10276b1cu64) } }
        0x10276b1c => { vm.decode(0x8008227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 9i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10276b20u64).wrapping_add((imm * 4) as u64) } else { 0x10276b20u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10276b20u64) }
        0x10276b78 => { vm.decode(0x408a536fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276b7cu64) }
        0x10276b7c => { vm.decode(0x20a4e06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276b80u64) }
        0x10276b80 => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276b84u64) }
        0x10276b84 => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276b88u64) }
        0x10276b88 => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276b8cu64) }
        0x10276b8c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276b90u64) }
        0x10276b90 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276b94u64 + 4);
        vm.wr(37, 0x10276b94u64 + 4);
        Ok(0x10276b94u64) }
        0x10276b94 => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276b98u64) }
        0x10276b98 => { vm.decode(0x626b8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3536i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276b9cu64) }
        0x10276b9c => { vm.decode(0x601d1419u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2088i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276ba0u64) }
        0x10276ba0 => { vm.decode(0x22c4632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276ba4u64) }
        0x10276ba4 => { vm.decode(0x2200012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276ba8u64) }
        0x10276ba8 => { vm.decode(0x19191cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276bacu64) }
        0x10276bac => { vm.decode(0x28a4602fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276bb0u64) }
        0x10276bb0 => { vm.decode(0xb0895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276bb4u64) }
        0x10276bb4 => { vm.decode(0x29399d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276bb8u64) }
        0x10276bb8 => { vm.decode(0x39319c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276bbcu64) }
        0x10276bbc => { vm.decode(0x9918d4b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276bc0u64) }
        0x10276bc0 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276bc4u64) }
        0x10276bc4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276bc8u64 + 4);
        vm.wr(37, 0x10276bc8u64 + 4);
        Ok(0x10276bc8u64) }
        0x10276bc8 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276bccu64) }
        0x10276bcc => { vm.decode(0x667d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3640i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276bd0u64) }
        0x10276bd0 => { vm.decode(0x20c4632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276bd4u64) }
        0x10276bd4 => { vm.decode(0x2200012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276bd8u64) }
        0x10276bd8 => { vm.decode(0x19211c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276bdcu64) }
        0x10276bdc => { vm.decode(0x2aa4402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276be0u64) }
        0x10276be0 => { vm.decode(0x3b1914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276be4u64) }
        0x10276be4 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276be8u64) }
        0x10276be8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276becu64 + 4);
        vm.wr(37, 0x10276becu64 + 4);
        Ok(0x10276becu64) }
        0x10276bec => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276bf0u64) }
        0x10276bf0 => { vm.decode(0x622b8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3520i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276bf4u64) }
        0x10276bf4 => { vm.decode(0xf904251u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10276bf8u64) }
        0x10276bf8 => { vm.decode(0x4b053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10276bfcu64) }
        0x10276bfc => { vm.decode(0x2aa4602fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276c00u64) }
        0x10276c00 => { vm.decode(0x39219c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276c04u64) }
        0x10276c04 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276c08u64) }
        0x10276c08 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276c0cu64 + 4);
        vm.wr(37, 0x10276c0cu64 + 4);
        Ok(0x10276c0cu64) }
        0x10276c0c => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276c10u64) }
        0x10276c10 => { vm.decode(0x8000a019u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 6i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276c14u64) }
        0x10276c14 => { vm.decode(0x62098219u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 4096i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276c18u64) }
        0x10276c18 => { vm.decode(0x19385430u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276c1cu64) }
        0x10276c1c => { vm.decode(0x657f0159u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276c20u64) }
        0x10276c20 => { vm.decode(0x501019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 48i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276c24u64) }
        0x10276c24 => { vm.decode(0x40b019u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276c28u64) }
        0x10276c28 => { vm.decode(0x8c2f36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 3528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276c2cu64) }
        0x10276c2c => { vm.decode(0x9295c30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276c30u64) }
        0x10276c30 => { vm.decode(0x88a632fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276c34u64) }
        0x10276c34 => { vm.decode(0x508191u32); let d = 4i16; let s = 6i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10276c38u64) }
        0x10276c38 => { vm.decode(0x22a0082fu32); let d = 9i16; let s = 6i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276c3cu64) }
        0x10276c3c => { vm.decode(0x88e536fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 1248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276c40u64) }
        0x10276c40 => { vm.decode(0x656f06d7u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 2548i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = u32::from_le_bytes(b.try_into().unwrap());
        vm.st(d, v as u64);
        Ok(0x10276c44u64) }
        0x10276c44 => { vm.decode(0x19211cf0u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 2400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276c48u64) }
        0x10276c48 => { vm.decode(0x19315db0u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 2416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276c4cu64) }
        0x10276c4c => { vm.decode(0x901dd30u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 2432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276c50u64) }
        0x10276c50 => { vm.decode(0x88a736fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 1224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276c54u64) }
        0x10276c54 => { vm.decode(0x911dd30u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 2448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276c58u64) }
        0x10276c58 => { vm.decode(0x19099c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276c5cu64) }
        0x10276c5c => { vm.decode(0x19391db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276c60u64) }
        0x10276c60 => { vm.decode(0x9395c30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 2392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276c64u64) }
        0x10276c64 => { vm.decode(0x6a151062u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10276c68u64) }
        0x10276c68 => { vm.decode(0x22a460efu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276c6cu64) }
        0x10276c6c => { vm.decode(0xd181530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276c70u64) }
        0x10276c70 => { vm.decode(0x8d101530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 4112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276c74u64) }
        0x10276c74 => { vm.decode(0xb089c70u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276c78u64) }
        0x10276c78 => { vm.decode(0xd081470u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 4104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276c7cu64) }
        0x10276c7c => { vm.decode(0x1d001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276c80u64) }
        0x10276c80 => { vm.decode(0x882e3efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276c84u64) }
        0x10276c84 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276c88u64) }
        0x10276c88 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276c8cu64 + 4);
        vm.wr(37, 0x10276c8cu64 + 4);
        Ok(0x10276c8cu64) }
        0x10276c8c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10276c90u64) } }
        0x10276c90 => { vm.decode(0x707b9459u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 3504i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276c94u64) }
        0x10276c94 => { vm.decode(0x20a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276c98u64) }
        0x10276c98 => { vm.decode(0x39119cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276c9cu64) }
        0x10276c9c => { vm.decode(0x6084532fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276ca0u64) }
        0x10276ca0 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276ca4u64) }
        0x10276ca4 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276ca8u64) }
        0x10276ca8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276cacu64 + 4);
        vm.wr(37, 0x10276cacu64 + 4);
        Ok(0x10276cacu64) }
        0x10276cac => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276cb0u64) }
        0x10276cb0 => { vm.decode(0x667f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3768i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276cb4u64) }
        0x10276cb4 => { vm.decode(0x1b10dcb0u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276cb8u64) }
        0x10276cb8 => { vm.decode(0x2ac4f32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276cbcu64) }
        0x10276cbc => { vm.decode(0x6002012fu32); let d = 21i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276cc0u64) }
        0x10276cc0 => { vm.decode(0x28a0602fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276cc4u64) }
        0x10276cc4 => { vm.decode(0x88cc3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276cc8u64) }
        0x10276cc8 => { vm.decode(0x2b299530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276cccu64) }
        0x10276ccc => { vm.decode(0xab21d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276cd0u64) }
        0x10276cd0 => { vm.decode(0x3b1994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276cd4u64) }
        0x10276cd4 => { vm.decode(0x282732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1048i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276cd8u64) }
        0x10276cd8 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276cdcu64) }
        0x10276cdc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276ce0u64 + 4);
        vm.wr(37, 0x10276ce0u64 + 4);
        Ok(0x10276ce0u64) }
        0x10276ce0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10276ce4u64) } }
        0x10276ce4 => { vm.decode(0x626b8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3472i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276ce8u64) }
        0x10276ce8 => { vm.decode(0x1b00dcb0u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276cecu64) }
        0x10276cec => { vm.decode(0x20a0602fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276cf0u64) }
        0x10276cf0 => { vm.decode(0xa901ddf0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276cf4u64) }
        0x10276cf4 => { vm.decode(0xb919dc30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 3480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276cf8u64) }
        0x10276cf8 => { vm.decode(0x39119c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276cfcu64) }
        0x10276cfc => { vm.decode(0x282532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276d00u64) }
        0x10276d00 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276d04u64) }
        0x10276d04 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276d08u64 + 4);
        vm.wr(37, 0x10276d08u64 + 4);
        Ok(0x10276d08u64) }
        0x10276d08 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10276d0cu64) } }
        0x10276d0c => { vm.decode(0x70191219u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276d10u64) }
        0x10276d10 => { vm.decode(0xf1ce051u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10276d14u64) }
        0x10276d14 => { vm.decode(0xb053022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10276d18u64) }
        0x10276d18 => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276d1cu64) }
        0x10276d1c => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276d20u64) }
        0x10276d20 => { vm.decode(0x8d0855f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276d24u64) }
        0x10276d24 => { vm.decode(0x4286532fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276d28u64) }
        0x10276d28 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276d2cu64) }
        0x10276d2c => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276d30u64) }
        0x10276d30 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276d34u64) }
        0x10276d34 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276d38u64 + 4);
        vm.wr(37, 0x10276d38u64 + 4);
        Ok(0x10276d38u64) }
        0x10276d38 => { vm.decode(0x8b189d30u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276d3cu64) }
        0x10276d3c => { vm.decode(0x20a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276d40u64) }
        0x10276d40 => { vm.decode(0x39119cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276d44u64) }
        0x10276d44 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276d48u64) }
        0x10276d48 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276d4cu64) }
        0x10276d4c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276d50u64 + 4);
        vm.wr(37, 0x10276d50u64 + 4);
        Ok(0x10276d50u64) }
        0x10276d50 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276d54u64) }
        0x10276d54 => { vm.decode(0x2ac4f32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276d58u64) }
        0x10276d58 => { vm.decode(0x600a012fu32); let d = 23i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276d5cu64) }
        0x10276d5c => { vm.decode(0x28a0602fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276d60u64) }
        0x10276d60 => { vm.decode(0x288436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276d64u64) }
        0x10276d64 => { vm.decode(0x2b299530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276d68u64) }
        0x10276d68 => { vm.decode(0xab21d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276d6cu64) }
        0x10276d6c => { vm.decode(0x3b1994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276d70u64) }
        0x10276d70 => { vm.decode(0x282732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1048i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276d74u64) }
        0x10276d74 => { vm.decode(0x6a86c3afu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276d78u64) }
        0x10276d78 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276d7cu64) }
        0x10276d7c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276d80u64 + 4);
        vm.wr(37, 0x10276d80u64 + 4);
        Ok(0x10276d80u64) }
        0x10276d80 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276d84u64) }
        0x10276d84 => { vm.decode(0x20a0602fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276d88u64) }
        0x10276d88 => { vm.decode(0xa901ddf0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276d8cu64) }
        0x10276d8c => { vm.decode(0xb919dd30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 3480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276d90u64) }
        0x10276d90 => { vm.decode(0x39119c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276d94u64) }
        0x10276d94 => { vm.decode(0x282532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276d98u64) }
        0x10276d98 => { vm.decode(0x2886c3afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276d9cu64) }
        0x10276d9c => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276da0u64) }
        0x10276da0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276da4u64 + 4);
        vm.wr(37, 0x10276da4u64 + 4);
        Ok(0x10276da4u64) }
        0x10276da4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10276da8u64) } }
        0x10276da8 => { vm.decode(0xf1ce051u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10276dacu64) }
        0x10276dac => { vm.decode(0xb153022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10276db0u64) }
        0x10276db0 => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276db4u64) }
        0x10276db4 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276db8u64) }
        0x10276db8 => { vm.decode(0x8d0855f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276dbcu64) }
        0x10276dbc => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276dc0u64) }
        0x10276dc0 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276dc4u64) }
        0x10276dc4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276dc8u64 + 4);
        vm.wr(37, 0x10276dc8u64 + 4);
        Ok(0x10276dc8u64) }
        0x10276dc8 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276dccu64) }
        0x10276dcc => { vm.decode(0x20a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276dd0u64) }
        0x10276dd0 => { vm.decode(0x39119cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276dd4u64) }
        0x10276dd4 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276dd8u64) }
        0x10276dd8 => { vm.decode(0x9b189db0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276ddcu64) }
        0x10276ddc => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276de0u64) }
        0x10276de0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276de4u64 + 4);
        vm.wr(37, 0x10276de4u64 + 4);
        Ok(0x10276de4u64) }
        0x10276de4 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276de8u64) }
        0x10276de8 => { vm.decode(0x2ac4f32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276decu64) }
        0x10276dec => { vm.decode(0x600a012fu32); let d = 23i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276df0u64) }
        0x10276df0 => { vm.decode(0x28a0602fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276df4u64) }
        0x10276df4 => { vm.decode(0x8c436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276df8u64) }
        0x10276df8 => { vm.decode(0x2b299530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276dfcu64) }
        0x10276dfc => { vm.decode(0xab21d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276e00u64) }
        0x10276e00 => { vm.decode(0x3b1994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276e04u64) }
        0x10276e04 => { vm.decode(0x4282732fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1048i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276e08u64) }
        0x10276e08 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276e0cu64) }
        0x10276e0c => { vm.decode(0x4c104875u32); let d = 18i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276e10u64) }
        0x10276e10 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276e14u64) }
        0x10276e14 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276e18u64 + 4);
        vm.wr(37, 0x10276e18u64 + 4);
        Ok(0x10276e18u64) }
        0x10276e18 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276e1cu64) }
        0x10276e1c => { vm.decode(0x20a0602fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276e20u64) }
        0x10276e20 => { vm.decode(0xa901ddf0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276e24u64) }
        0x10276e24 => { vm.decode(0xb919dd30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 3480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276e28u64) }
        0x10276e28 => { vm.decode(0x39119c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276e2cu64) }
        0x10276e2c => { vm.decode(0x6282532fu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 1040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276e30u64) }
        0x10276e30 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276e34u64) }
        0x10276e34 => { vm.decode(0x6884c3afu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276e38u64) }
        0x10276e38 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276e3cu64) }
        0x10276e3c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276e40u64 + 4);
        vm.wr(37, 0x10276e40u64 + 4);
        Ok(0x10276e40u64) }
        0x10276e40 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276e44u64) }
        0x10276e44 => { vm.decode(0xf1ce051u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10276e48u64) }
        0x10276e48 => { vm.decode(0xb053422u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2068i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10276e4cu64) }
        0x10276e4c => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276e50u64) }
        0x10276e50 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276e54u64) }
        0x10276e54 => { vm.decode(0x8d0855f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276e58u64) }
        0x10276e58 => { vm.decode(0x286532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276e5cu64) }
        0x10276e5c => { vm.decode(0x2a82e3afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276e60u64) }
        0x10276e60 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276e64u64) }
        0x10276e64 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276e68u64 + 4);
        vm.wr(37, 0x10276e68u64 + 4);
        Ok(0x10276e68u64) }
        0x10276e68 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10276e6cu64) } }
        0x10276e6c => { vm.decode(0x20a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276e70u64) }
        0x10276e70 => { vm.decode(0x39119cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276e74u64) }
        0x10276e74 => { vm.decode(0x86532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276e78u64) }
        0x10276e78 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276e7cu64) }
        0x10276e7c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276e80u64 + 4);
        vm.wr(37, 0x10276e80u64 + 4);
        Ok(0x10276e80u64) }
        0x10276e80 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276e84u64) }
        0x10276e84 => { vm.decode(0x2ac4f32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276e88u64) }
        0x10276e88 => { vm.decode(0x600a012fu32); let d = 23i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276e8cu64) }
        0x10276e8c => { vm.decode(0x28a0602fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276e90u64) }
        0x10276e90 => { vm.decode(0x8c636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276e94u64) }
        0x10276e94 => { vm.decode(0x2b299530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276e98u64) }
        0x10276e98 => { vm.decode(0xab21d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276e9cu64) }
        0x10276e9c => { vm.decode(0x3b1994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276ea0u64) }
        0x10276ea0 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276ea4u64) }
        0x10276ea4 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276ea8u64) }
        0x10276ea8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276eacu64 + 4);
        vm.wr(37, 0x10276eacu64 + 4);
        Ok(0x10276eacu64) }
        0x10276eac => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276eb0u64) }
        0x10276eb0 => { vm.decode(0x20a0602fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276eb4u64) }
        0x10276eb4 => { vm.decode(0xa901ddf0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276eb8u64) }
        0x10276eb8 => { vm.decode(0xb919dd30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 3480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276ebcu64) }
        0x10276ebc => { vm.decode(0x39119c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276ec0u64) }
        0x10276ec0 => { vm.decode(0xc904875u32); let d = 20i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276ec4u64) }
        0x10276ec4 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276ec8u64) }
        0x10276ec8 => { vm.decode(0x4c100875u32); let d = 18i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276eccu64) }
        0x10276ecc => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276ed0u64) }
        0x10276ed0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276ed4u64 + 4);
        vm.wr(37, 0x10276ed4u64 + 4);
        Ok(0x10276ed4u64) }
        0x10276ed4 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276ed8u64) }
        0x10276ed8 => { vm.decode(0xf1ce051u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10276edcu64) }
        0x10276edc => { vm.decode(0xb153422u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2076i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10276ee0u64) }
        0x10276ee0 => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276ee4u64) }
        0x10276ee4 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276ee8u64) }
        0x10276ee8 => { vm.decode(0x8d0855f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276eecu64) }
        0x10276eec => { vm.decode(0x6284532fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276ef0u64) }
        0x10276ef0 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276ef4u64) }
        0x10276ef4 => { vm.decode(0x6a82e3afu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276ef8u64) }
        0x10276ef8 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276efcu64) }
        0x10276efc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276f00u64 + 4);
        vm.wr(37, 0x10276f00u64 + 4);
        Ok(0x10276f00u64) }
        0x10276f00 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276f04u64) }
        0x10276f04 => { vm.decode(0x20a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276f08u64) }
        0x10276f08 => { vm.decode(0x39119cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276f0cu64) }
        0x10276f0c => { vm.decode(0x608c532fu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276f10u64) }
        0x10276f10 => { vm.decode(0x2a86e3afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276f14u64) }
        0x10276f14 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276f18u64) }
        0x10276f18 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276f1cu64 + 4);
        vm.wr(37, 0x10276f1cu64 + 4);
        Ok(0x10276f1cu64) }
        0x10276f1c => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276f20u64) }
        0x10276f20 => { vm.decode(0x2ac4f32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276f24u64) }
        0x10276f24 => { vm.decode(0x600a012fu32); let d = 23i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276f28u64) }
        0x10276f28 => { vm.decode(0x28a0602fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276f2cu64) }
        0x10276f2c => { vm.decode(0x28c436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276f30u64) }
        0x10276f30 => { vm.decode(0x2b299530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276f34u64) }
        0x10276f34 => { vm.decode(0xab21d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276f38u64) }
        0x10276f38 => { vm.decode(0x3b1994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276f3cu64) }
        0x10276f3c => { vm.decode(0x282732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1048i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276f40u64) }
        0x10276f40 => { vm.decode(0x2a86c3afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276f44u64) }
        0x10276f44 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276f48u64) }
        0x10276f48 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276f4cu64 + 4);
        vm.wr(37, 0x10276f4cu64 + 4);
        Ok(0x10276f4cu64) }
        0x10276f4c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10276f50u64) } }
        0x10276f50 => { vm.decode(0x20a0602fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276f54u64) }
        0x10276f54 => { vm.decode(0xa901ddf0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276f58u64) }
        0x10276f58 => { vm.decode(0xb919dd30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 3480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276f5cu64) }
        0x10276f5c => { vm.decode(0x39119c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276f60u64) }
        0x10276f60 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276f64u64) }
        0x10276f64 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276f68u64) }
        0x10276f68 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276f6cu64 + 4);
        vm.wr(37, 0x10276f6cu64 + 4);
        Ok(0x10276f6cu64) }
        0x10276f6c => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276f70u64) }
        0x10276f70 => { vm.decode(0xf1ce051u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10276f74u64) }
        0x10276f74 => { vm.decode(0x2b051022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10276f78u64) }
        0x10276f78 => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276f7cu64) }
        0x10276f7c => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276f80u64) }
        0x10276f80 => { vm.decode(0x8d0855f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276f84u64) }
        0x10276f84 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276f88u64) }
        0x10276f88 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276f8cu64) }
        0x10276f8c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276f90u64 + 4);
        vm.wr(37, 0x10276f90u64 + 4);
        Ok(0x10276f90u64) }
        0x10276f90 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276f94u64) }
        0x10276f94 => { vm.decode(0x28119u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 192i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10276f98u64) }
        0x10276f98 => { vm.decode(0x2ac60efu32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276f9cu64) }
        0x10276f9c => { vm.decode(0x1d181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276fa0u64) }
        0x10276fa0 => { vm.decode(0x2284636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276fa4u64) }
        0x10276fa4 => { vm.decode(0x1d101430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276fa8u64) }
        0x10276fa8 => { vm.decode(0x2a84e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276facu64) }
        0x10276fac => { vm.decode(0x1d081430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276fb0u64) }
        0x10276fb0 => { vm.decode(0xd001530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276fb4u64) }
        0x10276fb4 => { vm.decode(0x882e3efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276fb8u64) }
        0x10276fb8 => { vm.decode(0x2282636fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276fbcu64) }
        0x10276fbc => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276fc0u64) }
        0x10276fc0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276fc4u64 + 4);
        vm.wr(37, 0x10276fc4u64 + 4);
        Ok(0x10276fc4u64) }
        0x10276fc4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10276fc8u64) } }
        0x10276fc8 => { vm.decode(0x20a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276fccu64) }
        0x10276fcc => { vm.decode(0x39119cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276fd0u64) }
        0x10276fd0 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276fd4u64) }
        0x10276fd4 => { vm.decode(0x4a86e3afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276fd8u64) }
        0x10276fd8 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276fdcu64) }
        0x10276fdc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10276fe0u64 + 4);
        vm.wr(37, 0x10276fe0u64 + 4);
        Ok(0x10276fe0u64) }
        0x10276fe0 => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10276fe4u64) }
        0x10276fe4 => { vm.decode(0x2ac4f32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276fe8u64) }
        0x10276fe8 => { vm.decode(0x600a012fu32); let d = 23i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276fecu64) }
        0x10276fec => { vm.decode(0x28a0602fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276ff0u64) }
        0x10276ff0 => { vm.decode(0x888432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10276ff4u64) }
        0x10276ff4 => { vm.decode(0x2b299530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276ff8u64) }
        0x10276ff8 => { vm.decode(0xab21d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10276ffcu64) }
        0x10276ffc => { vm.decode(0x3b1994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277000u64) }
        0x10277000 => { vm.decode(0x6288732fu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 1048i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277004u64) }
        0x10277004 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277008u64) }
        0x10277008 => { vm.decode(0x4a8cc3afu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027700cu64) }
        0x1027700c => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277010u64) }
        0x10277010 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277014u64 + 4);
        vm.wr(37, 0x10277014u64 + 4);
        Ok(0x10277014u64) }
        0x10277014 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277018u64) }
        0x10277018 => { vm.decode(0x20a0602fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027701cu64) }
        0x1027701c => { vm.decode(0xa901ddf0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277020u64) }
        0x10277020 => { vm.decode(0xb919dd30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 3480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277024u64) }
        0x10277024 => { vm.decode(0x39119c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277028u64) }
        0x10277028 => { vm.decode(0x282532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027702cu64) }
        0x1027702c => { vm.decode(0x6884c3afu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277030u64) }
        0x10277030 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277034u64) }
        0x10277034 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277038u64 + 4);
        vm.wr(37, 0x10277038u64 + 4);
        Ok(0x10277038u64) }
        0x10277038 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027703cu64) }
        0x1027703c => { vm.decode(0xf1ce051u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10277040u64) }
        0x10277040 => { vm.decode(0x3b381c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277044u64) }
        0x10277044 => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277048u64) }
        0x10277048 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027704cu64) }
        0x1027704c => { vm.decode(0x8d0855f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277050u64) }
        0x10277050 => { vm.decode(0x6286532fu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277054u64) }
        0x10277054 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277058u64) }
        0x10277058 => { vm.decode(0x6a8ae3afu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027705cu64) }
        0x1027705c => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277060u64) }
        0x10277060 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277064u64 + 4);
        vm.wr(37, 0x10277064u64 + 4);
        Ok(0x10277064u64) }
        0x10277064 => { vm.decode(0x490d875u32); let d = 5i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277068u64) }
        0x10277068 => { vm.decode(0x20a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027706cu64) }
        0x1027706c => { vm.decode(0x39119cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277070u64) }
        0x10277070 => { vm.decode(0x86532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277074u64) }
        0x10277074 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277078u64) }
        0x10277078 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027707cu64 + 4);
        vm.wr(37, 0x1027707cu64 + 4);
        Ok(0x1027707cu64) }
        0x1027707c => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277080u64) }
        0x10277080 => { vm.decode(0x2ac4f32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277084u64) }
        0x10277084 => { vm.decode(0x4002012fu32); let d = 20i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277088u64) }
        0x10277088 => { vm.decode(0x28a0602fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027708cu64) }
        0x1027708c => { vm.decode(0xa8c432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277090u64) }
        0x10277090 => { vm.decode(0x2b299530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277094u64) }
        0x10277094 => { vm.decode(0xab21d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277098u64) }
        0x10277098 => { vm.decode(0x3b1994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027709cu64) }
        0x1027709c => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102770a0u64) }
        0x102770a0 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102770a4u64) }
        0x102770a4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102770a8u64 + 4);
        vm.wr(37, 0x102770a8u64 + 4);
        Ok(0x102770a8u64) }
        0x102770a8 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102770acu64) }
        0x102770ac => { vm.decode(0x20a0602fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102770b0u64) }
        0x102770b0 => { vm.decode(0xa901ddf0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102770b4u64) }
        0x102770b4 => { vm.decode(0xa919dc30u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 3480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102770b8u64) }
        0x102770b8 => { vm.decode(0x39119c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102770bcu64) }
        0x102770bc => { vm.decode(0x282532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102770c0u64) }
        0x102770c0 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102770c4u64) }
        0x102770c4 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102770c8u64) }
        0x102770c8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102770ccu64 + 4);
        vm.wr(37, 0x102770ccu64 + 4);
        Ok(0x102770ccu64) }
        0x102770cc => { vm.decode(0xc900875u32); let d = 20i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102770d0u64) }
        0x102770d0 => { vm.decode(0xf1ce051u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102770d4u64) }
        0x102770d4 => { vm.decode(0x3b201cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102770d8u64) }
        0x102770d8 => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102770dcu64) }
        0x102770dc => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102770e0u64) }
        0x102770e0 => { vm.decode(0x8d0855f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102770e4u64) }
        0x102770e4 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102770e8u64) }
        0x102770e8 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102770ecu64) }
        0x102770ec => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102770f0u64 + 4);
        vm.wr(37, 0x102770f0u64 + 4);
        Ok(0x102770f0u64) }
        0x102770f0 => { vm.decode(0x490d875u32); let d = 5i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102770f4u64) }
        0x102770f4 => { vm.decode(0x20a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102770f8u64) }
        0x102770f8 => { vm.decode(0x39119cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102770fcu64) }
        0x102770fc => { vm.decode(0x86532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277100u64) }
        0x10277100 => { vm.decode(0x2a86e3afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277104u64) }
        0x10277104 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277108u64) }
        0x10277108 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027710cu64 + 4);
        vm.wr(37, 0x1027710cu64 + 4);
        Ok(0x1027710cu64) }
        0x1027710c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10277110u64) } }
        0x10277110 => { vm.decode(0x2ac4f32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277114u64) }
        0x10277114 => { vm.decode(0x600a012fu32); let d = 23i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277118u64) }
        0x10277118 => { vm.decode(0x28a0602fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027711cu64) }
        0x1027711c => { vm.decode(0x88c632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277120u64) }
        0x10277120 => { vm.decode(0x2b299530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277124u64) }
        0x10277124 => { vm.decode(0xab21d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277128u64) }
        0x10277128 => { vm.decode(0x3b1994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027712cu64) }
        0x1027712c => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277130u64) }
        0x10277130 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277134u64) }
        0x10277134 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277138u64 + 4);
        vm.wr(37, 0x10277138u64 + 4);
        Ok(0x10277138u64) }
        0x10277138 => { vm.decode(0x909875u32); let d = 5i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027713cu64) }
        0x1027713c => { vm.decode(0x20a0602fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277140u64) }
        0x10277140 => { vm.decode(0xa901ddf0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277144u64) }
        0x10277144 => { vm.decode(0xb919dd30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 3480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277148u64) }
        0x10277148 => { vm.decode(0x39119c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027714cu64) }
        0x1027714c => { vm.decode(0x6280532fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 1040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277150u64) }
        0x10277150 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277154u64) }
        0x10277154 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277158u64) }
        0x10277158 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027715cu64 + 4);
        vm.wr(37, 0x1027715cu64 + 4);
        Ok(0x1027715cu64) }
        0x1027715c => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277160u64) }
        0x10277160 => { vm.decode(0xf1ce051u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10277164u64) }
        0x10277164 => { vm.decode(0x633d1c62u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2028i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10277168u64) }
        0x10277168 => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027716cu64) }
        0x1027716c => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277170u64) }
        0x10277170 => { vm.decode(0x8d0855f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277174u64) }
        0x10277174 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277178u64) }
        0x10277178 => { vm.decode(0xc904875u32); let d = 20i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027717cu64) }
        0x1027717c => { vm.decode(0x6a82e3afu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277180u64) }
        0x10277180 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277184u64) }
        0x10277184 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277188u64 + 4);
        vm.wr(37, 0x10277188u64 + 4);
        Ok(0x10277188u64) }
        0x10277188 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027718cu64) }
        0x1027718c => { vm.decode(0x20a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277190u64) }
        0x10277190 => { vm.decode(0x39119cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277194u64) }
        0x10277194 => { vm.decode(0x408c532fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277198u64) }
        0x10277198 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027719cu64) }
        0x1027719c => { vm.decode(0x6a8ce3afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102771a0u64) }
        0x102771a0 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102771a4u64) }
        0x102771a4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102771a8u64 + 4);
        vm.wr(37, 0x102771a8u64 + 4);
        Ok(0x102771a8u64) }
        0x102771a8 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102771acu64) }
        0x102771ac => { vm.decode(0x2ac4f32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102771b0u64) }
        0x102771b0 => { vm.decode(0x600a012fu32); let d = 23i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102771b4u64) }
        0x102771b4 => { vm.decode(0x28a0602fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102771b8u64) }
        0x102771b8 => { vm.decode(0x88c432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102771bcu64) }
        0x102771bc => { vm.decode(0x2b299530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102771c0u64) }
        0x102771c0 => { vm.decode(0xab21d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102771c4u64) }
        0x102771c4 => { vm.decode(0x3b1994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102771c8u64) }
        0x102771c8 => { vm.decode(0x282732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1048i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102771ccu64) }
        0x102771cc => { vm.decode(0x2a86c3afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102771d0u64) }
        0x102771d0 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102771d4u64) }
        0x102771d4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102771d8u64 + 4);
        vm.wr(37, 0x102771d8u64 + 4);
        Ok(0x102771d8u64) }
        0x102771d8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102771dcu64) } }
        0x102771dc => { vm.decode(0x20a0602fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102771e0u64) }
        0x102771e0 => { vm.decode(0xa901ddf0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102771e4u64) }
        0x102771e4 => { vm.decode(0xb919dd30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 3480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102771e8u64) }
        0x102771e8 => { vm.decode(0x39119c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102771ecu64) }
        0x102771ec => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102771f0u64) }
        0x102771f0 => { vm.decode(0x6884c3afu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102771f4u64) }
        0x102771f4 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102771f8u64) }
        0x102771f8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102771fcu64 + 4);
        vm.wr(37, 0x102771fcu64 + 4);
        Ok(0x102771fcu64) }
        0x102771fc => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277200u64) }
        0x10277200 => { vm.decode(0xf1ce051u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10277204u64) }
        0x10277204 => { vm.decode(0x632d3862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2032i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10277208u64) }
        0x10277208 => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027720cu64) }
        0x1027720c => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277210u64) }
        0x10277210 => { vm.decode(0x8d0855f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277214u64) }
        0x10277214 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277218u64) }
        0x10277218 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027721cu64) }
        0x1027721c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277220u64 + 4);
        vm.wr(37, 0x10277220u64 + 4);
        Ok(0x10277220u64) }
        0x10277220 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277224u64) }
        0x10277224 => { vm.decode(0x20a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277228u64) }
        0x10277228 => { vm.decode(0x39119cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027722cu64) }
        0x1027722c => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277230u64) }
        0x10277230 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277234u64) }
        0x10277234 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277238u64 + 4);
        vm.wr(37, 0x10277238u64 + 4);
        Ok(0x10277238u64) }
        0x10277238 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027723cu64) }
        0x1027723c => { vm.decode(0x2ac4f32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277240u64) }
        0x10277240 => { vm.decode(0x600a012fu32); let d = 23i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277244u64) }
        0x10277244 => { vm.decode(0x28a0602fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277248u64) }
        0x10277248 => { vm.decode(0xa88632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027724cu64) }
        0x1027724c => { vm.decode(0x2b299530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277250u64) }
        0x10277250 => { vm.decode(0xab21d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277254u64) }
        0x10277254 => { vm.decode(0x3b1994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277258u64) }
        0x10277258 => { vm.decode(0x6288732fu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 1048i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027725cu64) }
        0x1027725c => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277260u64) }
        0x10277260 => { vm.decode(0x6a86c3afu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277264u64) }
        0x10277264 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277268u64) }
        0x10277268 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027726cu64 + 4);
        vm.wr(37, 0x1027726cu64 + 4);
        Ok(0x1027726cu64) }
        0x1027726c => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277270u64) }
        0x10277270 => { vm.decode(0x20a0602fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277274u64) }
        0x10277274 => { vm.decode(0xa901ddf0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277278u64) }
        0x10277278 => { vm.decode(0xb919dd30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 3480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027727cu64) }
        0x1027727c => { vm.decode(0x39119c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277280u64) }
        0x10277280 => { vm.decode(0x628a532fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 1040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277284u64) }
        0x10277284 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277288u64) }
        0x10277288 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027728cu64) }
        0x1027728c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277290u64 + 4);
        vm.wr(37, 0x10277290u64 + 4);
        Ok(0x10277290u64) }
        0x10277290 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277294u64) }
        0x10277294 => { vm.decode(0xf1ce051u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10277298u64) }
        0x10277298 => { vm.decode(0x631d1c62u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1900i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027729cu64) }
        0x1027729c => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102772a0u64) }
        0x102772a0 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102772a4u64) }
        0x102772a4 => { vm.decode(0x8d0855f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102772a8u64) }
        0x102772a8 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102772acu64) }
        0x102772ac => { vm.decode(0x4a82e3afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102772b0u64) }
        0x102772b0 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102772b4u64) }
        0x102772b4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102772b8u64 + 4);
        vm.wr(37, 0x102772b8u64 + 4);
        Ok(0x102772b8u64) }
        0x102772b8 => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102772bcu64) }
        0x102772bc => { vm.decode(0x2aa4e02fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102772c0u64) }
        0x102772c0 => { vm.decode(0x39019c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102772c4u64) }
        0x102772c4 => { vm.decode(0x82e3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102772c8u64) }
        0x102772c8 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102772ccu64) }
        0x102772cc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102772d0u64 + 4);
        vm.wr(37, 0x102772d0u64 + 4);
        Ok(0x102772d0u64) }
        0x102772d0 => { vm.decode(0x622b8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3456i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102772d4u64) }
        0x102772d4 => { vm.decode(0x20a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102772d8u64) }
        0x102772d8 => { vm.decode(0x39119cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102772dcu64) }
        0x102772dc => { vm.decode(0x1f18e051u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 3464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102772e0u64) }
        0x102772e0 => { vm.decode(0x2a86e3afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102772e4u64) }
        0x102772e4 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102772e8u64) }
        0x102772e8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102772ecu64 + 4);
        vm.wr(37, 0x102772ecu64 + 4);
        Ok(0x102772ecu64) }
        0x102772ec => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102772f0u64) }
        0x102772f0 => { vm.decode(0x2ac4f32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102772f4u64) }
        0x102772f4 => { vm.decode(0x4008012fu32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102772f8u64) }
        0x102772f8 => { vm.decode(0x28a0602fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102772fcu64) }
        0x102772fc => { vm.decode(0xa88432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277300u64) }
        0x10277300 => { vm.decode(0x2b299530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277304u64) }
        0x10277304 => { vm.decode(0xab21d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277308u64) }
        0x10277308 => { vm.decode(0x3b1994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027730cu64) }
        0x1027730c => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277310u64) }
        0x10277310 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277314u64) }
        0x10277314 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277318u64 + 4);
        vm.wr(37, 0x10277318u64 + 4);
        Ok(0x10277318u64) }
        0x10277318 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027731cu64) }
        0x1027731c => { vm.decode(0x20a0602fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277320u64) }
        0x10277320 => { vm.decode(0xa901ddf0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277324u64) }
        0x10277324 => { vm.decode(0xa9199d30u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 3480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277328u64) }
        0x10277328 => { vm.decode(0x39119c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027732cu64) }
        0x1027732c => { vm.decode(0x2886c3afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277330u64) }
        0x10277330 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277334u64) }
        0x10277334 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277338u64 + 4);
        vm.wr(37, 0x10277338u64 + 4);
        Ok(0x10277338u64) }
        0x10277338 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027733cu64) }
        0x1027733c => { vm.decode(0xf1ce051u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10277340u64) }
        0x10277340 => { vm.decode(0xa8008b35u32); let d = 1i16; let s = 17i16; let s2 = 1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_sub(vm.ld(s2) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10277344u64) }
        0x10277344 => { vm.decode(0x608a736fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277348u64) }
        0x10277348 => { vm.decode(0x430d3862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027734cu64) }
        0x1027734c => { vm.decode(0x28a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277350u64) }
        0x10277350 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277354u64) }
        0x10277354 => { vm.decode(0x8d0855f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277358u64) }
        0x10277358 => { vm.decode(0x286532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027735cu64) }
        0x1027735c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277360u64) }
        0x10277360 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277364u64 + 4);
        vm.wr(37, 0x10277364u64 + 4);
        Ok(0x10277364u64) }
        0x10277364 => { vm.decode(0x90c875u32); let d = 5i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277368u64) }
        0x10277368 => { vm.decode(0xbc0fedf6u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027736cu64) }
        0x1027736c => { vm.decode(0x66398419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277370u64) }
        0x10277370 => { vm.decode(0x6886e3efu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277374u64) }
        0x10277374 => { vm.decode(0x88c12fu32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277378u64) }
        0x10277378 => { vm.decode(0x231d3822u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027737cu64) }
        0x1027737c => { vm.decode(0x2b1015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277380u64) }
        0x10277380 => { vm.decode(0x4082732fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1032i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277384u64) }
        0x10277384 => { vm.decode(0xab0854b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277388u64) }
        0x10277388 => { vm.decode(0x28a0c02fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027738cu64) }
        0x1027738c => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277390u64) }
        0x10277390 => { vm.decode(0x2b0915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277394u64) }
        0x10277394 => { vm.decode(0x3b0114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277398u64) }
        0x10277398 => { vm.decode(0x6a80c3efu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027739cu64) }
        0x1027739c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102773a0u64) }
        0x102773a0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102773a4u64 + 4);
        vm.wr(37, 0x102773a4u64 + 4);
        Ok(0x102773a4u64) }
        0x102773a4 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102773a8u64) }
        0x102773a8 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102773acu64) }
        0x102773ac => { vm.decode(0x2b181c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102773b0u64) }
        0x102773b0 => { vm.decode(0x2b101c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102773b4u64) }
        0x102773b4 => { vm.decode(0xf144a51u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102773b8u64) }
        0x102773b8 => { vm.decode(0x231d3822u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102773bcu64) }
        0x102773bc => { vm.decode(0x20a4c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102773c0u64) }
        0x102773c0 => { vm.decode(0xa8c412fu32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102773c4u64) }
        0x102773c4 => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102773c8u64) }
        0x102773c8 => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102773ccu64) }
        0x102773cc => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102773d0u64) }
        0x102773d0 => { vm.decode(0x4088532fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102773d4u64) }
        0x102773d4 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102773d8u64) }
        0x102773d8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102773dcu64 + 4);
        vm.wr(37, 0x102773dcu64 + 4);
        Ok(0x102773dcu64) }
        0x102773dc => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102773e0u64) }
        0x102773e0 => { vm.decode(0x2a84412fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102773e4u64) }
        0x102773e4 => { vm.decode(0x3b101c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102773e8u64) }
        0x102773e8 => { vm.decode(0xa8c612fu32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102773ecu64) }
        0x102773ec => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102773f0u64).wrapping_add((imm * 4) as u64) } else { 0x102773f0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102773f0u64) }
        0x102773f0 => { vm.decode(0x2b181d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102773f4u64) }
        0x102773f4 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102773f8u64) }
        0x102773f8 => { vm.decode(0x8003036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102773fcu64) }
        0x102773fc => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10277400u64) }
        0x10277400 => { vm.decode(0x20a4e06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277404u64) }
        0x10277404 => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277408u64) }
        0x10277408 => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027740cu64) }
        0x1027740c => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277410u64) }
        0x10277410 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277414u64) }
        0x10277414 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277418u64 + 4);
        vm.wr(37, 0x10277418u64 + 4);
        Ok(0x10277418u64) }
        0x10277418 => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027741cu64) }
        0x1027741c => { vm.decode(0xab0854b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277420u64) }
        0x10277420 => { vm.decode(0xd14ca51u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10277424u64) }
        0x10277424 => { vm.decode(0x82002031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10277428u64).wrapping_add((imm * 4) as u64) } else { 0x10277428u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10277428u64) }
        0x10277428 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027742cu64) } }
        0x1027742c => { vm.decode(0x28a0e02fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277430u64) }
        0x10277430 => { vm.decode(0x28c53afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277434u64) }
        0x10277434 => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277438u64) }
        0x10277438 => { vm.decode(0x2b0915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027743cu64) }
        0x1027743c => { vm.decode(0x3b0114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277440u64) }
        0x10277440 => { vm.decode(0x66398419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277444u64) }
        0x10277444 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277448u64) }
        0x10277448 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027744cu64 + 4);
        vm.wr(37, 0x1027744cu64 + 4);
        Ok(0x1027744cu64) }
        0x1027744c => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277450u64) }
        0x10277450 => { vm.decode(0x6280d3afu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 1808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277454u64) }
        0x10277454 => { vm.decode(0x20a4c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277458u64) }
        0x10277458 => { vm.decode(0xbb199430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027745cu64) }
        0x1027745c => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277460u64) }
        0x10277460 => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277464u64) }
        0x10277464 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277468u64) }
        0x10277468 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027746cu64) }
        0x1027746c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277470u64 + 4);
        vm.wr(37, 0x10277470u64 + 4);
        Ok(0x10277470u64) }
        0x10277470 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277474u64) }
        0x10277474 => { vm.decode(0x288f3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277478u64) }
        0x10277478 => { vm.decode(0x6a88e3efu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027747cu64) }
        0x1027747c => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10277480u64).wrapping_add((imm * 4) as u64) } else { 0x10277480u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10277480u64) }
        0x10277480 => { vm.decode(0x1242e35u32); let d = 17i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s) < vm.ld(s2)) as u64;
        vm.st(d, v);
        Ok(0x10277484u64) }
        0x10277484 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10277488u64) }
        0x10277488 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x1027748cu64) }
        0x1027748c => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10277490u64) }
        0x10277490 => { vm.decode(0x288f3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277494u64) }
        0x10277494 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10277498u64) }
        0x10277498 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x1027749cu64).wrapping_add((imm * 4) as u64) } else { 0x1027749cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027749cu64) }
        0x1027749c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102774a0u64) } }
        0x102774a0 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102774a4u64).wrapping_add((imm * 4) as u64) } else { 0x102774a4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102774a4u64) }
        0x102774f8 => { vm.decode(0x20a4e06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102774fcu64) }
        0x102774fc => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277500u64) }
        0x10277500 => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277504u64) }
        0x10277504 => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277508u64) }
        0x10277508 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027750cu64) }
        0x1027750c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277510u64 + 4);
        vm.wr(37, 0x10277510u64 + 4);
        Ok(0x10277510u64) }
        0x10277510 => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277514u64) }
        0x10277514 => { vm.decode(0x14860227u32); let d = 0i16; let s = 17i16; let s2 = -1i16; let imm = 108i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10277518u64).wrapping_add((imm * 4) as u64) } else { 0x10277518u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10277518u64) }
        0x10277518 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027751cu64) } }
        0x1027751c => { vm.decode(0x2080c12fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277520u64) }
        0x10277520 => { vm.decode(0xbc0fcff6u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10277524u64) }
        0x10277524 => { vm.decode(0x221d7822u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10277528u64) }
        0x10277528 => { vm.decode(0x3b1014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027752cu64) }
        0x1027752c => { vm.decode(0xab0854b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277530u64) }
        0x10277530 => { vm.decode(0x8a8c02fu32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277534u64) }
        0x10277534 => { vm.decode(0x40048019u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277538u64) }
        0x10277538 => { vm.decode(0x3b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027753cu64) }
        0x1027753c => { vm.decode(0x2b0115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277540u64) }
        0x10277540 => { vm.decode(0x66398419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277544u64) }
        0x10277544 => { vm.decode(0x6a80c3efu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277548u64) }
        0x10277548 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027754cu64) }
        0x1027754c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277550u64 + 4);
        vm.wr(37, 0x10277550u64 + 4);
        Ok(0x10277550u64) }
        0x10277550 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277554u64) }
        0x10277554 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277558u64) }
        0x10277558 => { vm.decode(0x2b181c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027755cu64) }
        0x1027755c => { vm.decode(0x2b101c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277560u64) }
        0x10277560 => { vm.decode(0xf144a51u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10277564u64) }
        0x10277564 => { vm.decode(0x231d3822u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10277568u64) }
        0x10277568 => { vm.decode(0x20a4c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027756cu64) }
        0x1027756c => { vm.decode(0xa8c412fu32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277570u64) }
        0x10277570 => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277574u64) }
        0x10277574 => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277578u64) }
        0x10277578 => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027757cu64) }
        0x1027757c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277580u64) }
        0x10277580 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277584u64 + 4);
        vm.wr(37, 0x10277584u64 + 4);
        Ok(0x10277584u64) }
        0x10277584 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277588u64) }
        0x10277588 => { vm.decode(0x2a84412fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027758cu64) }
        0x1027758c => { vm.decode(0x3b101c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277590u64) }
        0x10277590 => { vm.decode(0xa8c612fu32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277594u64) }
        0x10277594 => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10277598u64).wrapping_add((imm * 4) as u64) } else { 0x10277598u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10277598u64) }
        0x10277598 => { vm.decode(0x2b181d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027759cu64) }
        0x1027759c => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102775a0u64) }
        0x102775a0 => { vm.decode(0x8003036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102775a4u64) }
        0x102775a4 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102775a8u64) }
        0x102775a8 => { vm.decode(0x20a4e06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102775acu64) }
        0x102775ac => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102775b0u64) }
        0x102775b0 => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102775b4u64) }
        0x102775b4 => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102775b8u64) }
        0x102775b8 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102775bcu64) }
        0x102775bc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102775c0u64 + 4);
        vm.wr(37, 0x102775c0u64 + 4);
        Ok(0x102775c0u64) }
        0x102775c0 => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102775c4u64) }
        0x102775c4 => { vm.decode(0xab0854b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102775c8u64) }
        0x102775c8 => { vm.decode(0xd14ca51u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102775ccu64) }
        0x102775cc => { vm.decode(0x82002031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102775d0u64).wrapping_add((imm * 4) as u64) } else { 0x102775d0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102775d0u64) }
        0x102775d0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102775d4u64) } }
        0x102775d4 => { vm.decode(0x28a0e02fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102775d8u64) }
        0x102775d8 => { vm.decode(0x28c53afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102775dcu64) }
        0x102775dc => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102775e0u64) }
        0x102775e0 => { vm.decode(0x2b0915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102775e4u64) }
        0x102775e4 => { vm.decode(0x3b0114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102775e8u64) }
        0x102775e8 => { vm.decode(0x66398419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102775ecu64) }
        0x102775ec => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102775f0u64) }
        0x102775f0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102775f4u64 + 4);
        vm.wr(37, 0x102775f4u64 + 4);
        Ok(0x102775f4u64) }
        0x102775f4 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102775f8u64) }
        0x102775f8 => { vm.decode(0x2280d3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102775fcu64) }
        0x102775fc => { vm.decode(0x208012fu32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277600u64) }
        0x10277600 => { vm.decode(0x2b301db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277604u64) }
        0x10277604 => { vm.decode(0xacc06fu32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277608u64) }
        0x10277608 => { vm.decode(0x3b199430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027760cu64) }
        0x1027760c => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277610u64) }
        0x10277610 => { vm.decode(0x2b099530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277614u64) }
        0x10277614 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277618u64) }
        0x10277618 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027761cu64) }
        0x1027761c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277620u64 + 4);
        vm.wr(37, 0x10277620u64 + 4);
        Ok(0x10277620u64) }
        0x10277620 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277624u64) }
        0x10277624 => { vm.decode(0x288f3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277628u64) }
        0x10277628 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027762cu64).wrapping_add((imm * 4) as u64) } else { 0x1027762cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027762cu64) }
        0x1027762c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10277630u64) } }
        0x10277630 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10277634u64) }
        0x10277634 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10277638u64) }
        0x10277638 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027763cu64) }
        0x1027763c => { vm.decode(0x288f3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277640u64) }
        0x10277640 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10277644u64) }
        0x10277644 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x10277648u64).wrapping_add((imm * 4) as u64) } else { 0x10277648u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10277648u64) }
        0x10277648 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027764cu64) } }
        0x1027764c => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10277650u64).wrapping_add((imm * 4) as u64) } else { 0x10277650u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10277650u64) }
        0x102776a4 => { vm.decode(0x20a4e06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102776a8u64) }
        0x102776a8 => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102776acu64) }
        0x102776ac => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102776b0u64) }
        0x102776b0 => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102776b4u64) }
        0x102776b4 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102776b8u64) }
        0x102776b8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102776bcu64 + 4);
        vm.wr(37, 0x102776bcu64 + 4);
        Ok(0x102776bcu64) }
        0x102776bc => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102776c0u64) }
        0x102776c0 => { vm.decode(0x2080051du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 8724i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102776c4u64) }
        0x102776c4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102776c8u64) } }
        0x102776c8 => { vm.decode(0x2880c3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102776ccu64) }
        0x102776d4 => { vm.decode(0x28a0c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102776d8u64) }
        0x102776d8 => { vm.decode(0x3b1914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102776dcu64) }
        0x102776dc => { vm.decode(0x667d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3640i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102776e0u64) }
        0x102776e0 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102776e4u64) }
        0x102776e4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102776e8u64 + 4);
        vm.wr(37, 0x102776e8u64 + 4);
        Ok(0x102776e8u64) }
        0x102776e8 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102776ecu64) }
        0x102776ec => { vm.decode(0xf904251u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102776f0u64) }
        0x102776f0 => { vm.decode(0x631d3862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102776f4u64) }
        0x102776f4 => { vm.decode(0xbc0fedf6u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102776f8u64) }
        0x102776f8 => { vm.decode(0x66398419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102776fcu64) }
        0x102776fc => { vm.decode(0x88c12fu32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277700u64) }
        0x10277700 => { vm.decode(0x231d3822u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10277704u64) }
        0x10277704 => { vm.decode(0x2b1015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277708u64) }
        0x10277708 => { vm.decode(0xab0854b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027770cu64) }
        0x1027770c => { vm.decode(0x28a0c02fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277710u64) }
        0x10277710 => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277714u64) }
        0x10277714 => { vm.decode(0x2b0915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277718u64) }
        0x10277718 => { vm.decode(0x3b0114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027771cu64) }
        0x1027771c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277720u64) }
        0x10277720 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277724u64 + 4);
        vm.wr(37, 0x10277724u64 + 4);
        Ok(0x10277724u64) }
        0x10277724 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277728u64) }
        0x10277728 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027772cu64) }
        0x1027772c => { vm.decode(0x2b181c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277730u64) }
        0x10277730 => { vm.decode(0x2b101c30u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277734u64) }
        0x10277734 => { vm.decode(0xf144a51u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10277738u64) }
        0x10277738 => { vm.decode(0x231d3822u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027773cu64) }
        0x1027773c => { vm.decode(0x20a4c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277740u64) }
        0x10277740 => { vm.decode(0x8c612fu32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277744u64) }
        0x10277744 => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277748u64) }
        0x10277748 => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027774cu64) }
        0x1027774c => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277750u64) }
        0x10277750 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277754u64) }
        0x10277754 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277758u64 + 4);
        vm.wr(37, 0x10277758u64 + 4);
        Ok(0x10277758u64) }
        0x10277758 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027775cu64) }
        0x1027775c => { vm.decode(0x2084612fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277760u64) }
        0x10277760 => { vm.decode(0x3b101c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277764u64) }
        0x10277764 => { vm.decode(0x28c412fu32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277768u64) }
        0x10277768 => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x1027776cu64).wrapping_add((imm * 4) as u64) } else { 0x1027776cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x1027776cu64) }
        0x1027776c => { vm.decode(0x2b181d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277770u64) }
        0x10277770 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10277774u64) }
        0x10277774 => { vm.decode(0x8003036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10277778u64) }
        0x10277778 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027777cu64) }
        0x1027777c => { vm.decode(0x20a4e06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277780u64) }
        0x10277780 => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277784u64) }
        0x10277784 => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277788u64) }
        0x10277788 => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027778cu64) }
        0x1027778c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277790u64) }
        0x10277790 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277794u64 + 4);
        vm.wr(37, 0x10277794u64 + 4);
        Ok(0x10277794u64) }
        0x10277794 => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277798u64) }
        0x10277798 => { vm.decode(0xab0854b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027779cu64) }
        0x1027779c => { vm.decode(0xd14ca51u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102777a0u64) }
        0x102777a0 => { vm.decode(0x82002031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102777a4u64).wrapping_add((imm * 4) as u64) } else { 0x102777a4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102777a4u64) }
        0x102777a4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102777a8u64) } }
        0x102777a8 => { vm.decode(0x28a0e02fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102777acu64) }
        0x102777ac => { vm.decode(0x28c53afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102777b0u64) }
        0x102777b0 => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102777b4u64) }
        0x102777b4 => { vm.decode(0x2b0915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102777b8u64) }
        0x102777b8 => { vm.decode(0x3b0114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102777bcu64) }
        0x102777bc => { vm.decode(0x66398419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102777c0u64) }
        0x102777c0 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102777c4u64) }
        0x102777c4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102777c8u64 + 4);
        vm.wr(37, 0x102777c8u64 + 4);
        Ok(0x102777c8u64) }
        0x102777c8 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102777ccu64) }
        0x102777cc => { vm.decode(0x102236u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102777d0u64) }
        0x102777d0 => { vm.decode(0x2280d3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102777d4u64) }
        0x102777d4 => { vm.decode(0x48c0227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 60i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102777d8u64).wrapping_add((imm * 4) as u64) } else { 0x102777d8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102777d8u64) }
        0x102777d8 => { vm.decode(0x102036u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102777dcu64) }
        0x102777dc => { vm.decode(0x2080c12fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102777e0u64) }
        0x102777e0 => { vm.decode(0xbc0fcff6u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102777e4u64) }
        0x102777e4 => { vm.decode(0x221d7822u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102777e8u64) }
        0x102777e8 => { vm.decode(0x3b1014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102777ecu64) }
        0x102777ec => { vm.decode(0xab0854b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102777f0u64) }
        0x102777f0 => { vm.decode(0x8a8c02fu32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102777f4u64) }
        0x102777f4 => { vm.decode(0x40048019u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102777f8u64) }
        0x102777f8 => { vm.decode(0x3b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102777fcu64) }
        0x102777fc => { vm.decode(0x2b0115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277800u64) }
        0x10277800 => { vm.decode(0x66398419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277804u64) }
        0x10277804 => { vm.decode(0x6a80c3efu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277808u64) }
        0x10277808 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027780cu64) }
        0x1027780c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277810u64 + 4);
        vm.wr(37, 0x10277810u64 + 4);
        Ok(0x10277810u64) }
        0x10277810 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277814u64) }
        0x10277814 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277818u64) }
        0x10277818 => { vm.decode(0x2b101cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027781cu64) }
        0x1027781c => { vm.decode(0x2b081cb0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277820u64) }
        0x10277820 => { vm.decode(0xf144a51u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10277824u64) }
        0x10277824 => { vm.decode(0x231d3822u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10277828u64) }
        0x10277828 => { vm.decode(0x20a4c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027782cu64) }
        0x1027782c => { vm.decode(0x8c612fu32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277830u64) }
        0x10277830 => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277834u64) }
        0x10277834 => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277838u64) }
        0x10277838 => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027783cu64) }
        0x1027783c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277840u64) }
        0x10277840 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277844u64 + 4);
        vm.wr(37, 0x10277844u64 + 4);
        Ok(0x10277844u64) }
        0x10277844 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277848u64) }
        0x10277848 => { vm.decode(0x2084612fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027784cu64) }
        0x1027784c => { vm.decode(0x3b081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277850u64) }
        0x10277850 => { vm.decode(0x28c412fu32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277854u64) }
        0x10277854 => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10277858u64).wrapping_add((imm * 4) as u64) } else { 0x10277858u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10277858u64) }
        0x10277858 => { vm.decode(0x2b101db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027785cu64) }
        0x1027785c => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10277860u64) }
        0x10277860 => { vm.decode(0x8003036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10277864u64) }
        0x10277864 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10277868u64) }
        0x10277868 => { vm.decode(0x20a4e06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027786cu64) }
        0x1027786c => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277870u64) }
        0x10277870 => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277874u64) }
        0x10277874 => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277878u64) }
        0x10277878 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027787cu64) }
        0x1027787c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277880u64 + 4);
        vm.wr(37, 0x10277880u64 + 4);
        Ok(0x10277880u64) }
        0x10277880 => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277884u64) }
        0x10277884 => { vm.decode(0xab0854b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277888u64) }
        0x10277888 => { vm.decode(0xd14ca51u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1027788cu64) }
        0x1027788c => { vm.decode(0x82002031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10277890u64).wrapping_add((imm * 4) as u64) } else { 0x10277890u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10277890u64) }
        0x10277890 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10277894u64) } }
        0x10277894 => { vm.decode(0x28a0e02fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277898u64) }
        0x10277898 => { vm.decode(0x28c53afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027789cu64) }
        0x1027789c => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102778a0u64) }
        0x102778a0 => { vm.decode(0x2b0915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102778a4u64) }
        0x102778a4 => { vm.decode(0x3b0114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102778a8u64) }
        0x102778a8 => { vm.decode(0x66398419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3616i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102778acu64) }
        0x102778ac => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102778b0u64) }
        0x102778b0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102778b4u64 + 4);
        vm.wr(37, 0x102778b4u64 + 4);
        Ok(0x102778b4u64) }
        0x102778b4 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102778b8u64) }
        0x102778b8 => { vm.decode(0x2084f3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102778bcu64) }
        0x102778bc => { vm.decode(0xc008091u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102778c0u64) }
        0x102778c0 => { vm.decode(0x41008375u32); let d = 17i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = ((vm.ld(s) as i64) < (vm.ld(s2) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102778c4u64) }
        0x102778c4 => { vm.decode(0x8102236u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102778c8u64) }
        0x102778c8 => { vm.decode(0xc0e8027u32); let d = 0i16; let s = 19i16; let s2 = -1i16; let imm = 49i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102778ccu64).wrapping_add((imm * 4) as u64) } else { 0x102778ccu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102778ccu64) }
        0x102778cc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102778d0u64) } }
        0x102778d0 => { vm.decode(0x20a4c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102778d4u64) }
        0x102778d4 => { vm.decode(0x8cf3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102778d8u64) }
        0x102778d8 => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102778dcu64) }
        0x102778dc => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102778e0u64) }
        0x102778e0 => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102778e4u64) }
        0x102778e4 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102778e8u64) }
        0x102778e8 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102778ecu64) }
        0x102778ec => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102778f0u64 + 4);
        vm.wr(37, 0x102778f0u64 + 4);
        Ok(0x102778f0u64) }
        0x102778f0 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102778f4u64) }
        0x102778f4 => { vm.decode(0x28cd3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102778f8u64) }
        0x102778f8 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102778fcu64).wrapping_add((imm * 4) as u64) } else { 0x102778fcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102778fcu64) }
        0x102778fc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10277900u64) } }
        0x10277900 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10277904u64) }
        0x10277904 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10277908u64) }
        0x10277908 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027790cu64) }
        0x1027790c => { vm.decode(0x28cd3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277910u64) }
        0x10277910 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10277914u64) }
        0x10277914 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x10277918u64).wrapping_add((imm * 4) as u64) } else { 0x10277918u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10277918u64) }
        0x10277918 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1027791cu64) } }
        0x1027791c => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10277920u64).wrapping_add((imm * 4) as u64) } else { 0x10277920u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10277920u64) }
        0x10277974 => { vm.decode(0x20a4e06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277978u64) }
        0x10277978 => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027797cu64) }
        0x1027797c => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277980u64) }
        0x10277980 => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277984u64) }
        0x10277984 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277988u64) }
        0x10277988 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027798cu64 + 4);
        vm.wr(37, 0x1027798cu64 + 4);
        Ok(0x1027798cu64) }
        0x1027798c => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277990u64) }
        0x10277990 => { vm.decode(0x20a4c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277994u64) }
        0x10277994 => { vm.decode(0x288d3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277998u64) }
        0x10277998 => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027799cu64) }
        0x1027799c => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102779a0u64) }
        0x102779a0 => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102779a4u64) }
        0x102779a4 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102779a8u64) }
        0x102779a8 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102779acu64) }
        0x102779ac => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102779b0u64 + 4);
        vm.wr(37, 0x102779b0u64 + 4);
        Ok(0x102779b0u64) }
        0x102779b0 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102779b4u64) }
        0x102779b4 => { vm.decode(0x288f3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102779b8u64) }
        0x102779b8 => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102779bcu64).wrapping_add((imm * 4) as u64) } else { 0x102779bcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102779bcu64) }
        0x102779bc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102779c0u64) } }
        0x102779c0 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102779c4u64) }
        0x102779c4 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102779c8u64) }
        0x102779c8 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102779ccu64) }
        0x102779cc => { vm.decode(0x288f3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102779d0u64) }
        0x102779d0 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102779d4u64) }
        0x102779d4 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x102779d8u64).wrapping_add((imm * 4) as u64) } else { 0x102779d8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102779d8u64) }
        0x102779d8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102779dcu64) } }
        0x102779dc => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102779e0u64).wrapping_add((imm * 4) as u64) } else { 0x102779e0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102779e0u64) }
        0x10277a34 => { vm.decode(0x2084732fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277a38u64) }
        0x10277a38 => { vm.decode(0x888436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277a3cu64) }
        0x10277a3c => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10277a40u64) }
        0x10277a40 => { vm.decode(0x1b189cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277a44u64) }
        0x10277a44 => { vm.decode(0x20a4e06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277a48u64) }
        0x10277a48 => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277a4cu64) }
        0x10277a4c => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277a50u64) }
        0x10277a50 => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277a54u64) }
        0x10277a54 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277a58u64) }
        0x10277a58 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277a5cu64 + 4);
        vm.wr(37, 0x10277a5cu64 + 4);
        Ok(0x10277a5cu64) }
        0x10277a5c => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277a60u64) }
        0x10277a60 => { vm.decode(0x14068a27u32); let d = 0i16; let s = 17i16; let s2 = -1i16; let imm = 107i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10277a64u64).wrapping_add((imm * 4) as u64) } else { 0x10277a64u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10277a64u64) }
        0x10277a64 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10277a68u64) } }
        0x10277a68 => { vm.decode(0x688ce3efu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277a6cu64) }
        0x10277c10 => { vm.decode(0x2084732fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277c14u64) }
        0x10277c14 => { vm.decode(0x888636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277c18u64) }
        0x10277c18 => { vm.decode(0x820604f5u32); let d = 20i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10277c1cu64) }
        0x10277c1c => { vm.decode(0xa88436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277c20u64) }
        0x10277c20 => { vm.decode(0x8a4404f5u32); let d = 19i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10277c24u64) }
        0x10277c24 => { vm.decode(0x28cc3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277c28u64) }
        0x10277c28 => { vm.decode(0x824604f5u32); let d = 21i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10277c2cu64) }
        0x10277c2c => { vm.decode(0x28a0e06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277c30u64) }
        0x10277c30 => { vm.decode(0x3b1914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277c34u64) }
        0x10277c34 => { vm.decode(0x667d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3640i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277c38u64) }
        0x10277c38 => { vm.decode(0x6a80e3efu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277c3cu64) }
        0x10277c3c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277c40u64) }
        0x10277c40 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277c44u64 + 4);
        vm.wr(37, 0x10277c44u64 + 4);
        Ok(0x10277c44u64) }
        0x10277c44 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277c48u64) }
        0x10277c48 => { vm.decode(0xf904251u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10277c4cu64) }
        0x10277c4c => { vm.decode(0x233d3862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10277c50u64) }
        0x10277c50 => { vm.decode(0x2aa0c06fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277c54u64) }
        0x10277c54 => { vm.decode(0x39019c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277c58u64) }
        0x10277c58 => { vm.decode(0x82e3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277c5cu64) }
        0x10277c5c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277c60u64) }
        0x10277c60 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277c64u64 + 4);
        vm.wr(37, 0x10277c64u64 + 4);
        Ok(0x10277c64u64) }
        0x10277c64 => { vm.decode(0x622b8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3456i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277c68u64) }
        0x10277c68 => { vm.decode(0x130db28eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -14614528i64;
        vm.st(d, imm as u64);
        Ok(0x10277c6cu64) }
        0x10277c6c => { vm.decode(0x21450bd8u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 26472i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x10277c70u64) }
        0x10277c70 => { vm.decode(0x48105875u32); let d = 18i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277c74u64) }
        0x10277c74 => { vm.decode(0x8cc3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277c78u64) }
        0x10277c78 => { vm.decode(0x8a0604f5u32); let d = 22i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10277c7cu64) }
        0x10277c7c => { vm.decode(0x616f0697u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2548i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = u32::from_le_bytes(b.try_into().unwrap());
        vm.st(d, v as u64);
        Ok(0x10277c80u64) }
        0x10277c80 => { vm.decode(0x2a8ce3afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277c84u64) }
        0x10277c84 => { vm.decode(0x60480835u32); let d = 2i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277c88u64) }
        0x10277c88 => { vm.decode(0x2b209db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277c8cu64) }
        0x10277c8c => { vm.decode(0x8c8f32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277c90u64) }
        0x10277c90 => { vm.decode(0x404c0835u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277c94u64) }
        0x10277c94 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277c98u64) }
        0x10277c98 => { vm.decode(0x3b309c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277c9cu64) }
        0x10277c9c => { vm.decode(0x9500011u32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10277ca0u64) }
        0x10277ca0 => { vm.decode(0xa8843efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277ca4u64) }
        0x10277ca4 => { vm.decode(0x3022031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 35i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10277ca8u64).wrapping_add((imm * 4) as u64) } else { 0x10277ca8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10277ca8u64) }
        0x10277ca8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10277cacu64) } }
        0x10277cac => { vm.decode(0x2084732fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277cb0u64) }
        0x10277d34 => { vm.decode(0x9500011u32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10277d38u64) }
        0x10277d38 => { vm.decode(0x48801875u32); let d = 22i16; let s = 0i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277d3cu64) }
        0x10277d3c => { vm.decode(0x479c6891u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2548i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10277d40u64) }
        0x10277d40 => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277d44u64) }
        0x10277d44 => { vm.decode(0x433d3862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10277d48u64) }
        0x10277d48 => { vm.decode(0x2a20e06fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277d4cu64) }
        0x10277d4c => { vm.decode(0x667d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3640i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277d50u64) }
        0x10277d50 => { vm.decode(0x3b1914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277d54u64) }
        0x10277d54 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277d58u64) }
        0x10277d58 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277d5cu64 + 4);
        vm.wr(37, 0x10277d5cu64 + 4);
        Ok(0x10277d5cu64) }
        0x10277d5c => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277d60u64) }
        0x10277d60 => { vm.decode(0x626d8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3416i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277d64u64) }
        0x10277d64 => { vm.decode(0xf904251u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10277d68u64) }
        0x10277d68 => { vm.decode(0x633d1862u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10277d6cu64) }
        0x10277d6c => { vm.decode(0x2824c06fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277d70u64) }
        0x10277d70 => { vm.decode(0x39391c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277d74u64) }
        0x10277d74 => { vm.decode(0x2a8443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277d78u64) }
        0x10277d78 => { vm.decode(0x39211cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277d7cu64) }
        0x10277d7c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277d80u64) }
        0x10277d80 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277d84u64 + 4);
        vm.wr(37, 0x10277d84u64 + 4);
        Ok(0x10277d84u64) }
        0x10277d84 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277d88u64) }
        0x10277d88 => { vm.decode(0x20c4f36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277d8cu64) }
        0x10277d8c => { vm.decode(0x3b209c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277d90u64) }
        0x10277d90 => { vm.decode(0x2020402fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277d94u64) }
        0x10277d94 => { vm.decode(0x39119cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277d98u64) }
        0x10277d98 => { vm.decode(0x86532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277d9cu64) }
        0x10277d9c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277da0u64) }
        0x10277da0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277da4u64 + 4);
        vm.wr(37, 0x10277da4u64 + 4);
        Ok(0x10277da4u64) }
        0x10277da4 => { vm.decode(0x627b8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3504i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277da8u64) }
        0x10277da8 => { vm.decode(0x2ac4f32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277dacu64) }
        0x10277dac => { vm.decode(0x6008012fu32); let d = 19i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277db0u64) }
        0x10277db0 => { vm.decode(0x2820602fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277db4u64) }
        0x10277db4 => { vm.decode(0xbb29d430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 3784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277db8u64) }
        0x10277db8 => { vm.decode(0xab21d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277dbcu64) }
        0x10277dbc => { vm.decode(0x3b1994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277dc0u64) }
        0x10277dc0 => { vm.decode(0x282732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1048i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277dc4u64) }
        0x10277dc4 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277dc8u64) }
        0x10277dc8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277dccu64 + 4);
        vm.wr(37, 0x10277dccu64 + 4);
        Ok(0x10277dccu64) }
        0x10277dcc => { vm.decode(0x667f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3768i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277dd0u64) }
        0x10277dd0 => { vm.decode(0x627d8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3384i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277dd4u64) }
        0x10277dd4 => { vm.decode(0x2824e06fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277dd8u64) }
        0x10277dd8 => { vm.decode(0xa9295d70u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277ddcu64) }
        0x10277ddc => { vm.decode(0xb9211d30u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 3392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277de0u64) }
        0x10277de0 => { vm.decode(0x39191cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277de4u64) }
        0x10277de4 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277de8u64) }
        0x10277de8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277decu64 + 4);
        vm.wr(37, 0x10277decu64 + 4);
        Ok(0x10277decu64) }
        0x10277dec => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277df0u64) }
        0x10277df0 => { vm.decode(0x70198219u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277df4u64) }
        0x10277df4 => { vm.decode(0x2820402fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277df8u64) }
        0x10277df8 => { vm.decode(0x8d0855f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277dfcu64) }
        0x10277dfc => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277e00u64) }
        0x10277e00 => { vm.decode(0x62c8d36fu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 3408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277e04u64) }
        0x10277e04 => { vm.decode(0x4286532fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277e08u64) }
        0x10277e08 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277e0cu64) }
        0x10277e0c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277e10u64) }
        0x10277e10 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277e14u64 + 4);
        vm.wr(37, 0x10277e14u64 + 4);
        Ok(0x10277e14u64) }
        0x10277e14 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277e18u64) }
        0x10277e18 => { vm.decode(0xc72cf59u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = -16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277e1cu64) }
        0x10277e1c => { vm.decode(0x66690059u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1808i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277e20u64) }
        0x10277e20 => { vm.decode(0x2082e3efu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277e24u64) }
        0x10277e24 => { vm.decode(0x44900011u32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10277e28u64) }
        0x10277e28 => { vm.decode(0x2a8ce3efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277e2cu64) }
        0x10277e2c => { vm.decode(0x19219db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277e30u64) }
        0x10277e30 => { vm.decode(0x2888e3afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277e34u64) }
        0x10277e34 => { vm.decode(0x19199d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277e38u64) }
        0x10277e38 => { vm.decode(0x31217f5u32); let d = 1i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277e3cu64) }
        0x10277e3c => { vm.decode(0xc2f0c02du32); let d = 3i16; let s = 19i16; let s2 = -1i16; let imm = 240i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10277e40u64) }
        0x10277e40 => { vm.decode(0x919003b5u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10277e44u64) }
        0x10277e44 => { vm.decode(0x4488875u32); let d = 1i16; let s = 3i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277e48u64) }
        0x10277e48 => { vm.decode(0x70030d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 28i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10277e4cu64) }
        0x10277e4c => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10277e50u64) }
        0x10277e50 => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277e54u64) }
        0x10277e54 => { vm.decode(0x3b289c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277e58u64) }
        0x10277e58 => { vm.decode(0x288463efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277e5cu64) }
        0x10277e5c => { vm.decode(0x2200012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277e60u64) }
        0x10277e60 => { vm.decode(0x19019cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277e64u64) }
        0x10277e64 => { vm.decode(0xc900011u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10277e68u64) }
        0x10277e68 => { vm.decode(0x1986349u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 7168i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10277e6cu64) }
        0x10277e6c => { vm.decode(0x4b353062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10277e70u64) }
        0x10277e70 => { vm.decode(0x2224602fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277e74u64) }
        0x10277e74 => { vm.decode(0xb285c30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277e78u64) }
        0x10277e78 => { vm.decode(0xd0854b0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277e7cu64) }
        0x10277e7c => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277e80u64) }
        0x10277e80 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277e84u64) }
        0x10277e84 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277e88u64) }
        0x10277e88 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277e8cu64 + 4);
        vm.wr(37, 0x10277e8cu64 + 4);
        Ok(0x10277e8cu64) }
        0x10277e8c => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277e90u64) }
        0x10277e90 => { vm.decode(0x647f1119u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1784i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277e94u64) }
        0x10277e94 => { vm.decode(0x2224602fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277e98u64) }
        0x10277e98 => { vm.decode(0xb189db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277e9cu64) }
        0x10277e9c => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277ea0u64) }
        0x10277ea0 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277ea4u64) }
        0x10277ea4 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277ea8u64) }
        0x10277ea8 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277eacu64) }
        0x10277eac => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277eb0u64 + 4);
        vm.wr(37, 0x10277eb0u64 + 4);
        Ok(0x10277eb0u64) }
        0x10277eb0 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277eb4u64) }
        0x10277eb4 => { vm.decode(0x643b1119u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1760i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277eb8u64) }
        0x10277eb8 => { vm.decode(0x2224602fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277ebcu64) }
        0x10277ebc => { vm.decode(0xb201db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277ec0u64) }
        0x10277ec0 => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277ec4u64) }
        0x10277ec4 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277ec8u64) }
        0x10277ec8 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277eccu64) }
        0x10277ecc => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277ed0u64) }
        0x10277ed0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10277ed4u64 + 4);
        vm.wr(37, 0x10277ed4u64 + 4);
        Ok(0x10277ed4u64) }
        0x10277ed4 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277ed8u64) }
        0x10277ed8 => { vm.decode(0x8000236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10277edcu64) }
        0x10277edc => { vm.decode(0x88b0827u32); let d = 0i16; let s = 22i16; let s2 = -1i16; let imm = 22i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10277ee0u64).wrapping_add((imm * 4) as u64) } else { 0x10277ee0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10277ee0u64) }
        0x10277ee0 => { vm.decode(0x890d875u32); let d = 21i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10277ee4u64) }
        0x10277ee4 => { vm.decode(0xd108091u32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10277ee8u64) }
        0x10277f38 => { vm.decode(0x4282e3afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277f3cu64) }
        0x10277f3c => { vm.decode(0x6084732fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277f40u64) }
        0x10277f40 => { vm.decode(0x6088c3afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277f44u64) }
        0x10277f44 => { vm.decode(0x2284e3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277f48u64) }
        0x10277f48 => { vm.decode(0xa14004f5u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10277f4cu64) }
        0x10277f4c => { vm.decode(0x1b101cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277f50u64) }
        0x10277f50 => { vm.decode(0x2080c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277f54u64) }
        0x10277f54 => { vm.decode(0xa14004f5u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10277f58u64) }
        0x10277f58 => { vm.decode(0x1b289cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277f5cu64) }
        0x10277f5c => { vm.decode(0x2280c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277f60u64) }
        0x10277f60 => { vm.decode(0xa14004f5u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10277f64u64) }
        0x10277f64 => { vm.decode(0x1b181cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277f68u64) }
        0x10277f68 => { vm.decode(0x2280e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277f6cu64) }
        0x10277f6c => { vm.decode(0xa90604f5u32); let d = 22i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10277f70u64) }
        0x10277f70 => { vm.decode(0x80412du32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10277f74u64) }
        0x10277f74 => { vm.decode(0x647b1019u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1712i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277f78u64) }
        0x10277f78 => { vm.decode(0xb381d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277f7cu64) }
        0x10277f7c => { vm.decode(0x642f1119u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1736i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277f80u64) }
        0x10277f80 => { vm.decode(0xb189d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277f84u64) }
        0x10277f84 => { vm.decode(0x642b1019u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1664i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277f88u64) }
        0x10277f88 => { vm.decode(0xb009db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277f8cu64) }
        0x10277f8c => { vm.decode(0x643d1119u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1640i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277f90u64) }
        0x10277f90 => { vm.decode(0xb201d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277f94u64) }
        0x10277f94 => { vm.decode(0x646f1019u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1688i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10277f98u64) }
        0x10277f98 => { vm.decode(0xb301d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10277f9cu64) }
        0x10277f9c => { vm.decode(0x488863efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10277fa0u64) }
        0x10277fa0 => { vm.decode(0x2222071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10277fa4u64).wrapping_add((imm * 4) as u64) } else { 0x10277fa4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10277fa4u64) }
        0x10277fa4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10277fa8u64) } }
        0x10277fa8 => { vm.decode(0x80a0827u32); let d = 0i16; let s = 18i16; let s2 = -1i16; let imm = 18i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10277facu64).wrapping_add((imm * 4) as u64) } else { 0x10277facu64 + 4 });
        vm.wr(35, 2);
        Ok(0x10277facu64) }
        0x10278000 => { vm.decode(0x2080436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278004u64) }
        0x10278004 => { vm.decode(0xa14004f5u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10278008u64) }
        0x10278008 => { vm.decode(0xa024011du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 12484i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x1027800cu64) }
        0x1027800c => { vm.decode(0x19319c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278010u64) }
        0x10278010 => { vm.decode(0x8b10ddb0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278014u64) }
        0x10278584 => { vm.decode(0x2884436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278588u64) }
        0x10278588 => { vm.decode(0xa14004f5u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027858cu64) }
        0x1027858c => { vm.decode(0x1b309430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278590u64) }
        0x10278590 => { vm.decode(0x2884636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278594u64) }
        0x10278594 => { vm.decode(0xa14004f5u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10278598u64) }
        0x10278598 => { vm.decode(0x1b289430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027859cu64) }
        0x1027859c => { vm.decode(0x4088536fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102785a0u64) }
        0x102785a0 => { vm.decode(0x2020402fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102785a4u64) }
        0x102785a4 => { vm.decode(0x39119cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102785a8u64) }
        0x102785a8 => { vm.decode(0x86532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102785acu64) }
        0x102785ac => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102785b0u64) }
        0x102785b0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102785b4u64 + 4);
        vm.wr(37, 0x102785b4u64 + 4);
        Ok(0x102785b4u64) }
        0x102785b4 => { vm.decode(0x627b8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3504i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102785b8u64) }
        0x102785b8 => { vm.decode(0x2ac4f32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102785bcu64) }
        0x102785bc => { vm.decode(0x6000012fu32); let d = 17i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102785c0u64) }
        0x102785c0 => { vm.decode(0x2820602fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102785c4u64) }
        0x102785c4 => { vm.decode(0xab29d530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 3784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102785c8u64) }
        0x102785c8 => { vm.decode(0xab21d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102785ccu64) }
        0x102785cc => { vm.decode(0x3b1994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102785d0u64) }
        0x102785d0 => { vm.decode(0x282732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1048i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102785d4u64) }
        0x102785d4 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102785d8u64) }
        0x102785d8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102785dcu64 + 4);
        vm.wr(37, 0x102785dcu64 + 4);
        Ok(0x102785dcu64) }
        0x102785dc => { vm.decode(0x667f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3768i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102785e0u64) }
        0x102785e0 => { vm.decode(0x2020602fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102785e4u64) }
        0x102785e4 => { vm.decode(0xa901ddf0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102785e8u64) }
        0x102785e8 => { vm.decode(0xb9199c30u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 3480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102785ecu64) }
        0x102785ec => { vm.decode(0x39119c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102785f0u64) }
        0x102785f0 => { vm.decode(0x282532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102785f4u64) }
        0x102785f4 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102785f8u64) }
        0x102785f8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102785fcu64 + 4);
        vm.wr(37, 0x102785fcu64 + 4);
        Ok(0x102785fcu64) }
        0x102785fc => { vm.decode(0x626b8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3472i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10278600u64) }
        0x10278600 => { vm.decode(0x70198219u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10278604u64) }
        0x10278604 => { vm.decode(0x2820402fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278608u64) }
        0x10278608 => { vm.decode(0x8d0855f0u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027860cu64) }
        0x1027860c => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278610u64) }
        0x10278610 => { vm.decode(0xf1ce051u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10278614u64) }
        0x10278614 => { vm.decode(0x232d3062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10278618u64) }
        0x10278618 => { vm.decode(0x6286532fu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027861cu64) }
        0x1027861c => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278620u64) }
        0x10278620 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278624u64) }
        0x10278624 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278628u64 + 4);
        vm.wr(37, 0x10278628u64 + 4);
        Ok(0x10278628u64) }
        0x10278628 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027862cu64) }
        0x1027862c => { vm.decode(0x2a24e06fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278630u64) }
        0x10278630 => { vm.decode(0x288e3efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278634u64) }
        0x10278634 => { vm.decode(0x29399d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278638u64) }
        0x10278638 => { vm.decode(0x39319c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027863cu64) }
        0x1027863c => { vm.decode(0xa86632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278640u64) }
        0x10278640 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278644u64) }
        0x10278644 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278648u64 + 4);
        vm.wr(37, 0x10278648u64 + 4);
        Ok(0x10278648u64) }
        0x10278648 => { vm.decode(0x626b8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3536i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1027864cu64) }
        0x1027864c => { vm.decode(0x703b9519u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 3296i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10278650u64) }
        0x10278650 => { vm.decode(0x448019u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10278654u64) }
        0x10278654 => { vm.decode(0x22c402fu32); let d = 2i16; let s = 18i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278658u64) }
        0x10278658 => { vm.decode(0x392994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027865cu64) }
        0x1027865c => { vm.decode(0x292195b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278660u64) }
        0x10278660 => { vm.decode(0x4286c3afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278664u64) }
        0x10278664 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278668u64) }
        0x10278668 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027866cu64) }
        0x1027866c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278670u64 + 4);
        vm.wr(37, 0x10278670u64 + 4);
        Ok(0x10278670u64) }
        0x10278670 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278674u64) }
        0x10278674 => { vm.decode(0x2224602fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278678u64) }
        0x10278678 => { vm.decode(0x4ace536fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 3312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027867cu64) }
        0x1027867c => { vm.decode(0x8d0855b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278680u64) }
        0x10278680 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278684u64) }
        0x10278684 => { vm.decode(0x4904875u32); let d = 4i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278688u64) }
        0x10278688 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027868cu64) }
        0x1027868c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278690u64 + 4);
        vm.wr(37, 0x10278690u64 + 4);
        Ok(0x10278690u64) }
        0x10278690 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278694u64) }
        0x10278694 => { vm.decode(0x2b0094b0u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278698u64) }
        0x10278698 => { vm.decode(0x2b189430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027869cu64) }
        0x1027869c => { vm.decode(0x2024c06fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102786a0u64) }
        0x102786a0 => { vm.decode(0xab19d530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102786a4u64) }
        0x102786a4 => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102786a8u64) }
        0x102786a8 => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102786acu64) }
        0x102786ac => { vm.decode(0x82532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102786b0u64) }
        0x102786b0 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102786b4u64) }
        0x102786b4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102786b8u64 + 4);
        vm.wr(37, 0x102786b8u64 + 4);
        Ok(0x102786b8u64) }
        0x102786b8 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102786bcu64) }
        0x102786bc => { vm.decode(0xa019u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102786c0u64) }
        0x102786c0 => { vm.decode(0xab18d530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 1688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102786c4u64) }
        0x102786c4 => { vm.decode(0x22c402fu32); let d = 2i16; let s = 18i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102786c8u64) }
        0x102786c8 => { vm.decode(0x392994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102786ccu64) }
        0x102786cc => { vm.decode(0x292195b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102786d0u64) }
        0x102786d0 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102786d4u64) }
        0x102786d4 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102786d8u64) }
        0x102786d8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102786dcu64 + 4);
        vm.wr(37, 0x102786dcu64 + 4);
        Ok(0x102786dcu64) }
        0x102786dc => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102786e0u64) }
        0x102786e0 => { vm.decode(0xacc536fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102786e4u64) }
        0x102786e4 => { vm.decode(0x4000022u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102786e8u64) }
        0x102786e8 => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102786ecu64).wrapping_add((imm * 4) as u64) } else { 0x102786ecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102786ecu64) }
        0x102786ec => { vm.decode(0x2b0095b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102786f0u64) }
        0x102786f0 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102786f4u64) }
        0x102786f4 => { vm.decode(0x8003036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102786f8u64) }
        0x102786f8 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102786fcu64) }
        0x102786fc => { vm.decode(0x2084732fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278700u64) }
        0x10278700 => { vm.decode(0xa8c436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278704u64) }
        0x10278704 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10278708u64) }
        0x10278708 => { vm.decode(0x93095b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027870cu64) }
        0x1027870c => { vm.decode(0xa8c636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278710u64) }
        0x10278710 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10278714u64) }
        0x10278714 => { vm.decode(0x93895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278718u64) }
        0x10278718 => { vm.decode(0x88c32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027871cu64) }
        0x1027871c => { vm.decode(0x8a4404f5u32); let d = 19i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10278720u64) }
        0x10278720 => { vm.decode(0x88e32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278724u64) }
        0x10278724 => { vm.decode(0x824404f5u32); let d = 17i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10278728u64) }
        0x10278728 => { vm.decode(0x288c32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027872cu64) }
        0x1027872c => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10278730u64) }
        0x10278730 => { vm.decode(0xb209530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278734u64) }
        0x10278734 => { vm.decode(0x288e32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278738u64) }
        0x10278738 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027873cu64) }
        0x1027873c => { vm.decode(0x9181d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278740u64) }
        0x10278740 => { vm.decode(0x8cc32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278744u64) }
        0x10278744 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10278748u64) }
        0x10278748 => { vm.decode(0x9001db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027874cu64) }
        0x1027874c => { vm.decode(0x8ce32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278750u64) }
        0x10278750 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10278754u64) }
        0x10278754 => { vm.decode(0x9081db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278758u64) }
        0x10278758 => { vm.decode(0x28cc32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027875cu64) }
        0x1027875c => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10278760u64) }
        0x10278760 => { vm.decode(0x9101db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278764u64) }
        0x10278764 => { vm.decode(0x28ce32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278768u64) }
        0x10278768 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027876cu64) }
        0x1027876c => { vm.decode(0x9181db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278770u64) }
        0x10278770 => { vm.decode(0x88c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278774u64) }
        0x10278774 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10278778u64) }
        0x10278778 => { vm.decode(0x9201d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027877cu64) }
        0x1027877c => { vm.decode(0x88e36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278780u64) }
        0x10278780 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10278784u64) }
        0x10278784 => { vm.decode(0x9281d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278788u64) }
        0x10278788 => { vm.decode(0x288c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027878cu64) }
        0x1027878c => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10278790u64) }
        0x10278790 => { vm.decode(0x9301d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278794u64) }
        0x10278794 => { vm.decode(0x288e36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278798u64) }
        0x10278798 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027879cu64) }
        0x1027879c => { vm.decode(0x9381d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102787a0u64) }
        0x102787a0 => { vm.decode(0x8cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102787a4u64) }
        0x102787a4 => { vm.decode(0x8a4604f5u32); let d = 23i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102787a8u64) }
        0x102787a8 => { vm.decode(0x2024e06fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102787acu64) }
        0x102787ac => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102787b0u64) }
        0x102787b0 => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102787b4u64) }
        0x102787b4 => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102787b8u64) }
        0x102787b8 => { vm.decode(0x408a736fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102787bcu64) }
        0x102787bc => { vm.decode(0xa10d875u32); let d = 25i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102787c0u64) }
        0x102787c0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102787c4u64 + 4);
        vm.wr(37, 0x102787c4u64 + 4);
        Ok(0x102787c4u64) }
        0x102787c4 => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102787c8u64) }
        0x102787c8 => { vm.decode(0x2080536fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102787ccu64) }
        0x102787cc => { vm.decode(0x2000a12fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102787d0u64) }
        0x102787d0 => { vm.decode(0x451c6a11u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102787d4u64) }
        0x102787d4 => { vm.decode(0x4a357822u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102787d8u64) }
        0x102787d8 => { vm.decode(0x88863efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102787dcu64) }
        0x102787dc => { vm.decode(0x29319530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102787e0u64) }
        0x102787e0 => { vm.decode(0xa8873afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102787e4u64) }
        0x102787e4 => { vm.decode(0x29299530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102787e8u64) }
        0x102787e8 => { vm.decode(0x39219430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102787ecu64) }
        0x102787ec => { vm.decode(0xa8243efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102787f0u64) }
        0x102787f0 => { vm.decode(0xa10d875u32); let d = 25i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102787f4u64) }
        0x102787f4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102787f8u64 + 4);
        vm.wr(37, 0x102787f8u64 + 4);
        Ok(0x102787f8u64) }
        0x102787f8 => { vm.decode(0x622b8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3264i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102787fcu64) }
        0x102787fc => { vm.decode(0x2080536fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278800u64) }
        0x10278800 => { vm.decode(0x2200216fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278804u64) }
        0x10278804 => { vm.decode(0xa8873afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278808u64) }
        0x10278808 => { vm.decode(0x29111d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027880cu64) }
        0x1027880c => { vm.decode(0xa8c53afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278810u64) }
        0x10278810 => { vm.decode(0x29091d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278814u64) }
        0x10278814 => { vm.decode(0x488cc3efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278818u64) }
        0x10278818 => { vm.decode(0xa9011d30u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 3328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027881cu64) }
        0x1027881c => { vm.decode(0x393994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278820u64) }
        0x10278820 => { vm.decode(0x886e3efu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278824u64) }
        0x10278824 => { vm.decode(0xa10d875u32); let d = 25i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278828u64) }
        0x10278828 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027882cu64 + 4);
        vm.wr(37, 0x1027882cu64 + 4);
        Ok(0x1027882cu64) }
        0x1027882c => { vm.decode(0x627f8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3320i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10278830u64) }
        0x10278830 => { vm.decode(0x8000236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10278834u64) }
        0x10278834 => { vm.decode(0x2080536fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278838u64) }
        0x10278838 => { vm.decode(0x2200016fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027883cu64) }
        0x1027883c => { vm.decode(0x6a186873u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10278840u64) }
        0x10278840 => { vm.decode(0xa9091db0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 3368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278844u64) }
        0x10278844 => { vm.decode(0xa88e3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278848u64) }
        0x10278848 => { vm.decode(0x29011db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027884cu64) }
        0x1027884c => { vm.decode(0x39191c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278850u64) }
        0x10278850 => { vm.decode(0x286e3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278854u64) }
        0x10278854 => { vm.decode(0xa10d875u32); let d = 25i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278858u64) }
        0x10278858 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x1027885cu64 + 4);
        vm.wr(37, 0x1027885cu64 + 4);
        Ok(0x1027885cu64) }
        0x1027885c => { vm.decode(0x626d8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3352i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10278860u64) }
        0x10278860 => { vm.decode(0x2080536fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278864u64) }
        0x10278864 => { vm.decode(0x200001afu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278868u64) }
        0x10278868 => { vm.decode(0x88873efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027886cu64) }
        0x1027886c => { vm.decode(0x2b011d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278870u64) }
        0x10278870 => { vm.decode(0x3b3994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278874u64) }
        0x10278874 => { vm.decode(0x282c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278878u64) }
        0x10278878 => { vm.decode(0xa10d875u32); let d = 25i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1027887cu64) }
        0x1027887c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278880u64 + 4);
        vm.wr(37, 0x10278880u64 + 4);
        Ok(0x10278880u64) }
        0x10278880 => { vm.decode(0x667f8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3832i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10278884u64) }
        0x10278884 => { vm.decode(0xf10e251u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10278888u64) }
        0x10278888 => { vm.decode(0x63051062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1027888cu64) }
        0x1027888c => { vm.decode(0x2080536fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278890u64) }
        0x10278890 => { vm.decode(0x2004816fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278894u64) }
        0x10278894 => { vm.decode(0x88873efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278898u64) }
        0x10278898 => { vm.decode(0x2b199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027889cu64) }
        0x1027889c => { vm.decode(0xab11d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102788a0u64) }
        0x102788a0 => { vm.decode(0x3b099430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102788a4u64) }
        0x102788a4 => { vm.decode(0x82532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102788a8u64) }
        0x102788a8 => { vm.decode(0xa10d875u32); let d = 25i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102788acu64) }
        0x102788ac => { vm.decode(0x4088536fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102788b0u64) }
        0x102788b0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102788b4u64 + 4);
        vm.wr(37, 0x102788b4u64 + 4);
        Ok(0x102788b4u64) }
        0x102788b4 => { vm.decode(0x662f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3720i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102788b8u64) }
        0x102788b8 => { vm.decode(0xa8853efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102788bcu64) }
        0x102788bc => { vm.decode(0x8880a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 30i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102788c0u64).wrapping_add((imm * 4) as u64) } else { 0x102788c0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102788c0u64) }
        0x102788c0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102788c4u64) } }
        0x102788c4 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102788c8u64) }
        0x102788c8 => { vm.decode(0xbc0ffdf6u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102788ccu64) }
        0x102788cc => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102788d0u64) }
        0x102788d0 => { vm.decode(0xa8853efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102788d4u64) }
        0x102788d4 => { vm.decode(0x8400011u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102788d8u64) }
        0x102788d8 => { vm.decode(0x10900484u32); let d = -1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        vm.wr(36, if (vm.ld(s) as i64) > 0 { (0x102788dcu64).wrapping_add((imm * 4) as u64) } else { 0x102788dcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102788dcu64) }
        0x102788dc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102788e0u64) } }
        0x102788e0 => { vm.decode(0x8000227u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102788e4u64).wrapping_add((imm * 4) as u64) } else { 0x102788e4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102788e4u64) }
        0x102788e4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102788e8u64) } }
        0x102788e8 => { vm.decode(0x2220c06fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102788ecu64) }
        0x102788ec => { vm.decode(0x2b311530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102788f0u64) }
        0x102788f0 => { vm.decode(0x3b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102788f4u64) }
        0x102788f4 => { vm.decode(0xa82c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102788f8u64) }
        0x102788f8 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102788fcu64) }
        0x102788fc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278900u64 + 4);
        vm.wr(37, 0x10278900u64 + 4);
        Ok(0x10278900u64) }
        0x10278900 => { vm.decode(0x662d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3656i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10278904u64) }
        0x10278904 => { vm.decode(0x2b309430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278908u64) }
        0x10278908 => { vm.decode(0x88873efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027890cu64) }
        0x1027890c => { vm.decode(0x8000a27u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x10278910u64).wrapping_add((imm * 4) as u64) } else { 0x10278910u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10278910u64) }
        0x10278910 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10278914u64) } }
        0x10278914 => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278918u64) }
        0x10278918 => { vm.decode(0x2000212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027891cu64) }
        0x1027891c => { vm.decode(0xd0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278920u64) }
        0x10278920 => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278924u64) }
        0x10278924 => { vm.decode(0x286532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278928u64) }
        0x10278928 => { vm.decode(0x60807b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027892cu64) }
        0x1027892c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278930u64 + 4);
        vm.wr(37, 0x10278930u64 + 4);
        Ok(0x10278930u64) }
        0x10278930 => { vm.decode(0x62198219u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10278934u64) }
        0x10278934 => { vm.decode(0x2b289430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 1736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278938u64) }
        0x10278938 => { vm.decode(0x2880c32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027893cu64) }
        0x1027893c => { vm.decode(0x88c43efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278940u64) }
        0x10278940 => { vm.decode(0x604004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10278944u64) }
        0x10278944 => { vm.decode(0x39101430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278948u64) }
        0x10278948 => { vm.decode(0x2084732fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1064i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027894cu64) }
        0x1027894c => { vm.decode(0x8ce36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278950u64) }
        0x10278950 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10278954u64) }
        0x10278954 => { vm.decode(0xb2095b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278958u64) }
        0x10278958 => { vm.decode(0x28cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027895cu64) }
        0x1027895c => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10278960u64) }
        0x10278960 => { vm.decode(0x9009d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278964u64) }
        0x10278964 => { vm.decode(0x28ce36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278968u64) }
        0x10278968 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1027896cu64) }
        0x1027896c => { vm.decode(0x9381db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278970u64) }
        0x10278970 => { vm.decode(0x888e32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278974u64) }
        0x10278974 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10278978u64) }
        0x10278978 => { vm.decode(0xb309530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027897cu64) }
        0x1027897c => { vm.decode(0xa88c32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278980u64) }
        0x10278980 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10278984u64) }
        0x10278984 => { vm.decode(0xb289530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278988u64) }
        0x10278988 => { vm.decode(0xa88e32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1027898cu64) }
        0x1027898c => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10278990u64) }
        0x10278990 => { vm.decode(0x19189c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278994u64) }
        0x10278994 => { vm.decode(0x2024e06fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278998u64) }
        0x10278998 => { vm.decode(0xab01d570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 3712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1027899cu64) }
        0x1027899c => { vm.decode(0x3b3914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102789a0u64) }
        0x102789a0 => { vm.decode(0x286732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1080i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102789a4u64) }
        0x102789a4 => { vm.decode(0x408a736fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 1096i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102789a8u64) }
        0x102789a8 => { vm.decode(0xa10d875u32); let d = 25i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102789acu64) }
        0x102789ac => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102789b0u64 + 4);
        vm.wr(37, 0x102789b0u64 + 4);
        Ok(0x102789b0u64) }
        0x102789b0 => { vm.decode(0x667d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3704i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102789b4u64) }
        0x102789b4 => { vm.decode(0x2080536fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102789b8u64) }
        0x102789b8 => { vm.decode(0x2800012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102789bcu64) }
        0x102789bc => { vm.decode(0x62198219u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102789c0u64) }
        0x102789c0 => { vm.decode(0x1910dc30u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102789c4u64) }
        0x102789c4 => { vm.decode(0x488cc3efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102789c8u64) }
        0x102789c8 => { vm.decode(0x8d0815b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 4136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102789ccu64) }
        0x102789cc => { vm.decode(0x1d0014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 4128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102789d0u64) }
        0x102789d0 => { vm.decode(0x286532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1072i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102789d4u64) }
        0x102789d4 => { vm.decode(0xa10d875u32); let d = 25i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102789d8u64) }
        0x102789d8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102789dcu64 + 4);
        vm.wr(37, 0x102789dcu64 + 4);
        Ok(0x102789dcu64) }
        0x102789dc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102789e0u64) } }
        0x102789e0 => { vm.decode(0x623f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3240i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102789e4u64) }
        0x102789e4 => { vm.decode(0x2080536fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102789e8u64) }
        0x102789e8 => { vm.decode(0x200021afu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102789ecu64) }
        0x102789ec => { vm.decode(0xb919d5b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 3256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102789f0u64) }
        0x102789f0 => { vm.decode(0x88cc3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102789f4u64) }
        0x102789f4 => { vm.decode(0x291195b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102789f8u64) }
        0x102789f8 => { vm.decode(0x390994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102789fcu64) }
        0x102789fc => { vm.decode(0xa10d875u32); let d = 25i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278a00u64) }
        0x10278a00 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278a04u64 + 4);
        vm.wr(37, 0x10278a04u64 + 4);
        Ok(0x10278a04u64) }
        0x10278a04 => { vm.decode(0x4900875u32); let d = 4i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278a08u64) }
        0x10278a08 => { vm.decode(0x767d8419u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 3640i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10278a0cu64) }
        0x10278a0c => { vm.decode(0x2080536fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278a10u64) }
        0x10278a10 => { vm.decode(0x220021afu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278a14u64) }
        0x10278a14 => { vm.decode(0x3b1914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278a18u64) }
        0x10278a18 => { vm.decode(0x688a53afu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 1664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278a1cu64) }
        0x10278a1c => { vm.decode(0x4a82e3efu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278a20u64) }
        0x10278a20 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278a24u64) }
        0x10278a24 => { vm.decode(0xa10d875u32); let d = 25i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278a28u64) }
        0x10278a28 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278a2cu64 + 4);
        vm.wr(37, 0x10278a2cu64 + 4);
        Ok(0x10278a2cu64) }
        0x10278a2c => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278a30u64) }
        0x10278a30 => { vm.decode(0x702b8419u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 3200i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10278a34u64) }
        0x10278a34 => { vm.decode(0x2080536fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278a38u64) }
        0x10278a38 => { vm.decode(0x220001afu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278a3cu64) }
        0x10278a3c => { vm.decode(0x288e36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278a40u64) }
        0x10278a40 => { vm.decode(0x29119530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278a44u64) }
        0x10278a44 => { vm.decode(0xb909d530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 3208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278a48u64) }
        0x10278a48 => { vm.decode(0x39019430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278a4cu64) }
        0x10278a4c => { vm.decode(0x7904351u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 3648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10278a50u64) }
        0x10278a50 => { vm.decode(0x4890d875u32); let d = 23i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278a54u64) }
        0x10278a54 => { vm.decode(0x2400001au32); let d = 2i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32) as f64;
        vm.st(d, v.to_bits());
        Ok(0x10278a58u64) }
        0x10278a58 => { vm.decode(0x29199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278a5cu64) }
        0x10278a5c => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278a60u64) }
        0x10278a60 => { vm.decode(0xa10d875u32); let d = 25i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278a64u64) }
        0x10278a64 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278a68u64 + 4);
        vm.wr(37, 0x10278a68u64 + 4);
        Ok(0x10278a68u64) }
        0x10278a68 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278a6cu64) }
        0x10278a6c => { vm.decode(0x2080536fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278a70u64) }
        0x10278a70 => { vm.decode(0x200401afu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278a74u64) }
        0x10278a74 => { vm.decode(0x3b1914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278a78u64) }
        0x10278a78 => { vm.decode(0x488a53afu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 1664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278a7cu64) }
        0x10278a7c => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278a80u64) }
        0x10278a80 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278a84u64) }
        0x10278a84 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278a88u64 + 4);
        vm.wr(37, 0x10278a88u64 + 4);
        Ok(0x10278a88u64) }
        0x10278a88 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278a8cu64) }
        0x10278a8c => { vm.decode(0x2080536fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278a90u64) }
        0x10278a90 => { vm.decode(0x220001afu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278a94u64) }
        0x10278a94 => { vm.decode(0x288c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278a98u64) }
        0x10278a98 => { vm.decode(0x29119530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278a9cu64) }
        0x10278a9c => { vm.decode(0xa909d530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 3208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278aa0u64) }
        0x10278aa0 => { vm.decode(0x39019430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278aa4u64) }
        0x10278aa4 => { vm.decode(0x7904351u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 3648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10278aa8u64) }
        0x10278aa8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10278aacu64) } }
        0x10278aac => { vm.decode(0x2400001au32); let d = 2i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32) as f64;
        vm.st(d, v.to_bits());
        Ok(0x10278ab0u64) }
        0x10278ab0 => { vm.decode(0x29199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278ab4u64) }
        0x10278ab4 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278ab8u64) }
        0x10278ab8 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278abcu64) }
        0x10278abc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278ac0u64 + 4);
        vm.wr(37, 0x10278ac0u64 + 4);
        Ok(0x10278ac0u64) }
        0x10278ac0 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278ac4u64) }
        0x10278ac4 => { vm.decode(0x2080536fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278ac8u64) }
        0x10278ac8 => { vm.decode(0x200421afu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278accu64) }
        0x10278acc => { vm.decode(0x3b1914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278ad0u64) }
        0x10278ad0 => { vm.decode(0x488a53afu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 1664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278ad4u64) }
        0x10278ad4 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278ad8u64) }
        0x10278ad8 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278adcu64) }
        0x10278adc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278ae0u64 + 4);
        vm.wr(37, 0x10278ae0u64 + 4);
        Ok(0x10278ae0u64) }
        0x10278ae0 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278ae4u64) }
        0x10278ae4 => { vm.decode(0x2080536fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278ae8u64) }
        0x10278ae8 => { vm.decode(0x220001afu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278aecu64) }
        0x10278aec => { vm.decode(0x88e36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278af0u64) }
        0x10278af0 => { vm.decode(0x29119530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278af4u64) }
        0x10278af4 => { vm.decode(0xa909d530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 3208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278af8u64) }
        0x10278af8 => { vm.decode(0x39019430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278afcu64) }
        0x10278afc => { vm.decode(0x7904351u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 3648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10278b00u64) }
        0x10278b00 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10278b04u64) } }
        0x10278b04 => { vm.decode(0x2400001au32); let d = 2i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32) as f64;
        vm.st(d, v.to_bits());
        Ok(0x10278b08u64) }
        0x10278b08 => { vm.decode(0x29199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278b0cu64) }
        0x10278b0c => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278b10u64) }
        0x10278b10 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278b14u64) }
        0x10278b14 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278b18u64 + 4);
        vm.wr(37, 0x10278b18u64 + 4);
        Ok(0x10278b18u64) }
        0x10278b18 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278b1cu64) }
        0x10278b1c => { vm.decode(0x2080536fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278b20u64) }
        0x10278b20 => { vm.decode(0x220401afu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278b24u64) }
        0x10278b24 => { vm.decode(0x3b1914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278b28u64) }
        0x10278b28 => { vm.decode(0x488a53afu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 1664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278b2cu64) }
        0x10278b2c => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278b30u64) }
        0x10278b30 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278b34u64) }
        0x10278b34 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278b38u64 + 4);
        vm.wr(37, 0x10278b38u64 + 4);
        Ok(0x10278b38u64) }
        0x10278b38 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278b3cu64) }
        0x10278b3c => { vm.decode(0x2080536fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278b40u64) }
        0x10278b40 => { vm.decode(0x220001afu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278b44u64) }
        0x10278b44 => { vm.decode(0x88c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278b48u64) }
        0x10278b48 => { vm.decode(0x29119530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278b4cu64) }
        0x10278b4c => { vm.decode(0xa909d530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 3208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278b50u64) }
        0x10278b50 => { vm.decode(0x408a536fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 1088i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278b54u64) }
        0x10278b54 => { vm.decode(0x39019430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278b58u64) }
        0x10278b58 => { vm.decode(0x7904351u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 3648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10278b5cu64) }
        0x10278b5c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10278b60u64) } }
        0x10278b60 => { vm.decode(0x2400001au32); let d = 2i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32) as f64;
        vm.st(d, v.to_bits());
        Ok(0x10278b64u64) }
        0x10278b64 => { vm.decode(0x29199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278b68u64) }
        0x10278b68 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278b6cu64) }
        0x10278b6c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278b70u64) }
        0x10278b70 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278b74u64 + 4);
        vm.wr(37, 0x10278b74u64 + 4);
        Ok(0x10278b74u64) }
        0x10278b74 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278b78u64) }
        0x10278b78 => { vm.decode(0xd1c6a11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10278b7cu64) }
        0x10278b7c => { vm.decode(0x840bb5u32); let d = 4i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10278b80u64) }
        0x10278b80 => { vm.decode(0x2400001au32); let d = 2i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32) as f64;
        vm.st(d, v.to_bits());
        Ok(0x10278b84u64) }
        0x10278b84 => { vm.decode(0x22a040afu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278b88u64) }
        0x10278b88 => { vm.decode(0x29199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278b8cu64) }
        0x10278b8c => { vm.decode(0x28ce32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278b90u64) }
        0x10278b90 => { vm.decode(0x29119530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278b94u64) }
        0x10278b94 => { vm.decode(0x88853afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278b98u64) }
        0x10278b98 => { vm.decode(0x29099530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278b9cu64) }
        0x10278b9c => { vm.decode(0x39019430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278ba0u64) }
        0x10278ba0 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278ba4u64) }
        0x10278ba4 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278ba8u64) }
        0x10278ba8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278bacu64 + 4);
        vm.wr(37, 0x10278bacu64 + 4);
        Ok(0x10278bacu64) }
        0x10278bac => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278bb0u64) }
        0x10278bb0 => { vm.decode(0xd946011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10278bb4u64) }
        0x10278bb4 => { vm.decode(0x840bb5u32); let d = 4i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10278bb8u64) }
        0x10278bb8 => { vm.decode(0x2400001au32); let d = 2i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32) as f64;
        vm.st(d, v.to_bits());
        Ok(0x10278bbcu64) }
        0x10278bbc => { vm.decode(0x22a040afu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278bc0u64) }
        0x10278bc0 => { vm.decode(0x29199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278bc4u64) }
        0x10278bc4 => { vm.decode(0x28cc32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278bc8u64) }
        0x10278bc8 => { vm.decode(0x29119530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278bccu64) }
        0x10278bcc => { vm.decode(0x88853afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278bd0u64) }
        0x10278bd0 => { vm.decode(0x29099530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278bd4u64) }
        0x10278bd4 => { vm.decode(0x39019430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278bd8u64) }
        0x10278bd8 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278bdcu64) }
        0x10278bdc => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278be0u64) }
        0x10278be0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278be4u64 + 4);
        vm.wr(37, 0x10278be4u64 + 4);
        Ok(0x10278be4u64) }
        0x10278be4 => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278be8u64) }
        0x10278be8 => { vm.decode(0x20a040afu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278becu64) }
        0x10278bec => { vm.decode(0xa8c53afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278bf0u64) }
        0x10278bf0 => { vm.decode(0x2b011d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278bf4u64) }
        0x10278bf4 => { vm.decode(0x3b3994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278bf8u64) }
        0x10278bf8 => { vm.decode(0x282c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278bfcu64) }
        0x10278bfc => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278c00u64) }
        0x10278c00 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278c04u64 + 4);
        vm.wr(37, 0x10278c04u64 + 4);
        Ok(0x10278c04u64) }
        0x10278c04 => { vm.decode(0x667f8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3832i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10278c08u64) }
        0x10278c08 => { vm.decode(0x22a040afu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278c0cu64) }
        0x10278c0c => { vm.decode(0x8ce32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278c10u64) }
        0x10278c10 => { vm.decode(0x29119530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278c14u64) }
        0x10278c14 => { vm.decode(0x88853afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278c18u64) }
        0x10278c18 => { vm.decode(0x29099530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278c1cu64) }
        0x10278c1c => { vm.decode(0x39019430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278c20u64) }
        0x10278c20 => { vm.decode(0x710e351u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 3848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10278c24u64) }
        0x10278c24 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10278c28u64) } }
        0x10278c28 => { vm.decode(0x2420001au32); let d = 2i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = ((vm.ld(s) as u32) as i32) as f64;
        vm.st(d, v.to_bits());
        Ok(0x10278c2cu64) }
        0x10278c2c => { vm.decode(0x29199530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278c30u64) }
        0x10278c30 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278c34u64) }
        0x10278c34 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278c38u64) }
        0x10278c38 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278c3cu64 + 4);
        vm.wr(37, 0x10278c3cu64 + 4);
        Ok(0x10278c3cu64) }
        0x10278c3c => { vm.decode(0x4908875u32); let d = 5i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278c40u64) }
        0x10278c40 => { vm.decode(0x20a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278c44u64) }
        0x10278c44 => { vm.decode(0x39119cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278c48u64) }
        0x10278c48 => { vm.decode(0x86532fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278c4cu64) }
        0x10278c4c => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278c50u64) }
        0x10278c50 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278c54u64 + 4);
        vm.wr(37, 0x10278c54u64 + 4);
        Ok(0x10278c54u64) }
        0x10278c54 => { vm.decode(0x627b8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3504i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10278c58u64) }
        0x10278c58 => { vm.decode(0x2ac4f32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278c5cu64) }
        0x10278c5c => { vm.decode(0x6002012fu32); let d = 21i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278c60u64) }
        0x10278c60 => { vm.decode(0x28a0602fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278c64u64) }
        0x10278c64 => { vm.decode(0x8cc32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278c68u64) }
        0x10278c68 => { vm.decode(0x2b299530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 3784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278c6cu64) }
        0x10278c6c => { vm.decode(0xab219530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 3776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278c70u64) }
        0x10278c70 => { vm.decode(0x3b1994b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 3768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10278c74u64) }
        0x10278c74 => { vm.decode(0x282732fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1048i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10278c78u64) }
        0x10278c78 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10278c7cu64) }
        0x10278c7c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10278c80u64 + 4);
        vm.wr(37, 0x10278c80u64 + 4);
        Ok(0x10278c80u64) }
        0x10278c80 => { vm.decode(0x667f8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 3768i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10278c84u64) }
        _ => Err(Error::UnknownPc(pc)),
    }
}
