use crate::reg::FlagBit;

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
                cpu.reg.pc += u16::from(offset);
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
            handler: |cpu| {
                cpu.store_pc();
                cpu.reg.pc = cpu.read_next_word();
            },
        },
        Instruction {
            mnemonic: "CALL Z, a16",
            opcode: 0xCC,
            cycles: 6, // 3 if not taken
            length: 3,
            handler: |cpu| {
                if cpu.reg.is_set(FlagBit::Z) {
                    cpu.store_pc();
                    cpu.reg.pc = cpu.read_next_word();
                }
            },
        },
        Instruction {
            mnemonic: "CALL C, a16",
            opcode: 0xDC,
            cycles: 6, // 3 if not taken
            length: 3,
            handler: |cpu| {
                if cpu.reg.is_set(FlagBit::C) {
                    let pc = cpu.reg.pc + 1;
                    cpu.push_stack((pc >> 8) as u8);
                    cpu.push_stack(pc as u8);
                    cpu.reg.pc = cpu.read_next_word();
                }
            },
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
            handler: |cpu| {
                if !cpu.reg.is_set(FlagBit::Z) {
                    let b2 = cpu.pop_stack();
                    let b1 = cpu.pop_stack();
                    let new_pc = u16::from(b1) << 8 | u16::from(b2);
                    cpu.reg.pc = new_pc;
                }
            },
        },
        Instruction {
            mnemonic: "RET",
            opcode: 0xC9,
            cycles: 4,
            length: 1,
            handler: |cpu| {
                let b2 = cpu.pop_stack();
                let b1 = cpu.pop_stack();
                let new_pc = u16::from(b1) << 8 | u16::from(b2);
                cpu.reg.pc = new_pc;
            },
        },
        Instruction {
            mnemonic: "RET Z",
            opcode: 0xC8,
            cycles: 5, // 2 if not taken
            length: 1,
            handler: |cpu| {
                if cpu.reg.is_set(FlagBit::Z) {
                    let b2 = cpu.pop_stack();
                    let b1 = cpu.pop_stack();
                    let new_pc = u16::from(b1) << 8 | u16::from(b2);
                    cpu.reg.pc = new_pc;
                }
            },
        },
        Instruction {
            mnemonic: "RET NC",
            opcode: 0xD0,
            cycles: 5, // 2 if not taken
            length: 1,
            handler: |cpu| {
                if !cpu.reg.is_set(FlagBit::C) {
                    let b2 = cpu.pop_stack();
                    let b1 = cpu.pop_stack();
                    let new_pc = u16::from(b1) << 8 | u16::from(b2);
                    cpu.reg.pc = new_pc;
                }
            },
        },
        Instruction {
            mnemonic: "RET C",
            opcode: 0xD8,
            cycles: 5, // 2 if not taken
            length: 1,
            handler: |cpu| {
                if cpu.reg.is_set(FlagBit::C) {
                    let b2 = cpu.pop_stack();
                    let b1 = cpu.pop_stack();
                    let new_pc = u16::from(b1) << 8 | u16::from(b2);
                    cpu.reg.pc = new_pc;
                }
            },
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
