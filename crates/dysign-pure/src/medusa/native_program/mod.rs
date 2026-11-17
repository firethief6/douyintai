#![cfg_attr(rustfmt, rustfmt::skip)]
// 由 scripts/medusa-vm/gen_native.py 从 words.jsonl 生成，勿手改。
mod p0;
mod p1;
mod p2;
mod p3;
mod p4;
mod p5;
mod p6;
mod p7;
mod p8;
mod p9;
mod p10;
mod p11;
mod p12;
mod p13;
mod p14;
mod p15;
mod p16;
mod p17;
mod p18;
mod p19;
mod p20;
mod p21;
mod p22;
mod p23;
use super::Vm;
use super::Error;

pub fn exec(vm: &mut Vm, pc: u64) -> Result<u64, Error> {
    match pc {
        0x1026d390..=0x10273b7c => p0::exec(vm, pc),
        0x10273b80..=0x10275e30 => p1::exec(vm, pc),
        0x10275e34..=0x10278c80 => p2::exec(vm, pc),
        0x10278c84..=0x1027d7ec => p3::exec(vm, pc),
        0x1027d7f0..=0x102818ec => p4::exec(vm, pc),
        0x102818f0..=0x102887a0 => p5::exec(vm, pc),
        0x102887a4..=0x1028d308 => p6::exec(vm, pc),
        0x1028d30c..=0x1029257c => p7::exec(vm, pc),
        0x10292580..=0x102997d0 => p8::exec(vm, pc),
        0x102997d4..=0x1029ca6c => p9::exec(vm, pc),
        0x1029ca70..=0x1029fcfc => p10::exec(vm, pc),
        0x1029fd00..=0x102a2f90 => p11::exec(vm, pc),
        0x102a2f94..=0x102a9bac => p12::exec(vm, pc),
        0x102a9bb0..=0x102bc598 => p13::exec(vm, pc),
        0x102bc59c..=0x102c33a8 => p14::exec(vm, pc),
        0x102c33ac..=0x102c7b04 => p15::exec(vm, pc),
        0x102c7b08..=0x102cd59c => p16::exec(vm, pc),
        0x102cd5a0..=0x102cf60c => p17::exec(vm, pc),
        0x102cf610..=0x102d1698 => p18::exec(vm, pc),
        0x102d169c..=0x102d3710 => p19::exec(vm, pc),
        0x102d3714..=0x102d577c => p20::exec(vm, pc),
        0x102d5780..=0x102d77e0 => p21::exec(vm, pc),
        0x102d77e4..=0x102d984c => p22::exec(vm, pc),
        0x102d9850..=0x102e51dc => p23::exec(vm, pc),
        _ => Err(Error::UnknownPc(pc)),
    }
}
// 49575 条
