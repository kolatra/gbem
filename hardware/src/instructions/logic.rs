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

#[allow(dead_code)]
pub fn get_16bit() -> Vec<Instruction> {
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::CPU;
    use crate::FlagBit::*;

    #[test]
    fn test_add() {
        let mut cpu = CPU::new();
        let instructions = get();
        let instruction = instructions
            .iter()
            .find(|i| i.mnemonic == "ADD A,B")
            .unwrap();

        cpu.reg.a = 0;
        cpu.reg.b = 0;
        instruction.run(&mut cpu);
        assert!(cpu.is_set(Z));

        cpu.reg.a = 62;
        cpu.reg.b = 34;
        instruction.run(&mut cpu);
        assert_eq!(cpu.reg.a, 96);
        assert!(cpu.is_set(H));

        cpu.reg.a = 255;
        cpu.reg.b = 5;
        instruction.run(&mut cpu);
        assert!(cpu.is_set(C));
    }

    #[test]
    fn test_sub() {
        let mut cpu = CPU::new();
        let instructions = get();
        let instruction = instructions
            .iter()
            .find(|i| i.mnemonic == "SUB A,B")
            .unwrap();

        cpu.reg.a = 0;
        cpu.reg.b = 0;
        instruction.run(&mut cpu);
        assert!(cpu.is_set(Z));

        cpu.reg.a = 62;
        cpu.reg.b = 34;
        instruction.run(&mut cpu);
        assert_eq!(cpu.reg.a, 28);
        assert!(cpu.is_set(H));

        cpu.reg.a = 0;
        cpu.reg.b = 5;
        instruction.run(&mut cpu);
        assert!(cpu.is_set(C));
    }

    #[test]
    #[rustfmt::skip] // :)
    fn test_xor() {
        let mut cpu = CPU::new();
        let instructions = get();
        let instruction = instructions
            .iter()
            .find(|i| i.mnemonic == "XOR A")
            .unwrap();

        cpu.reg.a = 124;
        instruction.run(&mut cpu);
        assert_eq!(cpu.reg.a, 0);
    }
}
