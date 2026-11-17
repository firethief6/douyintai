#![cfg_attr(rustfmt, rustfmt::skip)]
// 由 scripts/medusa-vm/gen_native.py 生成，勿手改。
use super::super::Vm;
use super::super::Error;

#[allow(clippy::all, unused_parens, unused_variables, unused_mut)]
pub fn exec(vm: &mut Vm, pc: u64) -> Result<u64, Error> {
    match pc {
        0x102c7b08 => { vm.decode(0x8302875u32); let d = 16i16; let s = 16i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c7b0cu64) }
        0x102c7b0c => { vm.decode(0x24431835u32); let d = 16i16; let s = 15i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c7b10u64) }
        0x102c7b10 => { vm.decode(0x800c2fb5u32); let d = 16i16; let s = 16i16; let s2 = 18i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7b14u64) }
        0x102c7b14 => { vm.decode(0x318a2fb5u32); let d = 14i16; let s = 14i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7b18u64) }
        0x102c7b18 => { vm.decode(0x11926fb5u32); let d = 12i16; let s = 12i16; let s2 = 14i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7b1cu64) }
        0x102c7b1c => { vm.decode(0xe253d35u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 26i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7b20u64) }
        0x102c7b20 => { vm.decode(0x91124fb5u32); let d = 12i16; let s = 12i16; let s2 = 24i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7b24u64) }
        0x102c7b24 => { vm.decode(0x409129b5u32); let d = 14i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102c7b28u64) }
        0x102c7b28 => { vm.decode(0xb28e875u32); let d = 25i16; let s = 12i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c7b2cu64) }
        0x102c7b2c => { vm.decode(0x56123835u32); let d = 25i16; let s = 24i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c7b30u64) }
        0x102c7b30 => { vm.decode(0x15144a11u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c7b34u64) }
        0x102c7b34 => { vm.decode(0xc50c4fb5u32); let d = 25i16; let s = 16i16; let s2 = 25i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7b38u64) }
        0x102c7b38 => { vm.decode(0x75096fb5u32); let d = 11i16; let s = 11i16; let s2 = 25i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7b3cu64) }
        0x102c7b3c => { vm.decode(0x75116fb5u32); let d = 11i16; let s = 11i16; let s2 = 13i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7b40u64) }
        0x102c7b40 => { vm.decode(0x3c917d35u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 22i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7b44u64) }
        0x102c7b44 => { vm.decode(0x75a04fb5u32); let d = 11i16; let s = 11i16; let s2 = 12i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7b48u64) }
        0x102c7b48 => { vm.decode(0x509019b5u32); let d = 13i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102c7b4cu64) }
        0x102c7b4c => { vm.decode(0x57e01875u32); let d = 14i16; let s = 11i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c7b50u64) }
        0x102c7b50 => { vm.decode(0x130f8835u32); let d = 14i16; let s = 12i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c7b54u64) }
        0x102c7b54 => { vm.decode(0x1d14da11u32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c7b58u64) }
        0x102c7b58 => { vm.decode(0xb1334fb5u32); let d = 14i16; let s = 14i16; let s2 = 25i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7b5cu64) }
        0x102c7b5c => { vm.decode(0x31306fb5u32); let d = 10i16; let s = 10i16; let s2 = 14i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7b60u64) }
        0x102c7b60 => { vm.decode(0x35306fb5u32); let d = 10i16; let s = 10i16; let s2 = 15i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7b64u64) }
        0x102c7b64 => { vm.decode(0x2811bd35u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 17i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7b68u64) }
        0x102c7b68 => { vm.decode(0x31216fb5u32); let d = 10i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7b6cu64) }
        0x102c7b6c => { vm.decode(0x419309b5u32); let d = 14i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102c7b70u64) }
        0x102c7b70 => { vm.decode(0x13a89875u32); let d = 13i16; let s = 10i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c7b74u64) }
        0x102c7b74 => { vm.decode(0x7342a835u32); let d = 13i16; let s = 11i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c7b78u64) }
        0x102c7b78 => { vm.decode(0x4d905311u32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c7b7cu64) }
        0x102c7b7c => { vm.decode(0x55936fb5u32); let d = 13i16; let s = 13i16; let s2 = 15i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7b80u64) }
        0x102c7b80 => { vm.decode(0x24926fb5u32); let d = 6i16; let s = 6i16; let s2 = 13i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7b84u64) }
        0x102c7b84 => { vm.decode(0x208a6fb5u32); let d = 6i16; let s = 6i16; let s2 = 24i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7b88u64) }
        0x102c7b88 => { vm.decode(0x2e348d35u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 11i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7b8cu64) }
        0x102c7b8c => { vm.decode(0x30322fb5u32); let d = 6i16; let s = 6i16; let s2 = 10i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7b90u64) }
        0x102c7b90 => { vm.decode(0x509209b5u32); let d = 13i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102c7b94u64) }
        0x102c7b94 => { vm.decode(0x53c05875u32); let d = 14i16; let s = 6i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c7b98u64) }
        0x102c7b98 => { vm.decode(0x134e8835u32); let d = 14i16; let s = 10i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c7b9cu64) }
        0x102c7b9c => { vm.decode(0x4d90d311u32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c7ba0u64) }
        0x102c7ba0 => { vm.decode(0x31b36fb5u32); let d = 14i16; let s = 14i16; let s2 = 15i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7ba4u64) }
        0x102c7ba4 => { vm.decode(0x40b34fb5u32); let d = 5i16; let s = 5i16; let s2 = 14i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7ba8u64) }
        0x102c7ba8 => { vm.decode(0x40934fb5u32); let d = 5i16; let s = 5i16; let s2 = 12i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7bacu64) }
        0x102c7bac => { vm.decode(0x1e254d35u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 26i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7bb0u64) }
        0x102c7bb0 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7bb4u64) }
        0x102c7bb4 => { vm.decode(0x408219b5u32); let d = 12i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102c7bb8u64) }
        0x102c7bb8 => { vm.decode(0x1788c875u32); let d = 13i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c7bbcu64) }
        0x102c7bbc => { vm.decode(0x5341a835u32); let d = 13i16; let s = 6i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c7bc0u64) }
        0x102c7bc0 => { vm.decode(0x45905b11u32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c7bc4u64) }
        0x102c7bc4 => { vm.decode(0x55926fb5u32); let d = 13i16; let s = 13i16; let s2 = 14i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7bc8u64) }
        0x102c7bc8 => { vm.decode(0x4924fb5u32); let d = 4i16; let s = 4i16; let s2 = 13i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7bccu64) }
        0x102c7bcc => { vm.decode(0x4a24fb5u32); let d = 4i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7bd0u64) }
        0x102c7bd0 => { vm.decode(0xca50d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 22i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7bd4u64) }
        0x102c7bd4 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7bd8u64) }
        0x102c7bd8 => { vm.decode(0x64a10fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7bdcu64) }
        0x102c7bdc => { vm.decode(0x510119b5u32); let d = 11i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102c7be0u64) }
        0x102c7be0 => { vm.decode(0x13804875u32); let d = 12i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c7be4u64) }
        0x102c7be4 => { vm.decode(0x45804fb5u32); let d = 9i16; let s = 4i16; let s2 = 9i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7be8u64) }
        0x102c7be8 => { vm.decode(0x30d7062u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c7becu64) }
        0x102c7bec => { vm.decode(0x3009a835u32); let d = 3i16; let s = 5i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c7bf0u64) }
        0x102c7bf0 => { vm.decode(0xd90cb11u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c7bf4u64) }
        0x102c7bf4 => { vm.decode(0x64a10fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7bf8u64) }
        0x102c7bf8 => { vm.decode(0x24202fb5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7bfcu64) }
        0x102c7bfc => { vm.decode(0x20206fb5u32); let d = 2i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7c00u64) }
        0x102c7c00 => { vm.decode(0x28118d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 17i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7c04u64) }
        0x102c7c04 => { vm.decode(0x20a00fb5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7c08u64) }
        0x102c7c08 => { vm.decode(0x41489875u32); let d = 3i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c7c0cu64) }
        0x102c7c0c => { vm.decode(0x31f3462u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 796i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c7c10u64) }
        0x102c7c10 => { vm.decode(0x400d2835u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c7c14u64) }
        0x102c7c14 => { vm.decode(0x4322fb5u32); let d = 4i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7c18u64) }
        0x102c7c18 => { vm.decode(0x25d3062u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c7c1cu64) }
        0x102c7c1c => { vm.decode(0x5944b11u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c7c20u64) }
        0x102c7c20 => { vm.decode(0x64a00fb5u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7c24u64) }
        0x102c7c24 => { vm.decode(0x44210fb5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7c28u64) }
        0x102c7c28 => { vm.decode(0x40310fb5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7c2cu64) }
        0x102c7c2c => { vm.decode(0x1e00cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 11i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7c30u64) }
        0x102c7c30 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7c34u64) }
        0x102c7c34 => { vm.decode(0x40014fb5u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c7c38u64) }
        0x102c7c38 => { vm.decode(0x30d3462u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 788i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c7c3cu64) }
        0x102c7c3c => { vm.decode(0x408e6befu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c7c40u64) }
        0x102c7c40 => { vm.decode(0x408e43efu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c7c44u64) }
        0x102c7c44 => { vm.decode(0x641d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c7c48u64) }
        0x102c7c48 => { vm.decode(0x193894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c7c4cu64) }
        0x102c7c4c => { vm.decode(0x640d8519u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c7c50u64) }
        0x102c7c50 => { vm.decode(0x193094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c7c54u64) }
        0x102c7c54 => { vm.decode(0x8004e0d9u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 8463i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c7c58u64) }
        0x102c7c58 => { vm.decode(0xc002ad75u32); let d = 20i16; let s = 1i16; let s2 = -1i16; let imm = 18i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102c7c5cu64) }
        0x102c7c5c => { vm.decode(0x2c217d99u32); let d = 2i16; let s = 20i16; let s2 = -1i16; let imm = -4539i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c7c60u64) }
        0x102c7c60 => { vm.decode(0x12109019u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c7c64u64) }
        0x102c7c64 => { vm.decode(0x640d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c7c68u64) }
        0x102c7c68 => { vm.decode(0x1b3014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c7c6cu64) }
        0x102c7c6c => { vm.decode(0x4100236u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c7c70u64) }
        0x102c7c70 => { vm.decode(0x32028000u32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102c7c74u64) }
        0x102c7c74 => { vm.decode(0x40c0aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 186i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102c7c78u64).wrapping_add((imm * 4) as u64) } else { 0x102c7c78u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c7c78u64) }
        0x102c7c78 => { vm.decode(0x92895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c7c7cu64) }
        0x102c7c7c => { vm.decode(0xa9a0419du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 273132i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102c7c80u64) }
        0x102c7c80 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102c7c84u64) } }
        0x102c8e6c => { vm.decode(0xf8027u32); let d = 0i16; let s = 21i16; let s2 = -1i16; let imm = 17i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102c8e70u64).wrapping_add((imm * 4) as u64) } else { 0x102c8e70u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c8e70u64) }
        0x102c8e70 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102c8e74u64) } }
        0x102c8eb4 => { vm.decode(0x4a8063afu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c8eb8u64) }
        0x102c8eb8 => { vm.decode(0x2804402fu32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c8ebcu64) }
        0x102c8ebc => { vm.decode(0x10408019u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c8ec0u64) }
        0x102c8ec0 => { vm.decode(0x9b211430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c8ec4u64) }
        0x102c8ec4 => { vm.decode(0xa88636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c8ec8u64) }
        0x102c8ec8 => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c8eccu64) }
        0x102c8ecc => { vm.decode(0x8b1155b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c8ed0u64) }
        0x102c8ed0 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c8ed4u64) }
        0x102c8ed4 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c8ed8u64) }
        0x102c8ed8 => { vm.decode(0x488c43afu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c8edcu64) }
        0x102c8edc => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c8ee0u64) }
        0x102c8ee0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102c8ee4u64 + 4);
        vm.wr(37, 0x102c8ee4u64 + 4);
        Ok(0x102c8ee4u64) }
        0x102c8ee4 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c8ee8u64) }
        0x102c8ee8 => { vm.decode(0x2200406fu32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c8eecu64) }
        0x102c8eec => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c8ef0u64) }
        0x102c8ef0 => { vm.decode(0x9b2114b0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c8ef4u64) }
        0x102c8ef4 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c8ef8u64) }
        0x102c8ef8 => { vm.decode(0x8b315570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c8efcu64) }
        0x102c8efc => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c8f00u64) }
        0x102c8f00 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c8f04u64) }
        0x102c8f04 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c8f08u64) }
        0x102c8f08 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102c8f0cu64 + 4);
        vm.wr(37, 0x102c8f0cu64 + 4);
        Ok(0x102c8f0cu64) }
        0x102c8f0c => { vm.decode(0x100036u32); let d = 16i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c8f10u64) }
        0x102c8f10 => { vm.decode(0x8004e0d9u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 8463i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c8f14u64) }
        0x102c8f14 => { vm.decode(0x80422d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 18i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102c8f18u64) }
        0x102c8f18 => { vm.decode(0xdc24cd19u32); let d = 17i16; let s = 1i16; let s2 = -1i16; let imm = -4534i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c8f1cu64) }
        0x102c8f1c => { vm.decode(0x10101019u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c8f20u64) }
        0x102c8f20 => { vm.decode(0x740d9419u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c8f24u64) }
        0x102c8f24 => { vm.decode(0x4000446u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102c8f28u64) }
        0x102c8f28 => { vm.decode(0x2200031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102c8f2cu64).wrapping_add((imm * 4) as u64) } else { 0x102c8f2cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c8f2cu64) }
        0x102c8f2c => { vm.decode(0x6100036u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c8f30u64) }
        0x102c8f38 => { vm.decode(0x8624735u32); let d = 1i16; let s = 4i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c8f3cu64) }
        0x102c8f3c => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102c8f40u64) }
        0x102c8f40 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c8f44u64) }
        0x102c8f44 => { vm.decode(0x74304675u32); let d = 3i16; let s = 3i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c8f48u64) }
        0x102c8f48 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c8f4cu64) }
        0x102c8f4c => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c8f50u64) }
        0x102c8f50 => { vm.decode(0x8b21a4f5u32); let d = 2i16; let s = 21i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102c8f54u64) }
        0x102c8f54 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c8f58u64) }
        0x102c8f58 => { vm.decode(0x8900036u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c8f5cu64) }
        0x102c8f5c => { vm.decode(0x408e43efu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c8f60u64) }
        0x102c8f60 => { vm.decode(0x4000446u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102c8f64u64) }
        0x102c8f64 => { vm.decode(0x4040227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 40i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102c8f68u64).wrapping_add((imm * 4) as u64) } else { 0x102c8f68u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c8f68u64) }
        0x102c8f68 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102c8f6cu64) } }
        0x102c8f6c => { vm.decode(0x2084c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c8f70u64) }
        0x102c8f70 => { vm.decode(0xa14004f5u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102c8f74u64) }
        0x102c8f74 => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c8f78u64) }
        0x102c8f78 => { vm.decode(0x82c202fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c8f7cu64) }
        0x102c8f7c => { vm.decode(0x8b191530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c8f80u64) }
        0x102c8f80 => { vm.decode(0xb091530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c8f84u64) }
        0x102c8f84 => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c8f88u64) }
        0x102c8f88 => { vm.decode(0x40020bb5u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c8f8cu64) }
        0x102c8f8c => { vm.decode(0xc040ad75u32); let d = 21i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102c8f90u64) }
        0x102c8f90 => { vm.decode(0x836124f5u32); let d = 1i16; let s = 21i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102c8f94u64) }
        0x102c8f94 => { vm.decode(0x54000111u32); let d = 22i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c8f98u64) }
        0x102c8f98 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c8f9cu64) }
        0x102c8f9c => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c8fa0u64) }
        0x102c8fa0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102c8fa4u64 + 4);
        vm.wr(37, 0x102c8fa4u64 + 4);
        Ok(0x102c8fa4u64) }
        0x102c8fa4 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c8fa8u64) }
        0x102c8fa8 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c8facu64) }
        0x102c8fac => { vm.decode(0x28c40b35u32); let d = 4i16; let s = 3i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c8fb0u64) }
        0x102c8fb0 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c8fb4u64) }
        0x102c8fb4 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102c8fb8u64) }
        0x102c8fb8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102c8fbcu64).wrapping_add((imm * 4) as u64) } else { 0x102c8fbcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c8fbcu64) }
        0x102c8fbc => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c8fc0u64) }
        0x102c8fc0 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c8fc4u64) }
        0x102c8fc4 => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c8fc8u64) }
        0x102c8fc8 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102c8fccu64) }
        0x102c8fcc => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102c8fd0u64) }
        0x102c8fd0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102c8fd4u64).wrapping_add((imm * 4) as u64) } else { 0x102c8fd4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c8fd4u64) }
        0x102c8fd4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102c8fd8u64) } }
        0x102c8fd8 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c8fdcu64) }
        0x102c8fdc => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102c8fe0u64) }
        0x102c8fe0 => { vm.decode(0x739cb4a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -43i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102c8fe4u64).wrapping_add((imm * 4) as u64) } else { 0x102c8fe4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c8fe4u64) }
        0x102c8fe4 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c8fe8u64) }
        0x102c8fe8 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c8fecu64) }
        0x102c8fec => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102c8ff0u64) }
        0x102c8ff0 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102c8ff4u64) }
        0x102c8ff4 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102c8ff8u64) }
        0x102c8ff8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102c8ffcu64).wrapping_add((imm * 4) as u64) } else { 0x102c8ffcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c8ffcu64) }
        0x102c8ffc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102c9000u64) } }
        0x102c9000 => { vm.decode(0x8980559du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 273336i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102c9004u64) }
        0x102c9004 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102c9008u64) } }
        0x102c9008 => { vm.decode(0xd10e0d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1292i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c900cu64) }
        0x102c900c => { vm.decode(0x45106a91u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 788i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c9010u64) }
        0x102c9010 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c9014u64) }
        0x102c9014 => { vm.decode(0x4510e051u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c9018u64) }
        0x102c9018 => { vm.decode(0x4d106a11u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c901cu64) }
        0x102c901c => { vm.decode(0x204c0835u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c9020u64) }
        0x102c9020 => { vm.decode(0x4d106851u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c9024u64) }
        0x102c9024 => { vm.decode(0x510eb11u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c9028u64) }
        0x102c9028 => { vm.decode(0x20d7062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c902cu64) }
        0x102c902c => { vm.decode(0x30d3462u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 788i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c9030u64) }
        0x102c9030 => { vm.decode(0x40052835u32); let d = 1i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c9034u64) }
        0x102c9034 => { vm.decode(0x31d3062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c9038u64) }
        0x102c9038 => { vm.decode(0xd1068d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1300i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c903cu64) }
        0x102c903c => { vm.decode(0x4d10ea91u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 796i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c9040u64) }
        0x102c9040 => { vm.decode(0x60402835u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102c9044u64) }
        0x102c9044 => { vm.decode(0x31d3462u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 796i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102c9048u64) }
        0x102c9048 => { vm.decode(0x2888636fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c904cu64) }
        0x102c904c => { vm.decode(0x20200833u32); let d = 2i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c9050u64) }
        0x102c9050 => { vm.decode(0xd106a11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c9054u64) }
        0x102c9054 => { vm.decode(0x40900d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102c9058u64) }
        0x102c9058 => { vm.decode(0x80220833u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c905cu64) }
        0x102c905c => { vm.decode(0x1019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c9060u64) }
        0x102c9060 => { vm.decode(0xc2a08d23u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 786i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 2)?;
        let v = u16::from_le_bytes(b.try_into().unwrap()) as u64;
        vm.st(d, v);
        Ok(0x102c9064u64) }
        0x102c9064 => { vm.decode(0x2208b3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c9068u64) }
        0x102c9068 => { vm.decode(0x714132beu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 787i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102c906cu64) }
        0x102c906c => { vm.decode(0x802208b3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c9070u64) }
        0x102c9070 => { vm.decode(0xd106a91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 788i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c9074u64) }
        0x102c9074 => { vm.decode(0x220c33u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 4i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c9078u64) }
        0x102c9078 => { vm.decode(0xd106a91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 788i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c907cu64) }
        0x102c907c => { vm.decode(0x40900d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102c9080u64) }
        0x102c9080 => { vm.decode(0x80220c33u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 5i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c9084u64) }
        0x102c9084 => { vm.decode(0xc2a08da3u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 790i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 2)?;
        let v = u16::from_le_bytes(b.try_into().unwrap()) as u64;
        vm.st(d, v);
        Ok(0x102c9088u64) }
        0x102c9088 => { vm.decode(0x220cb3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 6i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c908cu64) }
        0x102c908c => { vm.decode(0x714132feu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 791i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102c9090u64) }
        0x102c9090 => { vm.decode(0x80220cb3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 7i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c9094u64) }
        0x102c9094 => { vm.decode(0xd10ea11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c9098u64) }
        0x102c9098 => { vm.decode(0x220933u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c909cu64) }
        0x102c909c => { vm.decode(0xd10ea11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c90a0u64) }
        0x102c90a0 => { vm.decode(0x40900d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102c90a4u64) }
        0x102c90a4 => { vm.decode(0x80220933u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 9i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c90a8u64) }
        0x102c90a8 => { vm.decode(0xc2a18d23u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 794i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 2)?;
        let v = u16::from_le_bytes(b.try_into().unwrap()) as u64;
        vm.st(d, v);
        Ok(0x102c90acu64) }
        0x102c90ac => { vm.decode(0x2209b3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 10i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c90b0u64) }
        0x102c90b0 => { vm.decode(0x71413abeu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 795i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102c90b4u64) }
        0x102c90b4 => { vm.decode(0x802209b3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 11i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c90b8u64) }
        0x102c90b8 => { vm.decode(0xd10ea91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 796i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c90bcu64) }
        0x102c90bc => { vm.decode(0x220d33u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c90c0u64) }
        0x102c90c0 => { vm.decode(0xd10ea91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 796i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102c90c4u64) }
        0x102c90c4 => { vm.decode(0x40900d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102c90c8u64) }
        0x102c90c8 => { vm.decode(0x80220d33u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 13i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c90ccu64) }
        0x102c90cc => { vm.decode(0xc2a18da3u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 798i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 2)?;
        let v = u16::from_le_bytes(b.try_into().unwrap()) as u64;
        vm.st(d, v);
        Ok(0x102c90d0u64) }
        0x102c90d0 => { vm.decode(0x220db3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 14i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c90d4u64) }
        0x102c90d4 => { vm.decode(0x71413afeu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 799i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102c90d8u64) }
        0x102c90d8 => { vm.decode(0x80220db3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 15i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c90dcu64) }
        0x102c90dc => { vm.decode(0x109119u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 96i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c90e0u64) }
        0x102c90e0 => { vm.decode(0x408c436fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c90e4u64) }
        0x102c90e4 => { vm.decode(0x886436fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c90e8u64) }
        0x102c90e8 => { vm.decode(0x3eb8c2a7u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = 3837i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102c90ecu64).wrapping_add((imm * 4) as u64) } else { 0x102c90ecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c90ecu64) }
        0x102c90ec => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102c90f0u64) } }
        0x102c90f0 => { vm.decode(0x404024f5u32); let d = 1i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102c90f4u64) }
        0x102c90f4 => { vm.decode(0x801099u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c90f8u64) }
        0x102c90f8 => { vm.decode(0xadfb8df1u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = -3i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102c90fcu64).wrapping_add((imm * 4) as u64) } else { 0x102c90fcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c90fcu64) }
        0x102c90fc => { vm.decode(0x833u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102c9100u64) }
        0x102c9100 => { vm.decode(0x120d19du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 289120i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102c9104u64) }
        0x102c9104 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102c9108u64) } }
        0x102c93e0 => { vm.decode(0x641d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c93e4u64) }
        0x102c93e4 => { vm.decode(0x193894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c93e8u64) }
        0x102c93e8 => { vm.decode(0x640d8519u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c93ecu64) }
        0x102c93ec => { vm.decode(0x193094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c93f0u64) }
        0x102c93f0 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102c93f4u64) }
        0x102c93f4 => { vm.decode(0xcc407059u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = -23786i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c93f8u64) }
        0x102c93f8 => { vm.decode(0x12109019u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c93fcu64) }
        0x102c93fc => { vm.decode(0x640d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102c9400u64) }
        0x102c9400 => { vm.decode(0x1b2014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c9404u64) }
        0x102c9404 => { vm.decode(0x4100236u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102c9408u64) }
        0x102c9408 => { vm.decode(0xa2028000u32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102c940cu64) }
        0x102c940c => { vm.decode(0x40c02a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 184i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102c9410u64).wrapping_add((imm * 4) as u64) } else { 0x102c9410u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102c9410u64) }
        0x102c9410 => { vm.decode(0x92895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102c9414u64) }
        0x102c9414 => { vm.decode(0x29a0809du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 279148i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102c9418u64) }
        0x102c9418 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102c941cu64) } }
        0x102c941c => { vm.decode(0x100039b5u32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102c9420u64) }
        0x102ca5ec => { vm.decode(0x6a8a636fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ca5f0u64) }
        0x102ca5f0 => { vm.decode(0x80b0027u32); let d = 0i16; let s = 22i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102ca5f4u64).wrapping_add((imm * 4) as u64) } else { 0x102ca5f4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ca5f4u64) }
        0x102ca5f4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102ca5f8u64) } }
        0x102ca5f8 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ca5fcu64) }
        0x102ca5fc => { vm.decode(0x2000016fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ca600u64) }
        0x102ca600 => { vm.decode(0x89215db0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ca604u64) }
        0x102ca604 => { vm.decode(0xa88e36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ca608u64) }
        0x102ca608 => { vm.decode(0x9391d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ca60cu64) }
        0x102ca60c => { vm.decode(0x19291c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ca610u64) }
        0x102ca610 => { vm.decode(0xd9c4011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca614u64) }
        0x102ca614 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102ca618u64) }
        0x102ca618 => { vm.decode(0xa24124f5u32); let d = 1i16; let s = 1i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102ca61cu64) }
        0x102ca61c => { vm.decode(0x40448019u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ca620u64) }
        0x102ca620 => { vm.decode(0x19311c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ca624u64) }
        0x102ca624 => { vm.decode(0x886632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ca628u64) }
        0x102ca628 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ca62cu64) }
        0x102ca62c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102ca630u64 + 4);
        vm.wr(37, 0x102ca630u64 + 4);
        Ok(0x102ca630u64) }
        0x102ca630 => { vm.decode(0x620d8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ca634u64) }
        0x102ca634 => { vm.decode(0xd10e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca638u64) }
        0x102ca638 => { vm.decode(0x9236u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102ca63cu64) }
        0x102ca63c => { vm.decode(0x21d5062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca640u64) }
        0x102ca640 => { vm.decode(0x20082e35u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) < vm.ld(s)) as u64;
        vm.st(d, v);
        Ok(0x102ca644u64) }
        0x102ca644 => { vm.decode(0x8800027u32); let d = 0i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102ca648u64).wrapping_add((imm * 4) as u64) } else { 0x102ca648u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ca648u64) }
        0x102ca648 => { vm.decode(0x50da812du32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 63i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102ca64cu64) }
        0x102ca64c => { vm.decode(0xd10e291u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 780i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca650u64) }
        0x102ca658 => { vm.decode(0x12002019u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ca65cu64) }
        0x102ca65c => { vm.decode(0x642d8159u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1864i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ca660u64) }
        0x102ca660 => { vm.decode(0x80c80a7u32); let d = 0i16; let s = 3i16; let s2 = -1i16; let imm = 145i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102ca664u64).wrapping_add((imm * 4) as u64) } else { 0x102ca664u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ca664u64) }
        0x102ca664 => { vm.decode(0x19209c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ca668u64) }
        0x102ca668 => { vm.decode(0x2076u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102ca66cu64) }
        0x102ca66c => { vm.decode(0xa0808b35u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_sub(vm.ld(s2) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102ca670u64) }
        0x102ca670 => { vm.decode(0x8a21c909u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102ca674u64) }
        0x102ca674 => { vm.decode(0xa0002040u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102ca678u64) }
        0x102ca678 => { vm.decode(0x8482a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 141i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102ca67cu64).wrapping_add((imm * 4) as u64) } else { 0x102ca67cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ca67cu64) }
        0x102ca67c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102ca680u64) } }
        0x102ca680 => { vm.decode(0x4a8863afu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ca684u64) }
        0x102ca684 => { vm.decode(0x2020406fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ca688u64) }
        0x102ca688 => { vm.decode(0x662d0159u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1864i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ca68cu64) }
        0x102ca68c => { vm.decode(0x9309d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ca690u64) }
        0x102ca690 => { vm.decode(0x9211db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ca694u64) }
        0x102ca694 => { vm.decode(0x928dc30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ca698u64) }
        0x102ca698 => { vm.decode(0x9395c30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 2392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ca69cu64) }
        0x102ca69c => { vm.decode(0x19291c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ca6a0u64) }
        0x102ca6a0 => { vm.decode(0x9823c909u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102ca6a4u64) }
        0x102ca6a4 => { vm.decode(0x60c51019u32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ca6a8u64) }
        0x102ca6a8 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102ca6acu64) }
        0x102ca6ac => { vm.decode(0x19311c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ca6b0u64) }
        0x102ca6b0 => { vm.decode(0x886632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ca6b4u64) }
        0x102ca6b4 => { vm.decode(0x688c43afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ca6b8u64) }
        0x102ca6b8 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ca6bcu64) }
        0x102ca6bc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102ca6c0u64 + 4);
        vm.wr(37, 0x102ca6c0u64 + 4);
        Ok(0x102ca6c0u64) }
        0x102ca6c0 => { vm.decode(0x620d8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ca6c4u64) }
        0x102ca6c4 => { vm.decode(0xd10ea91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 796i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca6c8u64) }
        0x102ca6c8 => { vm.decode(0x3353062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca6ccu64) }
        0x102ca6cc => { vm.decode(0xd10ea11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca6d0u64) }
        0x102ca6d0 => { vm.decode(0x43251062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca6d4u64) }
        0x102ca6d4 => { vm.decode(0x15106b91u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 788i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca6d8u64) }
        0x102ca6d8 => { vm.decode(0xd106a11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca6dcu64) }
        0x102ca6dc => { vm.decode(0x3351062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca6e0u64) }
        0x102ca6e0 => { vm.decode(0xd90ea91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 860i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca6e4u64) }
        0x102ca6e4 => { vm.decode(0x431d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca6e8u64) }
        0x102ca6e8 => { vm.decode(0xd90ea11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 856i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca6ecu64) }
        0x102ca6ec => { vm.decode(0x231d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca6f0u64) }
        0x102ca6f0 => { vm.decode(0xd906a91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 852i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca6f4u64) }
        0x102ca6f4 => { vm.decode(0x63253062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca6f8u64) }
        0x102ca6f8 => { vm.decode(0xd906a11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 848i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca6fcu64) }
        0x102ca6fc => { vm.decode(0x431d3022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca700u64) }
        0x102ca700 => { vm.decode(0xd90e291u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 844i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca704u64) }
        0x102ca704 => { vm.decode(0x63353062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca708u64) }
        0x102ca708 => { vm.decode(0xd90e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 840i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca70cu64) }
        0x102ca70c => { vm.decode(0x43353062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca710u64) }
        0x102ca710 => { vm.decode(0xd906291u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 836i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca714u64) }
        0x102ca714 => { vm.decode(0x231d3022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca718u64) }
        0x102ca718 => { vm.decode(0xd906211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 832i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca71cu64) }
        0x102ca71c => { vm.decode(0x230d3022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca720u64) }
        0x102ca720 => { vm.decode(0xd14ea91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 828i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca724u64) }
        0x102ca724 => { vm.decode(0x30d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca728u64) }
        0x102ca728 => { vm.decode(0xd14ea11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 824i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca72cu64) }
        0x102ca72c => { vm.decode(0x630d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca730u64) }
        0x102ca730 => { vm.decode(0xd146a91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 820i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca734u64) }
        0x102ca734 => { vm.decode(0x30d3022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca738u64) }
        0x102ca738 => { vm.decode(0xd146a11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 816i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca73cu64) }
        0x102ca73c => { vm.decode(0x230d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca740u64) }
        0x102ca740 => { vm.decode(0xd14e291u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 812i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca744u64) }
        0x102ca744 => { vm.decode(0x63251062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca748u64) }
        0x102ca748 => { vm.decode(0xd14e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 808i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca74cu64) }
        0x102ca74c => { vm.decode(0x31d3022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca750u64) }
        0x102ca750 => { vm.decode(0xd146291u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 804i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca754u64) }
        0x102ca754 => { vm.decode(0x430d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca758u64) }
        0x102ca758 => { vm.decode(0xd146211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 800i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca75cu64) }
        0x102ca75c => { vm.decode(0x63351062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca760u64) }
        0x102ca760 => { vm.decode(0x10008119u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ca764u64) }
        0x102ca764 => { vm.decode(0x2824402fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ca768u64) }
        0x102ca768 => { vm.decode(0x9b211430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ca76cu64) }
        0x102ca76c => { vm.decode(0x88cc32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ca770u64) }
        0x102ca770 => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ca774u64) }
        0x102ca774 => { vm.decode(0x9b1154b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ca778u64) }
        0x102ca778 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ca77cu64) }
        0x102ca77c => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ca780u64) }
        0x102ca780 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ca784u64) }
        0x102ca784 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102ca788u64 + 4);
        vm.wr(37, 0x102ca788u64 + 4);
        Ok(0x102ca788u64) }
        0x102ca788 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ca78cu64) }
        0x102ca78c => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ca790u64) }
        0x102ca790 => { vm.decode(0x2220406fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ca794u64) }
        0x102ca794 => { vm.decode(0x9b2114b0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ca798u64) }
        0x102ca798 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102ca79cu64) }
        0x102ca79c => { vm.decode(0x8b315570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ca7a0u64) }
        0x102ca7a0 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ca7a4u64) }
        0x102ca7a4 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ca7a8u64) }
        0x102ca7a8 => { vm.decode(0xe109875u32); let d = 25i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ca7acu64) }
        0x102ca7ac => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102ca7b0u64 + 4);
        vm.wr(37, 0x102ca7b0u64 + 4);
        Ok(0x102ca7b0u64) }
        0x102ca7b0 => { vm.decode(0x100036u32); let d = 16i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102ca7b4u64) }
        0x102ca7b4 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ca7b8u64) }
        0x102ca7b8 => { vm.decode(0xdc40e059u32); let d = 17i16; let s = 1i16; let s2 = -1i16; let imm = -23786i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ca7bcu64) }
        0x102ca7bc => { vm.decode(0x10109019u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ca7c0u64) }
        0x102ca7c0 => { vm.decode(0x4002406u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102ca7c4u64) }
        0x102ca7c4 => { vm.decode(0x2200031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102ca7c8u64).wrapping_add((imm * 4) as u64) } else { 0x102ca7c8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ca7c8u64) }
        0x102ca7c8 => { vm.decode(0x6102036u32); let d = 21i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102ca7ccu64) }
        0x102ca7cc => { vm.decode(0xd00949du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 279864i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102ca7d0u64) }
        0x102ca7d4 => { vm.decode(0x48624735u32); let d = 1i16; let s = 4i16; let s2 = 30i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102ca7d8u64) }
        0x102ca7d8 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102ca7dcu64) }
        0x102ca7dc => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ca7e0u64) }
        0x102ca7e0 => { vm.decode(0x7430c675u32); let d = 3i16; let s = 3i16; let s2 = 30i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102ca7e4u64) }
        0x102ca7e4 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ca7e8u64) }
        0x102ca7e8 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ca7ecu64) }
        0x102ca7ec => { vm.decode(0x8c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ca7f0u64) }
        0x102ca7f0 => { vm.decode(0x4b2004f5u32); let d = 2i16; let s = 22i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102ca7f4u64) }
        0x102ca7f4 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca7f8u64) }
        0x102ca7f8 => { vm.decode(0x8900036u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102ca7fcu64) }
        0x102ca7fc => { vm.decode(0x4002406u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102ca800u64) }
        0x102ca800 => { vm.decode(0x4848227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 45i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102ca804u64).wrapping_add((imm * 4) as u64) } else { 0x102ca804u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ca804u64) }
        0x102ca804 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102ca808u64) } }
        0x102ca808 => { vm.decode(0x2084c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ca80cu64) }
        0x102ca80c => { vm.decode(0xa14004f5u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102ca810u64) }
        0x102ca810 => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ca814u64) }
        0x102ca814 => { vm.decode(0x82c202fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ca818u64) }
        0x102ca818 => { vm.decode(0x9b191530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ca81cu64) }
        0x102ca81c => { vm.decode(0xb091530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ca820u64) }
        0x102ca820 => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ca824u64) }
        0x102ca824 => { vm.decode(0x40020bb5u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102ca828u64) }
        0x102ca828 => { vm.decode(0xc000bd75u32); let d = 22i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102ca82cu64) }
        0x102ca82c => { vm.decode(0x228043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ca830u64) }
        0x102ca830 => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102ca834u64) }
        0x102ca834 => { vm.decode(0x54001111u32); let d = 30i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca838u64) }
        0x102ca838 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ca83cu64) }
        0x102ca83c => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ca840u64) }
        0x102ca840 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102ca844u64 + 4);
        vm.wr(37, 0x102ca844u64 + 4);
        Ok(0x102ca844u64) }
        0x102ca844 => { vm.decode(0x660d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ca848u64) }
        0x102ca848 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca84cu64) }
        0x102ca84c => { vm.decode(0x28c48b35u32); let d = 4i16; let s = 3i16; let s2 = 21i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102ca850u64) }
        0x102ca850 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102ca854u64) }
        0x102ca854 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102ca858u64) }
        0x102ca858 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102ca85cu64).wrapping_add((imm * 4) as u64) } else { 0x102ca85cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ca85cu64) }
        0x102ca85c => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102ca860u64) }
        0x102ca860 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102ca864u64) }
        0x102ca864 => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102ca868u64) }
        0x102ca868 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102ca86cu64) }
        0x102ca86c => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102ca870u64) }
        0x102ca870 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102ca874u64).wrapping_add((imm * 4) as u64) } else { 0x102ca874u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ca874u64) }
        0x102ca874 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102ca878u64) } }
        0x102ca878 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102ca87cu64) }
        0x102ca87c => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102ca880u64) }
        0x102ca880 => { vm.decode(0x739c34a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -44i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102ca884u64).wrapping_add((imm * 4) as u64) } else { 0x102ca884u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ca884u64) }
        0x102ca884 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102ca888u64) }
        0x102ca888 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102ca88cu64) }
        0x102ca88c => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102ca890u64) }
        0x102ca890 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102ca894u64) }
        0x102ca894 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102ca898u64) }
        0x102ca898 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102ca89cu64).wrapping_add((imm * 4) as u64) } else { 0x102ca89cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ca89cu64) }
        0x102ca89c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102ca8a0u64) } }
        0x102ca8a0 => { vm.decode(0x2420849du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 279636i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102ca8a4u64) }
        0x102ca8a4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102ca8a8u64) } }
        0x102ca8a8 => { vm.decode(0x8424809du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 283840i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102ca8acu64) }
        0x102ca8b8 => { vm.decode(0xd9060d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1348i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca8bcu64) }
        0x102ca8bc => { vm.decode(0x3253062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca8c0u64) }
        0x102ca8c0 => { vm.decode(0xd906051u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca8c4u64) }
        0x102ca8c4 => { vm.decode(0x3251062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca8c8u64) }
        0x102ca8c8 => { vm.decode(0xd14e8d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1340i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca8ccu64) }
        0x102ca8cc => { vm.decode(0x63153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca8d0u64) }
        0x102ca8d0 => { vm.decode(0xd14e851u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca8d4u64) }
        0x102ca8d4 => { vm.decode(0x63053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca8d8u64) }
        0x102ca8d8 => { vm.decode(0xd1468d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1332i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca8dcu64) }
        0x102ca8dc => { vm.decode(0x63151062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca8e0u64) }
        0x102ca8e0 => { vm.decode(0xd146851u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca8e4u64) }
        0x102ca8e4 => { vm.decode(0x63051062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca8e8u64) }
        0x102ca8e8 => { vm.decode(0xd14e0d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1324i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca8ecu64) }
        0x102ca8ec => { vm.decode(0x43153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca8f0u64) }
        0x102ca8f0 => { vm.decode(0xd14e051u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca8f4u64) }
        0x102ca8f4 => { vm.decode(0x43053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca8f8u64) }
        0x102ca8f8 => { vm.decode(0xd1460d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1316i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca8fcu64) }
        0x102ca8fc => { vm.decode(0x43151062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca900u64) }
        0x102ca900 => { vm.decode(0xd146051u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca904u64) }
        0x102ca904 => { vm.decode(0x43051062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca908u64) }
        0x102ca908 => { vm.decode(0xd10e8d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1308i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca90cu64) }
        0x102ca90c => { vm.decode(0x23153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca910u64) }
        0x102ca910 => { vm.decode(0xd10e851u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca914u64) }
        0x102ca914 => { vm.decode(0x23053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca918u64) }
        0x102ca918 => { vm.decode(0xd1068d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1300i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca91cu64) }
        0x102ca91c => { vm.decode(0x23151062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca920u64) }
        0x102ca920 => { vm.decode(0xd106851u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca924u64) }
        0x102ca924 => { vm.decode(0x3153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ca928u64) }
        0x102ca928 => { vm.decode(0x5d10e1d1u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 1292i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca92cu64) }
        0x102ca92c => { vm.decode(0x5510f151u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 1288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca930u64) }
        0x102ca930 => { vm.decode(0x4a8a63afu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ca934u64) }
        0x102ca934 => { vm.decode(0x28a4402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ca938u64) }
        0x102ca938 => { vm.decode(0x12008119u32); let d = 21i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ca93cu64) }
        0x102ca93c => { vm.decode(0x9b215430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ca940u64) }
        0x102ca940 => { vm.decode(0x88ce32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ca944u64) }
        0x102ca944 => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ca948u64) }
        0x102ca948 => { vm.decode(0x704b0419u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2192i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ca94cu64) }
        0x102ca94c => { vm.decode(0x8b1114b0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ca950u64) }
        0x102ca950 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ca954u64) }
        0x102ca954 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ca958u64) }
        0x102ca958 => { vm.decode(0x488c43afu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ca95cu64) }
        0x102ca95c => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ca960u64) }
        0x102ca960 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102ca964u64 + 4);
        vm.wr(37, 0x102ca964u64 + 4);
        Ok(0x102ca964u64) }
        0x102ca964 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ca968u64) }
        0x102ca968 => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ca96cu64) }
        0x102ca96c => { vm.decode(0x70498519u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2128i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ca970u64) }
        0x102ca970 => { vm.decode(0x22a0406fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ca974u64) }
        0x102ca974 => { vm.decode(0x9b2154b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ca978u64) }
        0x102ca978 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102ca97cu64) }
        0x102ca97c => { vm.decode(0x9b311430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ca980u64) }
        0x102ca980 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ca984u64) }
        0x102ca984 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ca988u64) }
        0x102ca988 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ca98cu64) }
        0x102ca98c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102ca990u64 + 4);
        vm.wr(37, 0x102ca990u64 + 4);
        Ok(0x102ca990u64) }
        0x102ca990 => { vm.decode(0x102236u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102ca994u64) }
        0x102ca994 => { vm.decode(0xd986111u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca998u64) }
        0x102ca998 => { vm.decode(0x518e911u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca99cu64) }
        0x102ca99c => { vm.decode(0x2001a835u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ca9a0u64) }
        0x102ca9a0 => { vm.decode(0x130207f5u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ca9a4u64) }
        0x102ca9a4 => { vm.decode(0x40012835u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ca9a8u64) }
        0x102ca9a8 => { vm.decode(0x4598e811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca9acu64) }
        0x102ca9ac => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102ca9b0u64) }
        0x102ca9b0 => { vm.decode(0xd4802fb5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102ca9b4u64) }
        0x102ca9b4 => { vm.decode(0x4518e011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca9b8u64) }
        0x102ca9b8 => { vm.decode(0x40210fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102ca9bcu64) }
        0x102ca9bc => { vm.decode(0x1a01cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 25i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102ca9c0u64) }
        0x102ca9c0 => { vm.decode(0xc4800fb5u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102ca9c4u64) }
        0x102ca9c4 => { vm.decode(0x40198835u32); let d = 2i16; let s = 5i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ca9c8u64) }
        0x102ca9c8 => { vm.decode(0x440017f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ca9ccu64) }
        0x102ca9cc => { vm.decode(0x200d0835u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ca9d0u64) }
        0x102ca9d0 => { vm.decode(0x4d144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca9d4u64) }
        0x102ca9d4 => { vm.decode(0x20212fb5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102ca9d8u64) }
        0x102ca9d8 => { vm.decode(0xa0a12fb5u32); let d = 2i16; let s = 2i16; let s2 = 23i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102ca9dcu64) }
        0x102ca9dc => { vm.decode(0x20102fb5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102ca9e0u64) }
        0x102ca9e0 => { vm.decode(0x28910d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 20i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102ca9e4u64) }
        0x102ca9e4 => { vm.decode(0x20210fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102ca9e8u64) }
        0x102ca9e8 => { vm.decode(0x2008b835u32); let d = 3i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ca9ecu64) }
        0x102ca9ec => { vm.decode(0x461207f5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ca9f0u64) }
        0x102ca9f0 => { vm.decode(0x401d2835u32); let d = 3i16; let s = 3i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ca9f4u64) }
        0x102ca9f4 => { vm.decode(0x5104b11u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ca9f8u64) }
        0x102ca9f8 => { vm.decode(0x64a00fb5u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102ca9fcu64) }
        0x102ca9fc => { vm.decode(0x510e911u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102caa00u64) }
        0x102caa00 => { vm.decode(0x64a00fb5u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caa04u64) }
        0x102caa04 => { vm.decode(0x64112fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caa08u64) }
        0x102caa08 => { vm.decode(0x3e90cd35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 15i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caa0cu64) }
        0x102caa0c => { vm.decode(0x64202fb5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caa10u64) }
        0x102caa10 => { vm.decode(0x41400835u32); let d = 4i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caa14u64) }
        0x102caa14 => { vm.decode(0x1157f5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caa18u64) }
        0x102caa18 => { vm.decode(0x21008835u32); let d = 4i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caa1cu64) }
        0x102caa1c => { vm.decode(0xd14cb11u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102caa20u64) }
        0x102caa20 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caa24u64) }
        0x102caa24 => { vm.decode(0xd14e111u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102caa28u64) }
        0x102caa28 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caa2cu64) }
        0x102caa2c => { vm.decode(0x9a0fb5u32); let d = 4i16; let s = 4i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caa30u64) }
        0x102caa30 => { vm.decode(0xe240d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 10i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caa34u64) }
        0x102caa34 => { vm.decode(0x132fb5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caa38u64) }
        0x102caa38 => { vm.decode(0x21442835u32); let d = 5i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caa3cu64) }
        0x102caa3c => { vm.decode(0x120347f5u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caa40u64) }
        0x102caa40 => { vm.decode(0x4140a835u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caa44u64) }
        0x102caa44 => { vm.decode(0x4590cb11u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102caa48u64) }
        0x102caa48 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caa4cu64) }
        0x102caa4c => { vm.decode(0x45146911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102caa50u64) }
        0x102caa50 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caa54u64) }
        0x102caa54 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caa58u64) }
        0x102caa58 => { vm.decode(0x1a01cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 25i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caa5cu64) }
        0x102caa5c => { vm.decode(0x44800fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caa60u64) }
        0x102caa60 => { vm.decode(0x41052835u32); let d = 5i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caa64u64) }
        0x102caa64 => { vm.decode(0x20357f5u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caa68u64) }
        0x102caa68 => { vm.decode(0x6140a835u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caa6cu64) }
        0x102caa6c => { vm.decode(0x4510cb11u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102caa70u64) }
        0x102caa70 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caa74u64) }
        0x102caa74 => { vm.decode(0x4514e911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102caa78u64) }
        0x102caa78 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caa7cu64) }
        0x102caa7c => { vm.decode(0x20a10fb5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caa80u64) }
        0x102caa80 => { vm.decode(0x28910d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 20i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caa84u64) }
        0x102caa84 => { vm.decode(0x20210fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caa88u64) }
        0x102caa88 => { vm.decode(0x2100a835u32); let d = 5i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caa8cu64) }
        0x102caa8c => { vm.decode(0x21347f5u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caa90u64) }
        0x102caa90 => { vm.decode(0x4101a835u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caa94u64) }
        0x102caa94 => { vm.decode(0x459c6111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102caa98u64) }
        0x102caa98 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caa9cu64) }
        0x102caa9c => { vm.decode(0x45906111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102caaa0u64) }
        0x102caaa0 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caaa4u64) }
        0x102caaa4 => { vm.decode(0x64a10fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caaa8u64) }
        0x102caaa8 => { vm.decode(0x3e90cd35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 15i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caaacu64) }
        0x102caaac => { vm.decode(0x64202fb5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caab0u64) }
        0x102caab0 => { vm.decode(0x41402835u32); let d = 5i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caab4u64) }
        0x102caab4 => { vm.decode(0x21357f5u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caab8u64) }
        0x102caab8 => { vm.decode(0x6100a835u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caabcu64) }
        0x102caabc => { vm.decode(0x45144b11u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102caac0u64) }
        0x102caac0 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caac4u64) }
        0x102caac4 => { vm.decode(0x4590e111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102caac8u64) }
        0x102caac8 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caaccu64) }
        0x102caacc => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caad0u64) }
        0x102caad0 => { vm.decode(0xe240d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 10i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caad4u64) }
        0x102caad4 => { vm.decode(0x132fb5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caad8u64) }
        0x102caad8 => { vm.decode(0x21442835u32); let d = 5i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caadcu64) }
        0x102caadc => { vm.decode(0x120347f5u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caae0u64) }
        0x102caae0 => { vm.decode(0x4140a835u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caae4u64) }
        0x102caae4 => { vm.decode(0x459ce111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102caae8u64) }
        0x102caae8 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caaecu64) }
        0x102caaec => { vm.decode(0x45906911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102caaf0u64) }
        0x102caaf0 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caaf4u64) }
        0x102caaf4 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caaf8u64) }
        0x102caaf8 => { vm.decode(0x1a01cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 25i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caafcu64) }
        0x102caafc => { vm.decode(0x44800fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cab00u64) }
        0x102cab00 => { vm.decode(0x41052835u32); let d = 5i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cab04u64) }
        0x102cab04 => { vm.decode(0x20357f5u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cab08u64) }
        0x102cab08 => { vm.decode(0x6140a835u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cab0cu64) }
        0x102cab0c => { vm.decode(0x459c6911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cab10u64) }
        0x102cab10 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cab14u64) }
        0x102cab14 => { vm.decode(0x4590e911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cab18u64) }
        0x102cab18 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cab1cu64) }
        0x102cab1c => { vm.decode(0x20a10fb5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cab20u64) }
        0x102cab20 => { vm.decode(0x28910d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 20i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cab24u64) }
        0x102cab24 => { vm.decode(0x20210fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cab28u64) }
        0x102cab28 => { vm.decode(0x2100a835u32); let d = 5i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cab2cu64) }
        0x102cab2c => { vm.decode(0x21347f5u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cab30u64) }
        0x102cab30 => { vm.decode(0x4101a835u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cab34u64) }
        0x102cab34 => { vm.decode(0x45104311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cab38u64) }
        0x102cab38 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cab3cu64) }
        0x102cab3c => { vm.decode(0x45946111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cab40u64) }
        0x102cab40 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cab44u64) }
        0x102cab44 => { vm.decode(0x64a10fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cab48u64) }
        0x102cab48 => { vm.decode(0x3e90cd35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 15i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cab4cu64) }
        0x102cab4c => { vm.decode(0x64202fb5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cab50u64) }
        0x102cab50 => { vm.decode(0x41402835u32); let d = 5i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cab54u64) }
        0x102cab54 => { vm.decode(0x21357f5u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cab58u64) }
        0x102cab58 => { vm.decode(0x6100a835u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cab5cu64) }
        0x102cab5c => { vm.decode(0x45904311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cab60u64) }
        0x102cab60 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cab64u64) }
        0x102cab64 => { vm.decode(0x4594e111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cab68u64) }
        0x102cab68 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cab6cu64) }
        0x102cab6c => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cab70u64) }
        0x102cab70 => { vm.decode(0xe240d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 10i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cab74u64) }
        0x102cab74 => { vm.decode(0x132fb5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cab78u64) }
        0x102cab78 => { vm.decode(0x21442835u32); let d = 5i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cab7cu64) }
        0x102cab7c => { vm.decode(0x120347f5u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cab80u64) }
        0x102cab80 => { vm.decode(0x4140a835u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cab84u64) }
        0x102cab84 => { vm.decode(0x45944311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cab88u64) }
        0x102cab88 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cab8cu64) }
        0x102cab8c => { vm.decode(0x45946911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cab90u64) }
        0x102cab90 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cab94u64) }
        0x102cab94 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cab98u64) }
        0x102cab98 => { vm.decode(0x1a01cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 25i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cab9cu64) }
        0x102cab9c => { vm.decode(0x24820fb5u32); let d = 6i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caba0u64) }
        0x102caba0 => { vm.decode(0x40052835u32); let d = 1i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caba4u64) }
        0x102caba4 => { vm.decode(0x121207f5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caba8u64) }
        0x102caba8 => { vm.decode(0x60402835u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cabacu64) }
        0x102cabac => { vm.decode(0xd14c311u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cabb0u64) }
        0x102cabb0 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cabb4u64) }
        0x102cabb4 => { vm.decode(0xd94e911u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cabb8u64) }
        0x102cabb8 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cabbcu64) }
        0x102cabbc => { vm.decode(0x40210fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cabc0u64) }
        0x102cabc0 => { vm.decode(0x18814d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 20i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cabc4u64) }
        0x102cabc4 => { vm.decode(0x44822fb5u32); let d = 5i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cabc8u64) }
        0x102cabc8 => { vm.decode(0x41a835u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cabccu64) }
        0x102cabcc => { vm.decode(0x120217f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cabd0u64) }
        0x102cabd0 => { vm.decode(0x40012835u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cabd4u64) }
        0x102cabd4 => { vm.decode(0x4590c211u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cabd8u64) }
        0x102cabd8 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cabdcu64) }
        0x102cabdc => { vm.decode(0x45186011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cabe0u64) }
        0x102cabe0 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cabe4u64) }
        0x102cabe4 => { vm.decode(0x44210fb5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cabe8u64) }
        0x102cabe8 => { vm.decode(0x1e80cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cabecu64) }
        0x102cabec => { vm.decode(0x25870fb5u32); let d = 30i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cabf0u64) }
        0x102cabf0 => { vm.decode(0x3651062u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cabf4u64) }
        0x102cabf4 => { vm.decode(0x2005a835u32); let d = 1i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cabf8u64) }
        0x102cabf8 => { vm.decode(0x131a07f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cabfcu64) }
        0x102cabfc => { vm.decode(0x2755062u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cac00u64) }
        0x102cac00 => { vm.decode(0x40412835u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cac04u64) }
        0x102cac04 => { vm.decode(0x459ce811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cac08u64) }
        0x102cac08 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cac0cu64) }
        0x102cac0c => { vm.decode(0x45186811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cac10u64) }
        0x102cac10 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cac14u64) }
        0x102cac14 => { vm.decode(0x40110fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cac18u64) }
        0x102cac18 => { vm.decode(0x1e004d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 10i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cac1cu64) }
        0x102cac1c => { vm.decode(0xd4802fb5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cac20u64) }
        0x102cac20 => { vm.decode(0x3253062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cac24u64) }
        0x102cac24 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cac28u64) }
        0x102cac28 => { vm.decode(0xde40e0d9u32); let d = 21i16; let s = 1i16; let s2 = -1i16; let imm = -23785i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cac2cu64) }
        0x102cac2c => { vm.decode(0x12101019u32); let d = 22i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cac30u64) }
        0x102cac30 => { vm.decode(0x26002406u32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cac34u64) }
        0x102cac34 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102cac38u64).wrapping_add((imm * 4) as u64) } else { 0x102cac38u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cac38u64) }
        0x102cac38 => { vm.decode(0x6102236u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cac3cu64) }
        0x102cac3c => { vm.decode(0x8000d49du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 280976i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102cac40u64) }
        0x102cac44 => { vm.decode(0x8224735u32); let d = 1i16; let s = 4i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cac48u64) }
        0x102cac48 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102cac4cu64) }
        0x102cac4c => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cac50u64) }
        0x102cac50 => { vm.decode(0x64304675u32); let d = 3i16; let s = 3i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cac54u64) }
        0x102cac54 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cac58u64) }
        0x102cac58 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cac5cu64) }
        0x102cac5c => { vm.decode(0x690104f5u32); let d = 2i16; let s = 17i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cac60u64) }
        0x102cac60 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cac64u64) }
        0x102cac64 => { vm.decode(0x8b03236u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cac68u64) }
        0x102cac68 => { vm.decode(0x26002406u32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cac6cu64) }
        0x102cac6c => { vm.decode(0x4040227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 40i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cac70u64).wrapping_add((imm * 4) as u64) } else { 0x102cac70u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cac70u64) }
        0x102cac70 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cac74u64) } }
        0x102cac74 => { vm.decode(0x2084c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cac78u64) }
        0x102cac78 => { vm.decode(0xa16004f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cac7cu64) }
        0x102cac7c => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cac80u64) }
        0x102cac80 => { vm.decode(0x82c202fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cac84u64) }
        0x102cac84 => { vm.decode(0x8b195530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cac88u64) }
        0x102cac88 => { vm.decode(0xb091530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cac8cu64) }
        0x102cac8c => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cac90u64) }
        0x102cac90 => { vm.decode(0x40830bb5u32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cac94u64) }
        0x102cac94 => { vm.decode(0xc0003d75u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102cac98u64) }
        0x102cac98 => { vm.decode(0x414104f5u32); let d = 1i16; let s = 16i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cac9cu64) }
        0x102cac9c => { vm.decode(0x14000111u32); let d = 20i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102caca0u64) }
        0x102caca0 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102caca4u64) }
        0x102caca4 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102caca8u64) }
        0x102caca8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cacacu64 + 4);
        vm.wr(37, 0x102cacacu64 + 4);
        Ok(0x102cacacu64) }
        0x102cacac => { vm.decode(0x660d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cacb0u64) }
        0x102cacb0 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cacb4u64) }
        0x102cacb4 => { vm.decode(0x28cc8b35u32); let d = 4i16; let s = 3i16; let s2 = 23i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cacb8u64) }
        0x102cacb8 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cacbcu64) }
        0x102cacbc => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102cacc0u64) }
        0x102cacc0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cacc4u64).wrapping_add((imm * 4) as u64) } else { 0x102cacc4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cacc4u64) }
        0x102cacc4 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cacc8u64) }
        0x102cacc8 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cacccu64) }
        0x102caccc => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cacd0u64) }
        0x102cacd0 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102cacd4u64) }
        0x102cacd4 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102cacd8u64) }
        0x102cacd8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102cacdcu64).wrapping_add((imm * 4) as u64) } else { 0x102cacdcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cacdcu64) }
        0x102cacdc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cace0u64) } }
        0x102cace0 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cace4u64) }
        0x102cace4 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102cace8u64) }
        0x102cace8 => { vm.decode(0x739c3ca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -42i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cacecu64).wrapping_add((imm * 4) as u64) } else { 0x102cacecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cacecu64) }
        0x102cacec => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cacf0u64) }
        0x102cacf0 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cacf4u64) }
        0x102cacf4 => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cacf8u64) }
        0x102cacf8 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102cacfcu64) }
        0x102cacfc => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102cad00u64) }
        0x102cad00 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102cad04u64).wrapping_add((imm * 4) as u64) } else { 0x102cad04u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cad04u64) }
        0x102cad04 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cad08u64) } }
        0x102cad08 => { vm.decode(0xa020c09du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 280772i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102cad0cu64) }
        0x102cad0c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cad10u64) } }
        0x102cad10 => { vm.decode(0xf18c091u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2188i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cad14u64) }
        0x102cad14 => { vm.decode(0x43251062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cad18u64) }
        0x102cad18 => { vm.decode(0xf18c011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cad1cu64) }
        0x102cad1c => { vm.decode(0x3353062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cad20u64) }
        0x102cad20 => { vm.decode(0xf184091u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2180i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cad24u64) }
        0x102cad24 => { vm.decode(0x63153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cad28u64) }
        0x102cad28 => { vm.decode(0xf184011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cad2cu64) }
        0x102cad2c => { vm.decode(0x63053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cad30u64) }
        0x102cad30 => { vm.decode(0xf94c891u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2172i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cad34u64) }
        0x102cad34 => { vm.decode(0x63151062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cad38u64) }
        0x102cad38 => { vm.decode(0xf94c811u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cad3cu64) }
        0x102cad3c => { vm.decode(0x63051062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cad40u64) }
        0x102cad40 => { vm.decode(0xf944891u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2164i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cad44u64) }
        0x102cad44 => { vm.decode(0x43153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cad48u64) }
        0x102cad48 => { vm.decode(0xf944811u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cad4cu64) }
        0x102cad4c => { vm.decode(0x43053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cad50u64) }
        0x102cad50 => { vm.decode(0xf94c091u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2156i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cad54u64) }
        0x102cad54 => { vm.decode(0x43151062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cad58u64) }
        0x102cad58 => { vm.decode(0xf94c011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cad5cu64) }
        0x102cad5c => { vm.decode(0x43051062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cad60u64) }
        0x102cad60 => { vm.decode(0xf944091u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2148i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cad64u64) }
        0x102cad64 => { vm.decode(0x23153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cad68u64) }
        0x102cad68 => { vm.decode(0xf944011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cad6cu64) }
        0x102cad6c => { vm.decode(0x23053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cad70u64) }
        0x102cad70 => { vm.decode(0xf90c891u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2140i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cad74u64) }
        0x102cad74 => { vm.decode(0x23151062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cad78u64) }
        0x102cad78 => { vm.decode(0x1790c911u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cad7cu64) }
        0x102cad7c => { vm.decode(0x5f904891u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2132i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cad80u64) }
        0x102cad80 => { vm.decode(0x1f904911u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cad84u64) }
        0x102cad84 => { vm.decode(0x4a8863afu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cad88u64) }
        0x102cad88 => { vm.decode(0x2824402fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cad8cu64) }
        0x102cad8c => { vm.decode(0x12001119u32); let d = 22i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cad90u64) }
        0x102cad90 => { vm.decode(0x8b215530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cad94u64) }
        0x102cad94 => { vm.decode(0xa8cc32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cad98u64) }
        0x102cad98 => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cad9cu64) }
        0x102cad9c => { vm.decode(0x88863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cada0u64) }
        0x102cada0 => { vm.decode(0xb1115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cada4u64) }
        0x102cada4 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cada8u64) }
        0x102cada8 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cadacu64) }
        0x102cadac => { vm.decode(0x688e43afu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cadb0u64) }
        0x102cadb0 => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cadb4u64) }
        0x102cadb4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cadb8u64 + 4);
        vm.wr(37, 0x102cadb8u64 + 4);
        Ok(0x102cadb8u64) }
        0x102cadb8 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cadbcu64) }
        0x102cadbc => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cadc0u64) }
        0x102cadc0 => { vm.decode(0x746b0159u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2000i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cadc4u64) }
        0x102cadc4 => { vm.decode(0x2220406fu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cadc8u64) }
        0x102cadc8 => { vm.decode(0x8b2155b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cadccu64) }
        0x102cadcc => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102cadd0u64) }
        0x102cadd0 => { vm.decode(0x8b311430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cadd4u64) }
        0x102cadd4 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cadd8u64) }
        0x102cadd8 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102caddcu64) }
        0x102caddc => { vm.decode(0xe10d875u32); let d = 25i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cade0u64) }
        0x102cade0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cade4u64 + 4);
        vm.wr(37, 0x102cade4u64 + 4);
        Ok(0x102cade4u64) }
        0x102cade4 => { vm.decode(0x102036u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cade8u64) }
        0x102cade8 => { vm.decode(0x45186911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cadecu64) }
        0x102cadec => { vm.decode(0x1045b835u32); let d = 1i16; let s = 6i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cadf0u64) }
        0x102cadf0 => { vm.decode(0x5186111u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cadf4u64) }
        0x102cadf4 => { vm.decode(0x120207f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cadf8u64) }
        0x102cadf8 => { vm.decode(0x40532835u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cadfcu64) }
        0x102cadfc => { vm.decode(0x4598e811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cae00u64) }
        0x102cae00 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cae04u64) }
        0x102cae04 => { vm.decode(0xc4810fb5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cae08u64) }
        0x102cae08 => { vm.decode(0x4518e011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cae0cu64) }
        0x102cae0c => { vm.decode(0x40210fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cae10u64) }
        0x102cae10 => { vm.decode(0x1e01cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cae14u64) }
        0x102cae14 => { vm.decode(0x44802fb5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cae18u64) }
        0x102cae18 => { vm.decode(0x200c8835u32); let d = 2i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cae1cu64) }
        0x102cae1c => { vm.decode(0x551807f5u32); let d = 2i16; let s = 2i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cae20u64) }
        0x102cae20 => { vm.decode(0x4d0835u32); let d = 2i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cae24u64) }
        0x102cae24 => { vm.decode(0x4d104211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cae28u64) }
        0x102cae28 => { vm.decode(0x20212fb5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cae2cu64) }
        0x102cae2c => { vm.decode(0xa0212fb5u32); let d = 2i16; let s = 2i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cae30u64) }
        0x102cae30 => { vm.decode(0x20102fb5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cae34u64) }
        0x102cae34 => { vm.decode(0x2c918d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 23i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cae38u64) }
        0x102cae38 => { vm.decode(0x20210fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cae3cu64) }
        0x102cae3c => { vm.decode(0x40482835u32); let d = 3i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cae40u64) }
        0x102cae40 => { vm.decode(0x561207f5u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cae44u64) }
        0x102cae44 => { vm.decode(0x600c2835u32); let d = 3i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cae48u64) }
        0x102cae48 => { vm.decode(0x510cb11u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cae4cu64) }
        0x102cae4c => { vm.decode(0x64a00fb5u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cae50u64) }
        0x102cae50 => { vm.decode(0xe4a00fb5u32); let d = 3i16; let s = 3i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cae54u64) }
        0x102cae54 => { vm.decode(0x60396fb5u32); let d = 3i16; let s = 3i16; let s2 = 30i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cae58u64) }
        0x102cae58 => { vm.decode(0x3c114d35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 18i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cae5cu64) }
        0x102cae5c => { vm.decode(0x64202fb5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cae60u64) }
        0x102cae60 => { vm.decode(0x21440835u32); let d = 4i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cae64u64) }
        0x102cae64 => { vm.decode(0x157f5u32); let d = 4i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cae68u64) }
        0x102cae68 => { vm.decode(0x1408835u32); let d = 4i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cae6cu64) }
        0x102cae6c => { vm.decode(0xd90c311u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cae70u64) }
        0x102cae70 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cae74u64) }
        0x102cae74 => { vm.decode(0xd14e111u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cae78u64) }
        0x102cae78 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cae7cu64) }
        0x102cae7c => { vm.decode(0xb20fb5u32); let d = 4i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cae80u64) }
        0x102cae80 => { vm.decode(0xaa40d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 12i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cae84u64) }
        0x102cae84 => { vm.decode(0x132fb5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cae88u64) }
        0x102cae88 => { vm.decode(0x41052835u32); let d = 5i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cae8cu64) }
        0x102cae8c => { vm.decode(0x21347f5u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cae90u64) }
        0x102cae90 => { vm.decode(0x6140a835u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cae94u64) }
        0x102cae94 => { vm.decode(0x4514cb11u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cae98u64) }
        0x102cae98 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cae9cu64) }
        0x102cae9c => { vm.decode(0x45146911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102caea0u64) }
        0x102caea0 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caea4u64) }
        0x102caea4 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caea8u64) }
        0x102caea8 => { vm.decode(0x1e01cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caeacu64) }
        0x102caeac => { vm.decode(0x44800fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caeb0u64) }
        0x102caeb0 => { vm.decode(0x2100a835u32); let d = 5i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caeb4u64) }
        0x102caeb4 => { vm.decode(0x21357f5u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caeb8u64) }
        0x102caeb8 => { vm.decode(0x4101a835u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caebcu64) }
        0x102caebc => { vm.decode(0x4590cb11u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102caec0u64) }
        0x102caec0 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caec4u64) }
        0x102caec4 => { vm.decode(0x4514e911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102caec8u64) }
        0x102caec8 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caeccu64) }
        0x102caecc => { vm.decode(0x20a10fb5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caed0u64) }
        0x102caed0 => { vm.decode(0x2c918d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 23i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caed4u64) }
        0x102caed4 => { vm.decode(0x20210fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caed8u64) }
        0x102caed8 => { vm.decode(0x41402835u32); let d = 5i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caedcu64) }
        0x102caedc => { vm.decode(0x120347f5u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caee0u64) }
        0x102caee0 => { vm.decode(0x6100a835u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caee4u64) }
        0x102caee4 => { vm.decode(0x459ce911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102caee8u64) }
        0x102caee8 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caeecu64) }
        0x102caeec => { vm.decode(0x45906111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102caef0u64) }
        0x102caef0 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caef4u64) }
        0x102caef4 => { vm.decode(0x64a10fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caef8u64) }
        0x102caef8 => { vm.decode(0x3c114d35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 18i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caefcu64) }
        0x102caefc => { vm.decode(0x64202fb5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caf00u64) }
        0x102caf00 => { vm.decode(0x21442835u32); let d = 5i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caf04u64) }
        0x102caf04 => { vm.decode(0x20357f5u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caf08u64) }
        0x102caf08 => { vm.decode(0x4140a835u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caf0cu64) }
        0x102caf0c => { vm.decode(0x4514c311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102caf10u64) }
        0x102caf10 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caf14u64) }
        0x102caf14 => { vm.decode(0x4590e111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102caf18u64) }
        0x102caf18 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caf1cu64) }
        0x102caf1c => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caf20u64) }
        0x102caf20 => { vm.decode(0xaa40d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 12i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caf24u64) }
        0x102caf24 => { vm.decode(0x132fb5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caf28u64) }
        0x102caf28 => { vm.decode(0x41052835u32); let d = 5i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caf2cu64) }
        0x102caf2c => { vm.decode(0x21347f5u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caf30u64) }
        0x102caf30 => { vm.decode(0x6140a835u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caf34u64) }
        0x102caf34 => { vm.decode(0x459c6111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102caf38u64) }
        0x102caf38 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caf3cu64) }
        0x102caf3c => { vm.decode(0x45906911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102caf40u64) }
        0x102caf40 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caf44u64) }
        0x102caf44 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caf48u64) }
        0x102caf48 => { vm.decode(0x1e01cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caf4cu64) }
        0x102caf4c => { vm.decode(0x44800fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caf50u64) }
        0x102caf50 => { vm.decode(0x2100a835u32); let d = 5i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caf54u64) }
        0x102caf54 => { vm.decode(0x21357f5u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caf58u64) }
        0x102caf58 => { vm.decode(0x4101a835u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caf5cu64) }
        0x102caf5c => { vm.decode(0x45144311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102caf60u64) }
        0x102caf60 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caf64u64) }
        0x102caf64 => { vm.decode(0x4590e911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102caf68u64) }
        0x102caf68 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caf6cu64) }
        0x102caf6c => { vm.decode(0x20a10fb5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caf70u64) }
        0x102caf70 => { vm.decode(0x2c918d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 23i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caf74u64) }
        0x102caf74 => { vm.decode(0x20210fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caf78u64) }
        0x102caf78 => { vm.decode(0x41402835u32); let d = 5i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caf7cu64) }
        0x102caf7c => { vm.decode(0x120347f5u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caf80u64) }
        0x102caf80 => { vm.decode(0x6100a835u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caf84u64) }
        0x102caf84 => { vm.decode(0x45944311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102caf88u64) }
        0x102caf88 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caf8cu64) }
        0x102caf8c => { vm.decode(0x45946111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102caf90u64) }
        0x102caf90 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caf94u64) }
        0x102caf94 => { vm.decode(0x64a10fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caf98u64) }
        0x102caf98 => { vm.decode(0x3c114d35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 18i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caf9cu64) }
        0x102caf9c => { vm.decode(0x64202fb5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cafa0u64) }
        0x102cafa0 => { vm.decode(0x21442835u32); let d = 5i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cafa4u64) }
        0x102cafa4 => { vm.decode(0x20357f5u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cafa8u64) }
        0x102cafa8 => { vm.decode(0x4140a835u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cafacu64) }
        0x102cafac => { vm.decode(0x459ce111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cafb0u64) }
        0x102cafb0 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cafb4u64) }
        0x102cafb4 => { vm.decode(0x4594e111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cafb8u64) }
        0x102cafb8 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cafbcu64) }
        0x102cafbc => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cafc0u64) }
        0x102cafc0 => { vm.decode(0xaa40d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 12i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cafc4u64) }
        0x102cafc4 => { vm.decode(0x132fb5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cafc8u64) }
        0x102cafc8 => { vm.decode(0x41052835u32); let d = 5i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cafccu64) }
        0x102cafcc => { vm.decode(0x21347f5u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cafd0u64) }
        0x102cafd0 => { vm.decode(0x6140a835u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cafd4u64) }
        0x102cafd4 => { vm.decode(0x459c6911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cafd8u64) }
        0x102cafd8 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cafdcu64) }
        0x102cafdc => { vm.decode(0x45946911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cafe0u64) }
        0x102cafe0 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cafe4u64) }
        0x102cafe4 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cafe8u64) }
        0x102cafe8 => { vm.decode(0x1e01cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 27i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cafecu64) }
        0x102cafec => { vm.decode(0x24840fb5u32); let d = 18i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102caff0u64) }
        0x102caff0 => { vm.decode(0x50a835u32); let d = 1i16; let s = 4i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caff4u64) }
        0x102caff4 => { vm.decode(0x21217f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caff8u64) }
        0x102caff8 => { vm.decode(0x40012835u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102caffcu64) }
        0x102caffc => { vm.decode(0xd144b11u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb000u64) }
        0x102cb000 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb004u64) }
        0x102cb004 => { vm.decode(0xd94e911u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb008u64) }
        0x102cb008 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb00cu64) }
        0x102cb00c => { vm.decode(0x40210fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb010u64) }
        0x102cb010 => { vm.decode(0x1c81cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 23i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb014u64) }
        0x102cb014 => { vm.decode(0xe4042fb5u32); let d = 19i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb018u64) }
        0x102cb018 => { vm.decode(0x20543835u32); let d = 1i16; let s = 18i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb01cu64) }
        0x102cb01c => { vm.decode(0x120207f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb020u64) }
        0x102cb020 => { vm.decode(0x40502835u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb024u64) }
        0x102cb024 => { vm.decode(0x45904211u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb028u64) }
        0x102cb028 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb02cu64) }
        0x102cb02c => { vm.decode(0x4518e811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb030u64) }
        0x102cb030 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb034u64) }
        0x102cb034 => { vm.decode(0x44210fb5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb038u64) }
        0x102cb038 => { vm.decode(0x1c014d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 18i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb03cu64) }
        0x102cb03c => { vm.decode(0xe4012fb5u32); let d = 3i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb040u64) }
        0x102cb040 => { vm.decode(0x60443835u32); let d = 1i16; let s = 3i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb044u64) }
        0x102cb044 => { vm.decode(0x31207f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb048u64) }
        0x102cb048 => { vm.decode(0x60502835u32); let d = 1i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb04cu64) }
        0x102cb04c => { vm.decode(0x45104a11u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb050u64) }
        0x102cb050 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb054u64) }
        0x102cb054 => { vm.decode(0x45986011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb058u64) }
        0x102cb058 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb05cu64) }
        0x102cb05c => { vm.decode(0x40110fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb060u64) }
        0x102cb060 => { vm.decode(0x1a804d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 12i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb064u64) }
        0x102cb064 => { vm.decode(0x3357062u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cb068u64) }
        0x102cb068 => { vm.decode(0x44012fb5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb06cu64) }
        0x102cb06c => { vm.decode(0x3253062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cb070u64) }
        0x102cb070 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cb074u64) }
        0x102cb074 => { vm.decode(0x5e445059u32); let d = 22i16; let s = 1i16; let s2 = -1i16; let imm = -23784i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cb078u64) }
        0x102cb078 => { vm.decode(0x12109019u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cb07cu64) }
        0x102cb07c => { vm.decode(0x24002406u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cb080u64) }
        0x102cb080 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102cb084u64).wrapping_add((imm * 4) as u64) } else { 0x102cb084u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cb084u64) }
        0x102cb084 => { vm.decode(0x7100236u32); let d = 30i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cb088u64) }
        0x102cb088 => { vm.decode(0x8520d09du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 282080i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102cb08cu64) }
        0x102cb090 => { vm.decode(0xa224735u32); let d = 1i16; let s = 4i16; let s2 = 21i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb094u64) }
        0x102cb094 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102cb098u64) }
        0x102cb098 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb09cu64) }
        0x102cb09c => { vm.decode(0x64314675u32); let d = 3i16; let s = 3i16; let s2 = 21i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb0a0u64) }
        0x102cb0a0 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb0a4u64) }
        0x102cb0a4 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb0a8u64) }
        0x102cb0a8 => { vm.decode(0x92104f5u32); let d = 2i16; let s = 16i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cb0acu64) }
        0x102cb0ac => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cb0b0u64) }
        0x102cb0b0 => { vm.decode(0x8903036u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cb0b4u64) }
        0x102cb0b4 => { vm.decode(0x24002406u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cb0b8u64) }
        0x102cb0b8 => { vm.decode(0x4048227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 41i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cb0bcu64).wrapping_add((imm * 4) as u64) } else { 0x102cb0bcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cb0bcu64) }
        0x102cb0bc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cb0c0u64) } }
        0x102cb0c0 => { vm.decode(0x2084c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cb0c4u64) }
        0x102cb0c4 => { vm.decode(0x616004f5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cb0c8u64) }
        0x102cb0c8 => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cb0ccu64) }
        0x102cb0cc => { vm.decode(0x82c202fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cb0d0u64) }
        0x102cb0d0 => { vm.decode(0x9b195530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cb0d4u64) }
        0x102cb0d4 => { vm.decode(0xb091530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cb0d8u64) }
        0x102cb0d8 => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cb0dcu64) }
        0x102cb0dc => { vm.decode(0x40820bb5u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb0e0u64) }
        0x102cb0e0 => { vm.decode(0xc000ad75u32); let d = 20i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102cb0e4u64) }
        0x102cb0e4 => { vm.decode(0x288063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cb0e8u64) }
        0x102cb0e8 => { vm.decode(0x216004f5u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cb0ecu64) }
        0x102cb0ec => { vm.decode(0x1c000111u32); let d = 21i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb0f0u64) }
        0x102cb0f0 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cb0f4u64) }
        0x102cb0f4 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cb0f8u64) }
        0x102cb0f8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cb0fcu64 + 4);
        vm.wr(37, 0x102cb0fcu64 + 4);
        Ok(0x102cb0fcu64) }
        0x102cb0fc => { vm.decode(0x660d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cb100u64) }
        0x102cb100 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb104u64) }
        0x102cb104 => { vm.decode(0x2acc0b35u32); let d = 4i16; let s = 3i16; let s2 = 30i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb108u64) }
        0x102cb108 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cb10cu64) }
        0x102cb10c => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102cb110u64) }
        0x102cb110 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cb114u64).wrapping_add((imm * 4) as u64) } else { 0x102cb114u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cb114u64) }
        0x102cb114 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cb118u64) }
        0x102cb118 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cb11cu64) }
        0x102cb11c => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb120u64) }
        0x102cb120 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102cb124u64) }
        0x102cb124 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102cb128u64) }
        0x102cb128 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102cb12cu64).wrapping_add((imm * 4) as u64) } else { 0x102cb12cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cb12cu64) }
        0x102cb12c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cb130u64) } }
        0x102cb130 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cb134u64) }
        0x102cb134 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102cb138u64) }
        0x102cb138 => { vm.decode(0x739cb4a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -43i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cb13cu64).wrapping_add((imm * 4) as u64) } else { 0x102cb13cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cb13cu64) }
        0x102cb13c => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cb140u64) }
        0x102cb140 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cb144u64) }
        0x102cb144 => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb148u64) }
        0x102cb148 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102cb14cu64) }
        0x102cb14c => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102cb150u64) }
        0x102cb150 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102cb154u64).wrapping_add((imm * 4) as u64) } else { 0x102cb154u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cb154u64) }
        0x102cb154 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cb158u64) } }
        0x102cb158 => { vm.decode(0x400d49du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 281872i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102cb15cu64) }
        0x102cb15c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cb160u64) } }
        0x102cb160 => { vm.decode(0xf10c091u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2060i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb164u64) }
        0x102cb164 => { vm.decode(0x43251062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cb168u64) }
        0x102cb168 => { vm.decode(0xf10c011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2056i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb16cu64) }
        0x102cb16c => { vm.decode(0x3351062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cb170u64) }
        0x102cb170 => { vm.decode(0xf104091u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2052i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb174u64) }
        0x102cb174 => { vm.decode(0x3251062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cb178u64) }
        0x102cb178 => { vm.decode(0xf104011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2048i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb17cu64) }
        0x102cb17c => { vm.decode(0x63153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cb180u64) }
        0x102cb180 => { vm.decode(0xd9cead1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2044i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb184u64) }
        0x102cb184 => { vm.decode(0x63053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cb188u64) }
        0x102cb188 => { vm.decode(0xd9cea51u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2040i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb18cu64) }
        0x102cb18c => { vm.decode(0x63151062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cb190u64) }
        0x102cb190 => { vm.decode(0xd9c6ad1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2036i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb194u64) }
        0x102cb194 => { vm.decode(0x63051062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cb198u64) }
        0x102cb198 => { vm.decode(0xd9c6a51u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2032i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb19cu64) }
        0x102cb19c => { vm.decode(0x43153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cb1a0u64) }
        0x102cb1a0 => { vm.decode(0xd9ce2d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2028i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb1a4u64) }
        0x102cb1a4 => { vm.decode(0x43053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cb1a8u64) }
        0x102cb1a8 => { vm.decode(0xd9ce251u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2024i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb1acu64) }
        0x102cb1ac => { vm.decode(0x43151062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cb1b0u64) }
        0x102cb1b0 => { vm.decode(0xd9c62d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2020i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb1b4u64) }
        0x102cb1b4 => { vm.decode(0x43051062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cb1b8u64) }
        0x102cb1b8 => { vm.decode(0xd9c6251u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2016i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb1bcu64) }
        0x102cb1bc => { vm.decode(0x23153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cb1c0u64) }
        0x102cb1c0 => { vm.decode(0xd98ead1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2012i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb1c4u64) }
        0x102cb1c4 => { vm.decode(0x23053062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cb1c8u64) }
        0x102cb1c8 => { vm.decode(0x5598fb51u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2008i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb1ccu64) }
        0x102cb1cc => { vm.decode(0x1d986ad1u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2004i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb1d0u64) }
        0x102cb1d0 => { vm.decode(0x55986b51u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb1d4u64) }
        0x102cb1d4 => { vm.decode(0x6a8263afu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cb1d8u64) }
        0x102cb1d8 => { vm.decode(0x2884412fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cb1dcu64) }
        0x102cb1dc => { vm.decode(0x12009119u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cb1e0u64) }
        0x102cb1e0 => { vm.decode(0x9b215530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cb1e4u64) }
        0x102cb1e4 => { vm.decode(0xa8ce32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cb1e8u64) }
        0x102cb1e8 => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cb1ecu64) }
        0x102cb1ec => { vm.decode(0x88843afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cb1f0u64) }
        0x102cb1f0 => { vm.decode(0xb1115b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cb1f4u64) }
        0x102cb1f4 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cb1f8u64) }
        0x102cb1f8 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cb1fcu64) }
        0x102cb1fc => { vm.decode(0x488643afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cb200u64) }
        0x102cb200 => { vm.decode(0xa10c875u32); let d = 25i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb204u64) }
        0x102cb204 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cb208u64 + 4);
        vm.wr(37, 0x102cb208u64 + 4);
        Ok(0x102cb208u64) }
        0x102cb208 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cb20cu64) }
        0x102cb20c => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cb210u64) }
        0x102cb210 => { vm.decode(0x2280416fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cb214u64) }
        0x102cb214 => { vm.decode(0x9b2155b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cb218u64) }
        0x102cb218 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102cb21cu64) }
        0x102cb21c => { vm.decode(0x28c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cb220u64) }
        0x102cb220 => { vm.decode(0xb311530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cb224u64) }
        0x102cb224 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cb228u64) }
        0x102cb228 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cb22cu64) }
        0x102cb22c => { vm.decode(0xa10c875u32); let d = 25i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb230u64) }
        0x102cb230 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cb234u64 + 4);
        vm.wr(37, 0x102cb234u64 + 4);
        Ok(0x102cb234u64) }
        0x102cb234 => { vm.decode(0x100036u32); let d = 16i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cb238u64) }
        0x102cb238 => { vm.decode(0x518e911u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb23cu64) }
        0x102cb23c => { vm.decode(0x45186911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb240u64) }
        0x102cb240 => { vm.decode(0x41a835u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb244u64) }
        0x102cb244 => { vm.decode(0x60043835u32); let d = 1i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb248u64) }
        0x102cb248 => { vm.decode(0x459ce011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb24cu64) }
        0x102cb24c => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb250u64) }
        0x102cb250 => { vm.decode(0xc4802fb5u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb254u64) }
        0x102cb254 => { vm.decode(0x40290fb5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb258u64) }
        0x102cb258 => { vm.decode(0x1a814d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 28i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb25cu64) }
        0x102cb25c => { vm.decode(0x44802fb5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb260u64) }
        0x102cb260 => { vm.decode(0x200c8835u32); let d = 2i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb264u64) }
        0x102cb264 => { vm.decode(0x488835u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb268u64) }
        0x102cb268 => { vm.decode(0x4d104211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb26cu64) }
        0x102cb26c => { vm.decode(0x20212fb5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb270u64) }
        0x102cb270 => { vm.decode(0xa0210fb5u32); let d = 2i16; let s = 2i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb274u64) }
        0x102cb274 => { vm.decode(0x24282fb5u32); let d = 2i16; let s = 2i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb278u64) }
        0x102cb278 => { vm.decode(0x28918d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 21i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb27cu64) }
        0x102cb27c => { vm.decode(0x20210fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb280u64) }
        0x102cb280 => { vm.decode(0x40482835u32); let d = 3i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb284u64) }
        0x102cb284 => { vm.decode(0x204ca835u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb288u64) }
        0x102cb288 => { vm.decode(0x4d14cb11u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb28cu64) }
        0x102cb28c => { vm.decode(0x64a12fb5u32); let d = 3i16; let s = 3i16; let s2 = 7i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb290u64) }
        0x102cb290 => { vm.decode(0xf4a02fb5u32); let d = 3i16; let s = 3i16; let s2 = 30i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb294u64) }
        0x102cb294 => { vm.decode(0x60112fb5u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb298u64) }
        0x102cb298 => { vm.decode(0x38114d35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 16i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb29cu64) }
        0x102cb29c => { vm.decode(0x64202fb5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb2a0u64) }
        0x102cb2a0 => { vm.decode(0x21440835u32); let d = 4i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb2a4u64) }
        0x102cb2a4 => { vm.decode(0x41010835u32); let d = 4i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb2a8u64) }
        0x102cb2a8 => { vm.decode(0xd144311u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb2acu64) }
        0x102cb2ac => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb2b0u64) }
        0x102cb2b0 => { vm.decode(0xd146911u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb2b4u64) }
        0x102cb2b4 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb2b8u64) }
        0x102cb2b8 => { vm.decode(0xb20fb5u32); let d = 4i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb2bcu64) }
        0x102cb2bc => { vm.decode(0xa248d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 9i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb2c0u64) }
        0x102cb2c0 => { vm.decode(0x132fb5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb2c4u64) }
        0x102cb2c4 => { vm.decode(0x41052835u32); let d = 5i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb2c8u64) }
        0x102cb2c8 => { vm.decode(0x21052835u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb2ccu64) }
        0x102cb2cc => { vm.decode(0x4590c311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb2d0u64) }
        0x102cb2d0 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb2d4u64) }
        0x102cb2d4 => { vm.decode(0x4514e911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb2d8u64) }
        0x102cb2d8 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb2dcu64) }
        0x102cb2dc => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb2e0u64) }
        0x102cb2e0 => { vm.decode(0x1a814d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 28i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb2e4u64) }
        0x102cb2e4 => { vm.decode(0x44800fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb2e8u64) }
        0x102cb2e8 => { vm.decode(0x2100a835u32); let d = 5i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb2ecu64) }
        0x102cb2ec => { vm.decode(0x61052835u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb2f0u64) }
        0x102cb2f0 => { vm.decode(0x44932fb5u32); let d = 5i16; let s = 5i16; let s2 = 7i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb2f4u64) }
        0x102cb2f4 => { vm.decode(0x45906111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb2f8u64) }
        0x102cb2f8 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb2fcu64) }
        0x102cb2fc => { vm.decode(0x20a10fb5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb300u64) }
        0x102cb300 => { vm.decode(0x28918d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 21i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb304u64) }
        0x102cb304 => { vm.decode(0x20210fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb308u64) }
        0x102cb308 => { vm.decode(0x41402835u32); let d = 5i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb30cu64) }
        0x102cb30c => { vm.decode(0x2101a835u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb310u64) }
        0x102cb310 => { vm.decode(0x45144b11u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb314u64) }
        0x102cb314 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb318u64) }
        0x102cb318 => { vm.decode(0x4590e111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb31cu64) }
        0x102cb31c => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb320u64) }
        0x102cb320 => { vm.decode(0x64a10fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb324u64) }
        0x102cb324 => { vm.decode(0x38114d35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 16i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb328u64) }
        0x102cb328 => { vm.decode(0x64202fb5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb32cu64) }
        0x102cb32c => { vm.decode(0x21442835u32); let d = 5i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb330u64) }
        0x102cb330 => { vm.decode(0x61012835u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb334u64) }
        0x102cb334 => { vm.decode(0x45104b11u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb338u64) }
        0x102cb338 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb33cu64) }
        0x102cb33c => { vm.decode(0x45906911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb340u64) }
        0x102cb340 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb344u64) }
        0x102cb344 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb348u64) }
        0x102cb348 => { vm.decode(0xa248d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 9i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb34cu64) }
        0x102cb34c => { vm.decode(0x132fb5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb350u64) }
        0x102cb350 => { vm.decode(0x41052835u32); let d = 5i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb354u64) }
        0x102cb354 => { vm.decode(0x21052835u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb358u64) }
        0x102cb358 => { vm.decode(0x4510cb11u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb35cu64) }
        0x102cb35c => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb360u64) }
        0x102cb360 => { vm.decode(0x4590e911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb364u64) }
        0x102cb364 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb368u64) }
        0x102cb368 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb36cu64) }
        0x102cb36c => { vm.decode(0x1a814d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 28i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb370u64) }
        0x102cb370 => { vm.decode(0x44800fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb374u64) }
        0x102cb374 => { vm.decode(0x2100a835u32); let d = 5i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb378u64) }
        0x102cb378 => { vm.decode(0x61052835u32); let d = 5i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb37cu64) }
        0x102cb37c => { vm.decode(0x4598e911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb380u64) }
        0x102cb380 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb384u64) }
        0x102cb384 => { vm.decode(0x45946111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb388u64) }
        0x102cb388 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb38cu64) }
        0x102cb38c => { vm.decode(0x20a10fb5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb390u64) }
        0x102cb390 => { vm.decode(0x28918d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 21i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb394u64) }
        0x102cb394 => { vm.decode(0x20210fb5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb398u64) }
        0x102cb398 => { vm.decode(0x41402835u32); let d = 5i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb39cu64) }
        0x102cb39c => { vm.decode(0x2101a835u32); let d = 5i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb3a0u64) }
        0x102cb3a0 => { vm.decode(0x4514c311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb3a4u64) }
        0x102cb3a4 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb3a8u64) }
        0x102cb3a8 => { vm.decode(0x4594e111u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb3acu64) }
        0x102cb3ac => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb3b0u64) }
        0x102cb3b0 => { vm.decode(0x64a10fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb3b4u64) }
        0x102cb3b4 => { vm.decode(0x38114d35u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 16i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb3b8u64) }
        0x102cb3b8 => { vm.decode(0x64202fb5u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb3bcu64) }
        0x102cb3bc => { vm.decode(0x21442835u32); let d = 5i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb3c0u64) }
        0x102cb3c0 => { vm.decode(0x61012835u32); let d = 5i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb3c4u64) }
        0x102cb3c4 => { vm.decode(0x459c6911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb3c8u64) }
        0x102cb3c8 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb3ccu64) }
        0x102cb3cc => { vm.decode(0x45946911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb3d0u64) }
        0x102cb3d0 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb3d4u64) }
        0x102cb3d4 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb3d8u64) }
        0x102cb3d8 => { vm.decode(0xa248d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 9i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb3dcu64) }
        0x102cb3dc => { vm.decode(0x132fb5u32); let d = 4i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb3e0u64) }
        0x102cb3e0 => { vm.decode(0x41052835u32); let d = 5i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb3e4u64) }
        0x102cb3e4 => { vm.decode(0x21052835u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb3e8u64) }
        0x102cb3e8 => { vm.decode(0x45904311u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb3ecu64) }
        0x102cb3ec => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb3f0u64) }
        0x102cb3f0 => { vm.decode(0x4594e911u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb3f4u64) }
        0x102cb3f4 => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb3f8u64) }
        0x102cb3f8 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb3fcu64) }
        0x102cb3fc => { vm.decode(0x1a814d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 28i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb400u64) }
        0x102cb400 => { vm.decode(0x4860fb5u32); let d = 20i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb404u64) }
        0x102cb404 => { vm.decode(0x11a835u32); let d = 1i16; let s = 4i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb408u64) }
        0x102cb408 => { vm.decode(0x60042835u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb40cu64) }
        0x102cb40c => { vm.decode(0xd90cb11u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb410u64) }
        0x102cb410 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb414u64) }
        0x102cb414 => { vm.decode(0xd186111u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb418u64) }
        0x102cb418 => { vm.decode(0x44810fb5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb41cu64) }
        0x102cb41c => { vm.decode(0x40210fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb420u64) }
        0x102cb420 => { vm.decode(0x1881cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 21i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb424u64) }
        0x102cb424 => { vm.decode(0xe4840fb5u32); let d = 19i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb428u64) }
        0x102cb428 => { vm.decode(0x2050b835u32); let d = 1i16; let s = 19i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb42cu64) }
        0x102cb42c => { vm.decode(0x2000a835u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb430u64) }
        0x102cb430 => { vm.decode(0x459ce811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb434u64) }
        0x102cb434 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb438u64) }
        0x102cb438 => { vm.decode(0x4518e011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb43cu64) }
        0x102cb43c => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb440u64) }
        0x102cb440 => { vm.decode(0x44210fb5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb444u64) }
        0x102cb444 => { vm.decode(0x18014d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb448u64) }
        0x102cb448 => { vm.decode(0xa4052fb5u32); let d = 18i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb44cu64) }
        0x102cb44c => { vm.decode(0x40543835u32); let d = 1i16; let s = 18i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb450u64) }
        0x102cb450 => { vm.decode(0x2000b835u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb454u64) }
        0x102cb454 => { vm.decode(0x45944211u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb458u64) }
        0x102cb458 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb45cu64) }
        0x102cb45c => { vm.decode(0x45986011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb460u64) }
        0x102cb460 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb464u64) }
        0x102cb464 => { vm.decode(0x40110fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb468u64) }
        0x102cb468 => { vm.decode(0x1a00cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 9i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb46cu64) }
        0x102cb46c => { vm.decode(0xc4002fb5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb470u64) }
        0x102cb470 => { vm.decode(0x43353062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cb474u64) }
        0x102cb474 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cb478u64) }
        0x102cb478 => { vm.decode(0x5e44c0d9u32); let d = 21i16; let s = 1i16; let s2 = -1i16; let imm = -23783i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cb47cu64) }
        0x102cb47c => { vm.decode(0x12101019u32); let d = 22i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cb480u64) }
        0x102cb480 => { vm.decode(0x4002406u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cb484u64) }
        0x102cb484 => { vm.decode(0x2200031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102cb488u64).wrapping_add((imm * 4) as u64) } else { 0x102cb488u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cb488u64) }
        0x102cb488 => { vm.decode(0x6102236u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cb48cu64) }
        0x102cb48c => { vm.decode(0x8924909du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 283112i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102cb490u64) }
        0x102cb494 => { vm.decode(0xa024735u32); let d = 1i16; let s = 4i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb498u64) }
        0x102cb498 => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102cb49cu64) }
        0x102cb49c => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb4a0u64) }
        0x102cb4a0 => { vm.decode(0x64114675u32); let d = 3i16; let s = 3i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb4a4u64) }
        0x102cb4a4 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb4a8u64) }
        0x102cb4a8 => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb4acu64) }
        0x102cb4ac => { vm.decode(0x28c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cb4b0u64) }
        0x102cb4b0 => { vm.decode(0x4b2014f5u32); let d = 2i16; let s = 2i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cb4b4u64) }
        0x102cb4b4 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cb4b8u64) }
        0x102cb4b8 => { vm.decode(0x8900036u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cb4bcu64) }
        0x102cb4bc => { vm.decode(0x4002406u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102cb4c0u64) }
        0x102cb4c0 => { vm.decode(0x4048227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 41i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cb4c4u64).wrapping_add((imm * 4) as u64) } else { 0x102cb4c4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cb4c4u64) }
        0x102cb4c4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cb4c8u64) } }
        0x102cb4c8 => { vm.decode(0x2084c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cb4ccu64) }
        0x102cb4cc => { vm.decode(0xa16004f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cb4d0u64) }
        0x102cb4d0 => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cb4d4u64) }
        0x102cb4d4 => { vm.decode(0x82c202fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cb4d8u64) }
        0x102cb4d8 => { vm.decode(0x8b195530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cb4dcu64) }
        0x102cb4dc => { vm.decode(0xb091530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cb4e0u64) }
        0x102cb4e0 => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cb4e4u64) }
        0x102cb4e4 => { vm.decode(0x40020bb5u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb4e8u64) }
        0x102cb4e8 => { vm.decode(0xc008bd75u32); let d = 30i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102cb4ecu64) }
        0x102cb4ec => { vm.decode(0x288043afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cb4f0u64) }
        0x102cb4f0 => { vm.decode(0x616014f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cb4f4u64) }
        0x102cb4f4 => { vm.decode(0x1c000011u32); let d = 17i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb4f8u64) }
        0x102cb4f8 => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cb4fcu64) }
        0x102cb4fc => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cb500u64) }
        0x102cb500 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cb504u64 + 4);
        vm.wr(37, 0x102cb504u64 + 4);
        Ok(0x102cb504u64) }
        0x102cb504 => { vm.decode(0x660d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cb508u64) }
        0x102cb508 => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb50cu64) }
        0x102cb50c => { vm.decode(0x28cc8b35u32); let d = 4i16; let s = 3i16; let s2 = 23i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb510u64) }
        0x102cb510 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cb514u64) }
        0x102cb514 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102cb518u64) }
        0x102cb518 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cb51cu64).wrapping_add((imm * 4) as u64) } else { 0x102cb51cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cb51cu64) }
        0x102cb51c => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cb520u64) }
        0x102cb520 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cb524u64) }
        0x102cb524 => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb528u64) }
        0x102cb528 => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102cb52cu64) }
        0x102cb52c => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102cb530u64) }
        0x102cb530 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102cb534u64).wrapping_add((imm * 4) as u64) } else { 0x102cb534u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cb534u64) }
        0x102cb534 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cb538u64) } }
        0x102cb538 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cb53cu64) }
        0x102cb53c => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102cb540u64) }
        0x102cb540 => { vm.decode(0x739c34a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -44i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cb544u64).wrapping_add((imm * 4) as u64) } else { 0x102cb544u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cb544u64) }
        0x102cb544 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cb548u64) }
        0x102cb548 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cb54cu64) }
        0x102cb54c => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb550u64) }
        0x102cb550 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102cb554u64) }
        0x102cb554 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102cb558u64) }
        0x102cb558 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102cb55cu64).wrapping_add((imm * 4) as u64) } else { 0x102cb55cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cb55cu64) }
        0x102cb55c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cb560u64) } }
        0x102cb560 => { vm.decode(0x2004949du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 282900i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102cb564u64) }
        0x102cb564 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cb568u64) } }
        0x102cb568 => { vm.decode(0x110229b5u32); let d = 1i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102cb56cu64) }
        0x102cb56c => { vm.decode(0x1d98f811u32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb570u64) }
        0x102cb570 => { vm.decode(0x4388875u32); let d = 1i16; let s = 1i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb574u64) }
        0x102cb574 => { vm.decode(0x40502835u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb578u64) }
        0x102cb578 => { vm.decode(0x459ce011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb57cu64) }
        0x102cb57c => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb580u64) }
        0x102cb580 => { vm.decode(0x45906a51u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb584u64) }
        0x102cb584 => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb588u64) }
        0x102cb588 => { vm.decode(0x3019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cb58cu64) }
        0x102cb58c => { vm.decode(0x40190fb5u32); let d = 1i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb590u64) }
        0x102cb590 => { vm.decode(0x2a88c36fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cb594u64) }
        0x102cb594 => { vm.decode(0x48320b75u32); let d = 20i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_sub(vm.ld(s2));
        vm.st(d, v);
        Ok(0x102cb598u64) }
        0x102cb598 => { vm.decode(0x888e36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cb59cu64) }
        0x102cb59c => { vm.decode(0xca0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cb5a0u64) }
        0x102cb5a0 => { vm.decode(0x9209d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cb5a4u64) }
        0x102cb5a4 => { vm.decode(0x8229b5u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102cb5a8u64) }
        0x102cb5a8 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cb5acu64) }
        0x102cb5ac => { vm.decode(0x62255022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cb5b0u64) }
        0x102cb5b0 => { vm.decode(0x4d90eb51u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 1880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb5b4u64) }
        0x102cb5b4 => { vm.decode(0x590fad1u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 1884i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb5b8u64) }
        0x102cb5b8 => { vm.decode(0xd947251u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 1888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb5bcu64) }
        0x102cb5bc => { vm.decode(0xe114d35u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 26i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb5c0u64) }
        0x102cb5c0 => { vm.decode(0x411129b5u32); let d = 10i16; let s = 0i16; let s2 = 25i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102cb5c4u64) }
        0x102cb5c4 => { vm.decode(0x45906bd1u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 1876i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb5c8u64) }
        0x102cb5c8 => { vm.decode(0xd18e2d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1932i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb5ccu64) }
        0x102cb5cc => { vm.decode(0x91230fb5u32); let d = 12i16; let s = 2i16; let s2 = 25i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb5d0u64) }
        0x102cb5d0 => { vm.decode(0x4518e251u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb5d4u64) }
        0x102cb5d4 => { vm.decode(0x4d106a11u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb5d8u64) }
        0x102cb5d8 => { vm.decode(0x10a04875u32); let d = 4i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb5dcu64) }
        0x102cb5dc => { vm.decode(0x2112a835u32); let d = 5i16; let s = 4i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb5e0u64) }
        0x102cb5e0 => { vm.decode(0x51863d1u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1924i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb5e4u64) }
        0x102cb5e4 => { vm.decode(0x4d9c7811u32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb5e8u64) }
        0x102cb5e8 => { vm.decode(0x75112fb5u32); let d = 11i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb5ecu64) }
        0x102cb5ec => { vm.decode(0xd186351u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb5f0u64) }
        0x102cb5f0 => { vm.decode(0x65a06fb5u32); let d = 11i16; let s = 6i16; let s2 = 11i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb5f4u64) }
        0x102cb5f4 => { vm.decode(0x4594ebd1u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 1916i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb5f8u64) }
        0x102cb5f8 => { vm.decode(0x75292fb5u32); let d = 11i16; let s = 11i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb5fcu64) }
        0x102cb5fc => { vm.decode(0x3c917d35u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 22i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb600u64) }
        0x102cb600 => { vm.decode(0x55a24fb5u32); let d = 13i16; let s = 11i16; let s2 = 12i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb604u64) }
        0x102cb604 => { vm.decode(0x47604875u32); let d = 10i16; let s = 10i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb608u64) }
        0x102cb608 => { vm.decode(0x120f2835u32); let d = 11i16; let s = 10i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb60cu64) }
        0x102cb60c => { vm.decode(0x4594fa51u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 1912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb610u64) }
        0x102cb610 => { vm.decode(0x459cf911u32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb614u64) }
        0x102cb614 => { vm.decode(0x35a26fb5u32); let d = 14i16; let s = 11i16; let s2 = 14i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb618u64) }
        0x102cb618 => { vm.decode(0x4d947ad1u32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 1908i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb61cu64) }
        0x102cb61c => { vm.decode(0x60b36fb5u32); let d = 7i16; let s = 7i16; let s2 = 14i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb620u64) }
        0x102cb620 => { vm.decode(0x409119b5u32); let d = 14i16; let s = 0i16; let s2 = 12i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102cb624u64) }
        0x102cb624 => { vm.decode(0x60ab2fb5u32); let d = 7i16; let s = 7i16; let s2 = 18i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb628u64) }
        0x102cb628 => { vm.decode(0x3835cd35u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 17i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb62cu64) }
        0x102cb62c => { vm.decode(0x75b30fb5u32); let d = 15i16; let s = 7i16; let s2 = 13i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb630u64) }
        0x102cb630 => { vm.decode(0x55e0d875u32); let d = 7i16; let s = 14i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb634u64) }
        0x102cb634 => { vm.decode(0x610fa835u32); let d = 7i16; let s = 7i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb638u64) }
        0x102cb638 => { vm.decode(0x45947b51u32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 1904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb63cu64) }
        0x102cb63c => { vm.decode(0x15105211u32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb640u64) }
        0x102cb640 => { vm.decode(0xf4320fb5u32); let d = 7i16; let s = 7i16; let s2 = 24i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb644u64) }
        0x102cb644 => { vm.decode(0x1594f2d1u32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 1900i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb648u64) }
        0x102cb648 => { vm.decode(0x74320fb5u32); let d = 7i16; let s = 7i16; let s2 = 8i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb64cu64) }
        0x102cb64c => { vm.decode(0x411019b5u32); let d = 8i16; let s = 0i16; let s2 = 13i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102cb650u64) }
        0x102cb650 => { vm.decode(0x648b6fb5u32); let d = 7i16; let s = 7i16; let s2 = 25i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb654u64) }
        0x102cb654 => { vm.decode(0x3e34cd35u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 11i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb658u64) }
        0x102cb658 => { vm.decode(0x55b52fb5u32); let d = 25i16; let s = 7i16; let s2 = 15i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb65cu64) }
        0x102cb65c => { vm.decode(0x45b08875u32); let d = 7i16; let s = 8i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb660u64) }
        0x102cb660 => { vm.decode(0x614fa835u32); let d = 7i16; let s = 7i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb664u64) }
        0x102cb664 => { vm.decode(0x1594e251u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 1896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb668u64) }
        0x102cb668 => { vm.decode(0x5105a11u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb66cu64) }
        0x102cb66c => { vm.decode(0x74320fb5u32); let d = 7i16; let s = 7i16; let s2 = 8i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb670u64) }
        0x102cb670 => { vm.decode(0x59472d1u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 1892i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb674u64) }
        0x102cb674 => { vm.decode(0x74330fb5u32); let d = 7i16; let s = 7i16; let s2 = 9i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb678u64) }
        0x102cb678 => { vm.decode(0x511219b5u32); let d = 9i16; let s = 0i16; let s2 = 15i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102cb67cu64) }
        0x102cb67c => { vm.decode(0x60936fb5u32); let d = 7i16; let s = 7i16; let s2 = 12i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb680u64) }
        0x102cb680 => { vm.decode(0x3e354d35u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 26i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb684u64) }
        0x102cb684 => { vm.decode(0x95330fb5u32); let d = 12i16; let s = 7i16; let s2 = 25i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb688u64) }
        0x102cb688 => { vm.decode(0x41a8c875u32); let d = 7i16; let s = 9i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb68cu64) }
        0x102cb68c => { vm.decode(0x611ea835u32); let d = 7i16; let s = 7i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb690u64) }
        0x102cb690 => { vm.decode(0xd10fa91u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 796i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb694u64) }
        0x102cb694 => { vm.decode(0x1d9c6011u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb698u64) }
        0x102cb698 => { vm.decode(0xc4350fb5u32); let d = 17i16; let s = 7i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb69cu64) }
        0x102cb69c => { vm.decode(0x4d10eb11u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb6a0u64) }
        0x102cb6a0 => { vm.decode(0x15080fb5u32); let d = 8i16; let s = 8i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb6a4u64) }
        0x102cb6a4 => { vm.decode(0x1110a9b5u32); let d = 17i16; let s = 0i16; let s2 = 25i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102cb6a8u64) }
        0x102cb6a8 => { vm.decode(0x15104fb5u32); let d = 8i16; let s = 8i16; let s2 = 13i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb6acu64) }
        0x102cb6ac => { vm.decode(0xc813d35u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 22i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb6b0u64) }
        0x102cb6b0 => { vm.decode(0x51824fb5u32); let d = 13i16; let s = 8i16; let s2 = 12i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb6b4u64) }
        0x102cb6b4 => { vm.decode(0x6286875u32); let d = 8i16; let s = 13i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb6b8u64) }
        0x102cb6b8 => { vm.decode(0x14032835u32); let d = 17i16; let s = 8i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb6bcu64) }
        0x102cb6bc => { vm.decode(0x5107a91u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 788i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb6c0u64) }
        0x102cb6c0 => { vm.decode(0x5510ca11u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb6c4u64) }
        0x102cb6c4 => { vm.decode(0xc40c2fb5u32); let d = 17i16; let s = 17i16; let s2 = 18i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb6c8u64) }
        0x102cb6c8 => { vm.decode(0x1199b5u32); let d = 18i16; let s = 0i16; let s2 = 12i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102cb6ccu64) }
        0x102cb6cc => { vm.decode(0x840c0fb5u32); let d = 16i16; let s = 16i16; let s2 = 17i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb6d0u64) }
        0x102cb6d0 => { vm.decode(0xe5324fb5u32); let d = 15i16; let s = 15i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb6d4u64) }
        0x102cb6d4 => { vm.decode(0x3835fd35u32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 17i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb6d8u64) }
        0x102cb6d8 => { vm.decode(0x75b34fb5u32); let d = 15i16; let s = 13i16; let s2 = 15i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb6dcu64) }
        0x102cb6dc => { vm.decode(0x11099b5u32); let d = 16i16; let s = 0i16; let s2 = 13i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102cb6e0u64) }
        0x102cb6e0 => { vm.decode(0xc60f875u32); let d = 17i16; let s = 15i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb6e4u64) }
        0x102cb6e4 => { vm.decode(0x64033835u32); let d = 17i16; let s = 13i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb6e8u64) }
        0x102cb6e8 => { vm.decode(0x5514c211u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb6ecu64) }
        0x102cb6ec => { vm.decode(0xc40c2fb5u32); let d = 17i16; let s = 17i16; let s2 = 18i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb6f0u64) }
        0x102cb6f0 => { vm.decode(0x950c0fb5u32); let d = 24i16; let s = 17i16; let s2 = 24i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb6f4u64) }
        0x102cb6f4 => { vm.decode(0x950c4fb5u32); let d = 24i16; let s = 24i16; let s2 = 25i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb6f8u64) }
        0x102cb6f8 => { vm.decode(0x8f00bd35u32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 11i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb6fcu64) }
        0x102cb6fc => { vm.decode(0x118d6fb5u32); let d = 24i16; let s = 15i16; let s2 = 24i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb700u64) }
        0x102cb700 => { vm.decode(0x511299b5u32); let d = 25i16; let s = 0i16; let s2 = 15i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102cb704u64) }
        0x102cb704 => { vm.decode(0x8302875u32); let d = 16i16; let s = 16i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb708u64) }
        0x102cb708 => { vm.decode(0x24431835u32); let d = 16i16; let s = 15i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb70cu64) }
        0x102cb70c => { vm.decode(0x800c2fb5u32); let d = 16i16; let s = 16i16; let s2 = 18i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb710u64) }
        0x102cb710 => { vm.decode(0x318a2fb5u32); let d = 14i16; let s = 14i16; let s2 = 16i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb714u64) }
        0x102cb714 => { vm.decode(0x11926fb5u32); let d = 12i16; let s = 12i16; let s2 = 14i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb718u64) }
        0x102cb718 => { vm.decode(0xe253d35u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 26i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb71cu64) }
        0x102cb71c => { vm.decode(0x91124fb5u32); let d = 12i16; let s = 12i16; let s2 = 24i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb720u64) }
        0x102cb720 => { vm.decode(0x409129b5u32); let d = 14i16; let s = 0i16; let s2 = 24i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102cb724u64) }
        0x102cb724 => { vm.decode(0xb28e875u32); let d = 25i16; let s = 12i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb728u64) }
        0x102cb728 => { vm.decode(0x56123835u32); let d = 25i16; let s = 24i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb72cu64) }
        0x102cb72c => { vm.decode(0x15144211u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb730u64) }
        0x102cb730 => { vm.decode(0xc50c4fb5u32); let d = 25i16; let s = 16i16; let s2 = 25i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb734u64) }
        0x102cb734 => { vm.decode(0x75096fb5u32); let d = 11i16; let s = 11i16; let s2 = 25i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb738u64) }
        0x102cb738 => { vm.decode(0x75116fb5u32); let d = 11i16; let s = 11i16; let s2 = 13i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb73cu64) }
        0x102cb73c => { vm.decode(0x3c917d35u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 22i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb740u64) }
        0x102cb740 => { vm.decode(0x75a04fb5u32); let d = 11i16; let s = 11i16; let s2 = 12i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb744u64) }
        0x102cb744 => { vm.decode(0x509019b5u32); let d = 13i16; let s = 0i16; let s2 = 12i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102cb748u64) }
        0x102cb748 => { vm.decode(0x57e01875u32); let d = 14i16; let s = 11i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb74cu64) }
        0x102cb74c => { vm.decode(0x130f8835u32); let d = 14i16; let s = 12i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb750u64) }
        0x102cb750 => { vm.decode(0x1d145a11u32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb754u64) }
        0x102cb754 => { vm.decode(0xb1334fb5u32); let d = 14i16; let s = 14i16; let s2 = 25i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb758u64) }
        0x102cb758 => { vm.decode(0x31306fb5u32); let d = 10i16; let s = 10i16; let s2 = 14i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb75cu64) }
        0x102cb75c => { vm.decode(0x35306fb5u32); let d = 10i16; let s = 10i16; let s2 = 15i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb760u64) }
        0x102cb760 => { vm.decode(0x2811bd35u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 17i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb764u64) }
        0x102cb764 => { vm.decode(0x31216fb5u32); let d = 10i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb768u64) }
        0x102cb768 => { vm.decode(0x419309b5u32); let d = 14i16; let s = 0i16; let s2 = 11i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102cb76cu64) }
        0x102cb76c => { vm.decode(0x13a89875u32); let d = 13i16; let s = 10i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb770u64) }
        0x102cb770 => { vm.decode(0x7342a835u32); let d = 13i16; let s = 11i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb774u64) }
        0x102cb774 => { vm.decode(0x4d14db11u32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb778u64) }
        0x102cb778 => { vm.decode(0x55936fb5u32); let d = 13i16; let s = 13i16; let s2 = 15i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb77cu64) }
        0x102cb77c => { vm.decode(0x24926fb5u32); let d = 6i16; let s = 6i16; let s2 = 13i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb780u64) }
        0x102cb780 => { vm.decode(0x208a6fb5u32); let d = 6i16; let s = 6i16; let s2 = 24i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb784u64) }
        0x102cb784 => { vm.decode(0x2e348d35u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 11i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb788u64) }
        0x102cb788 => { vm.decode(0x30322fb5u32); let d = 6i16; let s = 6i16; let s2 = 10i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb78cu64) }
        0x102cb78c => { vm.decode(0x509209b5u32); let d = 13i16; let s = 0i16; let s2 = 10i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102cb790u64) }
        0x102cb790 => { vm.decode(0x53c05875u32); let d = 14i16; let s = 6i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb794u64) }
        0x102cb794 => { vm.decode(0x134e8835u32); let d = 14i16; let s = 10i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb798u64) }
        0x102cb798 => { vm.decode(0x4d905311u32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb79cu64) }
        0x102cb79c => { vm.decode(0x31b36fb5u32); let d = 14i16; let s = 14i16; let s2 = 15i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb7a0u64) }
        0x102cb7a0 => { vm.decode(0x40b34fb5u32); let d = 5i16; let s = 5i16; let s2 = 14i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb7a4u64) }
        0x102cb7a4 => { vm.decode(0x40934fb5u32); let d = 5i16; let s = 5i16; let s2 = 12i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb7a8u64) }
        0x102cb7a8 => { vm.decode(0x1e254d35u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 26i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb7acu64) }
        0x102cb7ac => { vm.decode(0x44922fb5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb7b0u64) }
        0x102cb7b0 => { vm.decode(0x408219b5u32); let d = 12i16; let s = 0i16; let s2 = 6i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102cb7b4u64) }
        0x102cb7b4 => { vm.decode(0x1788c875u32); let d = 13i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb7b8u64) }
        0x102cb7b8 => { vm.decode(0x5341a835u32); let d = 13i16; let s = 6i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb7bcu64) }
        0x102cb7bc => { vm.decode(0x4590d311u32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb7c0u64) }
        0x102cb7c0 => { vm.decode(0x55926fb5u32); let d = 13i16; let s = 13i16; let s2 = 14i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb7c4u64) }
        0x102cb7c4 => { vm.decode(0x4924fb5u32); let d = 4i16; let s = 4i16; let s2 = 13i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb7c8u64) }
        0x102cb7c8 => { vm.decode(0x4a24fb5u32); let d = 4i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb7ccu64) }
        0x102cb7cc => { vm.decode(0xca50d35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 22i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb7d0u64) }
        0x102cb7d0 => { vm.decode(0x930fb5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb7d4u64) }
        0x102cb7d4 => { vm.decode(0x64a10fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb7d8u64) }
        0x102cb7d8 => { vm.decode(0x510119b5u32); let d = 11i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102cb7dcu64) }
        0x102cb7dc => { vm.decode(0x13804875u32); let d = 12i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb7e0u64) }
        0x102cb7e0 => { vm.decode(0x45804fb5u32); let d = 9i16; let s = 4i16; let s2 = 9i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb7e4u64) }
        0x102cb7e4 => { vm.decode(0x30d7062u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cb7e8u64) }
        0x102cb7e8 => { vm.decode(0x3009a835u32); let d = 3i16; let s = 5i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb7ecu64) }
        0x102cb7ec => { vm.decode(0xd90cb11u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb7f0u64) }
        0x102cb7f0 => { vm.decode(0x64a10fb5u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb7f4u64) }
        0x102cb7f4 => { vm.decode(0x24202fb5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb7f8u64) }
        0x102cb7f8 => { vm.decode(0x20206fb5u32); let d = 2i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb7fcu64) }
        0x102cb7fc => { vm.decode(0x28118d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 17i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb800u64) }
        0x102cb800 => { vm.decode(0x20a00fb5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb804u64) }
        0x102cb804 => { vm.decode(0x41489875u32); let d = 3i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb808u64) }
        0x102cb808 => { vm.decode(0x31f3462u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 796i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cb80cu64) }
        0x102cb80c => { vm.decode(0x400d2835u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cb810u64) }
        0x102cb810 => { vm.decode(0x4322fb5u32); let d = 4i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb814u64) }
        0x102cb814 => { vm.decode(0x25d3062u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cb818u64) }
        0x102cb818 => { vm.decode(0x5944311u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cb81cu64) }
        0x102cb81c => { vm.decode(0x64a00fb5u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb820u64) }
        0x102cb820 => { vm.decode(0x44210fb5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb824u64) }
        0x102cb824 => { vm.decode(0x40310fb5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb828u64) }
        0x102cb828 => { vm.decode(0x1e00cd35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 11i64;
        let x = vm.ld(s) as u32;
        let v = x.rotate_right((imm as u32) & 31) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb82cu64) }
        0x102cb82c => { vm.decode(0x44002fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb830u64) }
        0x102cb830 => { vm.decode(0x40014fb5u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cb834u64) }
        0x102cb834 => { vm.decode(0x30d3462u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 788i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cb838u64) }
        0x102cb838 => { vm.decode(0x408e6befu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cb83cu64) }
        0x102cb83c => { vm.decode(0x628243efu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cb840u64) }
        0x102cb840 => { vm.decode(0x641d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cb844u64) }
        0x102cb844 => { vm.decode(0x193094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cb848u64) }
        0x102cb848 => { vm.decode(0x640d8519u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cb84cu64) }
        0x102cb84c => { vm.decode(0x192894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 232i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cb850u64) }
        0x102cb850 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cb854u64) }
        0x102cb854 => { vm.decode(0xcc40e059u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -23786i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cb858u64) }
        0x102cb858 => { vm.decode(0x1b2014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cb85cu64) }
        0x102cb85c => { vm.decode(0x12109019u32); let d = 23i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cb860u64) }
        0x102cb860 => { vm.decode(0x640d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cb864u64) }
        0x102cb864 => { vm.decode(0x1b381430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cb868u64) }
        0x102cb868 => { vm.decode(0x22028000u32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102cb86cu64) }
        0x102cb86c => { vm.decode(0x40c82a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 185i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cb870u64).wrapping_add((imm * 4) as u64) } else { 0x102cb870u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cb870u64) }
        0x102cb870 => { vm.decode(0x4100236u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cb874u64) }
        0x102cb874 => { vm.decode(0x84a0859du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 288464i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102cb878u64) }
        0x102cb878 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cb87cu64) } }
        0x102cb87c => { vm.decode(0x100039b5u32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let v = !(vm.ld(imm as i16) | vm.ld(s));
        vm.st(d, v);
        Ok(0x102cb880u64) }
        0x102cca50 => { vm.decode(0x6a8a636fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cca54u64) }
        0x102cca54 => { vm.decode(0xb0027u32); let d = 0i16; let s = 20i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cca58u64).wrapping_add((imm * 4) as u64) } else { 0x102cca58u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cca58u64) }
        0x102cca58 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cca5cu64) } }
        0x102cca5c => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cca60u64) }
        0x102cca98 => { vm.decode(0x4a8063afu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cca9cu64) }
        0x102cca9c => { vm.decode(0x2804402fu32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccaa0u64) }
        0x102ccaa0 => { vm.decode(0x10408019u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ccaa4u64) }
        0x102ccaa4 => { vm.decode(0x9b211430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ccaa8u64) }
        0x102ccaa8 => { vm.decode(0xa88436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccaacu64) }
        0x102ccaac => { vm.decode(0xb1915b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ccab0u64) }
        0x102ccab0 => { vm.decode(0x9b1154b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ccab4u64) }
        0x102ccab4 => { vm.decode(0x1b0914b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ccab8u64) }
        0x102ccab8 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccabcu64) }
        0x102ccabc => { vm.decode(0x488c43afu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccac0u64) }
        0x102ccac0 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ccac4u64) }
        0x102ccac4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102ccac8u64 + 4);
        vm.wr(37, 0x102ccac8u64 + 4);
        Ok(0x102ccac8u64) }
        0x102ccac8 => { vm.decode(0x661d8419u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ccaccu64) }
        0x102ccacc => { vm.decode(0x2200406fu32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccad0u64) }
        0x102ccad0 => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ccad4u64) }
        0x102ccad4 => { vm.decode(0x9b2114b0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ccad8u64) }
        0x102ccad8 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102ccadcu64) }
        0x102ccadc => { vm.decode(0x8b315570u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ccae0u64) }
        0x102ccae0 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ccae4u64) }
        0x102ccae4 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccae8u64) }
        0x102ccae8 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ccaecu64) }
        0x102ccaec => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102ccaf0u64 + 4);
        vm.wr(37, 0x102ccaf0u64 + 4);
        Ok(0x102ccaf0u64) }
        0x102ccaf0 => { vm.decode(0x100036u32); let d = 16i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102ccaf4u64) }
        0x102ccaf4 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccaf8u64) }
        0x102ccaf8 => { vm.decode(0xdc44c0d9u32); let d = 17i16; let s = 1i16; let s2 = -1i16; let imm = -23781i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ccafcu64) }
        0x102ccafc => { vm.decode(0x10101019u32); let d = 18i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ccb00u64) }
        0x102ccb00 => { vm.decode(0x740d9419u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ccb04u64) }
        0x102ccb04 => { vm.decode(0x4000446u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102ccb08u64) }
        0x102ccb08 => { vm.decode(0x2200031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102ccb0cu64).wrapping_add((imm * 4) as u64) } else { 0x102ccb0cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ccb0cu64) }
        0x102ccb0c => { vm.decode(0x6100036u32); let d = 20i16; let s = 0i16; let s2 = -1i16; let imm = 32i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102ccb10u64) }
        0x102ccb10 => { vm.decode(0x2920c19du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 288876i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102ccb14u64) }
        0x102ccb18 => { vm.decode(0x8624735u32); let d = 1i16; let s = 4i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shl(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102ccb1cu64) }
        0x102ccb1c => { vm.decode(0x18019b5u32); let d = 4i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102ccb20u64) }
        0x102ccb20 => { vm.decode(0x317f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ccb24u64) }
        0x102ccb24 => { vm.decode(0x74304675u32); let d = 3i16; let s = 3i16; let s2 = 22i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_shr(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102ccb28u64) }
        0x102ccb28 => { vm.decode(0x441017f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ccb2cu64) }
        0x102ccb2c => { vm.decode(0x89875u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ccb30u64) }
        0x102ccb30 => { vm.decode(0x8b21a4f5u32); let d = 2i16; let s = 21i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102ccb34u64) }
        0x102ccb34 => { vm.decode(0x5000022u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ccb38u64) }
        0x102ccb38 => { vm.decode(0x8900036u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102ccb3cu64) }
        0x102ccb3c => { vm.decode(0x628243efu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccb40u64) }
        0x102ccb40 => { vm.decode(0x4000446u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102ccb44u64) }
        0x102ccb44 => { vm.decode(0x4048227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 41i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102ccb48u64).wrapping_add((imm * 4) as u64) } else { 0x102ccb48u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ccb48u64) }
        0x102ccb48 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102ccb4cu64) } }
        0x102ccb4c => { vm.decode(0x2084c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 864i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccb50u64) }
        0x102ccb50 => { vm.decode(0xa14004f5u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102ccb54u64) }
        0x102ccb54 => { vm.decode(0xa8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccb58u64) }
        0x102ccb58 => { vm.decode(0x82c202fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccb5cu64) }
        0x102ccb5c => { vm.decode(0x8b191530u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ccb60u64) }
        0x102ccb60 => { vm.decode(0xb091530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ccb64u64) }
        0x102ccb64 => { vm.decode(0x1b111430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ccb68u64) }
        0x102ccb68 => { vm.decode(0x40020bb5u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102ccb6cu64) }
        0x102ccb6c => { vm.decode(0x44104875u32); let d = 2i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ccb70u64) }
        0x102ccb70 => { vm.decode(0xc040ad75u32); let d = 21i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102ccb74u64) }
        0x102ccb74 => { vm.decode(0x836004f5u32); let d = 1i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102ccb78u64) }
        0x102ccb78 => { vm.decode(0x54000111u32); let d = 22i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ccb7cu64) }
        0x102ccb7c => { vm.decode(0xa8243afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccb80u64) }
        0x102ccb80 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccb84u64) }
        0x102ccb84 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102ccb88u64 + 4);
        vm.wr(37, 0x102ccb88u64 + 4);
        Ok(0x102ccb88u64) }
        0x102ccb88 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ccb8cu64) }
        0x102ccb8c => { vm.decode(0x4f144211u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 2592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ccb90u64) }
        0x102ccb90 => { vm.decode(0x28c40b35u32); let d = 4i16; let s = 3i16; let s2 = 20i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102ccb94u64) }
        0x102ccb94 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102ccb98u64) }
        0x102ccb98 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102ccb9cu64) }
        0x102ccb9c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102ccba0u64).wrapping_add((imm * 4) as u64) } else { 0x102ccba0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ccba0u64) }
        0x102ccba0 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102ccba4u64) }
        0x102ccba4 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102ccba8u64) }
        0x102ccba8 => { vm.decode(0x38bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102ccbacu64) }
        0x102ccbac => { vm.decode(0x80004118u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102ccbb0u64) }
        0x102ccbb0 => { vm.decode(0x21204375u32); let d = 1i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102ccbb4u64) }
        0x102ccbb4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102ccbb8u64).wrapping_add((imm * 4) as u64) } else { 0x102ccbb8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ccbb8u64) }
        0x102ccbb8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102ccbbcu64) } }
        0x102ccbbc => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102ccbc0u64) }
        0x102ccbc0 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102ccbc4u64) }
        0x102ccbc4 => { vm.decode(0x739c34a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -44i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102ccbc8u64).wrapping_add((imm * 4) as u64) } else { 0x102ccbc8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ccbc8u64) }
        0x102ccbc8 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102ccbccu64) }
        0x102ccbcc => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102ccbd0u64) }
        0x102ccbd0 => { vm.decode(0xa28bb5u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102ccbd4u64) }
        0x102ccbd4 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102ccbd8u64) }
        0x102ccbd8 => { vm.decode(0x21280375u32); let d = 1i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = ((vm.ld(s2) as i64) < (vm.ld(s) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102ccbdcu64) }
        0x102ccbdc => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102ccbe0u64).wrapping_add((imm * 4) as u64) } else { 0x102ccbe0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ccbe0u64) }
        0x102ccbe0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102ccbe4u64) } }
        0x102ccbe4 => { vm.decode(0x8c80959du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 288664i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102ccbe8u64) }
        0x102ccbe8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102ccbecu64) } }
        0x102ccbec => { vm.decode(0xd10e0d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1292i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ccbf0u64) }
        0x102ccbf0 => { vm.decode(0x45106a91u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 788i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ccbf4u64) }
        0x102ccbf4 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ccbf8u64) }
        0x102ccbf8 => { vm.decode(0x4510e051u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ccbfcu64) }
        0x102ccbfc => { vm.decode(0x4d106a11u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ccc00u64) }
        0x102ccc00 => { vm.decode(0x204c0835u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ccc04u64) }
        0x102ccc04 => { vm.decode(0x4d106851u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ccc08u64) }
        0x102ccc08 => { vm.decode(0x510eb11u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ccc0cu64) }
        0x102ccc0c => { vm.decode(0x20d7062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ccc10u64) }
        0x102ccc10 => { vm.decode(0x30d3462u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 788i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ccc14u64) }
        0x102ccc14 => { vm.decode(0x40052835u32); let d = 1i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ccc18u64) }
        0x102ccc18 => { vm.decode(0x31d3062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ccc1cu64) }
        0x102ccc1c => { vm.decode(0xd1068d1u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1300i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ccc20u64) }
        0x102ccc20 => { vm.decode(0x4d10ea91u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 796i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ccc24u64) }
        0x102ccc24 => { vm.decode(0x60402835u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ccc28u64) }
        0x102ccc28 => { vm.decode(0x31d3462u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 796i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ccc2cu64) }
        0x102ccc2c => { vm.decode(0x2888636fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccc30u64) }
        0x102ccc30 => { vm.decode(0x20200833u32); let d = 2i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102ccc34u64) }
        0x102ccc34 => { vm.decode(0xd106a11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ccc38u64) }
        0x102ccc38 => { vm.decode(0x40900d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102ccc3cu64) }
        0x102ccc3c => { vm.decode(0x80220833u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102ccc40u64) }
        0x102ccc40 => { vm.decode(0x1019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ccc44u64) }
        0x102ccc44 => { vm.decode(0xc2a08d23u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 786i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 2)?;
        let v = u16::from_le_bytes(b.try_into().unwrap()) as u64;
        vm.st(d, v);
        Ok(0x102ccc48u64) }
        0x102ccc48 => { vm.decode(0x2208b3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102ccc4cu64) }
        0x102ccc4c => { vm.decode(0x714132beu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 787i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102ccc50u64) }
        0x102ccc50 => { vm.decode(0x802208b3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102ccc54u64) }
        0x102ccc54 => { vm.decode(0xd106a91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 788i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ccc58u64) }
        0x102ccc58 => { vm.decode(0x220c33u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 4i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102ccc5cu64) }
        0x102ccc5c => { vm.decode(0xd106a91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 788i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ccc60u64) }
        0x102ccc60 => { vm.decode(0x40900d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102ccc64u64) }
        0x102ccc64 => { vm.decode(0x80220c33u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 5i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102ccc68u64) }
        0x102ccc68 => { vm.decode(0xc2a08da3u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 790i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 2)?;
        let v = u16::from_le_bytes(b.try_into().unwrap()) as u64;
        vm.st(d, v);
        Ok(0x102ccc6cu64) }
        0x102ccc6c => { vm.decode(0x220cb3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 6i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102ccc70u64) }
        0x102ccc70 => { vm.decode(0x714132feu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 791i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102ccc74u64) }
        0x102ccc74 => { vm.decode(0x80220cb3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 7i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102ccc78u64) }
        0x102ccc78 => { vm.decode(0xd10ea11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ccc7cu64) }
        0x102ccc7c => { vm.decode(0x220933u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102ccc80u64) }
        0x102ccc80 => { vm.decode(0xd10ea11u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ccc84u64) }
        0x102ccc84 => { vm.decode(0x40900d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102ccc88u64) }
        0x102ccc88 => { vm.decode(0x80220933u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 9i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102ccc8cu64) }
        0x102ccc8c => { vm.decode(0xc2a18d23u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 794i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 2)?;
        let v = u16::from_le_bytes(b.try_into().unwrap()) as u64;
        vm.st(d, v);
        Ok(0x102ccc90u64) }
        0x102ccc90 => { vm.decode(0x2209b3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 10i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102ccc94u64) }
        0x102ccc94 => { vm.decode(0x71413abeu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 795i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102ccc98u64) }
        0x102ccc98 => { vm.decode(0x802209b3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 11i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102ccc9cu64) }
        0x102ccc9c => { vm.decode(0xd10ea91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 796i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ccca0u64) }
        0x102ccca0 => { vm.decode(0x220d33u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102ccca4u64) }
        0x102ccca4 => { vm.decode(0xd10ea91u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 796i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ccca8u64) }
        0x102ccca8 => { vm.decode(0x40900d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102cccacu64) }
        0x102cccac => { vm.decode(0x80220d33u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 13i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102cccb0u64) }
        0x102cccb0 => { vm.decode(0xc2a18da3u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 798i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 2)?;
        let v = u16::from_le_bytes(b.try_into().unwrap()) as u64;
        vm.st(d, v);
        Ok(0x102cccb4u64) }
        0x102cccb4 => { vm.decode(0x220db3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 14i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102cccb8u64) }
        0x102cccb8 => { vm.decode(0x71413afeu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 799i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102cccbcu64) }
        0x102cccbc => { vm.decode(0x80220db3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 15i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102cccc0u64) }
        0x102cccc0 => { vm.decode(0x109119u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 96i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cccc4u64) }
        0x102cccc4 => { vm.decode(0x408c436fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cccc8u64) }
        0x102cccc8 => { vm.decode(0x8a0c027u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = 5i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cccccu64).wrapping_add((imm * 4) as u64) } else { 0x102cccccu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cccccu64) }
        0x102ccccc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cccd0u64) } }
        0x102cccd0 => { vm.decode(0x624124f5u32); let d = 1i16; let s = 2i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cccd4u64) }
        0x102cccd4 => { vm.decode(0x801099u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cccd8u64) }
        0x102cccd8 => { vm.decode(0xadfb8df1u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = -3i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102cccdcu64).wrapping_add((imm * 4) as u64) } else { 0x102cccdcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cccdcu64) }
        0x102cccdc => { vm.decode(0x833u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102ccce0u64) }
        0x102ccce0 => { vm.decode(0x4a8063afu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccce4u64) }
        0x102ccce4 => { vm.decode(0x2200602fu32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccce8u64) }
        0x102ccce8 => { vm.decode(0x19191cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cccecu64) }
        0x102cccec => { vm.decode(0x705d9459u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2360i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cccf0u64) }
        0x102cccf0 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cccf4u64) }
        0x102cccf4 => { vm.decode(0x688443afu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cccf8u64) }
        0x102cccf8 => { vm.decode(0xe108875u32); let d = 25i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cccfcu64) }
        0x102cccfc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102ccd00u64 + 4);
        vm.wr(37, 0x102ccd00u64 + 4);
        Ok(0x102ccd00u64) }
        0x102ccd00 => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ccd04u64) }
        0x102ccd04 => { vm.decode(0x2000602fu32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccd08u64) }
        0x102ccd08 => { vm.decode(0x19191cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ccd0cu64) }
        0x102ccd0c => { vm.decode(0x17906011u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ccd10u64) }
        0x102ccd10 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ccd14u64) }
        0x102ccd14 => { vm.decode(0xe108875u32); let d = 25i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ccd18u64) }
        0x102ccd18 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102ccd1cu64 + 4);
        vm.wr(37, 0x102ccd1cu64 + 4);
        Ok(0x102ccd1cu64) }
        0x102ccd1c => { vm.decode(0x4909875u32); let d = 5i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ccd20u64) }
        0x102ccd20 => { vm.decode(0x8102036u32); let d = 17i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102ccd24u64) }
        0x102ccd24 => { vm.decode(0xf906011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ccd28u64) }
        0x102ccd28 => { vm.decode(0xc3282031u32); let d = 17i16; let s = 1i16; let s2 = -1i16; let imm = 58i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102ccd2cu64).wrapping_add((imm * 4) as u64) } else { 0x102ccd2cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ccd2cu64) }
        0x102ccd2c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102ccd30u64) } }
        0x102ccd30 => { vm.decode(0x200830eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 539099136i64;
        vm.st(d, imm as u64);
        Ok(0x102ccd34u64) }
        0x102ccd34 => { vm.decode(0x20010198u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1056i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102ccd38u64) }
        0x102ccd38 => { vm.decode(0x31d1062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ccd3cu64) }
        0x102ccd3c => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102ccd40u64) }
        0x102ccd40 => { vm.decode(0x2888636fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccd44u64) }
        0x102ccd44 => { vm.decode(0x6a80632fu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccd48u64) }
        0x102ccd48 => { vm.decode(0x428c636fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccd4cu64) }
        0x102ccd4c => { vm.decode(0x2000506u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102ccd50u64) }
        0x102ccd50 => { vm.decode(0x82002071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 14i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102ccd54u64).wrapping_add((imm * 4) as u64) } else { 0x102ccd54u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ccd54u64) }
        0x102ccd54 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102ccd58u64) } }
        0x102ccd58 => { vm.decode(0x8a0c19du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 289352i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102ccd5cu64) }
        0x102ccd60 => { vm.decode(0x2008023eu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102ccd64u64) }
        0x102ccd64 => { vm.decode(0x4368bb5u32); let d = 5i16; let s = 4i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102ccd68u64) }
        0x102ccd68 => { vm.decode(0x60012835u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ccd6cu64) }
        0x102ccd6c => { vm.decode(0x34ad35u32); let d = 5i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102ccd70u64) }
        0x102ccd70 => { vm.decode(0x6000a835u32); let d = 1i16; let s = 5i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ccd74u64) }
        0x102ccd74 => { vm.decode(0x40012835u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ccd78u64) }
        0x102ccd78 => { vm.decode(0x31d1062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ccd7cu64) }
        0x102ccd7c => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102ccd80u64) }
        0x102ccd80 => { vm.decode(0x2000506u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102ccd84u64) }
        0x102ccd84 => { vm.decode(0xc0027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102ccd88u64).wrapping_add((imm * 4) as u64) } else { 0x102ccd88u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ccd88u64) }
        0x102ccd88 => { vm.decode(0x40809099u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ccd8cu64) }
        0x102ccd8c => { vm.decode(0x80412du32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102ccd90u64) }
        0x102ccd90 => { vm.decode(0x771cbca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -13i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102ccd94u64).wrapping_add((imm * 4) as u64) } else { 0x102ccd94u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ccd94u64) }
        0x102ccd94 => { vm.decode(0x510e311u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ccd98u64) }
        0x102ccd98 => { vm.decode(0x2008023eu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102ccd9cu64) }
        0x102ccd9c => { vm.decode(0x426cbb5u32); let d = 5i16; let s = 4i16; let s2 = -1i16; let imm = 11i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102ccda0u64) }
        0x102ccda0 => { vm.decode(0x408c875u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ccda4u64) }
        0x102ccda4 => { vm.decode(0x1014ad35u32); let d = 5i16; let s = 4i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102ccda8u64) }
        0x102ccda8 => { vm.decode(0x6000a835u32); let d = 1i16; let s = 5i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ccdacu64) }
        0x102ccdac => { vm.decode(0x2000a835u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102ccdb0u64) }
        0x102ccdb0 => { vm.decode(0x110009b5u32); let d = 1i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102ccdb4u64) }
        0x102ccdb4 => { vm.decode(0x31d1062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ccdb8u64) }
        0x102ccdb8 => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102ccdbcu64) }
        0x102ccdbc => { vm.decode(0x2000506u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102ccdc0u64) }
        0x102ccdc0 => { vm.decode(0x27f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -14i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102ccdc4u64).wrapping_add((imm * 4) as u64) } else { 0x102ccdc4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ccdc4u64) }
        0x102ccdc4 => { vm.decode(0x40809099u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ccdc8u64) }
        0x102ccdc8 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccdccu64) }
        0x102ccdcc => { vm.decode(0x2000016fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccdd0u64) }
        0x102ccdd0 => { vm.decode(0x3019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ccdd4u64) }
        0x102ccdd4 => { vm.decode(0x9211db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ccdd8u64) }
        0x102ccdd8 => { vm.decode(0x640d1059u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ccddcu64) }
        0x102ccddc => { vm.decode(0x9391d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ccde0u64) }
        0x102ccde0 => { vm.decode(0x4880636fu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccde4u64) }
        0x102ccde4 => { vm.decode(0x20401019u32); let d = 2i16; let s = 16i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ccde8u64) }
        0x102ccde8 => { vm.decode(0x9311d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ccdecu64) }
        0x102ccdec => { vm.decode(0x19291c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ccdf0u64) }
        0x102ccdf0 => { vm.decode(0x886632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccdf4u64) }
        0x102ccdf4 => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccdf8u64) }
        0x102ccdf8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102ccdfcu64 + 4);
        vm.wr(37, 0x102ccdfcu64 + 4);
        Ok(0x102ccdfcu64) }
        0x102ccdfc => { vm.decode(0x620d8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2376i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cce00u64) }
        0x102cce00 => { vm.decode(0x40100875u32); let d = 2i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cce04u64) }
        0x102cce04 => { vm.decode(0x2140003eu32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102cce08u64) }
        0x102cce08 => { vm.decode(0xc2ca812du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 251i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102cce0cu64) }
        0x102cce0c => { vm.decode(0xa980d19du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 289708i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102cce10u64) }
        0x102cce10 => { vm.decode(0x2022033u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102cce14u64) }
        0x102cce14 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cce18u64) }
        0x102ccf2c => { vm.decode(0x1305778eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -4063232i64;
        vm.st(d, imm as u64);
        Ok(0x102ccf30u64) }
        0x102ccf30 => { vm.decode(0x45d2518u32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = 39592i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102ccf34u64) }
        0x102ccf34 => { vm.decode(0x214104f5u32); let d = 1i16; let s = 16i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102ccf38u64) }
        0x102ccf38 => { vm.decode(0x2000023eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102ccf3cu64) }
        0x102ccf3c => { vm.decode(0x80812du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102ccf40u64) }
        0x102ccf40 => { vm.decode(0x2886432fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccf44u64) }
        0x102ccf44 => { vm.decode(0x83202031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 58i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102ccf48u64).wrapping_add((imm * 4) as u64) } else { 0x102ccf48u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ccf48u64) }
        0x102ccf48 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102ccf4cu64) } }
        0x102ccf4c => { vm.decode(0x34104f5u32); let d = 1i16; let s = 16i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102ccf50u64) }
        0x102ccf50 => { vm.decode(0xc000011u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ccf54u64) }
        0x102ccf54 => { vm.decode(0x3022031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 35i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102ccf58u64).wrapping_add((imm * 4) as u64) } else { 0x102ccf58u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ccf58u64) }
        0x102ccf58 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102ccf5cu64) } }
        0x102ccf5c => { vm.decode(0x2084632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccf60u64) }
        0x102ccf60 => { vm.decode(0xa8c632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccf64u64) }
        0x102ccf64 => { vm.decode(0x600204f5u32); let d = 4i16; let s = 2i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102ccf68u64) }
        0x102ccf68 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccf6cu64) }
        0x102ccf6c => { vm.decode(0x2000216fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccf70u64) }
        0x102ccf70 => { vm.decode(0x640d1059u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ccf74u64) }
        0x102ccf74 => { vm.decode(0x9111db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ccf78u64) }
        0x102ccf78 => { vm.decode(0x19091cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102ccf7cu64) }
        0x102ccf7c => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccf80u64) }
        0x102ccf80 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102ccf84u64 + 4);
        vm.wr(37, 0x102ccf84u64 + 4);
        Ok(0x102ccf84u64) }
        0x102ccf84 => { vm.decode(0x621d8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2344i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102ccf88u64) }
        0x102ccf88 => { vm.decode(0x2080e3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccf8cu64) }
        0x102ccf8c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccf90u64) }
        0x102ccf90 => { vm.decode(0xa0c816fu32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccf94u64) }
        0x102ccf94 => { vm.decode(0x280ca16fu32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccf98u64) }
        0x102ccf98 => { vm.decode(0xc280e35u32); let d = 1i16; let s = 3i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s) < vm.ld(s2)) as u64;
        vm.st(d, v);
        Ok(0x102ccf9cu64) }
        0x102ccf9c => { vm.decode(0x40a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 10i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102ccfa0u64).wrapping_add((imm * 4) as u64) } else { 0x102ccfa0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ccfa0u64) }
        0x102ccfa0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102ccfa4u64) } }
        0x102ccfa4 => { vm.decode(0x1305778eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -4063232i64;
        vm.st(d, imm as u64);
        Ok(0x102ccfa8u64) }
        0x102ccfa8 => { vm.decode(0x205d2518u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 39592i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102ccfacu64) }
        0x102ccfac => { vm.decode(0x824104f5u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102ccfb0u64) }
        0x102ccfb0 => { vm.decode(0x4320b75u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_sub(vm.ld(s2));
        vm.st(d, v);
        Ok(0x102ccfb4u64) }
        0x102ccfb4 => { vm.decode(0x410bb5u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102ccfb8u64) }
        0x102ccfb8 => { vm.decode(0x2004022u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102ccfbcu64) }
        0x102ccfbc => { vm.decode(0x2886432fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102ccfc0u64) }
        0x102ccfc0 => { vm.decode(0x2520c19du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 289892i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102ccfc4u64) }
        0x102ccfc4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102ccfc8u64) } }
        0x102ccfc8 => { vm.decode(0x1305778eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -4063232i64;
        vm.st(d, imm as u64);
        Ok(0x102ccfccu64) }
        0x102ccfe4 => { vm.decode(0x1305778eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -4063232i64;
        vm.st(d, imm as u64);
        Ok(0x102ccfe8u64) }
        0x102ccfe8 => { vm.decode(0x805d2518u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 39592i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102ccfecu64) }
        0x102ccfec => { vm.decode(0x424104f5u32); let d = 1i16; let s = 2i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102ccff0u64) }
        0x102ccff0 => { vm.decode(0xc000011u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102ccff4u64) }
        0x102ccff4 => { vm.decode(0x2636u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102ccff8u64) }
        0x102ccff8 => { vm.decode(0x20a8071u32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102ccffcu64).wrapping_add((imm * 4) as u64) } else { 0x102ccffcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102ccffcu64) }
        0x102ccffc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cd000u64) } }
        0x102cd000 => { vm.decode(0x604024f5u32); let d = 1i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cd004u64) }
        0x102cd010 => { vm.decode(0x604024f5u32); let d = 1i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cd014u64) }
        0x102cd014 => { vm.decode(0x2000022u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cd018u64) }
        0x102cd018 => { vm.decode(0x1305778eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -4063232i64;
        vm.st(d, imm as u64);
        Ok(0x102cd01cu64) }
        0x102cd01c => { vm.decode(0x205d2518u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 39592i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102cd020u64) }
        0x102cd020 => { vm.decode(0xa04104f5u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cd024u64) }
        0x102cd024 => { vm.decode(0x8000236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cd028u64) }
        0x102cd028 => { vm.decode(0x20000833u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102cd02cu64) }
        0x102cd02c => { vm.decode(0x888636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd030u64) }
        0x102cd030 => { vm.decode(0x2284436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd034u64) }
        0x102cd034 => { vm.decode(0x2a8c632fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd038u64) }
        0x102cd038 => { vm.decode(0xe00604f5u32); let d = 20i16; let s = 3i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cd03cu64) }
        0x102cd03c => { vm.decode(0x2880432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd040u64) }
        0x102cd040 => { vm.decode(0xe80404f5u32); let d = 18i16; let s = 3i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cd044u64) }
        0x102cd044 => { vm.decode(0x2880632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd048u64) }
        0x102cd048 => { vm.decode(0xe84404f5u32); let d = 19i16; let s = 3i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cd04cu64) }
        0x102cd04c => { vm.decode(0x2a80432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd050u64) }
        0x102cd050 => { vm.decode(0xe00204f5u32); let d = 4i16; let s = 3i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cd054u64) }
        0x102cd054 => { vm.decode(0x2084636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd058u64) }
        0x102cd058 => { vm.decode(0xe04404f5u32); let d = 17i16; let s = 3i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cd05cu64) }
        0x102cd05c => { vm.decode(0x214024f5u32); let d = 1i16; let s = 16i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cd060u64) }
        0x102cd060 => { vm.decode(0xc000011u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cd064u64) }
        0x102cd064 => { vm.decode(0x809875u32); let d = 5i16; let s = 0i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd068u64) }
        0x102cd068 => { vm.decode(0x308003eu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102cd06cu64) }
        0x102cd06c => { vm.decode(0x8002236u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cd070u64) }
        0x102cd070 => { vm.decode(0x2088071u32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 4i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102cd074u64).wrapping_add((imm * 4) as u64) } else { 0x102cd074u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cd074u64) }
        0x102cd074 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cd078u64) } }
        0x102cd078 => { vm.decode(0xd28a412du32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 223i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102cd07cu64) }
        0x102cd084 => { vm.decode(0x28010018u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102cd088u64) }
        0x102cd088 => { vm.decode(0x40022833u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102cd08cu64) }
        0x102cd08c => { vm.decode(0x6a8a63afu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd090u64) }
        0x102cd090 => { vm.decode(0x22a0412fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd094u64) }
        0x102cd094 => { vm.decode(0x19191c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd098u64) }
        0x102cd098 => { vm.decode(0x488e43afu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd09cu64) }
        0x102cd09c => { vm.decode(0xa10d875u32); let d = 25i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd0a0u64) }
        0x102cd0a0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cd0a4u64 + 4);
        vm.wr(37, 0x102cd0a4u64 + 4);
        Ok(0x102cd0a4u64) }
        0x102cd0a4 => { vm.decode(0x624d8459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2328i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cd0a8u64) }
        0x102cd0a8 => { vm.decode(0x20c4c32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd0acu64) }
        0x102cd0ac => { vm.decode(0x4000012fu32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd0b0u64) }
        0x102cd0b0 => { vm.decode(0x62098459u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2304i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cd0b4u64) }
        0x102cd0b4 => { vm.decode(0x760d8059u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cd0b8u64) }
        0x102cd0b8 => { vm.decode(0x22a4412fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd0bcu64) }
        0x102cd0bc => { vm.decode(0x99111c30u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd0c0u64) }
        0x102cd0c0 => { vm.decode(0x99095c30u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd0c4u64) }
        0x102cd0c4 => { vm.decode(0x19011c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd0c8u64) }
        0x102cd0c8 => { vm.decode(0xa10d875u32); let d = 25i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd0ccu64) }
        0x102cd0cc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cd0d0u64 + 4);
        vm.wr(37, 0x102cd0d0u64 + 4);
        Ok(0x102cd0d0u64) }
        0x102cd0d0 => { vm.decode(0x4901875u32); let d = 4i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd0d4u64) }
        0x102cd0d4 => { vm.decode(0x621b8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2272i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cd0d8u64) }
        0x102cd0d8 => { vm.decode(0x20a4612fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd0dcu64) }
        0x102cd0dc => { vm.decode(0x9931d4b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd0e0u64) }
        0x102cd0e0 => { vm.decode(0x892994b0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd0e4u64) }
        0x102cd0e4 => { vm.decode(0x192194b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd0e8u64) }
        0x102cd0e8 => { vm.decode(0xa10d875u32); let d = 25i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd0ecu64) }
        0x102cd0ec => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cd0f0u64 + 4);
        vm.wr(37, 0x102cd0f0u64 + 4);
        Ok(0x102cd0f0u64) }
        0x102cd0f0 => { vm.decode(0x901875u32); let d = 4i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd0f4u64) }
        0x102cd0f4 => { vm.decode(0x624b8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2256i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cd0f8u64) }
        0x102cd0f8 => { vm.decode(0x22a4612fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd0fcu64) }
        0x102cd0fc => { vm.decode(0x9939d430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd100u64) }
        0x102cd100 => { vm.decode(0x19319430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd104u64) }
        0x102cd104 => { vm.decode(0x179cc811u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cd108u64) }
        0x102cd108 => { vm.decode(0xa10d875u32); let d = 25i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd10cu64) }
        0x102cd10c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cd110u64 + 4);
        vm.wr(37, 0x102cd110u64 + 4);
        Ok(0x102cd110u64) }
        0x102cd110 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd114u64) }
        0x102cd114 => { vm.decode(0x20044021u32); let d = -1i16; let s = 16i16; let s2 = -1i16; let imm = 5i64;
        vm.wr(36, if (vm.ld(s) as i64) < 1 { (0x102cd118u64).wrapping_add((imm * 4) as u64) } else { 0x102cd118u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cd118u64) }
        0x102cd118 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cd11cu64) } }
        0x102cd11c => { vm.decode(0x888636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd120u64) }
        0x102cd120 => { vm.decode(0x310800beu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 19i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102cd124u64) }
        0x102cd124 => { vm.decode(0x200008a0u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) ^ imm as u64;
        vm.st(d, v);
        Ok(0x102cd128u64) }
        0x102cd128 => { vm.decode(0x802220b3u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 19i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102cd12cu64) }
        0x102cd12c => { vm.decode(0x48c043afu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd130u64) }
        0x102cd130 => { vm.decode(0x68c063afu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 2696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd134u64) }
        0x102cd134 => { vm.decode(0x4ac843afu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 2704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd138u64) }
        0x102cd138 => { vm.decode(0x6ac863afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 2712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd13cu64) }
        0x102cd13c => { vm.decode(0x48c643afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 2720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd140u64) }
        0x102cd140 => { vm.decode(0x68c663afu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 2728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd144u64) }
        0x102cd144 => { vm.decode(0x4ace43afu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 2736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd148u64) }
        0x102cd148 => { vm.decode(0x6ace63afu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 2744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd14cu64) }
        0x102cd14c => { vm.decode(0x48ca4befu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 2752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd150u64) }
        0x102cd150 => { vm.decode(0x68ca6befu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 2760i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd154u64) }
        0x102cd154 => { vm.decode(0x54104cf5u32); let d = -1i16; let s = -1i16; let s2 = -1i16; let imm = 0i64;
        vm.jump_flag = 2;
        vm.wr(36, vm.slots[31]);
        vm.wr(35, 2);
        Ok(0x102cd158u64) }
        0x102cd158 => { vm.decode(0x774b8519u32); let d = 29i16; let s = 29i16; let s2 = -1i16; let imm = 2768i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cd15cu64) }
        0x102cd15c => { vm.decode(0x0u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102cd160u64) }
        0x102cd160 => { vm.decode(0x208830eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 539164672i64;
        vm.st(d, imm as u64);
        Ok(0x102cd164u64) }
        0x102cd164 => { vm.decode(0x80412158u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 2344i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102cd168u64) }
        0x102cd168 => { vm.decode(0x2088002fu32); let d = 3i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd16cu64) }
        0x102cd16c => { vm.decode(0x2222012fu32); let d = 5i16; let s = 3i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd170u64) }
        0x102cd170 => { vm.decode(0x900041du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 56i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102cd174u64) }
        0x102cd174 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cd178u64) }
        0x102cd178 => { vm.decode(0x2000123eu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102cd17cu64) }
        0x102cd17c => { vm.decode(0x4570bb5u32); let d = 7i16; let s = 2i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cd180u64) }
        0x102cd180 => { vm.decode(0x60412835u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd184u64) }
        0x102cd184 => { vm.decode(0x30118d35u32); let d = 7i16; let s = 2i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102cd188u64) }
        0x102cd188 => { vm.decode(0x6004a835u32); let d = 1i16; let s = 7i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd18cu64) }
        0x102cd18c => { vm.decode(0x40480835u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd190u64) }
        0x102cd190 => { vm.decode(0x42018099u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cd194u64) }
        0x102cd194 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cd198u64) }
        0x102cd198 => { vm.decode(0xc408091u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cd19cu64) }
        0x102cd19c => { vm.decode(0x21088375u32); let d = 1i16; let s = 6i16; let s2 = 1i16; let imm = 0i64;
        let v = ((vm.ld(s) as i64) < (vm.ld(s2) as i64)) as u64;
        vm.st(d, v);
        Ok(0x102cd1a0u64) }
        0x102cd1a0 => { vm.decode(0x840a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 14i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cd1a4u64).wrapping_add((imm * 4) as u64) } else { 0x102cd1a4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cd1a4u64) }
        0x102cd1a4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cd1a8u64) } }
        0x102cd1a8 => { vm.decode(0x80416du32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102cd1acu64) }
        0x102cd1ac => { vm.decode(0x771c3ca7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -14i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cd1b0u64).wrapping_add((imm * 4) as u64) } else { 0x102cd1b0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cd1b0u64) }
        0x102cd1b0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cd1b4u64) } }
        0x102cd1b4 => { vm.decode(0x2000123eu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102cd1b8u64) }
        0x102cd1b8 => { vm.decode(0x574bb5u32); let d = 7i16; let s = 2i16; let s2 = -1i16; let imm = 12i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102cd1bcu64) }
        0x102cd1bc => { vm.decode(0x408d875u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd1c0u64) }
        0x102cd1c0 => { vm.decode(0x30358d35u32); let d = 7i16; let s = 2i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102cd1c4u64) }
        0x102cd1c4 => { vm.decode(0x6004a835u32); let d = 1i16; let s = 7i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd1c8u64) }
        0x102cd1c8 => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd1ccu64) }
        0x102cd1cc => { vm.decode(0x10109b5u32); let d = 2i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102cd1d0u64) }
        0x102cd1d0 => { vm.decode(0x42018099u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cd1d4u64) }
        0x102cd1d4 => { vm.decode(0x900041du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 56i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102cd1d8u64) }
        0x102cd1d8 => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cd1dcu64) }
        0x102cd1dc => { vm.decode(0x54104cf5u32); let d = -1i16; let s = -1i16; let s2 = -1i16; let imm = 0i64;
        vm.jump_flag = 2;
        vm.wr(36, vm.slots[31]);
        vm.wr(35, 2);
        Ok(0x102cd1e0u64) }
        0x102cd1e0 => { vm.decode(0x105022u32); let d = 2i16; let s = 4i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cd1e4u64) }
        0x102cd1e4 => { vm.decode(0x0u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102cd1e8u64) }
        0x102cd1f0 => { vm.decode(0x7b3bcf19u32); let d = 29i16; let s = 29i16; let s2 = -1i16; let imm = -800i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cd1f4u64) }
        0x102cd1f4 => { vm.decode(0x9b185d70u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd1f8u64) }
        0x102cd1f8 => { vm.decode(0x8b105d70u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd1fcu64) }
        0x102cd1fc => { vm.decode(0x9b085d30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd200u64) }
        0x102cd200 => { vm.decode(0x8b005d30u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd204u64) }
        0x102cd204 => { vm.decode(0x9b38d4b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 760i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd208u64) }
        0x102cd208 => { vm.decode(0x8b30d4b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd20cu64) }
        0x102cd20c => { vm.decode(0x9b2895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd210u64) }
        0x102cd210 => { vm.decode(0x8b2095b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd214u64) }
        0x102cd214 => { vm.decode(0x9b389430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd218u64) }
        0x102cd218 => { vm.decode(0x8b309430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd21cu64) }
        0x102cd21c => { vm.decode(0x138df2ceu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -5439488i64;
        vm.st(d, imm as u64);
        Ok(0x102cd220u64) }
        0x102cd220 => { vm.decode(0x24592f58u32); let d = 17i16; let s = 1i16; let s2 = -1i16; let imm = 63912i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102cd224u64) }
        0x102cd224 => { vm.decode(0x88012fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd228u64) }
        0x102cd228 => { vm.decode(0x830404f5u32); let d = 16i16; let s = 2i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cd22cu64) }
        0x102cd22c => { vm.decode(0x286096fu32); let d = 12i16; let s = 5i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd230u64) }
        0x102cd230 => { vm.decode(0x208c296fu32); let d = 11i16; let s = 5i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd234u64) }
        0x102cd234 => { vm.decode(0x8c096fu32); let d = 10i16; let s = 5i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd238u64) }
        0x102cd238 => { vm.decode(0x2280296fu32); let d = 9i16; let s = 5i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd23cu64) }
        0x102cd23c => { vm.decode(0x280096fu32); let d = 8i16; let s = 5i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd240u64) }
        0x102cd240 => { vm.decode(0x2082296fu32); let d = 13i16; let s = 5i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd244u64) }
        0x102cd244 => { vm.decode(0x208a096fu32); let d = 15i16; let s = 5i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd248u64) }
        0x102cd248 => { vm.decode(0x4284292fu32); let d = 24i16; let s = 5i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd24cu64) }
        0x102cd24c => { vm.decode(0x6286012fu32); let d = 21i16; let s = 5i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd250u64) }
        0x102cd250 => { vm.decode(0x8a082fu32); let d = 14i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd254u64) }
        0x102cd254 => { vm.decode(0x2088212fu32); let d = 3i16; let s = 5i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd258u64) }
        0x102cd258 => { vm.decode(0x6084092fu32); let d = 25i16; let s = 5i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd25cu64) }
        0x102cd25c => { vm.decode(0x608e292fu32); let d = 31i16; let s = 5i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd260u64) }
        0x102cd260 => { vm.decode(0x28c216fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd264u64) }
        0x102cd264 => { vm.decode(0x93815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd268u64) }
        0x102cd268 => { vm.decode(0xa8002fu32); let d = 2i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd26cu64) }
        0x102cd26c => { vm.decode(0xa2202fu32); let d = 4i16; let s = 6i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd270u64) }
        0x102cd270 => { vm.decode(0x42a2002fu32); let d = 20i16; let s = 6i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd274u64) }
        0x102cd274 => { vm.decode(0x62a8202fu32); let d = 19i16; let s = 6i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd278u64) }
        0x102cd278 => { vm.decode(0x60ae002fu32); let d = 23i16; let s = 6i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd27cu64) }
        0x102cd27c => { vm.decode(0x40ac202fu32); let d = 18i16; let s = 6i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd280u64) }
        0x102cd280 => { vm.decode(0x893015b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd284u64) }
        0x102cd284 => { vm.decode(0x42ac002fu32); let d = 18i16; let s = 6i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd288u64) }
        0x102cd288 => { vm.decode(0x892815b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd28cu64) }
        0x102cd28c => { vm.decode(0x2ae202fu32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd290u64) }
        0x102cd290 => { vm.decode(0x4288012fu32); let d = 18i16; let s = 5i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd294u64) }
        0x102cd294 => { vm.decode(0x8679af58u32); let d = 30i16; let s = 1i16; let s2 = -1i16; let imm = 63928i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102cd298u64) }
        0x102cd298 => { vm.decode(0x428a212fu32); let d = 22i16; let s = 5i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd29cu64) }
        0x102cd29c => { vm.decode(0x30000037u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as i8 as i64;
        vm.st(d, v as u64);
        Ok(0x102cd2a0u64) }
        0x102cd2a0 => { vm.decode(0x810224f5u32); let d = 4i16; let s = 4i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cd2a4u64) }
        0x102cd2a4 => { vm.decode(0xab4004f5u32); let d = 3i16; let s = 3i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cd2a8u64) }
        0x102cd2a8 => { vm.decode(0x80812du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102cd2acu64) }
        0x102cd2ac => { vm.decode(0x8f5u32); let d = -1i16; let s = -1i16; let s2 = -1i16; let imm = 0i64;
        
        Ok(0x102cd2b0u64) }
        0x102cd2b0 => { vm.decode(0x640c00a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 2480i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102cd2b4u64).wrapping_add((imm * 4) as u64) } else { 0x102cd2b4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cd2b4u64) }
        0x102cd2b4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cd2b8u64) } }
        0x102cd2b8 => { vm.decode(0x814184f5u32); let d = 1i16; let s = 17i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cd2bcu64) }
        0x102cd2bc => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd2c0u64) }
        0x102cd2c0 => { vm.decode(0x2410471u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 2436i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102cd2c4u64).wrapping_add((imm * 4) as u64) } else { 0x102cd2c4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cd2c4u64) }
        0x102cd2c4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cd2c8u64) } }
        0x102cd2c8 => { vm.decode(0x48005875u32); let d = 18i16; let s = 0i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd2ccu64) }
        0x102cd2cc => { vm.decode(0x4c30d875u32); let d = 19i16; let s = 0i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd2d0u64) }
        0x102cd2d0 => { vm.decode(0xcb00875u32); let d = 20i16; let s = 0i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd2d4u64) }
        0x102cd2d4 => { vm.decode(0x90054b0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd2d8u64) }
        0x102cd2d8 => { vm.decode(0x1900d570u32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd2dcu64) }
        0x102cd2dc => { vm.decode(0x1908d470u32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd2e0u64) }
        0x102cd2e0 => { vm.decode(0x9285470u32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd2e4u64) }
        0x102cd2e4 => { vm.decode(0x19301570u32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd2e8u64) }
        0x102cd2e8 => { vm.decode(0x9109570u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd2ecu64) }
        0x102cd2ec => { vm.decode(0x19381470u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd2f0u64) }
        0x102cd2f0 => { vm.decode(0x92014f0u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd2f4u64) }
        0x102cd2f4 => { vm.decode(0x91055f0u32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd2f8u64) }
        0x102cd2f8 => { vm.decode(0x4b2734f5u32); let d = 22i16; let s = 22i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cd2fcu64) }
        0x102cd2fc => { vm.decode(0xab4724f5u32); let d = 23i16; let s = 17i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cd300u64) }
        0x102cd300 => { vm.decode(0x138df2ceu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -5439488i64;
        vm.st(d, imm as u64);
        Ok(0x102cd304u64) }
        0x102cd304 => { vm.decode(0x4592f58u32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = 63912i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102cd308u64) }
        0x102cd308 => { vm.decode(0x214124f5u32); let d = 1i16; let s = 16i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cd30cu64) }
        0x102cd30c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd310u64) }
        0x102cd310 => { vm.decode(0xa00dcbb6u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = -244i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cd314u64) }
        0x102cd314 => { vm.decode(0x22355022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cd318u64) }
        0x102cd318 => { vm.decode(0x190094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd31cu64) }
        0x102cd31c => { vm.decode(0x8918d530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd320u64) }
        0x102cd320 => { vm.decode(0x40048019u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cd324u64) }
        0x102cd324 => { vm.decode(0x191815b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd328u64) }
        0x102cd328 => { vm.decode(0x28016fu32); let d = 2i16; let s = 3i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd32cu64) }
        0x102cd32c => { vm.decode(0x1b2014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd330u64) }
        0x102cd330 => { vm.decode(0xb381530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd334u64) }
        0x102cd334 => { vm.decode(0x664d8119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cd338u64) }
        0x102cd338 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd33cu64) }
        0x102cd33c => { vm.decode(0x190855b0u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd340u64) }
        0x102cd340 => { vm.decode(0xe00d875u32); let d = 25i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd344u64) }
        0x102cd344 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cd348u64 + 4);
        vm.wr(37, 0x102cd348u64 + 4);
        Ok(0x102cd348u64) }
        0x102cd348 => { vm.decode(0x89201470u32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd34cu64) }
        0x102cd34c => { vm.decode(0x40804b6fu32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd350u64) }
        0x102cd350 => { vm.decode(0x14184f5u32); let d = 1i16; let s = 16i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cd354u64) }
        0x102cd354 => { vm.decode(0x4594c211u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102cd358u64) }
        0x102cd358 => { vm.decode(0x22355022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cd35cu64) }
        0x102cd35c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd360u64) }
        0x102cd360 => { vm.decode(0x2010571u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 2372i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102cd364u64).wrapping_add((imm * 4) as u64) } else { 0x102cd364u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102cd364u64) }
        0x102cd364 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102cd368u64) } }
        0x102cd368 => { vm.decode(0x89085530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd36cu64) }
        0x102cd36c => { vm.decode(0x99105530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd370u64) }
        0x102cd370 => { vm.decode(0x834104f5u32); let d = 1i16; let s = 17i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cd374u64) }
        0x102cd374 => { vm.decode(0x19181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd378u64) }
        0x102cd378 => { vm.decode(0x2084636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd37cu64) }
        0x102cd37c => { vm.decode(0xa14004f5u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cd380u64) }
        0x102cd380 => { vm.decode(0x192814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd384u64) }
        0x102cd384 => { vm.decode(0x2284436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd388u64) }
        0x102cd388 => { vm.decode(0xa90604f5u32); let d = 22i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cd38cu64) }
        0x102cd38c => { vm.decode(0x2284636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd390u64) }
        0x102cd390 => { vm.decode(0xa98604f5u32); let d = 30i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cd394u64) }
        0x102cd394 => { vm.decode(0xa34104f5u32); let d = 1i16; let s = 17i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cd398u64) }
        0x102cd398 => { vm.decode(0x193814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd39cu64) }
        0x102cd39c => { vm.decode(0x814124f5u32); let d = 1i16; let s = 17i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cd3a0u64) }
        0x102cd3a0 => { vm.decode(0x193014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd3a4u64) }
        0x102cd3a4 => { vm.decode(0x4286632fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd3a8u64) }
        0x102cd3a8 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd3acu64) }
        0x102cd3ac => { vm.decode(0x10009419u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 2048i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cd3b0u64) }
        0x102cd3b0 => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd3b4u64) }
        0x102cd3b4 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd3b8u64) }
        0x102cd3b8 => { vm.decode(0x761b8019u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cd3bcu64) }
        0x102cd3bc => { vm.decode(0x608e432fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd3c0u64) }
        0x102cd3c0 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd3c4u64) }
        0x102cd3c4 => { vm.decode(0x408c632fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd3c8u64) }
        0x102cd3c8 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd3ccu64) }
        0x102cd3cc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cd3d0u64 + 4);
        vm.wr(37, 0x102cd3d0u64 + 4);
        Ok(0x102cd3d0u64) }
        0x102cd3d0 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd3d4u64) }
        0x102cd3d4 => { vm.decode(0x2284402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd3d8u64) }
        0x102cd3d8 => { vm.decode(0x9b281530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd3dcu64) }
        0x102cd3dc => { vm.decode(0x6a8443afu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd3e0u64) }
        0x102cd3e0 => { vm.decode(0x9b1814b0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd3e4u64) }
        0x102cd3e4 => { vm.decode(0x1b1014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd3e8u64) }
        0x102cd3e8 => { vm.decode(0x420d1022u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102cd3ecu64) }
        0x102cd3ec => { vm.decode(0x66598019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cd3f0u64) }
        0x102cd3f0 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd3f4u64) }
        0x102cd3f4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cd3f8u64 + 4);
        vm.wr(37, 0x102cd3f8u64 + 4);
        Ok(0x102cd3f8u64) }
        0x102cd3f8 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd3fcu64) }
        0x102cd3fc => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd400u64) }
        0x102cd400 => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd404u64) }
        0x102cd404 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd408u64) }
        0x102cd408 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd40cu64) }
        0x102cd40c => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd410u64) }
        0x102cd410 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cd414u64 + 4);
        vm.wr(37, 0x102cd414u64 + 4);
        Ok(0x102cd414u64) }
        0x102cd414 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd418u64) }
        0x102cd418 => { vm.decode(0x2880432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd41cu64) }
        0x102cd41c => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cd420u64) }
        0x102cd420 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd424u64) }
        0x102cd424 => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd428u64) }
        0x102cd428 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd42cu64) }
        0x102cd42c => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd430u64) }
        0x102cd430 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd434u64) }
        0x102cd434 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd438u64) }
        0x102cd438 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd43cu64) }
        0x102cd43c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cd440u64 + 4);
        vm.wr(37, 0x102cd440u64 + 4);
        Ok(0x102cd440u64) }
        0x102cd440 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd444u64) }
        0x102cd444 => { vm.decode(0x2880632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd448u64) }
        0x102cd448 => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cd44cu64) }
        0x102cd44c => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd450u64) }
        0x102cd450 => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd454u64) }
        0x102cd454 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd458u64) }
        0x102cd458 => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd45cu64) }
        0x102cd45c => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd460u64) }
        0x102cd460 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd464u64) }
        0x102cd464 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd468u64) }
        0x102cd468 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cd46cu64 + 4);
        vm.wr(37, 0x102cd46cu64 + 4);
        Ok(0x102cd46cu64) }
        0x102cd46c => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd470u64) }
        0x102cd470 => { vm.decode(0x2084436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd474u64) }
        0x102cd474 => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cd478u64) }
        0x102cd478 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd47cu64) }
        0x102cd47c => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd480u64) }
        0x102cd480 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd484u64) }
        0x102cd484 => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd488u64) }
        0x102cd488 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd48cu64) }
        0x102cd48c => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd490u64) }
        0x102cd490 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd494u64) }
        0x102cd494 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cd498u64 + 4);
        vm.wr(37, 0x102cd498u64 + 4);
        Ok(0x102cd498u64) }
        0x102cd498 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd49cu64) }
        0x102cd49c => { vm.decode(0x2280636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd4a0u64) }
        0x102cd4a0 => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cd4a4u64) }
        0x102cd4a4 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd4a8u64) }
        0x102cd4a8 => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd4acu64) }
        0x102cd4ac => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd4b0u64) }
        0x102cd4b0 => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd4b4u64) }
        0x102cd4b4 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd4b8u64) }
        0x102cd4b8 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd4bcu64) }
        0x102cd4bc => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd4c0u64) }
        0x102cd4c0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cd4c4u64 + 4);
        vm.wr(37, 0x102cd4c4u64 + 4);
        Ok(0x102cd4c4u64) }
        0x102cd4c4 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd4c8u64) }
        0x102cd4c8 => { vm.decode(0x2a80432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd4ccu64) }
        0x102cd4cc => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cd4d0u64) }
        0x102cd4d0 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd4d4u64) }
        0x102cd4d4 => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd4d8u64) }
        0x102cd4d8 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd4dcu64) }
        0x102cd4dc => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd4e0u64) }
        0x102cd4e0 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd4e4u64) }
        0x102cd4e4 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd4e8u64) }
        0x102cd4e8 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd4ecu64) }
        0x102cd4ec => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cd4f0u64 + 4);
        vm.wr(37, 0x102cd4f0u64 + 4);
        Ok(0x102cd4f0u64) }
        0x102cd4f0 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd4f4u64) }
        0x102cd4f4 => { vm.decode(0x2280436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd4f8u64) }
        0x102cd4f8 => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cd4fcu64) }
        0x102cd4fc => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd500u64) }
        0x102cd500 => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd504u64) }
        0x102cd504 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd508u64) }
        0x102cd508 => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd50cu64) }
        0x102cd50c => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd510u64) }
        0x102cd510 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd514u64) }
        0x102cd514 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd518u64) }
        0x102cd518 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cd51cu64 + 4);
        vm.wr(37, 0x102cd51cu64 + 4);
        Ok(0x102cd51cu64) }
        0x102cd51c => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd520u64) }
        0x102cd520 => { vm.decode(0x2080636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd524u64) }
        0x102cd524 => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102cd528u64) }
        0x102cd528 => { vm.decode(0x76499019u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cd52cu64) }
        0x102cd52c => { vm.decode(0x721b1159u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cd530u64) }
        0x102cd530 => { vm.decode(0x102236u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102cd534u64) }
        0x102cd534 => { vm.decode(0x409019u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cd538u64) }
        0x102cd538 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd53cu64) }
        0x102cd53c => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd540u64) }
        0x102cd540 => { vm.decode(0x890c875u32); let d = 21i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd544u64) }
        0x102cd544 => { vm.decode(0x2084402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd548u64) }
        0x102cd548 => { vm.decode(0x1b0815b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd54cu64) }
        0x102cd54c => { vm.decode(0x4a000a73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102cd550u64) }
        0x102cd550 => { vm.decode(0x8b185530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd554u64) }
        0x102cd554 => { vm.decode(0x1b101430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd558u64) }
        0x102cd558 => { vm.decode(0x4086636fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd55cu64) }
        0x102cd55c => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd560u64) }
        0x102cd560 => { vm.decode(0x490d875u32); let d = 5i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd564u64) }
        0x102cd564 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd568u64) }
        0x102cd568 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cd56cu64 + 4);
        vm.wr(37, 0x102cd56cu64 + 4);
        Ok(0x102cd56cu64) }
        0x102cd56c => { vm.decode(0x990055b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd570u64) }
        0x102cd570 => { vm.decode(0x704b0159u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cd574u64) }
        0x102cd574 => { vm.decode(0x2084412fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102cd578u64) }
        0x102cd578 => { vm.decode(0x401019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cd57cu64) }
        0x102cd57c => { vm.decode(0xb0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd580u64) }
        0x102cd580 => { vm.decode(0x4a000a73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102cd584u64) }
        0x102cd584 => { vm.decode(0x8b181430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd588u64) }
        0x102cd588 => { vm.decode(0x1b101430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102cd58cu64) }
        0x102cd58c => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd590u64) }
        0x102cd590 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd594u64) }
        0x102cd594 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102cd598u64 + 4);
        vm.wr(37, 0x102cd598u64 + 4);
        Ok(0x102cd598u64) }
        0x102cd598 => { vm.decode(0x490d875u32); let d = 5i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102cd59cu64) }
        0x102cd59c => { vm.decode(0x724b8119u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102cd5a0u64) }
        _ => Err(Error::UnknownPc(pc)),
    }
}
