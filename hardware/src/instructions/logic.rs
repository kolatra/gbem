use super::Instruction;

macro_rules! inc_reg {
    ($mnemonic:ident, $opcode:expr, $reg:ident) => {
        Instruction {
            mnemonic: stringify!($mnemonic),
            opcode: $opcode,
            cycles: 1,
            length: 1,
            handler: |cpu| {
                cpu.reg.$reg = cpu.reg.$reg.wrapping_add(1);
            },
        }
    };
}

pub fn get() -> Vec<Instruction> {
    vec![
        inc_reg!(INC_C, 0x0C, c),
        Instruction {
            mnemonic: "INC DE",
            opcode: 0x13,
            cycles: 2,
            length: 1,
            handler: |cpu| {
                cpu.reg.e = cpu.reg.e.wrapping_add(1);
                if cpu.reg.e == 0 {
                    cpu.reg.d = cpu.reg.d.wrapping_add(1);
                }
            },
        },
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

    #[test]
    fn test_inc_c() {
        let mut cpu = CPU::new();
        let instructions = get();
        let instruction = instructions.iter().find(|i| i.opcode == 0x0C).unwrap();

        cpu.reg.c = 0;
        instruction.run(&mut cpu);
        assert_eq!(cpu.reg.c, 1);

        cpu.reg.c = 255;
        instruction.run(&mut cpu);
        assert_eq!(cpu.reg.c, 0);
    }
}
