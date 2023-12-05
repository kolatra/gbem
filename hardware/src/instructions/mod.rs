use std::sync::OnceLock;

use crate::{cpu::CPU, ProgramCounter};

pub fn get() -> &'static Vec<Instruction> {
    static INS: OnceLock<Vec<Instruction>> = OnceLock::new();
    INS.get_or_init(|| {
        let mut v = Vec::new();
        v.append(&mut load::get());
        v.append(&mut logic::get());
        v.append(&mut rotate::get());
        v.append(&mut bits::get());
        v.append(&mut control::get());
        v.append(&mut jump::get());
        v
    })
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub mnemonic: &'static str,
    pub opcode: u16, // NOTE: this was a u32, but u16 is probably fine, keep an eye
    pub cycles: i8,
    pub length: i8,
    handler: fn(cpu: &mut CPU) -> ProgramCounter,
}

impl Instruction {
    fn run(&self, cpu: &mut CPU) -> ProgramCounter {
        (self.handler)(cpu)
    }
}

mod bits;
mod control;
mod jump;
mod load;
mod logic;
mod rotate;
