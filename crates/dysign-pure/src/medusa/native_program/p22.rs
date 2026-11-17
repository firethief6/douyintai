#![cfg_attr(rustfmt, rustfmt::skip)]
// 由 scripts/medusa-vm/gen_native.py 生成，勿手改。
use super::super::Vm;
use super::super::Error;

#[allow(clippy::all, unused_parens, unused_variables, unused_mut)]
pub fn exec(vm: &mut Vm, pc: u64) -> Result<u64, Error> {
    match pc {
        0x102d77e4 => { vm.decode(0x191815b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d77e8u64) }
        0x102d77e8 => { vm.decode(0x28016fu32); let d = 2i16; let s = 3i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d77ecu64) }
        0x102d77ec => { vm.decode(0x1b2014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 608i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d77f0u64) }
        0x102d77f0 => { vm.decode(0xb381530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d77f4u64) }
        0x102d77f4 => { vm.decode(0x664d8119u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 600i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d77f8u64) }
        0x102d77f8 => { vm.decode(0x190855b0u32); let d = 7i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d77fcu64) }
        0x102d77fc => { vm.decode(0x99201470u32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7800u64) }
        0x102d7800 => { vm.decode(0xe00d875u32); let d = 25i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7804u64) }
        0x102d7804 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d7808u64 + 4);
        vm.wr(37, 0x102d7808u64 + 4);
        Ok(0x102d7808u64) }
        0x102d7808 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d780cu64) }
        0x102d780c => { vm.decode(0x60804b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 64i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7810u64) }
        0x102d7810 => { vm.decode(0x214184f5u32); let d = 1i16; let s = 16i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7814u64) }
        0x102d7814 => { vm.decode(0x4594c211u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 616i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let b = vm.mem.read(a, 4)?;
        let v = i32::from_le_bytes(b.try_into().unwrap()) as i64;
        vm.st(d, v as u64);
        Ok(0x102d7818u64) }
        0x102d7818 => { vm.decode(0x22355022u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 168i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102d781cu64) }
        0x102d781c => { vm.decode(0x2000012fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7820u64) }
        0x102d7820 => { vm.decode(0x2210571u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 2388i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d7824u64).wrapping_add((imm * 4) as u64) } else { 0x102d7824u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d7824u64) }
        0x102d7824 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d7828u64) } }
        0x102d7828 => { vm.decode(0x89085530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d782cu64) }
        0x102d782c => { vm.decode(0x99105530u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7830u64) }
        0x102d7830 => { vm.decode(0x834104f5u32); let d = 1i16; let s = 17i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7834u64) }
        0x102d7834 => { vm.decode(0x19181430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7838u64) }
        0x102d7838 => { vm.decode(0x2084636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d783cu64) }
        0x102d783c => { vm.decode(0xa14004f5u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7840u64) }
        0x102d7840 => { vm.decode(0x192814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7844u64) }
        0x102d7844 => { vm.decode(0x2284436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7848u64) }
        0x102d7848 => { vm.decode(0xa90604f5u32); let d = 22i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d784cu64) }
        0x102d784c => { vm.decode(0x2284636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7850u64) }
        0x102d7850 => { vm.decode(0xa98604f5u32); let d = 30i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7854u64) }
        0x102d7854 => { vm.decode(0xa34104f5u32); let d = 1i16; let s = 17i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7858u64) }
        0x102d7858 => { vm.decode(0x193814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d785cu64) }
        0x102d785c => { vm.decode(0x814124f5u32); let d = 1i16; let s = 17i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7860u64) }
        0x102d7860 => { vm.decode(0x193014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7864u64) }
        0x102d7864 => { vm.decode(0x4286632fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7868u64) }
        0x102d7868 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d786cu64) }
        0x102d786c => { vm.decode(0x10009419u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 2048i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d7870u64) }
        0x102d7870 => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7874u64) }
        0x102d7874 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7878u64) }
        0x102d7878 => { vm.decode(0x761b8019u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d787cu64) }
        0x102d787c => { vm.decode(0x608e432fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7880u64) }
        0x102d7880 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7884u64) }
        0x102d7884 => { vm.decode(0x408c632fu32); let d = 18i16; let s = 29i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7888u64) }
        0x102d7888 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d788cu64) }
        0x102d788c => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d7890u64 + 4);
        vm.wr(37, 0x102d7890u64 + 4);
        Ok(0x102d7890u64) }
        0x102d7890 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7894u64) }
        0x102d7894 => { vm.decode(0x2284402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 48i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7898u64) }
        0x102d7898 => { vm.decode(0x9b281530u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 584i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d789cu64) }
        0x102d789c => { vm.decode(0x6a8443afu32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d78a0u64) }
        0x102d78a0 => { vm.decode(0x9b1814b0u32); let d = 17i16; let s = 29i16; let s2 = -1i16; let imm = 568i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d78a4u64) }
        0x102d78a4 => { vm.decode(0x1b1014b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d78a8u64) }
        0x102d78a8 => { vm.decode(0x420d1022u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 576i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u32;
        vm.mwrite(a, &v.to_le_bytes())?;
        Ok(0x102d78acu64) }
        0x102d78ac => { vm.decode(0x66598019u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 560i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d78b0u64) }
        0x102d78b0 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d78b4u64) }
        0x102d78b4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d78b8u64 + 4);
        vm.wr(37, 0x102d78b8u64 + 4);
        Ok(0x102d78b8u64) }
        0x102d78b8 => { vm.decode(0x905875u32); let d = 4i16; let s = 0i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d78bcu64) }
        0x102d78bc => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d78c0u64) }
        0x102d78c0 => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d78c4u64) }
        0x102d78c4 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d78c8u64) }
        0x102d78c8 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d78ccu64) }
        0x102d78cc => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d78d0u64) }
        0x102d78d0 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d78d4u64 + 4);
        vm.wr(37, 0x102d78d4u64 + 4);
        Ok(0x102d78d4u64) }
        0x102d78d4 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d78d8u64) }
        0x102d78d8 => { vm.decode(0x2880432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d78dcu64) }
        0x102d78dc => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d78e0u64) }
        0x102d78e0 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d78e4u64) }
        0x102d78e4 => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d78e8u64) }
        0x102d78e8 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d78ecu64) }
        0x102d78ec => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d78f0u64) }
        0x102d78f0 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d78f4u64) }
        0x102d78f4 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d78f8u64) }
        0x102d78f8 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d78fcu64) }
        0x102d78fc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d7900u64 + 4);
        vm.wr(37, 0x102d7900u64 + 4);
        Ok(0x102d7900u64) }
        0x102d7900 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7904u64) }
        0x102d7904 => { vm.decode(0x2880632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7908u64) }
        0x102d7908 => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d790cu64) }
        0x102d790c => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7910u64) }
        0x102d7910 => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7914u64) }
        0x102d7914 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7918u64) }
        0x102d7918 => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d791cu64) }
        0x102d791c => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7920u64) }
        0x102d7920 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7924u64) }
        0x102d7924 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7928u64) }
        0x102d7928 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d792cu64 + 4);
        vm.wr(37, 0x102d792cu64 + 4);
        Ok(0x102d792cu64) }
        0x102d792c => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7930u64) }
        0x102d7930 => { vm.decode(0x2084436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7934u64) }
        0x102d7934 => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7938u64) }
        0x102d7938 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d793cu64) }
        0x102d793c => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7940u64) }
        0x102d7940 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7944u64) }
        0x102d7944 => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7948u64) }
        0x102d7948 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d794cu64) }
        0x102d794c => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7950u64) }
        0x102d7950 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7954u64) }
        0x102d7954 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d7958u64 + 4);
        vm.wr(37, 0x102d7958u64 + 4);
        Ok(0x102d7958u64) }
        0x102d7958 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d795cu64) }
        0x102d795c => { vm.decode(0x2280636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7960u64) }
        0x102d7960 => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7964u64) }
        0x102d7964 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7968u64) }
        0x102d7968 => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d796cu64) }
        0x102d796c => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7970u64) }
        0x102d7970 => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7974u64) }
        0x102d7974 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7978u64) }
        0x102d7978 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d797cu64) }
        0x102d797c => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7980u64) }
        0x102d7980 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d7984u64 + 4);
        vm.wr(37, 0x102d7984u64 + 4);
        Ok(0x102d7984u64) }
        0x102d7984 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7988u64) }
        0x102d7988 => { vm.decode(0x2a80432fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d798cu64) }
        0x102d798c => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7990u64) }
        0x102d7990 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7994u64) }
        0x102d7994 => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7998u64) }
        0x102d7998 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d799cu64) }
        0x102d799c => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d79a0u64) }
        0x102d79a0 => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d79a4u64) }
        0x102d79a4 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d79a8u64) }
        0x102d79a8 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d79acu64) }
        0x102d79ac => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d79b0u64 + 4);
        vm.wr(37, 0x102d79b0u64 + 4);
        Ok(0x102d79b0u64) }
        0x102d79b0 => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d79b4u64) }
        0x102d79b4 => { vm.decode(0x2280436fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d79b8u64) }
        0x102d79b8 => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d79bcu64) }
        0x102d79bc => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d79c0u64) }
        0x102d79c0 => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d79c4u64) }
        0x102d79c4 => { vm.decode(0x2084602fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 40i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d79c8u64) }
        0x102d79c8 => { vm.decode(0x9b0895b0u32); let d = 19i16; let s = 29i16; let s2 = -1i16; let imm = 680i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d79ccu64) }
        0x102d79cc => { vm.decode(0x1b0094b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 672i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d79d0u64) }
        0x102d79d0 => { vm.decode(0x4905875u32); let d = 4i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d79d4u64) }
        0x102d79d4 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d79d8u64) }
        0x102d79d8 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d79dcu64 + 4);
        vm.wr(37, 0x102d79dcu64 + 4);
        Ok(0x102d79dcu64) }
        0x102d79dc => { vm.decode(0x490c875u32); let d = 5i16; let s = 0i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d79e0u64) }
        0x102d79e0 => { vm.decode(0x2080636fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d79e4u64) }
        0x102d79e4 => { vm.decode(0x214004f5u32); let d = 1i16; let s = 1i16; let s2 = 16i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d79e8u64) }
        0x102d79e8 => { vm.decode(0x76499019u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d79ecu64) }
        0x102d79ec => { vm.decode(0x721b1159u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 480i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d79f0u64) }
        0x102d79f0 => { vm.decode(0x102236u32); let d = 19i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d79f4u64) }
        0x102d79f4 => { vm.decode(0x409019u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d79f8u64) }
        0x102d79f8 => { vm.decode(0xa8c43afu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 688i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d79fcu64) }
        0x102d79fc => { vm.decode(0x530u32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7a00u64) }
        0x102d7a00 => { vm.decode(0x890c875u32); let d = 21i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7a04u64) }
        0x102d7a04 => { vm.decode(0x2084402fu32); let d = 1i16; let s = 20i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7a08u64) }
        0x102d7a08 => { vm.decode(0x1b0815b0u32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7a0cu64) }
        0x102d7a0c => { vm.decode(0x4a000a73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d7a10u64) }
        0x102d7a10 => { vm.decode(0x8b185530u32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7a14u64) }
        0x102d7a14 => { vm.decode(0x1b101430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7a18u64) }
        0x102d7a18 => { vm.decode(0x4086636fu32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7a1cu64) }
        0x102d7a1c => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7a20u64) }
        0x102d7a20 => { vm.decode(0x490d875u32); let d = 5i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7a24u64) }
        0x102d7a24 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7a28u64) }
        0x102d7a28 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d7a2cu64 + 4);
        vm.wr(37, 0x102d7a2cu64 + 4);
        Ok(0x102d7a2cu64) }
        0x102d7a2c => { vm.decode(0x990055b0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7a30u64) }
        0x102d7a30 => { vm.decode(0x704b0159u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d7a34u64) }
        0x102d7a34 => { vm.decode(0x2084412fu32); let d = 1i16; let s = 21i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7a38u64) }
        0x102d7a38 => { vm.decode(0x401019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d7a3cu64) }
        0x102d7a3c => { vm.decode(0xb0815b0u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7a40u64) }
        0x102d7a40 => { vm.decode(0x4a000a73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d7a44u64) }
        0x102d7a44 => { vm.decode(0x8b181430u32); let d = 16i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7a48u64) }
        0x102d7a48 => { vm.decode(0x1b101430u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7a4cu64) }
        0x102d7a4c => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7a50u64) }
        0x102d7a50 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7a54u64) }
        0x102d7a54 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d7a58u64 + 4);
        vm.wr(37, 0x102d7a58u64 + 4);
        Ok(0x102d7a58u64) }
        0x102d7a58 => { vm.decode(0x490d875u32); let d = 5i16; let s = 0i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7a5cu64) }
        0x102d7a5c => { vm.decode(0x724b8119u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 208i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d7a60u64) }
        0x102d7a60 => { vm.decode(0x8059u32); let d = 1i16; let s = 0i16; let s2 = -1i16; let imm = 256i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d7a64u64) }
        0x102d7a64 => { vm.decode(0x628e632fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7a68u64) }
        0x102d7a68 => { vm.decode(0xac412fu32); let d = 2i16; let s = 23i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7a6cu64) }
        0x102d7a6c => { vm.decode(0x1b0814b0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 552i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7a70u64) }
        0x102d7a70 => { vm.decode(0x4a000a73u32); let d = 0i16; let s = 29i16; let s2 = -1i16; let imm = 544i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d7a74u64) }
        0x102d7a74 => { vm.decode(0x9b185430u32); let d = 21i16; let s = 29i16; let s2 = -1i16; let imm = 536i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7a78u64) }
        0x102d7a78 => { vm.decode(0xb101530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 528i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7a7cu64) }
        0x102d7a7c => { vm.decode(0x2086432fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 32i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7a80u64) }
        0x102d7a80 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7a84u64) }
        0x102d7a84 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d7a88u64 + 4);
        vm.wr(37, 0x102d7a88u64 + 4);
        Ok(0x102d7a88u64) }
        0x102d7a88 => { vm.decode(0x904875u32); let d = 4i16; let s = 0i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7a8cu64) }
        0x102d7a8c => { vm.decode(0x625b8159u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d7a90u64) }
        0x102d7a90 => { vm.decode(0x192854b0u32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7a94u64) }
        0x102d7a94 => { vm.decode(0x720b0119u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 192i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d7a98u64) }
        0x102d7a98 => { vm.decode(0x20a0616fu32); let d = 1i16; let s = 23i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7a9cu64) }
        0x102d7a9c => { vm.decode(0x401019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d7aa0u64) }
        0x102d7aa0 => { vm.decode(0xb081530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7aa4u64) }
        0x102d7aa4 => { vm.decode(0x28c436fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 112i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7aa8u64) }
        0x102d7aa8 => { vm.decode(0xb001530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7aacu64) }
        0x102d7aac => { vm.decode(0x8938dcb0u32); let d = 20i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7ab0u64) }
        0x102d7ab0 => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7ab4u64) }
        0x102d7ab4 => { vm.decode(0x282632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7ab8u64) }
        0x102d7ab8 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7abcu64) }
        0x102d7abc => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d7ac0u64 + 4);
        vm.wr(37, 0x102d7ac0u64 + 4);
        Ok(0x102d7ac0u64) }
        0x102d7ac0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d7ac4u64) } }
        0x102d7ac4 => { vm.decode(0x725b9019u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 176i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d7ac8u64) }
        0x102d7ac8 => { vm.decode(0x2284632fu32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 56i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7accu64) }
        0x102d7acc => { vm.decode(0x2000216fu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 72i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7ad0u64) }
        0x102d7ad0 => { vm.decode(0x401019u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 16i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d7ad4u64) }
        0x102d7ad4 => { vm.decode(0xb081530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 520i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7ad8u64) }
        0x102d7ad8 => { vm.decode(0x28c636fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 120i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7adcu64) }
        0x102d7adc => { vm.decode(0xb001530u32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 512i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7ae0u64) }
        0x102d7ae0 => { vm.decode(0x9938ddb0u32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 504i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7ae4u64) }
        0x102d7ae4 => { vm.decode(0x19309cb0u32); let d = 1i16; let s = 29i16; let s2 = -1i16; let imm = 496i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7ae8u64) }
        0x102d7ae8 => { vm.decode(0x282632fu32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 24i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7aecu64) }
        0x102d7aec => { vm.decode(0x2086636fu32); let d = 5i16; let s = 29i16; let s2 = -1i16; let imm = 104i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7af0u64) }
        0x102d7af0 => { vm.decode(0xa109875u32); let d = 25i16; let s = 0i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7af4u64) }
        0x102d7af4 => { vm.decode(0x4d21c535u32); let d = 31i16; let s = 25i16; let s2 = -1i16; let imm = 25i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s));
        vm.wr(35, 2);
        vm.wr(31, 0x102d7af8u64 + 4);
        vm.wr(37, 0x102d7af8u64 + 4);
        Ok(0x102d7af8u64) }
        0x102d7af8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d7afcu64) } }
        0x102d7afc => { vm.decode(0x26010406u32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 256i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d7b00u64) }
        0x102d7b00 => { vm.decode(0x2200031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d7b04u64).wrapping_add((imm * 4) as u64) } else { 0x102d7b04u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d7b04u64) }
        0x102d7b04 => { vm.decode(0xa00c4bb6u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = -256i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7b08u64) }
        0x102d7b08 => { vm.decode(0x400141du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 1296i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d7b0cu64) }
        0x102d7b10 => { vm.decode(0x110019b5u32); let d = 1i16; let s = 0i16; let s2 = 5i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102d7b14u64) }
        0x102d7b14 => { vm.decode(0x4268bb5u32); let d = 5i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d7b18u64) }
        0x102d7b18 => { vm.decode(0x120217f5u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7b1cu64) }
        0x102d7b1c => { vm.decode(0x1004ad35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102d7b20u64) }
        0x102d7b20 => { vm.decode(0x560207f5u32); let d = 3i16; let s = 3i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7b24u64) }
        0x102d7b24 => { vm.decode(0x4089875u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7b28u64) }
        0x102d7b28 => { vm.decode(0x40c30bb5u32); let d = 3i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d7b2cu64) }
        0x102d7b2c => { vm.decode(0xe84124f5u32); let d = 3i16; let s = 3i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7b30u64) }
        0x102d7b30 => { vm.decode(0x220833u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d7b34u64) }
        0x102d7b34 => { vm.decode(0x8b03236u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7b38u64) }
        0x102d7b38 => { vm.decode(0x26010406u32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 256i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d7b3cu64) }
        0x102d7b3c => { vm.decode(0x4040027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 32i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d7b40u64).wrapping_add((imm * 4) as u64) } else { 0x102d7b40u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d7b40u64) }
        0x102d7b40 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d7b44u64) } }
        0x102d7b44 => { vm.decode(0x7818e7b5u32); let d = 1i16; let s = 19i16; let s2 = -1i16; let imm = 31i64;
        let v = ((vm.ld(s) as i32) >> (imm as u32)) as i64;
        vm.st(d, v as u64);
        Ok(0x102d7b48u64) }
        0x102d7b48 => { vm.decode(0x40920d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 24i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102d7b4cu64) }
        0x102d7b4c => { vm.decode(0x44290fb5u32); let d = 1i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d7b50u64) }
        0x102d7b50 => { vm.decode(0x40217f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7b54u64) }
        0x102d7b54 => { vm.decode(0xa8088b35u32); let d = 1i16; let s = 1i16; let s2 = 19i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d7b58u64) }
        0x102d7b58 => { vm.decode(0x8241a4f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7b5cu64) }
        0x102d7b5c => { vm.decode(0x800023eu32); let d = 4i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7b60u64) }
        0x102d7b60 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7b64u64) }
        0x102d7b64 => { vm.decode(0x21120406u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 3i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d7b68u64) }
        0x102d7b68 => { vm.decode(0x40227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d7b6cu64).wrapping_add((imm * 4) as u64) } else { 0x102d7b6cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d7b6cu64) }
        0x102d7b6c => { vm.decode(0x2236u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7b70u64) }
        0x102d7b70 => { vm.decode(0xa403036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7b74u64) }
        0x102d7b74 => { vm.decode(0xd2cac12du32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d7b78u64) }
        0x102d7b78 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d7b7cu64) }
        0x102d7b7c => { vm.decode(0xa0004118u32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d7b80u64) }
        0x102d7b80 => { vm.decode(0x21120406u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 3i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d7b84u64) }
        0x102d7b84 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d7b88u64).wrapping_add((imm * 4) as u64) } else { 0x102d7b88u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d7b88u64) }
        0x102d7b88 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d7b8cu64) } }
        0x102d7b8c => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7b90u64) }
        0x102d7b90 => { vm.decode(0x3120406u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 3i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d7b94u64) }
        0x102d7b94 => { vm.decode(0x739c3ea7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -34i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d7b98u64).wrapping_add((imm * 4) as u64) } else { 0x102d7b98u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d7b98u64) }
        0x102d7b98 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7b9cu64) }
        0x102d7b9c => { vm.decode(0xa600236u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7ba0u64) }
        0x102d7ba0 => { vm.decode(0xd2ca816du32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d7ba4u64) }
        0x102d7ba4 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d7ba8u64) }
        0x102d7ba8 => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d7bacu64) }
        0x102d7bac => { vm.decode(0x3120406u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 3i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d7bb0u64) }
        0x102d7bb0 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d7bb4u64).wrapping_add((imm * 4) as u64) } else { 0x102d7bb4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d7bb4u64) }
        0x102d7bb4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d7bb8u64) } }
        0x102d7bb8 => { vm.decode(0x520001du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 1120i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d7bbcu64) }
        0x102d7bbc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d7bc0u64) } }
        0x102d7bc0 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7bc4u64) }
        0x102d7bc4 => { vm.decode(0x408e4b6fu32); let d = 30i16; let s = 29i16; let s2 = -1i16; let imm = 96i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7bc8u64) }
        0x102d7bc8 => { vm.decode(0x628a6b6fu32); let d = 31i16; let s = 29i16; let s2 = -1i16; let imm = 88i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7bccu64) }
        0x102d7bcc => { vm.decode(0x2002406u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d7bd0u64) }
        0x102d7bd0 => { vm.decode(0x2200031u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 16i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d7bd4u64).wrapping_add((imm * 4) as u64) } else { 0x102d7bd4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d7bd4u64) }
        0x102d7bd4 => { vm.decode(0xb40c6ff6u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = -16i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7bd8u64) }
        0x102d7bd8 => { vm.decode(0x25a0001du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 1636i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d7bdcu64) }
        0x102d7be0 => { vm.decode(0x110219b5u32); let d = 1i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102d7be4u64) }
        0x102d7be4 => { vm.decode(0x4668bb5u32); let d = 7i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d7be8u64) }
        0x102d7be8 => { vm.decode(0x121217f5u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7becu64) }
        0x102d7bec => { vm.decode(0x1004ad35u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102d7bf0u64) }
        0x102d7bf0 => { vm.decode(0x140147f5u32); let d = 4i16; let s = 4i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7bf4u64) }
        0x102d7bf4 => { vm.decode(0x8c875u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7bf8u64) }
        0x102d7bf8 => { vm.decode(0x624b0159u32); let d = 4i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d7bfcu64) }
        0x102d7bfc => { vm.decode(0x802224f5u32); let d = 4i16; let s = 4i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7c00u64) }
        0x102d7c00 => { vm.decode(0x40020033u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d7c04u64) }
        0x102d7c04 => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7c08u64) }
        0x102d7c08 => { vm.decode(0x2002406u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d7c0cu64) }
        0x102d7c0c => { vm.decode(0x10048027u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 65i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d7c10u64).wrapping_add((imm * 4) as u64) } else { 0x102d7c10u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d7c10u64) }
        0x102d7c10 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d7c14u64) } }
        0x102d7c14 => { vm.decode(0x3018e7b5u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 31i64;
        let v = ((vm.ld(s) as i32) >> (imm as u32)) as i64;
        vm.st(d, v as u64);
        Ok(0x102d7c18u64) }
        0x102d7c18 => { vm.decode(0x50920d35u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 28i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102d7c1cu64) }
        0x102d7c1c => { vm.decode(0x40210fb5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d7c20u64) }
        0x102d7c20 => { vm.decode(0x60217f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7c24u64) }
        0x102d7c24 => { vm.decode(0x200c0b35u32); let d = 4i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d7c28u64) }
        0x102d7c28 => { vm.decode(0x6124f5u32); let d = 1i16; let s = 4i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7c2cu64) }
        0x102d7c2c => { vm.decode(0xa00023eu32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7c30u64) }
        0x102d7c30 => { vm.decode(0x2002236u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7c34u64) }
        0x102d7c34 => { vm.decode(0x23120406u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 3i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d7c38u64) }
        0x102d7c38 => { vm.decode(0x40227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d7c3cu64).wrapping_add((imm * 4) as u64) } else { 0x102d7c3cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d7c3cu64) }
        0x102d7c3c => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7c40u64) }
        0x102d7c40 => { vm.decode(0xa603236u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7c44u64) }
        0x102d7c44 => { vm.decode(0xd2ca816du32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d7c48u64) }
        0x102d7c48 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d7c4cu64) }
        0x102d7c4c => { vm.decode(0x20204118u32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d7c50u64) }
        0x102d7c50 => { vm.decode(0x23120406u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 3i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d7c54u64) }
        0x102d7c54 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d7c58u64).wrapping_add((imm * 4) as u64) } else { 0x102d7c58u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d7c58u64) }
        0x102d7c58 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d7c5cu64) } }
        0x102d7c5c => { vm.decode(0x1000036u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7c60u64) }
        0x102d7c60 => { vm.decode(0x80120406u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d7c64u64) }
        0x102d7c64 => { vm.decode(0x40227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d7c68u64).wrapping_add((imm * 4) as u64) } else { 0x102d7c68u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d7c68u64) }
        0x102d7c68 => { vm.decode(0x2002236u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7c6cu64) }
        0x102d7c6c => { vm.decode(0x49000036u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7c70u64) }
        0x102d7c70 => { vm.decode(0xd2cac16du32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d7c74u64) }
        0x102d7c74 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d7c78u64) }
        0x102d7c78 => { vm.decode(0xa0204118u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d7c7cu64) }
        0x102d7c7c => { vm.decode(0x80120406u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d7c80u64) }
        0x102d7c80 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d7c84u64).wrapping_add((imm * 4) as u64) } else { 0x102d7c84u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d7c84u64) }
        0x102d7c84 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d7c88u64) } }
        0x102d7c88 => { vm.decode(0x110219b5u32); let d = 1i16; let s = 0i16; let s2 = 7i16; let imm = 0i64;
        let v = !(vm.ld(s) | vm.ld(s2));
        vm.st(d, v);
        Ok(0x102d7c8cu64) }
        0x102d7c8c => { vm.decode(0x4678bb5u32); let d = 7i16; let s = 6i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d7c90u64) }
        0x102d7c90 => { vm.decode(0x121217f5u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7c94u64) }
        0x102d7c94 => { vm.decode(0x3005ad35u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102d7c98u64) }
        0x102d7c98 => { vm.decode(0x121347f5u32); let d = 5i16; let s = 5i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7c9cu64) }
        0x102d7c9c => { vm.decode(0x408c875u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7ca0u64) }
        0x102d7ca0 => { vm.decode(0x70bb5u32); let d = 5i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d7ca4u64) }
        0x102d7ca4 => { vm.decode(0x8a2324f5u32); let d = 6i16; let s = 5i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7ca8u64) }
        0x102d7ca8 => { vm.decode(0x40220033u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d7cacu64) }
        0x102d7cac => { vm.decode(0x226124f5u32); let d = 1i16; let s = 4i16; let s2 = 23i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7cb0u64) }
        0x102d7cb0 => { vm.decode(0x800023eu32); let d = 4i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7cb4u64) }
        0x102d7cb4 => { vm.decode(0x2002236u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7cb8u64) }
        0x102d7cb8 => { vm.decode(0x23120406u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 3i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d7cbcu64) }
        0x102d7cbc => { vm.decode(0x40227u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 8i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d7cc0u64).wrapping_add((imm * 4) as u64) } else { 0x102d7cc0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d7cc0u64) }
        0x102d7cc0 => { vm.decode(0x2000236u32); let d = 6i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7cc4u64) }
        0x102d7cc4 => { vm.decode(0xa603236u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7cc8u64) }
        0x102d7cc8 => { vm.decode(0xd2ca416du32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d7cccu64) }
        0x102d7ccc => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d7cd0u64) }
        0x102d7cd0 => { vm.decode(0x80204118u32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d7cd4u64) }
        0x102d7cd4 => { vm.decode(0x23120406u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 3i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d7cd8u64) }
        0x102d7cd8 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d7cdcu64).wrapping_add((imm * 4) as u64) } else { 0x102d7cdcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d7cdcu64) }
        0x102d7cdc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d7ce0u64) } }
        0x102d7ce0 => { vm.decode(0x1000036u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7ce4u64) }
        0x102d7ce4 => { vm.decode(0x80120406u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d7ce8u64) }
        0x102d7ce8 => { vm.decode(0x679cb6a7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -67i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d7cecu64).wrapping_add((imm * 4) as u64) } else { 0x102d7cecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d7cecu64) }
        0x102d7cec => { vm.decode(0x2002236u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7cf0u64) }
        0x102d7cf0 => { vm.decode(0x49000036u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7cf4u64) }
        0x102d7cf4 => { vm.decode(0xd2cac16du32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d7cf8u64) }
        0x102d7cf8 => { vm.decode(0x828bb5u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d7cfcu64) }
        0x102d7cfc => { vm.decode(0xa0204118u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d7d00u64) }
        0x102d7d00 => { vm.decode(0x80120406u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d7d04u64) }
        0x102d7d04 => { vm.decode(0xa7f12db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -6i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d7d08u64).wrapping_add((imm * 4) as u64) } else { 0x102d7d08u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d7d08u64) }
        0x102d7d08 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d7d0cu64) } }
        0x102d7d0c => { vm.decode(0x500141du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 1328i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d7d10u64) }
        0x102d7d10 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d7d14u64) } }
        0x102d7d14 => { vm.decode(0x236u32); let d = 2i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7d18u64) }
        0x102d7d18 => { vm.decode(0x628a432fu32); let d = 23i16; let s = 29i16; let s2 = -1i16; let imm = 16i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7d1cu64) }
        0x102d7d1c => { vm.decode(0xa0000400u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102d7d20u64) }
        0x102d7d20 => { vm.decode(0x48a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 11i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d7d24u64).wrapping_add((imm * 4) as u64) } else { 0x102d7d24u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d7d24u64) }
        0x102d7d24 => { vm.decode(0x44108875u32); let d = 3i16; let s = 0i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7d28u64) }
        0x102d7d28 => { vm.decode(0x2040003eu32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7d2cu64) }
        0x102d7d2c => { vm.decode(0x848103eu32); let d = 4i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7d30u64) }
        0x102d7d30 => { vm.decode(0x40012835u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7d34u64) }
        0x102d7d34 => { vm.decode(0x220833u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d7d38u64) }
        0x102d7d38 => { vm.decode(0x32811099u32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d7d3cu64) }
        0x102d7d3c => { vm.decode(0x30000099u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d7d40u64) }
        0x102d7d40 => { vm.decode(0x8200236u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7d44u64) }
        0x102d7d44 => { vm.decode(0xa0000400u32); let d = 1i16; let s = 2i16; let s2 = -1i16; let imm = 16i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102d7d48u64) }
        0x102d7d48 => { vm.decode(0x27f32df1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -9i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d7d4cu64).wrapping_add((imm * 4) as u64) } else { 0x102d7d4cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d7d4cu64) }
        0x102d7d4c => { vm.decode(0x40809099u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d7d50u64) }
        0x102d7d50 => { vm.decode(0xc0801118u32); let d = 2i16; let s = 21i16; let s2 = -1i16; let imm = 2i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d7d54u64) }
        0x102d7d54 => { vm.decode(0xe057b119u32); let d = 3i16; let s = 21i16; let s2 = -1i16; let imm = 254i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d7d58u64) }
        0x102d7d58 => { vm.decode(0xe2041099u32); let d = 6i16; let s = 17i16; let s2 = -1i16; let imm = 11i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d7d5cu64) }
        0x102d7d5c => { vm.decode(0x2020036u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7d60u64) }
        0x102d7d60 => { vm.decode(0x200408eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -2139095040i64;
        vm.st(d, imm as u64);
        Ok(0x102d7d64u64) }
        0x102d7d64 => { vm.decode(0xa0384118u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 32897i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d7d68u64) }
        0x102d7d68 => { vm.decode(0x20048000u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 256i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102d7d6cu64) }
        0x102d7d6c => { vm.decode(0x82002171u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 78i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d7d70u64).wrapping_add((imm * 4) as u64) } else { 0x102d7d70u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d7d70u64) }
        0x102d7d70 => { vm.decode(0x2002036u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7d74u64) }
        0x102d7d74 => { vm.decode(0xa800441du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 2204i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d7d78u64) }
        0x102d7d7c => { vm.decode(0x108a016du32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 15i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d7d80u64) }
        0x102d7d80 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d7d84u64) }
        0x102d7d84 => { vm.decode(0x714b0159u32); let d = 24i16; let s = 29i16; let s2 = -1i16; let imm = 464i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d7d88u64) }
        0x102d7d88 => { vm.decode(0x804184f5u32); let d = 1i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7d8cu64) }
        0x102d7d8c => { vm.decode(0x2000023eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7d90u64) }
        0x102d7d90 => { vm.decode(0x40110fb5u32); let d = 1i16; let s = 1i16; let s2 = 4i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d7d94u64) }
        0x102d7d94 => { vm.decode(0xd5bfcdfeu32); let d = 24i16; let s = 2i16; let s2 = -1i16; let imm = -1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7d98u64) }
        0x102d7d98 => { vm.decode(0x95140fb5u32); let d = 24i16; let s = 5i16; let s2 = 24i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d7d9cu64) }
        0x102d7d9c => { vm.decode(0xd2ca812du32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d7da0u64) }
        0x102d7da0 => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d7da4u64) }
        0x102d7da4 => { vm.decode(0xa04124f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7da8u64) }
        0x102d7da8 => { vm.decode(0x2000023eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7dacu64) }
        0x102d7dac => { vm.decode(0x40122835u32); let d = 1i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7db0u64) }
        0x102d7db0 => { vm.decode(0x21880135u32); let d = 32i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let p = (vm.ld(s) as u32 as u64).wrapping_mul(vm.ld(s2) as u32 as u64);
        vm.st(d, (p as u32 as i32 as i64) as u64);
        vm.st(d.wrapping_add(1), ((p >> 32) as u32 as i32 as i64) as u64);
        Ok(0x102d7db4u64) }
        0x102d7db4 => { vm.decode(0x100401f5u32); let d = 24i16; let s = 33i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d7db8u64) }
        0x102d7db8 => { vm.decode(0x801099u32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d7dbcu64) }
        0x102d7dbc => { vm.decode(0xccf6ffd9u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d7dc0u64) }
        0x102d7dc0 => { vm.decode(0x11858019u32); let d = 25i16; let s = 6i16; let s2 = -1i16; let imm = 8i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d7dc4u64) }
        0x102d7dc4 => { vm.decode(0x2413036u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 8i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7dc8u64) }
        0x102d7dc8 => { vm.decode(0xa400036u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7dccu64) }
        0x102d7dcc => { vm.decode(0x44684d35u32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 10i64;
        let v = (vm.ld(s) as u32).wrapping_shr(imm as u32);
        vm.st(d, v as u64);
        Ok(0x102d7dd0u64) }
        0x102d7dd0 => { vm.decode(0xc400ebb5u32); let d = 16i16; let s = 24i16; let s2 = -1i16; let imm = 11i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d7dd4u64) }
        0x102d7dd4 => { vm.decode(0xc500abb5u32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d7dd8u64) }
        0x102d7dd8 => { vm.decode(0x4f000b35u32); let d = 24i16; let s = 16i16; let s2 = 24i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_sub(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d7ddcu64) }
        0x102d7ddc => { vm.decode(0xd4000fb5u32); let d = 1i16; let s = 1i16; let s2 = 24i16; let imm = 0i64;
        let v = (vm.ld(s2) as u32).wrapping_add(vm.ld(s) as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d7de0u64) }
        0x102d7de0 => { vm.decode(0x9129036u32); let d = 24i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7de4u64) }
        0x102d7de4 => { vm.decode(0x129036u32); let d = 16i16; let s = 1i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7de8u64) }
        0x102d7de8 => { vm.decode(0x8109236u32); let d = 18i16; let s = 1i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7decu64) }
        0x102d7dec => { vm.decode(0x10b236u32); let d = 19i16; let s = 1i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7df0u64) }
        0x102d7df0 => { vm.decode(0xa121036u32); let d = 20i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7df4u64) }
        0x102d7df4 => { vm.decode(0x8a65c909u32); let d = 22i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d7df8u64) }
        0x102d7df8 => { vm.decode(0x692704f5u32); let d = 22i16; let s = 17i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7dfcu64) }
        0x102d7dfc => { vm.decode(0xa4c103eu32); let d = 22i16; let s = 22i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7e00u64) }
        0x102d7e00 => { vm.decode(0x535da835u32); let d = 15i16; let s = 15i16; let s2 = 22i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7e04u64) }
        0x102d7e04 => { vm.decode(0xe0630433u32); let d = 15i16; let s = 6i16; let s2 = -1i16; let imm = 5i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d7e08u64) }
        0x102d7e08 => { vm.decode(0xeb274909u32); let d = 15i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d7e0cu64) }
        0x102d7e0c => { vm.decode(0x9b63c909u32); let d = 19i16; let s = 19i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d7e10u64) }
        0x102d7e10 => { vm.decode(0x9b614909u32); let d = 18i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d7e14u64) }
        0x102d7e14 => { vm.decode(0x89614909u32); let d = 16i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d7e18u64) }
        0x102d7e18 => { vm.decode(0xa9694909u32); let d = 24i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d7e1cu64) }
        0x102d7e1c => { vm.decode(0xa101036u32); let d = 20i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7e20u64) }
        0x102d7e20 => { vm.decode(0xc9654909u32); let d = 20i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d7e24u64) }
        0x102d7e24 => { vm.decode(0x212704f5u32); let d = 20i16; let s = 17i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7e28u64) }
        0x102d7e28 => { vm.decode(0x844103eu32); let d = 20i16; let s = 20i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7e2cu64) }
        0x102d7e2c => { vm.decode(0x131d8835u32); let d = 14i16; let s = 14i16; let s2 = 20i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7e30u64) }
        0x102d7e30 => { vm.decode(0x606104b3u32); let d = 14i16; let s = 6i16; let s2 = -1i16; let imm = 6i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d7e34u64) }
        0x102d7e34 => { vm.decode(0x298314f5u32); let d = 14i16; let s = 17i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7e38u64) }
        0x102d7e38 => { vm.decode(0x218504f5u32); let d = 24i16; let s = 16i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7e3cu64) }
        0x102d7e3c => { vm.decode(0x610504f5u32); let d = 16i16; let s = 17i16; let s2 = 18i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7e40u64) }
        0x102d7e40 => { vm.decode(0xe90504f5u32); let d = 18i16; let s = 17i16; let s2 = 19i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7e44u64) }
        0x102d7e44 => { vm.decode(0xe8e314f5u32); let d = 15i16; let s = 15i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7e48u64) }
        0x102d7e48 => { vm.decode(0x23036u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7e4cu64) }
        0x102d7e4c => { vm.decode(0x8823c909u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s) as u32 as u64;
        vm.st(d, v);
        Ok(0x102d7e50u64) }
        0x102d7e50 => { vm.decode(0xa04104f5u32); let d = 1i16; let s = 1i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7e54u64) }
        0x102d7e54 => { vm.decode(0x2000023eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7e58u64) }
        0x102d7e58 => { vm.decode(0x7000a835u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7e5cu64) }
        0x102d7e5c => { vm.decode(0xc02204b3u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 7i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d7e60u64) }
        0x102d7e60 => { vm.decode(0x2008323eu32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7e64u64) }
        0x102d7e64 => { vm.decode(0x3000a835u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7e68u64) }
        0x102d7e68 => { vm.decode(0x40220133u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d7e6cu64) }
        0x102d7e6c => { vm.decode(0x2048003eu32); let d = 1i16; let s = 18i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7e70u64) }
        0x102d7e70 => { vm.decode(0x70002835u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7e74u64) }
        0x102d7e74 => { vm.decode(0xc0220133u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 9i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d7e78u64) }
        0x102d7e78 => { vm.decode(0x2040003eu32); let d = 1i16; let s = 16i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7e7cu64) }
        0x102d7e7c => { vm.decode(0x70042835u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7e80u64) }
        0x102d7e80 => { vm.decode(0x402201b3u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 10i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d7e84u64) }
        0x102d7e84 => { vm.decode(0x2040203eu32); let d = 1i16; let s = 24i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7e88u64) }
        0x102d7e88 => { vm.decode(0x30042835u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7e8cu64) }
        0x102d7e8c => { vm.decode(0xc02201b3u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 11i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d7e90u64) }
        0x102d7e90 => { vm.decode(0x2008303eu32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7e94u64) }
        0x102d7e94 => { vm.decode(0x30002835u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7e98u64) }
        0x102d7e98 => { vm.decode(0x40220533u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 12i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d) as u8;
        vm.mwrite(a, &[v])?;
        Ok(0x102d7e9cu64) }
        0x102d7e9c => { vm.decode(0x20048000u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 256i64;
        let v = (vm.ld(s) < imm as u64) as u64;
        vm.st(d, v);
        Ok(0x102d7ea0u64) }
        0x102d7ea0 => { vm.decode(0x4040a27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 42i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d7ea4u64).wrapping_add((imm * 4) as u64) } else { 0x102d7ea4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d7ea4u64) }
        0x102d7ea4 => { vm.decode(0x44b00875u32); let d = 6i16; let s = 0i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7ea8u64) }
        0x102d7ea8 => { vm.decode(0xd5bbdd7eu32); let d = 8i16; let s = 6i16; let s2 = -1i16; let imm = -2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7eacu64) }
        0x102d7eac => { vm.decode(0xc7bbddfeu32); let d = 10i16; let s = 6i16; let s2 = -1i16; let imm = -3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7eb0u64) }
        0x102d7eb0 => { vm.decode(0x220a10beu32); let d = 11i16; let s = 6i16; let s2 = -1i16; let imm = 1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7eb4u64) }
        0x102d7eb4 => { vm.decode(0x200a103eu32); let d = 9i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7eb8u64) }
        0x102d7eb8 => { vm.decode(0x80a107eu32); let d = 12i16; let s = 6i16; let s2 = -1i16; let imm = 4i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7ebcu64) }
        0x102d7ebc => { vm.decode(0x1a0a103eu32); let d = 14i16; let s = 6i16; let s2 = -1i16; let imm = 2i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7ec0u64) }
        0x102d7ec0 => { vm.decode(0x380a10beu32); let d = 13i16; let s = 6i16; let s2 = -1i16; let imm = 3i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7ec4u64) }
        0x102d7ec4 => { vm.decode(0xffbbddfeu32); let d = 15i16; let s = 6i16; let s2 = -1i16; let imm = -1i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7ec8u64) }
        0x102d7ec8 => { vm.decode(0x82016du32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d7eccu64) }
        0x102d7ecc => { vm.decode(0xa7d32cb1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -85i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d7ed0u64).wrapping_add((imm * 4) as u64) } else { 0x102d7ed0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d7ed0u64) }
        0x102d7ed0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d7ed4u64) } }
        0x102d7ed4 => { vm.decode(0xd2ca112du32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d7ed8u64) }
        0x102d7ed8 => { vm.decode(0xda5a512du32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d7edcu64) }
        0x102d7edc => { vm.decode(0xdadad12du32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d7ee0u64) }
        0x102d7ee0 => { vm.decode(0xda4a91adu32); let d = 24i16; let s = 9i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d7ee4u64) }
        0x102d7ee4 => { vm.decode(0xda4e116du32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d7ee8u64) }
        0x102d7ee8 => { vm.decode(0xda4a916du32); let d = 8i16; let s = 13i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d7eecu64) }
        0x102d7eec => { vm.decode(0xdaca516du32); let d = 9i16; let s = 14i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d7ef0u64) }
        0x102d7ef0 => { vm.decode(0xdaced16du32); let d = 13i16; let s = 15i16; let s2 = -1i16; let imm = 255i64;
        let v = vm.ld(s) & imm as u64;
        vm.st(d, v);
        Ok(0x102d7ef4u64) }
        0x102d7ef4 => { vm.decode(0xa04124f5u32); let d = 1i16; let s = 1i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7ef8u64) }
        0x102d7ef8 => { vm.decode(0x208134f5u32); let d = 8i16; let s = 8i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7efcu64) }
        0x102d7efc => { vm.decode(0x2000023eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7f00u64) }
        0x102d7f00 => { vm.decode(0xa0a003eu32); let d = 14i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7f04u64) }
        0x102d7f04 => { vm.decode(0xa0c534f5u32); let d = 25i16; let s = 9i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7f08u64) }
        0x102d7f08 => { vm.decode(0x3204a835u32); let d = 9i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7f0cu64) }
        0x102d7f0c => { vm.decode(0x2203eu32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7f10u64) }
        0x102d7f10 => { vm.decode(0xe04134f5u32); let d = 1i16; let s = 11i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7f14u64) }
        0x102d7f14 => { vm.decode(0x688334f5u32); let d = 14i16; let s = 10i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7f18u64) }
        0x102d7f18 => { vm.decode(0xa8e134f5u32); let d = 11i16; let s = 13i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7f1cu64) }
        0x102d7f1c => { vm.decode(0x28a134f5u32); let d = 10i16; let s = 12i16; let s2 = 21i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7f20u64) }
        0x102d7f20 => { vm.decode(0x20a203eu32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7f24u64) }
        0x102d7f24 => { vm.decode(0x80a023eu32); let d = 12i16; let s = 3i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7f28u64) }
        0x102d7f28 => { vm.decode(0x134aa835u32); let d = 15i16; let s = 10i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7f2cu64) }
        0x102d7f2c => { vm.decode(0x242223eu32); let d = 10i16; let s = 25i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7f30u64) }
        0x102d7f30 => { vm.decode(0x220a223eu32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7f34u64) }
        0x102d7f34 => { vm.decode(0x80a303eu32); let d = 12i16; let s = 14i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7f38u64) }
        0x102d7f38 => { vm.decode(0x2802023eu32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7f3cu64) }
        0x102d7f3c => { vm.decode(0x214134f5u32); let d = 1i16; let s = 21i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7f40u64) }
        0x102d7f40 => { vm.decode(0xa02023eu32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.read(a, 1)?[0] as u64;
        vm.st(d, v);
        Ok(0x102d7f44u64) }
        0x102d7f44 => { vm.decode(0xaca0001du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 1740i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d7f48u64) }
        0x102d7f48 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d7f4cu64) } }
        0x102d7f4c => { vm.decode(0x131db28eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -13565952i64;
        vm.st(d, imm as u64);
        Ok(0x102d7f50u64) }
        0x102d7f50 => { vm.decode(0x23110d98u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 46192i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d7f54u64) }
        0x102d7f54 => { vm.decode(0x888432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7f58u64) }
        0x102d7f58 => { vm.decode(0x824004f5u32); let d = 1i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7f5cu64) }
        0x102d7f5c => { vm.decode(0x8012fu32); let d = 2i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7f60u64) }
        0x102d7f60 => { vm.decode(0x2236u32); let d = 3i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7f64u64) }
        0x102d7f64 => { vm.decode(0x22800406u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d7f68u64) }
        0x102d7f68 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d7f6cu64).wrapping_add((imm * 4) as u64) } else { 0x102d7f6cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d7f6cu64) }
        0x102d7f6c => { vm.decode(0x8e766fd9u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d7f70u64) }
        0x102d7f70 => { vm.decode(0x8520401du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 3296i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d7f74u64) }
        0x102d7f78 => { vm.decode(0x2002a835u32); let d = 1i16; let s = 4i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7f7cu64) }
        0x102d7f7c => { vm.decode(0xb02a2d75u32); let d = 8i16; let s = 5i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d7f80u64) }
        0x102d7f80 => { vm.decode(0x20a07f5u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7f84u64) }
        0x102d7f84 => { vm.decode(0x50a240f5u32); let d = 5i16; let s = 5i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d7f88u64) }
        0x102d7f88 => { vm.decode(0x160357f5u32); let d = 5i16; let s = 5i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7f8cu64) }
        0x102d7f8c => { vm.decode(0x408c875u32); let d = 1i16; let s = 1i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d7f90u64) }
        0x102d7f90 => { vm.decode(0x426204f5u32); let d = 5i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7f94u64) }
        0x102d7f94 => { vm.decode(0x10001430u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d7f98u64) }
        0x102d7f98 => { vm.decode(0x8203236u32); let d = 3i16; let s = 3i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7f9cu64) }
        0x102d7f9c => { vm.decode(0x22800406u32); let d = 1i16; let s = 3i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d7fa0u64) }
        0x102d7fa0 => { vm.decode(0x140c8aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 251i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d7fa4u64).wrapping_add((imm * 4) as u64) } else { 0x102d7fa4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d7fa4u64) }
        0x102d7fa4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d7fa8u64) } }
        0x102d7fa8 => { vm.decode(0x4c78bb5u32); let d = 7i16; let s = 3i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d7facu64) }
        0x102d7fac => { vm.decode(0xb240ad75u32); let d = 5i16; let s = 7i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d7fb0u64) }
        0x102d7fb0 => { vm.decode(0xa06104f5u32); let d = 1i16; let s = 5i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d7fb4u64) }
        0x102d7fb4 => { vm.decode(0xa012fu32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d7fb8u64) }
        0x102d7fb8 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7fbcu64) }
        0x102d7fbc => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d7fc0u64) }
        0x102d7fc0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d7fc4u64).wrapping_add((imm * 4) as u64) } else { 0x102d7fc4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d7fc4u64) }
        0x102d7fc4 => { vm.decode(0x1000019u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d7fc8u64) }
        0x102d7fc8 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7fccu64) }
        0x102d7fcc => { vm.decode(0x21400d75u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d7fd0u64) }
        0x102d7fd0 => { vm.decode(0xc118u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d7fd4u64) }
        0x102d7fd4 => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d7fd8u64) }
        0x102d7fd8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d7fdcu64).wrapping_add((imm * 4) as u64) } else { 0x102d7fdcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d7fdcu64) }
        0x102d7fdc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d7fe0u64) } }
        0x102d7fe0 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7fe4u64) }
        0x102d7fe4 => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d7fe8u64) }
        0x102d7fe8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d7fecu64).wrapping_add((imm * 4) as u64) } else { 0x102d7fecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d7fecu64) }
        0x102d7fec => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d7ff0u64) }
        0x102d7ff0 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d7ff4u64) }
        0x102d7ff4 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d7ff8u64) }
        0x102d7ff8 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d7ffcu64) }
        0x102d7ffc => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8000u64) }
        0x102d8000 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8004u64).wrapping_add((imm * 4) as u64) } else { 0x102d8004u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8004u64) }
        0x102d8004 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8008u64) } }
        0x102d8008 => { vm.decode(0x42a835u32); let d = 1i16; let s = 4i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d800cu64) }
        0x102d800c => { vm.decode(0xb26a0d75u32); let d = 9i16; let s = 6i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8010u64) }
        0x102d8010 => { vm.decode(0x20a17f5u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8014u64) }
        0x102d8014 => { vm.decode(0x548140f5u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d8018u64) }
        0x102d8018 => { vm.decode(0x505047f5u32); let d = 6i16; let s = 6i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d801cu64) }
        0x102d801c => { vm.decode(0x8d875u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8020u64) }
        0x102d8020 => { vm.decode(0x2621236u32); let d = 6i16; let s = 7i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8024u64) }
        0x102d8024 => { vm.decode(0xb2480d75u32); let d = 9i16; let s = 6i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8028u64) }
        0x102d8028 => { vm.decode(0x8a0214f5u32); let d = 6i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d802cu64) }
        0x102d802c => { vm.decode(0x10401030u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d8030u64) }
        0x102d8030 => { vm.decode(0x8603036u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8034u64) }
        0x102d8034 => { vm.decode(0xa000bd75u32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8038u64) }
        0x102d8038 => { vm.decode(0x606104f5u32); let d = 1i16; let s = 6i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d803cu64) }
        0x102d803c => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d8040u64) }
        0x102d8040 => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8044u64) }
        0x102d8044 => { vm.decode(0xa2122506u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8048u64) }
        0x102d8048 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d804cu64).wrapping_add((imm * 4) as u64) } else { 0x102d804cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d804cu64) }
        0x102d804c => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8050u64) }
        0x102d8050 => { vm.decode(0x49203236u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8054u64) }
        0x102d8054 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8058u64) }
        0x102d8058 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d805cu64) }
        0x102d805c => { vm.decode(0xa2122506u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8060u64) }
        0x102d8060 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8064u64).wrapping_add((imm * 4) as u64) } else { 0x102d8064u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8064u64) }
        0x102d8064 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8068u64) } }
        0x102d8068 => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d806cu64) }
        0x102d806c => { vm.decode(0xa2122506u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8070u64) }
        0x102d8070 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8074u64).wrapping_add((imm * 4) as u64) } else { 0x102d8074u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8074u64) }
        0x102d8074 => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8078u64) }
        0x102d8078 => { vm.decode(0x49203236u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d807cu64) }
        0x102d807c => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8080u64) }
        0x102d8080 => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8084u64) }
        0x102d8084 => { vm.decode(0xa2122506u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8088u64) }
        0x102d8088 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d808cu64).wrapping_add((imm * 4) as u64) } else { 0x102d808cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d808cu64) }
        0x102d808c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8090u64) } }
        0x102d8090 => { vm.decode(0x3a835u32); let d = 1i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8094u64) }
        0x102d8094 => { vm.decode(0xa16a1d75u32); let d = 11i16; let s = 8i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8098u64) }
        0x102d8098 => { vm.decode(0x21a17f5u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d809cu64) }
        0x102d809c => { vm.decode(0x309000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d80a0u64) }
        0x102d80a0 => { vm.decode(0x44c07f5u32); let d = 8i16; let s = 8i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d80a4u64) }
        0x102d80a4 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d80a8u64) }
        0x102d80a8 => { vm.decode(0x9609036u32); let d = 8i16; let s = 7i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d80acu64) }
        0x102d80ac => { vm.decode(0xa1080d75u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d80b0u64) }
        0x102d80b0 => { vm.decode(0xa8014f5u32); let d = 10i16; let s = 2i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d80b4u64) }
        0x102d80b4 => { vm.decode(0x18400030u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d80b8u64) }
        0x102d80b8 => { vm.decode(0xa04114f5u32); let d = 1i16; let s = 9i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d80bcu64) }
        0x102d80bc => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d80c0u64) }
        0x102d80c0 => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d80c4u64) }
        0x102d80c4 => { vm.decode(0xa2122506u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d80c8u64) }
        0x102d80c8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d80ccu64).wrapping_add((imm * 4) as u64) } else { 0x102d80ccu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d80ccu64) }
        0x102d80cc => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d80d0u64) }
        0x102d80d0 => { vm.decode(0x49203236u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d80d4u64) }
        0x102d80d4 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d80d8u64) }
        0x102d80d8 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d80dcu64) }
        0x102d80dc => { vm.decode(0xa2122506u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d80e0u64) }
        0x102d80e0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d80e4u64).wrapping_add((imm * 4) as u64) } else { 0x102d80e4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d80e4u64) }
        0x102d80e4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d80e8u64) } }
        0x102d80e8 => { vm.decode(0x1002236u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d80ecu64) }
        0x102d80ec => { vm.decode(0xa2122506u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d80f0u64) }
        0x102d80f0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d80f4u64).wrapping_add((imm * 4) as u64) } else { 0x102d80f4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d80f4u64) }
        0x102d80f4 => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d80f8u64) }
        0x102d80f8 => { vm.decode(0x49203236u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d80fcu64) }
        0x102d80fc => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8100u64) }
        0x102d8100 => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8104u64) }
        0x102d8104 => { vm.decode(0xa2122506u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8108u64) }
        0x102d8108 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d810cu64).wrapping_add((imm * 4) as u64) } else { 0x102d810cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d810cu64) }
        0x102d810c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8110u64) } }
        0x102d8110 => { vm.decode(0x3a835u32); let d = 1i16; let s = 4i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8114u64) }
        0x102d8114 => { vm.decode(0xa16a3d75u32); let d = 11i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8118u64) }
        0x102d8118 => { vm.decode(0x21a17f5u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d811cu64) }
        0x102d811c => { vm.decode(0x30b200f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d8120u64) }
        0x102d8120 => { vm.decode(0x44e17f5u32); let d = 9i16; let s = 9i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8124u64) }
        0x102d8124 => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8128u64) }
        0x102d8128 => { vm.decode(0x162b036u32); let d = 9i16; let s = 7i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d812cu64) }
        0x102d812c => { vm.decode(0xa1482d75u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8130u64) }
        0x102d8130 => { vm.decode(0x8a8014f5u32); let d = 10i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8134u64) }
        0x102d8134 => { vm.decode(0x18400030u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d8138u64) }
        0x102d8138 => { vm.decode(0x8623036u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d813cu64) }
        0x102d813c => { vm.decode(0xa0483d75u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8140u64) }
        0x102d8140 => { vm.decode(0xe04114f5u32); let d = 1i16; let s = 11i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8144u64) }
        0x102d8144 => { vm.decode(0x8092fu32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d8148u64) }
        0x102d8148 => { vm.decode(0x3002036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d814cu64) }
        0x102d814c => { vm.decode(0xa1122506u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8150u64) }
        0x102d8150 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8154u64).wrapping_add((imm * 4) as u64) } else { 0x102d8154u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8154u64) }
        0x102d8154 => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8158u64) }
        0x102d8158 => { vm.decode(0x4b403036u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d815cu64) }
        0x102d815c => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8160u64) }
        0x102d8160 => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8164u64) }
        0x102d8164 => { vm.decode(0xa1122506u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8168u64) }
        0x102d8168 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d816cu64).wrapping_add((imm * 4) as u64) } else { 0x102d816cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d816cu64) }
        0x102d816c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8170u64) } }
        0x102d8170 => { vm.decode(0x3002036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8174u64) }
        0x102d8174 => { vm.decode(0xa1122506u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8178u64) }
        0x102d8178 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d817cu64).wrapping_add((imm * 4) as u64) } else { 0x102d817cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d817cu64) }
        0x102d817c => { vm.decode(0x3001019u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8180u64) }
        0x102d8180 => { vm.decode(0x4b403036u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8184u64) }
        0x102d8184 => { vm.decode(0x33400d75u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8188u64) }
        0x102d8188 => { vm.decode(0x8020c118u32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d818cu64) }
        0x102d818c => { vm.decode(0xa1122506u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8190u64) }
        0x102d8190 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8194u64).wrapping_add((imm * 4) as u64) } else { 0x102d8194u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8194u64) }
        0x102d8194 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8198u64) } }
        0x102d8198 => { vm.decode(0x43a835u32); let d = 1i16; let s = 4i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d819cu64) }
        0x102d819c => { vm.decode(0xa36a8d75u32); let d = 13i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d81a0u64) }
        0x102d81a0 => { vm.decode(0x120a17f5u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d81a4u64) }
        0x102d81a4 => { vm.decode(0x349100f5u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d81a8u64) }
        0x102d81a8 => { vm.decode(0x405d07f5u32); let d = 10i16; let s = 10i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d81acu64) }
        0x102d81ac => { vm.decode(0x289875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d81b0u64) }
        0x102d81b0 => { vm.decode(0x9629236u32); let d = 10i16; let s = 7i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d81b4u64) }
        0x102d81b4 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d81b8u64) }
        0x102d81b8 => { vm.decode(0x428214f5u32); let d = 12i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d81bcu64) }
        0x102d81bc => { vm.decode(0x18001030u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d81c0u64) }
        0x102d81c0 => { vm.decode(0x60b036u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d81c4u64) }
        0x102d81c4 => { vm.decode(0xa040bd75u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d81c8u64) }
        0x102d81c8 => { vm.decode(0xe06104f5u32); let d = 1i16; let s = 7i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d81ccu64) }
        0x102d81cc => { vm.decode(0x2092fu32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d81d0u64) }
        0x102d81d0 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d81d4u64) }
        0x102d81d4 => { vm.decode(0x83122506u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d81d8u64) }
        0x102d81d8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d81dcu64).wrapping_add((imm * 4) as u64) } else { 0x102d81dcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d81dcu64) }
        0x102d81dc => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d81e0u64) }
        0x102d81e0 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d81e4u64) }
        0x102d81e4 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d81e8u64) }
        0x102d81e8 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d81ecu64) }
        0x102d81ec => { vm.decode(0x83122506u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d81f0u64) }
        0x102d81f0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d81f4u64).wrapping_add((imm * 4) as u64) } else { 0x102d81f4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d81f4u64) }
        0x102d81f4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d81f8u64) } }
        0x102d81f8 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d81fcu64) }
        0x102d81fc => { vm.decode(0x83122506u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8200u64) }
        0x102d8200 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8204u64).wrapping_add((imm * 4) as u64) } else { 0x102d8204u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8204u64) }
        0x102d8204 => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8208u64) }
        0x102d8208 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d820cu64) }
        0x102d820c => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8210u64) }
        0x102d8210 => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8214u64) }
        0x102d8214 => { vm.decode(0x83122506u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8218u64) }
        0x102d8218 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d821cu64).wrapping_add((imm * 4) as u64) } else { 0x102d821cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d821cu64) }
        0x102d821c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8220u64) } }
        0x102d8220 => { vm.decode(0x2043a835u32); let d = 1i16; let s = 4i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8224u64) }
        0x102d8224 => { vm.decode(0xb12a9d75u32); let d = 14i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8228u64) }
        0x102d8228 => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d822cu64) }
        0x102d822c => { vm.decode(0x709040f5u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d8230u64) }
        0x102d8230 => { vm.decode(0x124d47f5u32); let d = 12i16; let s = 12i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8234u64) }
        0x102d8234 => { vm.decode(0x28c875u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8238u64) }
        0x102d8238 => { vm.decode(0xcac014f5u32); let d = 11i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d823cu64) }
        0x102d823c => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d8240u64) }
        0x102d8240 => { vm.decode(0x204114f5u32); let d = 1i16; let s = 8i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8244u64) }
        0x102d8244 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d8248u64) }
        0x102d8248 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d824cu64) }
        0x102d824c => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8250u64) }
        0x102d8250 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8254u64).wrapping_add((imm * 4) as u64) } else { 0x102d8254u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8254u64) }
        0x102d8254 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8258u64) }
        0x102d8258 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d825cu64) }
        0x102d825c => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8260u64) }
        0x102d8260 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8264u64) }
        0x102d8264 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8268u64) }
        0x102d8268 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d826cu64).wrapping_add((imm * 4) as u64) } else { 0x102d826cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d826cu64) }
        0x102d826c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8270u64) } }
        0x102d8270 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8274u64) }
        0x102d8274 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8278u64) }
        0x102d8278 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d827cu64).wrapping_add((imm * 4) as u64) } else { 0x102d827cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d827cu64) }
        0x102d827c => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8280u64) }
        0x102d8280 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8284u64) }
        0x102d8284 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8288u64) }
        0x102d8288 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d828cu64) }
        0x102d828c => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8290u64) }
        0x102d8290 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8294u64).wrapping_add((imm * 4) as u64) } else { 0x102d8294u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8294u64) }
        0x102d8294 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8298u64) } }
        0x102d8298 => { vm.decode(0x2003a835u32); let d = 1i16; let s = 4i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d829cu64) }
        0x102d829c => { vm.decode(0xa12a8d75u32); let d = 12i16; let s = 8i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d82a0u64) }
        0x102d82a0 => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d82a4u64) }
        0x102d82a4 => { vm.decode(0x309000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d82a8u64) }
        0x102d82a8 => { vm.decode(0x64c07f5u32); let d = 8i16; let s = 8i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d82acu64) }
        0x102d82ac => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d82b0u64) }
        0x102d82b0 => { vm.decode(0xca6204f5u32); let d = 7i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d82b4u64) }
        0x102d82b4 => { vm.decode(0x10401430u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d82b8u64) }
        0x102d82b8 => { vm.decode(0xa04114f5u32); let d = 1i16; let s = 9i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d82bcu64) }
        0x102d82bc => { vm.decode(0x200a012fu32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d82c0u64) }
        0x102d82c0 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d82c4u64) }
        0x102d82c4 => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d82c8u64) }
        0x102d82c8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d82ccu64).wrapping_add((imm * 4) as u64) } else { 0x102d82ccu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d82ccu64) }
        0x102d82cc => { vm.decode(0x1000019u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d82d0u64) }
        0x102d82d0 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d82d4u64) }
        0x102d82d4 => { vm.decode(0x21400d75u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d82d8u64) }
        0x102d82d8 => { vm.decode(0xc118u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d82dcu64) }
        0x102d82dc => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d82e0u64) }
        0x102d82e0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d82e4u64).wrapping_add((imm * 4) as u64) } else { 0x102d82e4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d82e4u64) }
        0x102d82e4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d82e8u64) } }
        0x102d82e8 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d82ecu64) }
        0x102d82ec => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d82f0u64) }
        0x102d82f0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d82f4u64).wrapping_add((imm * 4) as u64) } else { 0x102d82f4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d82f4u64) }
        0x102d82f4 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d82f8u64) }
        0x102d82f8 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d82fcu64) }
        0x102d82fc => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8300u64) }
        0x102d8300 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8304u64) }
        0x102d8304 => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8308u64) }
        0x102d8308 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d830cu64).wrapping_add((imm * 4) as u64) } else { 0x102d830cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d830cu64) }
        0x102d830c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8310u64) } }
        0x102d8310 => { vm.decode(0x2042a835u32); let d = 1i16; let s = 4i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8314u64) }
        0x102d8314 => { vm.decode(0xb26a2d75u32); let d = 9i16; let s = 7i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8318u64) }
        0x102d8318 => { vm.decode(0x20a17f5u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d831cu64) }
        0x102d831c => { vm.decode(0x54a340f5u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d8320u64) }
        0x102d8320 => { vm.decode(0x505257f5u32); let d = 7i16; let s = 7i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8324u64) }
        0x102d8324 => { vm.decode(0x408d875u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8328u64) }
        0x102d8328 => { vm.decode(0x826204f5u32); let d = 5i16; let s = 2i16; let s2 = 5i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d832cu64) }
        0x102d832c => { vm.decode(0x10001430u32); let d = 1i16; let s = 5i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d8330u64) }
        0x102d8330 => { vm.decode(0x604114f5u32); let d = 1i16; let s = 10i16; let s2 = 17i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8334u64) }
        0x102d8334 => { vm.decode(0x2002012fu32); let d = 5i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d8338u64) }
        0x102d8338 => { vm.decode(0x1000036u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d833cu64) }
        0x102d833c => { vm.decode(0x80122506u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8340u64) }
        0x102d8340 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8344u64).wrapping_add((imm * 4) as u64) } else { 0x102d8344u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8344u64) }
        0x102d8344 => { vm.decode(0x2009019u32); let d = 7i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8348u64) }
        0x102d8348 => { vm.decode(0x49000036u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d834cu64) }
        0x102d834c => { vm.decode(0x32402d75u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8350u64) }
        0x102d8350 => { vm.decode(0xa0204118u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8354u64) }
        0x102d8354 => { vm.decode(0x80122506u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8358u64) }
        0x102d8358 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d835cu64).wrapping_add((imm * 4) as u64) } else { 0x102d835cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d835cu64) }
        0x102d835c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8360u64) } }
        0x102d8360 => { vm.decode(0x1000036u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8364u64) }
        0x102d8364 => { vm.decode(0x80122506u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8368u64) }
        0x102d8368 => { vm.decode(0x6314bc27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -253i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d836cu64).wrapping_add((imm * 4) as u64) } else { 0x102d836cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d836cu64) }
        0x102d836c => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8370u64) }
        0x102d8370 => { vm.decode(0x49000036u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8374u64) }
        0x102d8374 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8378u64) }
        0x102d8378 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d837cu64) }
        0x102d837c => { vm.decode(0x80122506u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8380u64) }
        0x102d8380 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8384u64).wrapping_add((imm * 4) as u64) } else { 0x102d8384u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8384u64) }
        0x102d8384 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8388u64) } }
        0x102d8388 => { vm.decode(0x8820401du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 2248i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d838cu64) }
        0x102d838c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8390u64) } }
        0x102d8390 => { vm.decode(0x131db28eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -13565952i64;
        vm.st(d, imm as u64);
        Ok(0x102d8394u64) }
        0x102d8394 => { vm.decode(0x23110d98u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 46192i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8398u64) }
        0x102d8398 => { vm.decode(0x888632fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d839cu64) }
        0x102d839c => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d83a0u64) }
        0x102d83a0 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d83a4u64) }
        0x102d83a4 => { vm.decode(0x2888432fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 128i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d83a8u64) }
        0x102d83a8 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d83acu64) }
        0x102d83ac => { vm.decode(0x2008012fu32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d83b0u64) }
        0x102d83b0 => { vm.decode(0x2000036u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d83b4u64) }
        0x102d83b4 => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d83b8u64) }
        0x102d83b8 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d83bcu64).wrapping_add((imm * 4) as u64) } else { 0x102d83bcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d83bcu64) }
        0x102d83bc => { vm.decode(0x8e76efd9u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d83c0u64) }
        0x102d83c0 => { vm.decode(0x104141du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 4400i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d83c4u64) }
        0x102d83c8 => { vm.decode(0x4042a835u32); let d = 1i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d83ccu64) }
        0x102d83cc => { vm.decode(0xb26a0d75u32); let d = 9i16; let s = 6i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d83d0u64) }
        0x102d83d0 => { vm.decode(0x20a17f5u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d83d4u64) }
        0x102d83d4 => { vm.decode(0x548140f5u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d83d8u64) }
        0x102d83d8 => { vm.decode(0x505047f5u32); let d = 6i16; let s = 6i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d83dcu64) }
        0x102d83dc => { vm.decode(0x8d875u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d83e0u64) }
        0x102d83e0 => { vm.decode(0xca2204f5u32); let d = 6i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d83e4u64) }
        0x102d83e4 => { vm.decode(0x10401030u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d83e8u64) }
        0x102d83e8 => { vm.decode(0xa400036u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d83ecu64) }
        0x102d83ec => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d83f0u64) }
        0x102d83f0 => { vm.decode(0x140c8aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 251i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d83f4u64).wrapping_add((imm * 4) as u64) } else { 0x102d83f4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d83f4u64) }
        0x102d83f4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d83f8u64) } }
        0x102d83f8 => { vm.decode(0x5208bb5u32); let d = 8i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d83fcu64) }
        0x102d83fc => { vm.decode(0xa1009d75u32); let d = 6i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8400u64) }
        0x102d8400 => { vm.decode(0x626004f5u32); let d = 1i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8404u64) }
        0x102d8404 => { vm.decode(0x200a012fu32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d8408u64) }
        0x102d8408 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d840cu64) }
        0x102d840c => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8410u64) }
        0x102d8410 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8414u64).wrapping_add((imm * 4) as u64) } else { 0x102d8414u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8414u64) }
        0x102d8414 => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8418u64) }
        0x102d8418 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d841cu64) }
        0x102d841c => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8420u64) }
        0x102d8420 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8424u64) }
        0x102d8424 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8428u64) }
        0x102d8428 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d842cu64).wrapping_add((imm * 4) as u64) } else { 0x102d842cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d842cu64) }
        0x102d842c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8430u64) } }
        0x102d8430 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8434u64) }
        0x102d8434 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8438u64) }
        0x102d8438 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d843cu64).wrapping_add((imm * 4) as u64) } else { 0x102d843cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d843cu64) }
        0x102d843c => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8440u64) }
        0x102d8440 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8444u64) }
        0x102d8444 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8448u64) }
        0x102d8448 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d844cu64) }
        0x102d844c => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8450u64) }
        0x102d8450 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8454u64).wrapping_add((imm * 4) as u64) } else { 0x102d8454u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8454u64) }
        0x102d8454 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8458u64) } }
        0x102d8458 => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d845cu64) }
        0x102d845c => { vm.decode(0xb22a3d75u32); let d = 10i16; let s = 7i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8460u64) }
        0x102d8460 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8464u64) }
        0x102d8464 => { vm.decode(0x54a340f5u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d8468u64) }
        0x102d8468 => { vm.decode(0x525257f5u32); let d = 7i16; let s = 7i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d846cu64) }
        0x102d846c => { vm.decode(0x408d875u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8470u64) }
        0x102d8470 => { vm.decode(0x42022236u32); let d = 7i16; let s = 8i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8474u64) }
        0x102d8474 => { vm.decode(0xb2083d75u32); let d = 10i16; let s = 7i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8478u64) }
        0x102d8478 => { vm.decode(0x4a4214f5u32); let d = 7i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d847cu64) }
        0x102d847c => { vm.decode(0x10401430u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d8480u64) }
        0x102d8480 => { vm.decode(0x48002036u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8484u64) }
        0x102d8484 => { vm.decode(0xa040bd75u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8488u64) }
        0x102d8488 => { vm.decode(0xe26004f5u32); let d = 1i16; let s = 3i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d848cu64) }
        0x102d848c => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d8490u64) }
        0x102d8490 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8494u64) }
        0x102d8494 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8498u64) }
        0x102d8498 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d849cu64).wrapping_add((imm * 4) as u64) } else { 0x102d849cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d849cu64) }
        0x102d849c => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d84a0u64) }
        0x102d84a0 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d84a4u64) }
        0x102d84a4 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d84a8u64) }
        0x102d84a8 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d84acu64) }
        0x102d84ac => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d84b0u64) }
        0x102d84b0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d84b4u64).wrapping_add((imm * 4) as u64) } else { 0x102d84b4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d84b4u64) }
        0x102d84b4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d84b8u64) } }
        0x102d84b8 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d84bcu64) }
        0x102d84bc => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d84c0u64) }
        0x102d84c0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d84c4u64).wrapping_add((imm * 4) as u64) } else { 0x102d84c4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d84c4u64) }
        0x102d84c4 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d84c8u64) }
        0x102d84c8 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d84ccu64) }
        0x102d84cc => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d84d0u64) }
        0x102d84d0 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d84d4u64) }
        0x102d84d4 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d84d8u64) }
        0x102d84d8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d84dcu64).wrapping_add((imm * 4) as u64) } else { 0x102d84dcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d84dcu64) }
        0x102d84dc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d84e0u64) } }
        0x102d84e0 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d84e4u64) }
        0x102d84e4 => { vm.decode(0xa12aad75u32); let d = 12i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d84e8u64) }
        0x102d84e8 => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d84ecu64) }
        0x102d84ec => { vm.decode(0x30b200f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d84f0u64) }
        0x102d84f0 => { vm.decode(0x64e17f5u32); let d = 9i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d84f4u64) }
        0x102d84f4 => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d84f8u64) }
        0x102d84f8 => { vm.decode(0x4900a036u32); let d = 9i16; let s = 8i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d84fcu64) }
        0x102d84fc => { vm.decode(0xa1482d75u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8500u64) }
        0x102d8500 => { vm.decode(0x8ac014f5u32); let d = 11i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8504u64) }
        0x102d8504 => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d8508u64) }
        0x102d8508 => { vm.decode(0x624014f5u32); let d = 1i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d850cu64) }
        0x102d850c => { vm.decode(0x8092fu32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d8510u64) }
        0x102d8510 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8514u64) }
        0x102d8514 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8518u64) }
        0x102d8518 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d851cu64).wrapping_add((imm * 4) as u64) } else { 0x102d851cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d851cu64) }
        0x102d851c => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8520u64) }
        0x102d8520 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8524u64) }
        0x102d8524 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8528u64) }
        0x102d8528 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d852cu64) }
        0x102d852c => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8530u64) }
        0x102d8530 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8534u64).wrapping_add((imm * 4) as u64) } else { 0x102d8534u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8534u64) }
        0x102d8534 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8538u64) } }
        0x102d8538 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d853cu64) }
        0x102d853c => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8540u64) }
        0x102d8540 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8544u64).wrapping_add((imm * 4) as u64) } else { 0x102d8544u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8544u64) }
        0x102d8544 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8548u64) }
        0x102d8548 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d854cu64) }
        0x102d854c => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8550u64) }
        0x102d8550 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8554u64) }
        0x102d8554 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8558u64) }
        0x102d8558 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d855cu64).wrapping_add((imm * 4) as u64) } else { 0x102d855cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d855cu64) }
        0x102d855c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8560u64) } }
        0x102d8560 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8564u64) }
        0x102d8564 => { vm.decode(0xa32a8d75u32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8568u64) }
        0x102d8568 => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d856cu64) }
        0x102d856c => { vm.decode(0x349100f5u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d8570u64) }
        0x102d8570 => { vm.decode(0x465c07f5u32); let d = 10i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8574u64) }
        0x102d8574 => { vm.decode(0x289875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8578u64) }
        0x102d8578 => { vm.decode(0x41028236u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d857cu64) }
        0x102d857c => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8580u64) }
        0x102d8580 => { vm.decode(0x4ac014f5u32); let d = 11i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8584u64) }
        0x102d8584 => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d8588u64) }
        0x102d8588 => { vm.decode(0x48022036u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d858cu64) }
        0x102d858c => { vm.decode(0xa008ad75u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8590u64) }
        0x102d8590 => { vm.decode(0x226014f5u32); let d = 1i16; let s = 3i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8594u64) }
        0x102d8594 => { vm.decode(0x2008092fu32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d8598u64) }
        0x102d8598 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d859cu64) }
        0x102d859c => { vm.decode(0x83122506u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d85a0u64) }
        0x102d85a0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d85a4u64).wrapping_add((imm * 4) as u64) } else { 0x102d85a4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d85a4u64) }
        0x102d85a4 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d85a8u64) }
        0x102d85a8 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d85acu64) }
        0x102d85ac => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d85b0u64) }
        0x102d85b0 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d85b4u64) }
        0x102d85b4 => { vm.decode(0x83122506u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d85b8u64) }
        0x102d85b8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d85bcu64).wrapping_add((imm * 4) as u64) } else { 0x102d85bcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d85bcu64) }
        0x102d85bc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d85c0u64) } }
        0x102d85c0 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d85c4u64) }
        0x102d85c4 => { vm.decode(0x83122506u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d85c8u64) }
        0x102d85c8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d85ccu64).wrapping_add((imm * 4) as u64) } else { 0x102d85ccu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d85ccu64) }
        0x102d85cc => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d85d0u64) }
        0x102d85d0 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d85d4u64) }
        0x102d85d4 => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d85d8u64) }
        0x102d85d8 => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d85dcu64) }
        0x102d85dc => { vm.decode(0x83122506u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d85e0u64) }
        0x102d85e0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d85e4u64).wrapping_add((imm * 4) as u64) } else { 0x102d85e4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d85e4u64) }
        0x102d85e4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d85e8u64) } }
        0x102d85e8 => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d85ecu64) }
        0x102d85ec => { vm.decode(0xa32abd75u32); let d = 14i16; let s = 11i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d85f0u64) }
        0x102d85f0 => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d85f4u64) }
        0x102d85f4 => { vm.decode(0x34b300f5u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d85f8u64) }
        0x102d85f8 => { vm.decode(0x425f17f5u32); let d = 11i16; let s = 11i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d85fcu64) }
        0x102d85fc => { vm.decode(0x4289875u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8600u64) }
        0x102d8600 => { vm.decode(0x4902a236u32); let d = 11i16; let s = 8i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8604u64) }
        0x102d8604 => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8608u64) }
        0x102d8608 => { vm.decode(0xc2c214f5u32); let d = 13i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d860cu64) }
        0x102d860c => { vm.decode(0x18001430u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d8610u64) }
        0x102d8610 => { vm.decode(0x4000a036u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8614u64) }
        0x102d8614 => { vm.decode(0xa0082d75u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8618u64) }
        0x102d8618 => { vm.decode(0x224014f5u32); let d = 1i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d861cu64) }
        0x102d861c => { vm.decode(0x2002092fu32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d8620u64) }
        0x102d8620 => { vm.decode(0x3002236u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8624u64) }
        0x102d8624 => { vm.decode(0xa3122506u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8628u64) }
        0x102d8628 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d862cu64).wrapping_add((imm * 4) as u64) } else { 0x102d862cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d862cu64) }
        0x102d862c => { vm.decode(0x3001019u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8630u64) }
        0x102d8630 => { vm.decode(0x4b603236u32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8634u64) }
        0x102d8634 => { vm.decode(0x33400d75u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8638u64) }
        0x102d8638 => { vm.decode(0x8020c118u32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d863cu64) }
        0x102d863c => { vm.decode(0xa3122506u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8640u64) }
        0x102d8640 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8644u64).wrapping_add((imm * 4) as u64) } else { 0x102d8644u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8644u64) }
        0x102d8644 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8648u64) } }
        0x102d8648 => { vm.decode(0x3002236u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d864cu64) }
        0x102d864c => { vm.decode(0xa3122506u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8650u64) }
        0x102d8650 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8654u64).wrapping_add((imm * 4) as u64) } else { 0x102d8654u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8654u64) }
        0x102d8654 => { vm.decode(0x11000019u32); let d = 24i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8658u64) }
        0x102d8658 => { vm.decode(0x4b603236u32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d865cu64) }
        0x102d865c => { vm.decode(0x21404d75u32); let d = 1i16; let s = 24i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8660u64) }
        0x102d8660 => { vm.decode(0x400c118u32); let d = 24i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8664u64) }
        0x102d8664 => { vm.decode(0xa3122506u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8668u64) }
        0x102d8668 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d866cu64).wrapping_add((imm * 4) as u64) } else { 0x102d866cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d866cu64) }
        0x102d866c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8670u64) } }
        0x102d8670 => { vm.decode(0x4012a835u32); let d = 1i16; let s = 5i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8674u64) }
        0x102d8674 => { vm.decode(0xb16abd75u32); let d = 15i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8678u64) }
        0x102d8678 => { vm.decode(0x121a17f5u32); let d = 1i16; let s = 1i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d867cu64) }
        0x102d867c => { vm.decode(0x70b240f5u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d8680u64) }
        0x102d8680 => { vm.decode(0x144f57f5u32); let d = 13i16; let s = 13i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8684u64) }
        0x102d8684 => { vm.decode(0x428c875u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8688u64) }
        0x102d8688 => { vm.decode(0x2a214f5u32); let d = 12i16; let s = 2i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d868cu64) }
        0x102d868c => { vm.decode(0x18001030u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d8690u64) }
        0x102d8690 => { vm.decode(0xa24014f5u32); let d = 1i16; let s = 3i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8694u64) }
        0x102d8694 => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d8698u64) }
        0x102d8698 => { vm.decode(0x3002036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d869cu64) }
        0x102d869c => { vm.decode(0xa1122506u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d86a0u64) }
        0x102d86a0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d86a4u64).wrapping_add((imm * 4) as u64) } else { 0x102d86a4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d86a4u64) }
        0x102d86a4 => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d86a8u64) }
        0x102d86a8 => { vm.decode(0x4b403036u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d86acu64) }
        0x102d86ac => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d86b0u64) }
        0x102d86b0 => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d86b4u64) }
        0x102d86b4 => { vm.decode(0xa1122506u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d86b8u64) }
        0x102d86b8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d86bcu64).wrapping_add((imm * 4) as u64) } else { 0x102d86bcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d86bcu64) }
        0x102d86bc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d86c0u64) } }
        0x102d86c0 => { vm.decode(0x3002036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d86c4u64) }
        0x102d86c4 => { vm.decode(0xa1122506u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d86c8u64) }
        0x102d86c8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d86ccu64).wrapping_add((imm * 4) as u64) } else { 0x102d86ccu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d86ccu64) }
        0x102d86cc => { vm.decode(0x3001019u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d86d0u64) }
        0x102d86d0 => { vm.decode(0x4b403036u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d86d4u64) }
        0x102d86d4 => { vm.decode(0x33400d75u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d86d8u64) }
        0x102d86d8 => { vm.decode(0x8020c118u32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d86dcu64) }
        0x102d86dc => { vm.decode(0xa1122506u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d86e0u64) }
        0x102d86e0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d86e4u64).wrapping_add((imm * 4) as u64) } else { 0x102d86e4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d86e4u64) }
        0x102d86e4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d86e8u64) } }
        0x102d86e8 => { vm.decode(0x4043a835u32); let d = 1i16; let s = 5i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d86ecu64) }
        0x102d86ec => { vm.decode(0xa16aad75u32); let d = 13i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d86f0u64) }
        0x102d86f0 => { vm.decode(0x120a17f5u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d86f4u64) }
        0x102d86f4 => { vm.decode(0x30b200f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d86f8u64) }
        0x102d86f8 => { vm.decode(0x4f17f5u32); let d = 9i16; let s = 9i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d86fcu64) }
        0x102d86fc => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8700u64) }
        0x102d8700 => { vm.decode(0x28014f5u32); let d = 8i16; let s = 2i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8704u64) }
        0x102d8704 => { vm.decode(0x18000030u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d8708u64) }
        0x102d8708 => { vm.decode(0x624014f5u32); let d = 1i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d870cu64) }
        0x102d870c => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d8710u64) }
        0x102d8710 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8714u64) }
        0x102d8714 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8718u64) }
        0x102d8718 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d871cu64).wrapping_add((imm * 4) as u64) } else { 0x102d871cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d871cu64) }
        0x102d871c => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8720u64) }
        0x102d8720 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8724u64) }
        0x102d8724 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8728u64) }
        0x102d8728 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d872cu64) }
        0x102d872c => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8730u64) }
        0x102d8730 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8734u64).wrapping_add((imm * 4) as u64) } else { 0x102d8734u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8734u64) }
        0x102d8734 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8738u64) } }
        0x102d8738 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d873cu64) }
        0x102d873c => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8740u64) }
        0x102d8740 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8744u64).wrapping_add((imm * 4) as u64) } else { 0x102d8744u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8744u64) }
        0x102d8744 => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8748u64) }
        0x102d8748 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d874cu64) }
        0x102d874c => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8750u64) }
        0x102d8750 => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8754u64) }
        0x102d8754 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8758u64) }
        0x102d8758 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d875cu64).wrapping_add((imm * 4) as u64) } else { 0x102d875cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d875cu64) }
        0x102d875c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8760u64) } }
        0x102d8760 => { vm.decode(0x4003a835u32); let d = 1i16; let s = 5i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8764u64) }
        0x102d8764 => { vm.decode(0xa12a1d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8768u64) }
        0x102d8768 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d876cu64) }
        0x102d876c => { vm.decode(0x309000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d8770u64) }
        0x102d8770 => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8774u64) }
        0x102d8774 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8778u64) }
        0x102d8778 => { vm.decode(0x4a2204f5u32); let d = 6i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d877cu64) }
        0x102d877c => { vm.decode(0x10401030u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d8780u64) }
        0x102d8780 => { vm.decode(0xe24014f5u32); let d = 1i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8784u64) }
        0x102d8784 => { vm.decode(0xa012fu32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d8788u64) }
        0x102d8788 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d878cu64) }
        0x102d878c => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8790u64) }
        0x102d8790 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8794u64).wrapping_add((imm * 4) as u64) } else { 0x102d8794u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8794u64) }
        0x102d8794 => { vm.decode(0x1000019u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8798u64) }
        0x102d8798 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d879cu64) }
        0x102d879c => { vm.decode(0x21400d75u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d87a0u64) }
        0x102d87a0 => { vm.decode(0xc118u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d87a4u64) }
        0x102d87a4 => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d87a8u64) }
        0x102d87a8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d87acu64).wrapping_add((imm * 4) as u64) } else { 0x102d87acu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d87acu64) }
        0x102d87ac => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d87b0u64) } }
        0x102d87b0 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d87b4u64) }
        0x102d87b4 => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d87b8u64) }
        0x102d87b8 => { vm.decode(0x6314bc27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -253i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d87bcu64).wrapping_add((imm * 4) as u64) } else { 0x102d87bcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d87bcu64) }
        0x102d87bc => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d87c0u64) }
        0x102d87c0 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d87c4u64) }
        0x102d87c4 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d87c8u64) }
        0x102d87c8 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d87ccu64) }
        0x102d87cc => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d87d0u64) }
        0x102d87d0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d87d4u64).wrapping_add((imm * 4) as u64) } else { 0x102d87d4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d87d4u64) }
        0x102d87d4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d87d8u64) } }
        0x102d87d8 => { vm.decode(0xc00541du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 3352i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d87dcu64) }
        0x102d87dc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d87e0u64) } }
        0x102d87e0 => { vm.decode(0x131db28eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -13565952i64;
        vm.st(d, imm as u64);
        Ok(0x102d87e4u64) }
        0x102d87e4 => { vm.decode(0x23110d98u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 46192i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d87e8u64) }
        0x102d87e8 => { vm.decode(0x8a01a4f5u32); let d = 2i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d87ecu64) }
        0x102d87ec => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d87f0u64) }
        0x102d87f0 => { vm.decode(0x2888632fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 136i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d87f4u64) }
        0x102d87f4 => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d87f8u64) }
        0x102d87f8 => { vm.decode(0x2008012fu32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d87fcu64) }
        0x102d87fc => { vm.decode(0x2000036u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8800u64) }
        0x102d8800 => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8804u64) }
        0x102d8804 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8808u64).wrapping_add((imm * 4) as u64) } else { 0x102d8808u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8808u64) }
        0x102d8808 => { vm.decode(0x8e76efd9u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d880cu64) }
        0x102d880c => { vm.decode(0x2d24141du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 5500i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d8810u64) }
        0x102d8814 => { vm.decode(0x4042a835u32); let d = 1i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8818u64) }
        0x102d8818 => { vm.decode(0xb26a0d75u32); let d = 9i16; let s = 6i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d881cu64) }
        0x102d881c => { vm.decode(0x20a17f5u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8820u64) }
        0x102d8820 => { vm.decode(0x548140f5u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d8824u64) }
        0x102d8824 => { vm.decode(0x505047f5u32); let d = 6i16; let s = 6i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8828u64) }
        0x102d8828 => { vm.decode(0x8d875u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d882cu64) }
        0x102d882c => { vm.decode(0xca2204f5u32); let d = 6i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8830u64) }
        0x102d8830 => { vm.decode(0x10401030u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d8834u64) }
        0x102d8834 => { vm.decode(0xa400036u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8838u64) }
        0x102d8838 => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d883cu64) }
        0x102d883c => { vm.decode(0x140c8aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 251i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8840u64).wrapping_add((imm * 4) as u64) } else { 0x102d8840u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8840u64) }
        0x102d8840 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8844u64) } }
        0x102d8844 => { vm.decode(0x5208bb5u32); let d = 8i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d8848u64) }
        0x102d8848 => { vm.decode(0xa1009d75u32); let d = 6i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d884cu64) }
        0x102d884c => { vm.decode(0x626004f5u32); let d = 1i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8850u64) }
        0x102d8850 => { vm.decode(0x200a012fu32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d8854u64) }
        0x102d8854 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8858u64) }
        0x102d8858 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d885cu64) }
        0x102d885c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8860u64).wrapping_add((imm * 4) as u64) } else { 0x102d8860u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8860u64) }
        0x102d8860 => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8864u64) }
        0x102d8864 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8868u64) }
        0x102d8868 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d886cu64) }
        0x102d886c => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8870u64) }
        0x102d8870 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8874u64) }
        0x102d8874 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8878u64).wrapping_add((imm * 4) as u64) } else { 0x102d8878u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8878u64) }
        0x102d8878 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d887cu64) } }
        0x102d887c => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8880u64) }
        0x102d8880 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8884u64) }
        0x102d8884 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8888u64).wrapping_add((imm * 4) as u64) } else { 0x102d8888u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8888u64) }
        0x102d8888 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d888cu64) }
        0x102d888c => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8890u64) }
        0x102d8890 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8894u64) }
        0x102d8894 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8898u64) }
        0x102d8898 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d889cu64) }
        0x102d889c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d88a0u64).wrapping_add((imm * 4) as u64) } else { 0x102d88a0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d88a0u64) }
        0x102d88a0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d88a4u64) } }
        0x102d88a4 => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d88a8u64) }
        0x102d88a8 => { vm.decode(0xb22a3d75u32); let d = 10i16; let s = 7i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d88acu64) }
        0x102d88ac => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d88b0u64) }
        0x102d88b0 => { vm.decode(0x54a340f5u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d88b4u64) }
        0x102d88b4 => { vm.decode(0x525257f5u32); let d = 7i16; let s = 7i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d88b8u64) }
        0x102d88b8 => { vm.decode(0x408d875u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d88bcu64) }
        0x102d88bc => { vm.decode(0x42022236u32); let d = 7i16; let s = 8i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d88c0u64) }
        0x102d88c0 => { vm.decode(0xb2083d75u32); let d = 10i16; let s = 7i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d88c4u64) }
        0x102d88c4 => { vm.decode(0x4a4214f5u32); let d = 7i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d88c8u64) }
        0x102d88c8 => { vm.decode(0x10401430u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d88ccu64) }
        0x102d88cc => { vm.decode(0x48002036u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d88d0u64) }
        0x102d88d0 => { vm.decode(0xa040bd75u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d88d4u64) }
        0x102d88d4 => { vm.decode(0xe26004f5u32); let d = 1i16; let s = 3i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d88d8u64) }
        0x102d88d8 => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d88dcu64) }
        0x102d88dc => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d88e0u64) }
        0x102d88e0 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d88e4u64) }
        0x102d88e4 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d88e8u64).wrapping_add((imm * 4) as u64) } else { 0x102d88e8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d88e8u64) }
        0x102d88e8 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d88ecu64) }
        0x102d88ec => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d88f0u64) }
        0x102d88f0 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d88f4u64) }
        0x102d88f4 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d88f8u64) }
        0x102d88f8 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d88fcu64) }
        0x102d88fc => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8900u64).wrapping_add((imm * 4) as u64) } else { 0x102d8900u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8900u64) }
        0x102d8900 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8904u64) } }
        0x102d8904 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8908u64) }
        0x102d8908 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d890cu64) }
        0x102d890c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8910u64).wrapping_add((imm * 4) as u64) } else { 0x102d8910u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8910u64) }
        0x102d8910 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8914u64) }
        0x102d8914 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8918u64) }
        0x102d8918 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d891cu64) }
        0x102d891c => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8920u64) }
        0x102d8920 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8924u64) }
        0x102d8924 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8928u64).wrapping_add((imm * 4) as u64) } else { 0x102d8928u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8928u64) }
        0x102d8928 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d892cu64) } }
        0x102d892c => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8930u64) }
        0x102d8930 => { vm.decode(0xa12aad75u32); let d = 12i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8934u64) }
        0x102d8934 => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8938u64) }
        0x102d8938 => { vm.decode(0x30b200f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d893cu64) }
        0x102d893c => { vm.decode(0x64e17f5u32); let d = 9i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8940u64) }
        0x102d8940 => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8944u64) }
        0x102d8944 => { vm.decode(0x4900a036u32); let d = 9i16; let s = 8i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8948u64) }
        0x102d8948 => { vm.decode(0xa1482d75u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d894cu64) }
        0x102d894c => { vm.decode(0x8ac014f5u32); let d = 11i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8950u64) }
        0x102d8950 => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d8954u64) }
        0x102d8954 => { vm.decode(0x624014f5u32); let d = 1i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8958u64) }
        0x102d8958 => { vm.decode(0x8092fu32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d895cu64) }
        0x102d895c => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8960u64) }
        0x102d8960 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8964u64) }
        0x102d8964 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8968u64).wrapping_add((imm * 4) as u64) } else { 0x102d8968u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8968u64) }
        0x102d8968 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d896cu64) }
        0x102d896c => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8970u64) }
        0x102d8970 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8974u64) }
        0x102d8974 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8978u64) }
        0x102d8978 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d897cu64) }
        0x102d897c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8980u64).wrapping_add((imm * 4) as u64) } else { 0x102d8980u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8980u64) }
        0x102d8980 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8984u64) } }
        0x102d8984 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8988u64) }
        0x102d8988 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d898cu64) }
        0x102d898c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8990u64).wrapping_add((imm * 4) as u64) } else { 0x102d8990u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8990u64) }
        0x102d8990 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8994u64) }
        0x102d8994 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8998u64) }
        0x102d8998 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d899cu64) }
        0x102d899c => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d89a0u64) }
        0x102d89a0 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d89a4u64) }
        0x102d89a4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d89a8u64).wrapping_add((imm * 4) as u64) } else { 0x102d89a8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d89a8u64) }
        0x102d89a8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d89acu64) } }
        0x102d89ac => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d89b0u64) }
        0x102d89b0 => { vm.decode(0xa32a8d75u32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d89b4u64) }
        0x102d89b4 => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d89b8u64) }
        0x102d89b8 => { vm.decode(0x349100f5u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d89bcu64) }
        0x102d89bc => { vm.decode(0x465c07f5u32); let d = 10i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d89c0u64) }
        0x102d89c0 => { vm.decode(0x289875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d89c4u64) }
        0x102d89c4 => { vm.decode(0x41028236u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d89c8u64) }
        0x102d89c8 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d89ccu64) }
        0x102d89cc => { vm.decode(0x4ac014f5u32); let d = 11i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d89d0u64) }
        0x102d89d0 => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d89d4u64) }
        0x102d89d4 => { vm.decode(0x48022036u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d89d8u64) }
        0x102d89d8 => { vm.decode(0xa008ad75u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d89dcu64) }
        0x102d89dc => { vm.decode(0x226014f5u32); let d = 1i16; let s = 3i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d89e0u64) }
        0x102d89e0 => { vm.decode(0x2008092fu32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d89e4u64) }
        0x102d89e4 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d89e8u64) }
        0x102d89e8 => { vm.decode(0x83122506u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d89ecu64) }
        0x102d89ec => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d89f0u64).wrapping_add((imm * 4) as u64) } else { 0x102d89f0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d89f0u64) }
        0x102d89f0 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d89f4u64) }
        0x102d89f4 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d89f8u64) }
        0x102d89f8 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d89fcu64) }
        0x102d89fc => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8a00u64) }
        0x102d8a00 => { vm.decode(0x83122506u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8a04u64) }
        0x102d8a04 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8a08u64).wrapping_add((imm * 4) as u64) } else { 0x102d8a08u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8a08u64) }
        0x102d8a08 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8a0cu64) } }
        0x102d8a0c => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8a10u64) }
        0x102d8a10 => { vm.decode(0x83122506u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8a14u64) }
        0x102d8a14 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8a18u64).wrapping_add((imm * 4) as u64) } else { 0x102d8a18u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8a18u64) }
        0x102d8a18 => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8a1cu64) }
        0x102d8a1c => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8a20u64) }
        0x102d8a20 => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8a24u64) }
        0x102d8a24 => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8a28u64) }
        0x102d8a28 => { vm.decode(0x83122506u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8a2cu64) }
        0x102d8a2c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8a30u64).wrapping_add((imm * 4) as u64) } else { 0x102d8a30u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8a30u64) }
        0x102d8a30 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8a34u64) } }
        0x102d8a34 => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8a38u64) }
        0x102d8a38 => { vm.decode(0xa32abd75u32); let d = 14i16; let s = 11i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8a3cu64) }
        0x102d8a3c => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8a40u64) }
        0x102d8a40 => { vm.decode(0x34b300f5u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d8a44u64) }
        0x102d8a44 => { vm.decode(0x425f17f5u32); let d = 11i16; let s = 11i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8a48u64) }
        0x102d8a48 => { vm.decode(0x4289875u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8a4cu64) }
        0x102d8a4c => { vm.decode(0x4902a236u32); let d = 11i16; let s = 8i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8a50u64) }
        0x102d8a50 => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8a54u64) }
        0x102d8a54 => { vm.decode(0xc2c214f5u32); let d = 13i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8a58u64) }
        0x102d8a58 => { vm.decode(0x18001430u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d8a5cu64) }
        0x102d8a5c => { vm.decode(0x4000a036u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8a60u64) }
        0x102d8a60 => { vm.decode(0xa0082d75u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8a64u64) }
        0x102d8a64 => { vm.decode(0x224014f5u32); let d = 1i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8a68u64) }
        0x102d8a68 => { vm.decode(0x2002092fu32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d8a6cu64) }
        0x102d8a6c => { vm.decode(0x3002236u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8a70u64) }
        0x102d8a70 => { vm.decode(0xa3122506u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8a74u64) }
        0x102d8a74 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8a78u64).wrapping_add((imm * 4) as u64) } else { 0x102d8a78u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8a78u64) }
        0x102d8a78 => { vm.decode(0x3001019u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8a7cu64) }
        0x102d8a7c => { vm.decode(0x4b603236u32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8a80u64) }
        0x102d8a80 => { vm.decode(0x33400d75u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8a84u64) }
        0x102d8a84 => { vm.decode(0x8020c118u32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8a88u64) }
        0x102d8a88 => { vm.decode(0xa3122506u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8a8cu64) }
        0x102d8a8c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8a90u64).wrapping_add((imm * 4) as u64) } else { 0x102d8a90u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8a90u64) }
        0x102d8a90 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8a94u64) } }
        0x102d8a94 => { vm.decode(0x3002236u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8a98u64) }
        0x102d8a98 => { vm.decode(0xa3122506u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8a9cu64) }
        0x102d8a9c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8aa0u64).wrapping_add((imm * 4) as u64) } else { 0x102d8aa0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8aa0u64) }
        0x102d8aa0 => { vm.decode(0x11000019u32); let d = 24i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8aa4u64) }
        0x102d8aa4 => { vm.decode(0x4b603236u32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8aa8u64) }
        0x102d8aa8 => { vm.decode(0x21404d75u32); let d = 1i16; let s = 24i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8aacu64) }
        0x102d8aac => { vm.decode(0x400c118u32); let d = 24i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8ab0u64) }
        0x102d8ab0 => { vm.decode(0xa3122506u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8ab4u64) }
        0x102d8ab4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8ab8u64).wrapping_add((imm * 4) as u64) } else { 0x102d8ab8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8ab8u64) }
        0x102d8ab8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8abcu64) } }
        0x102d8abc => { vm.decode(0x4012a835u32); let d = 1i16; let s = 5i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8ac0u64) }
        0x102d8ac0 => { vm.decode(0xb16abd75u32); let d = 15i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8ac4u64) }
        0x102d8ac4 => { vm.decode(0x121a17f5u32); let d = 1i16; let s = 1i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8ac8u64) }
        0x102d8ac8 => { vm.decode(0x70b240f5u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d8accu64) }
        0x102d8acc => { vm.decode(0x144f57f5u32); let d = 13i16; let s = 13i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8ad0u64) }
        0x102d8ad0 => { vm.decode(0x428c875u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8ad4u64) }
        0x102d8ad4 => { vm.decode(0x2a214f5u32); let d = 12i16; let s = 2i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8ad8u64) }
        0x102d8ad8 => { vm.decode(0x18001030u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d8adcu64) }
        0x102d8adc => { vm.decode(0xa24014f5u32); let d = 1i16; let s = 3i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8ae0u64) }
        0x102d8ae0 => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d8ae4u64) }
        0x102d8ae4 => { vm.decode(0x3002036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8ae8u64) }
        0x102d8ae8 => { vm.decode(0xa1122506u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8aecu64) }
        0x102d8aec => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8af0u64).wrapping_add((imm * 4) as u64) } else { 0x102d8af0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8af0u64) }
        0x102d8af0 => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8af4u64) }
        0x102d8af4 => { vm.decode(0x4b403036u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8af8u64) }
        0x102d8af8 => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8afcu64) }
        0x102d8afc => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8b00u64) }
        0x102d8b00 => { vm.decode(0xa1122506u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8b04u64) }
        0x102d8b04 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8b08u64).wrapping_add((imm * 4) as u64) } else { 0x102d8b08u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8b08u64) }
        0x102d8b08 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8b0cu64) } }
        0x102d8b0c => { vm.decode(0x3002036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8b10u64) }
        0x102d8b10 => { vm.decode(0xa1122506u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8b14u64) }
        0x102d8b14 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8b18u64).wrapping_add((imm * 4) as u64) } else { 0x102d8b18u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8b18u64) }
        0x102d8b18 => { vm.decode(0x3001019u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8b1cu64) }
        0x102d8b1c => { vm.decode(0x4b403036u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8b20u64) }
        0x102d8b20 => { vm.decode(0x33400d75u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8b24u64) }
        0x102d8b24 => { vm.decode(0x8020c118u32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8b28u64) }
        0x102d8b28 => { vm.decode(0xa1122506u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8b2cu64) }
        0x102d8b2c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8b30u64).wrapping_add((imm * 4) as u64) } else { 0x102d8b30u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8b30u64) }
        0x102d8b30 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8b34u64) } }
        0x102d8b34 => { vm.decode(0x4043a835u32); let d = 1i16; let s = 5i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8b38u64) }
        0x102d8b38 => { vm.decode(0xa16aad75u32); let d = 13i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8b3cu64) }
        0x102d8b3c => { vm.decode(0x120a17f5u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8b40u64) }
        0x102d8b40 => { vm.decode(0x30b200f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d8b44u64) }
        0x102d8b44 => { vm.decode(0x4f17f5u32); let d = 9i16; let s = 9i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8b48u64) }
        0x102d8b48 => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8b4cu64) }
        0x102d8b4c => { vm.decode(0x28014f5u32); let d = 8i16; let s = 2i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8b50u64) }
        0x102d8b50 => { vm.decode(0x18000030u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d8b54u64) }
        0x102d8b54 => { vm.decode(0x624014f5u32); let d = 1i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8b58u64) }
        0x102d8b58 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d8b5cu64) }
        0x102d8b5c => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8b60u64) }
        0x102d8b60 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8b64u64) }
        0x102d8b64 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8b68u64).wrapping_add((imm * 4) as u64) } else { 0x102d8b68u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8b68u64) }
        0x102d8b68 => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8b6cu64) }
        0x102d8b6c => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8b70u64) }
        0x102d8b70 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8b74u64) }
        0x102d8b74 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8b78u64) }
        0x102d8b78 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8b7cu64) }
        0x102d8b7c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8b80u64).wrapping_add((imm * 4) as u64) } else { 0x102d8b80u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8b80u64) }
        0x102d8b80 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8b84u64) } }
        0x102d8b84 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8b88u64) }
        0x102d8b88 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8b8cu64) }
        0x102d8b8c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8b90u64).wrapping_add((imm * 4) as u64) } else { 0x102d8b90u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8b90u64) }
        0x102d8b90 => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8b94u64) }
        0x102d8b94 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8b98u64) }
        0x102d8b98 => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8b9cu64) }
        0x102d8b9c => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8ba0u64) }
        0x102d8ba0 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8ba4u64) }
        0x102d8ba4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8ba8u64).wrapping_add((imm * 4) as u64) } else { 0x102d8ba8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8ba8u64) }
        0x102d8ba8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8bacu64) } }
        0x102d8bac => { vm.decode(0x4003a835u32); let d = 1i16; let s = 5i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8bb0u64) }
        0x102d8bb0 => { vm.decode(0xa12a1d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8bb4u64) }
        0x102d8bb4 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8bb8u64) }
        0x102d8bb8 => { vm.decode(0x309000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d8bbcu64) }
        0x102d8bbc => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8bc0u64) }
        0x102d8bc0 => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8bc4u64) }
        0x102d8bc4 => { vm.decode(0x4a2204f5u32); let d = 6i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8bc8u64) }
        0x102d8bc8 => { vm.decode(0x10401030u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d8bccu64) }
        0x102d8bcc => { vm.decode(0xe24014f5u32); let d = 1i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8bd0u64) }
        0x102d8bd0 => { vm.decode(0xa012fu32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d8bd4u64) }
        0x102d8bd4 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8bd8u64) }
        0x102d8bd8 => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8bdcu64) }
        0x102d8bdc => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8be0u64).wrapping_add((imm * 4) as u64) } else { 0x102d8be0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8be0u64) }
        0x102d8be0 => { vm.decode(0x1000019u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8be4u64) }
        0x102d8be4 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8be8u64) }
        0x102d8be8 => { vm.decode(0x21400d75u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8becu64) }
        0x102d8bec => { vm.decode(0xc118u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8bf0u64) }
        0x102d8bf0 => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8bf4u64) }
        0x102d8bf4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8bf8u64).wrapping_add((imm * 4) as u64) } else { 0x102d8bf8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8bf8u64) }
        0x102d8bf8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8bfcu64) } }
        0x102d8bfc => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8c00u64) }
        0x102d8c00 => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8c04u64) }
        0x102d8c04 => { vm.decode(0x6314bc27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -253i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8c08u64).wrapping_add((imm * 4) as u64) } else { 0x102d8c08u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8c08u64) }
        0x102d8c08 => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8c0cu64) }
        0x102d8c0c => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8c10u64) }
        0x102d8c10 => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8c14u64) }
        0x102d8c14 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8c18u64) }
        0x102d8c18 => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8c1cu64) }
        0x102d8c1c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8c20u64).wrapping_add((imm * 4) as u64) } else { 0x102d8c20u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8c20u64) }
        0x102d8c20 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8c24u64) } }
        0x102d8c24 => { vm.decode(0x2124101du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 4452i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d8c28u64) }
        0x102d8c28 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8c2cu64) } }
        0x102d8c2c => { vm.decode(0x131db28eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -13565952i64;
        vm.st(d, imm as u64);
        Ok(0x102d8c30u64) }
        0x102d8c30 => { vm.decode(0x23110d98u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 46192i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8c34u64) }
        0x102d8c34 => { vm.decode(0xaa01a4f5u32); let d = 2i16; let s = 1i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8c38u64) }
        0x102d8c38 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d8c3cu64) }
        0x102d8c3c => { vm.decode(0x8241a4f5u32); let d = 1i16; let s = 1i16; let s2 = 30i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8c40u64) }
        0x102d8c40 => { vm.decode(0x2008012fu32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d8c44u64) }
        0x102d8c44 => { vm.decode(0x2000036u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8c48u64) }
        0x102d8c48 => { vm.decode(0x408a632fu32); let d = 22i16; let s = 29i16; let s2 = -1i16; let imm = 8i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d8c4cu64) }
        0x102d8c4c => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8c50u64) }
        0x102d8c50 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8c54u64).wrapping_add((imm * 4) as u64) } else { 0x102d8c54u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8c54u64) }
        0x102d8c54 => { vm.decode(0x8e76efd9u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8c58u64) }
        0x102d8c58 => { vm.decode(0x8824501du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 6600i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d8c5cu64) }
        0x102d8c60 => { vm.decode(0x4042a835u32); let d = 1i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8c64u64) }
        0x102d8c64 => { vm.decode(0xb26a0d75u32); let d = 9i16; let s = 6i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8c68u64) }
        0x102d8c68 => { vm.decode(0x20a17f5u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8c6cu64) }
        0x102d8c6c => { vm.decode(0x548140f5u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d8c70u64) }
        0x102d8c70 => { vm.decode(0x505047f5u32); let d = 6i16; let s = 6i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8c74u64) }
        0x102d8c74 => { vm.decode(0x8d875u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8c78u64) }
        0x102d8c78 => { vm.decode(0xca2204f5u32); let d = 6i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8c7cu64) }
        0x102d8c7c => { vm.decode(0x10401030u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d8c80u64) }
        0x102d8c80 => { vm.decode(0xa400036u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8c84u64) }
        0x102d8c84 => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8c88u64) }
        0x102d8c88 => { vm.decode(0x140c8aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 251i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8c8cu64).wrapping_add((imm * 4) as u64) } else { 0x102d8c8cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8c8cu64) }
        0x102d8c8c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8c90u64) } }
        0x102d8c90 => { vm.decode(0x5208bb5u32); let d = 8i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d8c94u64) }
        0x102d8c94 => { vm.decode(0xa1009d75u32); let d = 6i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8c98u64) }
        0x102d8c98 => { vm.decode(0x626004f5u32); let d = 1i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8c9cu64) }
        0x102d8c9c => { vm.decode(0x200a012fu32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d8ca0u64) }
        0x102d8ca0 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8ca4u64) }
        0x102d8ca4 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8ca8u64) }
        0x102d8ca8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8cacu64).wrapping_add((imm * 4) as u64) } else { 0x102d8cacu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8cacu64) }
        0x102d8cac => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8cb0u64) }
        0x102d8cb0 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8cb4u64) }
        0x102d8cb4 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8cb8u64) }
        0x102d8cb8 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8cbcu64) }
        0x102d8cbc => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8cc0u64) }
        0x102d8cc0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8cc4u64).wrapping_add((imm * 4) as u64) } else { 0x102d8cc4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8cc4u64) }
        0x102d8cc4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8cc8u64) } }
        0x102d8cc8 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8cccu64) }
        0x102d8ccc => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8cd0u64) }
        0x102d8cd0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8cd4u64).wrapping_add((imm * 4) as u64) } else { 0x102d8cd4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8cd4u64) }
        0x102d8cd4 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8cd8u64) }
        0x102d8cd8 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8cdcu64) }
        0x102d8cdc => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8ce0u64) }
        0x102d8ce0 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8ce4u64) }
        0x102d8ce4 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8ce8u64) }
        0x102d8ce8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8cecu64).wrapping_add((imm * 4) as u64) } else { 0x102d8cecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8cecu64) }
        0x102d8cec => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8cf0u64) } }
        0x102d8cf0 => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8cf4u64) }
        0x102d8cf4 => { vm.decode(0xb22a3d75u32); let d = 10i16; let s = 7i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8cf8u64) }
        0x102d8cf8 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8cfcu64) }
        0x102d8cfc => { vm.decode(0x54a340f5u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d8d00u64) }
        0x102d8d00 => { vm.decode(0x525257f5u32); let d = 7i16; let s = 7i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8d04u64) }
        0x102d8d04 => { vm.decode(0x408d875u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8d08u64) }
        0x102d8d08 => { vm.decode(0x42022236u32); let d = 7i16; let s = 8i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8d0cu64) }
        0x102d8d0c => { vm.decode(0xb2083d75u32); let d = 10i16; let s = 7i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8d10u64) }
        0x102d8d10 => { vm.decode(0x4a4214f5u32); let d = 7i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8d14u64) }
        0x102d8d14 => { vm.decode(0x10401430u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d8d18u64) }
        0x102d8d18 => { vm.decode(0x48002036u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8d1cu64) }
        0x102d8d1c => { vm.decode(0xa040bd75u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8d20u64) }
        0x102d8d20 => { vm.decode(0xe26004f5u32); let d = 1i16; let s = 3i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8d24u64) }
        0x102d8d24 => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d8d28u64) }
        0x102d8d28 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8d2cu64) }
        0x102d8d2c => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8d30u64) }
        0x102d8d30 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8d34u64).wrapping_add((imm * 4) as u64) } else { 0x102d8d34u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8d34u64) }
        0x102d8d34 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8d38u64) }
        0x102d8d38 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8d3cu64) }
        0x102d8d3c => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8d40u64) }
        0x102d8d40 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8d44u64) }
        0x102d8d44 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8d48u64) }
        0x102d8d48 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8d4cu64).wrapping_add((imm * 4) as u64) } else { 0x102d8d4cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8d4cu64) }
        0x102d8d4c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8d50u64) } }
        0x102d8d50 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8d54u64) }
        0x102d8d54 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8d58u64) }
        0x102d8d58 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8d5cu64).wrapping_add((imm * 4) as u64) } else { 0x102d8d5cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8d5cu64) }
        0x102d8d5c => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8d60u64) }
        0x102d8d60 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8d64u64) }
        0x102d8d64 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8d68u64) }
        0x102d8d68 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8d6cu64) }
        0x102d8d6c => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8d70u64) }
        0x102d8d70 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8d74u64).wrapping_add((imm * 4) as u64) } else { 0x102d8d74u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8d74u64) }
        0x102d8d74 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8d78u64) } }
        0x102d8d78 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8d7cu64) }
        0x102d8d7c => { vm.decode(0xa12aad75u32); let d = 12i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8d80u64) }
        0x102d8d80 => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8d84u64) }
        0x102d8d84 => { vm.decode(0x30b200f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d8d88u64) }
        0x102d8d88 => { vm.decode(0x64e17f5u32); let d = 9i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8d8cu64) }
        0x102d8d8c => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8d90u64) }
        0x102d8d90 => { vm.decode(0x4900a036u32); let d = 9i16; let s = 8i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8d94u64) }
        0x102d8d94 => { vm.decode(0xa1482d75u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8d98u64) }
        0x102d8d98 => { vm.decode(0x8ac014f5u32); let d = 11i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8d9cu64) }
        0x102d8d9c => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d8da0u64) }
        0x102d8da0 => { vm.decode(0x624014f5u32); let d = 1i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8da4u64) }
        0x102d8da4 => { vm.decode(0x8092fu32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d8da8u64) }
        0x102d8da8 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8dacu64) }
        0x102d8dac => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8db0u64) }
        0x102d8db0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8db4u64).wrapping_add((imm * 4) as u64) } else { 0x102d8db4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8db4u64) }
        0x102d8db4 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8db8u64) }
        0x102d8db8 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8dbcu64) }
        0x102d8dbc => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8dc0u64) }
        0x102d8dc0 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8dc4u64) }
        0x102d8dc4 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8dc8u64) }
        0x102d8dc8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8dccu64).wrapping_add((imm * 4) as u64) } else { 0x102d8dccu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8dccu64) }
        0x102d8dcc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8dd0u64) } }
        0x102d8dd0 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8dd4u64) }
        0x102d8dd4 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8dd8u64) }
        0x102d8dd8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8ddcu64).wrapping_add((imm * 4) as u64) } else { 0x102d8ddcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8ddcu64) }
        0x102d8ddc => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8de0u64) }
        0x102d8de0 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8de4u64) }
        0x102d8de4 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8de8u64) }
        0x102d8de8 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8decu64) }
        0x102d8dec => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8df0u64) }
        0x102d8df0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8df4u64).wrapping_add((imm * 4) as u64) } else { 0x102d8df4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8df4u64) }
        0x102d8df4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8df8u64) } }
        0x102d8df8 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8dfcu64) }
        0x102d8dfc => { vm.decode(0xa32a8d75u32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8e00u64) }
        0x102d8e00 => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8e04u64) }
        0x102d8e04 => { vm.decode(0x349100f5u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d8e08u64) }
        0x102d8e08 => { vm.decode(0x465c07f5u32); let d = 10i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8e0cu64) }
        0x102d8e0c => { vm.decode(0x289875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8e10u64) }
        0x102d8e10 => { vm.decode(0x41028236u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8e14u64) }
        0x102d8e14 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8e18u64) }
        0x102d8e18 => { vm.decode(0x4ac014f5u32); let d = 11i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8e1cu64) }
        0x102d8e1c => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d8e20u64) }
        0x102d8e20 => { vm.decode(0x48022036u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8e24u64) }
        0x102d8e24 => { vm.decode(0xa008ad75u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8e28u64) }
        0x102d8e28 => { vm.decode(0x226014f5u32); let d = 1i16; let s = 3i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8e2cu64) }
        0x102d8e2c => { vm.decode(0x2008092fu32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d8e30u64) }
        0x102d8e30 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8e34u64) }
        0x102d8e34 => { vm.decode(0x83122506u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8e38u64) }
        0x102d8e38 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8e3cu64).wrapping_add((imm * 4) as u64) } else { 0x102d8e3cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8e3cu64) }
        0x102d8e3c => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8e40u64) }
        0x102d8e40 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8e44u64) }
        0x102d8e44 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8e48u64) }
        0x102d8e48 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8e4cu64) }
        0x102d8e4c => { vm.decode(0x83122506u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8e50u64) }
        0x102d8e50 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8e54u64).wrapping_add((imm * 4) as u64) } else { 0x102d8e54u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8e54u64) }
        0x102d8e54 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8e58u64) } }
        0x102d8e58 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8e5cu64) }
        0x102d8e5c => { vm.decode(0x83122506u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8e60u64) }
        0x102d8e60 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8e64u64).wrapping_add((imm * 4) as u64) } else { 0x102d8e64u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8e64u64) }
        0x102d8e64 => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8e68u64) }
        0x102d8e68 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8e6cu64) }
        0x102d8e6c => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8e70u64) }
        0x102d8e70 => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8e74u64) }
        0x102d8e74 => { vm.decode(0x83122506u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8e78u64) }
        0x102d8e78 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8e7cu64).wrapping_add((imm * 4) as u64) } else { 0x102d8e7cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8e7cu64) }
        0x102d8e7c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8e80u64) } }
        0x102d8e80 => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8e84u64) }
        0x102d8e84 => { vm.decode(0xa32abd75u32); let d = 14i16; let s = 11i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8e88u64) }
        0x102d8e88 => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8e8cu64) }
        0x102d8e8c => { vm.decode(0x34b300f5u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d8e90u64) }
        0x102d8e90 => { vm.decode(0x425f17f5u32); let d = 11i16; let s = 11i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8e94u64) }
        0x102d8e94 => { vm.decode(0x4289875u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8e98u64) }
        0x102d8e98 => { vm.decode(0x4902a236u32); let d = 11i16; let s = 8i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8e9cu64) }
        0x102d8e9c => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8ea0u64) }
        0x102d8ea0 => { vm.decode(0xc2c214f5u32); let d = 13i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8ea4u64) }
        0x102d8ea4 => { vm.decode(0x18001430u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d8ea8u64) }
        0x102d8ea8 => { vm.decode(0x4000a036u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8eacu64) }
        0x102d8eac => { vm.decode(0xa0082d75u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8eb0u64) }
        0x102d8eb0 => { vm.decode(0x224014f5u32); let d = 1i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8eb4u64) }
        0x102d8eb4 => { vm.decode(0x2002092fu32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d8eb8u64) }
        0x102d8eb8 => { vm.decode(0x3002236u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8ebcu64) }
        0x102d8ebc => { vm.decode(0xa3122506u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8ec0u64) }
        0x102d8ec0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8ec4u64).wrapping_add((imm * 4) as u64) } else { 0x102d8ec4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8ec4u64) }
        0x102d8ec4 => { vm.decode(0x3001019u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8ec8u64) }
        0x102d8ec8 => { vm.decode(0x4b603236u32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8eccu64) }
        0x102d8ecc => { vm.decode(0x33400d75u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8ed0u64) }
        0x102d8ed0 => { vm.decode(0x8020c118u32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8ed4u64) }
        0x102d8ed4 => { vm.decode(0xa3122506u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8ed8u64) }
        0x102d8ed8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8edcu64).wrapping_add((imm * 4) as u64) } else { 0x102d8edcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8edcu64) }
        0x102d8edc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8ee0u64) } }
        0x102d8ee0 => { vm.decode(0x3002236u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8ee4u64) }
        0x102d8ee4 => { vm.decode(0xa3122506u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8ee8u64) }
        0x102d8ee8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8eecu64).wrapping_add((imm * 4) as u64) } else { 0x102d8eecu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8eecu64) }
        0x102d8eec => { vm.decode(0x11000019u32); let d = 24i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8ef0u64) }
        0x102d8ef0 => { vm.decode(0x4b603236u32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8ef4u64) }
        0x102d8ef4 => { vm.decode(0x21404d75u32); let d = 1i16; let s = 24i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8ef8u64) }
        0x102d8ef8 => { vm.decode(0x400c118u32); let d = 24i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8efcu64) }
        0x102d8efc => { vm.decode(0xa3122506u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8f00u64) }
        0x102d8f00 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8f04u64).wrapping_add((imm * 4) as u64) } else { 0x102d8f04u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8f04u64) }
        0x102d8f04 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8f08u64) } }
        0x102d8f08 => { vm.decode(0x4012a835u32); let d = 1i16; let s = 5i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8f0cu64) }
        0x102d8f0c => { vm.decode(0xb16abd75u32); let d = 15i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8f10u64) }
        0x102d8f10 => { vm.decode(0x121a17f5u32); let d = 1i16; let s = 1i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8f14u64) }
        0x102d8f14 => { vm.decode(0x70b240f5u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d8f18u64) }
        0x102d8f18 => { vm.decode(0x144f57f5u32); let d = 13i16; let s = 13i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8f1cu64) }
        0x102d8f1c => { vm.decode(0x428c875u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8f20u64) }
        0x102d8f20 => { vm.decode(0x2a214f5u32); let d = 12i16; let s = 2i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8f24u64) }
        0x102d8f24 => { vm.decode(0x18001030u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d8f28u64) }
        0x102d8f28 => { vm.decode(0xa24014f5u32); let d = 1i16; let s = 3i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8f2cu64) }
        0x102d8f2c => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d8f30u64) }
        0x102d8f30 => { vm.decode(0x3002036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8f34u64) }
        0x102d8f34 => { vm.decode(0xa1122506u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8f38u64) }
        0x102d8f38 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8f3cu64).wrapping_add((imm * 4) as u64) } else { 0x102d8f3cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8f3cu64) }
        0x102d8f3c => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8f40u64) }
        0x102d8f40 => { vm.decode(0x4b403036u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8f44u64) }
        0x102d8f44 => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8f48u64) }
        0x102d8f48 => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8f4cu64) }
        0x102d8f4c => { vm.decode(0xa1122506u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8f50u64) }
        0x102d8f50 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8f54u64).wrapping_add((imm * 4) as u64) } else { 0x102d8f54u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8f54u64) }
        0x102d8f54 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8f58u64) } }
        0x102d8f58 => { vm.decode(0x3002036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8f5cu64) }
        0x102d8f5c => { vm.decode(0xa1122506u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8f60u64) }
        0x102d8f60 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8f64u64).wrapping_add((imm * 4) as u64) } else { 0x102d8f64u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8f64u64) }
        0x102d8f64 => { vm.decode(0x3001019u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8f68u64) }
        0x102d8f68 => { vm.decode(0x4b403036u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8f6cu64) }
        0x102d8f6c => { vm.decode(0x33400d75u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8f70u64) }
        0x102d8f70 => { vm.decode(0x8020c118u32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8f74u64) }
        0x102d8f74 => { vm.decode(0xa1122506u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8f78u64) }
        0x102d8f78 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8f7cu64).wrapping_add((imm * 4) as u64) } else { 0x102d8f7cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8f7cu64) }
        0x102d8f7c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8f80u64) } }
        0x102d8f80 => { vm.decode(0x4043a835u32); let d = 1i16; let s = 5i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8f84u64) }
        0x102d8f84 => { vm.decode(0xa16aad75u32); let d = 13i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8f88u64) }
        0x102d8f88 => { vm.decode(0x120a17f5u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8f8cu64) }
        0x102d8f8c => { vm.decode(0x30b200f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d8f90u64) }
        0x102d8f90 => { vm.decode(0x4f17f5u32); let d = 9i16; let s = 9i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8f94u64) }
        0x102d8f94 => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8f98u64) }
        0x102d8f98 => { vm.decode(0x28014f5u32); let d = 8i16; let s = 2i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8f9cu64) }
        0x102d8f9c => { vm.decode(0x18000030u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d8fa0u64) }
        0x102d8fa0 => { vm.decode(0x624014f5u32); let d = 1i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d8fa4u64) }
        0x102d8fa4 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d8fa8u64) }
        0x102d8fa8 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8facu64) }
        0x102d8fac => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8fb0u64) }
        0x102d8fb0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8fb4u64).wrapping_add((imm * 4) as u64) } else { 0x102d8fb4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8fb4u64) }
        0x102d8fb4 => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8fb8u64) }
        0x102d8fb8 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8fbcu64) }
        0x102d8fbc => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8fc0u64) }
        0x102d8fc0 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8fc4u64) }
        0x102d8fc4 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8fc8u64) }
        0x102d8fc8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8fccu64).wrapping_add((imm * 4) as u64) } else { 0x102d8fccu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8fccu64) }
        0x102d8fcc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8fd0u64) } }
        0x102d8fd0 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8fd4u64) }
        0x102d8fd4 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8fd8u64) }
        0x102d8fd8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d8fdcu64).wrapping_add((imm * 4) as u64) } else { 0x102d8fdcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8fdcu64) }
        0x102d8fdc => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d8fe0u64) }
        0x102d8fe0 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d8fe4u64) }
        0x102d8fe4 => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d8fe8u64) }
        0x102d8fe8 => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d8fecu64) }
        0x102d8fec => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d8ff0u64) }
        0x102d8ff0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d8ff4u64).wrapping_add((imm * 4) as u64) } else { 0x102d8ff4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d8ff4u64) }
        0x102d8ff4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d8ff8u64) } }
        0x102d8ff8 => { vm.decode(0x4003a835u32); let d = 1i16; let s = 5i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d8ffcu64) }
        0x102d8ffc => { vm.decode(0xa12a1d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9000u64) }
        0x102d9000 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9004u64) }
        0x102d9004 => { vm.decode(0x309000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d9008u64) }
        0x102d9008 => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d900cu64) }
        0x102d900c => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9010u64) }
        0x102d9010 => { vm.decode(0x4a2204f5u32); let d = 6i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9014u64) }
        0x102d9014 => { vm.decode(0x10401030u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9018u64) }
        0x102d9018 => { vm.decode(0xe24014f5u32); let d = 1i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d901cu64) }
        0x102d901c => { vm.decode(0xa012fu32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9020u64) }
        0x102d9020 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9024u64) }
        0x102d9024 => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9028u64) }
        0x102d9028 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d902cu64).wrapping_add((imm * 4) as u64) } else { 0x102d902cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d902cu64) }
        0x102d902c => { vm.decode(0x1000019u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9030u64) }
        0x102d9030 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9034u64) }
        0x102d9034 => { vm.decode(0x21400d75u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9038u64) }
        0x102d9038 => { vm.decode(0xc118u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d903cu64) }
        0x102d903c => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9040u64) }
        0x102d9040 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9044u64).wrapping_add((imm * 4) as u64) } else { 0x102d9044u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9044u64) }
        0x102d9044 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9048u64) } }
        0x102d9048 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d904cu64) }
        0x102d904c => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9050u64) }
        0x102d9050 => { vm.decode(0x6314bc27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -253i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9054u64).wrapping_add((imm * 4) as u64) } else { 0x102d9054u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9054u64) }
        0x102d9054 => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9058u64) }
        0x102d9058 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d905cu64) }
        0x102d905c => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9060u64) }
        0x102d9060 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9064u64) }
        0x102d9064 => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9068u64) }
        0x102d9068 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d906cu64).wrapping_add((imm * 4) as u64) } else { 0x102d906cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d906cu64) }
        0x102d906c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9070u64) } }
        0x102d9070 => { vm.decode(0x8504141du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 5552i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d9074u64) }
        0x102d9074 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9078u64) } }
        0x102d9078 => { vm.decode(0x131db28eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -13565952i64;
        vm.st(d, imm as u64);
        Ok(0x102d907cu64) }
        0x102d907c => { vm.decode(0x23110d98u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 46192i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9080u64) }
        0x102d9080 => { vm.decode(0xa88432fu32); let d = 2i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9084u64) }
        0x102d9084 => { vm.decode(0x8a0004f5u32); let d = 2i16; let s = 1i16; let s2 = 2i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9088u64) }
        0x102d9088 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d908cu64) }
        0x102d908c => { vm.decode(0xa241a4f5u32); let d = 1i16; let s = 1i16; let s2 = 31i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9090u64) }
        0x102d9090 => { vm.decode(0x2008012fu32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9094u64) }
        0x102d9094 => { vm.decode(0x2000036u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9098u64) }
        0x102d9098 => { vm.decode(0x62804b6fu32); let d = 25i16; let s = 29i16; let s2 = -1i16; let imm = 80i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d909cu64) }
        0x102d909c => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d90a0u64) }
        0x102d90a0 => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d90a4u64).wrapping_add((imm * 4) as u64) } else { 0x102d90a4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d90a4u64) }
        0x102d90a4 => { vm.decode(0x8e76efd9u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d90a8u64) }
        0x102d90a8 => { vm.decode(0xc84441du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 7704i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d90acu64) }
        0x102d90b0 => { vm.decode(0x4042a835u32); let d = 1i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d90b4u64) }
        0x102d90b4 => { vm.decode(0xb26a0d75u32); let d = 9i16; let s = 6i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d90b8u64) }
        0x102d90b8 => { vm.decode(0x20a17f5u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d90bcu64) }
        0x102d90bc => { vm.decode(0x548140f5u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d90c0u64) }
        0x102d90c0 => { vm.decode(0x505047f5u32); let d = 6i16; let s = 6i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d90c4u64) }
        0x102d90c4 => { vm.decode(0x8d875u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d90c8u64) }
        0x102d90c8 => { vm.decode(0xca2204f5u32); let d = 6i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d90ccu64) }
        0x102d90cc => { vm.decode(0x10401030u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d90d0u64) }
        0x102d90d0 => { vm.decode(0xa400036u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d90d4u64) }
        0x102d90d4 => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d90d8u64) }
        0x102d90d8 => { vm.decode(0x140c8aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 251i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d90dcu64).wrapping_add((imm * 4) as u64) } else { 0x102d90dcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d90dcu64) }
        0x102d90dc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d90e0u64) } }
        0x102d90e0 => { vm.decode(0x5208bb5u32); let d = 8i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d90e4u64) }
        0x102d90e4 => { vm.decode(0xa1009d75u32); let d = 6i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d90e8u64) }
        0x102d90e8 => { vm.decode(0x626004f5u32); let d = 1i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d90ecu64) }
        0x102d90ec => { vm.decode(0x200a012fu32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d90f0u64) }
        0x102d90f0 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d90f4u64) }
        0x102d90f4 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d90f8u64) }
        0x102d90f8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d90fcu64).wrapping_add((imm * 4) as u64) } else { 0x102d90fcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d90fcu64) }
        0x102d90fc => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9100u64) }
        0x102d9100 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9104u64) }
        0x102d9104 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9108u64) }
        0x102d9108 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d910cu64) }
        0x102d910c => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9110u64) }
        0x102d9110 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9114u64).wrapping_add((imm * 4) as u64) } else { 0x102d9114u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9114u64) }
        0x102d9114 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9118u64) } }
        0x102d9118 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d911cu64) }
        0x102d911c => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9120u64) }
        0x102d9120 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9124u64).wrapping_add((imm * 4) as u64) } else { 0x102d9124u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9124u64) }
        0x102d9124 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9128u64) }
        0x102d9128 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d912cu64) }
        0x102d912c => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9130u64) }
        0x102d9130 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9134u64) }
        0x102d9134 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9138u64) }
        0x102d9138 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d913cu64).wrapping_add((imm * 4) as u64) } else { 0x102d913cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d913cu64) }
        0x102d913c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9140u64) } }
        0x102d9140 => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9144u64) }
        0x102d9144 => { vm.decode(0xb22a3d75u32); let d = 10i16; let s = 7i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9148u64) }
        0x102d9148 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d914cu64) }
        0x102d914c => { vm.decode(0x54a340f5u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d9150u64) }
        0x102d9150 => { vm.decode(0x525257f5u32); let d = 7i16; let s = 7i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9154u64) }
        0x102d9154 => { vm.decode(0x408d875u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9158u64) }
        0x102d9158 => { vm.decode(0x42022236u32); let d = 7i16; let s = 8i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d915cu64) }
        0x102d915c => { vm.decode(0xb2083d75u32); let d = 10i16; let s = 7i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9160u64) }
        0x102d9160 => { vm.decode(0x4a4214f5u32); let d = 7i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9164u64) }
        0x102d9164 => { vm.decode(0x10401430u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9168u64) }
        0x102d9168 => { vm.decode(0x48002036u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d916cu64) }
        0x102d916c => { vm.decode(0xa040bd75u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9170u64) }
        0x102d9170 => { vm.decode(0xe26004f5u32); let d = 1i16; let s = 3i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9174u64) }
        0x102d9174 => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9178u64) }
        0x102d9178 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d917cu64) }
        0x102d917c => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9180u64) }
        0x102d9180 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9184u64).wrapping_add((imm * 4) as u64) } else { 0x102d9184u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9184u64) }
        0x102d9184 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9188u64) }
        0x102d9188 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d918cu64) }
        0x102d918c => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9190u64) }
        0x102d9190 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9194u64) }
        0x102d9194 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9198u64) }
        0x102d9198 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d919cu64).wrapping_add((imm * 4) as u64) } else { 0x102d919cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d919cu64) }
        0x102d919c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d91a0u64) } }
        0x102d91a0 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d91a4u64) }
        0x102d91a4 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d91a8u64) }
        0x102d91a8 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d91acu64).wrapping_add((imm * 4) as u64) } else { 0x102d91acu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d91acu64) }
        0x102d91ac => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d91b0u64) }
        0x102d91b0 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d91b4u64) }
        0x102d91b4 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d91b8u64) }
        0x102d91b8 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d91bcu64) }
        0x102d91bc => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d91c0u64) }
        0x102d91c0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d91c4u64).wrapping_add((imm * 4) as u64) } else { 0x102d91c4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d91c4u64) }
        0x102d91c4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d91c8u64) } }
        0x102d91c8 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d91ccu64) }
        0x102d91cc => { vm.decode(0xa12aad75u32); let d = 12i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d91d0u64) }
        0x102d91d0 => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d91d4u64) }
        0x102d91d4 => { vm.decode(0x30b200f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d91d8u64) }
        0x102d91d8 => { vm.decode(0x64e17f5u32); let d = 9i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d91dcu64) }
        0x102d91dc => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d91e0u64) }
        0x102d91e0 => { vm.decode(0x4900a036u32); let d = 9i16; let s = 8i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d91e4u64) }
        0x102d91e4 => { vm.decode(0xa1482d75u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d91e8u64) }
        0x102d91e8 => { vm.decode(0x8ac014f5u32); let d = 11i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d91ecu64) }
        0x102d91ec => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d91f0u64) }
        0x102d91f0 => { vm.decode(0x624014f5u32); let d = 1i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d91f4u64) }
        0x102d91f4 => { vm.decode(0x8092fu32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d91f8u64) }
        0x102d91f8 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d91fcu64) }
        0x102d91fc => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9200u64) }
        0x102d9200 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9204u64).wrapping_add((imm * 4) as u64) } else { 0x102d9204u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9204u64) }
        0x102d9204 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9208u64) }
        0x102d9208 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d920cu64) }
        0x102d920c => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9210u64) }
        0x102d9210 => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9214u64) }
        0x102d9214 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9218u64) }
        0x102d9218 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d921cu64).wrapping_add((imm * 4) as u64) } else { 0x102d921cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d921cu64) }
        0x102d921c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9220u64) } }
        0x102d9220 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9224u64) }
        0x102d9224 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9228u64) }
        0x102d9228 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d922cu64).wrapping_add((imm * 4) as u64) } else { 0x102d922cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d922cu64) }
        0x102d922c => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9230u64) }
        0x102d9230 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9234u64) }
        0x102d9234 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9238u64) }
        0x102d9238 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d923cu64) }
        0x102d923c => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9240u64) }
        0x102d9240 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9244u64).wrapping_add((imm * 4) as u64) } else { 0x102d9244u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9244u64) }
        0x102d9244 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9248u64) } }
        0x102d9248 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d924cu64) }
        0x102d924c => { vm.decode(0xa32a8d75u32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9250u64) }
        0x102d9250 => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9254u64) }
        0x102d9254 => { vm.decode(0x349100f5u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d9258u64) }
        0x102d9258 => { vm.decode(0x465c07f5u32); let d = 10i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d925cu64) }
        0x102d925c => { vm.decode(0x289875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9260u64) }
        0x102d9260 => { vm.decode(0x41028236u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9264u64) }
        0x102d9264 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9268u64) }
        0x102d9268 => { vm.decode(0x4ac014f5u32); let d = 11i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d926cu64) }
        0x102d926c => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9270u64) }
        0x102d9270 => { vm.decode(0x48022036u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9274u64) }
        0x102d9274 => { vm.decode(0xa008ad75u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9278u64) }
        0x102d9278 => { vm.decode(0x226014f5u32); let d = 1i16; let s = 3i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d927cu64) }
        0x102d927c => { vm.decode(0x2008092fu32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9280u64) }
        0x102d9280 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9284u64) }
        0x102d9284 => { vm.decode(0x83122506u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9288u64) }
        0x102d9288 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d928cu64).wrapping_add((imm * 4) as u64) } else { 0x102d928cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d928cu64) }
        0x102d928c => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9290u64) }
        0x102d9290 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9294u64) }
        0x102d9294 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9298u64) }
        0x102d9298 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d929cu64) }
        0x102d929c => { vm.decode(0x83122506u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d92a0u64) }
        0x102d92a0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d92a4u64).wrapping_add((imm * 4) as u64) } else { 0x102d92a4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d92a4u64) }
        0x102d92a4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d92a8u64) } }
        0x102d92a8 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d92acu64) }
        0x102d92ac => { vm.decode(0x83122506u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d92b0u64) }
        0x102d92b0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d92b4u64).wrapping_add((imm * 4) as u64) } else { 0x102d92b4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d92b4u64) }
        0x102d92b4 => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d92b8u64) }
        0x102d92b8 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d92bcu64) }
        0x102d92bc => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d92c0u64) }
        0x102d92c0 => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d92c4u64) }
        0x102d92c4 => { vm.decode(0x83122506u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d92c8u64) }
        0x102d92c8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d92ccu64).wrapping_add((imm * 4) as u64) } else { 0x102d92ccu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d92ccu64) }
        0x102d92cc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d92d0u64) } }
        0x102d92d0 => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d92d4u64) }
        0x102d92d4 => { vm.decode(0xa32abd75u32); let d = 14i16; let s = 11i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d92d8u64) }
        0x102d92d8 => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d92dcu64) }
        0x102d92dc => { vm.decode(0x34b300f5u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d92e0u64) }
        0x102d92e0 => { vm.decode(0x425f17f5u32); let d = 11i16; let s = 11i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d92e4u64) }
        0x102d92e4 => { vm.decode(0x4289875u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d92e8u64) }
        0x102d92e8 => { vm.decode(0x4902a236u32); let d = 11i16; let s = 8i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d92ecu64) }
        0x102d92ec => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d92f0u64) }
        0x102d92f0 => { vm.decode(0xc2c214f5u32); let d = 13i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d92f4u64) }
        0x102d92f4 => { vm.decode(0x18001430u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d92f8u64) }
        0x102d92f8 => { vm.decode(0x4000a036u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d92fcu64) }
        0x102d92fc => { vm.decode(0xa0082d75u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9300u64) }
        0x102d9300 => { vm.decode(0x224014f5u32); let d = 1i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9304u64) }
        0x102d9304 => { vm.decode(0x2002092fu32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9308u64) }
        0x102d9308 => { vm.decode(0x3002236u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d930cu64) }
        0x102d930c => { vm.decode(0xa3122506u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9310u64) }
        0x102d9310 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9314u64).wrapping_add((imm * 4) as u64) } else { 0x102d9314u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9314u64) }
        0x102d9314 => { vm.decode(0x3001019u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9318u64) }
        0x102d9318 => { vm.decode(0x4b603236u32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d931cu64) }
        0x102d931c => { vm.decode(0x33400d75u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9320u64) }
        0x102d9320 => { vm.decode(0x8020c118u32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9324u64) }
        0x102d9324 => { vm.decode(0xa3122506u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9328u64) }
        0x102d9328 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d932cu64).wrapping_add((imm * 4) as u64) } else { 0x102d932cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d932cu64) }
        0x102d932c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9330u64) } }
        0x102d9330 => { vm.decode(0x3002236u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9334u64) }
        0x102d9334 => { vm.decode(0xa3122506u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9338u64) }
        0x102d9338 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d933cu64).wrapping_add((imm * 4) as u64) } else { 0x102d933cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d933cu64) }
        0x102d933c => { vm.decode(0x11000019u32); let d = 24i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9340u64) }
        0x102d9340 => { vm.decode(0x4b603236u32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9344u64) }
        0x102d9344 => { vm.decode(0x21404d75u32); let d = 1i16; let s = 24i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9348u64) }
        0x102d9348 => { vm.decode(0x400c118u32); let d = 24i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d934cu64) }
        0x102d934c => { vm.decode(0xa3122506u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9350u64) }
        0x102d9350 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9354u64).wrapping_add((imm * 4) as u64) } else { 0x102d9354u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9354u64) }
        0x102d9354 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9358u64) } }
        0x102d9358 => { vm.decode(0x4012a835u32); let d = 1i16; let s = 5i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d935cu64) }
        0x102d935c => { vm.decode(0xb16abd75u32); let d = 15i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9360u64) }
        0x102d9360 => { vm.decode(0x121a17f5u32); let d = 1i16; let s = 1i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9364u64) }
        0x102d9364 => { vm.decode(0x70b240f5u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d9368u64) }
        0x102d9368 => { vm.decode(0x144f57f5u32); let d = 13i16; let s = 13i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d936cu64) }
        0x102d936c => { vm.decode(0x428c875u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9370u64) }
        0x102d9370 => { vm.decode(0x2a214f5u32); let d = 12i16; let s = 2i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9374u64) }
        0x102d9374 => { vm.decode(0x18001030u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9378u64) }
        0x102d9378 => { vm.decode(0xa24014f5u32); let d = 1i16; let s = 3i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d937cu64) }
        0x102d937c => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9380u64) }
        0x102d9380 => { vm.decode(0x3002036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9384u64) }
        0x102d9384 => { vm.decode(0xa1122506u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9388u64) }
        0x102d9388 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d938cu64).wrapping_add((imm * 4) as u64) } else { 0x102d938cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d938cu64) }
        0x102d938c => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9390u64) }
        0x102d9390 => { vm.decode(0x4b403036u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9394u64) }
        0x102d9394 => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9398u64) }
        0x102d9398 => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d939cu64) }
        0x102d939c => { vm.decode(0xa1122506u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d93a0u64) }
        0x102d93a0 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d93a4u64).wrapping_add((imm * 4) as u64) } else { 0x102d93a4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d93a4u64) }
        0x102d93a4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d93a8u64) } }
        0x102d93a8 => { vm.decode(0x3002036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d93acu64) }
        0x102d93ac => { vm.decode(0xa1122506u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d93b0u64) }
        0x102d93b0 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d93b4u64).wrapping_add((imm * 4) as u64) } else { 0x102d93b4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d93b4u64) }
        0x102d93b4 => { vm.decode(0x3001019u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d93b8u64) }
        0x102d93b8 => { vm.decode(0x4b403036u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d93bcu64) }
        0x102d93bc => { vm.decode(0x33400d75u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d93c0u64) }
        0x102d93c0 => { vm.decode(0x8020c118u32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d93c4u64) }
        0x102d93c4 => { vm.decode(0xa1122506u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d93c8u64) }
        0x102d93c8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d93ccu64).wrapping_add((imm * 4) as u64) } else { 0x102d93ccu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d93ccu64) }
        0x102d93cc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d93d0u64) } }
        0x102d93d0 => { vm.decode(0x4043a835u32); let d = 1i16; let s = 5i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d93d4u64) }
        0x102d93d4 => { vm.decode(0xa16aad75u32); let d = 13i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d93d8u64) }
        0x102d93d8 => { vm.decode(0x120a17f5u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d93dcu64) }
        0x102d93dc => { vm.decode(0x30b200f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d93e0u64) }
        0x102d93e0 => { vm.decode(0x4f17f5u32); let d = 9i16; let s = 9i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d93e4u64) }
        0x102d93e4 => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d93e8u64) }
        0x102d93e8 => { vm.decode(0x28014f5u32); let d = 8i16; let s = 2i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d93ecu64) }
        0x102d93ec => { vm.decode(0x18000030u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d93f0u64) }
        0x102d93f0 => { vm.decode(0x624014f5u32); let d = 1i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d93f4u64) }
        0x102d93f4 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d93f8u64) }
        0x102d93f8 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d93fcu64) }
        0x102d93fc => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9400u64) }
        0x102d9400 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9404u64).wrapping_add((imm * 4) as u64) } else { 0x102d9404u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9404u64) }
        0x102d9404 => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9408u64) }
        0x102d9408 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d940cu64) }
        0x102d940c => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9410u64) }
        0x102d9410 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9414u64) }
        0x102d9414 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9418u64) }
        0x102d9418 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d941cu64).wrapping_add((imm * 4) as u64) } else { 0x102d941cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d941cu64) }
        0x102d941c => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9420u64) } }
        0x102d9420 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9424u64) }
        0x102d9424 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9428u64) }
        0x102d9428 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d942cu64).wrapping_add((imm * 4) as u64) } else { 0x102d942cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d942cu64) }
        0x102d942c => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9430u64) }
        0x102d9430 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9434u64) }
        0x102d9434 => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9438u64) }
        0x102d9438 => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d943cu64) }
        0x102d943c => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9440u64) }
        0x102d9440 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9444u64).wrapping_add((imm * 4) as u64) } else { 0x102d9444u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9444u64) }
        0x102d9444 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9448u64) } }
        0x102d9448 => { vm.decode(0x4003a835u32); let d = 1i16; let s = 5i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d944cu64) }
        0x102d944c => { vm.decode(0xa12a1d75u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9450u64) }
        0x102d9450 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9454u64) }
        0x102d9454 => { vm.decode(0x309000f5u32); let d = 8i16; let s = 8i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d9458u64) }
        0x102d9458 => { vm.decode(0x24c07f5u32); let d = 8i16; let s = 8i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d945cu64) }
        0x102d945c => { vm.decode(0x288875u32); let d = 1i16; let s = 1i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9460u64) }
        0x102d9460 => { vm.decode(0x4a2204f5u32); let d = 6i16; let s = 2i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9464u64) }
        0x102d9464 => { vm.decode(0x10401030u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9468u64) }
        0x102d9468 => { vm.decode(0xe24014f5u32); let d = 1i16; let s = 3i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d946cu64) }
        0x102d946c => { vm.decode(0xa012fu32); let d = 6i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9470u64) }
        0x102d9470 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9474u64) }
        0x102d9474 => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9478u64) }
        0x102d9478 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d947cu64).wrapping_add((imm * 4) as u64) } else { 0x102d947cu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d947cu64) }
        0x102d947c => { vm.decode(0x1000019u32); let d = 8i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9480u64) }
        0x102d9480 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9484u64) }
        0x102d9484 => { vm.decode(0x21400d75u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9488u64) }
        0x102d9488 => { vm.decode(0xc118u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d948cu64) }
        0x102d948c => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9490u64) }
        0x102d9490 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9494u64).wrapping_add((imm * 4) as u64) } else { 0x102d9494u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9494u64) }
        0x102d9494 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9498u64) } }
        0x102d9498 => { vm.decode(0x1002036u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d949cu64) }
        0x102d949c => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d94a0u64) }
        0x102d94a0 => { vm.decode(0x6314bc27u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -253i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d94a4u64).wrapping_add((imm * 4) as u64) } else { 0x102d94a4u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d94a4u64) }
        0x102d94a4 => { vm.decode(0x1001019u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d94a8u64) }
        0x102d94a8 => { vm.decode(0x49003036u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d94acu64) }
        0x102d94ac => { vm.decode(0x23400d75u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d94b0u64) }
        0x102d94b0 => { vm.decode(0x8000c118u32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d94b4u64) }
        0x102d94b4 => { vm.decode(0xa0122506u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d94b8u64) }
        0x102d94b8 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d94bcu64).wrapping_add((imm * 4) as u64) } else { 0x102d94bcu64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d94bcu64) }
        0x102d94bc => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d94c0u64) } }
        0x102d94c0 => { vm.decode(0x84401du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 6656i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d94c4u64) }
        0x102d94c4 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d94c8u64) } }
        0x102d94c8 => { vm.decode(0x131db28eu32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = -13565952i64;
        vm.st(d, imm as u64);
        Ok(0x102d94ccu64) }
        0x102d94cc => { vm.decode(0x23110d98u32); let d = 1i16; let s = 1i16; let s2 = -1i16; let imm = 46192i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d94d0u64) }
        0x102d94d0 => { vm.decode(0xa80184f5u32); let d = 2i16; let s = 1i16; let s2 = 25i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d94d4u64) }
        0x102d94d4 => { vm.decode(0x28002fu32); let d = 2i16; let s = 2i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d94d8u64) }
        0x102d94d8 => { vm.decode(0x2a88432fu32); let d = 3i16; let s = 29i16; let s2 = -1i16; let imm = 144i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d94dcu64) }
        0x102d94dc => { vm.decode(0xa24004f5u32); let d = 1i16; let s = 1i16; let s2 = 3i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d94e0u64) }
        0x102d94e0 => { vm.decode(0x2008012fu32); let d = 3i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d94e4u64) }
        0x102d94e4 => { vm.decode(0x2000036u32); let d = 4i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d94e8u64) }
        0x102d94e8 => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d94ecu64) }
        0x102d94ec => { vm.decode(0x82022071u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 15i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d94f0u64).wrapping_add((imm * 4) as u64) } else { 0x102d94f0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d94f0u64) }
        0x102d94f0 => { vm.decode(0x8e76efd9u32); let d = 5i16; let s = 0i16; let s2 = -1i16; let imm = -1i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d94f4u64) }
        0x102d94f4 => { vm.decode(0x21a0011du32); let d = -1i16; let s = 38i16; let s2 = -1i16; let imm = 8804i64;
        vm.jump_flag = 1;
        vm.wr(36, vm.ld(s).wrapping_add(imm as u64));
        vm.wr(35, 2);
        Ok(0x102d94f8u64) }
        0x102d94fc => { vm.decode(0x4042a835u32); let d = 1i16; let s = 5i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9500u64) }
        0x102d9500 => { vm.decode(0xb26a0d75u32); let d = 9i16; let s = 6i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9504u64) }
        0x102d9504 => { vm.decode(0x20a17f5u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9508u64) }
        0x102d9508 => { vm.decode(0x548140f5u32); let d = 6i16; let s = 6i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d950cu64) }
        0x102d950c => { vm.decode(0x505047f5u32); let d = 6i16; let s = 6i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9510u64) }
        0x102d9510 => { vm.decode(0x8d875u32); let d = 1i16; let s = 1i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9514u64) }
        0x102d9514 => { vm.decode(0xca2204f5u32); let d = 6i16; let s = 2i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9518u64) }
        0x102d9518 => { vm.decode(0x10401030u32); let d = 1i16; let s = 6i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d951cu64) }
        0x102d951c => { vm.decode(0xa400036u32); let d = 4i16; let s = 4i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9520u64) }
        0x102d9520 => { vm.decode(0x1800406u32); let d = 1i16; let s = 4i16; let s2 = -1i16; let imm = 32i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9524u64) }
        0x102d9524 => { vm.decode(0x140c8aa7u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 251i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9528u64).wrapping_add((imm * 4) as u64) } else { 0x102d9528u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9528u64) }
        0x102d9528 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d952cu64) } }
        0x102d952c => { vm.decode(0x5208bb5u32); let d = 8i16; let s = 4i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as u32).wrapping_shl(imm as u32) as i32 as i64;
        vm.st(d, v as u64);
        Ok(0x102d9530u64) }
        0x102d9530 => { vm.decode(0xa1009d75u32); let d = 6i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9534u64) }
        0x102d9534 => { vm.decode(0x626004f5u32); let d = 1i16; let s = 3i16; let s2 = 6i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9538u64) }
        0x102d9538 => { vm.decode(0x200a012fu32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d953cu64) }
        0x102d953c => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9540u64) }
        0x102d9540 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9544u64) }
        0x102d9544 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9548u64).wrapping_add((imm * 4) as u64) } else { 0x102d9548u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9548u64) }
        0x102d9548 => { vm.decode(0x1008019u32); let d = 9i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d954cu64) }
        0x102d954c => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9550u64) }
        0x102d9550 => { vm.decode(0x21402d75u32); let d = 1i16; let s = 9i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9554u64) }
        0x102d9554 => { vm.decode(0x2000c118u32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9558u64) }
        0x102d9558 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d955cu64) }
        0x102d955c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9560u64).wrapping_add((imm * 4) as u64) } else { 0x102d9560u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9560u64) }
        0x102d9560 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9564u64) } }
        0x102d9564 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9568u64) }
        0x102d9568 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d956cu64) }
        0x102d956c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9570u64).wrapping_add((imm * 4) as u64) } else { 0x102d9570u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9570u64) }
        0x102d9570 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9574u64) }
        0x102d9574 => { vm.decode(0x49200236u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9578u64) }
        0x102d9578 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d957cu64) }
        0x102d957c => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9580u64) }
        0x102d9580 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9584u64) }
        0x102d9584 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9588u64).wrapping_add((imm * 4) as u64) } else { 0x102d9588u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9588u64) }
        0x102d9588 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d958cu64) } }
        0x102d958c => { vm.decode(0x6042a835u32); let d = 1i16; let s = 5i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9590u64) }
        0x102d9590 => { vm.decode(0xb22a3d75u32); let d = 10i16; let s = 7i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9594u64) }
        0x102d9594 => { vm.decode(0x21a07f5u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9598u64) }
        0x102d9598 => { vm.decode(0x54a340f5u32); let d = 7i16; let s = 7i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d959cu64) }
        0x102d959c => { vm.decode(0x525257f5u32); let d = 7i16; let s = 7i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d95a0u64) }
        0x102d95a0 => { vm.decode(0x408d875u32); let d = 1i16; let s = 1i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d95a4u64) }
        0x102d95a4 => { vm.decode(0x42022236u32); let d = 7i16; let s = 8i16; let s2 = -1i16; let imm = 2i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d95a8u64) }
        0x102d95a8 => { vm.decode(0xb2083d75u32); let d = 10i16; let s = 7i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d95acu64) }
        0x102d95ac => { vm.decode(0x4a4214f5u32); let d = 7i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d95b0u64) }
        0x102d95b0 => { vm.decode(0x10401430u32); let d = 1i16; let s = 7i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d95b4u64) }
        0x102d95b4 => { vm.decode(0x48002036u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d95b8u64) }
        0x102d95b8 => { vm.decode(0xa040bd75u32); let d = 7i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d95bcu64) }
        0x102d95bc => { vm.decode(0xe26004f5u32); let d = 1i16; let s = 3i16; let s2 = 7i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d95c0u64) }
        0x102d95c0 => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d95c4u64) }
        0x102d95c4 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d95c8u64) }
        0x102d95c8 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d95ccu64) }
        0x102d95cc => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d95d0u64).wrapping_add((imm * 4) as u64) } else { 0x102d95d0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d95d0u64) }
        0x102d95d0 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d95d4u64) }
        0x102d95d4 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d95d8u64) }
        0x102d95d8 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d95dcu64) }
        0x102d95dc => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d95e0u64) }
        0x102d95e0 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d95e4u64) }
        0x102d95e4 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d95e8u64).wrapping_add((imm * 4) as u64) } else { 0x102d95e8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d95e8u64) }
        0x102d95e8 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d95ecu64) } }
        0x102d95ec => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d95f0u64) }
        0x102d95f0 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d95f4u64) }
        0x102d95f4 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d95f8u64).wrapping_add((imm * 4) as u64) } else { 0x102d95f8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d95f8u64) }
        0x102d95f8 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d95fcu64) }
        0x102d95fc => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9600u64) }
        0x102d9600 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9604u64) }
        0x102d9604 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9608u64) }
        0x102d9608 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d960cu64) }
        0x102d960c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9610u64).wrapping_add((imm * 4) as u64) } else { 0x102d9610u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9610u64) }
        0x102d9610 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9614u64) } }
        0x102d9614 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9618u64) }
        0x102d9618 => { vm.decode(0xa12aad75u32); let d = 12i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d961cu64) }
        0x102d961c => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9620u64) }
        0x102d9620 => { vm.decode(0x30b200f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d9624u64) }
        0x102d9624 => { vm.decode(0x64e17f5u32); let d = 9i16; let s = 9i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9628u64) }
        0x102d9628 => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d962cu64) }
        0x102d962c => { vm.decode(0x4900a036u32); let d = 9i16; let s = 8i16; let s2 = -1i16; let imm = 5i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9630u64) }
        0x102d9630 => { vm.decode(0xa1482d75u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9634u64) }
        0x102d9634 => { vm.decode(0x8ac014f5u32); let d = 11i16; let s = 2i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9638u64) }
        0x102d9638 => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d963cu64) }
        0x102d963c => { vm.decode(0x624014f5u32); let d = 1i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9640u64) }
        0x102d9640 => { vm.decode(0x8092fu32); let d = 10i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9644u64) }
        0x102d9644 => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9648u64) }
        0x102d9648 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d964cu64) }
        0x102d964c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9650u64).wrapping_add((imm * 4) as u64) } else { 0x102d9650u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9650u64) }
        0x102d9650 => { vm.decode(0x1009019u32); let d = 11i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9654u64) }
        0x102d9654 => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9658u64) }
        0x102d9658 => { vm.decode(0x23402d75u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d965cu64) }
        0x102d965c => { vm.decode(0xa000c118u32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9660u64) }
        0x102d9660 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9664u64) }
        0x102d9664 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9668u64).wrapping_add((imm * 4) as u64) } else { 0x102d9668u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9668u64) }
        0x102d9668 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d966cu64) } }
        0x102d966c => { vm.decode(0x3000036u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9670u64) }
        0x102d9670 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9674u64) }
        0x102d9674 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9678u64).wrapping_add((imm * 4) as u64) } else { 0x102d9678u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9678u64) }
        0x102d9678 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d967cu64) }
        0x102d967c => { vm.decode(0x4b400036u32); let d = 12i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9680u64) }
        0x102d9680 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9684u64) }
        0x102d9684 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9688u64) }
        0x102d9688 => { vm.decode(0x81122506u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d968cu64) }
        0x102d968c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9690u64).wrapping_add((imm * 4) as u64) } else { 0x102d9690u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9690u64) }
        0x102d9690 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d9694u64) } }
        0x102d9694 => { vm.decode(0x6003a835u32); let d = 1i16; let s = 5i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9698u64) }
        0x102d9698 => { vm.decode(0xa32a8d75u32); let d = 12i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d969cu64) }
        0x102d969c => { vm.decode(0x120a07f5u32); let d = 1i16; let s = 1i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d96a0u64) }
        0x102d96a0 => { vm.decode(0x349100f5u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d96a4u64) }
        0x102d96a4 => { vm.decode(0x465c07f5u32); let d = 10i16; let s = 10i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d96a8u64) }
        0x102d96a8 => { vm.decode(0x289875u32); let d = 1i16; let s = 1i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d96acu64) }
        0x102d96ac => { vm.decode(0x41028236u32); let d = 10i16; let s = 8i16; let s2 = -1i16; let imm = 6i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d96b0u64) }
        0x102d96b0 => { vm.decode(0xa3081d75u32); let d = 10i16; let s = 10i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d96b4u64) }
        0x102d96b4 => { vm.decode(0x4ac014f5u32); let d = 11i16; let s = 2i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d96b8u64) }
        0x102d96b8 => { vm.decode(0x18400430u32); let d = 1i16; let s = 11i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d96bcu64) }
        0x102d96bc => { vm.decode(0x48022036u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 3i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d96c0u64) }
        0x102d96c0 => { vm.decode(0xa008ad75u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d96c4u64) }
        0x102d96c4 => { vm.decode(0x226014f5u32); let d = 1i16; let s = 3i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d96c8u64) }
        0x102d96c8 => { vm.decode(0x2008092fu32); let d = 11i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d96ccu64) }
        0x102d96cc => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d96d0u64) }
        0x102d96d0 => { vm.decode(0x83122506u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d96d4u64) }
        0x102d96d4 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d96d8u64).wrapping_add((imm * 4) as u64) } else { 0x102d96d8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d96d8u64) }
        0x102d96d8 => { vm.decode(0x3008019u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d96dcu64) }
        0x102d96dc => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d96e0u64) }
        0x102d96e0 => { vm.decode(0x31402d75u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d96e4u64) }
        0x102d96e4 => { vm.decode(0x2020c118u32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d96e8u64) }
        0x102d96e8 => { vm.decode(0x83122506u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d96ecu64) }
        0x102d96ec => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d96f0u64).wrapping_add((imm * 4) as u64) } else { 0x102d96f0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d96f0u64) }
        0x102d96f0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d96f4u64) } }
        0x102d96f4 => { vm.decode(0x3000236u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d96f8u64) }
        0x102d96f8 => { vm.decode(0x83122506u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d96fcu64) }
        0x102d96fc => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9700u64).wrapping_add((imm * 4) as u64) } else { 0x102d9700u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9700u64) }
        0x102d9700 => { vm.decode(0x3009019u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9704u64) }
        0x102d9704 => { vm.decode(0x4b600236u32); let d = 14i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9708u64) }
        0x102d9708 => { vm.decode(0x33402d75u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d970cu64) }
        0x102d970c => { vm.decode(0xa020c118u32); let d = 15i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9710u64) }
        0x102d9710 => { vm.decode(0x83122506u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9714u64) }
        0x102d9714 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9718u64).wrapping_add((imm * 4) as u64) } else { 0x102d9718u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9718u64) }
        0x102d9718 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d971cu64) } }
        0x102d971c => { vm.decode(0x6043a835u32); let d = 1i16; let s = 5i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9720u64) }
        0x102d9720 => { vm.decode(0xa32abd75u32); let d = 14i16; let s = 11i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9724u64) }
        0x102d9724 => { vm.decode(0x121a07f5u32); let d = 1i16; let s = 1i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9728u64) }
        0x102d9728 => { vm.decode(0x34b300f5u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d972cu64) }
        0x102d972c => { vm.decode(0x425f17f5u32); let d = 11i16; let s = 11i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9730u64) }
        0x102d9730 => { vm.decode(0x4289875u32); let d = 1i16; let s = 1i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9734u64) }
        0x102d9734 => { vm.decode(0x4902a236u32); let d = 11i16; let s = 8i16; let s2 = -1i16; let imm = 7i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9738u64) }
        0x102d9738 => { vm.decode(0xa3483d75u32); let d = 11i16; let s = 11i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d973cu64) }
        0x102d973c => { vm.decode(0xc2c214f5u32); let d = 13i16; let s = 2i16; let s2 = 11i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9740u64) }
        0x102d9740 => { vm.decode(0x18001430u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d9744u64) }
        0x102d9744 => { vm.decode(0x4000a036u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 4i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9748u64) }
        0x102d9748 => { vm.decode(0xa0082d75u32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 3i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d974cu64) }
        0x102d974c => { vm.decode(0x224014f5u32); let d = 1i16; let s = 3i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9750u64) }
        0x102d9750 => { vm.decode(0x2002092fu32); let d = 13i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9754u64) }
        0x102d9754 => { vm.decode(0x3002236u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9758u64) }
        0x102d9758 => { vm.decode(0xa3122506u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d975cu64) }
        0x102d975c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9760u64).wrapping_add((imm * 4) as u64) } else { 0x102d9760u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9760u64) }
        0x102d9760 => { vm.decode(0x3001019u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9764u64) }
        0x102d9764 => { vm.decode(0x4b603236u32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9768u64) }
        0x102d9768 => { vm.decode(0x33400d75u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d976cu64) }
        0x102d976c => { vm.decode(0x8020c118u32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9770u64) }
        0x102d9770 => { vm.decode(0xa3122506u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9774u64) }
        0x102d9774 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9778u64).wrapping_add((imm * 4) as u64) } else { 0x102d9778u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9778u64) }
        0x102d9778 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d977cu64) } }
        0x102d977c => { vm.decode(0x3002236u32); let d = 15i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9780u64) }
        0x102d9780 => { vm.decode(0xa3122506u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9784u64) }
        0x102d9784 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9788u64).wrapping_add((imm * 4) as u64) } else { 0x102d9788u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9788u64) }
        0x102d9788 => { vm.decode(0x11000019u32); let d = 24i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d978cu64) }
        0x102d978c => { vm.decode(0x4b603236u32); let d = 15i16; let s = 15i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9790u64) }
        0x102d9790 => { vm.decode(0x21404d75u32); let d = 1i16; let s = 24i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9794u64) }
        0x102d9794 => { vm.decode(0x400c118u32); let d = 24i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9798u64) }
        0x102d9798 => { vm.decode(0xa3122506u32); let d = 1i16; let s = 15i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d979cu64) }
        0x102d979c => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d97a0u64).wrapping_add((imm * 4) as u64) } else { 0x102d97a0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d97a0u64) }
        0x102d97a0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d97a4u64) } }
        0x102d97a4 => { vm.decode(0x4012a835u32); let d = 1i16; let s = 5i16; let s2 = 24i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d97a8u64) }
        0x102d97a8 => { vm.decode(0xb16abd75u32); let d = 15i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d97acu64) }
        0x102d97ac => { vm.decode(0x121a17f5u32); let d = 1i16; let s = 1i16; let s2 = 15i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d97b0u64) }
        0x102d97b0 => { vm.decode(0x70b240f5u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d97b4u64) }
        0x102d97b4 => { vm.decode(0x144f57f5u32); let d = 13i16; let s = 13i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d97b8u64) }
        0x102d97b8 => { vm.decode(0x428c875u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d97bcu64) }
        0x102d97bc => { vm.decode(0x2a214f5u32); let d = 12i16; let s = 2i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d97c0u64) }
        0x102d97c0 => { vm.decode(0x18001030u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d97c4u64) }
        0x102d97c4 => { vm.decode(0xa24014f5u32); let d = 1i16; let s = 3i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d97c8u64) }
        0x102d97c8 => { vm.decode(0x2000092fu32); let d = 9i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d97ccu64) }
        0x102d97cc => { vm.decode(0x3002036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d97d0u64) }
        0x102d97d0 => { vm.decode(0xa1122506u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d97d4u64) }
        0x102d97d4 => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d97d8u64).wrapping_add((imm * 4) as u64) } else { 0x102d97d8u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d97d8u64) }
        0x102d97d8 => { vm.decode(0x3000019u32); let d = 12i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d97dcu64) }
        0x102d97dc => { vm.decode(0x4b403036u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d97e0u64) }
        0x102d97e0 => { vm.decode(0x31400d75u32); let d = 1i16; let s = 12i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d97e4u64) }
        0x102d97e4 => { vm.decode(0x20c118u32); let d = 12i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d97e8u64) }
        0x102d97e8 => { vm.decode(0xa1122506u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d97ecu64) }
        0x102d97ec => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d97f0u64).wrapping_add((imm * 4) as u64) } else { 0x102d97f0u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d97f0u64) }
        0x102d97f0 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d97f4u64) } }
        0x102d97f4 => { vm.decode(0x3002036u32); let d = 13i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d97f8u64) }
        0x102d97f8 => { vm.decode(0xa1122506u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d97fcu64) }
        0x102d97fc => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9800u64).wrapping_add((imm * 4) as u64) } else { 0x102d9800u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9800u64) }
        0x102d9800 => { vm.decode(0x3001019u32); let d = 14i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = vm.ld(s).wrapping_add(imm as u64);
        vm.st(d, v);
        Ok(0x102d9804u64) }
        0x102d9804 => { vm.decode(0x4b403036u32); let d = 13i16; let s = 13i16; let s2 = -1i16; let imm = 1i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9808u64) }
        0x102d9808 => { vm.decode(0x33400d75u32); let d = 1i16; let s = 14i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d980cu64) }
        0x102d980c => { vm.decode(0x8020c118u32); let d = 14i16; let s = 1i16; let s2 = -1i16; let imm = 1i64;
        let v = vm.ld(s) | imm as u64;
        vm.st(d, v);
        Ok(0x102d9810u64) }
        0x102d9810 => { vm.decode(0xa1122506u32); let d = 1i16; let s = 13i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d9814u64) }
        0x102d9814 => { vm.decode(0xa7f32db1u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = -5i64;
        vm.wr(36, if vm.ld(d) != vm.ld(s) { (0x102d9818u64).wrapping_add((imm * 4) as u64) } else { 0x102d9818u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9818u64) }
        0x102d9818 => { vm.decode(0xbb5u32); let d = 0i16; let s = 0i16; let s2 = -1i16; let imm = -65537i64;
        let m = vm.slots[35];
        vm.wr(0, 0);
        if m == 1 || m == 2 || m == 3 { vm.wr(35, 0); Ok(vm.slots[36]) } else { Ok(0x102d981cu64) } }
        0x102d981c => { vm.decode(0x4043a835u32); let d = 1i16; let s = 5i16; let s2 = 14i16; let imm = 0i64;
        let v = vm.ld(s) ^ vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9820u64) }
        0x102d9820 => { vm.decode(0xa16aad75u32); let d = 13i16; let s = 9i16; let s2 = -1i16; let imm = 27i64;
        let v = vm.ld(s).wrapping_shl(imm as u32);
        vm.st(d, v);
        Ok(0x102d9824u64) }
        0x102d9824 => { vm.decode(0x120a17f5u32); let d = 1i16; let s = 1i16; let s2 = 13i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9828u64) }
        0x102d9828 => { vm.decode(0x30b200f5u32); let d = 9i16; let s = 9i16; let s2 = -1i16; let imm = 6949i64;
        let sh = (imm & 0xff) as u32;
        let wd = (imm >> 8) as u32;
        let mask = if wd >= 64 { u64::MAX } else { (1u64 << wd) - 1 };
        let v = (vm.ld(s) >> sh) & mask;
        vm.st(d, v);
        Ok(0x102d982cu64) }
        0x102d982c => { vm.decode(0x4f17f5u32); let d = 9i16; let s = 9i16; let s2 = 12i16; let imm = 0i64;
        let v = vm.ld(s) & vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9830u64) }
        0x102d9830 => { vm.decode(0x4288875u32); let d = 1i16; let s = 1i16; let s2 = 9i16; let imm = 0i64;
        let v = vm.ld(s) | vm.ld(s2);
        vm.st(d, v);
        Ok(0x102d9834u64) }
        0x102d9834 => { vm.decode(0x28014f5u32); let d = 8i16; let s = 2i16; let s2 = 8i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9838u64) }
        0x102d9838 => { vm.decode(0x18000030u32); let d = 1i16; let s = 8i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.ld(d);
        vm.sw64(a, v)?;
        Ok(0x102d983cu64) }
        0x102d983c => { vm.decode(0x624014f5u32); let d = 1i16; let s = 3i16; let s2 = 10i16; let imm = 0i64;
        let v = vm.ld(s2).wrapping_add(vm.ld(s));
        vm.st(d, v);
        Ok(0x102d9840u64) }
        0x102d9840 => { vm.decode(0x92fu32); let d = 8i16; let s = 1i16; let s2 = -1i16; let imm = 0i64;
        let a = vm.ld(s).wrapping_add(imm as u64);
        let v = vm.mem.r64(a)?;
        vm.st(d, v);
        Ok(0x102d9844u64) }
        0x102d9844 => { vm.decode(0x1000236u32); let d = 10i16; let s = 0i16; let s2 = -1i16; let imm = 0i64;
        let v = (vm.ld(s) as i32 as i64).wrapping_add(imm) as u64;
        vm.st(d, v);
        Ok(0x102d9848u64) }
        0x102d9848 => { vm.decode(0x82122506u32); let d = 1i16; let s = 10i16; let s2 = -1i16; let imm = 27i64;
        let v = ((vm.ld(s) as i64) < imm) as u64;
        vm.st(d, v);
        Ok(0x102d984cu64) }
        0x102d984c => { vm.decode(0x848827u32); let d = 0i16; let s = 1i16; let s2 = -1i16; let imm = 7i64;
        vm.wr(36, if vm.ld(d) == vm.ld(s) { (0x102d9850u64).wrapping_add((imm * 4) as u64) } else { 0x102d9850u64 + 4 });
        vm.wr(35, 2);
        Ok(0x102d9850u64) }
        _ => Err(Error::UnknownPc(pc)),
    }
}
