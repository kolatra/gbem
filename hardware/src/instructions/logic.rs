use crate::ProgramCounter;

use super::Instruction;

pub fn get() -> Vec<Instruction> {
    vec![
        Instruction {
            mnemonic: "XOR A",
            opcode: 0xAF,
            cycles: 1,
            length: 1,
            handler: |cpu| {
                cpu.reg.a = (cpu.reg.a ^ cpu.reg.a);
                ProgramCounter::Next
            },
        },
        
        Instruction {
            mnemonic: "XOR B",
            opcode: 0xA8,
            cycles: 1,
            length: 1,
            handler: |cpu| {
                cpu.reg.a = (cpu.reg.a ^ cpu.reg.b);
                ProgramCounter::Next
            },
        },
    ]
}

pub fn get_16bit() -> Vec<Instruction> {
    vec![]
}
