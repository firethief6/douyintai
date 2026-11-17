#![cfg_attr(rustfmt, rustfmt::skip)]
// 由 scripts/medusa-vm/gen_native.py 生成，勿手改。
use super::super::Vm;
use super::super::Error;

#[allow(clippy::all, unused_parens, unused_variables, unused_mut)]
pub fn exec(vm: &mut Vm, pc: u64) -> Result<u64, Error> {
    match pc {
        0x102d169c => { vm.decode(0xe29060f5u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d16a0u64) }
        0x102d16a0 => { vm.decode(0x124d47f5u32); let d = 12i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d16a4u64) }
        0x102d16a4 => { vm.decode(0x28c875u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d16a8u64) }
        0x102d16a8 => { vm.decode(0xcac014f5u32); let d = 11i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d16acu64) }
        0x102d16ac => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d16b0u64) }
        0x102d16b0 => { vm.decode(0x8623036u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d16b4u64) }
        0x102d16b4 => { vm.decode(0xa0483d75u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d16b8u64) }
        0x102d16b8 => { vm.decode(0xe24014f5u32); let d = 1i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d16bcu64) }
        0x102d16bc => { vm.decode(0x2092fu32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d16c0u64) }
        0x102d16c0 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d16c4u64) }
        0x102d16c4 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d16c8u64) }
        0x102d16c8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d16ccu64).wrapping_add((imm * 4) as u64) } else { 0x102d16ccu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d16ccu64) }
        0x102d16cc => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d16d0u64) }
        0x102d16d0 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d16d4u64) }
        0x102d16d4 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d16d8u64) }
        0x102d16d8 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d16dcu64) }
        0x102d16dc => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d16e0u64) }
        0x102d16e0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d16e4u64).wrapping_add((imm * 4) as u64) } else { 0x102d16e4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d16e4u64) }
        0x102d16e4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d16e8u64) } }
        0x102d16e8 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d16ecu64) }
        0x102d16ec => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d16f0u64) }
        0x102d16f0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d16f4u64).wrapping_add((imm * 4) as u64) } else { 0x102d16f4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d16f4u64) }
        0x102d16f4 => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d16f8u64) }
        0x102d16f8 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d16fcu64) }
        0x102d16fc => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1700u64) }
        0x102d1700 => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1704u64) }
        0x102d1704 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1708u64) }
        0x102d1708 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d170cu64).wrapping_add((imm * 4) as u64) } else { 0x102d170cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d170cu64) }
        0x102d170c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1710u64) } }
        0x102d1710 => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1714u64) }
        0x102d1714 => { vm.decode(0x39089d75u32); let d = 14i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1718u64) }
        0x102d1718 => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d171cu64) }
        0x102d171c => { vm.decode(0xe29060f5u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d1720u64) }
        0x102d1720 => { vm.decode(0x124d47f5u32); let d = 12i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1724u64) }
        0x102d1724 => { vm.decode(0x28c875u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1728u64) }
        0x102d1728 => { vm.decode(0x3609036u32); let d = 12i16; let s = 7i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d172cu64) }
        0x102d172c => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1730u64) }
        0x102d1730 => { vm.decode(0x2e214f5u32); let d = 13i16; let s = 2i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1734u64) }
        0x102d1734 => { vm.decode(0x18001430u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d1738u64) }
        0x102d1738 => { vm.decode(0x226014f5u32); let d = 1i16; let s = 3i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d173cu64) }
        0x102d173c => { vm.decode(0x2092fu32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d1740u64) }
        0x102d1740 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1744u64) }
        0x102d1744 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1748u64) }
        0x102d1748 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d174cu64).wrapping_add((imm * 4) as u64) } else { 0x102d174cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d174cu64) }
        0x102d174c => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1750u64) }
        0x102d1750 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1754u64) }
        0x102d1754 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1758u64) }
        0x102d1758 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d175cu64) }
        0x102d175c => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1760u64) }
        0x102d1760 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1764u64).wrapping_add((imm * 4) as u64) } else { 0x102d1764u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1764u64) }
        0x102d1764 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1768u64) } }
        0x102d1768 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d176cu64) }
        0x102d176c => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1770u64) }
        0x102d1770 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1774u64).wrapping_add((imm * 4) as u64) } else { 0x102d1774u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1774u64) }
        0x102d1774 => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1778u64) }
        0x102d1778 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d177cu64) }
        0x102d177c => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1780u64) }
        0x102d1780 => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1784u64) }
        0x102d1784 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1788u64) }
        0x102d1788 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d178cu64).wrapping_add((imm * 4) as u64) } else { 0x102d178cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d178cu64) }
        0x102d178c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1790u64) } }
        0x102d1790 => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1794u64) }
        0x102d1794 => { vm.decode(0x39089d75u32); let d = 14i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1798u64) }
        0x102d1798 => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d179cu64) }
        0x102d179c => { vm.decode(0xe29060f5u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d17a0u64) }
        0x102d17a0 => { vm.decode(0x124d47f5u32); let d = 12i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d17a4u64) }
        0x102d17a4 => { vm.decode(0x28c875u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d17a8u64) }
        0x102d17a8 => { vm.decode(0x4a8014f5u32); let d = 10i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d17acu64) }
        0x102d17ac => { vm.decode(0x18400030u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d17b0u64) }
        0x102d17b0 => { vm.decode(0xa24014f5u32); let d = 1i16; let s = 3i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d17b4u64) }
        0x102d17b4 => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d17b8u64) }
        0x102d17b8 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d17bcu64) }
        0x102d17bc => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d17c0u64) }
        0x102d17c0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d17c4u64).wrapping_add((imm * 4) as u64) } else { 0x102d17c4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d17c4u64) }
        0x102d17c4 => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d17c8u64) }
        0x102d17c8 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d17ccu64) }
        0x102d17cc => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d17d0u64) }
        0x102d17d0 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d17d4u64) }
        0x102d17d4 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d17d8u64) }
        0x102d17d8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d17dcu64).wrapping_add((imm * 4) as u64) } else { 0x102d17dcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d17dcu64) }
        0x102d17dc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d17e0u64) } }
        0x102d17e0 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d17e4u64) }
        0x102d17e4 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d17e8u64) }
        0x102d17e8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d17ecu64).wrapping_add((imm * 4) as u64) } else { 0x102d17ecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d17ecu64) }
        0x102d17ec => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d17f0u64) }
        0x102d17f0 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d17f4u64) }
        0x102d17f4 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d17f8u64) }
        0x102d17f8 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d17fcu64) }
        0x102d17fc => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1800u64) }
        0x102d1800 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1804u64).wrapping_add((imm * 4) as u64) } else { 0x102d1804u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1804u64) }
        0x102d1804 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1808u64) } }
        0x102d1808 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d180cu64) }
        0x102d180c => { vm.decode(0x2908ad75u32); let d = 12i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1810u64) }
        0x102d1810 => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1814u64) }
        0x102d1814 => { vm.decode(0xa2b220f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d1818u64) }
        0x102d1818 => { vm.decode(0x44e17f5u32); let d = 9i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d181cu64) }
        0x102d181c => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1820u64) }
        0x102d1820 => { vm.decode(0xc2c014f5u32); let d = 9i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1824u64) }
        0x102d1824 => { vm.decode(0x18000430u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d1828u64) }
        0x102d1828 => { vm.decode(0x62b036u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d182cu64) }
        0x102d182c => { vm.decode(0xa040bd75u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1830u64) }
        0x102d1830 => { vm.decode(0xe26004f5u32); let d = 1i16; let s = 3i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1834u64) }
        0x102d1834 => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d1838u64) }
        0x102d1838 => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d183cu64) }
        0x102d183c => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1840u64) }
        0x102d1840 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1844u64).wrapping_add((imm * 4) as u64) } else { 0x102d1844u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1844u64) }
        0x102d1844 => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1848u64) }
        0x102d1848 => { vm.decode(0x49203236u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d184cu64) }
        0x102d184c => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1850u64) }
        0x102d1850 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1854u64) }
        0x102d1854 => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1858u64) }
        0x102d1858 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d185cu64).wrapping_add((imm * 4) as u64) } else { 0x102d185cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d185cu64) }
        0x102d185c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1860u64) } }
        0x102d1860 => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1864u64) }
        0x102d1864 => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1868u64) }
        0x102d1868 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d186cu64).wrapping_add((imm * 4) as u64) } else { 0x102d186cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d186cu64) }
        0x102d186c => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1870u64) }
        0x102d1870 => { vm.decode(0x49203236u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1874u64) }
        0x102d1874 => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1878u64) }
        0x102d1878 => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d187cu64) }
        0x102d187c => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1880u64) }
        0x102d1880 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1884u64).wrapping_add((imm * 4) as u64) } else { 0x102d1884u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1884u64) }
        0x102d1884 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1888u64) } }
        0x102d1888 => { vm.decode(0x4003a835u32); let d = 1i16; let s = 5i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d188cu64) }
        0x102d188c => { vm.decode(0x29483d75u32); let d = 11i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1890u64) }
        0x102d1890 => { vm.decode(0x21a17f5u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1894u64) }
        0x102d1894 => { vm.decode(0xa2b220f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d1898u64) }
        0x102d1898 => { vm.decode(0x44e17f5u32); let d = 9i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d189cu64) }
        0x102d189c => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d18a0u64) }
        0x102d18a0 => { vm.decode(0xca6204f5u32); let d = 7i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d18a4u64) }
        0x102d18a4 => { vm.decode(0x10401430u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d18a8u64) }
        0x102d18a8 => { vm.decode(0x224014f5u32); let d = 1i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d18acu64) }
        0x102d18ac => { vm.decode(0x200a012fu32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d18b0u64) }
        0x102d18b0 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d18b4u64) }
        0x102d18b4 => { vm.decode(0xa0100446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d18b8u64) }
        0x102d18b8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d18bcu64).wrapping_add((imm * 4) as u64) } else { 0x102d18bcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d18bcu64) }
        0x102d18bc => { vm.decode(0x1000019u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d18c0u64) }
        0x102d18c0 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d18c4u64) }
        0x102d18c4 => { vm.decode(0x21400d75u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d18c8u64) }
        0x102d18c8 => { vm.decode(0xc118u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d18ccu64) }
        0x102d18cc => { vm.decode(0xa0100446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d18d0u64) }
        0x102d18d0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d18d4u64).wrapping_add((imm * 4) as u64) } else { 0x102d18d4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d18d4u64) }
        0x102d18d4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d18d8u64) } }
        0x102d18d8 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d18dcu64) }
        0x102d18dc => { vm.decode(0xa0100446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d18e0u64) }
        0x102d18e0 => { vm.decode(0x6314bc27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -253i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d18e4u64).wrapping_add((imm * 4) as u64) } else { 0x102d18e4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d18e4u64) }
        0x102d18e4 => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d18e8u64) }
        0x102d18e8 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d18ecu64) }
        0x102d18ec => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d18f0u64) }
        0x102d18f0 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d18f4u64) }
        0x102d18f4 => { vm.decode(0xa0100446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d18f8u64) }
        0x102d18f8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d18fcu64).wrapping_add((imm * 4) as u64) } else { 0x102d18fcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d18fcu64) }
        0x102d18fc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1900u64) } }
        0x102d1900 => { vm.decode(0x84401du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 6656i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d1904u64) }
        0x102d1904 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1908u64) } }
        0x102d1908 => { vm.decode(0x138d728eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -7798784i64;
        vm.st(d, imm as u64);
        Ok(0x102d190cu64) }
        0x102d190c => { vm.decode(0x214829d8u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 11720i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1910u64) }
        0x102d1910 => { vm.decode(0x880184f5u32); let d = 2i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1914u64) }
        0x102d1914 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d1918u64) }
        0x102d1918 => { vm.decode(0x2a88432fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d191cu64) }
        0x102d191c => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1920u64) }
        0x102d1920 => { vm.decode(0x2008012fu32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d1924u64) }
        0x102d1924 => { vm.decode(0x2000036u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1928u64) }
        0x102d1928 => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d192cu64) }
        0x102d192c => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1930u64).wrapping_add((imm * 4) as u64) } else { 0x102d1930u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1930u64) }
        0x102d1930 => { vm.decode(0x8e76efd9u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1934u64) }
        0x102d193c => { vm.decode(0x4042a835u32); let d = 1i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1940u64) }
        0x102d1940 => { vm.decode(0x3a482d75u32); let d = 9i16; let s = 7i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1944u64) }
        0x102d1944 => { vm.decode(0x20a17f5u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1948u64) }
        0x102d1948 => { vm.decode(0xc6a360f5u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d194cu64) }
        0x102d194c => { vm.decode(0x505257f5u32); let d = 7i16; let s = 7i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1950u64) }
        0x102d1950 => { vm.decode(0x408d875u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1954u64) }
        0x102d1954 => { vm.decode(0x4a2204f5u32); let d = 6i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1958u64) }
        0x102d1958 => { vm.decode(0x10401030u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d195cu64) }
        0x102d195c => { vm.decode(0xa400036u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1960u64) }
        0x102d1960 => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1964u64) }
        0x102d1964 => { vm.decode(0x140c8aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 251i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1968u64).wrapping_add((imm * 4) as u64) } else { 0x102d1968u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1968u64) }
        0x102d1968 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d196cu64) } }
        0x102d196c => { vm.decode(0x4668bb5u32); let d = 7i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d1970u64) }
        0x102d1970 => { vm.decode(0xb200bd75u32); let d = 6i16; let s = 7i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1974u64) }
        0x102d1974 => { vm.decode(0x626004f5u32); let d = 1i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1978u64) }
        0x102d1978 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d197cu64) }
        0x102d197c => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1980u64) }
        0x102d1980 => { vm.decode(0x82100446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1984u64) }
        0x102d1984 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1988u64).wrapping_add((imm * 4) as u64) } else { 0x102d1988u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1988u64) }
        0x102d1988 => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d198cu64) }
        0x102d198c => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1990u64) }
        0x102d1990 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1994u64) }
        0x102d1994 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1998u64) }
        0x102d1998 => { vm.decode(0x82100446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d199cu64) }
        0x102d199c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d19a0u64).wrapping_add((imm * 4) as u64) } else { 0x102d19a0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d19a0u64) }
        0x102d19a0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d19a4u64) } }
        0x102d19a4 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d19a8u64) }
        0x102d19a8 => { vm.decode(0x82100446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d19acu64) }
        0x102d19ac => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d19b0u64).wrapping_add((imm * 4) as u64) } else { 0x102d19b0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d19b0u64) }
        0x102d19b0 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d19b4u64) }
        0x102d19b4 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d19b8u64) }
        0x102d19b8 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d19bcu64) }
        0x102d19bc => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d19c0u64) }
        0x102d19c0 => { vm.decode(0x82100446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d19c4u64) }
        0x102d19c4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d19c8u64).wrapping_add((imm * 4) as u64) } else { 0x102d19c8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d19c8u64) }
        0x102d19c8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d19ccu64) } }
        0x102d19cc => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d19d0u64) }
        0x102d19d0 => { vm.decode(0x29081d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d19d4u64) }
        0x102d19d4 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d19d8u64) }
        0x102d19d8 => { vm.decode(0xa29020f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d19dcu64) }
        0x102d19dc => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d19e0u64) }
        0x102d19e0 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d19e4u64) }
        0x102d19e4 => { vm.decode(0x9609036u32); let d = 8i16; let s = 7i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d19e8u64) }
        0x102d19e8 => { vm.decode(0xa1480d75u32); let d = 9i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d19ecu64) }
        0x102d19ec => { vm.decode(0x828014f5u32); let d = 8i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d19f0u64) }
        0x102d19f0 => { vm.decode(0x18000030u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d19f4u64) }
        0x102d19f4 => { vm.decode(0x8603036u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d19f8u64) }
        0x102d19f8 => { vm.decode(0xa0083d75u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d19fcu64) }
        0x102d19fc => { vm.decode(0x624014f5u32); let d = 1i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1a00u64) }
        0x102d1a00 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d1a04u64) }
        0x102d1a04 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1a08u64) }
        0x102d1a08 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1a0cu64) }
        0x102d1a0c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1a10u64).wrapping_add((imm * 4) as u64) } else { 0x102d1a10u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1a10u64) }
        0x102d1a10 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1a14u64) }
        0x102d1a14 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1a18u64) }
        0x102d1a18 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1a1cu64) }
        0x102d1a1c => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1a20u64) }
        0x102d1a20 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1a24u64) }
        0x102d1a24 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1a28u64).wrapping_add((imm * 4) as u64) } else { 0x102d1a28u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1a28u64) }
        0x102d1a28 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1a2cu64) } }
        0x102d1a2c => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1a30u64) }
        0x102d1a30 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1a34u64) }
        0x102d1a34 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1a38u64).wrapping_add((imm * 4) as u64) } else { 0x102d1a38u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1a38u64) }
        0x102d1a38 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1a3cu64) }
        0x102d1a3c => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1a40u64) }
        0x102d1a40 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1a44u64) }
        0x102d1a44 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1a48u64) }
        0x102d1a48 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1a4cu64) }
        0x102d1a4c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1a50u64).wrapping_add((imm * 4) as u64) } else { 0x102d1a50u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1a50u64) }
        0x102d1a50 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1a54u64) } }
        0x102d1a54 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1a58u64) }
        0x102d1a58 => { vm.decode(0x29088d75u32); let d = 12i16; let s = 8i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1a5cu64) }
        0x102d1a5c => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1a60u64) }
        0x102d1a60 => { vm.decode(0xa29020f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d1a64u64) }
        0x102d1a64 => { vm.decode(0x64c07f5u32); let d = 8i16; let s = 8i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1a68u64) }
        0x102d1a68 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1a6cu64) }
        0x102d1a6c => { vm.decode(0x9629036u32); let d = 8i16; let s = 7i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1a70u64) }
        0x102d1a70 => { vm.decode(0xa1080d75u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1a74u64) }
        0x102d1a74 => { vm.decode(0xac014f5u32); let d = 11i16; let s = 2i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1a78u64) }
        0x102d1a78 => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d1a7cu64) }
        0x102d1a7c => { vm.decode(0x623036u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1a80u64) }
        0x102d1a80 => { vm.decode(0xa0483d75u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1a84u64) }
        0x102d1a84 => { vm.decode(0xe24014f5u32); let d = 1i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1a88u64) }
        0x102d1a88 => { vm.decode(0x2092fu32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d1a8cu64) }
        0x102d1a8c => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1a90u64) }
        0x102d1a90 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1a94u64) }
        0x102d1a94 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1a98u64).wrapping_add((imm * 4) as u64) } else { 0x102d1a98u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1a98u64) }
        0x102d1a98 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1a9cu64) }
        0x102d1a9c => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1aa0u64) }
        0x102d1aa0 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1aa4u64) }
        0x102d1aa4 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1aa8u64) }
        0x102d1aa8 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1aacu64) }
        0x102d1aac => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1ab0u64).wrapping_add((imm * 4) as u64) } else { 0x102d1ab0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1ab0u64) }
        0x102d1ab0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1ab4u64) } }
        0x102d1ab4 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1ab8u64) }
        0x102d1ab8 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1abcu64) }
        0x102d1abc => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1ac0u64).wrapping_add((imm * 4) as u64) } else { 0x102d1ac0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1ac0u64) }
        0x102d1ac0 => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1ac4u64) }
        0x102d1ac4 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1ac8u64) }
        0x102d1ac8 => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1accu64) }
        0x102d1acc => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1ad0u64) }
        0x102d1ad0 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1ad4u64) }
        0x102d1ad4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1ad8u64).wrapping_add((imm * 4) as u64) } else { 0x102d1ad8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1ad8u64) }
        0x102d1ad8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1adcu64) } }
        0x102d1adc => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1ae0u64) }
        0x102d1ae0 => { vm.decode(0x39089d75u32); let d = 14i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1ae4u64) }
        0x102d1ae4 => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1ae8u64) }
        0x102d1ae8 => { vm.decode(0xe29060f5u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d1aecu64) }
        0x102d1aec => { vm.decode(0x124d47f5u32); let d = 12i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1af0u64) }
        0x102d1af0 => { vm.decode(0x28c875u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1af4u64) }
        0x102d1af4 => { vm.decode(0xcac014f5u32); let d = 11i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1af8u64) }
        0x102d1af8 => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d1afcu64) }
        0x102d1afc => { vm.decode(0x8623036u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1b00u64) }
        0x102d1b00 => { vm.decode(0xa0483d75u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1b04u64) }
        0x102d1b04 => { vm.decode(0xe24014f5u32); let d = 1i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1b08u64) }
        0x102d1b08 => { vm.decode(0x2092fu32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d1b0cu64) }
        0x102d1b0c => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1b10u64) }
        0x102d1b10 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1b14u64) }
        0x102d1b14 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1b18u64).wrapping_add((imm * 4) as u64) } else { 0x102d1b18u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1b18u64) }
        0x102d1b18 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1b1cu64) }
        0x102d1b1c => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1b20u64) }
        0x102d1b20 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1b24u64) }
        0x102d1b24 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1b28u64) }
        0x102d1b28 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1b2cu64) }
        0x102d1b2c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1b30u64).wrapping_add((imm * 4) as u64) } else { 0x102d1b30u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1b30u64) }
        0x102d1b30 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1b34u64) } }
        0x102d1b34 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1b38u64) }
        0x102d1b38 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1b3cu64) }
        0x102d1b3c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1b40u64).wrapping_add((imm * 4) as u64) } else { 0x102d1b40u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1b40u64) }
        0x102d1b40 => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1b44u64) }
        0x102d1b44 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1b48u64) }
        0x102d1b48 => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1b4cu64) }
        0x102d1b4c => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1b50u64) }
        0x102d1b50 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1b54u64) }
        0x102d1b54 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1b58u64).wrapping_add((imm * 4) as u64) } else { 0x102d1b58u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1b58u64) }
        0x102d1b58 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1b5cu64) } }
        0x102d1b5c => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1b60u64) }
        0x102d1b60 => { vm.decode(0x39089d75u32); let d = 14i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1b64u64) }
        0x102d1b64 => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1b68u64) }
        0x102d1b68 => { vm.decode(0xe29060f5u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d1b6cu64) }
        0x102d1b6c => { vm.decode(0x124d47f5u32); let d = 12i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1b70u64) }
        0x102d1b70 => { vm.decode(0x28c875u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1b74u64) }
        0x102d1b74 => { vm.decode(0x3609036u32); let d = 12i16; let s = 7i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1b78u64) }
        0x102d1b78 => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1b7cu64) }
        0x102d1b7c => { vm.decode(0x2e214f5u32); let d = 13i16; let s = 2i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1b80u64) }
        0x102d1b80 => { vm.decode(0x18001430u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d1b84u64) }
        0x102d1b84 => { vm.decode(0x226014f5u32); let d = 1i16; let s = 3i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1b88u64) }
        0x102d1b88 => { vm.decode(0x2092fu32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d1b8cu64) }
        0x102d1b8c => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1b90u64) }
        0x102d1b90 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1b94u64) }
        0x102d1b94 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1b98u64).wrapping_add((imm * 4) as u64) } else { 0x102d1b98u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1b98u64) }
        0x102d1b98 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1b9cu64) }
        0x102d1b9c => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1ba0u64) }
        0x102d1ba0 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1ba4u64) }
        0x102d1ba4 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1ba8u64) }
        0x102d1ba8 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1bacu64) }
        0x102d1bac => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1bb0u64).wrapping_add((imm * 4) as u64) } else { 0x102d1bb0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1bb0u64) }
        0x102d1bb0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1bb4u64) } }
        0x102d1bb4 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1bb8u64) }
        0x102d1bb8 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1bbcu64) }
        0x102d1bbc => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1bc0u64).wrapping_add((imm * 4) as u64) } else { 0x102d1bc0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1bc0u64) }
        0x102d1bc0 => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1bc4u64) }
        0x102d1bc4 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1bc8u64) }
        0x102d1bc8 => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1bccu64) }
        0x102d1bcc => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1bd0u64) }
        0x102d1bd0 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1bd4u64) }
        0x102d1bd4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1bd8u64).wrapping_add((imm * 4) as u64) } else { 0x102d1bd8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1bd8u64) }
        0x102d1bd8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1bdcu64) } }
        0x102d1bdc => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1be0u64) }
        0x102d1be0 => { vm.decode(0x39089d75u32); let d = 14i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1be4u64) }
        0x102d1be4 => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1be8u64) }
        0x102d1be8 => { vm.decode(0xe29060f5u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d1becu64) }
        0x102d1bec => { vm.decode(0x124d47f5u32); let d = 12i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1bf0u64) }
        0x102d1bf0 => { vm.decode(0x28c875u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1bf4u64) }
        0x102d1bf4 => { vm.decode(0x4a8014f5u32); let d = 10i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1bf8u64) }
        0x102d1bf8 => { vm.decode(0x18400030u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d1bfcu64) }
        0x102d1bfc => { vm.decode(0xa24014f5u32); let d = 1i16; let s = 3i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1c00u64) }
        0x102d1c00 => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d1c04u64) }
        0x102d1c04 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1c08u64) }
        0x102d1c08 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1c0cu64) }
        0x102d1c0c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1c10u64).wrapping_add((imm * 4) as u64) } else { 0x102d1c10u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1c10u64) }
        0x102d1c10 => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1c14u64) }
        0x102d1c14 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1c18u64) }
        0x102d1c18 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1c1cu64) }
        0x102d1c1c => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1c20u64) }
        0x102d1c20 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1c24u64) }
        0x102d1c24 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1c28u64).wrapping_add((imm * 4) as u64) } else { 0x102d1c28u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1c28u64) }
        0x102d1c28 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1c2cu64) } }
        0x102d1c2c => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1c30u64) }
        0x102d1c30 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1c34u64) }
        0x102d1c34 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1c38u64).wrapping_add((imm * 4) as u64) } else { 0x102d1c38u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1c38u64) }
        0x102d1c38 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1c3cu64) }
        0x102d1c3c => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1c40u64) }
        0x102d1c40 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1c44u64) }
        0x102d1c44 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1c48u64) }
        0x102d1c48 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1c4cu64) }
        0x102d1c4c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1c50u64).wrapping_add((imm * 4) as u64) } else { 0x102d1c50u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1c50u64) }
        0x102d1c50 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1c54u64) } }
        0x102d1c54 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1c58u64) }
        0x102d1c58 => { vm.decode(0x2908ad75u32); let d = 12i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1c5cu64) }
        0x102d1c5c => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1c60u64) }
        0x102d1c60 => { vm.decode(0xa2b220f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d1c64u64) }
        0x102d1c64 => { vm.decode(0x44e17f5u32); let d = 9i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1c68u64) }
        0x102d1c68 => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1c6cu64) }
        0x102d1c6c => { vm.decode(0xc2c014f5u32); let d = 9i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1c70u64) }
        0x102d1c70 => { vm.decode(0x18000430u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d1c74u64) }
        0x102d1c74 => { vm.decode(0x62b036u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1c78u64) }
        0x102d1c78 => { vm.decode(0xa040bd75u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1c7cu64) }
        0x102d1c7c => { vm.decode(0xe26004f5u32); let d = 1i16; let s = 3i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1c80u64) }
        0x102d1c80 => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d1c84u64) }
        0x102d1c84 => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1c88u64) }
        0x102d1c88 => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1c8cu64) }
        0x102d1c8c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1c90u64).wrapping_add((imm * 4) as u64) } else { 0x102d1c90u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1c90u64) }
        0x102d1c90 => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1c94u64) }
        0x102d1c94 => { vm.decode(0x49203236u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1c98u64) }
        0x102d1c98 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1c9cu64) }
        0x102d1c9c => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1ca0u64) }
        0x102d1ca0 => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1ca4u64) }
        0x102d1ca4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1ca8u64).wrapping_add((imm * 4) as u64) } else { 0x102d1ca8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1ca8u64) }
        0x102d1ca8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1cacu64) } }
        0x102d1cac => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1cb0u64) }
        0x102d1cb0 => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1cb4u64) }
        0x102d1cb4 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1cb8u64).wrapping_add((imm * 4) as u64) } else { 0x102d1cb8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1cb8u64) }
        0x102d1cb8 => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1cbcu64) }
        0x102d1cbc => { vm.decode(0x49203236u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1cc0u64) }
        0x102d1cc0 => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1cc4u64) }
        0x102d1cc4 => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1cc8u64) }
        0x102d1cc8 => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1cccu64) }
        0x102d1ccc => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1cd0u64).wrapping_add((imm * 4) as u64) } else { 0x102d1cd0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1cd0u64) }
        0x102d1cd0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1cd4u64) } }
        0x102d1cd4 => { vm.decode(0x4003a835u32); let d = 1i16; let s = 5i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1cd8u64) }
        0x102d1cd8 => { vm.decode(0x29483d75u32); let d = 11i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1cdcu64) }
        0x102d1cdc => { vm.decode(0x21a17f5u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1ce0u64) }
        0x102d1ce0 => { vm.decode(0xa2b220f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d1ce4u64) }
        0x102d1ce4 => { vm.decode(0x44e17f5u32); let d = 9i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1ce8u64) }
        0x102d1ce8 => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1cecu64) }
        0x102d1cec => { vm.decode(0xca6204f5u32); let d = 7i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1cf0u64) }
        0x102d1cf0 => { vm.decode(0x10401430u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d1cf4u64) }
        0x102d1cf4 => { vm.decode(0x224014f5u32); let d = 1i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1cf8u64) }
        0x102d1cf8 => { vm.decode(0x200a012fu32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d1cfcu64) }
        0x102d1cfc => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1d00u64) }
        0x102d1d00 => { vm.decode(0xa0100446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1d04u64) }
        0x102d1d04 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1d08u64).wrapping_add((imm * 4) as u64) } else { 0x102d1d08u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1d08u64) }
        0x102d1d08 => { vm.decode(0x1000019u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1d0cu64) }
        0x102d1d0c => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1d10u64) }
        0x102d1d10 => { vm.decode(0x21400d75u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1d14u64) }
        0x102d1d14 => { vm.decode(0xc118u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1d18u64) }
        0x102d1d18 => { vm.decode(0xa0100446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1d1cu64) }
        0x102d1d1c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1d20u64).wrapping_add((imm * 4) as u64) } else { 0x102d1d20u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1d20u64) }
        0x102d1d20 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1d24u64) } }
        0x102d1d24 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1d28u64) }
        0x102d1d28 => { vm.decode(0xa0100446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1d2cu64) }
        0x102d1d2c => { vm.decode(0x6314bc27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -253i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1d30u64).wrapping_add((imm * 4) as u64) } else { 0x102d1d30u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1d30u64) }
        0x102d1d30 => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1d34u64) }
        0x102d1d34 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1d38u64) }
        0x102d1d38 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1d3cu64) }
        0x102d1d3c => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1d40u64) }
        0x102d1d40 => { vm.decode(0xa0100446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1d44u64) }
        0x102d1d44 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1d48u64).wrapping_add((imm * 4) as u64) } else { 0x102d1d48u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1d48u64) }
        0x102d1d48 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1d4cu64) } }
        0x102d1d4c => { vm.decode(0x2ca4401du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 7756i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d1d50u64) }
        0x102d1d50 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1d54u64) } }
        0x102d1d54 => { vm.decode(0x138d728eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -7798784i64;
        vm.st(d, imm as u64);
        Ok(0x102d1d58u64) }
        0x102d1d58 => { vm.decode(0x214829d8u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 11720i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1d5cu64) }
        0x102d1d5c => { vm.decode(0x88636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d1d60u64) }
        0x102d1d60 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1d64u64) }
        0x102d1d64 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d1d68u64) }
        0x102d1d68 => { vm.decode(0x804184f5u32); let d = 1i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1d6cu64) }
        0x102d1d6c => { vm.decode(0x2008012fu32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d1d70u64) }
        0x102d1d70 => { vm.decode(0x2000036u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1d74u64) }
        0x102d1d74 => { vm.decode(0x40804b6fu32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d1d78u64) }
        0x102d1d78 => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1d7cu64) }
        0x102d1d7c => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1d80u64).wrapping_add((imm * 4) as u64) } else { 0x102d1d80u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1d80u64) }
        0x102d1d80 => { vm.decode(0x8e76efd9u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1d84u64) }
        0x102d1d8c => { vm.decode(0x4042a835u32); let d = 1i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1d90u64) }
        0x102d1d90 => { vm.decode(0x3a482d75u32); let d = 9i16; let s = 7i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1d94u64) }
        0x102d1d94 => { vm.decode(0x20a17f5u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1d98u64) }
        0x102d1d98 => { vm.decode(0xc6a360f5u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d1d9cu64) }
        0x102d1d9c => { vm.decode(0x505257f5u32); let d = 7i16; let s = 7i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1da0u64) }
        0x102d1da0 => { vm.decode(0x408d875u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1da4u64) }
        0x102d1da4 => { vm.decode(0x4a2204f5u32); let d = 6i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1da8u64) }
        0x102d1da8 => { vm.decode(0x10401030u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d1dacu64) }
        0x102d1dac => { vm.decode(0xa400036u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1db0u64) }
        0x102d1db0 => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1db4u64) }
        0x102d1db4 => { vm.decode(0x140c8aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 251i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1db8u64).wrapping_add((imm * 4) as u64) } else { 0x102d1db8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1db8u64) }
        0x102d1db8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1dbcu64) } }
        0x102d1dbc => { vm.decode(0x4668bb5u32); let d = 7i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d1dc0u64) }
        0x102d1dc0 => { vm.decode(0xb200bd75u32); let d = 6i16; let s = 7i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1dc4u64) }
        0x102d1dc4 => { vm.decode(0x626004f5u32); let d = 1i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1dc8u64) }
        0x102d1dc8 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d1dccu64) }
        0x102d1dcc => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1dd0u64) }
        0x102d1dd0 => { vm.decode(0x82100446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1dd4u64) }
        0x102d1dd4 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1dd8u64).wrapping_add((imm * 4) as u64) } else { 0x102d1dd8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1dd8u64) }
        0x102d1dd8 => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1ddcu64) }
        0x102d1ddc => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1de0u64) }
        0x102d1de0 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1de4u64) }
        0x102d1de4 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1de8u64) }
        0x102d1de8 => { vm.decode(0x82100446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1decu64) }
        0x102d1dec => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1df0u64).wrapping_add((imm * 4) as u64) } else { 0x102d1df0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1df0u64) }
        0x102d1df0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1df4u64) } }
        0x102d1df4 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1df8u64) }
        0x102d1df8 => { vm.decode(0x82100446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1dfcu64) }
        0x102d1dfc => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1e00u64).wrapping_add((imm * 4) as u64) } else { 0x102d1e00u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1e00u64) }
        0x102d1e00 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1e04u64) }
        0x102d1e04 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1e08u64) }
        0x102d1e08 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1e0cu64) }
        0x102d1e0c => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1e10u64) }
        0x102d1e10 => { vm.decode(0x82100446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1e14u64) }
        0x102d1e14 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1e18u64).wrapping_add((imm * 4) as u64) } else { 0x102d1e18u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1e18u64) }
        0x102d1e18 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1e1cu64) } }
        0x102d1e1c => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1e20u64) }
        0x102d1e20 => { vm.decode(0x29081d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1e24u64) }
        0x102d1e24 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1e28u64) }
        0x102d1e28 => { vm.decode(0xa29020f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d1e2cu64) }
        0x102d1e2c => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1e30u64) }
        0x102d1e30 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1e34u64) }
        0x102d1e34 => { vm.decode(0x9609036u32); let d = 8i16; let s = 7i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1e38u64) }
        0x102d1e38 => { vm.decode(0xa1480d75u32); let d = 9i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1e3cu64) }
        0x102d1e3c => { vm.decode(0x828014f5u32); let d = 8i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1e40u64) }
        0x102d1e40 => { vm.decode(0x18000030u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d1e44u64) }
        0x102d1e44 => { vm.decode(0x8603036u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1e48u64) }
        0x102d1e48 => { vm.decode(0xa0083d75u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1e4cu64) }
        0x102d1e4c => { vm.decode(0x624014f5u32); let d = 1i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1e50u64) }
        0x102d1e50 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d1e54u64) }
        0x102d1e54 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1e58u64) }
        0x102d1e58 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1e5cu64) }
        0x102d1e5c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1e60u64).wrapping_add((imm * 4) as u64) } else { 0x102d1e60u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1e60u64) }
        0x102d1e60 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1e64u64) }
        0x102d1e64 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1e68u64) }
        0x102d1e68 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1e6cu64) }
        0x102d1e6c => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1e70u64) }
        0x102d1e70 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1e74u64) }
        0x102d1e74 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1e78u64).wrapping_add((imm * 4) as u64) } else { 0x102d1e78u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1e78u64) }
        0x102d1e78 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1e7cu64) } }
        0x102d1e7c => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1e80u64) }
        0x102d1e80 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1e84u64) }
        0x102d1e84 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1e88u64).wrapping_add((imm * 4) as u64) } else { 0x102d1e88u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1e88u64) }
        0x102d1e88 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1e8cu64) }
        0x102d1e8c => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1e90u64) }
        0x102d1e90 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1e94u64) }
        0x102d1e94 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1e98u64) }
        0x102d1e98 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1e9cu64) }
        0x102d1e9c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1ea0u64).wrapping_add((imm * 4) as u64) } else { 0x102d1ea0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1ea0u64) }
        0x102d1ea0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1ea4u64) } }
        0x102d1ea4 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1ea8u64) }
        0x102d1ea8 => { vm.decode(0x29088d75u32); let d = 12i16; let s = 8i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1eacu64) }
        0x102d1eac => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1eb0u64) }
        0x102d1eb0 => { vm.decode(0xa29020f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d1eb4u64) }
        0x102d1eb4 => { vm.decode(0x64c07f5u32); let d = 8i16; let s = 8i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1eb8u64) }
        0x102d1eb8 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1ebcu64) }
        0x102d1ebc => { vm.decode(0x9629036u32); let d = 8i16; let s = 7i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1ec0u64) }
        0x102d1ec0 => { vm.decode(0xa1080d75u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1ec4u64) }
        0x102d1ec4 => { vm.decode(0xac014f5u32); let d = 11i16; let s = 2i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1ec8u64) }
        0x102d1ec8 => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d1eccu64) }
        0x102d1ecc => { vm.decode(0x623036u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1ed0u64) }
        0x102d1ed0 => { vm.decode(0xa0483d75u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1ed4u64) }
        0x102d1ed4 => { vm.decode(0xe24014f5u32); let d = 1i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1ed8u64) }
        0x102d1ed8 => { vm.decode(0x2092fu32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d1edcu64) }
        0x102d1edc => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1ee0u64) }
        0x102d1ee0 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1ee4u64) }
        0x102d1ee4 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1ee8u64).wrapping_add((imm * 4) as u64) } else { 0x102d1ee8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1ee8u64) }
        0x102d1ee8 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1eecu64) }
        0x102d1eec => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1ef0u64) }
        0x102d1ef0 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1ef4u64) }
        0x102d1ef4 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1ef8u64) }
        0x102d1ef8 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1efcu64) }
        0x102d1efc => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1f00u64).wrapping_add((imm * 4) as u64) } else { 0x102d1f00u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1f00u64) }
        0x102d1f00 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1f04u64) } }
        0x102d1f04 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1f08u64) }
        0x102d1f08 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1f0cu64) }
        0x102d1f0c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1f10u64).wrapping_add((imm * 4) as u64) } else { 0x102d1f10u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1f10u64) }
        0x102d1f10 => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1f14u64) }
        0x102d1f14 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1f18u64) }
        0x102d1f18 => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1f1cu64) }
        0x102d1f1c => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1f20u64) }
        0x102d1f20 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1f24u64) }
        0x102d1f24 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1f28u64).wrapping_add((imm * 4) as u64) } else { 0x102d1f28u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1f28u64) }
        0x102d1f28 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1f2cu64) } }
        0x102d1f2c => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1f30u64) }
        0x102d1f30 => { vm.decode(0x39089d75u32); let d = 14i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1f34u64) }
        0x102d1f34 => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1f38u64) }
        0x102d1f38 => { vm.decode(0xe29060f5u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d1f3cu64) }
        0x102d1f3c => { vm.decode(0x124d47f5u32); let d = 12i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1f40u64) }
        0x102d1f40 => { vm.decode(0x28c875u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1f44u64) }
        0x102d1f44 => { vm.decode(0xcac014f5u32); let d = 11i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1f48u64) }
        0x102d1f48 => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d1f4cu64) }
        0x102d1f4c => { vm.decode(0x8623036u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1f50u64) }
        0x102d1f50 => { vm.decode(0xa0483d75u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1f54u64) }
        0x102d1f54 => { vm.decode(0xe24014f5u32); let d = 1i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1f58u64) }
        0x102d1f58 => { vm.decode(0x2092fu32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d1f5cu64) }
        0x102d1f5c => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1f60u64) }
        0x102d1f60 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1f64u64) }
        0x102d1f64 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1f68u64).wrapping_add((imm * 4) as u64) } else { 0x102d1f68u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1f68u64) }
        0x102d1f68 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1f6cu64) }
        0x102d1f6c => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1f70u64) }
        0x102d1f70 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1f74u64) }
        0x102d1f74 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1f78u64) }
        0x102d1f78 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1f7cu64) }
        0x102d1f7c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1f80u64).wrapping_add((imm * 4) as u64) } else { 0x102d1f80u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1f80u64) }
        0x102d1f80 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1f84u64) } }
        0x102d1f84 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1f88u64) }
        0x102d1f88 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1f8cu64) }
        0x102d1f8c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1f90u64).wrapping_add((imm * 4) as u64) } else { 0x102d1f90u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1f90u64) }
        0x102d1f90 => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1f94u64) }
        0x102d1f94 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1f98u64) }
        0x102d1f98 => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1f9cu64) }
        0x102d1f9c => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1fa0u64) }
        0x102d1fa0 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1fa4u64) }
        0x102d1fa4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d1fa8u64).wrapping_add((imm * 4) as u64) } else { 0x102d1fa8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1fa8u64) }
        0x102d1fa8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d1facu64) } }
        0x102d1fac => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1fb0u64) }
        0x102d1fb0 => { vm.decode(0x39089d75u32); let d = 14i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1fb4u64) }
        0x102d1fb4 => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1fb8u64) }
        0x102d1fb8 => { vm.decode(0xe29060f5u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d1fbcu64) }
        0x102d1fbc => { vm.decode(0x124d47f5u32); let d = 12i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1fc0u64) }
        0x102d1fc0 => { vm.decode(0x28c875u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d1fc4u64) }
        0x102d1fc4 => { vm.decode(0x3609036u32); let d = 12i16; let s = 7i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1fc8u64) }
        0x102d1fc8 => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1fccu64) }
        0x102d1fcc => { vm.decode(0x2e214f5u32); let d = 13i16; let s = 2i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1fd0u64) }
        0x102d1fd0 => { vm.decode(0x18001430u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d1fd4u64) }
        0x102d1fd4 => { vm.decode(0x226014f5u32); let d = 1i16; let s = 3i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d1fd8u64) }
        0x102d1fd8 => { vm.decode(0x2092fu32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d1fdcu64) }
        0x102d1fdc => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1fe0u64) }
        0x102d1fe0 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1fe4u64) }
        0x102d1fe4 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d1fe8u64).wrapping_add((imm * 4) as u64) } else { 0x102d1fe8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d1fe8u64) }
        0x102d1fe8 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d1fecu64) }
        0x102d1fec => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d1ff0u64) }
        0x102d1ff0 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d1ff4u64) }
        0x102d1ff4 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d1ff8u64) }
        0x102d1ff8 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d1ffcu64) }
        0x102d1ffc => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d2000u64).wrapping_add((imm * 4) as u64) } else { 0x102d2000u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2000u64) }
        0x102d2000 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d2004u64) } }
        0x102d2004 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2008u64) }
        0x102d2008 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d200cu64) }
        0x102d200c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d2010u64).wrapping_add((imm * 4) as u64) } else { 0x102d2010u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2010u64) }
        0x102d2010 => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2014u64) }
        0x102d2014 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2018u64) }
        0x102d2018 => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d201cu64) }
        0x102d201c => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d2020u64) }
        0x102d2020 => { vm.decode(0x83100446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2024u64) }
        0x102d2024 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d2028u64).wrapping_add((imm * 4) as u64) } else { 0x102d2028u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2028u64) }
        0x102d2028 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d202cu64) } }
        0x102d202c => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2030u64) }
        0x102d2030 => { vm.decode(0x39089d75u32); let d = 14i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d2034u64) }
        0x102d2034 => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2038u64) }
        0x102d2038 => { vm.decode(0xe29060f5u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d203cu64) }
        0x102d203c => { vm.decode(0x124d47f5u32); let d = 12i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2040u64) }
        0x102d2040 => { vm.decode(0x28c875u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2044u64) }
        0x102d2044 => { vm.decode(0x4a8014f5u32); let d = 10i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2048u64) }
        0x102d2048 => { vm.decode(0x18400030u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d204cu64) }
        0x102d204c => { vm.decode(0xa24014f5u32); let d = 1i16; let s = 3i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2050u64) }
        0x102d2050 => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2054u64) }
        0x102d2054 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2058u64) }
        0x102d2058 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d205cu64) }
        0x102d205c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d2060u64).wrapping_add((imm * 4) as u64) } else { 0x102d2060u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2060u64) }
        0x102d2060 => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2064u64) }
        0x102d2064 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2068u64) }
        0x102d2068 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d206cu64) }
        0x102d206c => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d2070u64) }
        0x102d2070 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2074u64) }
        0x102d2074 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d2078u64).wrapping_add((imm * 4) as u64) } else { 0x102d2078u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2078u64) }
        0x102d2078 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d207cu64) } }
        0x102d207c => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2080u64) }
        0x102d2080 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2084u64) }
        0x102d2084 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d2088u64).wrapping_add((imm * 4) as u64) } else { 0x102d2088u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2088u64) }
        0x102d2088 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d208cu64) }
        0x102d208c => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2090u64) }
        0x102d2090 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d2094u64) }
        0x102d2094 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d2098u64) }
        0x102d2098 => { vm.decode(0x81100446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d209cu64) }
        0x102d209c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d20a0u64).wrapping_add((imm * 4) as u64) } else { 0x102d20a0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d20a0u64) }
        0x102d20a0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d20a4u64) } }
        0x102d20a4 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d20a8u64) }
        0x102d20a8 => { vm.decode(0x2908ad75u32); let d = 12i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d20acu64) }
        0x102d20ac => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d20b0u64) }
        0x102d20b0 => { vm.decode(0xa2b220f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d20b4u64) }
        0x102d20b4 => { vm.decode(0x44e17f5u32); let d = 9i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d20b8u64) }
        0x102d20b8 => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d20bcu64) }
        0x102d20bc => { vm.decode(0xc2c014f5u32); let d = 9i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d20c0u64) }
        0x102d20c0 => { vm.decode(0x18000430u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d20c4u64) }
        0x102d20c4 => { vm.decode(0x62b036u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d20c8u64) }
        0x102d20c8 => { vm.decode(0xa040bd75u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d20ccu64) }
        0x102d20cc => { vm.decode(0xe26004f5u32); let d = 1i16; let s = 3i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d20d0u64) }
        0x102d20d0 => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d20d4u64) }
        0x102d20d4 => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d20d8u64) }
        0x102d20d8 => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d20dcu64) }
        0x102d20dc => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d20e0u64).wrapping_add((imm * 4) as u64) } else { 0x102d20e0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d20e0u64) }
        0x102d20e0 => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d20e4u64) }
        0x102d20e4 => { vm.decode(0x49203236u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d20e8u64) }
        0x102d20e8 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d20ecu64) }
        0x102d20ec => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d20f0u64) }
        0x102d20f0 => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d20f4u64) }
        0x102d20f4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d20f8u64).wrapping_add((imm * 4) as u64) } else { 0x102d20f8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d20f8u64) }
        0x102d20f8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d20fcu64) } }
        0x102d20fc => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2100u64) }
        0x102d2100 => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2104u64) }
        0x102d2104 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d2108u64).wrapping_add((imm * 4) as u64) } else { 0x102d2108u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2108u64) }
        0x102d2108 => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d210cu64) }
        0x102d210c => { vm.decode(0x49203236u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2110u64) }
        0x102d2110 => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d2114u64) }
        0x102d2114 => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d2118u64) }
        0x102d2118 => { vm.decode(0xa2100446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d211cu64) }
        0x102d211c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d2120u64).wrapping_add((imm * 4) as u64) } else { 0x102d2120u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2120u64) }
        0x102d2120 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d2124u64) } }
        0x102d2124 => { vm.decode(0x4003a835u32); let d = 1i16; let s = 5i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2128u64) }
        0x102d2128 => { vm.decode(0x29483d75u32); let d = 11i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d212cu64) }
        0x102d212c => { vm.decode(0x21a17f5u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2130u64) }
        0x102d2130 => { vm.decode(0xa2b220f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 59i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102d2134u64) }
        0x102d2134 => { vm.decode(0x44e17f5u32); let d = 9i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2138u64) }
        0x102d2138 => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d213cu64) }
        0x102d213c => { vm.decode(0xca6204f5u32); let d = 7i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2140u64) }
        0x102d2140 => { vm.decode(0x10401430u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2144u64) }
        0x102d2144 => { vm.decode(0x224014f5u32); let d = 1i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2148u64) }
        0x102d2148 => { vm.decode(0x200a012fu32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d214cu64) }
        0x102d214c => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2150u64) }
        0x102d2150 => { vm.decode(0xa0100446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2154u64) }
        0x102d2154 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d2158u64).wrapping_add((imm * 4) as u64) } else { 0x102d2158u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2158u64) }
        0x102d2158 => { vm.decode(0x1000019u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d215cu64) }
        0x102d215c => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2160u64) }
        0x102d2160 => { vm.decode(0x21400d75u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d2164u64) }
        0x102d2164 => { vm.decode(0xc118u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d2168u64) }
        0x102d2168 => { vm.decode(0xa0100446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d216cu64) }
        0x102d216c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d2170u64).wrapping_add((imm * 4) as u64) } else { 0x102d2170u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2170u64) }
        0x102d2170 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d2174u64) } }
        0x102d2174 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2178u64) }
        0x102d2178 => { vm.decode(0xa0100446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d217cu64) }
        0x102d217c => { vm.decode(0x6314bc27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -253i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d2180u64).wrapping_add((imm * 4) as u64) } else { 0x102d2180u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2180u64) }
        0x102d2180 => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2184u64) }
        0x102d2184 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2188u64) }
        0x102d2188 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d218cu64) }
        0x102d218c => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d2190u64) }
        0x102d2190 => { vm.decode(0xa0100446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 5i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2194u64) }
        0x102d2194 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d2198u64).wrapping_add((imm * 4) as u64) } else { 0x102d2198u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2198u64) }
        0x102d2198 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d219cu64) } }
        0x102d219c => { vm.decode(0xa880051du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 8860i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d21a0u64) }
        0x102d21a0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d21a4u64) } }
        0x102d21a4 => { vm.decode(0x138d728eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -7798784i64;
        vm.st(d, imm as u64);
        Ok(0x102d21a8u64) }
        0x102d21a8 => { vm.decode(0x214829d8u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 11720i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d21acu64) }
        0x102d21ac => { vm.decode(0x804184f5u32); let d = 1i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d21b0u64) }
        0x102d21b0 => { vm.decode(0x90000430u32); let d = 17i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d21b4u64) }
        0x102d21b4 => { vm.decode(0x8918d530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d21b8u64) }
        0x102d21b8 => { vm.decode(0xd1cc011u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102d21bcu64) }
        0x102d21bc => { vm.decode(0x28e4b2fu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d21c0u64) }
        0x102d21c0 => { vm.decode(0x82020071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 13i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d21c4u64).wrapping_add((imm * 4) as u64) } else { 0x102d21c4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d21c4u64) }
        0x102d21c4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d21c8u64) } }
        0x102d21c8 => { vm.decode(0x2284632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d21ccu64) }
        0x102d21cc => { vm.decode(0x2200212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d21d0u64) }
        0x102d21d0 => { vm.decode(0x88c432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d21d4u64) }
        0x102d21d4 => { vm.decode(0x841019u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d21d8u64) }
        0x102d21d8 => { vm.decode(0xb2015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d21dcu64) }
        0x102d21dc => { vm.decode(0x1b381430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d21e0u64) }
        0x102d21e0 => { vm.decode(0x664d8119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d21e4u64) }
        0x102d21e4 => { vm.decode(0x60846b2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d21e8u64) }
        0x102d21e8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d21ecu64 + 4);
        vm.wr(37, 0x102d21ecu64 + 4);
        Ok(0x102d21ecu64) }
        0x102d21ec => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d21f0u64) }
        0x102d21f0 => { vm.decode(0x40804b6fu32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d21f4u64) }
        0x102d21f4 => { vm.decode(0x28e4b2fu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d21f8u64) }
        0x102d21f8 => { vm.decode(0x844b6fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d21fcu64) }
        0x102d21fc => { vm.decode(0x22806b6fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2200u64) }
        0x102d2200 => { vm.decode(0xa884b2fu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2204u64) }
        0x102d2204 => { vm.decode(0x22884b6fu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2208u64) }
        0x102d2208 => { vm.decode(0x826b6fu32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d220cu64) }
        0x102d220c => { vm.decode(0x28826b2fu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2210u64) }
        0x102d2210 => { vm.decode(0x288a4b2fu32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2214u64) }
        0x102d2214 => { vm.decode(0x8098b27u32); let d = 0i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d2218u64).wrapping_add((imm * 4) as u64) } else { 0x102d2218u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2218u64) }
        0x102d2218 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d221cu64) } }
        0x102d221c => { vm.decode(0x138d728eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -7798784i64;
        vm.st(d, imm as u64);
        Ok(0x102d2220u64) }
        0x102d2220 => { vm.decode(0x214829d8u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 11720i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d2224u64) }
        0x102d2224 => { vm.decode(0x880184f5u32); let d = 2i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2228u64) }
        0x102d2228 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d222cu64) }
        0x102d222c => { vm.decode(0x84891b0u32); let d = 2i16; let s = 14i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2230u64) }
        0x102d2230 => { vm.decode(0xaa00a4f5u32); let d = 2i16; let s = 1i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2234u64) }
        0x102d2234 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2238u64) }
        0x102d2238 => { vm.decode(0x85091b0u32); let d = 2i16; let s = 14i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d223cu64) }
        0x102d223c => { vm.decode(0xa800a4f5u32); let d = 2i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2240u64) }
        0x102d2240 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2244u64) }
        0x102d2244 => { vm.decode(0x85891b0u32); let d = 2i16; let s = 14i16; let s2 = -1i16; let imm = 184i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2248u64) }
        0x102d2248 => { vm.decode(0x8800a4f5u32); let d = 2i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d224cu64) }
        0x102d224c => { vm.decode(0xaa4084f5u32); let d = 3i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2250u64) }
        0x102d2250 => { vm.decode(0x820284f5u32); let d = 4i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2254u64) }
        0x102d2254 => { vm.decode(0xa04284f5u32); let d = 5i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2258u64) }
        0x102d2258 => { vm.decode(0x804084f5u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d225cu64) }
        0x102d225c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2260u64) }
        0x102d2260 => { vm.decode(0x18609030u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2264u64) }
        0x102d2264 => { vm.decode(0x2080012fu32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2268u64) }
        0x102d2268 => { vm.decode(0x18689030u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d226cu64) }
        0x102d226c => { vm.decode(0x2080002fu32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2270u64) }
        0x102d2270 => { vm.decode(0x18709030u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2274u64) }
        0x102d2274 => { vm.decode(0x2020012fu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2278u64) }
        0x102d2278 => { vm.decode(0x18789030u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d227cu64) }
        0x102d227c => { vm.decode(0x2020002fu32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2280u64) }
        0x102d2280 => { vm.decode(0x186090b0u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 224i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2284u64) }
        0x102d2284 => { vm.decode(0x4a8043efu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2288u64) }
        0x102d2288 => { vm.decode(0x6a8063efu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d228cu64) }
        0x102d228c => { vm.decode(0x488c43efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2290u64) }
        0x102d2290 => { vm.decode(0x688c63efu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2294u64) }
        0x102d2294 => { vm.decode(0x4a8643efu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2298u64) }
        0x102d2298 => { vm.decode(0x6a8663efu32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 760i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d229cu64) }
        0x102d229c => { vm.decode(0x408ac3afu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d22a0u64) }
        0x102d22a0 => { vm.decode(0x608ae3afu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d22a4u64) }
        0x102d22a4 => { vm.decode(0x428acbafu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d22a8u64) }
        0x102d22a8 => { vm.decode(0x628aebafu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d22acu64) }
        0x102d22ac => { vm.decode(0x54104cf5u32); let d = -1i16; let s = -1i16; let s2 = -1i16; let imm = 0i64;
        vm.jump_flag = 2;
        vm.wr(36, vm.slots[31]);
        vm.wr(35, 2);
        Ok(0x102d22b0u64) }
        0x102d22b0 => { vm.decode(0x77198059u32); let d = 29i16; let s = 29i16; let s2 = -1i16; let imm = 800i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d22b4u64) }
        0x102d22b4 => { vm.decode(0x90054b0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d22b8u64) }
        0x102d22b8 => { vm.decode(0x830204f5u32); let d = 4i16; let s = 2i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d22bcu64) }
        0x102d22bc => { vm.decode(0x2020012fu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d22c0u64) }
        0x102d22c0 => { vm.decode(0x8b209430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d22c4u64) }
        0x102d22c4 => { vm.decode(0x1b1894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d22c8u64) }
        0x102d22c8 => { vm.decode(0x665f8019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 696i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d22ccu64) }
        0x102d22cc => { vm.decode(0x190855b0u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d22d0u64) }
        0x102d22d0 => { vm.decode(0x99181470u32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d22d4u64) }
        0x102d22d4 => { vm.decode(0x60846b2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d22d8u64) }
        0x102d22d8 => { vm.decode(0x91055f0u32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d22dcu64) }
        0x102d22dc => { vm.decode(0x92014f0u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d22e0u64) }
        0x102d22e0 => { vm.decode(0x19381470u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d22e4u64) }
        0x102d22e4 => { vm.decode(0x9109570u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d22e8u64) }
        0x102d22e8 => { vm.decode(0x19301570u32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d22ecu64) }
        0x102d22ec => { vm.decode(0x9285470u32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d22f0u64) }
        0x102d22f0 => { vm.decode(0x1908d470u32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d22f4u64) }
        0x102d22f4 => { vm.decode(0x1900d570u32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d22f8u64) }
        0x102d22f8 => { vm.decode(0x89201470u32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d22fcu64) }
        0x102d22fc => { vm.decode(0x191815b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2300u64) }
        0x102d2300 => { vm.decode(0x99105570u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2304u64) }
        0x102d2304 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d2308u64 + 4);
        vm.wr(37, 0x102d2308u64 + 4);
        Ok(0x102d2308u64) }
        0x102d2308 => { vm.decode(0x9085530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d230cu64) }
        0x102d230c => { vm.decode(0x8a632fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2310u64) }
        0x102d2310 => { vm.decode(0x628a4b2fu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2314u64) }
        0x102d2314 => { vm.decode(0x62806b2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2318u64) }
        0x102d2318 => { vm.decode(0x86432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d231cu64) }
        0x102d231c => { vm.decode(0x228c632fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2320u64) }
        0x102d2320 => { vm.decode(0x40804b6fu32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2324u64) }
        0x102d2324 => { vm.decode(0x288a4b2fu32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2328u64) }
        0x102d2328 => { vm.decode(0x208e632fu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d232cu64) }
        0x102d232c => { vm.decode(0x28826b2fu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2330u64) }
        0x102d2330 => { vm.decode(0x826b6fu32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2334u64) }
        0x102d2334 => { vm.decode(0x22884b6fu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2338u64) }
        0x102d2338 => { vm.decode(0xa884b2fu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d233cu64) }
        0x102d233c => { vm.decode(0x22806b6fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2340u64) }
        0x102d2340 => { vm.decode(0x844b6fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2344u64) }
        0x102d2344 => { vm.decode(0x28e4b2fu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2348u64) }
        0x102d2348 => { vm.decode(0xd98c211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 712i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102d234cu64) }
        0x102d234c => { vm.decode(0x31c3e27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -2534i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d2350u64).wrapping_add((imm * 4) as u64) } else { 0x102d2350u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2350u64) }
        0x102d2350 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d2354u64) } }
        0x102d2354 => { vm.decode(0x4b2514f5u32); let d = 18i16; let s = 18i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2358u64) }
        0x102d2358 => { vm.decode(0xa34104f5u32); let d = 1i16; let s = 17i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d235cu64) }
        0x102d235c => { vm.decode(0x19001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2360u64) }
        0x102d2360 => { vm.decode(0x810724f5u32); let d = 20i16; let s = 17i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2364u64) }
        0x102d2364 => { vm.decode(0x2020212fu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2368u64) }
        0x102d2368 => { vm.decode(0x501019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 48i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d236cu64) }
        0x102d236c => { vm.decode(0xb0895b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2370u64) }
        0x102d2370 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2374u64) }
        0x102d2374 => { vm.decode(0x86432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2378u64) }
        0x102d2378 => { vm.decode(0x60846b2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d237cu64) }
        0x102d237c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d2380u64 + 4);
        vm.wr(37, 0x102d2380u64 + 4);
        Ok(0x102d2380u64) }
        0x102d2380 => { vm.decode(0x661b8019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2384u64) }
        0x102d2384 => { vm.decode(0x6a8c43afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2388u64) }
        0x102d2388 => { vm.decode(0x81400530u32); let d = 18i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d238cu64) }
        0x102d238c => { vm.decode(0x660b8019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2390u64) }
        0x102d2390 => { vm.decode(0x2284632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2394u64) }
        0x102d2394 => { vm.decode(0x2204212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2398u64) }
        0x102d2398 => { vm.decode(0xb109430u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d239cu64) }
        0x102d239c => { vm.decode(0x60841019u32); let d = 2i16; let s = 19i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d23a0u64) }
        0x102d23a0 => { vm.decode(0xb089530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d23a4u64) }
        0x102d23a4 => { vm.decode(0x1b009430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d23a8u64) }
        0x102d23a8 => { vm.decode(0x60846b2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d23acu64) }
        0x102d23ac => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d23b0u64 + 4);
        vm.wr(37, 0x102d23b0u64 + 4);
        Ok(0x102d23b0u64) }
        0x102d23b0 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d23b4u64) }
        0x102d23b4 => { vm.decode(0x138d728eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -7798784i64;
        vm.st(d, imm as u64);
        Ok(0x102d23b8u64) }
        0x102d23b8 => { vm.decode(0x214829d8u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 11720i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d23bcu64) }
        0x102d23bc => { vm.decode(0xa04124f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d23c0u64) }
        0x102d23c0 => { vm.decode(0x90000530u32); let d = 19i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d23c4u64) }
        0x102d23c4 => { vm.decode(0x2284632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d23c8u64) }
        0x102d23c8 => { vm.decode(0x2200012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d23ccu64) }
        0x102d23cc => { vm.decode(0x8b3814b0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d23d0u64) }
        0x102d23d0 => { vm.decode(0x1b3014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d23d4u64) }
        0x102d23d4 => { vm.decode(0x82432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d23d8u64) }
        0x102d23d8 => { vm.decode(0x60846b2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d23dcu64) }
        0x102d23dc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d23e0u64 + 4);
        vm.wr(37, 0x102d23e0u64 + 4);
        Ok(0x102d23e0u64) }
        0x102d23e0 => { vm.decode(0x66598119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d23e4u64) }
        0x102d23e4 => { vm.decode(0x8a632fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d23e8u64) }
        0x102d23e8 => { vm.decode(0x628a4b2fu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d23ecu64) }
        0x102d23ec => { vm.decode(0x62806b2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d23f0u64) }
        0x102d23f0 => { vm.decode(0x86432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d23f4u64) }
        0x102d23f4 => { vm.decode(0x228c632fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d23f8u64) }
        0x102d23f8 => { vm.decode(0x40804b6fu32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d23fcu64) }
        0x102d23fc => { vm.decode(0x288a4b2fu32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2400u64) }
        0x102d2400 => { vm.decode(0x208e632fu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2404u64) }
        0x102d2404 => { vm.decode(0x28826b2fu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2408u64) }
        0x102d2408 => { vm.decode(0x826b6fu32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d240cu64) }
        0x102d240c => { vm.decode(0x22884b6fu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2410u64) }
        0x102d2410 => { vm.decode(0xa884b2fu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2414u64) }
        0x102d2414 => { vm.decode(0x22806b6fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2418u64) }
        0x102d2418 => { vm.decode(0x844b6fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d241cu64) }
        0x102d241c => { vm.decode(0x28e4b2fu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2420u64) }
        0x102d2420 => { vm.decode(0x8820001du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 200i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d2424u64) }
        0x102d2424 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d2428u64) } }
        0x102d2430 => { vm.decode(0x7b3bcf19u32); let d = 29i16; let s = 29i16; let s2 = -1i16; let imm = -800i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2434u64) }
        0x102d2434 => { vm.decode(0x9b185d70u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 792i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2438u64) }
        0x102d2438 => { vm.decode(0x8b105d70u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 784i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d243cu64) }
        0x102d243c => { vm.decode(0x9b085d30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2440u64) }
        0x102d2440 => { vm.decode(0x8b005d30u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 768i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2444u64) }
        0x102d2444 => { vm.decode(0x9b38d4b0u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 760i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2448u64) }
        0x102d2448 => { vm.decode(0x8b30d4b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 752i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d244cu64) }
        0x102d244c => { vm.decode(0x9b2895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 744i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2450u64) }
        0x102d2450 => { vm.decode(0x8b2095b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 736i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2454u64) }
        0x102d2454 => { vm.decode(0x9b389430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 728i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2458u64) }
        0x102d2458 => { vm.decode(0x8b309430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 720i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d245cu64) }
        0x102d245c => { vm.decode(0x1395738eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -6684672i64;
        vm.st(d, imm as u64);
        Ok(0x102d2460u64) }
        0x102d2460 => { vm.decode(0x25482518u32); let d = 17i16; let s = 1i16; let s2 = -1i16; let imm = 6344i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d2464u64) }
        0x102d2464 => { vm.decode(0x88012fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2468u64) }
        0x102d2468 => { vm.decode(0x830404f5u32); let d = 16i16; let s = 2i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d246cu64) }
        0x102d246c => { vm.decode(0x286096fu32); let d = 12i16; let s = 5i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2470u64) }
        0x102d2470 => { vm.decode(0x208c296fu32); let d = 11i16; let s = 5i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2474u64) }
        0x102d2474 => { vm.decode(0x8c096fu32); let d = 10i16; let s = 5i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2478u64) }
        0x102d2478 => { vm.decode(0x2280296fu32); let d = 9i16; let s = 5i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d247cu64) }
        0x102d247c => { vm.decode(0x280096fu32); let d = 8i16; let s = 5i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2480u64) }
        0x102d2480 => { vm.decode(0x2082296fu32); let d = 13i16; let s = 5i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2484u64) }
        0x102d2484 => { vm.decode(0x208a096fu32); let d = 15i16; let s = 5i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2488u64) }
        0x102d2488 => { vm.decode(0x4284292fu32); let d = 24i16; let s = 5i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d248cu64) }
        0x102d248c => { vm.decode(0x6286012fu32); let d = 21i16; let s = 5i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2490u64) }
        0x102d2490 => { vm.decode(0x8a082fu32); let d = 14i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2494u64) }
        0x102d2494 => { vm.decode(0x2088212fu32); let d = 3i16; let s = 5i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2498u64) }
        0x102d2498 => { vm.decode(0x6084092fu32); let d = 25i16; let s = 5i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d249cu64) }
        0x102d249c => { vm.decode(0x608e292fu32); let d = 31i16; let s = 5i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d24a0u64) }
        0x102d24a0 => { vm.decode(0x28c216fu32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d24a4u64) }
        0x102d24a4 => { vm.decode(0x93815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d24a8u64) }
        0x102d24a8 => { vm.decode(0xa8002fu32); let d = 2i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d24acu64) }
        0x102d24ac => { vm.decode(0xa2202fu32); let d = 4i16; let s = 6i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d24b0u64) }
        0x102d24b0 => { vm.decode(0x42a2002fu32); let d = 20i16; let s = 6i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d24b4u64) }
        0x102d24b4 => { vm.decode(0x62a8202fu32); let d = 19i16; let s = 6i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d24b8u64) }
        0x102d24b8 => { vm.decode(0x60ae002fu32); let d = 23i16; let s = 6i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d24bcu64) }
        0x102d24bc => { vm.decode(0x40ac202fu32); let d = 18i16; let s = 6i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d24c0u64) }
        0x102d24c0 => { vm.decode(0x893015b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d24c4u64) }
        0x102d24c4 => { vm.decode(0x42ac002fu32); let d = 18i16; let s = 6i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d24c8u64) }
        0x102d24c8 => { vm.decode(0x892815b0u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d24ccu64) }
        0x102d24cc => { vm.decode(0x2ae202fu32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d24d0u64) }
        0x102d24d0 => { vm.decode(0x4288012fu32); let d = 18i16; let s = 5i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d24d4u64) }
        0x102d24d4 => { vm.decode(0x8768a518u32); let d = 30i16; let s = 1i16; let s2 = -1i16; let imm = 6360i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d24d8u64) }
        0x102d24d8 => { vm.decode(0x428a212fu32); let d = 22i16; let s = 5i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d24dcu64) }
        0x102d24dc => { vm.decode(0x30000037u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as i8 as i64;
        vm.st(d, v as u64);
        Ok(0x102d24e0u64) }
        0x102d24e0 => { vm.decode(0x810224f5u32); let d = 4i16; let s = 4i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d24e4u64) }
        0x102d24e4 => { vm.decode(0xab4004f5u32); let d = 3i16; let s = 3i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d24e8u64) }
        0x102d24e8 => { vm.decode(0x80812du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d24ecu64) }
        0x102d24ec => { vm.decode(0x8f5u32); let d = -1i16; let s = -1i16; let s2 = -1i16; let imm = 0i64;
        
        Ok(0x102d24f0u64) }
        0x102d24f0 => { vm.decode(0x700400a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 2496i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d24f4u64).wrapping_add((imm * 4) as u64) } else { 0x102d24f4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d24f4u64) }
        0x102d24f4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d24f8u64) } }
        0x102d24f8 => { vm.decode(0x814184f5u32); let d = 1i16; let s = 17i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d24fcu64) }
        0x102d24fc => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2500u64) }
        0x102d2500 => { vm.decode(0x2610471u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 2452i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d2504u64).wrapping_add((imm * 4) as u64) } else { 0x102d2504u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2504u64) }
        0x102d2504 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d2508u64) } }
        0x102d2508 => { vm.decode(0x48005875u32); let d = 18i16; let s = 0i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d250cu64) }
        0x102d250c => { vm.decode(0x4c30d875u32); let d = 19i16; let s = 0i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2510u64) }
        0x102d2510 => { vm.decode(0xcb00875u32); let d = 20i16; let s = 0i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2514u64) }
        0x102d2514 => { vm.decode(0x90054b0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2518u64) }
        0x102d2518 => { vm.decode(0x1900d570u32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d251cu64) }
        0x102d251c => { vm.decode(0x1908d470u32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2520u64) }
        0x102d2520 => { vm.decode(0x9285470u32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2524u64) }
        0x102d2524 => { vm.decode(0x19301570u32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2528u64) }
        0x102d2528 => { vm.decode(0x9109570u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d252cu64) }
        0x102d252c => { vm.decode(0x19381470u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2530u64) }
        0x102d2530 => { vm.decode(0x92014f0u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2534u64) }
        0x102d2534 => { vm.decode(0x91055f0u32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2538u64) }
        0x102d2538 => { vm.decode(0x4b2734f5u32); let d = 22i16; let s = 22i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d253cu64) }
        0x102d253c => { vm.decode(0xab4724f5u32); let d = 23i16; let s = 17i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2540u64) }
        0x102d2540 => { vm.decode(0x1395738eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -6684672i64;
        vm.st(d, imm as u64);
        Ok(0x102d2544u64) }
        0x102d2544 => { vm.decode(0x5482518u32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = 6344i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d2548u64) }
        0x102d2548 => { vm.decode(0x214124f5u32); let d = 1i16; let s = 16i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d254cu64) }
        0x102d254c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2550u64) }
        0x102d2550 => { vm.decode(0xa00dcbb6u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = -244i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2554u64) }
        0x102d2554 => { vm.decode(0x22355022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102d2558u64) }
        0x102d2558 => { vm.decode(0x190094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 160i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d255cu64) }
        0x102d255c => { vm.decode(0x8918d530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2560u64) }
        0x102d2560 => { vm.decode(0x40048019u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2564u64) }
        0x102d2564 => { vm.decode(0x191815b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2568u64) }
        0x102d2568 => { vm.decode(0x28016fu32); let d = 2i16; let s = 3i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d256cu64) }
        0x102d256c => { vm.decode(0x1b2014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2570u64) }
        0x102d2570 => { vm.decode(0xb381530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2574u64) }
        0x102d2574 => { vm.decode(0x664d8119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2578u64) }
        0x102d2578 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d257cu64) }
        0x102d257c => { vm.decode(0x190855b0u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2580u64) }
        0x102d2580 => { vm.decode(0xe00d875u32); let d = 25i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2584u64) }
        0x102d2584 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d2588u64 + 4);
        vm.wr(37, 0x102d2588u64 + 4);
        Ok(0x102d2588u64) }
        0x102d2588 => { vm.decode(0x89201470u32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d258cu64) }
        0x102d258c => { vm.decode(0x40804b6fu32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2590u64) }
        0x102d2590 => { vm.decode(0x14184f5u32); let d = 1i16; let s = 16i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2594u64) }
        0x102d2594 => { vm.decode(0x4594c211u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102d2598u64) }
        0x102d2598 => { vm.decode(0x22355022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102d259cu64) }
        0x102d259c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d25a0u64) }
        0x102d25a0 => { vm.decode(0x2210571u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 2388i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d25a4u64).wrapping_add((imm * 4) as u64) } else { 0x102d25a4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d25a4u64) }
        0x102d25a4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d25a8u64) } }
        0x102d25a8 => { vm.decode(0x89085530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d25acu64) }
        0x102d25ac => { vm.decode(0x99105530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d25b0u64) }
        0x102d25b0 => { vm.decode(0x834104f5u32); let d = 1i16; let s = 17i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d25b4u64) }
        0x102d25b4 => { vm.decode(0x19181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d25b8u64) }
        0x102d25b8 => { vm.decode(0x2084636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d25bcu64) }
        0x102d25bc => { vm.decode(0xa14004f5u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d25c0u64) }
        0x102d25c0 => { vm.decode(0x192814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d25c4u64) }
        0x102d25c4 => { vm.decode(0x2284436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d25c8u64) }
        0x102d25c8 => { vm.decode(0xa90604f5u32); let d = 22i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d25ccu64) }
        0x102d25cc => { vm.decode(0x2284636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d25d0u64) }
        0x102d25d0 => { vm.decode(0xa98604f5u32); let d = 30i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d25d4u64) }
        0x102d25d4 => { vm.decode(0xa34104f5u32); let d = 1i16; let s = 17i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d25d8u64) }
        0x102d25d8 => { vm.decode(0x193814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d25dcu64) }
        0x102d25dc => { vm.decode(0x814124f5u32); let d = 1i16; let s = 17i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d25e0u64) }
        0x102d25e0 => { vm.decode(0x193014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d25e4u64) }
        0x102d25e4 => { vm.decode(0x4286632fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d25e8u64) }
        0x102d25e8 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d25ecu64) }
        0x102d25ec => { vm.decode(0x10009419u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 2048i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d25f0u64) }
        0x102d25f0 => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d25f4u64) }
        0x102d25f4 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d25f8u64) }
        0x102d25f8 => { vm.decode(0x761b8019u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d25fcu64) }
        0x102d25fc => { vm.decode(0x608e432fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2600u64) }
        0x102d2600 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2604u64) }
        0x102d2604 => { vm.decode(0x408c632fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2608u64) }
        0x102d2608 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d260cu64) }
        0x102d260c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d2610u64 + 4);
        vm.wr(37, 0x102d2610u64 + 4);
        Ok(0x102d2610u64) }
        0x102d2610 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2614u64) }
        0x102d2614 => { vm.decode(0x2284402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2618u64) }
        0x102d2618 => { vm.decode(0x9b281530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d261cu64) }
        0x102d261c => { vm.decode(0x6a8443afu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2620u64) }
        0x102d2620 => { vm.decode(0x9b1814b0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2624u64) }
        0x102d2624 => { vm.decode(0x1b1014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2628u64) }
        0x102d2628 => { vm.decode(0x420d1022u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102d262cu64) }
        0x102d262c => { vm.decode(0x66598019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2630u64) }
        0x102d2630 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2634u64) }
        0x102d2634 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d2638u64 + 4);
        vm.wr(37, 0x102d2638u64 + 4);
        Ok(0x102d2638u64) }
        0x102d2638 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d263cu64) }
        0x102d263c => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2640u64) }
        0x102d2640 => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2644u64) }
        0x102d2644 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2648u64) }
        0x102d2648 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d264cu64) }
        0x102d264c => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2650u64) }
        0x102d2650 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d2654u64 + 4);
        vm.wr(37, 0x102d2654u64 + 4);
        Ok(0x102d2654u64) }
        0x102d2654 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2658u64) }
        0x102d2658 => { vm.decode(0x2880432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d265cu64) }
        0x102d265c => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2660u64) }
        0x102d2660 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2664u64) }
        0x102d2664 => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2668u64) }
        0x102d2668 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d266cu64) }
        0x102d266c => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2670u64) }
        0x102d2670 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2674u64) }
        0x102d2674 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2678u64) }
        0x102d2678 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d267cu64) }
        0x102d267c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d2680u64 + 4);
        vm.wr(37, 0x102d2680u64 + 4);
        Ok(0x102d2680u64) }
        0x102d2680 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2684u64) }
        0x102d2684 => { vm.decode(0x2880632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2688u64) }
        0x102d2688 => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d268cu64) }
        0x102d268c => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2690u64) }
        0x102d2690 => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2694u64) }
        0x102d2694 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2698u64) }
        0x102d2698 => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d269cu64) }
        0x102d269c => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d26a0u64) }
        0x102d26a0 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d26a4u64) }
        0x102d26a4 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d26a8u64) }
        0x102d26a8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d26acu64 + 4);
        vm.wr(37, 0x102d26acu64 + 4);
        Ok(0x102d26acu64) }
        0x102d26ac => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d26b0u64) }
        0x102d26b0 => { vm.decode(0x2084436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d26b4u64) }
        0x102d26b4 => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d26b8u64) }
        0x102d26b8 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d26bcu64) }
        0x102d26bc => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d26c0u64) }
        0x102d26c0 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d26c4u64) }
        0x102d26c4 => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d26c8u64) }
        0x102d26c8 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d26ccu64) }
        0x102d26cc => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d26d0u64) }
        0x102d26d0 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d26d4u64) }
        0x102d26d4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d26d8u64 + 4);
        vm.wr(37, 0x102d26d8u64 + 4);
        Ok(0x102d26d8u64) }
        0x102d26d8 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d26dcu64) }
        0x102d26dc => { vm.decode(0x2280636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d26e0u64) }
        0x102d26e0 => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d26e4u64) }
        0x102d26e4 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d26e8u64) }
        0x102d26e8 => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d26ecu64) }
        0x102d26ec => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d26f0u64) }
        0x102d26f0 => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d26f4u64) }
        0x102d26f4 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d26f8u64) }
        0x102d26f8 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d26fcu64) }
        0x102d26fc => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2700u64) }
        0x102d2700 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d2704u64 + 4);
        vm.wr(37, 0x102d2704u64 + 4);
        Ok(0x102d2704u64) }
        0x102d2704 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2708u64) }
        0x102d2708 => { vm.decode(0x2a80432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d270cu64) }
        0x102d270c => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2710u64) }
        0x102d2710 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2714u64) }
        0x102d2714 => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2718u64) }
        0x102d2718 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d271cu64) }
        0x102d271c => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2720u64) }
        0x102d2720 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2724u64) }
        0x102d2724 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2728u64) }
        0x102d2728 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d272cu64) }
        0x102d272c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d2730u64 + 4);
        vm.wr(37, 0x102d2730u64 + 4);
        Ok(0x102d2730u64) }
        0x102d2730 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2734u64) }
        0x102d2734 => { vm.decode(0x2280436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2738u64) }
        0x102d2738 => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d273cu64) }
        0x102d273c => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2740u64) }
        0x102d2740 => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2744u64) }
        0x102d2744 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2748u64) }
        0x102d2748 => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d274cu64) }
        0x102d274c => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2750u64) }
        0x102d2750 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2754u64) }
        0x102d2754 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2758u64) }
        0x102d2758 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d275cu64 + 4);
        vm.wr(37, 0x102d275cu64 + 4);
        Ok(0x102d275cu64) }
        0x102d275c => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2760u64) }
        0x102d2760 => { vm.decode(0x2080636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2764u64) }
        0x102d2764 => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2768u64) }
        0x102d2768 => { vm.decode(0x76499019u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d276cu64) }
        0x102d276c => { vm.decode(0x721b1159u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2770u64) }
        0x102d2770 => { vm.decode(0x102236u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2774u64) }
        0x102d2774 => { vm.decode(0x409019u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2778u64) }
        0x102d2778 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d277cu64) }
        0x102d277c => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2780u64) }
        0x102d2780 => { vm.decode(0x890c875u32); let d = 21i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2784u64) }
        0x102d2784 => { vm.decode(0x2084402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2788u64) }
        0x102d2788 => { vm.decode(0x1b0815b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d278cu64) }
        0x102d278c => { vm.decode(0x4a000a73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d2790u64) }
        0x102d2790 => { vm.decode(0x8b185530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2794u64) }
        0x102d2794 => { vm.decode(0x1b101430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2798u64) }
        0x102d2798 => { vm.decode(0x4086636fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d279cu64) }
        0x102d279c => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d27a0u64) }
        0x102d27a0 => { vm.decode(0x490d875u32); let d = 5i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d27a4u64) }
        0x102d27a4 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d27a8u64) }
        0x102d27a8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d27acu64 + 4);
        vm.wr(37, 0x102d27acu64 + 4);
        Ok(0x102d27acu64) }
        0x102d27ac => { vm.decode(0x990055b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d27b0u64) }
        0x102d27b0 => { vm.decode(0x704b0159u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d27b4u64) }
        0x102d27b4 => { vm.decode(0x2084412fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d27b8u64) }
        0x102d27b8 => { vm.decode(0x401019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d27bcu64) }
        0x102d27bc => { vm.decode(0xb0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d27c0u64) }
        0x102d27c0 => { vm.decode(0x4a000a73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d27c4u64) }
        0x102d27c4 => { vm.decode(0x8b181430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d27c8u64) }
        0x102d27c8 => { vm.decode(0x1b101430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d27ccu64) }
        0x102d27cc => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d27d0u64) }
        0x102d27d0 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d27d4u64) }
        0x102d27d4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d27d8u64 + 4);
        vm.wr(37, 0x102d27d8u64 + 4);
        Ok(0x102d27d8u64) }
        0x102d27d8 => { vm.decode(0x490d875u32); let d = 5i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d27dcu64) }
        0x102d27dc => { vm.decode(0x724b8119u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d27e0u64) }
        0x102d27e0 => { vm.decode(0x8059u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 256i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d27e4u64) }
        0x102d27e4 => { vm.decode(0x628e632fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d27e8u64) }
        0x102d27e8 => { vm.decode(0xac412fu32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d27ecu64) }
        0x102d27ec => { vm.decode(0x1b0814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d27f0u64) }
        0x102d27f0 => { vm.decode(0x4a000a73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d27f4u64) }
        0x102d27f4 => { vm.decode(0x9b185430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d27f8u64) }
        0x102d27f8 => { vm.decode(0xb101530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d27fcu64) }
        0x102d27fc => { vm.decode(0x2086432fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2800u64) }
        0x102d2800 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2804u64) }
        0x102d2804 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d2808u64 + 4);
        vm.wr(37, 0x102d2808u64 + 4);
        Ok(0x102d2808u64) }
        0x102d2808 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d280cu64) }
        0x102d280c => { vm.decode(0x625b8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2810u64) }
        0x102d2810 => { vm.decode(0x192854b0u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2814u64) }
        0x102d2814 => { vm.decode(0x720b0119u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2818u64) }
        0x102d2818 => { vm.decode(0x20a0616fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d281cu64) }
        0x102d281c => { vm.decode(0x401019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2820u64) }
        0x102d2820 => { vm.decode(0xb081530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2824u64) }
        0x102d2824 => { vm.decode(0x28c436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2828u64) }
        0x102d2828 => { vm.decode(0xb001530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d282cu64) }
        0x102d282c => { vm.decode(0x8938dcb0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2830u64) }
        0x102d2830 => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2834u64) }
        0x102d2834 => { vm.decode(0x282632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2838u64) }
        0x102d2838 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d283cu64) }
        0x102d283c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d2840u64 + 4);
        vm.wr(37, 0x102d2840u64 + 4);
        Ok(0x102d2840u64) }
        0x102d2840 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d2844u64) } }
        0x102d2844 => { vm.decode(0x725b9019u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2848u64) }
        0x102d2848 => { vm.decode(0x2284632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d284cu64) }
        0x102d284c => { vm.decode(0x2000216fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2850u64) }
        0x102d2850 => { vm.decode(0x401019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2854u64) }
        0x102d2854 => { vm.decode(0xb081530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2858u64) }
        0x102d2858 => { vm.decode(0x28c636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d285cu64) }
        0x102d285c => { vm.decode(0xb001530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2860u64) }
        0x102d2860 => { vm.decode(0x9938ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2864u64) }
        0x102d2864 => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2868u64) }
        0x102d2868 => { vm.decode(0x282632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d286cu64) }
        0x102d286c => { vm.decode(0x2086636fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2870u64) }
        0x102d2870 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2874u64) }
        0x102d2874 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d2878u64 + 4);
        vm.wr(37, 0x102d2878u64 + 4);
        Ok(0x102d2878u64) }
        0x102d2878 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d287cu64) } }
        0x102d287c => { vm.decode(0x26010406u32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 256i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2880u64) }
        0x102d2880 => { vm.decode(0x2200031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d2884u64).wrapping_add((imm * 4) as u64) } else { 0x102d2884u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2884u64) }
        0x102d2884 => { vm.decode(0xa00c4bb6u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = -256i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2888u64) }
        0x102d2888 => { vm.decode(0x400141du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 1296i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d288cu64) }
        0x102d2890 => { vm.decode(0x110019b5u32); let d = 1i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102d2894u64) }
        0x102d2894 => { vm.decode(0x360bb5u32); let d = 5i16; let s = 4i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d2898u64) }
        0x102d2898 => { vm.decode(0x120217f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d289cu64) }
        0x102d289c => { vm.decode(0x1000ad35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102d28a0u64) }
        0x102d28a0 => { vm.decode(0x560207f5u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d28a4u64) }
        0x102d28a4 => { vm.decode(0x4089875u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d28a8u64) }
        0x102d28a8 => { vm.decode(0x40c30bb5u32); let d = 3i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d28acu64) }
        0x102d28ac => { vm.decode(0xe84124f5u32); let d = 3i16; let s = 3i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d28b0u64) }
        0x102d28b0 => { vm.decode(0x220833u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d28b4u64) }
        0x102d28b4 => { vm.decode(0x8b03236u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d28b8u64) }
        0x102d28b8 => { vm.decode(0x26010406u32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 256i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d28bcu64) }
        0x102d28bc => { vm.decode(0x4040027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 32i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d28c0u64).wrapping_add((imm * 4) as u64) } else { 0x102d28c0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d28c0u64) }
        0x102d28c0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d28c4u64) } }
        0x102d28c4 => { vm.decode(0x7818e7b5u32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 31i64;
        let v = ((vm.ld(s) as i32) >> (imm as u32)) as i64;
        vm.st(d, v as u64);
        Ok(0x102d28c8u64) }
        0x102d28c8 => { vm.decode(0x40920d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102d28ccu64) }
        0x102d28cc => { vm.decode(0x44290fb5u32); let d = 1i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d28d0u64) }
        0x102d28d0 => { vm.decode(0x40217f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d28d4u64) }
        0x102d28d4 => { vm.decode(0xa8088b35u32); let d = 1i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d28d8u64) }
        0x102d28d8 => { vm.decode(0x8241a4f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d28dcu64) }
        0x102d28dc => { vm.decode(0x800023eu32); let d = 4i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d28e0u64) }
        0x102d28e0 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d28e4u64) }
        0x102d28e4 => { vm.decode(0x21000446u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d28e8u64) }
        0x102d28e8 => { vm.decode(0x40227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d28ecu64).wrapping_add((imm * 4) as u64) } else { 0x102d28ecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d28ecu64) }
        0x102d28ec => { vm.decode(0x2236u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d28f0u64) }
        0x102d28f0 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d28f4u64) }
        0x102d28f4 => { vm.decode(0xd2cac12du32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d28f8u64) }
        0x102d28f8 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d28fcu64) }
        0x102d28fc => { vm.decode(0xa0004118u32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d2900u64) }
        0x102d2900 => { vm.decode(0x21000446u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2904u64) }
        0x102d2904 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d2908u64).wrapping_add((imm * 4) as u64) } else { 0x102d2908u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2908u64) }
        0x102d2908 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d290cu64) } }
        0x102d290c => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2910u64) }
        0x102d2910 => { vm.decode(0x3000446u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2914u64) }
        0x102d2914 => { vm.decode(0x739c3ea7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -34i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d2918u64).wrapping_add((imm * 4) as u64) } else { 0x102d2918u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2918u64) }
        0x102d2918 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d291cu64) }
        0x102d291c => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2920u64) }
        0x102d2920 => { vm.decode(0xd2ca816du32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d2924u64) }
        0x102d2924 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d2928u64) }
        0x102d2928 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d292cu64) }
        0x102d292c => { vm.decode(0x3000446u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2930u64) }
        0x102d2930 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d2934u64).wrapping_add((imm * 4) as u64) } else { 0x102d2934u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2934u64) }
        0x102d2934 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d2938u64) } }
        0x102d2938 => { vm.decode(0x520001du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 1120i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d293cu64) }
        0x102d293c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d2940u64) } }
        0x102d2940 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2944u64) }
        0x102d2944 => { vm.decode(0x408e4b6fu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2948u64) }
        0x102d2948 => { vm.decode(0x628a6b6fu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d294cu64) }
        0x102d294c => { vm.decode(0x2002406u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2950u64) }
        0x102d2950 => { vm.decode(0x2200031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d2954u64).wrapping_add((imm * 4) as u64) } else { 0x102d2954u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2954u64) }
        0x102d2954 => { vm.decode(0xb40c6ff6u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = -16i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2958u64) }
        0x102d2958 => { vm.decode(0x25a0001du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 1636i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d295cu64) }
        0x102d2960 => { vm.decode(0x110219b5u32); let d = 1i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102d2964u64) }
        0x102d2964 => { vm.decode(0x760bb5u32); let d = 7i16; let s = 4i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d2968u64) }
        0x102d2968 => { vm.decode(0x121217f5u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d296cu64) }
        0x102d296c => { vm.decode(0x1000ad35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102d2970u64) }
        0x102d2970 => { vm.decode(0x140147f5u32); let d = 4i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2974u64) }
        0x102d2974 => { vm.decode(0x8c875u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2978u64) }
        0x102d2978 => { vm.decode(0x624b0159u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d297cu64) }
        0x102d297c => { vm.decode(0x802224f5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2980u64) }
        0x102d2980 => { vm.decode(0x40020033u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d2984u64) }
        0x102d2984 => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2988u64) }
        0x102d2988 => { vm.decode(0x2002406u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d298cu64) }
        0x102d298c => { vm.decode(0x10048027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 65i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d2990u64).wrapping_add((imm * 4) as u64) } else { 0x102d2990u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2990u64) }
        0x102d2990 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d2994u64) } }
        0x102d2994 => { vm.decode(0x3018e7b5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 31i64;
        let v = ((vm.ld(s) as i32) >> (imm as u32)) as i64;
        vm.st(d, v as u64);
        Ok(0x102d2998u64) }
        0x102d2998 => { vm.decode(0x50920d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 28i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102d299cu64) }
        0x102d299c => { vm.decode(0x40210fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d29a0u64) }
        0x102d29a0 => { vm.decode(0x60217f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d29a4u64) }
        0x102d29a4 => { vm.decode(0x200c0b35u32); let d = 4i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d29a8u64) }
        0x102d29a8 => { vm.decode(0x6124f5u32); let d = 1i16; let s = 4i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d29acu64) }
        0x102d29ac => { vm.decode(0xa00023eu32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d29b0u64) }
        0x102d29b0 => { vm.decode(0x2002236u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d29b4u64) }
        0x102d29b4 => { vm.decode(0x23000446u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d29b8u64) }
        0x102d29b8 => { vm.decode(0x40227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d29bcu64).wrapping_add((imm * 4) as u64) } else { 0x102d29bcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d29bcu64) }
        0x102d29bc => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d29c0u64) }
        0x102d29c0 => { vm.decode(0xa603236u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d29c4u64) }
        0x102d29c4 => { vm.decode(0xd2ca816du32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d29c8u64) }
        0x102d29c8 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d29ccu64) }
        0x102d29cc => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d29d0u64) }
        0x102d29d0 => { vm.decode(0x23000446u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d29d4u64) }
        0x102d29d4 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d29d8u64).wrapping_add((imm * 4) as u64) } else { 0x102d29d8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d29d8u64) }
        0x102d29d8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d29dcu64) } }
        0x102d29dc => { vm.decode(0x1000036u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d29e0u64) }
        0x102d29e0 => { vm.decode(0x80000446u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d29e4u64) }
        0x102d29e4 => { vm.decode(0x40227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d29e8u64).wrapping_add((imm * 4) as u64) } else { 0x102d29e8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d29e8u64) }
        0x102d29e8 => { vm.decode(0x2002236u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d29ecu64) }
        0x102d29ec => { vm.decode(0x49000036u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d29f0u64) }
        0x102d29f0 => { vm.decode(0xd2cac16du32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d29f4u64) }
        0x102d29f4 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d29f8u64) }
        0x102d29f8 => { vm.decode(0xa0204118u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d29fcu64) }
        0x102d29fc => { vm.decode(0x80000446u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2a00u64) }
        0x102d2a00 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d2a04u64).wrapping_add((imm * 4) as u64) } else { 0x102d2a04u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2a04u64) }
        0x102d2a04 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d2a08u64) } }
        0x102d2a08 => { vm.decode(0x110219b5u32); let d = 1i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102d2a0cu64) }
        0x102d2a0c => { vm.decode(0x770bb5u32); let d = 7i16; let s = 6i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d2a10u64) }
        0x102d2a10 => { vm.decode(0x121217f5u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2a14u64) }
        0x102d2a14 => { vm.decode(0x3001ad35u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102d2a18u64) }
        0x102d2a18 => { vm.decode(0x121347f5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2a1cu64) }
        0x102d2a1c => { vm.decode(0x408c875u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2a20u64) }
        0x102d2a20 => { vm.decode(0x70bb5u32); let d = 5i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d2a24u64) }
        0x102d2a24 => { vm.decode(0x8a2324f5u32); let d = 6i16; let s = 5i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2a28u64) }
        0x102d2a28 => { vm.decode(0x40220033u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d2a2cu64) }
        0x102d2a2c => { vm.decode(0x226124f5u32); let d = 1i16; let s = 4i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2a30u64) }
        0x102d2a30 => { vm.decode(0x800023eu32); let d = 4i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2a34u64) }
        0x102d2a34 => { vm.decode(0x2002236u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2a38u64) }
        0x102d2a38 => { vm.decode(0x23000446u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2a3cu64) }
        0x102d2a3c => { vm.decode(0x40227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d2a40u64).wrapping_add((imm * 4) as u64) } else { 0x102d2a40u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2a40u64) }
        0x102d2a40 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2a44u64) }
        0x102d2a44 => { vm.decode(0xa603236u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2a48u64) }
        0x102d2a48 => { vm.decode(0xd2ca416du32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d2a4cu64) }
        0x102d2a4c => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d2a50u64) }
        0x102d2a50 => { vm.decode(0x80204118u32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d2a54u64) }
        0x102d2a54 => { vm.decode(0x23000446u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2a58u64) }
        0x102d2a58 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d2a5cu64).wrapping_add((imm * 4) as u64) } else { 0x102d2a5cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2a5cu64) }
        0x102d2a5c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d2a60u64) } }
        0x102d2a60 => { vm.decode(0x1000036u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2a64u64) }
        0x102d2a64 => { vm.decode(0x80000446u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2a68u64) }
        0x102d2a68 => { vm.decode(0x679cb6a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -67i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d2a6cu64).wrapping_add((imm * 4) as u64) } else { 0x102d2a6cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2a6cu64) }
        0x102d2a6c => { vm.decode(0x2002236u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2a70u64) }
        0x102d2a70 => { vm.decode(0x49000036u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2a74u64) }
        0x102d2a74 => { vm.decode(0xd2cac16du32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d2a78u64) }
        0x102d2a78 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d2a7cu64) }
        0x102d2a7c => { vm.decode(0xa0204118u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d2a80u64) }
        0x102d2a80 => { vm.decode(0x80000446u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 4i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2a84u64) }
        0x102d2a84 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d2a88u64).wrapping_add((imm * 4) as u64) } else { 0x102d2a88u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2a88u64) }
        0x102d2a88 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d2a8cu64) } }
        0x102d2a8c => { vm.decode(0x500141du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 1328i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d2a90u64) }
        0x102d2a90 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d2a94u64) } }
        0x102d2a94 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2a98u64) }
        0x102d2a98 => { vm.decode(0x628a432fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2a9cu64) }
        0x102d2a9c => { vm.decode(0xa0000400u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102d2aa0u64) }
        0x102d2aa0 => { vm.decode(0x48a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 11i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d2aa4u64).wrapping_add((imm * 4) as u64) } else { 0x102d2aa4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2aa4u64) }
        0x102d2aa4 => { vm.decode(0x44108875u32); let d = 3i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2aa8u64) }
        0x102d2aa8 => { vm.decode(0x2040003eu32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2aacu64) }
        0x102d2aac => { vm.decode(0x848103eu32); let d = 4i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2ab0u64) }
        0x102d2ab0 => { vm.decode(0x40012835u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2ab4u64) }
        0x102d2ab4 => { vm.decode(0x220833u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d2ab8u64) }
        0x102d2ab8 => { vm.decode(0x32811099u32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2abcu64) }
        0x102d2abc => { vm.decode(0x30000099u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2ac0u64) }
        0x102d2ac0 => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2ac4u64) }
        0x102d2ac4 => { vm.decode(0xa0000400u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102d2ac8u64) }
        0x102d2ac8 => { vm.decode(0x27f32df1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -9i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d2accu64).wrapping_add((imm * 4) as u64) } else { 0x102d2accu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2accu64) }
        0x102d2acc => { vm.decode(0x40809099u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2ad0u64) }
        0x102d2ad0 => { vm.decode(0xc0801118u32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d2ad4u64) }
        0x102d2ad4 => { vm.decode(0xe057b119u32); let d = 3i16; let s = 21i16; let s2 = -1i16; let imm = 254i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2ad8u64) }
        0x102d2ad8 => { vm.decode(0xe2041099u32); let d = 6i16; let s = 17i16; let s2 = -1i16; let imm = 11i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2adcu64) }
        0x102d2adc => { vm.decode(0x2020036u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2ae0u64) }
        0x102d2ae0 => { vm.decode(0x200408eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -2139095040i64;
        vm.st(d, imm as u64);
        Ok(0x102d2ae4u64) }
        0x102d2ae4 => { vm.decode(0xa0384118u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 32897i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d2ae8u64) }
        0x102d2ae8 => { vm.decode(0x20048000u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 256i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102d2aecu64) }
        0x102d2aec => { vm.decode(0x82002171u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 78i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d2af0u64).wrapping_add((imm * 4) as u64) } else { 0x102d2af0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2af0u64) }
        0x102d2af0 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2af4u64) }
        0x102d2af4 => { vm.decode(0xa800441du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 2204i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d2af8u64) }
        0x102d2afc => { vm.decode(0x108a016du32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 15i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d2b00u64) }
        0x102d2b00 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d2b04u64) }
        0x102d2b04 => { vm.decode(0x714b0159u32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2b08u64) }
        0x102d2b08 => { vm.decode(0x804184f5u32); let d = 1i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2b0cu64) }
        0x102d2b0c => { vm.decode(0x2000023eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2b10u64) }
        0x102d2b10 => { vm.decode(0x40110fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d2b14u64) }
        0x102d2b14 => { vm.decode(0xd5bfcdfeu32); let d = 24i16; let s = 2i16; let s2 = -1i16; let imm = -1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2b18u64) }
        0x102d2b18 => { vm.decode(0x95140fb5u32); let d = 24i16; let s = 5i16; let s2 = 24i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d2b1cu64) }
        0x102d2b1c => { vm.decode(0xd2ca812du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d2b20u64) }
        0x102d2b20 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d2b24u64) }
        0x102d2b24 => { vm.decode(0xa04124f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2b28u64) }
        0x102d2b28 => { vm.decode(0x2000023eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2b2cu64) }
        0x102d2b2c => { vm.decode(0x40122835u32); let d = 1i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2b30u64) }
        0x102d2b30 => { vm.decode(0x21880135u32); let d = 32i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let p = (vm.ld(s) as u32 as u64).wrapping_mul(vm.ld(s2) as u32 as u64);
        vm.st(d, (p as u32 as i32 as i64) as u64);
        vm.st(d.wrapping_add(1), ((p >> 32) as u32 as i32 as i64) as u64);
        Ok(0x102d2b34u64) }
        0x102d2b34 => { vm.decode(0x100401f5u32); let d = 24i16; let s = 33i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d2b38u64) }
        0x102d2b38 => { vm.decode(0x801099u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2b3cu64) }
        0x102d2b3c => { vm.decode(0xccf6ffd9u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2b40u64) }
        0x102d2b40 => { vm.decode(0x11858019u32); let d = 25i16; let s = 6i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2b44u64) }
        0x102d2b44 => { vm.decode(0x2413036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2b48u64) }
        0x102d2b48 => { vm.decode(0xa400036u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2b4cu64) }
        0x102d2b4c => { vm.decode(0x44684d35u32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 10i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102d2b50u64) }
        0x102d2b50 => { vm.decode(0xc400ebb5u32); let d = 16i16; let s = 24i16; let s2 = -1i16; let imm = 11i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d2b54u64) }
        0x102d2b54 => { vm.decode(0xc500abb5u32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d2b58u64) }
        0x102d2b58 => { vm.decode(0x4f000b35u32); let d = 24i16; let s = 16i16; let s2 = 24i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d2b5cu64) }
        0x102d2b5c => { vm.decode(0xd4000fb5u32); let d = 1i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d2b60u64) }
        0x102d2b60 => { vm.decode(0x9129036u32); let d = 24i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2b64u64) }
        0x102d2b64 => { vm.decode(0x129036u32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2b68u64) }
        0x102d2b68 => { vm.decode(0x8109236u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2b6cu64) }
        0x102d2b6c => { vm.decode(0x10b236u32); let d = 19i16; let s = 1i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2b70u64) }
        0x102d2b70 => { vm.decode(0xa121036u32); let d = 20i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2b74u64) }
        0x102d2b74 => { vm.decode(0x8a65c909u32); let d = 22i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d2b78u64) }
        0x102d2b78 => { vm.decode(0x692704f5u32); let d = 22i16; let s = 17i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2b7cu64) }
        0x102d2b7c => { vm.decode(0xa4c103eu32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2b80u64) }
        0x102d2b80 => { vm.decode(0x535da835u32); let d = 15i16; let s = 15i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2b84u64) }
        0x102d2b84 => { vm.decode(0xe0630433u32); let d = 15i16; let s = 6i16; let s2 = -1i16; let imm = 5i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d2b88u64) }
        0x102d2b88 => { vm.decode(0xeb274909u32); let d = 15i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d2b8cu64) }
        0x102d2b8c => { vm.decode(0x9b63c909u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d2b90u64) }
        0x102d2b90 => { vm.decode(0x9b614909u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d2b94u64) }
        0x102d2b94 => { vm.decode(0x89614909u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d2b98u64) }
        0x102d2b98 => { vm.decode(0xa9694909u32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d2b9cu64) }
        0x102d2b9c => { vm.decode(0xa101036u32); let d = 20i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2ba0u64) }
        0x102d2ba0 => { vm.decode(0xc9654909u32); let d = 20i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d2ba4u64) }
        0x102d2ba4 => { vm.decode(0x212704f5u32); let d = 20i16; let s = 17i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2ba8u64) }
        0x102d2ba8 => { vm.decode(0x844103eu32); let d = 20i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2bacu64) }
        0x102d2bac => { vm.decode(0x131d8835u32); let d = 14i16; let s = 14i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2bb0u64) }
        0x102d2bb0 => { vm.decode(0x606104b3u32); let d = 14i16; let s = 6i16; let s2 = -1i16; let imm = 6i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d2bb4u64) }
        0x102d2bb4 => { vm.decode(0x298314f5u32); let d = 14i16; let s = 17i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2bb8u64) }
        0x102d2bb8 => { vm.decode(0x218504f5u32); let d = 24i16; let s = 16i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2bbcu64) }
        0x102d2bbc => { vm.decode(0x610504f5u32); let d = 16i16; let s = 17i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2bc0u64) }
        0x102d2bc0 => { vm.decode(0xe90504f5u32); let d = 18i16; let s = 17i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2bc4u64) }
        0x102d2bc4 => { vm.decode(0xe8e314f5u32); let d = 15i16; let s = 15i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2bc8u64) }
        0x102d2bc8 => { vm.decode(0x23036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2bccu64) }
        0x102d2bcc => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d2bd0u64) }
        0x102d2bd0 => { vm.decode(0xa04104f5u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2bd4u64) }
        0x102d2bd4 => { vm.decode(0x2000023eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2bd8u64) }
        0x102d2bd8 => { vm.decode(0x7000a835u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2bdcu64) }
        0x102d2bdc => { vm.decode(0xc02204b3u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 7i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d2be0u64) }
        0x102d2be0 => { vm.decode(0x2008323eu32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2be4u64) }
        0x102d2be4 => { vm.decode(0x3000a835u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2be8u64) }
        0x102d2be8 => { vm.decode(0x40220133u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d2becu64) }
        0x102d2bec => { vm.decode(0x2048003eu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2bf0u64) }
        0x102d2bf0 => { vm.decode(0x70002835u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2bf4u64) }
        0x102d2bf4 => { vm.decode(0xc0220133u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 9i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d2bf8u64) }
        0x102d2bf8 => { vm.decode(0x2040003eu32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2bfcu64) }
        0x102d2bfc => { vm.decode(0x70042835u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2c00u64) }
        0x102d2c00 => { vm.decode(0x402201b3u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 10i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d2c04u64) }
        0x102d2c04 => { vm.decode(0x2040203eu32); let d = 1i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2c08u64) }
        0x102d2c08 => { vm.decode(0x30042835u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2c0cu64) }
        0x102d2c0c => { vm.decode(0xc02201b3u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 11i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d2c10u64) }
        0x102d2c10 => { vm.decode(0x2008303eu32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2c14u64) }
        0x102d2c14 => { vm.decode(0x30002835u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2c18u64) }
        0x102d2c18 => { vm.decode(0x40220533u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d2c1cu64) }
        0x102d2c1c => { vm.decode(0x20048000u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 256i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102d2c20u64) }
        0x102d2c20 => { vm.decode(0x4040a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 42i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d2c24u64).wrapping_add((imm * 4) as u64) } else { 0x102d2c24u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2c24u64) }
        0x102d2c24 => { vm.decode(0x44b00875u32); let d = 6i16; let s = 0i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2c28u64) }
        0x102d2c28 => { vm.decode(0xa107eu32); let d = 8i16; let s = 6i16; let s2 = -1i16; let imm = 4i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2c2cu64) }
        0x102d2c2c => { vm.decode(0xd7bbdd7eu32); let d = 10i16; let s = 6i16; let s2 = -1i16; let imm = -2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2c30u64) }
        0x102d2c30 => { vm.decode(0x320a10beu32); let d = 11i16; let s = 6i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2c34u64) }
        0x102d2c34 => { vm.decode(0xf5bbddfeu32); let d = 9i16; let s = 6i16; let s2 = -1i16; let imm = -1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2c38u64) }
        0x102d2c38 => { vm.decode(0x80a103eu32); let d = 12i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2c3cu64) }
        0x102d2c3c => { vm.decode(0xa0a10beu32); let d = 14i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2c40u64) }
        0x102d2c40 => { vm.decode(0x380a103eu32); let d = 13i16; let s = 6i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2c44u64) }
        0x102d2c44 => { vm.decode(0xefbbddfeu32); let d = 15i16; let s = 6i16; let s2 = -1i16; let imm = -3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2c48u64) }
        0x102d2c48 => { vm.decode(0x82016du32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d2c4cu64) }
        0x102d2c4c => { vm.decode(0xa7d32cb1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -85i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d2c50u64).wrapping_add((imm * 4) as u64) } else { 0x102d2c50u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2c50u64) }
        0x102d2c50 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d2c54u64) } }
        0x102d2c54 => { vm.decode(0xd2ca112du32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d2c58u64) }
        0x102d2c58 => { vm.decode(0xda5a512du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d2c5cu64) }
        0x102d2c5c => { vm.decode(0xdadad12du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d2c60u64) }
        0x102d2c60 => { vm.decode(0xda4a91adu32); let d = 24i16; let s = 9i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d2c64u64) }
        0x102d2c64 => { vm.decode(0xda4e116du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d2c68u64) }
        0x102d2c68 => { vm.decode(0xda4a916du32); let d = 8i16; let s = 13i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d2c6cu64) }
        0x102d2c6c => { vm.decode(0xdaca516du32); let d = 9i16; let s = 14i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d2c70u64) }
        0x102d2c70 => { vm.decode(0xdaced16du32); let d = 13i16; let s = 15i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d2c74u64) }
        0x102d2c74 => { vm.decode(0xa04124f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2c78u64) }
        0x102d2c78 => { vm.decode(0x208134f5u32); let d = 8i16; let s = 8i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2c7cu64) }
        0x102d2c7c => { vm.decode(0x2000023eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2c80u64) }
        0x102d2c80 => { vm.decode(0xa0a003eu32); let d = 14i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2c84u64) }
        0x102d2c84 => { vm.decode(0xa0c534f5u32); let d = 25i16; let s = 9i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2c88u64) }
        0x102d2c88 => { vm.decode(0x3204a835u32); let d = 9i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2c8cu64) }
        0x102d2c8c => { vm.decode(0x2203eu32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2c90u64) }
        0x102d2c90 => { vm.decode(0xe04134f5u32); let d = 1i16; let s = 11i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2c94u64) }
        0x102d2c94 => { vm.decode(0x688334f5u32); let d = 14i16; let s = 10i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2c98u64) }
        0x102d2c98 => { vm.decode(0xa8e134f5u32); let d = 11i16; let s = 13i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2c9cu64) }
        0x102d2c9c => { vm.decode(0x28a134f5u32); let d = 10i16; let s = 12i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2ca0u64) }
        0x102d2ca0 => { vm.decode(0x20a203eu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2ca4u64) }
        0x102d2ca4 => { vm.decode(0x80a023eu32); let d = 12i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2ca8u64) }
        0x102d2ca8 => { vm.decode(0x134aa835u32); let d = 15i16; let s = 10i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2cacu64) }
        0x102d2cac => { vm.decode(0x242223eu32); let d = 10i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2cb0u64) }
        0x102d2cb0 => { vm.decode(0x220a223eu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2cb4u64) }
        0x102d2cb4 => { vm.decode(0x80a303eu32); let d = 12i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2cb8u64) }
        0x102d2cb8 => { vm.decode(0x2802023eu32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2cbcu64) }
        0x102d2cbc => { vm.decode(0x214134f5u32); let d = 1i16; let s = 21i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2cc0u64) }
        0x102d2cc0 => { vm.decode(0xa02023eu32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d2cc4u64) }
        0x102d2cc4 => { vm.decode(0xaca0001du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 1740i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d2cc8u64) }
        0x102d2cc8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d2cccu64) } }
        0x102d2ccc => { vm.decode(0x1395738eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -6684672i64;
        vm.st(d, imm as u64);
        Ok(0x102d2cd0u64) }
        0x102d2cd0 => { vm.decode(0x21482518u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 6344i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d2cd4u64) }
        0x102d2cd4 => { vm.decode(0x888432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2cd8u64) }
        0x102d2cd8 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2cdcu64) }
        0x102d2cdc => { vm.decode(0x8012fu32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2ce0u64) }
        0x102d2ce0 => { vm.decode(0x2236u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2ce4u64) }
        0x102d2ce4 => { vm.decode(0x22800406u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2ce8u64) }
        0x102d2ce8 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d2cecu64).wrapping_add((imm * 4) as u64) } else { 0x102d2cecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2cecu64) }
        0x102d2cec => { vm.decode(0x8e766fd9u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2cf0u64) }
        0x102d2cf0 => { vm.decode(0x8520401du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 3296i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d2cf4u64) }
        0x102d2cf8 => { vm.decode(0x2002a835u32); let d = 1i16; let s = 4i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2cfcu64) }
        0x102d2cfc => { vm.decode(0x1a0a0d75u32); let d = 8i16; let s = 6i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d2d00u64) }
        0x102d2d00 => { vm.decode(0x20a07f5u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2d04u64) }
        0x102d2d04 => { vm.decode(0x560140f5u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d2d08u64) }
        0x102d2d08 => { vm.decode(0x561147f5u32); let d = 6i16; let s = 6i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2d0cu64) }
        0x102d2d0c => { vm.decode(0x8d875u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2d10u64) }
        0x102d2d10 => { vm.decode(0x826204f5u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2d14u64) }
        0x102d2d14 => { vm.decode(0x10001430u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2d18u64) }
        0x102d2d18 => { vm.decode(0x8203236u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2d1cu64) }
        0x102d2d1c => { vm.decode(0x22800406u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2d20u64) }
        0x102d2d20 => { vm.decode(0x140c8aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 251i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d2d24u64).wrapping_add((imm * 4) as u64) } else { 0x102d2d24u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2d24u64) }
        0x102d2d24 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d2d28u64) } }
        0x102d2d28 => { vm.decode(0x4878bb5u32); let d = 5i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d2d2cu64) }
        0x102d2d2c => { vm.decode(0xb000bd75u32); let d = 6i16; let s = 5i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d2d30u64) }
        0x102d2d30 => { vm.decode(0x606104f5u32); let d = 1i16; let s = 6i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2d34u64) }
        0x102d2d34 => { vm.decode(0x200a012fu32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2d38u64) }
        0x102d2d38 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2d3cu64) }
        0x102d2d3c => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2d40u64) }
        0x102d2d40 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d2d44u64).wrapping_add((imm * 4) as u64) } else { 0x102d2d44u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2d44u64) }
        0x102d2d44 => { vm.decode(0x1000019u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2d48u64) }
        0x102d2d48 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2d4cu64) }
        0x102d2d4c => { vm.decode(0x21400d75u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d2d50u64) }
        0x102d2d50 => { vm.decode(0xc118u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d2d54u64) }
        0x102d2d54 => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2d58u64) }
        0x102d2d58 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d2d5cu64).wrapping_add((imm * 4) as u64) } else { 0x102d2d5cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2d5cu64) }
        0x102d2d5c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d2d60u64) } }
        0x102d2d60 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2d64u64) }
        0x102d2d64 => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2d68u64) }
        0x102d2d68 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d2d6cu64).wrapping_add((imm * 4) as u64) } else { 0x102d2d6cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2d6cu64) }
        0x102d2d6c => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2d70u64) }
        0x102d2d70 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2d74u64) }
        0x102d2d74 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d2d78u64) }
        0x102d2d78 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d2d7cu64) }
        0x102d2d7c => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2d80u64) }
        0x102d2d80 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d2d84u64).wrapping_add((imm * 4) as u64) } else { 0x102d2d84u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2d84u64) }
        0x102d2d84 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d2d88u64) } }
        0x102d2d88 => { vm.decode(0x42a835u32); let d = 1i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2d8cu64) }
        0x102d2d8c => { vm.decode(0x1a4a2d75u32); let d = 9i16; let s = 7i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d2d90u64) }
        0x102d2d90 => { vm.decode(0x20a17f5u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2d94u64) }
        0x102d2d94 => { vm.decode(0x562340f5u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d2d98u64) }
        0x102d2d98 => { vm.decode(0x505257f5u32); let d = 7i16; let s = 7i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2d9cu64) }
        0x102d2d9c => { vm.decode(0x408d875u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2da0u64) }
        0x102d2da0 => { vm.decode(0x4a2204f5u32); let d = 6i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2da4u64) }
        0x102d2da4 => { vm.decode(0x10401030u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2da8u64) }
        0x102d2da8 => { vm.decode(0x8403036u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2dacu64) }
        0x102d2dac => { vm.decode(0xa000bd75u32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d2db0u64) }
        0x102d2db0 => { vm.decode(0x606104f5u32); let d = 1i16; let s = 6i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2db4u64) }
        0x102d2db4 => { vm.decode(0x200a012fu32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2db8u64) }
        0x102d2db8 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2dbcu64) }
        0x102d2dbc => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2dc0u64) }
        0x102d2dc0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d2dc4u64).wrapping_add((imm * 4) as u64) } else { 0x102d2dc4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2dc4u64) }
        0x102d2dc4 => { vm.decode(0x1000019u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2dc8u64) }
        0x102d2dc8 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2dccu64) }
        0x102d2dcc => { vm.decode(0x21400d75u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d2dd0u64) }
        0x102d2dd0 => { vm.decode(0xc118u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d2dd4u64) }
        0x102d2dd4 => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2dd8u64) }
        0x102d2dd8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d2ddcu64).wrapping_add((imm * 4) as u64) } else { 0x102d2ddcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2ddcu64) }
        0x102d2ddc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d2de0u64) } }
        0x102d2de0 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2de4u64) }
        0x102d2de4 => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2de8u64) }
        0x102d2de8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d2decu64).wrapping_add((imm * 4) as u64) } else { 0x102d2decu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2decu64) }
        0x102d2dec => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2df0u64) }
        0x102d2df0 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2df4u64) }
        0x102d2df4 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d2df8u64) }
        0x102d2df8 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d2dfcu64) }
        0x102d2dfc => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2e00u64) }
        0x102d2e00 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d2e04u64).wrapping_add((imm * 4) as u64) } else { 0x102d2e04u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2e04u64) }
        0x102d2e04 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d2e08u64) } }
        0x102d2e08 => { vm.decode(0x42a835u32); let d = 1i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2e0cu64) }
        0x102d2e0c => { vm.decode(0x1a4a2d75u32); let d = 9i16; let s = 7i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d2e10u64) }
        0x102d2e10 => { vm.decode(0x20a17f5u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2e14u64) }
        0x102d2e14 => { vm.decode(0x562340f5u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d2e18u64) }
        0x102d2e18 => { vm.decode(0x505257f5u32); let d = 7i16; let s = 7i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2e1cu64) }
        0x102d2e1c => { vm.decode(0x408d875u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2e20u64) }
        0x102d2e20 => { vm.decode(0x240b236u32); let d = 7i16; let s = 5i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2e24u64) }
        0x102d2e24 => { vm.decode(0xb2082d75u32); let d = 8i16; let s = 7i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d2e28u64) }
        0x102d2e28 => { vm.decode(0xa4214f5u32); let d = 7i16; let s = 2i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2e2cu64) }
        0x102d2e2c => { vm.decode(0x10401430u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2e30u64) }
        0x102d2e30 => { vm.decode(0x423036u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2e34u64) }
        0x102d2e34 => { vm.decode(0xa040bd75u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d2e38u64) }
        0x102d2e38 => { vm.decode(0xe06104f5u32); let d = 1i16; let s = 7i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2e3cu64) }
        0x102d2e3c => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2e40u64) }
        0x102d2e40 => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2e44u64) }
        0x102d2e44 => { vm.decode(0xa2002446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2e48u64) }
        0x102d2e48 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d2e4cu64).wrapping_add((imm * 4) as u64) } else { 0x102d2e4cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2e4cu64) }
        0x102d2e4c => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2e50u64) }
        0x102d2e50 => { vm.decode(0x49203236u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2e54u64) }
        0x102d2e54 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d2e58u64) }
        0x102d2e58 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d2e5cu64) }
        0x102d2e5c => { vm.decode(0xa2002446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2e60u64) }
        0x102d2e60 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d2e64u64).wrapping_add((imm * 4) as u64) } else { 0x102d2e64u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2e64u64) }
        0x102d2e64 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d2e68u64) } }
        0x102d2e68 => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2e6cu64) }
        0x102d2e6c => { vm.decode(0xa2002446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2e70u64) }
        0x102d2e70 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d2e74u64).wrapping_add((imm * 4) as u64) } else { 0x102d2e74u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2e74u64) }
        0x102d2e74 => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2e78u64) }
        0x102d2e78 => { vm.decode(0x49203236u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2e7cu64) }
        0x102d2e7c => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d2e80u64) }
        0x102d2e80 => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d2e84u64) }
        0x102d2e84 => { vm.decode(0xa2002446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2e88u64) }
        0x102d2e88 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d2e8cu64).wrapping_add((imm * 4) as u64) } else { 0x102d2e8cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2e8cu64) }
        0x102d2e8c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d2e90u64) } }
        0x102d2e90 => { vm.decode(0x3a835u32); let d = 1i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2e94u64) }
        0x102d2e94 => { vm.decode(0x94a3d75u32); let d = 11i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d2e98u64) }
        0x102d2e98 => { vm.decode(0x21a17f5u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2e9cu64) }
        0x102d2e9c => { vm.decode(0x323200f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d2ea0u64) }
        0x102d2ea0 => { vm.decode(0x44e17f5u32); let d = 9i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2ea4u64) }
        0x102d2ea4 => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2ea8u64) }
        0x102d2ea8 => { vm.decode(0x940b036u32); let d = 9i16; let s = 5i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2eacu64) }
        0x102d2eac => { vm.decode(0xa1482d75u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d2eb0u64) }
        0x102d2eb0 => { vm.decode(0x8a8014f5u32); let d = 10i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2eb4u64) }
        0x102d2eb4 => { vm.decode(0x18400030u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2eb8u64) }
        0x102d2eb8 => { vm.decode(0x8423036u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2ebcu64) }
        0x102d2ebc => { vm.decode(0xa0083d75u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d2ec0u64) }
        0x102d2ec0 => { vm.decode(0x604114f5u32); let d = 1i16; let s = 10i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2ec4u64) }
        0x102d2ec4 => { vm.decode(0x2008092fu32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2ec8u64) }
        0x102d2ec8 => { vm.decode(0x3002036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2eccu64) }
        0x102d2ecc => { vm.decode(0xa1002446u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2ed0u64) }
        0x102d2ed0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d2ed4u64).wrapping_add((imm * 4) as u64) } else { 0x102d2ed4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2ed4u64) }
        0x102d2ed4 => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2ed8u64) }
        0x102d2ed8 => { vm.decode(0x4b403036u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2edcu64) }
        0x102d2edc => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d2ee0u64) }
        0x102d2ee0 => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d2ee4u64) }
        0x102d2ee4 => { vm.decode(0xa1002446u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2ee8u64) }
        0x102d2ee8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d2eecu64).wrapping_add((imm * 4) as u64) } else { 0x102d2eecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2eecu64) }
        0x102d2eec => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d2ef0u64) } }
        0x102d2ef0 => { vm.decode(0x3002036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2ef4u64) }
        0x102d2ef4 => { vm.decode(0xa1002446u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2ef8u64) }
        0x102d2ef8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d2efcu64).wrapping_add((imm * 4) as u64) } else { 0x102d2efcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2efcu64) }
        0x102d2efc => { vm.decode(0x3001019u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2f00u64) }
        0x102d2f00 => { vm.decode(0x4b403036u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2f04u64) }
        0x102d2f04 => { vm.decode(0x33400d75u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d2f08u64) }
        0x102d2f08 => { vm.decode(0x8020c118u32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d2f0cu64) }
        0x102d2f0c => { vm.decode(0xa1002446u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2f10u64) }
        0x102d2f10 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d2f14u64).wrapping_add((imm * 4) as u64) } else { 0x102d2f14u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2f14u64) }
        0x102d2f14 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d2f18u64) } }
        0x102d2f18 => { vm.decode(0x43a835u32); let d = 1i16; let s = 4i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2f1cu64) }
        0x102d2f1c => { vm.decode(0xb4aad75u32); let d = 13i16; let s = 11i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d2f20u64) }
        0x102d2f20 => { vm.decode(0x120a17f5u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2f24u64) }
        0x102d2f24 => { vm.decode(0x363300f5u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d2f28u64) }
        0x102d2f28 => { vm.decode(0x405f17f5u32); let d = 11i16; let s = 11i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2f2cu64) }
        0x102d2f2c => { vm.decode(0x4289875u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2f30u64) }
        0x102d2f30 => { vm.decode(0x4a8014f5u32); let d = 10i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2f34u64) }
        0x102d2f34 => { vm.decode(0x18400030u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2f38u64) }
        0x102d2f38 => { vm.decode(0x204114f5u32); let d = 1i16; let s = 8i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2f3cu64) }
        0x102d2f3c => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2f40u64) }
        0x102d2f40 => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2f44u64) }
        0x102d2f44 => { vm.decode(0xa2002446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2f48u64) }
        0x102d2f48 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d2f4cu64).wrapping_add((imm * 4) as u64) } else { 0x102d2f4cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2f4cu64) }
        0x102d2f4c => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2f50u64) }
        0x102d2f50 => { vm.decode(0x49203236u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2f54u64) }
        0x102d2f54 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d2f58u64) }
        0x102d2f58 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d2f5cu64) }
        0x102d2f5c => { vm.decode(0xa2002446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2f60u64) }
        0x102d2f60 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d2f64u64).wrapping_add((imm * 4) as u64) } else { 0x102d2f64u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2f64u64) }
        0x102d2f64 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d2f68u64) } }
        0x102d2f68 => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2f6cu64) }
        0x102d2f6c => { vm.decode(0xa2002446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2f70u64) }
        0x102d2f70 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d2f74u64).wrapping_add((imm * 4) as u64) } else { 0x102d2f74u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2f74u64) }
        0x102d2f74 => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2f78u64) }
        0x102d2f78 => { vm.decode(0x49203236u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2f7cu64) }
        0x102d2f7c => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d2f80u64) }
        0x102d2f80 => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d2f84u64) }
        0x102d2f84 => { vm.decode(0xa2002446u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2f88u64) }
        0x102d2f88 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d2f8cu64).wrapping_add((imm * 4) as u64) } else { 0x102d2f8cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2f8cu64) }
        0x102d2f8c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d2f90u64) } }
        0x102d2f90 => { vm.decode(0x3a835u32); let d = 1i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2f94u64) }
        0x102d2f94 => { vm.decode(0x94a1d75u32); let d = 11i16; let s = 8i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d2f98u64) }
        0x102d2f98 => { vm.decode(0x21a17f5u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2f9cu64) }
        0x102d2f9c => { vm.decode(0x321000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d2fa0u64) }
        0x102d2fa0 => { vm.decode(0x44c07f5u32); let d = 8i16; let s = 8i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2fa4u64) }
        0x102d2fa4 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d2fa8u64) }
        0x102d2fa8 => { vm.decode(0xca6204f5u32); let d = 7i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2facu64) }
        0x102d2fac => { vm.decode(0x10401430u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d2fb0u64) }
        0x102d2fb0 => { vm.decode(0xa04114f5u32); let d = 1i16; let s = 9i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d2fb4u64) }
        0x102d2fb4 => { vm.decode(0x200a012fu32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d2fb8u64) }
        0x102d2fb8 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2fbcu64) }
        0x102d2fbc => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2fc0u64) }
        0x102d2fc0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d2fc4u64).wrapping_add((imm * 4) as u64) } else { 0x102d2fc4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2fc4u64) }
        0x102d2fc4 => { vm.decode(0x1000019u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2fc8u64) }
        0x102d2fc8 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2fccu64) }
        0x102d2fcc => { vm.decode(0x21400d75u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d2fd0u64) }
        0x102d2fd0 => { vm.decode(0xc118u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d2fd4u64) }
        0x102d2fd4 => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2fd8u64) }
        0x102d2fd8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d2fdcu64).wrapping_add((imm * 4) as u64) } else { 0x102d2fdcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2fdcu64) }
        0x102d2fdc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d2fe0u64) } }
        0x102d2fe0 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2fe4u64) }
        0x102d2fe4 => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d2fe8u64) }
        0x102d2fe8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d2fecu64).wrapping_add((imm * 4) as u64) } else { 0x102d2fecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d2fecu64) }
        0x102d2fec => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d2ff0u64) }
        0x102d2ff0 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d2ff4u64) }
        0x102d2ff4 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d2ff8u64) }
        0x102d2ff8 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d2ffcu64) }
        0x102d2ffc => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3000u64) }
        0x102d3000 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3004u64).wrapping_add((imm * 4) as u64) } else { 0x102d3004u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3004u64) }
        0x102d3004 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3008u64) } }
        0x102d3008 => { vm.decode(0x42a835u32); let d = 1i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d300cu64) }
        0x102d300c => { vm.decode(0x1a4a2d75u32); let d = 9i16; let s = 7i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3010u64) }
        0x102d3010 => { vm.decode(0x20a17f5u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3014u64) }
        0x102d3014 => { vm.decode(0x562340f5u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d3018u64) }
        0x102d3018 => { vm.decode(0x505257f5u32); let d = 7i16; let s = 7i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d301cu64) }
        0x102d301c => { vm.decode(0x408d875u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3020u64) }
        0x102d3020 => { vm.decode(0x242b236u32); let d = 7i16; let s = 5i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3024u64) }
        0x102d3024 => { vm.decode(0xb240bd75u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3028u64) }
        0x102d3028 => { vm.decode(0xc2a004f5u32); let d = 8i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d302cu64) }
        0x102d302c => { vm.decode(0x18000030u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d3030u64) }
        0x102d3030 => { vm.decode(0xe06104f5u32); let d = 1i16; let s = 7i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3034u64) }
        0x102d3034 => { vm.decode(0x200a012fu32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3038u64) }
        0x102d3038 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d303cu64) }
        0x102d303c => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3040u64) }
        0x102d3040 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3044u64).wrapping_add((imm * 4) as u64) } else { 0x102d3044u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3044u64) }
        0x102d3044 => { vm.decode(0x1000019u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3048u64) }
        0x102d3048 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d304cu64) }
        0x102d304c => { vm.decode(0x21400d75u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3050u64) }
        0x102d3050 => { vm.decode(0xc118u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3054u64) }
        0x102d3054 => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3058u64) }
        0x102d3058 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d305cu64).wrapping_add((imm * 4) as u64) } else { 0x102d305cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d305cu64) }
        0x102d305c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3060u64) } }
        0x102d3060 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3064u64) }
        0x102d3064 => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3068u64) }
        0x102d3068 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d306cu64).wrapping_add((imm * 4) as u64) } else { 0x102d306cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d306cu64) }
        0x102d306c => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3070u64) }
        0x102d3070 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3074u64) }
        0x102d3074 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3078u64) }
        0x102d3078 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d307cu64) }
        0x102d307c => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3080u64) }
        0x102d3080 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3084u64).wrapping_add((imm * 4) as u64) } else { 0x102d3084u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3084u64) }
        0x102d3084 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3088u64) } }
        0x102d3088 => { vm.decode(0x42a835u32); let d = 1i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d308cu64) }
        0x102d308c => { vm.decode(0x1a4a2d75u32); let d = 9i16; let s = 7i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3090u64) }
        0x102d3090 => { vm.decode(0x20a17f5u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3094u64) }
        0x102d3094 => { vm.decode(0x562340f5u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d3098u64) }
        0x102d3098 => { vm.decode(0x505257f5u32); let d = 7i16; let s = 7i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d309cu64) }
        0x102d309c => { vm.decode(0x408d875u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d30a0u64) }
        0x102d30a0 => { vm.decode(0x4a2204f5u32); let d = 6i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d30a4u64) }
        0x102d30a4 => { vm.decode(0x10401030u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d30a8u64) }
        0x102d30a8 => { vm.decode(0x842b036u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d30acu64) }
        0x102d30ac => { vm.decode(0xa040ad75u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d30b0u64) }
        0x102d30b0 => { vm.decode(0xa06104f5u32); let d = 1i16; let s = 5i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d30b4u64) }
        0x102d30b4 => { vm.decode(0xa012fu32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d30b8u64) }
        0x102d30b8 => { vm.decode(0x1000036u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d30bcu64) }
        0x102d30bc => { vm.decode(0x80002446u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d30c0u64) }
        0x102d30c0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d30c4u64).wrapping_add((imm * 4) as u64) } else { 0x102d30c4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d30c4u64) }
        0x102d30c4 => { vm.decode(0x2009019u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d30c8u64) }
        0x102d30c8 => { vm.decode(0x49000036u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d30ccu64) }
        0x102d30cc => { vm.decode(0x32402d75u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d30d0u64) }
        0x102d30d0 => { vm.decode(0xa0204118u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d30d4u64) }
        0x102d30d4 => { vm.decode(0x80002446u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d30d8u64) }
        0x102d30d8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d30dcu64).wrapping_add((imm * 4) as u64) } else { 0x102d30dcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d30dcu64) }
        0x102d30dc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d30e0u64) } }
        0x102d30e0 => { vm.decode(0x1000036u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d30e4u64) }
        0x102d30e4 => { vm.decode(0x80002446u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d30e8u64) }
        0x102d30e8 => { vm.decode(0x6314bc27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -253i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d30ecu64).wrapping_add((imm * 4) as u64) } else { 0x102d30ecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d30ecu64) }
        0x102d30ec => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d30f0u64) }
        0x102d30f0 => { vm.decode(0x49000036u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d30f4u64) }
        0x102d30f4 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d30f8u64) }
        0x102d30f8 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d30fcu64) }
        0x102d30fc => { vm.decode(0x80002446u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3100u64) }
        0x102d3100 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3104u64).wrapping_add((imm * 4) as u64) } else { 0x102d3104u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3104u64) }
        0x102d3104 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3108u64) } }
        0x102d3108 => { vm.decode(0x8820401du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 2248i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d310cu64) }
        0x102d310c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3110u64) } }
        0x102d3110 => { vm.decode(0x1395738eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -6684672i64;
        vm.st(d, imm as u64);
        Ok(0x102d3114u64) }
        0x102d3114 => { vm.decode(0x21482518u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 6344i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3118u64) }
        0x102d3118 => { vm.decode(0x888632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d311cu64) }
        0x102d311c => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3120u64) }
        0x102d3120 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3124u64) }
        0x102d3124 => { vm.decode(0x2888432fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3128u64) }
        0x102d3128 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d312cu64) }
        0x102d312c => { vm.decode(0x2008012fu32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3130u64) }
        0x102d3130 => { vm.decode(0x2000036u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3134u64) }
        0x102d3134 => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3138u64) }
        0x102d3138 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d313cu64).wrapping_add((imm * 4) as u64) } else { 0x102d313cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d313cu64) }
        0x102d313c => { vm.decode(0x8e76efd9u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3140u64) }
        0x102d3140 => { vm.decode(0x104141du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 4400i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d3144u64) }
        0x102d3148 => { vm.decode(0x4042a835u32); let d = 1i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d314cu64) }
        0x102d314c => { vm.decode(0x1a4a2d75u32); let d = 9i16; let s = 7i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3150u64) }
        0x102d3150 => { vm.decode(0x20a17f5u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3154u64) }
        0x102d3154 => { vm.decode(0x562340f5u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d3158u64) }
        0x102d3158 => { vm.decode(0x505257f5u32); let d = 7i16; let s = 7i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d315cu64) }
        0x102d315c => { vm.decode(0x408d875u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3160u64) }
        0x102d3160 => { vm.decode(0x4a2204f5u32); let d = 6i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3164u64) }
        0x102d3164 => { vm.decode(0x10401030u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d3168u64) }
        0x102d3168 => { vm.decode(0xa400036u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d316cu64) }
        0x102d316c => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3170u64) }
        0x102d3170 => { vm.decode(0x140c8aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 251i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3174u64).wrapping_add((imm * 4) as u64) } else { 0x102d3174u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3174u64) }
        0x102d3174 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3178u64) } }
        0x102d3178 => { vm.decode(0x4648bb5u32); let d = 6i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d317cu64) }
        0x102d317c => { vm.decode(0xb2409d75u32); let d = 7i16; let s = 6i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3180u64) }
        0x102d3180 => { vm.decode(0xe26004f5u32); let d = 1i16; let s = 3i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3184u64) }
        0x102d3184 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3188u64) }
        0x102d3188 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d318cu64) }
        0x102d318c => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3190u64) }
        0x102d3190 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3194u64).wrapping_add((imm * 4) as u64) } else { 0x102d3194u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3194u64) }
        0x102d3194 => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3198u64) }
        0x102d3198 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d319cu64) }
        0x102d319c => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d31a0u64) }
        0x102d31a0 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d31a4u64) }
        0x102d31a4 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d31a8u64) }
        0x102d31a8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d31acu64).wrapping_add((imm * 4) as u64) } else { 0x102d31acu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d31acu64) }
        0x102d31ac => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d31b0u64) } }
        0x102d31b0 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d31b4u64) }
        0x102d31b4 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d31b8u64) }
        0x102d31b8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d31bcu64).wrapping_add((imm * 4) as u64) } else { 0x102d31bcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d31bcu64) }
        0x102d31bc => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d31c0u64) }
        0x102d31c0 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d31c4u64) }
        0x102d31c4 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d31c8u64) }
        0x102d31c8 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d31ccu64) }
        0x102d31cc => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d31d0u64) }
        0x102d31d0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d31d4u64).wrapping_add((imm * 4) as u64) } else { 0x102d31d4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d31d4u64) }
        0x102d31d4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d31d8u64) } }
        0x102d31d8 => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d31dcu64) }
        0x102d31dc => { vm.decode(0x90a1d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d31e0u64) }
        0x102d31e0 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d31e4u64) }
        0x102d31e4 => { vm.decode(0x321000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d31e8u64) }
        0x102d31e8 => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d31ecu64) }
        0x102d31ec => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d31f0u64) }
        0x102d31f0 => { vm.decode(0xca6204f5u32); let d = 7i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d31f4u64) }
        0x102d31f4 => { vm.decode(0x10401430u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d31f8u64) }
        0x102d31f8 => { vm.decode(0x8602036u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d31fcu64) }
        0x102d31fc => { vm.decode(0xa040bd75u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3200u64) }
        0x102d3200 => { vm.decode(0xe26004f5u32); let d = 1i16; let s = 3i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3204u64) }
        0x102d3204 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3208u64) }
        0x102d3208 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d320cu64) }
        0x102d320c => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3210u64) }
        0x102d3210 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3214u64).wrapping_add((imm * 4) as u64) } else { 0x102d3214u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3214u64) }
        0x102d3214 => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3218u64) }
        0x102d3218 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d321cu64) }
        0x102d321c => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3220u64) }
        0x102d3220 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3224u64) }
        0x102d3224 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3228u64) }
        0x102d3228 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d322cu64).wrapping_add((imm * 4) as u64) } else { 0x102d322cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d322cu64) }
        0x102d322c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3230u64) } }
        0x102d3230 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3234u64) }
        0x102d3234 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3238u64) }
        0x102d3238 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d323cu64).wrapping_add((imm * 4) as u64) } else { 0x102d323cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d323cu64) }
        0x102d323c => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3240u64) }
        0x102d3240 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3244u64) }
        0x102d3244 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3248u64) }
        0x102d3248 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d324cu64) }
        0x102d324c => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3250u64) }
        0x102d3250 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3254u64).wrapping_add((imm * 4) as u64) } else { 0x102d3254u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3254u64) }
        0x102d3254 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3258u64) } }
        0x102d3258 => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d325cu64) }
        0x102d325c => { vm.decode(0x90a1d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3260u64) }
        0x102d3260 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3264u64) }
        0x102d3264 => { vm.decode(0x321000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d3268u64) }
        0x102d3268 => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d326cu64) }
        0x102d326c => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3270u64) }
        0x102d3270 => { vm.decode(0x1608036u32); let d = 8i16; let s = 6i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3274u64) }
        0x102d3274 => { vm.decode(0xa1480d75u32); let d = 9i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3278u64) }
        0x102d3278 => { vm.decode(0x828014f5u32); let d = 8i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d327cu64) }
        0x102d327c => { vm.decode(0x18000030u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d3280u64) }
        0x102d3280 => { vm.decode(0x622036u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3284u64) }
        0x102d3284 => { vm.decode(0xa0082d75u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3288u64) }
        0x102d3288 => { vm.decode(0x224014f5u32); let d = 1i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d328cu64) }
        0x102d328c => { vm.decode(0x8092fu32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3290u64) }
        0x102d3290 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3294u64) }
        0x102d3294 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3298u64) }
        0x102d3298 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d329cu64).wrapping_add((imm * 4) as u64) } else { 0x102d329cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d329cu64) }
        0x102d329c => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d32a0u64) }
        0x102d32a0 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d32a4u64) }
        0x102d32a4 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d32a8u64) }
        0x102d32a8 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d32acu64) }
        0x102d32ac => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d32b0u64) }
        0x102d32b0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d32b4u64).wrapping_add((imm * 4) as u64) } else { 0x102d32b4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d32b4u64) }
        0x102d32b4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d32b8u64) } }
        0x102d32b8 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d32bcu64) }
        0x102d32bc => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d32c0u64) }
        0x102d32c0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d32c4u64).wrapping_add((imm * 4) as u64) } else { 0x102d32c4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d32c4u64) }
        0x102d32c4 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d32c8u64) }
        0x102d32c8 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d32ccu64) }
        0x102d32cc => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d32d0u64) }
        0x102d32d0 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d32d4u64) }
        0x102d32d4 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d32d8u64) }
        0x102d32d8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d32dcu64).wrapping_add((imm * 4) as u64) } else { 0x102d32dcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d32dcu64) }
        0x102d32dc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d32e0u64) } }
        0x102d32e0 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d32e4u64) }
        0x102d32e4 => { vm.decode(0xb0a8d75u32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d32e8u64) }
        0x102d32e8 => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d32ecu64) }
        0x102d32ec => { vm.decode(0x361100f5u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d32f0u64) }
        0x102d32f0 => { vm.decode(0x465c07f5u32); let d = 10i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d32f4u64) }
        0x102d32f4 => { vm.decode(0x289875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d32f8u64) }
        0x102d32f8 => { vm.decode(0x9608236u32); let d = 10i16; let s = 6i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d32fcu64) }
        0x102d32fc => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3300u64) }
        0x102d3300 => { vm.decode(0x4ac014f5u32); let d = 11i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3304u64) }
        0x102d3304 => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d3308u64) }
        0x102d3308 => { vm.decode(0x8622036u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d330cu64) }
        0x102d330c => { vm.decode(0xa0483d75u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3310u64) }
        0x102d3310 => { vm.decode(0xe24014f5u32); let d = 1i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3314u64) }
        0x102d3314 => { vm.decode(0x2092fu32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3318u64) }
        0x102d3318 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d331cu64) }
        0x102d331c => { vm.decode(0x83002446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3320u64) }
        0x102d3320 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3324u64).wrapping_add((imm * 4) as u64) } else { 0x102d3324u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3324u64) }
        0x102d3324 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3328u64) }
        0x102d3328 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d332cu64) }
        0x102d332c => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3330u64) }
        0x102d3330 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3334u64) }
        0x102d3334 => { vm.decode(0x83002446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3338u64) }
        0x102d3338 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d333cu64).wrapping_add((imm * 4) as u64) } else { 0x102d333cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d333cu64) }
        0x102d333c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3340u64) } }
        0x102d3340 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3344u64) }
        0x102d3344 => { vm.decode(0x83002446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3348u64) }
        0x102d3348 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d334cu64).wrapping_add((imm * 4) as u64) } else { 0x102d334cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d334cu64) }
        0x102d334c => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3350u64) }
        0x102d3350 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3354u64) }
        0x102d3354 => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3358u64) }
        0x102d3358 => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d335cu64) }
        0x102d335c => { vm.decode(0x83002446u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3360u64) }
        0x102d3360 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3364u64).wrapping_add((imm * 4) as u64) } else { 0x102d3364u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3364u64) }
        0x102d3364 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3368u64) } }
        0x102d3368 => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d336cu64) }
        0x102d336c => { vm.decode(0x190a9d75u32); let d = 14i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3370u64) }
        0x102d3370 => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3374u64) }
        0x102d3374 => { vm.decode(0x721040f5u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d3378u64) }
        0x102d3378 => { vm.decode(0x124d47f5u32); let d = 12i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d337cu64) }
        0x102d337c => { vm.decode(0x28c875u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3380u64) }
        0x102d3380 => { vm.decode(0xcac014f5u32); let d = 11i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3384u64) }
        0x102d3384 => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d3388u64) }
        0x102d3388 => { vm.decode(0xa24014f5u32); let d = 1i16; let s = 3i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d338cu64) }
        0x102d338c => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3390u64) }
        0x102d3390 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3394u64) }
        0x102d3394 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3398u64) }
        0x102d3398 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d339cu64).wrapping_add((imm * 4) as u64) } else { 0x102d339cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d339cu64) }
        0x102d339c => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d33a0u64) }
        0x102d33a0 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d33a4u64) }
        0x102d33a4 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d33a8u64) }
        0x102d33a8 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d33acu64) }
        0x102d33ac => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d33b0u64) }
        0x102d33b0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d33b4u64).wrapping_add((imm * 4) as u64) } else { 0x102d33b4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d33b4u64) }
        0x102d33b4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d33b8u64) } }
        0x102d33b8 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d33bcu64) }
        0x102d33bc => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d33c0u64) }
        0x102d33c0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d33c4u64).wrapping_add((imm * 4) as u64) } else { 0x102d33c4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d33c4u64) }
        0x102d33c4 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d33c8u64) }
        0x102d33c8 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d33ccu64) }
        0x102d33cc => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d33d0u64) }
        0x102d33d0 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d33d4u64) }
        0x102d33d4 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d33d8u64) }
        0x102d33d8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d33dcu64).wrapping_add((imm * 4) as u64) } else { 0x102d33dcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d33dcu64) }
        0x102d33dc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d33e0u64) } }
        0x102d33e0 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d33e4u64) }
        0x102d33e4 => { vm.decode(0x90aad75u32); let d = 12i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d33e8u64) }
        0x102d33e8 => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d33ecu64) }
        0x102d33ec => { vm.decode(0x323200f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d33f0u64) }
        0x102d33f0 => { vm.decode(0x64e17f5u32); let d = 9i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d33f4u64) }
        0x102d33f4 => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d33f8u64) }
        0x102d33f8 => { vm.decode(0x28014f5u32); let d = 8i16; let s = 2i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d33fcu64) }
        0x102d33fc => { vm.decode(0x18000030u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d3400u64) }
        0x102d3400 => { vm.decode(0x624014f5u32); let d = 1i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3404u64) }
        0x102d3404 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3408u64) }
        0x102d3408 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d340cu64) }
        0x102d340c => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3410u64) }
        0x102d3410 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3414u64).wrapping_add((imm * 4) as u64) } else { 0x102d3414u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3414u64) }
        0x102d3414 => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3418u64) }
        0x102d3418 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d341cu64) }
        0x102d341c => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3420u64) }
        0x102d3420 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3424u64) }
        0x102d3424 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3428u64) }
        0x102d3428 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d342cu64).wrapping_add((imm * 4) as u64) } else { 0x102d342cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d342cu64) }
        0x102d342c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3430u64) } }
        0x102d3430 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3434u64) }
        0x102d3434 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3438u64) }
        0x102d3438 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d343cu64).wrapping_add((imm * 4) as u64) } else { 0x102d343cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d343cu64) }
        0x102d343c => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3440u64) }
        0x102d3440 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3444u64) }
        0x102d3444 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3448u64) }
        0x102d3448 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d344cu64) }
        0x102d344c => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3450u64) }
        0x102d3450 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3454u64).wrapping_add((imm * 4) as u64) } else { 0x102d3454u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3454u64) }
        0x102d3454 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3458u64) } }
        0x102d3458 => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d345cu64) }
        0x102d345c => { vm.decode(0x90a1d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3460u64) }
        0x102d3460 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3464u64) }
        0x102d3464 => { vm.decode(0x321000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d3468u64) }
        0x102d3468 => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d346cu64) }
        0x102d346c => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3470u64) }
        0x102d3470 => { vm.decode(0x1628036u32); let d = 8i16; let s = 6i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3474u64) }
        0x102d3474 => { vm.decode(0xa1080d75u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3478u64) }
        0x102d3478 => { vm.decode(0x2c014f5u32); let d = 9i16; let s = 2i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d347cu64) }
        0x102d347c => { vm.decode(0x18000430u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d3480u64) }
        0x102d3480 => { vm.decode(0x224014f5u32); let d = 1i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3484u64) }
        0x102d3484 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3488u64) }
        0x102d3488 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d348cu64) }
        0x102d348c => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3490u64) }
        0x102d3490 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3494u64).wrapping_add((imm * 4) as u64) } else { 0x102d3494u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3494u64) }
        0x102d3494 => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3498u64) }
        0x102d3498 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d349cu64) }
        0x102d349c => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d34a0u64) }
        0x102d34a0 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d34a4u64) }
        0x102d34a4 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d34a8u64) }
        0x102d34a8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d34acu64).wrapping_add((imm * 4) as u64) } else { 0x102d34acu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d34acu64) }
        0x102d34ac => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d34b0u64) } }
        0x102d34b0 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d34b4u64) }
        0x102d34b4 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d34b8u64) }
        0x102d34b8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d34bcu64).wrapping_add((imm * 4) as u64) } else { 0x102d34bcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d34bcu64) }
        0x102d34bc => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d34c0u64) }
        0x102d34c0 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d34c4u64) }
        0x102d34c4 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d34c8u64) }
        0x102d34c8 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d34ccu64) }
        0x102d34cc => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d34d0u64) }
        0x102d34d0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d34d4u64).wrapping_add((imm * 4) as u64) } else { 0x102d34d4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d34d4u64) }
        0x102d34d4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d34d8u64) } }
        0x102d34d8 => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d34dcu64) }
        0x102d34dc => { vm.decode(0x90a1d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d34e0u64) }
        0x102d34e0 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d34e4u64) }
        0x102d34e4 => { vm.decode(0x321000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d34e8u64) }
        0x102d34e8 => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d34ecu64) }
        0x102d34ec => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d34f0u64) }
        0x102d34f0 => { vm.decode(0xca6204f5u32); let d = 7i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d34f4u64) }
        0x102d34f4 => { vm.decode(0x10401430u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d34f8u64) }
        0x102d34f8 => { vm.decode(0x862a036u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d34fcu64) }
        0x102d34fc => { vm.decode(0xa000bd75u32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3500u64) }
        0x102d3500 => { vm.decode(0x626004f5u32); let d = 1i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3504u64) }
        0x102d3504 => { vm.decode(0x200a012fu32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3508u64) }
        0x102d3508 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d350cu64) }
        0x102d350c => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3510u64) }
        0x102d3510 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3514u64).wrapping_add((imm * 4) as u64) } else { 0x102d3514u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3514u64) }
        0x102d3514 => { vm.decode(0x1000019u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3518u64) }
        0x102d3518 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d351cu64) }
        0x102d351c => { vm.decode(0x21400d75u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3520u64) }
        0x102d3520 => { vm.decode(0xc118u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3524u64) }
        0x102d3524 => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3528u64) }
        0x102d3528 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d352cu64).wrapping_add((imm * 4) as u64) } else { 0x102d352cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d352cu64) }
        0x102d352c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3530u64) } }
        0x102d3530 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3534u64) }
        0x102d3534 => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3538u64) }
        0x102d3538 => { vm.decode(0x6314bc27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -253i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d353cu64).wrapping_add((imm * 4) as u64) } else { 0x102d353cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d353cu64) }
        0x102d353c => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3540u64) }
        0x102d3540 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3544u64) }
        0x102d3544 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3548u64) }
        0x102d3548 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d354cu64) }
        0x102d354c => { vm.decode(0xa0002446u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3550u64) }
        0x102d3550 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3554u64).wrapping_add((imm * 4) as u64) } else { 0x102d3554u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3554u64) }
        0x102d3554 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3558u64) } }
        0x102d3558 => { vm.decode(0xc00541du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 3352i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d355cu64) }
        0x102d355c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3560u64) } }
        0x102d3560 => { vm.decode(0x1395738eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -6684672i64;
        vm.st(d, imm as u64);
        Ok(0x102d3564u64) }
        0x102d3564 => { vm.decode(0x21482518u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 6344i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3568u64) }
        0x102d3568 => { vm.decode(0x8a01a4f5u32); let d = 2i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d356cu64) }
        0x102d356c => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3570u64) }
        0x102d3570 => { vm.decode(0x2888632fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3574u64) }
        0x102d3574 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3578u64) }
        0x102d3578 => { vm.decode(0x2008012fu32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d357cu64) }
        0x102d357c => { vm.decode(0x2000036u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3580u64) }
        0x102d3580 => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3584u64) }
        0x102d3584 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3588u64).wrapping_add((imm * 4) as u64) } else { 0x102d3588u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3588u64) }
        0x102d3588 => { vm.decode(0x8e76efd9u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d358cu64) }
        0x102d358c => { vm.decode(0x2d24141du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 5500i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d3590u64) }
        0x102d3594 => { vm.decode(0x4042a835u32); let d = 1i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3598u64) }
        0x102d3598 => { vm.decode(0x1a4a2d75u32); let d = 9i16; let s = 7i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d359cu64) }
        0x102d359c => { vm.decode(0x20a17f5u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d35a0u64) }
        0x102d35a0 => { vm.decode(0x562340f5u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d35a4u64) }
        0x102d35a4 => { vm.decode(0x505257f5u32); let d = 7i16; let s = 7i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d35a8u64) }
        0x102d35a8 => { vm.decode(0x408d875u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d35acu64) }
        0x102d35ac => { vm.decode(0x4a2204f5u32); let d = 6i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d35b0u64) }
        0x102d35b0 => { vm.decode(0x10401030u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d35b4u64) }
        0x102d35b4 => { vm.decode(0xa400036u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d35b8u64) }
        0x102d35b8 => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d35bcu64) }
        0x102d35bc => { vm.decode(0x140c8aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 251i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d35c0u64).wrapping_add((imm * 4) as u64) } else { 0x102d35c0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d35c0u64) }
        0x102d35c0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d35c4u64) } }
        0x102d35c4 => { vm.decode(0x4648bb5u32); let d = 6i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d35c8u64) }
        0x102d35c8 => { vm.decode(0xb2409d75u32); let d = 7i16; let s = 6i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d35ccu64) }
        0x102d35cc => { vm.decode(0xe26004f5u32); let d = 1i16; let s = 3i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d35d0u64) }
        0x102d35d0 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d35d4u64) }
        0x102d35d4 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d35d8u64) }
        0x102d35d8 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d35dcu64) }
        0x102d35dc => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d35e0u64).wrapping_add((imm * 4) as u64) } else { 0x102d35e0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d35e0u64) }
        0x102d35e0 => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d35e4u64) }
        0x102d35e4 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d35e8u64) }
        0x102d35e8 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d35ecu64) }
        0x102d35ec => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d35f0u64) }
        0x102d35f0 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d35f4u64) }
        0x102d35f4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d35f8u64).wrapping_add((imm * 4) as u64) } else { 0x102d35f8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d35f8u64) }
        0x102d35f8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d35fcu64) } }
        0x102d35fc => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3600u64) }
        0x102d3600 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3604u64) }
        0x102d3604 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3608u64).wrapping_add((imm * 4) as u64) } else { 0x102d3608u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3608u64) }
        0x102d3608 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d360cu64) }
        0x102d360c => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3610u64) }
        0x102d3610 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3614u64) }
        0x102d3614 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3618u64) }
        0x102d3618 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d361cu64) }
        0x102d361c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3620u64).wrapping_add((imm * 4) as u64) } else { 0x102d3620u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3620u64) }
        0x102d3620 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3624u64) } }
        0x102d3624 => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3628u64) }
        0x102d3628 => { vm.decode(0x90a1d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d362cu64) }
        0x102d362c => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3630u64) }
        0x102d3630 => { vm.decode(0x321000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d3634u64) }
        0x102d3634 => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d3638u64) }
        0x102d3638 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d363cu64) }
        0x102d363c => { vm.decode(0xca6204f5u32); let d = 7i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3640u64) }
        0x102d3640 => { vm.decode(0x10401430u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d3644u64) }
        0x102d3644 => { vm.decode(0x8602036u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3648u64) }
        0x102d3648 => { vm.decode(0xa040bd75u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d364cu64) }
        0x102d364c => { vm.decode(0xe26004f5u32); let d = 1i16; let s = 3i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d3650u64) }
        0x102d3650 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d3654u64) }
        0x102d3654 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3658u64) }
        0x102d3658 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d365cu64) }
        0x102d365c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3660u64).wrapping_add((imm * 4) as u64) } else { 0x102d3660u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3660u64) }
        0x102d3660 => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3664u64) }
        0x102d3664 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3668u64) }
        0x102d3668 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d366cu64) }
        0x102d366c => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3670u64) }
        0x102d3670 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3674u64) }
        0x102d3674 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3678u64).wrapping_add((imm * 4) as u64) } else { 0x102d3678u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3678u64) }
        0x102d3678 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d367cu64) } }
        0x102d367c => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3680u64) }
        0x102d3680 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d3684u64) }
        0x102d3684 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3688u64).wrapping_add((imm * 4) as u64) } else { 0x102d3688u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3688u64) }
        0x102d3688 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d368cu64) }
        0x102d368c => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3690u64) }
        0x102d3690 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d3694u64) }
        0x102d3694 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d3698u64) }
        0x102d3698 => { vm.decode(0x82002446u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d369cu64) }
        0x102d369c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d36a0u64).wrapping_add((imm * 4) as u64) } else { 0x102d36a0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d36a0u64) }
        0x102d36a0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d36a4u64) } }
        0x102d36a4 => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d36a8u64) }
        0x102d36a8 => { vm.decode(0x90a1d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 20i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d36acu64) }
        0x102d36ac => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d36b0u64) }
        0x102d36b0 => { vm.decode(0x321000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 5164i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d36b4u64) }
        0x102d36b4 => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d36b8u64) }
        0x102d36b8 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d36bcu64) }
        0x102d36bc => { vm.decode(0x1608036u32); let d = 8i16; let s = 6i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d36c0u64) }
        0x102d36c0 => { vm.decode(0xa1480d75u32); let d = 9i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d36c4u64) }
        0x102d36c4 => { vm.decode(0x828014f5u32); let d = 8i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d36c8u64) }
        0x102d36c8 => { vm.decode(0x18000030u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d36ccu64) }
        0x102d36cc => { vm.decode(0x622036u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d36d0u64) }
        0x102d36d0 => { vm.decode(0xa0082d75u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d36d4u64) }
        0x102d36d4 => { vm.decode(0x224014f5u32); let d = 1i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d36d8u64) }
        0x102d36d8 => { vm.decode(0x8092fu32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d36dcu64) }
        0x102d36dc => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d36e0u64) }
        0x102d36e0 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d36e4u64) }
        0x102d36e4 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d36e8u64).wrapping_add((imm * 4) as u64) } else { 0x102d36e8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d36e8u64) }
        0x102d36e8 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d36ecu64) }
        0x102d36ec => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d36f0u64) }
        0x102d36f0 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d36f4u64) }
        0x102d36f4 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d36f8u64) }
        0x102d36f8 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d36fcu64) }
        0x102d36fc => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d3700u64).wrapping_add((imm * 4) as u64) } else { 0x102d3700u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3700u64) }
        0x102d3700 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d3704u64) } }
        0x102d3704 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d3708u64) }
        0x102d3708 => { vm.decode(0x81002446u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 20i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d370cu64) }
        0x102d370c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d3710u64).wrapping_add((imm * 4) as u64) } else { 0x102d3710u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d3710u64) }
        0x102d3710 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d3714u64) }
        _ => Err(Error::UnknownPc(pc)),
    }
}
