use super::Instruction;

pub fn get() -> Vec<Instruction> {
    vec![
        Instruction {
            mnemonic: "JR s8",
            opcode: 0x18,
            cycles: 3,
            length: 2,
            handler: |_cpu| todo!(),
        },
        Instruction {
            mnemonic: "JR NZ, s8",
            opcode: 0x20,
            cycles: 3, // 2 if not taken
            length: 2,
            handler: |_cpu| todo!(),
        },
        Instruction {
            mnemonic: "JR Z, s8",
            opcode: 0x28,
            cycles: 3, // 2 if not taken
            length: 2,
            handler: |_cpu| todo!(),
        },
        Instruction {
            mnemonic: "CALL a16",
            opcode: 0xCD,
            cycles: 6,
            length: 3,
            handler: |_cpu| todo!(),
        },
        Instruction {
            mnemonic: "CALL Z, a16",
            opcode: 0xCC,
            cycles: 6, // 3 if not taken
            length: 3,
            handler: |_cpu| todo!(),
        },
        Instruction {
            mnemonic: "RST 7",
            opcode: 0xFF,
            cycles: 4,
            length: 1,
            handler: |cpu| {
                cpu.push_stack((cpu.reg.pc >> 8) as u8);
                cpu.push_stack(cpu.reg.pc as u8);
                // cpu.reg.pc = 0x38;
                cpu.reg.pc = 0x0138;
            },
        },
        Instruction {
            mnemonic: "RET NZ",
            opcode: 0xC0,
            cycles: 5, // 2 if not taken
            length: 1,
            handler: |_cpu| todo!(),
        },
        Instruction {
            mnemonic: "RET",
            opcode: 0xC9,
            cycles: 4,
            length: 1,
            handler: |_cpu| todo!(),
        },
    ]
}
