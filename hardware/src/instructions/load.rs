use super::Instruction;

pub fn get() -> Vec<Instruction> {
    vec![
        Instruction {
            mnemonic: "LD HL, d16",
            opcode: 0x21,
            cycles: 3,
            length: 3,
            handler: |cpu| {
                let d16 = cpu.mmu.read_word(cpu.reg.pc + 1);
                cpu.reg.h = (d16 >> 8) as u8;
                cpu.reg.l = d16 as u8;
            },
        },
        Instruction {
            mnemonic: "LD SP, d16",
            opcode: 0x31,
            cycles: 3,
            length: 3,
            handler: |cpu| {
                let d16 = cpu.mmu.read_word(cpu.reg.pc + 1);
                cpu.reg.sp = d16;
            },
        },
        Instruction {
            mnemonic: "LD (HL-), A",
            opcode: 0x32,
            cycles: 2,
            length: 1,
            handler: |_cpu| (),
        },
    ]
}
