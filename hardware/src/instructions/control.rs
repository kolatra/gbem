use crate::ProgramCounter;

use super::Instruction;

pub fn get() -> Vec<Instruction> {
    vec![
        Instruction {
            mnemonic: "NOP",
            opcode: 0x00,
            cycles: 1,
            length: 1,
            handler: |_| ProgramCounter::Next,
        },
        Instruction {
            mnemonic: "EI",
            opcode: 0xFB,
            cycles: 1,
            length: 1,
            handler: |_| ProgramCounter::Next,
        },
        Instruction {
            mnemonic: "DI",
            opcode: 0xF3,
            cycles: 1,
            length: 1,
            handler: |_| ProgramCounter::Next,
        },
        Instruction {
            mnemonic: "CCF",
            opcode: 0x3F,
            cycles: 1,
            length: 1,
            handler: |_| ProgramCounter::Next,
        },
        Instruction {
            mnemonic: "SCF",
            opcode: 0x37,
            cycles: 1,
            length: 1,
            handler: |_| ProgramCounter::Next,
        },
        Instruction {
            mnemonic: "HALT",
            opcode: 0x76,
            cycles: 1,
            length: 1,
            handler: |_| ProgramCounter::Next,
        },
        Instruction {
            mnemonic: "STOP",
            opcode: 0x10_00,
            cycles: 1,
            length: 2,
            handler: |_| ProgramCounter::Next,
        },
    ]
}
