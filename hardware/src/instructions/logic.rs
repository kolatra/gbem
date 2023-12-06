use crate::{inc_reg, xor_reg};

use super::Instruction;

pub fn get() -> Vec<Instruction> {
    vec![
        inc_reg!(INC_A, 0x3C, a),
        inc_reg!(INC_B, 0x04, b),
        inc_reg!(INC_C, 0x0C, c),
        inc_reg!(INC_D, 0x14, d),
        inc_reg!(INC_E, 0x1C, e),
        inc_reg!(INC_H, 0x24, h),
        inc_reg!(INC_L, 0x2C, l),
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
        xor_reg!(XOR_A, 0xAF, a),
        xor_reg!(XOR_B, 0xA8, b),
        xor_reg!(XOR_C, 0xA9, c),
        xor_reg!(XOR_D, 0xAA, d),
        xor_reg!(XOR_E, 0xAB, e),
        xor_reg!(XOR_H, 0xAC, h),
        xor_reg!(XOR_L, 0xAD, l),
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
