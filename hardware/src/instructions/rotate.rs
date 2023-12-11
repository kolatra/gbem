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
    ]
}
