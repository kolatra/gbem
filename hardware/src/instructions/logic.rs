use super::Instruction;

pub fn get() -> Vec<Instruction> {
    vec![
        Instruction {
            mnemonic: "XOR A",
            opcode: 0xAF,
            cycles: 1,
            length: 1,
            handler: |cpu| {
                cpu.reg.a ^= cpu.reg.a;
            },
        },
        Instruction {
            mnemonic: "XOR B",
            opcode: 0xA8,
            cycles: 1,
            length: 1,
            handler: |cpu| {
                cpu.reg.a ^= cpu.reg.b;
            },
        },
        Instruction {
            mnemonic: "ADD A,B",
            opcode: 0x80,
            cycles: 1,
            length: 1,
            handler: |cpu| {
                cpu.add(cpu.reg.b, false);
            },
        },
        Instruction {
            mnemonic: "SUB A,B",
            opcode: 0x90,
            cycles: 1,
            length: 1,
            handler: |cpu| {
                cpu.sub(cpu.reg.b, false);
            },
        },
    ]
}

pub fn get_16bit() -> Vec<Instruction> {
    vec![]
}
