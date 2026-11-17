#![cfg_attr(rustfmt, rustfmt::skip)]
// 由 scripts/medusa-vm/gen_native.py 生成，勿手改。
use super::super::Vm;
use super::super::Error;

#[allow(clippy::all, unused_parens, unused_variables, unused_mut)]
pub fn exec(vm: &mut Vm, pc: u64) -> Result<u64, Error> {
    match pc {
        0x10292580 => { vm.decode(0xb3855b0u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292584u64) }
        0x10292584 => { vm.decode(0x610f1835u32); let d = 6i16; let s = 13i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292588u64) }
        0x10292588 => { vm.decode(0xb08d530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029258cu64) }
        0x1029258c => { vm.decode(0x10f1835u32); let d = 6i16; let s = 12i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292590u64) }
        0x10292590 => { vm.decode(0xb00d530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292594u64) }
        0x10292594 => { vm.decode(0x274a9835u32); let d = 30i16; let s = 11i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292598u64) }
        0x10292598 => { vm.decode(0x474aa835u32); let d = 31i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029259cu64) }
        0x1029259c => { vm.decode(0x193095b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102925a0u64) }
        0x102925a0 => { vm.decode(0x40d29119u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 240i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102925a4u64) }
        0x102925a4 => { vm.decode(0x1b0815b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102925a8u64) }
        0x102925a8 => { vm.decode(0x46449835u32); let d = 24i16; let s = 2i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102925acu64) }
        0x102925ac => { vm.decode(0x2208a835u32); let d = 11i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102925b0u64) }
        0x102925b0 => { vm.decode(0x20122200u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 11i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102925b4u64) }
        0x102925b4 => { vm.decode(0x420c88a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 915i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102925b8u64).wrapping_add((imm * 4) as u64) } else { 0x102925b8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102925b8u64) }
        0x102925b8 => { vm.decode(0x2004875u32); let d = 8i16; let s = 0i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102925bcu64) }
        0x102925bc => { vm.decode(0x32a200f5u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102925c0u64) }
        0x102925c0 => { vm.decode(0x54a080f5u32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102925c4u64) }
        0x102925c4 => { vm.decode(0x76b180f5u32); let d = 10i16; let s = 31i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102925c8u64) }
        0x102925c8 => { vm.decode(0x9b1055f0u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102925ccu64) }
        0x102925cc => { vm.decode(0x1b3015f0u32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102925d0u64) }
        0x102925d0 => { vm.decode(0x36b300f5u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102925d4u64) }
        0x102925d4 => { vm.decode(0x5290c0f5u32); let d = 12i16; let s = 20i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102925d8u64) }
        0x102925d8 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102925dcu64) }
        0x102925dc => { vm.decode(0x309260f5u32); let d = 13i16; let s = 8i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102925e0u64) }
        0x102925e0 => { vm.decode(0xb381470u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102925e4u64) }
        0x102925e4 => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102925e8u64) }
        0x102925e8 => { vm.decode(0x2a8e636fu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102925ecu64) }
        0x102925ec => { vm.decode(0xa2e234f5u32); let d = 13i16; let s = 7i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102925f0u64) }
        0x102925f0 => { vm.decode(0x82c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102925f4u64) }
        0x102925f4 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102925f8u64) }
        0x102925f8 => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102925fcu64) }
        0x102925fc => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292600u64) }
        0x10292600 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292604u64) }
        0x10292604 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292608u64) }
        0x10292608 => { vm.decode(0x2088e32fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029260cu64) }
        0x1029260c => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292610u64) }
        0x10292610 => { vm.decode(0x1491e0f5u32); let d = 14i16; let s = 18i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292614u64) }
        0x10292614 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292618u64) }
        0x10292618 => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029261cu64) }
        0x1029261c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292620u64) }
        0x10292620 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292624u64) }
        0x10292624 => { vm.decode(0xa234f5u32); let d = 12i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292628u64) }
        0x10292628 => { vm.decode(0x3692c0f5u32); let d = 15i16; let s = 24i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029262cu64) }
        0x1029262c => { vm.decode(0x50d2a0f5u32); let d = 25i16; let s = 20i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292630u64) }
        0x10292630 => { vm.decode(0x220d1122u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10292634u64) }
        0x10292634 => { vm.decode(0x52e000f5u32); let d = 16i16; let s = 5i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292638u64) }
        0x10292638 => { vm.decode(0x70c3a0f5u32); let d = 17i16; let s = 30i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029263cu64) }
        0x1029263c => { vm.decode(0xc6e8a2adu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292640u64) }
        0x10292640 => { vm.decode(0xa34424f5u32); let d = 17i16; let s = 7i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292644u64) }
        0x10292644 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292648u64) }
        0x10292648 => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029264cu64) }
        0x1029264c => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292650u64) }
        0x10292650 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292654u64) }
        0x10292654 => { vm.decode(0x48901875u32); let d = 22i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292658u64) }
        0x10292658 => { vm.decode(0x54e0a0f5u32); let d = 18i16; let s = 21i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029265cu64) }
        0x1029265c => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292660u64) }
        0x10292660 => { vm.decode(0xcedcf26du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292664u64) }
        0x10292664 => { vm.decode(0x60032835u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292668u64) }
        0x10292668 => { vm.decode(0x62e234f5u32); let d = 13i16; let s = 7i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029266cu64) }
        0x1029266c => { vm.decode(0x16b1c0f5u32); let d = 14i16; let s = 19i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292670u64) }
        0x10292670 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292674u64) }
        0x10292674 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292678u64) }
        0x10292678 => { vm.decode(0x6000412fu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029267cu64) }
        0x1029267c => { vm.decode(0x10424f5u32); let d = 16i16; let s = 4i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292680u64) }
        0x10292680 => { vm.decode(0xc8e234f5u32); let d = 15i16; let s = 4i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292684u64) }
        0x10292684 => { vm.decode(0x44901875u32); let d = 6i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292688u64) }
        0x10292688 => { vm.decode(0xab4434f5u32); let d = 19i16; let s = 7i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029268cu64) }
        0x1029268c => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292690u64) }
        0x10292690 => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292694u64) }
        0x10292694 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292698u64) }
        0x10292698 => { vm.decode(0x32d200f5u32); let d = 25i16; let s = 8i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029269cu64) }
        0x1029269c => { vm.decode(0x8b2054b0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102926a0u64) }
        0x102926a0 => { vm.decode(0xc66cb2adu32); let d = 20i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102926a4u64) }
        0x102926a4 => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102926a8u64) }
        0x102926a8 => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102926acu64) }
        0x102926ac => { vm.decode(0x56a180f5u32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102926b0u64) }
        0x102926b0 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102926b4u64) }
        0x102926b4 => { vm.decode(0x480024f5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102926b8u64) }
        0x102926b8 => { vm.decode(0x3603a835u32); let d = 25i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102926bcu64) }
        0x102926bc => { vm.decode(0x50b1e0f5u32); let d = 12i16; let s = 23i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102926c0u64) }
        0x102926c0 => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102926c4u64) }
        0x102926c4 => { vm.decode(0x6028412fu32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102926c8u64) }
        0x102926c8 => { vm.decode(0x12624f5u32); let d = 20i16; let s = 4i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102926ccu64) }
        0x102926cc => { vm.decode(0x6b0424f5u32); let d = 18i16; let s = 7i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102926d0u64) }
        0x102926d0 => { vm.decode(0x48a234f5u32); let d = 14i16; let s = 4i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102926d4u64) }
        0x102926d4 => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102926d8u64) }
        0x102926d8 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102926dcu64) }
        0x102926dc => { vm.decode(0x22a234f5u32); let d = 12i16; let s = 7i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102926e0u64) }
        0x102926e0 => { vm.decode(0x33142835u32); let d = 13i16; let s = 10i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102926e4u64) }
        0x102926e4 => { vm.decode(0x3490a0f5u32); let d = 10i16; let s = 24i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102926e8u64) }
        0x102926e8 => { vm.decode(0x8b1814f0u32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102926ecu64) }
        0x102926ec => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102926f0u64) }
        0x102926f0 => { vm.decode(0x6a8034f5u32); let d = 10i16; let s = 7i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102926f4u64) }
        0x102926f4 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102926f8u64) }
        0x102926f8 => { vm.decode(0x50462835u32); let d = 1i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102926fcu64) }
        0x102926fc => { vm.decode(0x1b081430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292700u64) }
        0x10292700 => { vm.decode(0x20a80a2fu32); let d = 11i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292704u64) }
        0x10292704 => { vm.decode(0x2a482fu32); let d = 14i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292708u64) }
        0x10292708 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029270cu64) }
        0x1029270c => { vm.decode(0x6080402fu32); let d = 17i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292710u64) }
        0x10292710 => { vm.decode(0x4088022fu32); let d = 18i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292714u64) }
        0x10292714 => { vm.decode(0x24501835u32); let d = 16i16; let s = 16i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292718u64) }
        0x10292718 => { vm.decode(0x70b1e0f5u32); let d = 12i16; let s = 31i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029271cu64) }
        0x1029271c => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292720u64) }
        0x10292720 => { vm.decode(0x22a234f5u32); let d = 12i16; let s = 7i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292724u64) }
        0x10292724 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292728u64) }
        0x10292728 => { vm.decode(0x5007a835u32); let d = 1i16; let s = 12i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029272cu64) }
        0x1029272c => { vm.decode(0x1b001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292730u64) }
        0x10292730 => { vm.decode(0x688a4bafu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292734u64) }
        0x10292734 => { vm.decode(0x76b3c0f5u32); let d = 15i16; let s = 31i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292738u64) }
        0x10292738 => { vm.decode(0xcedcf26du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029273cu64) }
        0x1029273c => { vm.decode(0xc8e234f5u32); let d = 15i16; let s = 4i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292740u64) }
        0x10292740 => { vm.decode(0x60a8032fu32); let d = 19i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292744u64) }
        0x10292744 => { vm.decode(0x43501835u32); let d = 12i16; let s = 17i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292748u64) }
        0x10292748 => { vm.decode(0x54b260f5u32); let d = 15i16; let s = 5i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029274cu64) }
        0x1029274c => { vm.decode(0xcedcf26du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292750u64) }
        0x10292750 => { vm.decode(0xeae234f5u32); let d = 15i16; let s = 7i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292754u64) }
        0x10292754 => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292758u64) }
        0x10292758 => { vm.decode(0x9b285430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029275cu64) }
        0x1029275c => { vm.decode(0x20472835u32); let d = 1i16; let s = 2i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292760u64) }
        0x10292760 => { vm.decode(0x19389cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292764u64) }
        0x10292764 => { vm.decode(0x534f2835u32); let d = 15i16; let s = 11i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292768u64) }
        0x10292768 => { vm.decode(0x288063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029276cu64) }
        0x1029276c => { vm.decode(0x14a020f5u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292770u64) }
        0x10292770 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292774u64) }
        0x10292774 => { vm.decode(0x6a0024f5u32); let d = 2i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292778u64) }
        0x10292778 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029277cu64) }
        0x1029277c => { vm.decode(0x1b089430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292780u64) }
        0x10292780 => { vm.decode(0x404c1835u32); let d = 2i16; let s = 2i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292784u64) }
        0x10292784 => { vm.decode(0xb101530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292788u64) }
        0x10292788 => { vm.decode(0x56a080f5u32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029278cu64) }
        0x1029278c => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292790u64) }
        0x10292790 => { vm.decode(0x480024f5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292794u64) }
        0x10292794 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292798u64) }
        0x10292798 => { vm.decode(0x1b301470u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029279cu64) }
        0x1029279c => { vm.decode(0x34b220f5u32); let d = 11i16; let s = 9i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102927a0u64) }
        0x102927a0 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102927a4u64) }
        0x102927a4 => { vm.decode(0xeac034f5u32); let d = 11i16; let s = 7i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102927a8u64) }
        0x102927a8 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102927acu64) }
        0x102927ac => { vm.decode(0x25462835u32); let d = 21i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102927b0u64) }
        0x102927b0 => { vm.decode(0x569380f5u32); let d = 11i16; let s = 22i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102927b4u64) }
        0x102927b4 => { vm.decode(0x42105875u32); let d = 10i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102927b8u64) }
        0x102927b8 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102927bcu64) }
        0x102927bc => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102927c0u64) }
        0x102927c0 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102927c4u64) }
        0x102927c4 => { vm.decode(0x549160f5u32); let d = 14i16; let s = 6i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102927c8u64) }
        0x102927c8 => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102927ccu64) }
        0x102927cc => { vm.decode(0x6aa234f5u32); let d = 14i16; let s = 7i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102927d0u64) }
        0x102927d0 => { vm.decode(0xaa0a2fu32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102927d4u64) }
        0x102927d4 => { vm.decode(0x524f2835u32); let d = 11i16; let s = 11i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102927d8u64) }
        0x102927d8 => { vm.decode(0x16b040f5u32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102927dcu64) }
        0x102927dc => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102927e0u64) }
        0x102927e0 => { vm.decode(0x48a234f5u32); let d = 14i16; let s = 4i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102927e4u64) }
        0x102927e4 => { vm.decode(0xaa0a2fu32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102927e8u64) }
        0x102927e8 => { vm.decode(0x228463efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102927ecu64) }
        0x102927ec => { vm.decode(0x14e020f5u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102927f0u64) }
        0x102927f0 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102927f4u64) }
        0x102927f4 => { vm.decode(0x6b0424f5u32); let d = 18i16; let s = 7i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102927f8u64) }
        0x102927f8 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102927fcu64) }
        0x102927fc => { vm.decode(0x145ca835u32); let d = 19i16; let s = 14i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292800u64) }
        0x10292800 => { vm.decode(0x7691c0f5u32); let d = 14i16; let s = 30i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292804u64) }
        0x10292804 => { vm.decode(0x40305875u32); let d = 2i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292808u64) }
        0x10292808 => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029280cu64) }
        0x1029280c => { vm.decode(0x48a234f5u32); let d = 14i16; let s = 4i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292810u64) }
        0x10292810 => { vm.decode(0xaa0a2fu32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292814u64) }
        0x10292814 => { vm.decode(0x74e1a0f5u32); let d = 18i16; let s = 31i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292818u64) }
        0x10292818 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029281cu64) }
        0x1029281c => { vm.decode(0x6b0424f5u32); let d = 18i16; let s = 7i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292820u64) }
        0x10292820 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292824u64) }
        0x10292824 => { vm.decode(0x135c8835u32); let d = 14i16; let s = 14i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292828u64) }
        0x10292828 => { vm.decode(0xa804bafu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029282cu64) }
        0x1029282c => { vm.decode(0x36c000f5u32); let d = 18i16; let s = 8i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292830u64) }
        0x10292830 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292834u64) }
        0x10292834 => { vm.decode(0x490424f5u32); let d = 18i16; let s = 4i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292838u64) }
        0x10292838 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029283cu64) }
        0x1029283c => { vm.decode(0x428e4befu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292840u64) }
        0x10292840 => { vm.decode(0x70c1e0f5u32); let d = 20i16; let s = 30i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292844u64) }
        0x10292844 => { vm.decode(0xc66c22edu32); let d = 20i16; let s = 20i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292848u64) }
        0x10292848 => { vm.decode(0x232624f5u32); let d = 20i16; let s = 7i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029284cu64) }
        0x1029284c => { vm.decode(0x4082402fu32); let d = 20i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292850u64) }
        0x10292850 => { vm.decode(0x9b00d570u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292854u64) }
        0x10292854 => { vm.decode(0x4153835u32); let d = 17i16; let s = 18i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292858u64) }
        0x10292858 => { vm.decode(0x54e000f5u32); let d = 18i16; let s = 5i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029285cu64) }
        0x1029285c => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292860u64) }
        0x10292860 => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292864u64) }
        0x10292864 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292868u64) }
        0x10292868 => { vm.decode(0x26163835u32); let d = 25i16; let s = 18i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029286cu64) }
        0x1029286c => { vm.decode(0x99289cf0u32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292870u64) }
        0x10292870 => { vm.decode(0x131c63b5u32); let d = 18i16; let s = 6i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292874u64) }
        0x10292874 => { vm.decode(0xb2855b0u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292878u64) }
        0x10292878 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029287cu64) }
        0x1029287c => { vm.decode(0x1b205430u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292880u64) }
        0x10292880 => { vm.decode(0x2282c32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292884u64) }
        0x10292884 => { vm.decode(0x690424f5u32); let d = 18i16; let s = 5i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292888u64) }
        0x10292888 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029288cu64) }
        0x1029288c => { vm.decode(0x6a84cb6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292890u64) }
        0x10292890 => { vm.decode(0x26163835u32); let d = 25i16; let s = 18i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292894u64) }
        0x10292894 => { vm.decode(0x16e000f5u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292898u64) }
        0x10292898 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029289cu64) }
        0x1029289c => { vm.decode(0x410624f5u32); let d = 20i16; let s = 4i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102928a0u64) }
        0x102928a0 => { vm.decode(0x34c080f5u32); let d = 18i16; let s = 24i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102928a4u64) }
        0x102928a4 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102928a8u64) }
        0x102928a8 => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102928acu64) }
        0x102928ac => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102928b0u64) }
        0x102928b0 => { vm.decode(0x4800875u32); let d = 4i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102928b4u64) }
        0x102928b4 => { vm.decode(0x250f1835u32); let d = 22i16; let s = 13i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102928b8u64) }
        0x102928b8 => { vm.decode(0xb109470u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102928bcu64) }
        0x102928bc => { vm.decode(0x309260f5u32); let d = 13i16; let s = 8i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102928c0u64) }
        0x102928c0 => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102928c4u64) }
        0x102928c4 => { vm.decode(0xaa2434f5u32); let d = 18i16; let s = 7i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102928c8u64) }
        0x102928c8 => { vm.decode(0x9930dd30u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102928ccu64) }
        0x102928cc => { vm.decode(0x50b3c0f5u32); let d = 13i16; let s = 23i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102928d0u64) }
        0x102928d0 => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102928d4u64) }
        0x102928d4 => { vm.decode(0xa2e214f5u32); let d = 13i16; let s = 3i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102928d8u64) }
        0x102928d8 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102928dcu64) }
        0x102928dc => { vm.decode(0x5010a835u32); let d = 1i16; let s = 13i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102928e0u64) }
        0x102928e0 => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102928e4u64) }
        0x102928e4 => { vm.decode(0x309240f5u32); let d = 13i16; let s = 8i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102928e8u64) }
        0x102928e8 => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102928ecu64) }
        0x102928ec => { vm.decode(0xa2e214f5u32); let d = 13i16; let s = 3i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102928f0u64) }
        0x102928f0 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102928f4u64) }
        0x102928f4 => { vm.decode(0x208063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102928f8u64) }
        0x102928f8 => { vm.decode(0x72008835u32); let d = 8i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102928fcu64) }
        0x102928fc => { vm.decode(0x30b240f5u32); let d = 13i16; let s = 9i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292900u64) }
        0x10292900 => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292904u64) }
        0x10292904 => { vm.decode(0xa2e214f5u32); let d = 13i16; let s = 3i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292908u64) }
        0x10292908 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029290cu64) }
        0x1029290c => { vm.decode(0x5003a835u32); let d = 1i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292910u64) }
        0x10292910 => { vm.decode(0x19209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292914u64) }
        0x10292914 => { vm.decode(0x7093c0f5u32); let d = 13i16; let s = 30i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292918u64) }
        0x10292918 => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029291cu64) }
        0x1029291c => { vm.decode(0xa2e214f5u32); let d = 13i16; let s = 3i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292920u64) }
        0x10292920 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292924u64) }
        0x10292924 => { vm.decode(0x208043afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292928u64) }
        0x10292928 => { vm.decode(0x7300a835u32); let d = 13i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029292cu64) }
        0x1029292c => { vm.decode(0x309140f5u32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292930u64) }
        0x10292930 => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292934u64) }
        0x10292934 => { vm.decode(0x22a214f5u32); let d = 12i16; let s = 3i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292938u64) }
        0x10292938 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029293cu64) }
        0x1029293c => { vm.decode(0x3043a835u32); let d = 1i16; let s = 12i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292940u64) }
        0x10292940 => { vm.decode(0x19389c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292944u64) }
        0x10292944 => { vm.decode(0x42806befu32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292948u64) }
        0x10292948 => { vm.decode(0x3492c0f5u32); let d = 15i16; let s = 24i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029294cu64) }
        0x1029294c => { vm.decode(0xcedcf26du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292950u64) }
        0x10292950 => { vm.decode(0xeae214f5u32); let d = 15i16; let s = 3i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292954u64) }
        0x10292954 => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292958u64) }
        0x10292958 => { vm.decode(0x2a84e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029295cu64) }
        0x1029295c => { vm.decode(0x730ca835u32); let d = 15i16; let s = 1i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292960u64) }
        0x10292960 => { vm.decode(0x50c100f5u32); let d = 16i16; let s = 6i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292964u64) }
        0x10292964 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292968u64) }
        0x10292968 => { vm.decode(0x230404f5u32); let d = 16i16; let s = 3i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029296cu64) }
        0x1029296c => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292970u64) }
        0x10292970 => { vm.decode(0x20113835u32); let d = 1i16; let s = 16i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292974u64) }
        0x10292974 => { vm.decode(0x19289c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292978u64) }
        0x10292978 => { vm.decode(0x2801875u32); let d = 12i16; let s = 0i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029297cu64) }
        0x1029297c => { vm.decode(0x148100f5u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292980u64) }
        0x10292980 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292984u64) }
        0x10292984 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292988u64) }
        0x10292988 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029298cu64) }
        0x1029298c => { vm.decode(0x228043afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292990u64) }
        0x10292990 => { vm.decode(0x200c0835u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292994u64) }
        0x10292994 => { vm.decode(0x228e43afu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292998u64) }
        0x10292998 => { vm.decode(0x50e100f5u32); let d = 16i16; let s = 7i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x1029299cu64) }
        0x1029299c => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102929a0u64) }
        0x102929a0 => { vm.decode(0x230404f5u32); let d = 16i16; let s = 3i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102929a4u64) }
        0x102929a4 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102929a8u64) }
        0x102929a8 => { vm.decode(0x433835u32); let d = 1i16; let s = 14i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102929acu64) }
        0x102929ac => { vm.decode(0x1b001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102929b0u64) }
        0x102929b0 => { vm.decode(0x70e180f5u32); let d = 16i16; let s = 31i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102929b4u64) }
        0x102929b4 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102929b8u64) }
        0x102929b8 => { vm.decode(0x230404f5u32); let d = 16i16; let s = 3i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102929bcu64) }
        0x102929bc => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102929c0u64) }
        0x102929c0 => { vm.decode(0x25503835u32); let d = 21i16; let s = 16i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102929c4u64) }
        0x102929c4 => { vm.decode(0x208443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102929c8u64) }
        0x102929c8 => { vm.decode(0x10e000f5u32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102929ccu64) }
        0x102929cc => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102929d0u64) }
        0x102929d0 => { vm.decode(0x230404f5u32); let d = 16i16; let s = 3i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102929d4u64) }
        0x102929d4 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102929d8u64) }
        0x102929d8 => { vm.decode(0x244a3835u32); let d = 19i16; let s = 11i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102929dcu64) }
        0x102929dc => { vm.decode(0x4a80c875u32); let d = 31i16; let s = 0i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102929e0u64) }
        0x102929e0 => { vm.decode(0x549200f5u32); let d = 11i16; let s = 4i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102929e4u64) }
        0x102929e4 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102929e8u64) }
        0x102929e8 => { vm.decode(0xeac014f5u32); let d = 11i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102929ecu64) }
        0x102929ec => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102929f0u64) }
        0x102929f0 => { vm.decode(0x71140835u32); let d = 4i16; let s = 11i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102929f4u64) }
        0x102929f4 => { vm.decode(0x938dcb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102929f8u64) }
        0x102929f8 => { vm.decode(0x52bb5u32); let d = 4i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102929fcu64) }
        0x102929fc => { vm.decode(0x24d3022u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10292a00u64) }
        0x10292a00 => { vm.decode(0x2209875u32); let d = 9i16; let s = 0i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292a04u64) }
        0x10292a04 => { vm.decode(0xb181570u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292a08u64) }
        0x10292a08 => { vm.decode(0x401b2d35u32); let d = 11i16; let s = 4i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10292a0cu64) }
        0x10292a0c => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292a10u64) }
        0x10292a10 => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292a14u64) }
        0x10292a14 => { vm.decode(0x282e32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292a18u64) }
        0x10292a18 => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292a1cu64) }
        0x10292a1c => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292a20u64) }
        0x10292a20 => { vm.decode(0x74160835u32); let d = 16i16; let s = 11i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292a24u64) }
        0x10292a24 => { vm.decode(0xd39c53b5u32); let d = 11i16; let s = 23i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292a28u64) }
        0x10292a28 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292a2cu64) }
        0x10292a2c => { vm.decode(0xe8c034f5u32); let d = 11i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292a30u64) }
        0x10292a30 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292a34u64) }
        0x10292a34 => { vm.decode(0x88ee36fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292a38u64) }
        0x10292a38 => { vm.decode(0x554d2835u32); let d = 23i16; let s = 6i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292a3cu64) }
        0x10292a3c => { vm.decode(0x6080482fu32); let d = 25i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292a40u64) }
        0x10292a40 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292a44u64) }
        0x10292a44 => { vm.decode(0x51581835u32); let d = 6i16; let s = 18i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292a48u64) }
        0x10292a48 => { vm.decode(0xb085530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292a4cu64) }
        0x10292a4c => { vm.decode(0x639c73b5u32); let d = 25i16; let s = 30i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292a50u64) }
        0x10292a50 => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292a54u64) }
        0x10292a54 => { vm.decode(0xa1c434f5u32); let d = 25i16; let s = 5i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292a58u64) }
        0x10292a58 => { vm.decode(0x60004b2fu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292a5cu64) }
        0x10292a5c => { vm.decode(0x54591835u32); let d = 18i16; let s = 22i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292a60u64) }
        0x10292a60 => { vm.decode(0x28886bafu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292a64u64) }
        0x10292a64 => { vm.decode(0x30f300f5u32); let d = 25i16; let s = 11i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292a68u64) }
        0x10292a68 => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292a6cu64) }
        0x10292a6c => { vm.decode(0xab0614f5u32); let d = 22i16; let s = 3i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292a70u64) }
        0x10292a70 => { vm.decode(0xe39c73b5u32); let d = 25i16; let s = 31i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292a74u64) }
        0x10292a74 => { vm.decode(0x9b3855f0u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292a78u64) }
        0x10292a78 => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292a7cu64) }
        0x10292a7c => { vm.decode(0xa1c434f5u32); let d = 25i16; let s = 5i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292a80u64) }
        0x10292a80 => { vm.decode(0x60004b2fu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292a84u64) }
        0x10292a84 => { vm.decode(0x55021835u32); let d = 20i16; let s = 8i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292a88u64) }
        0x10292a88 => { vm.decode(0x40b00875u32); let d = 6i16; let s = 0i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292a8cu64) }
        0x10292a8c => { vm.decode(0x721c13b5u32); let d = 10i16; let s = 24i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292a90u64) }
        0x10292a90 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292a94u64) }
        0x10292a94 => { vm.decode(0x688034f5u32); let d = 10i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292a98u64) }
        0x10292a98 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292a9cu64) }
        0x10292a9c => { vm.decode(0x2a8cc36fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292aa0u64) }
        0x10292aa0 => { vm.decode(0x374c0835u32); let d = 30i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292aa4u64) }
        0x10292aa4 => { vm.decode(0xa804bafu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292aa8u64) }
        0x10292aa8 => { vm.decode(0x229c33b5u32); let d = 25i16; let s = 8i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292aacu64) }
        0x10292aac => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292ab0u64) }
        0x10292ab0 => { vm.decode(0xa1c434f5u32); let d = 25i16; let s = 5i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292ab4u64) }
        0x10292ab4 => { vm.decode(0x60004b2fu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292ab8u64) }
        0x10292ab8 => { vm.decode(0x700b3835u32); let d = 3i16; let s = 13i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292abcu64) }
        0x10292abc => { vm.decode(0x19109d30u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292ac0u64) }
        0x10292ac0 => { vm.decode(0x886befu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292ac4u64) }
        0x10292ac4 => { vm.decode(0x23dc13b5u32); let d = 13i16; let s = 10i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292ac8u64) }
        0x10292ac8 => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292accu64) }
        0x10292acc => { vm.decode(0xa0e234f5u32); let d = 13i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292ad0u64) }
        0x10292ad0 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292ad4u64) }
        0x10292ad4 => { vm.decode(0x288cc36fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292ad8u64) }
        0x10292ad8 => { vm.decode(0x7048a835u32); let d = 3i16; let s = 3i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292adcu64) }
        0x10292adc => { vm.decode(0x19189db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292ae0u64) }
        0x10292ae0 => { vm.decode(0x22824befu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292ae4u64) }
        0x10292ae4 => { vm.decode(0xa29c73b5u32); let d = 25i16; let s = 13i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292ae8u64) }
        0x10292ae8 => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292aecu64) }
        0x10292aec => { vm.decode(0xa1c434f5u32); let d = 25i16; let s = 5i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292af0u64) }
        0x10292af0 => { vm.decode(0x60004b2fu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292af4u64) }
        0x10292af4 => { vm.decode(0x76433835u32); let d = 25i16; let s = 15i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292af8u64) }
        0x10292af8 => { vm.decode(0x44008875u32); let d = 3i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292afcu64) }
        0x10292afc => { vm.decode(0x92dc13b5u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292b00u64) }
        0x10292b00 => { vm.decode(0xcedcf26du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292b04u64) }
        0x10292b04 => { vm.decode(0xe8e234f5u32); let d = 15i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292b08u64) }
        0x10292b08 => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292b0cu64) }
        0x10292b0c => { vm.decode(0x2a80e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292b10u64) }
        0x10292b10 => { vm.decode(0x7004a835u32); let d = 1i16; let s = 1i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292b14u64) }
        0x10292b14 => { vm.decode(0x19289cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292b18u64) }
        0x10292b18 => { vm.decode(0x93dc53b5u32); let d = 15i16; let s = 7i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292b1cu64) }
        0x10292b1c => { vm.decode(0x46805875u32); let d = 14i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292b20u64) }
        0x10292b20 => { vm.decode(0xcedcf26du32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292b24u64) }
        0x10292b24 => { vm.decode(0xe8e234f5u32); let d = 15i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292b28u64) }
        0x10292b28 => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292b2cu64) }
        0x10292b2c => { vm.decode(0x5044a835u32); let d = 1i16; let s = 2i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292b30u64) }
        0x10292b30 => { vm.decode(0x19009c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292b34u64) }
        0x10292b34 => { vm.decode(0xb21c03b5u32); let d = 2i16; let s = 9i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292b38u64) }
        0x10292b38 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292b3cu64) }
        0x10292b3c => { vm.decode(0x680024f5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292b40u64) }
        0x10292b40 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292b44u64) }
        0x10292b44 => { vm.decode(0x2880e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292b48u64) }
        0x10292b48 => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292b4cu64) }
        0x10292b4c => { vm.decode(0x19109cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292b50u64) }
        0x10292b50 => { vm.decode(0x608043efu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292b54u64) }
        0x10292b54 => { vm.decode(0xd21c03b5u32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292b58u64) }
        0x10292b58 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292b5cu64) }
        0x10292b5c => { vm.decode(0x680024f5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292b60u64) }
        0x10292b60 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292b64u64) }
        0x10292b64 => { vm.decode(0x20542835u32); let d = 1i16; let s = 2i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292b68u64) }
        0x10292b68 => { vm.decode(0x19381cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292b6cu64) }
        0x10292b6c => { vm.decode(0x321c43b5u32); let d = 2i16; let s = 12i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292b70u64) }
        0x10292b70 => { vm.decode(0x4820c875u32); let d = 19i16; let s = 0i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292b74u64) }
        0x10292b74 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292b78u64) }
        0x10292b78 => { vm.decode(0x680024f5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292b7cu64) }
        0x10292b7c => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292b80u64) }
        0x10292b80 => { vm.decode(0x20152835u32); let d = 1i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292b84u64) }
        0x10292b84 => { vm.decode(0x19189c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292b88u64) }
        0x10292b88 => { vm.decode(0x228e63afu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292b8cu64) }
        0x10292b8c => { vm.decode(0x931c43b5u32); let d = 2i16; let s = 7i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292b90u64) }
        0x10292b90 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292b94u64) }
        0x10292b94 => { vm.decode(0x680024f5u32); let d = 2i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292b98u64) }
        0x10292b98 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292b9cu64) }
        0x10292b9c => { vm.decode(0x208043afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292ba0u64) }
        0x10292ba0 => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292ba4u64) }
        0x10292ba4 => { vm.decode(0x19209c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292ba8u64) }
        0x10292ba8 => { vm.decode(0x1830bb5u32); let d = 9i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10292bacu64) }
        0x10292bac => { vm.decode(0x405e4349u32); let d = 2i16; let s = 9i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10292bb0u64) }
        0x10292bb0 => { vm.decode(0x30f1022u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10292bb4u64) }
        0x10292bb4 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292bb8u64) }
        0x10292bb8 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292bbcu64) }
        0x10292bbc => { vm.decode(0x208cc32fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292bc0u64) }
        0x10292bc0 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292bc4u64) }
        0x10292bc4 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292bc8u64) }
        0x10292bc8 => { vm.decode(0x142835u32); let d = 1i16; let s = 2i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292bccu64) }
        0x10292bcc => { vm.decode(0x19009cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292bd0u64) }
        0x10292bd0 => { vm.decode(0x44201875u32); let d = 2i16; let s = 0i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292bd4u64) }
        0x10292bd4 => { vm.decode(0xa39c03b5u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292bd8u64) }
        0x10292bd8 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292bdcu64) }
        0x10292bdc => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292be0u64) }
        0x10292be0 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292be4u64) }
        0x10292be4 => { vm.decode(0x2a8ceb6fu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292be8u64) }
        0x10292be8 => { vm.decode(0x60422835u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292becu64) }
        0x10292bec => { vm.decode(0x19289c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292bf0u64) }
        0x10292bf0 => { vm.decode(0x28884bafu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292bf4u64) }
        0x10292bf4 => { vm.decode(0xa39c03b5u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10292bf8u64) }
        0x10292bf8 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292bfcu64) }
        0x10292bfc => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292c00u64) }
        0x10292c00 => { vm.decode(0x19089cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292c04u64) }
        0x10292c04 => { vm.decode(0x230bb5u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10292c08u64) }
        0x10292c08 => { vm.decode(0x43353062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10292c0cu64) }
        0x10292c0c => { vm.decode(0x40920d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10292c10u64) }
        0x10292c10 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292c14u64) }
        0x10292c14 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292c18u64) }
        0x10292c18 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292c1cu64) }
        0x10292c1c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292c20u64) }
        0x10292c20 => { vm.decode(0x60512835u32); let d = 1i16; let s = 1i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292c24u64) }
        0x10292c24 => { vm.decode(0x19209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292c28u64) }
        0x10292c28 => { vm.decode(0x22bb5u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10292c2cu64) }
        0x10292c2c => { vm.decode(0x63253062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10292c30u64) }
        0x10292c30 => { vm.decode(0x40920d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10292c34u64) }
        0x10292c34 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292c38u64) }
        0x10292c38 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292c3cu64) }
        0x10292c3c => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292c40u64) }
        0x10292c40 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292c44u64) }
        0x10292c44 => { vm.decode(0x40502835u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292c48u64) }
        0x10292c48 => { vm.decode(0x19281cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292c4cu64) }
        0x10292c4c => { vm.decode(0x20a0402fu32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292c50u64) }
        0x10292c50 => { vm.decode(0x208263afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292c54u64) }
        0x10292c54 => { vm.decode(0x65012835u32); let d = 21i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292c58u64) }
        0x10292c58 => { vm.decode(0x80010bb5u32); let d = 16i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10292c5cu64) }
        0x10292c5c => { vm.decode(0x40520d35u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10292c60u64) }
        0x10292c60 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292c64u64) }
        0x10292c64 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292c68u64) }
        0x10292c68 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292c6cu64) }
        0x10292c6c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292c70u64) }
        0x10292c70 => { vm.decode(0x45192835u32); let d = 23i16; let s = 1i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292c74u64) }
        0x10292c74 => { vm.decode(0xe02bb5u32); let d = 2i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10292c78u64) }
        0x10292c78 => { vm.decode(0x60120d35u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10292c7cu64) }
        0x10292c7c => { vm.decode(0xa001875u32); let d = 24i16; let s = 0i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292c80u64) }
        0x10292c80 => { vm.decode(0x62357062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10292c84u64) }
        0x10292c84 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292c88u64) }
        0x10292c88 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292c8cu64) }
        0x10292c8c => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292c90u64) }
        0x10292c90 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292c94u64) }
        0x10292c94 => { vm.decode(0x40532835u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292c98u64) }
        0x10292c98 => { vm.decode(0x19089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292c9cu64) }
        0x10292c9c => { vm.decode(0x40e12bb5u32); let d = 2i16; let s = 31i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10292ca0u64) }
        0x10292ca0 => { vm.decode(0x60120d35u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10292ca4u64) }
        0x10292ca4 => { vm.decode(0x21d5022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10292ca8u64) }
        0x10292ca8 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292cacu64) }
        0x10292cac => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292cb0u64) }
        0x10292cb0 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292cb4u64) }
        0x10292cb4 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292cb8u64) }
        0x10292cb8 => { vm.decode(0x2a82c32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292cbcu64) }
        0x10292cbc => { vm.decode(0x64090835u32); let d = 18i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292cc0u64) }
        0x10292cc0 => { vm.decode(0x208463efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292cc4u64) }
        0x10292cc4 => { vm.decode(0x1840bb5u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10292cc8u64) }
        0x10292cc8 => { vm.decode(0x40126d35u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10292cccu64) }
        0x10292ccc => { vm.decode(0x42471062u32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10292cd0u64) }
        0x10292cd0 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292cd4u64) }
        0x10292cd4 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292cd8u64) }
        0x10292cd8 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292cdcu64) }
        0x10292cdc => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292ce0u64) }
        0x10292ce0 => { vm.decode(0x2a86e32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292ce4u64) }
        0x10292ce4 => { vm.decode(0x60012835u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292ce8u64) }
        0x10292ce8 => { vm.decode(0x19109c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292cecu64) }
        0x10292cec => { vm.decode(0x1472bb5u32); let d = 15i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10292cf0u64) }
        0x10292cf0 => { vm.decode(0x60926d35u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10292cf4u64) }
        0x10292cf4 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292cf8u64) }
        0x10292cf8 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292cfcu64) }
        0x10292cfc => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292d00u64) }
        0x10292d00 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292d04u64) }
        0x10292d04 => { vm.decode(0x65120835u32); let d = 20i16; let s = 1i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292d08u64) }
        0x10292d08 => { vm.decode(0x41800bb5u32); let d = 8i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10292d0cu64) }
        0x10292d0c => { vm.decode(0x400b4d35u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10292d10u64) }
        0x10292d10 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292d14u64) }
        0x10292d14 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292d18u64) }
        0x10292d18 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292d1cu64) }
        0x10292d1c => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292d20u64) }
        0x10292d20 => { vm.decode(0x2884e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292d24u64) }
        0x10292d24 => { vm.decode(0x30042835u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292d28u64) }
        0x10292d28 => { vm.decode(0x19189cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292d2cu64) }
        0x10292d2c => { vm.decode(0xa70bb5u32); let d = 5i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10292d30u64) }
        0x10292d30 => { vm.decode(0x408b2d35u32); let d = 10i16; let s = 5i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10292d34u64) }
        0x10292d34 => { vm.decode(0x4480c875u32); let d = 7i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292d38u64) }
        0x10292d38 => { vm.decode(0x63751062u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10292d3cu64) }
        0x10292d3c => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292d40u64) }
        0x10292d40 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292d44u64) }
        0x10292d44 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292d48u64) }
        0x10292d48 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292d4cu64) }
        0x10292d4c => { vm.decode(0x2880c32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292d50u64) }
        0x10292d50 => { vm.decode(0x30042835u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292d54u64) }
        0x10292d54 => { vm.decode(0x19381c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292d58u64) }
        0x10292d58 => { vm.decode(0x2a82cb6fu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292d5cu64) }
        0x10292d5c => { vm.decode(0xa62bb5u32); let d = 5i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10292d60u64) }
        0x10292d60 => { vm.decode(0x408b2d35u32); let d = 10i16; let s = 5i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10292d64u64) }
        0x10292d64 => { vm.decode(0x43453062u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10292d68u64) }
        0x10292d68 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292d6cu64) }
        0x10292d6c => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292d70u64) }
        0x10292d70 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292d74u64) }
        0x10292d74 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292d78u64) }
        0x10292d78 => { vm.decode(0x2284e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292d7cu64) }
        0x10292d7c => { vm.decode(0x36042835u32); let d = 25i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292d80u64) }
        0x10292d80 => { vm.decode(0x408b4d35u32); let d = 10i16; let s = 9i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10292d84u64) }
        0x10292d84 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292d88u64) }
        0x10292d88 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292d8cu64) }
        0x10292d8c => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292d90u64) }
        0x10292d90 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292d94u64) }
        0x10292d94 => { vm.decode(0x2a84c32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292d98u64) }
        0x10292d98 => { vm.decode(0x30042835u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292d9cu64) }
        0x10292d9c => { vm.decode(0x19281c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292da0u64) }
        0x10292da0 => { vm.decode(0x232bb5u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10292da4u64) }
        0x10292da4 => { vm.decode(0x63153062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10292da8u64) }
        0x10292da8 => { vm.decode(0x408b0d35u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10292dacu64) }
        0x10292dac => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292db0u64) }
        0x10292db0 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292db4u64) }
        0x10292db4 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292db8u64) }
        0x10292db8 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292dbcu64) }
        0x10292dbc => { vm.decode(0x2a80e32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292dc0u64) }
        0x10292dc0 => { vm.decode(0x370c2835u32); let d = 31i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292dc4u64) }
        0x10292dc4 => { vm.decode(0x228443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292dc8u64) }
        0x10292dc8 => { vm.decode(0x820bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10292dccu64) }
        0x10292dcc => { vm.decode(0x408b0d35u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10292dd0u64) }
        0x10292dd0 => { vm.decode(0x3353062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10292dd4u64) }
        0x10292dd4 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292dd8u64) }
        0x10292dd8 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292ddcu64) }
        0x10292ddc => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292de0u64) }
        0x10292de0 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292de4u64) }
        0x10292de4 => { vm.decode(0x88ac36fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292de8u64) }
        0x10292de8 => { vm.decode(0x114d0835u32); let d = 6i16; let s = 6i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292decu64) }
        0x10292dec => { vm.decode(0x9305db0u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292df0u64) }
        0x10292df0 => { vm.decode(0x524a4349u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10292df4u64) }
        0x10292df4 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292df8u64) }
        0x10292df8 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292dfcu64) }
        0x10292dfc => { vm.decode(0xc109875u32); let d = 17i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292e00u64) }
        0x10292e00 => { vm.decode(0x8ee32fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292e04u64) }
        0x10292e04 => { vm.decode(0x4a8034f5u32); let d = 10i16; let s = 6i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292e08u64) }
        0x10292e08 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292e0cu64) }
        0x10292e0c => { vm.decode(0x2884cb2fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292e10u64) }
        0x10292e10 => { vm.decode(0x340e2835u32); let d = 19i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292e14u64) }
        0x10292e14 => { vm.decode(0x1c52bb5u32); let d = 14i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10292e18u64) }
        0x10292e18 => { vm.decode(0x600b6d35u32); let d = 10i16; let s = 14i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10292e1cu64) }
        0x10292e1c => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292e20u64) }
        0x10292e20 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292e24u64) }
        0x10292e24 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292e28u64) }
        0x10292e28 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292e2cu64) }
        0x10292e2c => { vm.decode(0x2880eb6fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292e30u64) }
        0x10292e30 => { vm.decode(0x350e0835u32); let d = 22i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292e34u64) }
        0x10292e34 => { vm.decode(0x2884eb2fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292e38u64) }
        0x10292e38 => { vm.decode(0x80b2fu32); let d = 10i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292e3cu64) }
        0x10292e3c => { vm.decode(0x32152835u32); let d = 9i16; let s = 10i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292e40u64) }
        0x10292e40 => { vm.decode(0x19209c70u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292e44u64) }
        0x10292e44 => { vm.decode(0x504d4349u32); let d = 10i16; let s = 2i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10292e48u64) }
        0x10292e48 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292e4cu64) }
        0x10292e4c => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292e50u64) }
        0x10292e50 => { vm.decode(0x6a8014f5u32); let d = 10i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292e54u64) }
        0x10292e54 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292e58u64) }
        0x10292e58 => { vm.decode(0x8ce36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292e5cu64) }
        0x10292e5c => { vm.decode(0x15442835u32); let d = 21i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292e60u64) }
        0x10292e60 => { vm.decode(0x544e4349u32); let d = 10i16; let s = 24i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10292e64u64) }
        0x10292e64 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292e68u64) }
        0x10292e68 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292e6cu64) }
        0x10292e6c => { vm.decode(0x6a8014f5u32); let d = 10i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292e70u64) }
        0x10292e70 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292e74u64) }
        0x10292e74 => { vm.decode(0x88cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292e78u64) }
        0x10292e78 => { vm.decode(0x16440835u32); let d = 24i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292e7cu64) }
        0x10292e7c => { vm.decode(0x41c20bb5u32); let d = 11i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10292e80u64) }
        0x10292e80 => { vm.decode(0x608b4d35u32); let d = 10i16; let s = 11i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10292e84u64) }
        0x10292e84 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292e88u64) }
        0x10292e88 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292e8cu64) }
        0x10292e8c => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292e90u64) }
        0x10292e90 => { vm.decode(0x544cd349u32); let d = 30i16; let s = 16i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10292e94u64) }
        0x10292e94 => { vm.decode(0xfb6d4909u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292e98u64) }
        0x10292e98 => { vm.decode(0xf308dd75u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292e9cu64) }
        0x10292e9c => { vm.decode(0x6ba614f5u32); let d = 30i16; let s = 3i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292ea0u64) }
        0x10292ea0 => { vm.decode(0x40aa4a2fu32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292ea4u64) }
        0x10292ea4 => { vm.decode(0x105c9835u32); let d = 2i16; let s = 18i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292ea8u64) }
        0x10292ea8 => { vm.decode(0x9109db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292eacu64) }
        0x10292eac => { vm.decode(0x407f5349u32); let d = 18i16; let s = 15i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10292eb0u64) }
        0x10292eb0 => { vm.decode(0x9b614909u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292eb4u64) }
        0x10292eb4 => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292eb8u64) }
        0x10292eb8 => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292ebcu64) }
        0x10292ebc => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292ec0u64) }
        0x10292ec0 => { vm.decode(0x888e32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292ec4u64) }
        0x10292ec4 => { vm.decode(0x4c1835u32); let d = 2i16; let s = 2i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292ec8u64) }
        0x10292ec8 => { vm.decode(0x9281db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292eccu64) }
        0x10292ecc => { vm.decode(0x406f5349u32); let d = 18i16; let s = 14i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10292ed0u64) }
        0x10292ed0 => { vm.decode(0x9b614909u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292ed4u64) }
        0x10292ed4 => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292ed8u64) }
        0x10292ed8 => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292edcu64) }
        0x10292edc => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292ee0u64) }
        0x10292ee0 => { vm.decode(0x205d1835u32); let d = 2i16; let s = 18i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292ee4u64) }
        0x10292ee4 => { vm.decode(0x9089db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292ee8u64) }
        0x10292ee8 => { vm.decode(0x406e5349u32); let d = 18i16; let s = 12i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10292eecu64) }
        0x10292eec => { vm.decode(0x9b614909u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292ef0u64) }
        0x10292ef0 => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292ef4u64) }
        0x10292ef4 => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292ef8u64) }
        0x10292ef8 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292efcu64) }
        0x10292efc => { vm.decode(0x1d1835u32); let d = 2i16; let s = 18i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292f00u64) }
        0x10292f00 => { vm.decode(0x9201db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292f04u64) }
        0x10292f04 => { vm.decode(0xd105a11u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10292f08u64) }
        0x10292f08 => { vm.decode(0x405e4349u32); let d = 2i16; let s = 9i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10292f0cu64) }
        0x10292f0c => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292f10u64) }
        0x10292f10 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292f14u64) }
        0x10292f14 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292f18u64) }
        0x10292f18 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292f1cu64) }
        0x10292f1c => { vm.decode(0xa82c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292f20u64) }
        0x10292f20 => { vm.decode(0xd0835u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292f24u64) }
        0x10292f24 => { vm.decode(0x9009d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292f28u64) }
        0x10292f28 => { vm.decode(0x5d98e911u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10292f2cu64) }
        0x10292f2c => { vm.decode(0x447d4349u32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10292f30u64) }
        0x10292f30 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292f34u64) }
        0x10292f34 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292f38u64) }
        0x10292f38 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292f3cu64) }
        0x10292f3c => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292f40u64) }
        0x10292f40 => { vm.decode(0x201e0835u32); let d = 2i16; let s = 2i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292f44u64) }
        0x10292f44 => { vm.decode(0x9209db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292f48u64) }
        0x10292f48 => { vm.decode(0x405c4349u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10292f4cu64) }
        0x10292f4c => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292f50u64) }
        0x10292f50 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292f54u64) }
        0x10292f54 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292f58u64) }
        0x10292f58 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292f5cu64) }
        0x10292f5c => { vm.decode(0x2280e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292f60u64) }
        0x10292f60 => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292f64u64) }
        0x10292f64 => { vm.decode(0x19289c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292f68u64) }
        0x10292f68 => { vm.decode(0x407c4349u32); let d = 2i16; let s = 5i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10292f6cu64) }
        0x10292f6c => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292f70u64) }
        0x10292f70 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292f74u64) }
        0x10292f74 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292f78u64) }
        0x10292f78 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292f7cu64) }
        0x10292f7c => { vm.decode(0x2a84e32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292f80u64) }
        0x10292f80 => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292f84u64) }
        0x10292f84 => { vm.decode(0x19009cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292f88u64) }
        0x10292f88 => { vm.decode(0x407d4349u32); let d = 2i16; let s = 7i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10292f8cu64) }
        0x10292f8c => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292f90u64) }
        0x10292f90 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292f94u64) }
        0x10292f94 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292f98u64) }
        0x10292f98 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292f9cu64) }
        0x10292f9c => { vm.decode(0x20572835u32); let d = 1i16; let s = 2i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292fa0u64) }
        0x10292fa0 => { vm.decode(0x19381c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292fa4u64) }
        0x10292fa4 => { vm.decode(0xce4349u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10292fa8u64) }
        0x10292fa8 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292facu64) }
        0x10292fac => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292fb0u64) }
        0x10292fb0 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292fb4u64) }
        0x10292fb4 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292fb8u64) }
        0x10292fb8 => { vm.decode(0x88e36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292fbcu64) }
        0x10292fbc => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292fc0u64) }
        0x10292fc0 => { vm.decode(0x19281c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292fc4u64) }
        0x10292fc4 => { vm.decode(0xdf4349u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10292fc8u64) }
        0x10292fc8 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10292fccu64) }
        0x10292fcc => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292fd0u64) }
        0x10292fd0 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292fd4u64) }
        0x10292fd4 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292fd8u64) }
        0x10292fd8 => { vm.decode(0x40512835u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292fdcu64) }
        0x10292fdc => { vm.decode(0x19181cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292fe0u64) }
        0x10292fe0 => { vm.decode(0xd2ca916du32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10292fe4u64) }
        0x10292fe4 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10292fe8u64) }
        0x10292fe8 => { vm.decode(0x628ecb2fu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292fecu64) }
        0x10292fec => { vm.decode(0xa241a4f5u32); let d = 1i16; let s = 1i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10292ff0u64) }
        0x10292ff0 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10292ff4u64) }
        0x10292ff4 => { vm.decode(0x60502835u32); let d = 1i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10292ff8u64) }
        0x10292ff8 => { vm.decode(0x19109c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10292ffcu64) }
        0x10292ffc => { vm.decode(0x4d9c6911u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10293000u64) }
        0x10293000 => { vm.decode(0xfd4349u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10293004u64) }
        0x10293004 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10293008u64) }
        0x10293008 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029300cu64) }
        0x1029300c => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10293010u64) }
        0x10293010 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293014u64) }
        0x10293014 => { vm.decode(0x28cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293018u64) }
        0x10293018 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029301cu64) }
        0x1029301c => { vm.decode(0x19301cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10293020u64) }
        0x10293020 => { vm.decode(0x2fb4349u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10293024u64) }
        0x10293024 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10293028u64) }
        0x10293028 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029302cu64) }
        0x1029302c => { vm.decode(0x824024f5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10293030u64) }
        0x10293030 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293034u64) }
        0x10293034 => { vm.decode(0x40122835u32); let d = 1i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10293038u64) }
        0x10293038 => { vm.decode(0x19309c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029303cu64) }
        0x1029303c => { vm.decode(0x6c84349u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10293040u64) }
        0x10293040 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10293044u64) }
        0x10293044 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10293048u64) }
        0x10293048 => { vm.decode(0x824024f5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029304cu64) }
        0x1029304c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293050u64) }
        0x10293050 => { vm.decode(0x60112835u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10293054u64) }
        0x10293054 => { vm.decode(0x19189cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10293058u64) }
        0x10293058 => { vm.decode(0x594f811u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029305cu64) }
        0x1029305c => { vm.decode(0xce4349u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10293060u64) }
        0x10293060 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10293064u64) }
        0x10293064 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10293068u64) }
        0x10293068 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029306cu64) }
        0x1029306c => { vm.decode(0x19089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10293070u64) }
        0x10293070 => { vm.decode(0x2020022fu32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293074u64) }
        0x10293074 => { vm.decode(0x888c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293078u64) }
        0x10293078 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029307cu64) }
        0x1029307c => { vm.decode(0x19209c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10293080u64) }
        0x10293080 => { vm.decode(0x228443afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293084u64) }
        0x10293084 => { vm.decode(0xd2ca812du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10293088u64) }
        0x10293088 => { vm.decode(0x8843efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029308cu64) }
        0x1029308c => { vm.decode(0xd25a412du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10293090u64) }
        0x10293090 => { vm.decode(0x208c63efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293094u64) }
        0x10293094 => { vm.decode(0xda5ac12du32); let d = 10i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10293098u64) }
        0x10293098 => { vm.decode(0x208863efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029309cu64) }
        0x1029309c => { vm.decode(0xda4ec12du32); let d = 12i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102930a0u64) }
        0x102930a0 => { vm.decode(0x228843efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102930a4u64) }
        0x102930a4 => { vm.decode(0xdacec12du32); let d = 13i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102930a8u64) }
        0x102930a8 => { vm.decode(0x228863efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102930acu64) }
        0x102930ac => { vm.decode(0xdadec12du32); let d = 15i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102930b0u64) }
        0x102930b0 => { vm.decode(0x208c43efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102930b4u64) }
        0x102930b4 => { vm.decode(0xdacac1adu32); let d = 25i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102930b8u64) }
        0x102930b8 => { vm.decode(0x228c63afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102930bcu64) }
        0x102930bc => { vm.decode(0xd24ac1adu32); let d = 16i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102930c0u64) }
        0x102930c0 => { vm.decode(0x288843afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102930c4u64) }
        0x102930c4 => { vm.decode(0xd25ac1adu32); let d = 18i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102930c8u64) }
        0x102930c8 => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102930ccu64) }
        0x102930cc => { vm.decode(0xb348bd75u32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102930d0u64) }
        0x102930d0 => { vm.decode(0xb148ad75u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102930d4u64) }
        0x102930d4 => { vm.decode(0xf1401d75u32); let d = 19i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102930d8u64) }
        0x102930d8 => { vm.decode(0xe3408d75u32); let d = 21i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102930dcu64) }
        0x102930dc => { vm.decode(0xe000cd75u32); let d = 20i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102930e0u64) }
        0x102930e0 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102930e4u64) }
        0x102930e4 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102930e8u64) }
        0x102930e8 => { vm.decode(0xa1087d75u32); let d = 10i16; let s = 25i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102930ecu64) }
        0x102930ec => { vm.decode(0x228863afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102930f0u64) }
        0x102930f0 => { vm.decode(0xda4ec12du32); let d = 12i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102930f4u64) }
        0x102930f4 => { vm.decode(0x288863afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102930f8u64) }
        0x102930f8 => { vm.decode(0xdacac1adu32); let d = 25i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102930fcu64) }
        0x102930fc => { vm.decode(0x228c63efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293100u64) }
        0x10293100 => { vm.decode(0xd24ac1adu32); let d = 16i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10293104u64) }
        0x10293104 => { vm.decode(0x2a8843afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293108u64) }
        0x10293108 => { vm.decode(0xd25ec1adu32); let d = 22i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029310cu64) }
        0x1029310c => { vm.decode(0x228c43efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293110u64) }
        0x10293110 => { vm.decode(0xda5ec1adu32); let d = 30i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10293114u64) }
        0x10293114 => { vm.decode(0xf3084d75u32); let d = 24i16; let s = 30i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10293118u64) }
        0x10293118 => { vm.decode(0xb2405d75u32); let d = 3i16; let s = 22i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029311cu64) }
        0x1029311c => { vm.decode(0xa000cd75u32); let d = 4i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10293120u64) }
        0x10293120 => { vm.decode(0xa140ed75u32); let d = 5i16; let s = 25i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10293124u64) }
        0x10293124 => { vm.decode(0xf1009d75u32); let d = 22i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10293128u64) }
        0x10293128 => { vm.decode(0xda6e812du32); let d = 12i16; let s = 17i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029312cu64) }
        0x1029312c => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10293130u64) }
        0x10293130 => { vm.decode(0x2225b4f5u32); let d = 16i16; let s = 12i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10293134u64) }
        0x10293134 => { vm.decode(0x62c5b4f5u32); let d = 25i16; let s = 10i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10293138u64) }
        0x10293138 => { vm.decode(0xa283a4f5u32); let d = 12i16; let s = 1i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029313cu64) }
        0x1029313c => { vm.decode(0x6a81a4f5u32); let d = 10i16; let s = 2i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10293140u64) }
        0x10293140 => { vm.decode(0x2361a4f5u32); let d = 1i16; let s = 20i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10293144u64) }
        0x10293144 => { vm.decode(0x6327a4f5u32); let d = 20i16; let s = 22i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10293148u64) }
        0x10293148 => { vm.decode(0xab21a4f5u32); let d = 2i16; let s = 21i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029314cu64) }
        0x1029314c => { vm.decode(0xb089530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10293150u64) }
        0x10293150 => { vm.decode(0xeb01a4f5u32); let d = 2i16; let s = 19i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10293154u64) }
        0x10293154 => { vm.decode(0xb009530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10293158u64) }
        0x10293158 => { vm.decode(0xaa65b4f5u32); let d = 19i16; let s = 13i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029315cu64) }
        0x1029315c => { vm.decode(0xeae3b4f5u32); let d = 15i16; let s = 15i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10293160u64) }
        0x10293160 => { vm.decode(0x6b01a4f5u32); let d = 2i16; let s = 18i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10293164u64) }
        0x10293164 => { vm.decode(0xb3815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10293168u64) }
        0x10293168 => { vm.decode(0xa267a4f5u32); let d = 21i16; let s = 5i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029316cu64) }
        0x1029316c => { vm.decode(0x2a25a4f5u32); let d = 18i16; let s = 4i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10293170u64) }
        0x10293170 => { vm.decode(0xe2c3a4f5u32); let d = 13i16; let s = 3i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10293174u64) }
        0x10293174 => { vm.decode(0x2b01b4f5u32); let d = 2i16; let s = 24i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10293178u64) }
        0x10293178 => { vm.decode(0xb109530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029317cu64) }
        0x1029317c => { vm.decode(0x45906011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10293180u64) }
        0x10293180 => { vm.decode(0x42494349u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10293184u64) }
        0x10293184 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10293188u64) }
        0x10293188 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029318cu64) }
        0x1029318c => { vm.decode(0x4a0024f5u32); let d = 2i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10293190u64) }
        0x10293190 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293194u64) }
        0x10293194 => { vm.decode(0x208ce36fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293198u64) }
        0x10293198 => { vm.decode(0x204c0835u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029319cu64) }
        0x1029319c => { vm.decode(0x42eb4349u32); let d = 3i16; let s = 14i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102931a0u64) }
        0x102931a0 => { vm.decode(0x9a23c909u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102931a4u64) }
        0x102931a4 => { vm.decode(0xa2403d75u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102931a8u64) }
        0x102931a8 => { vm.decode(0xca4024f5u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102931acu64) }
        0x102931ac => { vm.decode(0x2028012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102931b0u64) }
        0x102931b0 => { vm.decode(0xa86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102931b4u64) }
        0x102931b4 => { vm.decode(0x430d0835u32); let d = 14i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102931b8u64) }
        0x102931b8 => { vm.decode(0x42da4349u32); let d = 3i16; let s = 9i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102931bcu64) }
        0x102931bc => { vm.decode(0x9a23c909u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102931c0u64) }
        0x102931c0 => { vm.decode(0xa2403d75u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102931c4u64) }
        0x102931c4 => { vm.decode(0xca4024f5u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102931c8u64) }
        0x102931c8 => { vm.decode(0x2028012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102931ccu64) }
        0x102931cc => { vm.decode(0x86c36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102931d0u64) }
        0x102931d0 => { vm.decode(0x400d2835u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102931d4u64) }
        0x102931d4 => { vm.decode(0x25bc349u32); let d = 4i16; let s = 11i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102931d8u64) }
        0x102931d8 => { vm.decode(0xc8254909u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102931dcu64) }
        0x102931dc => { vm.decode(0xb0008d75u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102931e0u64) }
        0x102931e0 => { vm.decode(0x22224f5u32); let d = 4i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102931e4u64) }
        0x102931e4 => { vm.decode(0x82002fu32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102931e8u64) }
        0x102931e8 => { vm.decode(0x2886e32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102931ecu64) }
        0x102931ec => { vm.decode(0x21018835u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102931f0u64) }
        0x102931f0 => { vm.decode(0xd104311u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102931f4u64) }
        0x102931f4 => { vm.decode(0x2f8c349u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102931f8u64) }
        0x102931f8 => { vm.decode(0xc827c909u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102931fcu64) }
        0x102931fc => { vm.decode(0xb040ad75u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10293200u64) }
        0x10293200 => { vm.decode(0x826224f5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10293204u64) }
        0x10293204 => { vm.decode(0x2082012fu32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293208u64) }
        0x10293208 => { vm.decode(0x2880cb2fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029320cu64) }
        0x1029320c => { vm.decode(0x6102a835u32); let d = 5i16; let s = 5i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10293210u64) }
        0x10293210 => { vm.decode(0x52ca4349u32); let d = 11i16; let s = 8i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10293214u64) }
        0x10293214 => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10293218u64) }
        0x10293218 => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029321cu64) }
        0x1029321c => { vm.decode(0xcac034f5u32); let d = 11i16; let s = 6i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10293220u64) }
        0x10293220 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293224u64) }
        0x10293224 => { vm.decode(0x284eb2fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293228u64) }
        0x10293228 => { vm.decode(0x520e2835u32); let d = 11i16; let s = 8i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029322cu64) }
        0x1029322c => { vm.decode(0x5907811u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10293230u64) }
        0x10293230 => { vm.decode(0x124a5349u32); let d = 24i16; let s = 8i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10293234u64) }
        0x10293234 => { vm.decode(0xa9694909u32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10293238u64) }
        0x10293238 => { vm.decode(0xe1084d75u32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029323cu64) }
        0x1029323c => { vm.decode(0x38434f5u32); let d = 24i16; let s = 6i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10293240u64) }
        0x10293240 => { vm.decode(0x40004a2fu32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293244u64) }
        0x10293244 => { vm.decode(0x80eb6fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293248u64) }
        0x10293248 => { vm.decode(0x16021835u32); let d = 24i16; let s = 8i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029324cu64) }
        0x1029324c => { vm.decode(0x518f811u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10293250u64) }
        0x10293250 => { vm.decode(0x2ca5349u32); let d = 17i16; let s = 8i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10293254u64) }
        0x10293254 => { vm.decode(0x8963c909u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10293258u64) }
        0x10293258 => { vm.decode(0xe0406d75u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029325cu64) }
        0x1029325c => { vm.decode(0x834424f5u32); let d = 17i16; let s = 6i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10293260u64) }
        0x10293260 => { vm.decode(0x6000412fu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293264u64) }
        0x10293264 => { vm.decode(0x280eb6fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293268u64) }
        0x10293268 => { vm.decode(0x44023835u32); let d = 17i16; let s = 8i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029326cu64) }
        0x1029326c => { vm.decode(0x46f9d349u32); let d = 23i16; let s = 23i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10293270u64) }
        0x10293270 => { vm.decode(0xdb67c909u32); let d = 23i16; let s = 23i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10293274u64) }
        0x10293274 => { vm.decode(0xf240fd75u32); let d = 23i16; let s = 23i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10293278u64) }
        0x10293278 => { vm.decode(0xcb6624f5u32); let d = 23i16; let s = 6i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029327cu64) }
        0x1029327c => { vm.decode(0x60aa412fu32); let d = 23i16; let s = 23i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293280u64) }
        0x10293280 => { vm.decode(0x884cb2fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293284u64) }
        0x10293284 => { vm.decode(0x4206b835u32); let d = 9i16; let s = 8i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10293288u64) }
        0x10293288 => { vm.decode(0x52f9d349u32); let d = 31i16; let s = 7i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029328cu64) }
        0x1029328c => { vm.decode(0xfb6fc909u32); let d = 31i16; let s = 31i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10293290u64) }
        0x10293290 => { vm.decode(0xf348fd75u32); let d = 31i16; let s = 31i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10293294u64) }
        0x10293294 => { vm.decode(0xcbe634f5u32); let d = 31i16; let s = 6i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10293298u64) }
        0x10293298 => { vm.decode(0x60aa4b2fu32); let d = 31i16; let s = 31i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029329cu64) }
        0x1029329c => { vm.decode(0x288ae36fu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102932a0u64) }
        0x102932a0 => { vm.decode(0x774db835u32); let d = 31i16; let s = 7i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102932a4u64) }
        0x102932a4 => { vm.decode(0x4d9ce911u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102932a8u64) }
        0x102932a8 => { vm.decode(0x4279d349u32); let d = 22i16; let s = 7i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102932acu64) }
        0x102932ac => { vm.decode(0xdb654909u32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102932b0u64) }
        0x102932b0 => { vm.decode(0xf200dd75u32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102932b4u64) }
        0x102932b4 => { vm.decode(0x4b2624f5u32); let d = 22i16; let s = 6i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102932b8u64) }
        0x102932b8 => { vm.decode(0x40aa402fu32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102932bcu64) }
        0x102932bc => { vm.decode(0x288ec36fu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102932c0u64) }
        0x102932c0 => { vm.decode(0x254d9835u32); let d = 22i16; let s = 7i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102932c4u64) }
        0x102932c4 => { vm.decode(0x4d10c311u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102932c8u64) }
        0x102932c8 => { vm.decode(0x5279d349u32); let d = 30i16; let s = 7i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102932ccu64) }
        0x102932cc => { vm.decode(0xfb6d4909u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102932d0u64) }
        0x102932d0 => { vm.decode(0xf308dd75u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102932d4u64) }
        0x102932d4 => { vm.decode(0x4ba634f5u32); let d = 30i16; let s = 6i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102932d8u64) }
        0x102932d8 => { vm.decode(0x40aa4a2fu32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102932dcu64) }
        0x102932dc => { vm.decode(0x228ec36fu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102932e0u64) }
        0x102932e0 => { vm.decode(0x374d9835u32); let d = 30i16; let s = 7i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102932e4u64) }
        0x102932e4 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102932e8u64) }
        0x102932e8 => { vm.decode(0x60422835u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102932ecu64) }
        0x102932ec => { vm.decode(0x208e63afu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102932f0u64) }
        0x102932f0 => { vm.decode(0x20a8092fu32); let d = 11i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102932f4u64) }
        0x102932f4 => { vm.decode(0xa80cb2fu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102932f8u64) }
        0x102932f8 => { vm.decode(0x520e2835u32); let d = 11i16; let s = 8i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102932fcu64) }
        0x102932fc => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293300u64) }
        0x10293300 => { vm.decode(0x350d2835u32); let d = 23i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10293304u64) }
        0x10293304 => { vm.decode(0x2082022fu32); let d = 5i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293308u64) }
        0x10293308 => { vm.decode(0x41018835u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029330cu64) }
        0x1029330c => { vm.decode(0x2002432fu32); let d = 5i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293310u64) }
        0x10293310 => { vm.decode(0x62408835u32); let d = 8i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10293314u64) }
        0x10293314 => { vm.decode(0x2008402fu32); let d = 3i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293318u64) }
        0x10293318 => { vm.decode(0x15144211u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029331cu64) }
        0x1029331c => { vm.decode(0x404f2835u32); let d = 3i16; let s = 3i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10293320u64) }
        0x10293320 => { vm.decode(0x2082402fu32); let d = 5i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293324u64) }
        0x10293324 => { vm.decode(0x45408835u32); let d = 20i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10293328u64) }
        0x10293328 => { vm.decode(0x42859019u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029332cu64) }
        0x1029332c => { vm.decode(0x1b0855b0u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10293330u64) }
        0x10293330 => { vm.decode(0x44a09875u32); let d = 7i16; let s = 0i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10293334u64) }
        0x10293334 => { vm.decode(0x8900036u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10293338u64) }
        0x10293338 => { vm.decode(0xa8032fu32); let d = 2i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029333cu64) }
        0x1029333c => { vm.decode(0x2082032fu32); let d = 5i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293340u64) }
        0x10293340 => { vm.decode(0x28492fu32); let d = 10i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293344u64) }
        0x10293344 => { vm.decode(0x2028482fu32); let d = 11i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293348u64) }
        0x10293348 => { vm.decode(0x8824bafu32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029334cu64) }
        0x1029334c => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293350u64) }
        0x10293350 => { vm.decode(0x2082492fu32); let d = 13i16; let s = 21i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293354u64) }
        0x10293354 => { vm.decode(0x88a6bafu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293358u64) }
        0x10293358 => { vm.decode(0xaa0a2fu32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029335cu64) }
        0x1029335c => { vm.decode(0x228e6befu32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293360u64) }
        0x10293360 => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293364u64) }
        0x10293364 => { vm.decode(0x6880eb2fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293368u64) }
        0x10293368 => { vm.decode(0x60004b2fu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029336cu64) }
        0x1029336c => { vm.decode(0x4a8ce32fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293370u64) }
        0x10293370 => { vm.decode(0x535ca835u32); let d = 15i16; let s = 15i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10293374u64) }
        0x10293374 => { vm.decode(0x4a88c36fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293378u64) }
        0x10293378 => { vm.decode(0x145c8835u32); let d = 18i16; let s = 14i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029337cu64) }
        0x1029337c => { vm.decode(0x5353a835u32); let d = 13i16; let s = 13i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10293380u64) }
        0x10293380 => { vm.decode(0x1459a835u32); let d = 19i16; let s = 12i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10293384u64) }
        0x10293384 => { vm.decode(0x725f2835u32); let d = 11i16; let s = 11i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10293388u64) }
        0x10293388 => { vm.decode(0x35062835u32); let d = 21i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029338cu64) }
        0x1029338c => { vm.decode(0x6110a835u32); let d = 5i16; let s = 5i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10293390u64) }
        0x10293390 => { vm.decode(0x2162835u32); let d = 9i16; let s = 2i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10293394u64) }
        0x10293394 => { vm.decode(0x888c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293398u64) }
        0x10293398 => { vm.decode(0x50481835u32); let d = 2i16; let s = 2i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029339cu64) }
        0x1029339c => { vm.decode(0x459cf011u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102933a0u64) }
        0x102933a0 => { vm.decode(0x524b4349u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102933a4u64) }
        0x102933a4 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102933a8u64) }
        0x102933a8 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102933acu64) }
        0x102933ac => { vm.decode(0x4a8034f5u32); let d = 10i16; let s = 6i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102933b0u64) }
        0x102933b0 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102933b4u64) }
        0x102933b4 => { vm.decode(0x104c0835u32); let d = 2i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102933b8u64) }
        0x102933b8 => { vm.decode(0xa8a43afu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102933bcu64) }
        0x102933bc => { vm.decode(0xa8082fu32); let d = 10i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102933c0u64) }
        0x102933c0 => { vm.decode(0x104c0835u32); let d = 2i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102933c4u64) }
        0x102933c4 => { vm.decode(0x50480835u32); let d = 2i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102933c8u64) }
        0x102933c8 => { vm.decode(0xb109530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102933ccu64) }
        0x102933cc => { vm.decode(0x60099835u32); let d = 2i16; let s = 5i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102933d0u64) }
        0x102933d0 => { vm.decode(0xb3815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102933d4u64) }
        0x102933d4 => { vm.decode(0xa08875u32); let d = 5i16; let s = 0i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102933d8u64) }
        0x102933d8 => { vm.decode(0x604e1835u32); let d = 2i16; let s = 11i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102933dcu64) }
        0x102933dc => { vm.decode(0xb089530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102933e0u64) }
        0x102933e0 => { vm.decode(0x200f1835u32); let d = 2i16; let s = 13i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102933e4u64) }
        0x102933e4 => { vm.decode(0xb009530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102933e8u64) }
        0x102933e8 => { vm.decode(0x274b9835u32); let d = 30i16; let s = 15i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102933ecu64) }
        0x102933ec => { vm.decode(0x37482835u32); let d = 31i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102933f0u64) }
        0x102933f0 => { vm.decode(0x46059835u32); let d = 24i16; let s = 4i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102933f4u64) }
        0x102933f4 => { vm.decode(0x620ca835u32); let d = 11i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102933f8u64) }
        0x102933f8 => { vm.decode(0x20122200u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 11i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102933fcu64) }
        0x102933fc => { vm.decode(0xa78329f1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -913i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10293400u64).wrapping_add((imm * 4) as u64) } else { 0x10293400u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10293400u64) }
        0x10293400 => { vm.decode(0x6005875u32); let d = 8i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10293404u64) }
        0x10293404 => { vm.decode(0x208473afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293408u64) }
        0x10293408 => { vm.decode(0x8c53afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029340cu64) }
        0x1029340c => { vm.decode(0x228873afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 1560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293410u64) }
        0x10293410 => { vm.decode(0x401e2835u32); let d = 3i16; let s = 3i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10293414u64) }
        0x10293414 => { vm.decode(0x205f0835u32); let d = 2i16; let s = 2i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10293418u64) }
        0x10293418 => { vm.decode(0x40532835u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029341cu64) }
        0x1029341c => { vm.decode(0x28653afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 1584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293420u64) }
        0x10293420 => { vm.decode(0x288243afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293424u64) }
        0x10293424 => { vm.decode(0x21018835u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10293428u64) }
        0x10293428 => { vm.decode(0x2286e3efu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029342cu64) }
        0x1029342c => { vm.decode(0x88ac3afu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293430u64) }
        0x10293430 => { vm.decode(0x288ae3afu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293434u64) }
        0x10293434 => { vm.decode(0xa80cbafu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293438u64) }
        0x10293438 => { vm.decode(0x1028835u32); let d = 4i16; let s = 4i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029343cu64) }
        0x1029343c => { vm.decode(0x60412835u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10293440u64) }
        0x10293440 => { vm.decode(0x4d0835u32); let d = 2i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10293444u64) }
        0x10293444 => { vm.decode(0x600d2835u32); let d = 3i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10293448u64) }
        0x10293448 => { vm.decode(0x228253afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 1552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029344cu64) }
        0x1029344c => { vm.decode(0x6142a835u32); let d = 5i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10293450u64) }
        0x10293450 => { vm.decode(0x28ec3efu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293454u64) }
        0x10293454 => { vm.decode(0x4141a835u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10293458u64) }
        0x10293458 => { vm.decode(0x1b305cb0u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029345cu64) }
        0x1029345c => { vm.decode(0x1b381db0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10293460u64) }
        0x10293460 => { vm.decode(0xb009d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10293464u64) }
        0x10293464 => { vm.decode(0x1b089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10293468u64) }
        0x10293468 => { vm.decode(0xb10dc30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029346cu64) }
        0x1029346c => { vm.decode(0x228473afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293470u64) }
        0x10293470 => { vm.decode(0x88863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293474u64) }
        0x10293474 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10293478u64) }
        0x10293478 => { vm.decode(0xa88e3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029347cu64) }
        0x1029347c => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10293480u64) }
        0x10293480 => { vm.decode(0x1b189c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10293484u64) }
        0x10293484 => { vm.decode(0x208053efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293488u64) }
        0x10293488 => { vm.decode(0x28c63efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029348cu64) }
        0x1029348c => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10293490u64) }
        0x10293490 => { vm.decode(0x88cc3afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293494u64) }
        0x10293494 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10293498u64) }
        0x10293498 => { vm.decode(0xa8c436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029349cu64) }
        0x1029349c => { vm.decode(0x949119u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = 104i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102934a0u64) }
        0x102934a0 => { vm.decode(0x88436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102934a4u64) }
        0x102934a4 => { vm.decode(0x82c42099u32); let d = 4i16; let s = 2i16; let s2 = -1i16; let imm = 31i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102934a8u64) }
        0x102934a8 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102934acu64) }
        0x102934ac => { vm.decode(0x1b009cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102934b0u64) }
        0x102934b0 => { vm.decode(0x208073afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102934b4u64) }
        0x102934b4 => { vm.decode(0x2a8243afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102934b8u64) }
        0x102934b8 => { vm.decode(0x60012835u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102934bcu64) }
        0x102934bc => { vm.decode(0x2886e3afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102934c0u64) }
        0x102934c0 => { vm.decode(0x60012835u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102934c4u64) }
        0x102934c4 => { vm.decode(0x1b089cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102934c8u64) }
        0x102934c8 => { vm.decode(0xa0000200u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102934ccu64) }
        0x102934cc => { vm.decode(0xc8827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 19i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102934d0u64).wrapping_add((imm * 4) as u64) } else { 0x102934d0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102934d0u64) }
        0x102934d0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102934d4u64) } }
        0x102934d4 => { vm.decode(0x10a080a3u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 6i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 2)?;
        let v = u16::from_le_bytes(b.try_into().unwrap()) as u64;
        vm.st(d, v);
        Ok(0x102934d8u64) }
        0x102934d8 => { vm.decode(0x40020033u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102934dcu64) }
        0x102934dc => { vm.decode(0x2020012fu32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102934e0u64) }
        0x102934e0 => { vm.decode(0x22200f5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 40i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102934e4u64) }
        0x102934e4 => { vm.decode(0xdd9e77b3u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = -1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102934e8u64) }
        0x102934e8 => { vm.decode(0xc77ef59u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = -4i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102934ecu64) }
        0x102934ec => { vm.decode(0x42848019u32); let d = 5i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102934f0u64) }
        0x102934f0 => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102934f4u64) }
        0x102934f4 => { vm.decode(0x2a012fu32); let d = 6i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102934f8u64) }
        0x102934f8 => { vm.decode(0x134443b5u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102934fcu64) }
        0x102934fc => { vm.decode(0x7ddc77b3u32); let d = 6i16; let s = 4i16; let s2 = -1i16; let imm = -2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10293500u64) }
        0x10293500 => { vm.decode(0x2028012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293504u64) }
        0x10293504 => { vm.decode(0x918403b5u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10293508u64) }
        0x10293508 => { vm.decode(0xfd9e7733u32); let d = 3i16; let s = 4i16; let s2 = -1i16; let imm = -3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x1029350cu64) }
        0x1029350c => { vm.decode(0x4800875u32); let d = 4i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10293510u64) }
        0x10293510 => { vm.decode(0xa0000200u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x10293514u64) }
        0x10293514 => { vm.decode(0xa7d32df1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -17i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10293518u64).wrapping_add((imm * 4) as u64) } else { 0x10293518u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10293518u64) }
        0x10293518 => { vm.decode(0x4400c875u32); let d = 3i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029351cu64) }
        0x1029351c => { vm.decode(0x2a8463afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 696i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293520u64) }
        0x10293520 => { vm.decode(0x88c63afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293524u64) }
        0x10293524 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10293528u64) }
        0x10293528 => { vm.decode(0x2888636fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 200i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029352cu64) }
        0x1029352c => { vm.decode(0x10400430u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10293530u64) }
        0x10293530 => { vm.decode(0x288043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 704i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293534u64) }
        0x10293534 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293538u64) }
        0x10293538 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029353cu64) }
        0x1029353c => { vm.decode(0x10480430u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10293540u64) }
        0x10293540 => { vm.decode(0x408c436fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293544u64) }
        0x10293544 => { vm.decode(0x120d19du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 289120i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10293548u64) }
        0x10293548 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1029354cu64) } }
        0x1029354c => { vm.decode(0x1d984811u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10293550u64) }
        0x10293550 => { vm.decode(0x4880436fu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293554u64) }
        0x10293554 => { vm.decode(0x208c101du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 70404i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10293558u64) }
        0x10293558 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1029355cu64) } }
        0x1029355c => { vm.decode(0x1d984811u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10293560u64) }
        0x10293560 => { vm.decode(0x4880436fu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293564u64) }
        0x10293564 => { vm.decode(0x2928c41du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 84092i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10293568u64) }
        0x10293568 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1029356cu64) } }
        0x1029356c => { vm.decode(0x1d984011u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10293570u64) }
        0x10293570 => { vm.decode(0x4a84432fu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293574u64) }
        0x10293574 => { vm.decode(0xad2cd11du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 97772i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10293578u64) }
        0x10293578 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1029357cu64) } }
        0x1029357c => { vm.decode(0x1d984811u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10293580u64) }
        0x10293580 => { vm.decode(0x4880436fu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10293584u64) }
        0x10293584 => { vm.decode(0x29ec111du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 111468i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x10293588u64) }
        0x10293588 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x1029358cu64) } }
        0x1029358c => { vm.decode(0x1d984811u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10293590u64) }
        0x10297684 => { vm.decode(0x640d8059u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10297688u64) }
        0x10297688 => { vm.decode(0x191094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029768cu64) }
        0x1029768c => { vm.decode(0x40528119u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 240i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10297690u64) }
        0x10297690 => { vm.decode(0x19381430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10297694u64) }
        0x10297694 => { vm.decode(0x641d8419u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10297698u64) }
        0x10297698 => { vm.decode(0x19301430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029769cu64) }
        0x1029769c => { vm.decode(0x642d8019u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102976a0u64) }
        0x102976a0 => { vm.decode(0x19281430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102976a4u64) }
        0x102976a4 => { vm.decode(0x32020000u32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102976a8u64) }
        0x102976a8 => { vm.decode(0x48227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 9i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102976acu64).wrapping_add((imm * 4) as u64) } else { 0x102976acu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102976acu64) }
        0x102976ac => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102976b0u64) } }
        0x102976b0 => { vm.decode(0x428051du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 74832i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102976b4u64) }
        0x102976b4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102976b8u64) } }
        0x102987d0 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102987d4u64) }
        0x102987d4 => { vm.decode(0x2a00212fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 152i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102987d8u64) }
        0x102987d8 => { vm.decode(0x41159u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 328i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102987dcu64) }
        0x102987dc => { vm.decode(0x9399db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102987e0u64) }
        0x102987e0 => { vm.decode(0x8b21c909u32); let d = 2i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102987e4u64) }
        0x102987e4 => { vm.decode(0x9319db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102987e8u64) }
        0x102987e8 => { vm.decode(0x89299cb0u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 2536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102987ecu64) }
        0x102987ec => { vm.decode(0x640d1059u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102987f0u64) }
        0x102987f0 => { vm.decode(0x9219db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 2528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102987f4u64) }
        0x102987f4 => { vm.decode(0x19399c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102987f8u64) }
        0x102987f8 => { vm.decode(0x286432fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102987fcu64) }
        0x102987fc => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298800u64) }
        0x10298800 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10298804u64 + 4);
        vm.wr(37, 0x10298804u64 + 4);
        Ok(0x10298804u64) }
        0x10298804 => { vm.decode(0x624f8559u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2520i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10298808u64) }
        0x10298808 => { vm.decode(0xa82636fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 216i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029880cu64) }
        0x1029880c => { vm.decode(0x631d1162u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10298810u64) }
        0x10298810 => { vm.decode(0xd94e211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 872i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10298814u64) }
        0x10298814 => { vm.decode(0x8a21c909u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10298818u64) }
        0x10298818 => { vm.decode(0x480024f5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029881cu64) }
        0x1029881c => { vm.decode(0x10002236u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 128i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10298820u64) }
        0x10298820 => { vm.decode(0x20220033u32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10298824u64) }
        0x10298824 => { vm.decode(0x8001236u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10298828u64) }
        0x10298828 => { vm.decode(0xa0006000u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 33i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x1029882cu64) }
        0x1029882c => { vm.decode(0x3222871u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 1079i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10298830u64).wrapping_add((imm * 4) as u64) } else { 0x10298830u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10298830u64) }
        0x10298830 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x10298834u64) } }
        0x10298834 => { vm.decode(0x2a8063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 664i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298838u64) }
        0x10298838 => { vm.decode(0x2200016fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029883cu64) }
        0x1029883c => { vm.decode(0x2276u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 64i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10298840u64) }
        0x10298840 => { vm.decode(0x100036u32); let d = 16i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10298844u64) }
        0x10298844 => { vm.decode(0x52102b73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 2648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x10298848u64) }
        0x10298848 => { vm.decode(0x1b291430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029884cu64) }
        0x1029884c => { vm.decode(0x80888b35u32); let d = 1i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10298850u64) }
        0x10298850 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10298854u64) }
        0x10298854 => { vm.decode(0x1b2114b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298858u64) }
        0x10298858 => { vm.decode(0x98234909u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029885cu64) }
        0x1029885c => { vm.decode(0x740d8059u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10298860u64) }
        0x10298860 => { vm.decode(0xa04104f5u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298864u64) }
        0x10298864 => { vm.decode(0x1b311430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 2640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298868u64) }
        0x10298868 => { vm.decode(0x86c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 288i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029886cu64) }
        0x1029886c => { vm.decode(0x68844bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298870u64) }
        0x10298870 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x10298874u64 + 4);
        vm.wr(37, 0x10298874u64 + 4);
        Ok(0x10298874u64) }
        0x10298874 => { vm.decode(0x660d8519u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 2632i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10298878u64) }
        0x10298878 => { vm.decode(0x2884e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1000i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029887cu64) }
        0x1029887c => { vm.decode(0x19101cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298880u64) }
        0x10298880 => { vm.decode(0x2884c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 992i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298884u64) }
        0x10298884 => { vm.decode(0x19081cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298888u64) }
        0x10298888 => { vm.decode(0x2a80e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 984i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029888cu64) }
        0x1029888c => { vm.decode(0x19181c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298890u64) }
        0x10298890 => { vm.decode(0x2a80c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 976i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298894u64) }
        0x10298894 => { vm.decode(0x19101c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298898u64) }
        0x10298898 => { vm.decode(0x2880e3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 968i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029889cu64) }
        0x1029889c => { vm.decode(0x19081c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102988a0u64) }
        0x102988a0 => { vm.decode(0x2880c3efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 960i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102988a4u64) }
        0x102988a4 => { vm.decode(0x19001c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102988a8u64) }
        0x102988a8 => { vm.decode(0x2a84e3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 952i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102988acu64) }
        0x102988ac => { vm.decode(0x193894b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102988b0u64) }
        0x102988b0 => { vm.decode(0x2a84c3afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 944i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102988b4u64) }
        0x102988b4 => { vm.decode(0x193094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102988b8u64) }
        0x102988b8 => { vm.decode(0x20020200u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102988bcu64) }
        0x102988bc => { vm.decode(0x8c8a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 31i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102988c0u64).wrapping_add((imm * 4) as u64) } else { 0x102988c0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102988c0u64) }
        0x102988c0 => { vm.decode(0x642d1019u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102988c4u64) }
        0x102988c4 => { vm.decode(0x204002beu32); let d = 1i16; let s = 17i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102988c8u64) }
        0x102988c8 => { vm.decode(0x49935u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 48i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102988ccu64) }
        0x102988cc => { vm.decode(0xa0480037u32); let d = 10i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as i8 as i64;
        vm.st(d, v as u64);
        Ok(0x102988d0u64) }
        0x102988d0 => { vm.decode(0x81740935u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 56i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102988d4u64) }
        0x102988d4 => { vm.decode(0x289875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102988d8u64) }
        0x102988d8 => { vm.decode(0x1242023eu32); let d = 10i16; let s = 17i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102988dcu64) }
        0x102988dc => { vm.decode(0x81700935u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 40i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102988e0u64) }
        0x102988e0 => { vm.decode(0x5408875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102988e4u64) }
        0x102988e4 => { vm.decode(0x124202beu32); let d = 10i16; let s = 17i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102988e8u64) }
        0x102988e8 => { vm.decode(0x81300935u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 32i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102988ecu64) }
        0x102988ec => { vm.decode(0x5408875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102988f0u64) }
        0x102988f0 => { vm.decode(0x242027eu32); let d = 10i16; let s = 17i16; let s2 = -1i16; let imm = 4i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102988f4u64) }
        0x102988f4 => { vm.decode(0x32a1d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 24i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102988f8u64) }
        0x102988f8 => { vm.decode(0x5408875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102988fcu64) }
        0x102988fc => { vm.decode(0x24202feu32); let d = 10i16; let s = 17i16; let s2 = -1i16; let imm = 5i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10298900u64) }
        0x10298900 => { vm.decode(0x30a1d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10298904u64) }
        0x10298904 => { vm.decode(0x5408875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298908u64) }
        0x10298908 => { vm.decode(0x61041019u32); let d = 10i16; let s = 17i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029890cu64) }
        0x1029890c => { vm.decode(0x1849019u32); let d = 11i16; let s = 2i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10298910u64) }
        0x10298910 => { vm.decode(0x8900036u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10298914u64) }
        0x10298914 => { vm.decode(0x184202feu32); let d = 12i16; let s = 17i16; let s2 = -1i16; let imm = 7i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x10298918u64) }
        0x10298918 => { vm.decode(0x3842027eu32); let d = 13i16; let s = 17i16; let s2 = -1i16; let imm = 6i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x1029891cu64) }
        0x1029891c => { vm.decode(0x1168ad75u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10298920u64) }
        0x10298920 => { vm.decode(0x7000a835u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298924u64) }
        0x10298924 => { vm.decode(0x3000a835u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298928u64) }
        0x10298928 => { vm.decode(0x10400030u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029892cu64) }
        0x1029892c => { vm.decode(0x44201875u32); let d = 2i16; let s = 0i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298930u64) }
        0x10298930 => { vm.decode(0x20020200u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x10298934u64) }
        0x10298934 => { vm.decode(0x27d32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -29i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x10298938u64).wrapping_add((imm * 4) as u64) } else { 0x10298938u64 + 4 });
        vm.wr(35, 2);
        Ok(0x10298938u64) }
        0x10298938 => { vm.decode(0x8209875u32); let d = 17i16; let s = 0i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029893cu64) }
        0x1029893c => { vm.decode(0x4a8aebafu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 920i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298940u64) }
        0x10298940 => { vm.decode(0x886c3afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 928i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298944u64) }
        0x10298944 => { vm.decode(0x628ce3efu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 888i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298948u64) }
        0x10298948 => { vm.decode(0x4880c3afu32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 896i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029894cu64) }
        0x1029894c => { vm.decode(0x8002036u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10298950u64) }
        0x10298950 => { vm.decode(0x631d1022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10298954u64) }
        0x10298954 => { vm.decode(0x2a82c3afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 912i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298958u64) }
        0x10298958 => { vm.decode(0x228453afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 1584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029895cu64) }
        0x1029895c => { vm.decode(0x8853efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 1600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298960u64) }
        0x10298960 => { vm.decode(0x650d1059u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 776i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x10298964u64) }
        0x10298964 => { vm.decode(0x22887bafu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 1560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298968u64) }
        0x10298968 => { vm.decode(0x88ee3afu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 936i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029896cu64) }
        0x1029896c => { vm.decode(0x867bafu32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 1576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298970u64) }
        0x10298970 => { vm.decode(0x428cc3efu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 880i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298974u64) }
        0x10298974 => { vm.decode(0x22825bafu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 1552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298978u64) }
        0x10298978 => { vm.decode(0x8e5bafu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 1568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029897cu64) }
        0x1029897c => { vm.decode(0x42847bafu32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 1592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298980u64) }
        0x10298980 => { vm.decode(0x2880ebafu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 904i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298984u64) }
        0x10298984 => { vm.decode(0x60807bafu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 1544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298988u64) }
        0x10298988 => { vm.decode(0x701a2835u32); let d = 3i16; let s = 9i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029898cu64) }
        0x1029898c => { vm.decode(0x1b109530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298990u64) }
        0x10298990 => { vm.decode(0x1a3835u32); let d = 3i16; let s = 16i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298994u64) }
        0x10298994 => { vm.decode(0x1b009530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298998u64) }
        0x10298998 => { vm.decode(0x404f3835u32); let d = 3i16; let s = 14i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029899cu64) }
        0x1029899c => { vm.decode(0x1b089530u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102989a0u64) }
        0x102989a0 => { vm.decode(0x23073835u32); let d = 13i16; let s = 13i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102989a4u64) }
        0x102989a4 => { vm.decode(0x4a80d875u32); let d = 31i16; let s = 0i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102989a8u64) }
        0x102989a8 => { vm.decode(0x407a835u32); let d = 17i16; let s = 6i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102989acu64) }
        0x102989ac => { vm.decode(0x48804875u32); let d = 22i16; let s = 0i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102989b0u64) }
        0x102989b0 => { vm.decode(0x23428835u32); let d = 12i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102989b4u64) }
        0x102989b4 => { vm.decode(0xda9119u32); let d = 3i16; let s = 10i16; let s2 = -1i16; let imm = 240i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102989b8u64) }
        0x102989b8 => { vm.decode(0x1b0815b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102989bcu64) }
        0x102989bc => { vm.decode(0x114c9835u32); let d = 6i16; let s = 2i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102989c0u64) }
        0x102989c0 => { vm.decode(0x4680c875u32); let d = 15i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102989c4u64) }
        0x102989c4 => { vm.decode(0x6208a835u32); let d = 11i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102989c8u64) }
        0x102989c8 => { vm.decode(0xd94c211u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102989ccu64) }
        0x102989cc => { vm.decode(0x30102200u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 11i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102989d0u64) }
        0x102989d0 => { vm.decode(0x460480a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 929i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102989d4u64).wrapping_add((imm * 4) as u64) } else { 0x102989d4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102989d4u64) }
        0x102989d4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102989d8u64) } }
        0x102989d8 => { vm.decode(0x32a200f5u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102989dcu64) }
        0x102989dc => { vm.decode(0x148080f5u32); let d = 2i16; let s = 16i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102989e0u64) }
        0x102989e0 => { vm.decode(0x769000f5u32); let d = 10i16; let s = 12i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102989e4u64) }
        0x102989e4 => { vm.decode(0x1b3815f0u32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102989e8u64) }
        0x102989e8 => { vm.decode(0x36b300f5u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102989ecu64) }
        0x102989ec => { vm.decode(0xa0c875u32); let d = 5i16; let s = 0i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102989f0u64) }
        0x102989f0 => { vm.decode(0xb1054f0u32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102989f4u64) }
        0x102989f4 => { vm.decode(0x72b1c0f5u32); let d = 12i16; let s = 31i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x102989f8u64) }
        0x102989f8 => { vm.decode(0x9b385570u32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102989fcu64) }
        0x102989fc => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298a00u64) }
        0x10298a00 => { vm.decode(0x6204875u32); let d = 8i16; let s = 0i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298a04u64) }
        0x10298a04 => { vm.decode(0x70b360f5u32); let d = 13i16; let s = 15i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298a08u64) }
        0x10298a08 => { vm.decode(0xceccb26du32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298a0cu64) }
        0x10298a0c => { vm.decode(0x1b3014f0u32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298a10u64) }
        0x10298a10 => { vm.decode(0x2a844b6fu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 240i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298a14u64) }
        0x10298a14 => { vm.decode(0xa0e294f5u32); let d = 13i16; let s = 9i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298a18u64) }
        0x10298a18 => { vm.decode(0xa86636fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 248i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298a1cu64) }
        0x10298a1c => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298a20u64) }
        0x10298a20 => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298a24u64) }
        0x10298a24 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298a28u64) }
        0x10298a28 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298a2cu64) }
        0x10298a2c => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298a30u64) }
        0x10298a30 => { vm.decode(0x2088c32fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 256i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298a34u64) }
        0x10298a34 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298a38u64) }
        0x10298a38 => { vm.decode(0x1491e0f5u32); let d = 14i16; let s = 18i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298a3cu64) }
        0x10298a3c => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298a40u64) }
        0x10298a40 => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298a44u64) }
        0x10298a44 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298a48u64) }
        0x10298a48 => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298a4cu64) }
        0x10298a4c => { vm.decode(0xa234f5u32); let d = 12i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298a50u64) }
        0x10298a50 => { vm.decode(0x52d100f5u32); let d = 24i16; let s = 6i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298a54u64) }
        0x10298a54 => { vm.decode(0x70f3a0f5u32); let d = 25i16; let s = 31i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298a58u64) }
        0x10298a58 => { vm.decode(0x40908875u32); let d = 7i16; let s = 0i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298a5cu64) }
        0x10298a5c => { vm.decode(0x52c180f5u32); let d = 16i16; let s = 22i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298a60u64) }
        0x10298a60 => { vm.decode(0x9b201430u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298a64u64) }
        0x10298a64 => { vm.decode(0x10e2a0f5u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298a68u64) }
        0x10298a68 => { vm.decode(0xc6e8a2adu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298a6cu64) }
        0x10298a6c => { vm.decode(0xa14484f5u32); let d = 17i16; let s = 9i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298a70u64) }
        0x10298a70 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298a74u64) }
        0x10298a74 => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298a78u64) }
        0x10298a78 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298a7cu64) }
        0x10298a7c => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298a80u64) }
        0x10298a80 => { vm.decode(0x4a909875u32); let d = 31i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298a84u64) }
        0x10298a84 => { vm.decode(0x54e120f5u32); let d = 18i16; let s = 7i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298a88u64) }
        0x10298a88 => { vm.decode(0x1b305530u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298a8cu64) }
        0x10298a8c => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298a90u64) }
        0x10298a90 => { vm.decode(0xce6832adu32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298a94u64) }
        0x10298a94 => { vm.decode(0x60032835u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298a98u64) }
        0x10298a98 => { vm.decode(0x60e294f5u32); let d = 13i16; let s = 9i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298a9cu64) }
        0x10298a9c => { vm.decode(0x16b1c0f5u32); let d = 14i16; let s = 19i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298aa0u64) }
        0x10298aa0 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298aa4u64) }
        0x10298aa4 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298aa8u64) }
        0x10298aa8 => { vm.decode(0x6000412fu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298aacu64) }
        0x10298aac => { vm.decode(0x10424f5u32); let d = 16i16; let s = 4i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298ab0u64) }
        0x10298ab0 => { vm.decode(0x18434f5u32); let d = 24i16; let s = 4i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298ab4u64) }
        0x10298ab4 => { vm.decode(0x4c909875u32); let d = 23i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298ab8u64) }
        0x10298ab8 => { vm.decode(0xa94494f5u32); let d = 19i16; let s = 9i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298abcu64) }
        0x10298abc => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298ac0u64) }
        0x10298ac0 => { vm.decode(0xc64c72edu32); let d = 20i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298ac4u64) }
        0x10298ac4 => { vm.decode(0xc67862adu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298ac8u64) }
        0x10298ac8 => { vm.decode(0x76b140f5u32); let d = 14i16; let s = 15i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298accu64) }
        0x10298acc => { vm.decode(0xc6cc72edu32); let d = 21i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298ad0u64) }
        0x10298ad0 => { vm.decode(0x20042835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298ad4u64) }
        0x10298ad4 => { vm.decode(0x19289cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298ad8u64) }
        0x10298ad8 => { vm.decode(0x728380f5u32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298adcu64) }
        0x10298adc => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298ae0u64) }
        0x10298ae0 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298ae4u64) }
        0x10298ae4 => { vm.decode(0x330b8835u32); let d = 14i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298ae8u64) }
        0x10298ae8 => { vm.decode(0x7481a0f5u32); let d = 2i16; let s = 30i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298aecu64) }
        0x10298aec => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298af0u64) }
        0x10298af0 => { vm.decode(0x22492fu32); let d = 12i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298af4u64) }
        0x10298af4 => { vm.decode(0x81e224f5u32); let d = 13i16; let s = 4i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298af8u64) }
        0x10298af8 => { vm.decode(0x690484f5u32); let d = 18i16; let s = 9i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298afcu64) }
        0x10298afc => { vm.decode(0x96424f5u32); let d = 19i16; let s = 4i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298b00u64) }
        0x10298b00 => { vm.decode(0x4002422fu32); let d = 20i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298b04u64) }
        0x10298b04 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298b08u64) }
        0x10298b08 => { vm.decode(0x680084f5u32); let d = 2i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298b0cu64) }
        0x10298b0c => { vm.decode(0x321c0835u32); let d = 10i16; let s = 10i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298b10u64) }
        0x10298b10 => { vm.decode(0x9309d70u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298b14u64) }
        0x10298b14 => { vm.decode(0x549120f5u32); let d = 10i16; let s = 6i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298b18u64) }
        0x10298b18 => { vm.decode(0xb285530u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298b1cu64) }
        0x10298b1c => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298b20u64) }
        0x10298b20 => { vm.decode(0x688094f5u32); let d = 10i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298b24u64) }
        0x10298b24 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298b28u64) }
        0x10298b28 => { vm.decode(0x524e0835u32); let d = 10i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298b2cu64) }
        0x10298b2c => { vm.decode(0xb081570u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298b30u64) }
        0x10298b30 => { vm.decode(0x28492fu32); let d = 10i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298b34u64) }
        0x10298b34 => { vm.decode(0x2028482fu32); let d = 11i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298b38u64) }
        0x10298b38 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298b3cu64) }
        0x10298b3c => { vm.decode(0x20820b2fu32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298b40u64) }
        0x10298b40 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298b44u64) }
        0x10298b44 => { vm.decode(0x4031835u32); let d = 16i16; let s = 12i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298b48u64) }
        0x10298b48 => { vm.decode(0x50b060f5u32); let d = 12i16; let s = 5i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298b4cu64) }
        0x10298b4c => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298b50u64) }
        0x10298b50 => { vm.decode(0x20a294f5u32); let d = 12i16; let s = 9i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298b54u64) }
        0x10298b54 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298b58u64) }
        0x10298b58 => { vm.decode(0x103b835u32); let d = 5i16; let s = 12i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298b5cu64) }
        0x10298b5c => { vm.decode(0x1938dcb0u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298b60u64) }
        0x10298b60 => { vm.decode(0xa08875u32); let d = 5i16; let s = 0i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298b64u64) }
        0x10298b64 => { vm.decode(0x32c200f5u32); let d = 17i16; let s = 8i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298b68u64) }
        0x10298b68 => { vm.decode(0xc6e8a2adu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298b6cu64) }
        0x10298b6c => { vm.decode(0x814424f5u32); let d = 17i16; let s = 4i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298b70u64) }
        0x10298b70 => { vm.decode(0x4008412fu32); let d = 18i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298b74u64) }
        0x10298b74 => { vm.decode(0x53408835u32); let d = 12i16; let s = 2i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298b78u64) }
        0x10298b78 => { vm.decode(0xa105875u32); let d = 24i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298b7cu64) }
        0x10298b7c => { vm.decode(0x8b1855b0u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298b80u64) }
        0x10298b80 => { vm.decode(0x5481a0f5u32); let d = 2i16; let s = 22i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298b84u64) }
        0x10298b84 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298b88u64) }
        0x10298b88 => { vm.decode(0x680084f5u32); let d = 2i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298b8cu64) }
        0x10298b8c => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298b90u64) }
        0x10298b90 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298b94u64) }
        0x10298b94 => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298b98u64) }
        0x10298b98 => { vm.decode(0x30462835u32); let d = 1i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298b9cu64) }
        0x10298b9c => { vm.decode(0x19209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298ba0u64) }
        0x10298ba0 => { vm.decode(0x28826bafu32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298ba4u64) }
        0x10298ba4 => { vm.decode(0x70a220f5u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298ba8u64) }
        0x10298ba8 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298bacu64) }
        0x10298bac => { vm.decode(0xa04084f5u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298bb0u64) }
        0x10298bb0 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298bb4u64) }
        0x10298bb4 => { vm.decode(0x20043835u32); let d = 1i16; let s = 1i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298bb8u64) }
        0x10298bb8 => { vm.decode(0x1b181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298bbcu64) }
        0x10298bbc => { vm.decode(0x56b100f5u32); let d = 10i16; let s = 7i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298bc0u64) }
        0x10298bc0 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298bc4u64) }
        0x10298bc4 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298bc8u64) }
        0x10298bc8 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298bccu64) }
        0x10298bcc => { vm.decode(0x2844befu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298bd0u64) }
        0x10298bd0 => { vm.decode(0x349220f5u32); let d = 11i16; let s = 8i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298bd4u64) }
        0x10298bd4 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298bd8u64) }
        0x10298bd8 => { vm.decode(0xe8c094f5u32); let d = 11i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298bdcu64) }
        0x10298bdc => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298be0u64) }
        0x10298be0 => { vm.decode(0x35462835u32); let d = 21i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298be4u64) }
        0x10298be4 => { vm.decode(0x76b180f5u32); let d = 10i16; let s = 31i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298be8u64) }
        0x10298be8 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298becu64) }
        0x10298bec => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298bf0u64) }
        0x10298bf0 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298bf4u64) }
        0x10298bf4 => { vm.decode(0x54b3a0f5u32); let d = 11i16; let s = 23i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298bf8u64) }
        0x10298bf8 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298bfcu64) }
        0x10298bfc => { vm.decode(0xe8c094f5u32); let d = 11i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298c00u64) }
        0x10298c00 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298c04u64) }
        0x10298c04 => { vm.decode(0x30462835u32); let d = 1i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298c08u64) }
        0x10298c08 => { vm.decode(0x1b001430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298c0cu64) }
        0x10298c0c => { vm.decode(0x76b000f5u32); let d = 10i16; let s = 13i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298c10u64) }
        0x10298c10 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298c14u64) }
        0x10298c14 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298c18u64) }
        0x10298c18 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298c1cu64) }
        0x10298c1c => { vm.decode(0x688843afu32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298c20u64) }
        0x10298c20 => { vm.decode(0x14b3a0f5u32); let d = 11i16; let s = 19i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298c24u64) }
        0x10298c24 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298c28u64) }
        0x10298c28 => { vm.decode(0xe8c094f5u32); let d = 11i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298c2cu64) }
        0x10298c2c => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298c30u64) }
        0x10298c30 => { vm.decode(0x33462835u32); let d = 13i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298c34u64) }
        0x10298c34 => { vm.decode(0x208043efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298c38u64) }
        0x10298c38 => { vm.decode(0x16b000f5u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298c3cu64) }
        0x10298c3c => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298c40u64) }
        0x10298c40 => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298c44u64) }
        0x10298c44 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298c48u64) }
        0x10298c48 => { vm.decode(0x54b220f5u32); let d = 11i16; let s = 5i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298c4cu64) }
        0x10298c4c => { vm.decode(0x4c004875u32); let d = 18i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298c50u64) }
        0x10298c50 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298c54u64) }
        0x10298c54 => { vm.decode(0xe8c094f5u32); let d = 11i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298c58u64) }
        0x10298c58 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298c5cu64) }
        0x10298c5c => { vm.decode(0x354e0835u32); let d = 22i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298c60u64) }
        0x10298c60 => { vm.decode(0x4a8243afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298c64u64) }
        0x10298c64 => { vm.decode(0x569080f5u32); let d = 10i16; let s = 20i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298c68u64) }
        0x10298c68 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298c6cu64) }
        0x10298c6c => { vm.decode(0x488034f5u32); let d = 10i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298c70u64) }
        0x10298c70 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298c74u64) }
        0x10298c74 => { vm.decode(0x228e63efu32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298c78u64) }
        0x10298c78 => { vm.decode(0x54b320f5u32); let d = 11i16; let s = 7i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298c7cu64) }
        0x10298c7c => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298c80u64) }
        0x10298c80 => { vm.decode(0xe8c094f5u32); let d = 11i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298c84u64) }
        0x10298c84 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298c88u64) }
        0x10298c88 => { vm.decode(0x36462835u32); let d = 25i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298c8cu64) }
        0x10298c8c => { vm.decode(0x349080f5u32); let d = 10i16; let s = 24i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298c90u64) }
        0x10298c90 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298c94u64) }
        0x10298c94 => { vm.decode(0x6a8014f5u32); let d = 10i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298c98u64) }
        0x10298c98 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298c9cu64) }
        0x10298c9c => { vm.decode(0x11472835u32); let d = 5i16; let s = 10i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298ca0u64) }
        0x10298ca0 => { vm.decode(0x1928dc30u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298ca4u64) }
        0x10298ca4 => { vm.decode(0xd31c53b5u32); let d = 10i16; let s = 23i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298ca8u64) }
        0x10298ca8 => { vm.decode(0x9b2055b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298cacu64) }
        0x10298cac => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298cb0u64) }
        0x10298cb0 => { vm.decode(0x2082e32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 264i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298cb4u64) }
        0x10298cb4 => { vm.decode(0x688034f5u32); let d = 10i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298cb8u64) }
        0x10298cb8 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298cbcu64) }
        0x10298cbc => { vm.decode(0x4210c875u32); let d = 11i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298cc0u64) }
        0x10298cc0 => { vm.decode(0x88eeb6fu32); let d = 14i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298cc4u64) }
        0x10298cc4 => { vm.decode(0x134f0835u32); let d = 14i16; let s = 10i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298cc8u64) }
        0x10298cc8 => { vm.decode(0x16b180f5u32); let d = 10i16; let s = 19i16; let s2 = -1i16; let imm = 45i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298cccu64) }
        0x10298ccc => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298cd0u64) }
        0x10298cd0 => { vm.decode(0x408434f5u32); let d = 24i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298cd4u64) }
        0x10298cd4 => { vm.decode(0x549100f5u32); let d = 10i16; let s = 6i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298cd8u64) }
        0x10298cd8 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298cdcu64) }
        0x10298cdc => { vm.decode(0x6a8014f5u32); let d = 10i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298ce0u64) }
        0x10298ce0 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298ce4u64) }
        0x10298ce4 => { vm.decode(0xa82c36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298ce8u64) }
        0x10298ce8 => { vm.decode(0x14052835u32); let d = 17i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298cecu64) }
        0x10298cec => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298cf0u64) }
        0x10298cf0 => { vm.decode(0x8b10d430u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298cf4u64) }
        0x10298cf4 => { vm.decode(0x5490a0f5u32); let d = 10i16; let s = 20i16; let s2 = -1i16; let imm = 53i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298cf8u64) }
        0x10298cf8 => { vm.decode(0xce58722du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298cfcu64) }
        0x10298cfc => { vm.decode(0x688094f5u32); let d = 10i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298d00u64) }
        0x10298d00 => { vm.decode(0x230d875u32); let d = 9i16; let s = 0i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298d04u64) }
        0x10298d04 => { vm.decode(0x8938dd70u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298d08u64) }
        0x10298d08 => { vm.decode(0x749380f5u32); let d = 11i16; let s = 30i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298d0cu64) }
        0x10298d0c => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298d10u64) }
        0x10298d10 => { vm.decode(0xeac014f5u32); let d = 11i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298d14u64) }
        0x10298d14 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298d18u64) }
        0x10298d18 => { vm.decode(0x511c0835u32); let d = 6i16; let s = 11i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298d1cu64) }
        0x10298d1c => { vm.decode(0x928ddb0u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298d20u64) }
        0x10298d20 => { vm.decode(0x549280f5u32); let d = 11i16; let s = 20i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298d24u64) }
        0x10298d24 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298d28u64) }
        0x10298d28 => { vm.decode(0xeac014f5u32); let d = 11i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298d2cu64) }
        0x10298d2c => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298d30u64) }
        0x10298d30 => { vm.decode(0x8263afu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298d34u64) }
        0x10298d34 => { vm.decode(0x570d0835u32); let d = 30i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298d38u64) }
        0x10298d38 => { vm.decode(0x349200f5u32); let d = 11i16; let s = 8i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298d3cu64) }
        0x10298d3c => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298d40u64) }
        0x10298d40 => { vm.decode(0xeac014f5u32); let d = 11i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298d44u64) }
        0x10298d44 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298d48u64) }
        0x10298d48 => { vm.decode(0x51070835u32); let d = 4i16; let s = 11i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298d4cu64) }
        0x10298d4c => { vm.decode(0x930dc30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298d50u64) }
        0x10298d50 => { vm.decode(0x44805875u32); let d = 6i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298d54u64) }
        0x10298d54 => { vm.decode(0x54b300f5u32); let d = 11i16; let s = 7i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298d58u64) }
        0x10298d58 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298d5cu64) }
        0x10298d5c => { vm.decode(0xeac014f5u32); let d = 11i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298d60u64) }
        0x10298d60 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298d64u64) }
        0x10298d64 => { vm.decode(0xa86e36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298d68u64) }
        0x10298d68 => { vm.decode(0x53050835u32); let d = 12i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298d6cu64) }
        0x10298d6c => { vm.decode(0x6305875u32); let d = 8i16; let s = 0i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298d70u64) }
        0x10298d70 => { vm.decode(0x74b380f5u32); let d = 11i16; let s = 31i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298d74u64) }
        0x10298d74 => { vm.decode(0xced8f22du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298d78u64) }
        0x10298d78 => { vm.decode(0xeac014f5u32); let d = 11i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298d7cu64) }
        0x10298d7c => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298d80u64) }
        0x10298d80 => { vm.decode(0x88cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298d84u64) }
        0x10298d84 => { vm.decode(0x504c0835u32); let d = 2i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298d88u64) }
        0x10298d88 => { vm.decode(0x9209d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298d8cu64) }
        0x10298d8c => { vm.decode(0x74a100f5u32); let d = 2i16; let s = 15i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298d90u64) }
        0x10298d90 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298d94u64) }
        0x10298d94 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298d98u64) }
        0x10298d98 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298d9cu64) }
        0x10298d9c => { vm.decode(0x628a6befu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298da0u64) }
        0x10298da0 => { vm.decode(0xa86c36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298da4u64) }
        0x10298da4 => { vm.decode(0xd0835u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298da8u64) }
        0x10298da8 => { vm.decode(0x9109db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298dacu64) }
        0x10298dac => { vm.decode(0x54a180f5u32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298db0u64) }
        0x10298db0 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298db4u64) }
        0x10298db4 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298db8u64) }
        0x10298db8 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298dbcu64) }
        0x10298dbc => { vm.decode(0x201d0835u32); let d = 2i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298dc0u64) }
        0x10298dc0 => { vm.decode(0x9189db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298dc4u64) }
        0x10298dc4 => { vm.decode(0x44808875u32); let d = 7i16; let s = 0i16; let s2 = 1i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298dc8u64) }
        0x10298dc8 => { vm.decode(0x14a000f5u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298dccu64) }
        0x10298dcc => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298dd0u64) }
        0x10298dd0 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298dd4u64) }
        0x10298dd4 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298dd8u64) }
        0x10298dd8 => { vm.decode(0x228063afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298ddcu64) }
        0x10298ddc => { vm.decode(0x25040835u32); let d = 20i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298de0u64) }
        0x10298de0 => { vm.decode(0x228c4bafu32); let d = 11i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298de4u64) }
        0x10298de4 => { vm.decode(0x34a100f5u32); let d = 2i16; let s = 11i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298de8u64) }
        0x10298de8 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298decu64) }
        0x10298dec => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298df0u64) }
        0x10298df0 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298df4u64) }
        0x10298df4 => { vm.decode(0x552835u32); let d = 1i16; let s = 2i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298df8u64) }
        0x10298df8 => { vm.decode(0x19389cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298dfcu64) }
        0x10298dfc => { vm.decode(0x148180f5u32); let d = 2i16; let s = 18i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298e00u64) }
        0x10298e00 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298e04u64) }
        0x10298e04 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298e08u64) }
        0x10298e08 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298e0cu64) }
        0x10298e0c => { vm.decode(0x250f0835u32); let d = 22i16; let s = 2i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298e10u64) }
        0x10298e10 => { vm.decode(0x74a180f5u32); let d = 2i16; let s = 31i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298e14u64) }
        0x10298e14 => { vm.decode(0xc658622du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298e18u64) }
        0x10298e18 => { vm.decode(0x6a0004f5u32); let d = 2i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298e1cu64) }
        0x10298e1c => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298e20u64) }
        0x10298e20 => { vm.decode(0x208043afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298e24u64) }
        0x10298e24 => { vm.decode(0x25042835u32); let d = 21i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298e28u64) }
        0x10298e28 => { vm.decode(0x6909875u32); let d = 13i16; let s = 0i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298e2cu64) }
        0x10298e2c => { vm.decode(0x10e180f5u32); let d = 16i16; let s = 19i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298e30u64) }
        0x10298e30 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298e34u64) }
        0x10298e34 => { vm.decode(0x230404f5u32); let d = 16i16; let s = 3i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298e38u64) }
        0x10298e38 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298e3cu64) }
        0x10298e3c => { vm.decode(0x20123835u32); let d = 1i16; let s = 16i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298e40u64) }
        0x10298e40 => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298e44u64) }
        0x10298e44 => { vm.decode(0x22bb5u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10298e48u64) }
        0x10298e48 => { vm.decode(0x31d3022u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10298e4cu64) }
        0x10298e4c => { vm.decode(0xb0014f0u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298e50u64) }
        0x10298e50 => { vm.decode(0x44820d35u32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10298e54u64) }
        0x10298e54 => { vm.decode(0x89614909u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10298e58u64) }
        0x10298e58 => { vm.decode(0xe0004d75u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10298e5cu64) }
        0x10298e5c => { vm.decode(0x282c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 272i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298e60u64) }
        0x10298e60 => { vm.decode(0x10424f5u32); let d = 16i16; let s = 4i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298e64u64) }
        0x10298e64 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298e68u64) }
        0x10298e68 => { vm.decode(0x54b3835u32); let d = 23i16; let s = 14i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298e6cu64) }
        0x10298e6c => { vm.decode(0xa21c23b5u32); let d = 16i16; let s = 9i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298e70u64) }
        0x10298e70 => { vm.decode(0xc66822adu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298e74u64) }
        0x10298e74 => { vm.decode(0x210424f5u32); let d = 16i16; let s = 5i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298e78u64) }
        0x10298e78 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298e7cu64) }
        0x10298e7c => { vm.decode(0x2880e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298e80u64) }
        0x10298e80 => { vm.decode(0x20003835u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298e84u64) }
        0x10298e84 => { vm.decode(0x19209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298e88u64) }
        0x10298e88 => { vm.decode(0x60004a2fu32); let d = 25i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298e8cu64) }
        0x10298e8c => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298e90u64) }
        0x10298e90 => { vm.decode(0x50423835u32); let d = 1i16; let s = 10i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298e94u64) }
        0x10298e94 => { vm.decode(0x1b081430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298e98u64) }
        0x10298e98 => { vm.decode(0x39c73b5u32); let d = 25i16; let s = 6i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298e9cu64) }
        0x10298e9c => { vm.decode(0xcee8b2adu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298ea0u64) }
        0x10298ea0 => { vm.decode(0xa1c434f5u32); let d = 25i16; let s = 5i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298ea4u64) }
        0x10298ea4 => { vm.decode(0x60004b2fu32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298ea8u64) }
        0x10298ea8 => { vm.decode(0x74183835u32); let d = 19i16; let s = 17i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298eacu64) }
        0x10298eac => { vm.decode(0x88a63afu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298eb0u64) }
        0x10298eb0 => { vm.decode(0x50d100f5u32); let d = 24i16; let s = 6i16; let s2 = -1i16; let imm = 37i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298eb4u64) }
        0x10298eb4 => { vm.decode(0xce6832adu32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298eb8u64) }
        0x10298eb8 => { vm.decode(0x230414f5u32); let d = 16i16; let s = 3i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298ebcu64) }
        0x10298ebc => { vm.decode(0xa21c73b5u32); let d = 24i16; let s = 13i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298ec0u64) }
        0x10298ec0 => { vm.decode(0xe20c875u32); let d = 25i16; let s = 0i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298ec4u64) }
        0x10298ec4 => { vm.decode(0x1b00d470u32); let d = 13i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298ec8u64) }
        0x10298ec8 => { vm.decode(0xce6832adu32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298eccu64) }
        0x10298ecc => { vm.decode(0x218434f5u32); let d = 24i16; let s = 5i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298ed0u64) }
        0x10298ed0 => { vm.decode(0x40004a2fu32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298ed4u64) }
        0x10298ed4 => { vm.decode(0x175b1835u32); let d = 30i16; let s = 24i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298ed8u64) }
        0x10298ed8 => { vm.decode(0xa31c73b5u32); let d = 24i16; let s = 15i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298edcu64) }
        0x10298edc => { vm.decode(0x1b105570u32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298ee0u64) }
        0x10298ee0 => { vm.decode(0xce6832adu32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298ee4u64) }
        0x10298ee4 => { vm.decode(0x218434f5u32); let d = 24i16; let s = 5i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298ee8u64) }
        0x10298ee8 => { vm.decode(0x40004a2fu32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298eecu64) }
        0x10298eec => { vm.decode(0x2884e36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298ef0u64) }
        0x10298ef0 => { vm.decode(0x36001835u32); let d = 24i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298ef4u64) }
        0x10298ef4 => { vm.decode(0xa8843afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298ef8u64) }
        0x10298ef8 => { vm.decode(0x135c13b5u32); let d = 14i16; let s = 2i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298efcu64) }
        0x10298efc => { vm.decode(0xce5c726du32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298f00u64) }
        0x10298f00 => { vm.decode(0x68a234f5u32); let d = 14i16; let s = 5i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298f04u64) }
        0x10298f04 => { vm.decode(0xaa0a2fu32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298f08u64) }
        0x10298f08 => { vm.decode(0x1007a835u32); let d = 1i16; let s = 12i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298f0cu64) }
        0x10298f0c => { vm.decode(0x19089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298f10u64) }
        0x10298f10 => { vm.decode(0x2884befu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298f14u64) }
        0x10298f14 => { vm.decode(0x235c13b5u32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298f18u64) }
        0x10298f18 => { vm.decode(0xce4c326du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298f1cu64) }
        0x10298f1c => { vm.decode(0x20a234f5u32); let d = 12i16; let s = 5i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298f20u64) }
        0x10298f20 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298f24u64) }
        0x10298f24 => { vm.decode(0x2a80c36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298f28u64) }
        0x10298f28 => { vm.decode(0x33008835u32); let d = 12i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298f2cu64) }
        0x10298f2c => { vm.decode(0x22844befu32); let d = 9i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298f30u64) }
        0x10298f30 => { vm.decode(0xa29c23b5u32); let d = 17i16; let s = 9i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298f34u64) }
        0x10298f34 => { vm.decode(0xc6e8a2adu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298f38u64) }
        0x10298f38 => { vm.decode(0xa14424f5u32); let d = 17i16; let s = 5i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298f3cu64) }
        0x10298f3c => { vm.decode(0x6000412fu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298f40u64) }
        0x10298f40 => { vm.decode(0x2a84c32fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298f44u64) }
        0x10298f44 => { vm.decode(0x60003835u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298f48u64) }
        0x10298f48 => { vm.decode(0x19009cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298f4cu64) }
        0x10298f4c => { vm.decode(0xe39c63b5u32); let d = 17i16; let s = 31i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298f50u64) }
        0x10298f50 => { vm.decode(0xc6e8a2adu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298f54u64) }
        0x10298f54 => { vm.decode(0xa14424f5u32); let d = 17i16; let s = 5i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298f58u64) }
        0x10298f58 => { vm.decode(0x6000412fu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298f5cu64) }
        0x10298f5c => { vm.decode(0x2880c36fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298f60u64) }
        0x10298f60 => { vm.decode(0x60003835u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298f64u64) }
        0x10298f64 => { vm.decode(0x19289cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298f68u64) }
        0x10298f68 => { vm.decode(0x6a09875u32); let d = 13i16; let s = 0i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298f6cu64) }
        0x10298f6c => { vm.decode(0xa39c23b5u32); let d = 17i16; let s = 11i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298f70u64) }
        0x10298f70 => { vm.decode(0xc6e8a2adu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298f74u64) }
        0x10298f74 => { vm.decode(0xa14424f5u32); let d = 17i16; let s = 5i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298f78u64) }
        0x10298f78 => { vm.decode(0x6000412fu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298f7cu64) }
        0x10298f7c => { vm.decode(0x40113835u32); let d = 1i16; let s = 17i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298f80u64) }
        0x10298f80 => { vm.decode(0x19381cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298f84u64) }
        0x10298f84 => { vm.decode(0x229c03b5u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298f88u64) }
        0x10298f88 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298f8cu64) }
        0x10298f8c => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298f90u64) }
        0x10298f90 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298f94u64) }
        0x10298f94 => { vm.decode(0x2a8ce32fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298f98u64) }
        0x10298f98 => { vm.decode(0x60402835u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298f9cu64) }
        0x10298f9c => { vm.decode(0x19109cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298fa0u64) }
        0x10298fa0 => { vm.decode(0x428663afu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298fa4u64) }
        0x10298fa4 => { vm.decode(0x429c43b5u32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298fa8u64) }
        0x10298fa8 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298facu64) }
        0x10298fac => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298fb0u64) }
        0x10298fb0 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298fb4u64) }
        0x10298fb4 => { vm.decode(0x64112835u32); let d = 17i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298fb8u64) }
        0x10298fb8 => { vm.decode(0x839c43b5u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298fbcu64) }
        0x10298fbc => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298fc0u64) }
        0x10298fc0 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298fc4u64) }
        0x10298fc4 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298fc8u64) }
        0x10298fc8 => { vm.decode(0x40512835u32); let d = 1i16; let s = 1i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298fccu64) }
        0x10298fcc => { vm.decode(0x19189c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298fd0u64) }
        0x10298fd0 => { vm.decode(0x806befu32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298fd4u64) }
        0x10298fd4 => { vm.decode(0x229c03b5u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10298fd8u64) }
        0x10298fd8 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10298fdcu64) }
        0x10298fdc => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10298fe0u64) }
        0x10298fe0 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298fe4u64) }
        0x10298fe4 => { vm.decode(0x2a8ce36fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10298fe8u64) }
        0x10298fe8 => { vm.decode(0x60402835u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298fecu64) }
        0x10298fec => { vm.decode(0x19189cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10298ff0u64) }
        0x10298ff0 => { vm.decode(0x40e32bb5u32); let d = 3i16; let s = 31i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10298ff4u64) }
        0x10298ff4 => { vm.decode(0xdd4349u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10298ff8u64) }
        0x10298ff8 => { vm.decode(0x4e809875u32); let d = 31i16; let s = 0i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10298ffcu64) }
        0x10298ffc => { vm.decode(0x30d5022u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10299000u64) }
        0x10299000 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10299004u64) }
        0x10299004 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299008u64) }
        0x10299008 => { vm.decode(0x2288e32fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 280i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029900cu64) }
        0x1029900c => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299010u64) }
        0x10299010 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299014u64) }
        0x10299014 => { vm.decode(0x60512835u32); let d = 1i16; let s = 1i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299018u64) }
        0x10299018 => { vm.decode(0x19109c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029901cu64) }
        0x1029901c => { vm.decode(0x4080d875u32); let d = 7i16; let s = 0i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299020u64) }
        0x10299020 => { vm.decode(0x39c43b5u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299024u64) }
        0x10299024 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299028u64) }
        0x10299028 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029902cu64) }
        0x1029902c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299030u64) }
        0x10299030 => { vm.decode(0xa8ec36fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299034u64) }
        0x10299034 => { vm.decode(0x40412835u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299038u64) }
        0x10299038 => { vm.decode(0x19209c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029903cu64) }
        0x1029903c => { vm.decode(0x439c03b5u32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 29i64;
        let v = vm.ld(s).wrapping_shr(imm as u32);
        vm.st(d, v);
        Ok(0x10299040u64) }
        0x10299040 => { vm.decode(0x42901875u32); let d = 14i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299044u64) }
        0x10299044 => { vm.decode(0x89281d30u32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299048u64) }
        0x10299048 => { vm.decode(0xc6c8a22du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2040i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029904cu64) }
        0x1029904c => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299050u64) }
        0x10299050 => { vm.decode(0x19089cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299054u64) }
        0x10299054 => { vm.decode(0x80e52bb5u32); let d = 22i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10299058u64) }
        0x10299058 => { vm.decode(0x60522d35u32); let d = 1i16; let s = 22i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029905cu64) }
        0x1029905c => { vm.decode(0x42755162u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10299060u64) }
        0x10299060 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10299064u64) }
        0x10299064 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299068u64) }
        0x10299068 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029906cu64) }
        0x1029906c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299070u64) }
        0x10299070 => { vm.decode(0x2886c36fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299074u64) }
        0x10299074 => { vm.decode(0x60012835u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299078u64) }
        0x10299078 => { vm.decode(0x19309c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029907cu64) }
        0x1029907c => { vm.decode(0x30bb5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10299080u64) }
        0x10299080 => { vm.decode(0x63253062u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10299084u64) }
        0x10299084 => { vm.decode(0x40920d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10299088u64) }
        0x10299088 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029908cu64) }
        0x1029908c => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299090u64) }
        0x10299090 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299094u64) }
        0x10299094 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299098u64) }
        0x10299098 => { vm.decode(0x65582835u32); let d = 23i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029909cu64) }
        0x1029909c => { vm.decode(0x2000402fu32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102990a0u64) }
        0x102990a0 => { vm.decode(0x8863afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102990a4u64) }
        0x102990a4 => { vm.decode(0x44480835u32); let d = 18i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102990a8u64) }
        0x102990a8 => { vm.decode(0x80a10bb5u32); let d = 16i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102990acu64) }
        0x102990ac => { vm.decode(0x40520d35u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102990b0u64) }
        0x102990b0 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102990b4u64) }
        0x102990b4 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102990b8u64) }
        0x102990b8 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102990bcu64) }
        0x102990bc => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102990c0u64) }
        0x102990c0 => { vm.decode(0x40532835u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102990c4u64) }
        0x102990c4 => { vm.decode(0x19209cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102990c8u64) }
        0x102990c8 => { vm.decode(0x81c42bb5u32); let d = 30i16; let s = 9i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102990ccu64) }
        0x102990cc => { vm.decode(0x60526d35u32); let d = 1i16; let s = 30i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102990d0u64) }
        0x102990d0 => { vm.decode(0x62777162u32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102990d4u64) }
        0x102990d4 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102990d8u64) }
        0x102990d8 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102990dcu64) }
        0x102990dc => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102990e0u64) }
        0x102990e0 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102990e4u64) }
        0x102990e4 => { vm.decode(0x40122835u32); let d = 1i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102990e8u64) }
        0x102990e8 => { vm.decode(0x19009c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102990ecu64) }
        0x102990ec => { vm.decode(0x40862bb5u32); let d = 5i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102990f0u64) }
        0x102990f0 => { vm.decode(0x40922d35u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102990f4u64) }
        0x102990f4 => { vm.decode(0x35d1022u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102990f8u64) }
        0x102990f8 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102990fcu64) }
        0x102990fc => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299100u64) }
        0x10299100 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299104u64) }
        0x10299104 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299108u64) }
        0x10299108 => { vm.decode(0x888e32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029910cu64) }
        0x1029910c => { vm.decode(0x44482835u32); let d = 19i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299110u64) }
        0x10299110 => { vm.decode(0x208443efu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299114u64) }
        0x10299114 => { vm.decode(0x1820bb5u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10299118u64) }
        0x10299118 => { vm.decode(0x40924d35u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029911cu64) }
        0x1029911c => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10299120u64) }
        0x10299120 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299124u64) }
        0x10299124 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299128u64) }
        0x10299128 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029912cu64) }
        0x1029912c => { vm.decode(0x40032835u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299130u64) }
        0x10299130 => { vm.decode(0x19089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299134u64) }
        0x10299134 => { vm.decode(0x81012bb5u32); let d = 24i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10299138u64) }
        0x10299138 => { vm.decode(0x40524d35u32); let d = 1i16; let s = 24i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x1029913cu64) }
        0x1029913c => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10299140u64) }
        0x10299140 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299144u64) }
        0x10299144 => { vm.decode(0x804024f5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299148u64) }
        0x10299148 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029914cu64) }
        0x1029914c => { vm.decode(0x88cc32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299150u64) }
        0x10299150 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299154u64) }
        0x10299154 => { vm.decode(0x19201cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299158u64) }
        0x10299158 => { vm.decode(0x40220bb5u32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029915cu64) }
        0x1029915c => { vm.decode(0x408a8d35u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10299160u64) }
        0x10299160 => { vm.decode(0xe82d4909u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10299164u64) }
        0x10299164 => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299168u64) }
        0x10299168 => { vm.decode(0xa234f5u32); let d = 12i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029916cu64) }
        0x1029916c => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299170u64) }
        0x10299170 => { vm.decode(0x88ce36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299174u64) }
        0x10299174 => { vm.decode(0x10488835u32); let d = 2i16; let s = 2i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299178u64) }
        0x10299178 => { vm.decode(0x9009db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029917cu64) }
        0x1029917c => { vm.decode(0x1002bb5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10299180u64) }
        0x10299180 => { vm.decode(0x401b4d35u32); let d = 11i16; let s = 8i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10299184u64) }
        0x10299184 => { vm.decode(0x62371062u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 488i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x10299188u64) }
        0x10299188 => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029918cu64) }
        0x1029918c => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299190u64) }
        0x10299190 => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299194u64) }
        0x10299194 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299198u64) }
        0x10299198 => { vm.decode(0x28ce36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029919cu64) }
        0x1029919c => { vm.decode(0x504c0835u32); let d = 2i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102991a0u64) }
        0x102991a0 => { vm.decode(0x9381d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102991a4u64) }
        0x102991a4 => { vm.decode(0x2a8aeb6fu32); let d = 15i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102991a8u64) }
        0x102991a8 => { vm.decode(0x80a72bb5u32); let d = 21i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102991acu64) }
        0x102991ac => { vm.decode(0x40db2d35u32); let d = 11i16; let s = 21i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102991b0u64) }
        0x102991b0 => { vm.decode(0x43453162u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102991b4u64) }
        0x102991b4 => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102991b8u64) }
        0x102991b8 => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102991bcu64) }
        0x102991bc => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102991c0u64) }
        0x102991c0 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102991c4u64) }
        0x102991c4 => { vm.decode(0x73140835u32); let d = 12i16; let s = 11i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102991c8u64) }
        0x102991c8 => { vm.decode(0x60c36d35u32); let d = 2i16; let s = 31i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102991ccu64) }
        0x102991cc => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102991d0u64) }
        0x102991d0 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102991d4u64) }
        0x102991d4 => { vm.decode(0x480024f5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102991d8u64) }
        0x102991d8 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102991dcu64) }
        0x102991dc => { vm.decode(0xa8ec32fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102991e0u64) }
        0x102991e0 => { vm.decode(0x4d0835u32); let d = 2i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102991e4u64) }
        0x102991e4 => { vm.decode(0x9301db0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102991e8u64) }
        0x102991e8 => { vm.decode(0xe02bb5u32); let d = 2i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102991ecu64) }
        0x102991ec => { vm.decode(0x62157062u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102991f0u64) }
        0x102991f0 => { vm.decode(0x60030d35u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102991f4u64) }
        0x102991f4 => { vm.decode(0x9a214909u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102991f8u64) }
        0x102991f8 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102991fcu64) }
        0x102991fc => { vm.decode(0x480024f5u32); let d = 2i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299200u64) }
        0x10299200 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299204u64) }
        0x10299204 => { vm.decode(0xa8ae32fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299208u64) }
        0x10299208 => { vm.decode(0x4d0835u32); let d = 2i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029920cu64) }
        0x1029920c => { vm.decode(0x28c6befu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299210u64) }
        0x10299210 => { vm.decode(0x472bb5u32); let d = 7i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x10299214u64) }
        0x10299214 => { vm.decode(0x609b2d35u32); let d = 11i16; let s = 7i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10299218u64) }
        0x10299218 => { vm.decode(0x3757062u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029921cu64) }
        0x1029921c => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10299220u64) }
        0x10299220 => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299224u64) }
        0x10299224 => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299228u64) }
        0x10299228 => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029922cu64) }
        0x1029922c => { vm.decode(0xa8ee32fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299230u64) }
        0x10299230 => { vm.decode(0x514d0835u32); let d = 6i16; let s = 6i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299234u64) }
        0x10299234 => { vm.decode(0x918ddb0u32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299238u64) }
        0x10299238 => { vm.decode(0x52d84349u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029923cu64) }
        0x1029923c => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10299240u64) }
        0x10299240 => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299244u64) }
        0x10299244 => { vm.decode(0x8ee32fu32); let d = 6i16; let s = 29i16; let s2 = -1i16; let imm = 296i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299248u64) }
        0x10299248 => { vm.decode(0xcac034f5u32); let d = 11i16; let s = 6i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029924cu64) }
        0x1029924c => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299250u64) }
        0x10299250 => { vm.decode(0xa88cb2fu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299254u64) }
        0x10299254 => { vm.decode(0x53462835u32); let d = 13i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299258u64) }
        0x10299258 => { vm.decode(0x80232bb5u32); let d = 17i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x1029925cu64) }
        0x1029925c => { vm.decode(0x40db0d35u32); let d = 11i16; let s = 17i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x10299260u64) }
        0x10299260 => { vm.decode(0xba2bc909u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10299264u64) }
        0x10299264 => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299268u64) }
        0x10299268 => { vm.decode(0xc8c034f5u32); let d = 11i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029926cu64) }
        0x1029926c => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299270u64) }
        0x10299270 => { vm.decode(0x888cb6fu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299274u64) }
        0x10299274 => { vm.decode(0x524e2835u32); let d = 11i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299278u64) }
        0x10299278 => { vm.decode(0x88ceb2fu32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029927cu64) }
        0x1029927c => { vm.decode(0x60200a2fu32); let d = 25i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299280u64) }
        0x10299280 => { vm.decode(0x52581835u32); let d = 10i16; let s = 18i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299284u64) }
        0x10299284 => { vm.decode(0x9209d70u32); let d = 10i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299288u64) }
        0x10299288 => { vm.decode(0x507c4349u32); let d = 10i16; let s = 5i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029928cu64) }
        0x1029928c => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10299290u64) }
        0x10299290 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299294u64) }
        0x10299294 => { vm.decode(0x6a8014f5u32); let d = 10i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299298u64) }
        0x10299298 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029929cu64) }
        0x1029929c => { vm.decode(0x36552835u32); let d = 25i16; let s = 10i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102992a0u64) }
        0x102992a0 => { vm.decode(0x546f4349u32); let d = 10i16; let s = 30i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102992a4u64) }
        0x102992a4 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102992a8u64) }
        0x102992a8 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102992acu64) }
        0x102992ac => { vm.decode(0x6a8014f5u32); let d = 10i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102992b0u64) }
        0x102992b0 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102992b4u64) }
        0x102992b4 => { vm.decode(0x2a82c36fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102992b8u64) }
        0x102992b8 => { vm.decode(0x330d0835u32); let d = 14i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102992bcu64) }
        0x102992bc => { vm.decode(0x608a4befu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102992c0u64) }
        0x102992c0 => { vm.decode(0x41e12bb5u32); let d = 10i16; let s = 31i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102992c4u64) }
        0x102992c4 => { vm.decode(0x6413cd35u32); let d = 23i16; let s = 10i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102992c8u64) }
        0x102992c8 => { vm.decode(0xdb67c909u32); let d = 23i16; let s = 23i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102992ccu64) }
        0x102992cc => { vm.decode(0xf240fd75u32); let d = 23i16; let s = 23i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102992d0u64) }
        0x102992d0 => { vm.decode(0xc96624f5u32); let d = 23i16; let s = 4i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102992d4u64) }
        0x102992d4 => { vm.decode(0x544cd349u32); let d = 30i16; let s = 16i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102992d8u64) }
        0x102992d8 => { vm.decode(0xfb6d4909u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102992dcu64) }
        0x102992dc => { vm.decode(0xf308dd75u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102992e0u64) }
        0x102992e0 => { vm.decode(0x6ba614f5u32); let d = 30i16; let s = 3i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102992e4u64) }
        0x102992e4 => { vm.decode(0x40aa4a2fu32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102992e8u64) }
        0x102992e8 => { vm.decode(0x31549835u32); let d = 4i16; let s = 19i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102992ecu64) }
        0x102992ec => { vm.decode(0x910dcb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102992f0u64) }
        0x102992f0 => { vm.decode(0x44ce5349u32); let d = 19i16; let s = 24i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102992f4u64) }
        0x102992f4 => { vm.decode(0x9b63c909u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102992f8u64) }
        0x102992f8 => { vm.decode(0xe2407d75u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102992fcu64) }
        0x102992fc => { vm.decode(0xeb4404f5u32); let d = 19i16; let s = 3i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299300u64) }
        0x10299300 => { vm.decode(0x6028412fu32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299304u64) }
        0x10299304 => { vm.decode(0x882c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299308u64) }
        0x10299308 => { vm.decode(0x41051835u32); let d = 4i16; let s = 4i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029930cu64) }
        0x1029930c => { vm.decode(0x9285cb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299310u64) }
        0x10299310 => { vm.decode(0x44dc5349u32); let d = 19i16; let s = 17i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10299314u64) }
        0x10299314 => { vm.decode(0x9b63c909u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10299318u64) }
        0x10299318 => { vm.decode(0xe2407d75u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029931cu64) }
        0x1029931c => { vm.decode(0xeb4404f5u32); let d = 19i16; let s = 3i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299320u64) }
        0x10299320 => { vm.decode(0x6028412fu32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299324u64) }
        0x10299324 => { vm.decode(0x886c36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299328u64) }
        0x10299328 => { vm.decode(0x41051835u32); let d = 4i16; let s = 4i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029932cu64) }
        0x1029932c => { vm.decode(0x908dcb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299330u64) }
        0x10299330 => { vm.decode(0x40de5349u32); let d = 19i16; let s = 9i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10299334u64) }
        0x10299334 => { vm.decode(0x9b63c909u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10299338u64) }
        0x10299338 => { vm.decode(0xe2407d75u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029933cu64) }
        0x1029933c => { vm.decode(0xeb4404f5u32); let d = 19i16; let s = 3i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299340u64) }
        0x10299340 => { vm.decode(0x6028412fu32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299344u64) }
        0x10299344 => { vm.decode(0x86c36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299348u64) }
        0x10299348 => { vm.decode(0x41051835u32); let d = 4i16; let s = 4i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029934cu64) }
        0x1029934c => { vm.decode(0x9205cb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299350u64) }
        0x10299350 => { vm.decode(0x1510cb11u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10299354u64) }
        0x10299354 => { vm.decode(0x446c5349u32); let d = 18i16; let s = 20i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10299358u64) }
        0x10299358 => { vm.decode(0x9b614909u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029935cu64) }
        0x1029935c => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299360u64) }
        0x10299360 => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299364u64) }
        0x10299364 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299368u64) }
        0x10299368 => { vm.decode(0x882e32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029936cu64) }
        0x1029936c => { vm.decode(0x1051835u32); let d = 4i16; let s = 4i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299370u64) }
        0x10299370 => { vm.decode(0x900dc30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299374u64) }
        0x10299374 => { vm.decode(0x446d5349u32); let d = 18i16; let s = 22i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10299378u64) }
        0x10299378 => { vm.decode(0x9b614909u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029937cu64) }
        0x1029937c => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299380u64) }
        0x10299380 => { vm.decode(0x6b0404f5u32); let d = 18i16; let s = 3i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299384u64) }
        0x10299384 => { vm.decode(0x4028402fu32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299388u64) }
        0x10299388 => { vm.decode(0x1071835u32); let d = 4i16; let s = 12i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029938cu64) }
        0x1029938c => { vm.decode(0x920dcb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299390u64) }
        0x10299390 => { vm.decode(0x107dc349u32); let d = 12i16; let s = 7i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10299394u64) }
        0x10299394 => { vm.decode(0xe82d4909u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10299398u64) }
        0x10299398 => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029939cu64) }
        0x1029939c => { vm.decode(0x22a214f5u32); let d = 12i16; let s = 3i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102993a0u64) }
        0x102993a0 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102993a4u64) }
        0x102993a4 => { vm.decode(0x282e36fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102993a8u64) }
        0x102993a8 => { vm.decode(0x11018835u32); let d = 4i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102993acu64) }
        0x102993ac => { vm.decode(0x930dc30u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102993b0u64) }
        0x102993b0 => { vm.decode(0x147cc349u32); let d = 12i16; let s = 21i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102993b4u64) }
        0x102993b4 => { vm.decode(0xe82d4909u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102993b8u64) }
        0x102993b8 => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102993bcu64) }
        0x102993bc => { vm.decode(0x22a214f5u32); let d = 12i16; let s = 3i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102993c0u64) }
        0x102993c0 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102993c4u64) }
        0x102993c4 => { vm.decode(0x886c32fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102993c8u64) }
        0x102993c8 => { vm.decode(0x11018835u32); let d = 4i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102993ccu64) }
        0x102993cc => { vm.decode(0x900dcb0u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102993d0u64) }
        0x102993d0 => { vm.decode(0x104ec349u32); let d = 12i16; let s = 8i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102993d4u64) }
        0x102993d4 => { vm.decode(0xe82d4909u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102993d8u64) }
        0x102993d8 => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102993dcu64) }
        0x102993dc => { vm.decode(0x22a214f5u32); let d = 12i16; let s = 3i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102993e0u64) }
        0x102993e0 => { vm.decode(0x820a2fu32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102993e4u64) }
        0x102993e4 => { vm.decode(0x10488835u32); let d = 2i16; let s = 2i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102993e8u64) }
        0x102993e8 => { vm.decode(0x9381d30u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102993ecu64) }
        0x102993ec => { vm.decode(0xdc4349u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102993f0u64) }
        0x102993f0 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102993f4u64) }
        0x102993f4 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102993f8u64) }
        0x102993f8 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102993fcu64) }
        0x102993fc => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299400u64) }
        0x10299400 => { vm.decode(0x28cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299404u64) }
        0x10299404 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299408u64) }
        0x10299408 => { vm.decode(0x19201c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029940cu64) }
        0x1029940c => { vm.decode(0xcf4349u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10299410u64) }
        0x10299410 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10299414u64) }
        0x10299414 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299418u64) }
        0x10299418 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029941cu64) }
        0x1029941c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299420u64) }
        0x10299420 => { vm.decode(0x60422835u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299424u64) }
        0x10299424 => { vm.decode(0x19181cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299428u64) }
        0x10299428 => { vm.decode(0xd2cad16du32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029942cu64) }
        0x1029942c => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299430u64) }
        0x10299430 => { vm.decode(0x2286c32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 304i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299434u64) }
        0x10299434 => { vm.decode(0xa04024f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299438u64) }
        0x10299438 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029943cu64) }
        0x1029943c => { vm.decode(0x60032835u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299440u64) }
        0x10299440 => { vm.decode(0x19109c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299444u64) }
        0x10299444 => { vm.decode(0x4d9c6911u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10299448u64) }
        0x10299448 => { vm.decode(0xfd4349u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029944cu64) }
        0x1029944c => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10299450u64) }
        0x10299450 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299454u64) }
        0x10299454 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299458u64) }
        0x10299458 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029945cu64) }
        0x1029945c => { vm.decode(0xa8ce32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299460u64) }
        0x10299460 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299464u64) }
        0x10299464 => { vm.decode(0x19301cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299468u64) }
        0x10299468 => { vm.decode(0x6ca4349u32); let d = 1i16; let s = 24i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029946cu64) }
        0x1029946c => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10299470u64) }
        0x10299470 => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299474u64) }
        0x10299474 => { vm.decode(0x824024f5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299478u64) }
        0x10299478 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029947cu64) }
        0x1029947c => { vm.decode(0x40432835u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299480u64) }
        0x10299480 => { vm.decode(0x19389c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299484u64) }
        0x10299484 => { vm.decode(0x6c84349u32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10299488u64) }
        0x10299488 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029948cu64) }
        0x1029948c => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299490u64) }
        0x10299490 => { vm.decode(0x824024f5u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299494u64) }
        0x10299494 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299498u64) }
        0x10299498 => { vm.decode(0x60122835u32); let d = 1i16; let s = 1i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029949cu64) }
        0x1029949c => { vm.decode(0x19189cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102994a0u64) }
        0x102994a0 => { vm.decode(0x594f811u32); let d = 8i16; let s = 29i16; let s2 = -1i16; let imm = 376i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102994a4u64) }
        0x102994a4 => { vm.decode(0xce4349u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 2064i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102994a8u64) }
        0x102994a8 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102994acu64) }
        0x102994ac => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102994b0u64) }
        0x102994b0 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102994b4u64) }
        0x102994b4 => { vm.decode(0x19089c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102994b8u64) }
        0x102994b8 => { vm.decode(0x20a0412fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102994bcu64) }
        0x102994bc => { vm.decode(0x888c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102994c0u64) }
        0x102994c0 => { vm.decode(0x40402835u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102994c4u64) }
        0x102994c4 => { vm.decode(0x19209c30u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 448i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102994c8u64) }
        0x102994c8 => { vm.decode(0x228443afu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102994ccu64) }
        0x102994cc => { vm.decode(0xd2ca812du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102994d0u64) }
        0x102994d0 => { vm.decode(0x28c63afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102994d4u64) }
        0x102994d4 => { vm.decode(0xd25a412du32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102994d8u64) }
        0x102994d8 => { vm.decode(0x208c43efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102994dcu64) }
        0x102994dc => { vm.decode(0xdadac12du32); let d = 11i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102994e0u64) }
        0x102994e0 => { vm.decode(0x228843efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 592i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102994e4u64) }
        0x102994e4 => { vm.decode(0xda4ec12du32); let d = 12i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102994e8u64) }
        0x102994e8 => { vm.decode(0x228c43efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102994ecu64) }
        0x102994ec => { vm.decode(0xda5ec12du32); let d = 14i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102994f0u64) }
        0x102994f0 => { vm.decode(0x228843afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102994f4u64) }
        0x102994f4 => { vm.decode(0xda4ac1adu32); let d = 24i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102994f8u64) }
        0x102994f8 => { vm.decode(0x228863efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102994fcu64) }
        0x102994fc => { vm.decode(0xdacac1adu32); let d = 25i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299500u64) }
        0x10299500 => { vm.decode(0x208863efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299504u64) }
        0x10299504 => { vm.decode(0xd24ac1adu32); let d = 16i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299508u64) }
        0x10299508 => { vm.decode(0x2088e36fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 328i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029950cu64) }
        0x1029950c => { vm.decode(0xd25ac1adu32); let d = 18i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299510u64) }
        0x10299510 => { vm.decode(0xe2005d75u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299514u64) }
        0x10299514 => { vm.decode(0xe100dd75u32); let d = 22i16; let s = 24i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299518u64) }
        0x10299518 => { vm.decode(0xb3089d75u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029951cu64) }
        0x1029951c => { vm.decode(0xf1401d75u32); let d = 19i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299520u64) }
        0x10299520 => { vm.decode(0xe340ad75u32); let d = 21i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299524u64) }
        0x10299524 => { vm.decode(0xe040dd75u32); let d = 23i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299528u64) }
        0x10299528 => { vm.decode(0xa2001d75u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029952cu64) }
        0x1029952c => { vm.decode(0xa0402d75u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299530u64) }
        0x10299530 => { vm.decode(0xa1487d75u32); let d = 11i16; let s = 25i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299534u64) }
        0x10299534 => { vm.decode(0x208c43afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299538u64) }
        0x10299538 => { vm.decode(0xda4ec12du32); let d = 12i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029953cu64) }
        0x1029953c => { vm.decode(0x288863afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299540u64) }
        0x10299540 => { vm.decode(0xda4ac1adu32); let d = 24i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299544u64) }
        0x10299544 => { vm.decode(0x288843afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299548u64) }
        0x10299548 => { vm.decode(0xdacac1adu32); let d = 25i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029954cu64) }
        0x1029954c => { vm.decode(0x2a8843afu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299550u64) }
        0x10299550 => { vm.decode(0xd24ac1adu32); let d = 16i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299554u64) }
        0x10299554 => { vm.decode(0x228c63efu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299558u64) }
        0x10299558 => { vm.decode(0xda5ec1adu32); let d = 30i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x1029955cu64) }
        0x1029955c => { vm.decode(0xb348dd75u32); let d = 15i16; let s = 30i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299560u64) }
        0x10299560 => { vm.decode(0xa0405d75u32); let d = 3i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299564u64) }
        0x10299564 => { vm.decode(0xe1486d75u32); let d = 25i16; let s = 25i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299568u64) }
        0x10299568 => { vm.decode(0xa100cd75u32); let d = 4i16; let s = 24i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029956cu64) }
        0x1029956c => { vm.decode(0xf1089d75u32); let d = 30i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299570u64) }
        0x10299570 => { vm.decode(0xda6ed16du32); let d = 12i16; let s = 31i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x10299574u64) }
        0x10299574 => { vm.decode(0xb1088d75u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299578u64) }
        0x10299578 => { vm.decode(0x20a234f5u32); let d = 12i16; let s = 5i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029957cu64) }
        0x1029957c => { vm.decode(0xb3054f0u32); let d = 12i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299580u64) }
        0x10299580 => { vm.decode(0xe08434f5u32); let d = 24i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299584u64) }
        0x10299584 => { vm.decode(0xa08224f5u32); let d = 12i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299588u64) }
        0x10299588 => { vm.decode(0x68c024f5u32); let d = 11i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029958cu64) }
        0x1029958c => { vm.decode(0xe16024f5u32); let d = 1i16; let s = 5i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299590u64) }
        0x10299590 => { vm.decode(0x696634f5u32); let d = 23i16; let s = 5i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299594u64) }
        0x10299594 => { vm.decode(0xa92024f5u32); let d = 2i16; let s = 5i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299598u64) }
        0x10299598 => { vm.decode(0xb089530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x1029959cu64) }
        0x1029959c => { vm.decode(0xe90024f5u32); let d = 2i16; let s = 5i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102995a0u64) }
        0x102995a0 => { vm.decode(0xb009530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102995a4u64) }
        0x102995a4 => { vm.decode(0x682034f5u32); let d = 2i16; let s = 5i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102995a8u64) }
        0x102995a8 => { vm.decode(0xb2015b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102995acu64) }
        0x102995ac => { vm.decode(0x696424f5u32); let d = 19i16; let s = 5i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102995b0u64) }
        0x102995b0 => { vm.decode(0x690024f5u32); let d = 2i16; let s = 5i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102995b4u64) }
        0x102995b4 => { vm.decode(0xb3815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102995b8u64) }
        0x102995b8 => { vm.decode(0x282424f5u32); let d = 18i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102995bcu64) }
        0x102995bc => { vm.decode(0xa1c434f5u32); let d = 25i16; let s = 5i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102995c0u64) }
        0x102995c0 => { vm.decode(0xe88224f5u32); let d = 14i16; let s = 3i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102995c4u64) }
        0x102995c4 => { vm.decode(0xe82034f5u32); let d = 2i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102995c8u64) }
        0x102995c8 => { vm.decode(0xb109530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 656i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102995ccu64) }
        0x102995cc => { vm.decode(0x42da4349u32); let d = 3i16; let s = 9i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102995d0u64) }
        0x102995d0 => { vm.decode(0x9a23c909u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102995d4u64) }
        0x102995d4 => { vm.decode(0xa2403d75u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102995d8u64) }
        0x102995d8 => { vm.decode(0xca4024f5u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102995dcu64) }
        0x102995dc => { vm.decode(0x2028012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102995e0u64) }
        0x102995e0 => { vm.decode(0x8ce36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 360i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102995e4u64) }
        0x102995e4 => { vm.decode(0x474c2835u32); let d = 31i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102995e8u64) }
        0x102995e8 => { vm.decode(0x46d84349u32); let d = 3i16; let s = 17i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102995ecu64) }
        0x102995ec => { vm.decode(0x9a23c909u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102995f0u64) }
        0x102995f0 => { vm.decode(0xa2403d75u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102995f4u64) }
        0x102995f4 => { vm.decode(0xca4024f5u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102995f8u64) }
        0x102995f8 => { vm.decode(0x2028012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102995fcu64) }
        0x102995fc => { vm.decode(0xa8cc32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 432i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299600u64) }
        0x10299600 => { vm.decode(0x43442835u32); let d = 13i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299604u64) }
        0x10299604 => { vm.decode(0x46e84349u32); let d = 3i16; let s = 20i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10299608u64) }
        0x10299608 => { vm.decode(0x9a23c909u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x1029960cu64) }
        0x1029960c => { vm.decode(0xa2403d75u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299610u64) }
        0x10299610 => { vm.decode(0xca4024f5u32); let d = 3i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299614u64) }
        0x10299614 => { vm.decode(0x2028012fu32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299618u64) }
        0x10299618 => { vm.decode(0x8cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 352i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029961cu64) }
        0x1029961c => { vm.decode(0x404c2835u32); let d = 3i16; let s = 2i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299620u64) }
        0x10299620 => { vm.decode(0x24bc349u32); let d = 4i16; let s = 10i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10299624u64) }
        0x10299624 => { vm.decode(0xc8254909u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10299628u64) }
        0x10299628 => { vm.decode(0xb0008d75u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029962cu64) }
        0x1029962c => { vm.decode(0x22224f5u32); let d = 4i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299630u64) }
        0x10299630 => { vm.decode(0x82002fu32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299634u64) }
        0x10299634 => { vm.decode(0x88ce32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 424i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299638u64) }
        0x10299638 => { vm.decode(0x1408835u32); let d = 4i16; let s = 2i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029963cu64) }
        0x1029963c => { vm.decode(0x45104211u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10299640u64) }
        0x10299640 => { vm.decode(0x52494349u32); let d = 10i16; let s = 2i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10299644u64) }
        0x10299644 => { vm.decode(0xba294909u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10299648u64) }
        0x10299648 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x1029964cu64) }
        0x1029964c => { vm.decode(0x4a8034f5u32); let d = 10i16; let s = 6i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299650u64) }
        0x10299650 => { vm.decode(0x280a2fu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299654u64) }
        0x10299654 => { vm.decode(0x888c32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 384i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299658u64) }
        0x10299658 => { vm.decode(0x124c0835u32); let d = 10i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029965cu64) }
        0x1029965c => { vm.decode(0x52cac349u32); let d = 15i16; let s = 8i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10299660u64) }
        0x10299660 => { vm.decode(0xfa2fc909u32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10299664u64) }
        0x10299664 => { vm.decode(0xb348bd75u32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299668u64) }
        0x10299668 => { vm.decode(0xcae234f5u32); let d = 15i16; let s = 6i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029966cu64) }
        0x1029966c => { vm.decode(0x20aa0b2fu32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299670u64) }
        0x10299670 => { vm.decode(0x28ce32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 312i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299674u64) }
        0x10299674 => { vm.decode(0x534ca835u32); let d = 15i16; let s = 2i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299678u64) }
        0x10299678 => { vm.decode(0x45906811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 336i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029967cu64) }
        0x1029967c => { vm.decode(0x2c95349u32); let d = 17i16; let s = 2i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x10299680u64) }
        0x10299680 => { vm.decode(0x8963c909u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10299684u64) }
        0x10299684 => { vm.decode(0xe0406d75u32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299688u64) }
        0x10299688 => { vm.decode(0x834424f5u32); let d = 17i16; let s = 6i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x1029968cu64) }
        0x1029968c => { vm.decode(0x6000412fu32); let d = 17i16; let s = 17i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299690u64) }
        0x10299690 => { vm.decode(0x88c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 320i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299694u64) }
        0x10299694 => { vm.decode(0x44403835u32); let d = 17i16; let s = 2i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299698u64) }
        0x10299698 => { vm.decode(0x4518e811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 408i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x1029969cu64) }
        0x1029969c => { vm.decode(0x249d349u32); let d = 20i16; let s = 2i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102996a0u64) }
        0x102996a0 => { vm.decode(0xc9654909u32); let d = 20i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102996a4u64) }
        0x102996a4 => { vm.decode(0xf000cd75u32); let d = 20i16; let s = 20i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102996a8u64) }
        0x102996a8 => { vm.decode(0x32624f5u32); let d = 20i16; let s = 6i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102996acu64) }
        0x102996ac => { vm.decode(0x4082402fu32); let d = 20i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102996b0u64) }
        0x102996b0 => { vm.decode(0x288e36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 344i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102996b4u64) }
        0x102996b4 => { vm.decode(0x5409835u32); let d = 20i16; let s = 2i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102996b8u64) }
        0x102996b8 => { vm.decode(0x4598e011u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 456i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102996bcu64) }
        0x102996bc => { vm.decode(0x5249d349u32); let d = 30i16; let s = 2i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102996c0u64) }
        0x102996c0 => { vm.decode(0xfb6d4909u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102996c4u64) }
        0x102996c4 => { vm.decode(0xf308dd75u32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102996c8u64) }
        0x102996c8 => { vm.decode(0x4ba634f5u32); let d = 30i16; let s = 6i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102996ccu64) }
        0x102996cc => { vm.decode(0x40aa4a2fu32); let d = 30i16; let s = 30i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102996d0u64) }
        0x102996d0 => { vm.decode(0x88cc32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 416i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102996d4u64) }
        0x102996d4 => { vm.decode(0x1244b835u32); let d = 9i16; let s = 2i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102996d8u64) }
        0x102996d8 => { vm.decode(0x2f9d349u32); let d = 21i16; let s = 7i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102996dcu64) }
        0x102996dc => { vm.decode(0xc967c909u32); let d = 21i16; let s = 21i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102996e0u64) }
        0x102996e0 => { vm.decode(0xf040ed75u32); let d = 21i16; let s = 21i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102996e4u64) }
        0x102996e4 => { vm.decode(0x836624f5u32); let d = 21i16; let s = 6i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102996e8u64) }
        0x102996e8 => { vm.decode(0x6082412fu32); let d = 21i16; let s = 21i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102996ecu64) }
        0x102996ec => { vm.decode(0xa88c36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102996f0u64) }
        0x102996f0 => { vm.decode(0x4540b835u32); let d = 21i16; let s = 2i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102996f4u64) }
        0x102996f4 => { vm.decode(0x459ce811u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102996f8u64) }
        0x102996f8 => { vm.decode(0x4249d349u32); let d = 22i16; let s = 2i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x102996fcu64) }
        0x102996fc => { vm.decode(0xdb654909u32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10299700u64) }
        0x10299700 => { vm.decode(0xf200dd75u32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299704u64) }
        0x10299704 => { vm.decode(0x4b2624f5u32); let d = 22i16; let s = 6i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299708u64) }
        0x10299708 => { vm.decode(0x40aa402fu32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029970cu64) }
        0x1029970c => { vm.decode(0x88cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299710u64) }
        0x10299710 => { vm.decode(0x54c9835u32); let d = 22i16; let s = 2i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299714u64) }
        0x10299714 => { vm.decode(0x4510c211u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10299718u64) }
        0x10299718 => { vm.decode(0x2495349u32); let d = 16i16; let s = 2i16; let s2 = -1i16; let imm = 2056i64;
        let sh = (imm & 0xff) as u32;
        let wd = ((imm >> 8) & 0xff) as u32;
        let mask = if wd >= 32 { u32::MAX } else { (1u32 << wd) - 1 };
        let v = ((vm.ld(s) as u32) >> sh) & mask;
        vm.st(d, v as i32 as i64 as u64);
        Ok(0x1029971cu64) }
        0x1029971c => { vm.decode(0x89614909u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x10299720u64) }
        0x10299720 => { vm.decode(0xe0004d75u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x10299724u64) }
        0x10299724 => { vm.decode(0x30424f5u32); let d = 16i16; let s = 6i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x10299728u64) }
        0x10299728 => { vm.decode(0x4000402fu32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029972cu64) }
        0x1029972c => { vm.decode(0x28cc36fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 368i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299730u64) }
        0x10299730 => { vm.decode(0x4401835u32); let d = 16i16; let s = 2i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299734u64) }
        0x10299734 => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299738u64) }
        0x10299738 => { vm.decode(0x60432835u32); let d = 1i16; let s = 1i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029973cu64) }
        0x1029973c => { vm.decode(0x208663afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299740u64) }
        0x10299740 => { vm.decode(0x208a092fu32); let d = 15i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299744u64) }
        0x10299744 => { vm.decode(0xa88c32fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 400i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299748u64) }
        0x10299748 => { vm.decode(0x514ca835u32); let d = 7i16; let s = 2i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029974cu64) }
        0x1029974c => { vm.decode(0x20280b2fu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299750u64) }
        0x10299750 => { vm.decode(0x574e0835u32); let d = 30i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299754u64) }
        0x10299754 => { vm.decode(0x880a2fu32); let d = 10i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299758u64) }
        0x10299758 => { vm.decode(0x11050835u32); let d = 4i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x1029975cu64) }
        0x1029975c => { vm.decode(0x84a2fu32); let d = 10i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299760u64) }
        0x10299760 => { vm.decode(0x32440835u32); let d = 8i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299764u64) }
        0x10299764 => { vm.decode(0x28c43efu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 624i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299768u64) }
        0x10299768 => { vm.decode(0x2028002fu32); let d = 3i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029976cu64) }
        0x1029976c => { vm.decode(0x600f2835u32); let d = 3i16; let s = 3i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299770u64) }
        0x10299770 => { vm.decode(0xa8492fu32); let d = 10i16; let s = 23i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299774u64) }
        0x10299774 => { vm.decode(0x355f2835u32); let d = 23i16; let s = 10i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x10299778u64) }
        0x10299778 => { vm.decode(0x42058019u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x1029977cu64) }
        0x1029977c => { vm.decode(0x1b0854b0u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x10299780u64) }
        0x10299780 => { vm.decode(0x4594c211u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x10299784u64) }
        0x10299784 => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x10299788u64) }
        0x10299788 => { vm.decode(0x621d5022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x1029978cu64) }
        0x1029978c => { vm.decode(0x28412fu32); let d = 2i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299790u64) }
        0x10299790 => { vm.decode(0xa80a2fu32); let d = 10i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299794u64) }
        0x10299794 => { vm.decode(0x208643efu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x10299798u64) }
        0x10299798 => { vm.decode(0x2088092fu32); let d = 11i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x1029979cu64) }
        0x1029979c => { vm.decode(0x24b2fu32); let d = 12i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102997a0u64) }
        0x102997a0 => { vm.decode(0x288243afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 640i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102997a4u64) }
        0x102997a4 => { vm.decode(0x2082092fu32); let d = 13i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102997a8u64) }
        0x102997a8 => { vm.decode(0x2a482fu32); let d = 14i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102997acu64) }
        0x102997ac => { vm.decode(0x288263afu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 648i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102997b0u64) }
        0x102997b0 => { vm.decode(0x208a092fu32); let d = 15i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102997b4u64) }
        0x102997b4 => { vm.decode(0x228663efu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 632i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102997b8u64) }
        0x102997b8 => { vm.decode(0x4080092fu32); let d = 24i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102997bcu64) }
        0x102997bc => { vm.decode(0x2882e32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 392i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102997c0u64) }
        0x102997c0 => { vm.decode(0x6080092fu32); let d = 25i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102997c4u64) }
        0x102997c4 => { vm.decode(0x2a86e32fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 440i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102997c8u64) }
        0x102997c8 => { vm.decode(0x36011835u32); let d = 24i16; let s = 5i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102997ccu64) }
        0x102997cc => { vm.decode(0x2a82e36fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 472i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102997d0u64) }
        0x102997d0 => { vm.decode(0x740d8835u32); let d = 18i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102997d4u64) }
        _ => Err(Error::UnknownPc(pc)),
    }
}
