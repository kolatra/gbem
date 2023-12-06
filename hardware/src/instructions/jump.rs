use super::Instruction;

pub fn get() -> Vec<Instruction> {
    vec![
        Instruction {
            mnemonic: "JR NZ, s8",
            opcode: 0x20,
            cycles: 3, // 2 if not taken
            length: 2,
            handler: |_cpu| {},
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
    ]
}
