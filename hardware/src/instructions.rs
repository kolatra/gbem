use std::sync::LazyLock;

use tracing::trace;

use crate::cpu::CPU;

pub static INSTRUCTIONS: LazyLock<Vec<Instruction>> = LazyLock::new(|| {
    trace!("initializing instruction list");
    let mut v = Vec::new();

    // silly goofy hack to make the disassembler continue.
    // the issue is probably a wrong instruction length,
    // but i don't want to find it rn
    v.insert(
        0,
        Instruction {
            mnemonic: "unimplemented instruction?",
            opcode: 0xdd,
            cycles: 1,
            length: 1,
            handler: |_| todo!(),
        },
    );

    v.append(&mut load::get());
    v.append(&mut logic::get());
    v.append(&mut rotate::get());
    v.append(&mut bits::get());
    v.append(&mut control::get());
    v.append(&mut jump::get());
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
    handler: fn(cpu: &mut CPU),
}

impl Instruction {
    pub fn run(&self, cpu: &mut CPU) {
        (self.handler)(cpu)
    }
}

mod bits;
mod control;
mod jump;
mod load;
mod logic;
/// Macros for some instructions that are logically
/// the same but act on different registers/locations
mod macros;
mod rotate;
