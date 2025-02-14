use std::sync::LazyLock;

use tracing::trace;

use crate::cpu::CPU;

pub static INSTRUCTIONS: LazyLock<Vec<Instruction>> = LazyLock::new(|| {
    trace!("initializing instruction list");
    let mut v = Vec::new();

    v.append(&mut load::get());
    v.append(&mut logic::get());
    v.append(&mut rotate::get());
    v.append(&mut bits::get());
    v.append(&mut control::get());
    v.append(&mut jump::get());
    v.append(&mut cb::get());
    v
});

// https://meganesu.github.io/generate-gb-opcodes/
// https://gekkio.fi/files/gb-docs/gbctr.pdf
#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub mnemonic: &'static str,
    pub opcode: u32,
    pub cycles: u16,
    /// Length in bytes
    pub length: u16,
    handler: fn(cpu: &mut CPU) -> u16,
}

impl Instruction {
    pub fn run(&self, cpu: &mut CPU) -> u16 {
        (self.handler)(cpu)
    }
}

mod bits;
mod cb;
mod control;
mod jump;
mod load;
mod logic;
/// Macros for some instructions that are logically
/// the same but act on different registers/locations
mod macros;
mod rotate;
