use super::Instruction;

pub fn get() -> Vec<Instruction> {
    vec![
        Instruction {
            mnemonic: "RLA",
            opcode: 0x17,
            cycles: 1,
            length: 1,
            handler: |_| todo!(),
        },
        Instruction {
            mnemonic: "RRA",
            opcode: 0x1F,
            cycles: 1,
            length: 1,
            handler: |_| todo!(),
        },
        Instruction {
            mnemonic: "RLCA",
            opcode: 0x07,
            cycles: 1,
            length: 1,
            handler: |cpu| {
                cpu.reg.a <<= 1;
                cpu.reg.f = cpu.reg.a & 0x40;
                cpu.reg.a &= 0x1;
                1
            }
        },
    ]
}
