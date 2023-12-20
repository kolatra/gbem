use super::Instruction;

pub fn get() -> Vec<Instruction> {
    vec![
        Instruction {
            mnemonic: "JP a16",
            opcode: 0xC3,
            cycles: 4,
            length: 3,
            handler: |cpu| {
                let low = cpu.reg.pc;
                let high = cpu.reg.pc + 1;
                cpu.reg.pc = (high << 8) | low;
            },
        },
        Instruction {
            mnemonic: "JR s8",
            opcode: 0x18,
            cycles: 3,
            length: 2,
            handler: |cpu| {
                let offset = cpu.read_next_byte();
                cpu.reg.pc += offset as u16;
            },
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
        Instruction {
            mnemonic: "RET Z",
            opcode: 0xC8,
            cycles: 5, // 2 if not taken
            length: 1,
            handler: |_cpu| todo!(),
        },
        Instruction {
            mnemonic: "RET NC",
            opcode: 0xD0,
            cycles: 5, // 2 if not taken
            length: 1,
            handler: |_cpu| todo!(),
        },
        Instruction {
            mnemonic: "RET C",
            opcode: 0xD8,
            cycles: 5, // 2 if not taken
            length: 1,
            handler: |_cpu| todo!(),
        },
        Instruction {
            mnemonic: "RETI",
            opcode: 0xD9,
            cycles: 4,
            length: 1,
            handler: |_cpu| todo!(),
        },
    ]
}
