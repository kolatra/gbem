use crate::{
    add, addc, and_reg, cp_r, dec_pair, dec_reg, inc_pair, inc_reg,
    reg::{FlagBit, Pair},
    sub, subc, xor_reg,
};

use super::Instruction;

pub fn get() -> Vec<Instruction> {
    vec![
        Instruction {
            mnemonic: "CP d8",
            opcode: 0xFE,
            cycles: 2,
            length: 2,
            handler: |cpu| {
                let cmp = cpu.reg.a - cpu.mmu.read(cpu.reg.pc + 1);
                cpu.set_flag(FlagBit::Z, cmp == 0);
                2
            },
        },
        inc_reg!(INC_A, 0x3C, a),
        inc_reg!(INC_B, 0x04, b),
        inc_reg!(INC_C, 0x0C, c),
        inc_reg!(INC_D, 0x14, d),
        inc_reg!(INC_E, 0x1C, e),
        inc_reg!(INC_H, 0x24, h),
        inc_reg!(INC_L, 0x2C, l),
        dec_reg!(DEC_A, 0x3D, a),
        dec_reg!(DEC_B, 0x05, b),
        dec_reg!(DEC_C, 0x0D, c),
        dec_reg!(DEC_D, 0x15, d),
        dec_reg!(DEC_E, 0x1D, e),
        inc_pair!(INC_HL, 0x23, HL),
        inc_pair!(INC_BC, 0x03, BC),
        inc_pair!(INC_DE, 0x13, DE),
        inc_pair!(INC_SH, 0x33, SH),
        dec_pair!(DEC_HL, 0x2B, HL),
        dec_pair!(DEC_BC, 0x0B, BC),
        xor_reg!(XOR_A, 0xAF, a),
        xor_reg!(XOR_B, 0xA8, b),
        xor_reg!(XOR_C, 0xA9, c),
        xor_reg!(XOR_D, 0xAA, d),
        xor_reg!(XOR_E, 0xAB, e),
        xor_reg!(XOR_H, 0xAC, h),
        xor_reg!(XOR_L, 0xAD, l),
        Instruction {
            mnemonic: "XOR (HL)",
            opcode: 0xAE,
            cycles: 2,
            length: 1,
            handler: |cpu| {
                cpu.reg.a |= cpu.reg.read_pair(Pair::HL) as u8;
                1
            },
        },
        and_reg!(AND_A, 0xA7, a),
        and_reg!(AND_B, 0xA0, b),
        and_reg!(AND_C, 0xA1, c),
        and_reg!(AND_D, 0xA2, d),
        and_reg!(AND_E, 0xA3, e),
        and_reg!(AND_H, 0xA4, h),
        and_reg!(AND_L, 0xA5, l),
        cp_r!(CP_A_A, 0xBF, a),
        cp_r!(CP_A_B, 0xB8, b),
        cp_r!(CP_A_C, 0xB9, c),
        cp_r!(CP_A_D, 0xBA, d),
        cp_r!(CP_A_E, 0xBB, e),
        cp_r!(CP_A_H, 0xBC, h),
        cp_r!(CP_A_L, 0xBD, l),
        Instruction {
            mnemonic: "CP (HL)",
            opcode: 0xBE,
            cycles: 2,
            length: 1,
            handler: |cpu| {
                let cmp = cpu.reg.a - cpu.mmu.read(cpu.reg.read_pair(Pair::HL));
                cpu.set_flag(FlagBit::Z, cmp == 0);
                1
            },
        },
        add!(ADD_A_A, 0x87, a),
        add!(ADD_A_B, 0x80, b),
        add!(ADD_A_C, 0x81, c),
        add!(ADD_A_D, 0x82, d),
        add!(ADD_A_E, 0x83, e),
        add!(ADD_A_H, 0x84, h),
        add!(ADD_A_L, 0x85, l),
        Instruction {
            mnemonic: "ADD A, (HL)",
            opcode: 0x86,
            cycles: 2,
            length: 1,
            handler: |cpu| {
                let hl = cpu.reg.read_pair(Pair::HL);
                let to_add = cpu.mmu.read(hl);
                cpu.add(to_add, false);
                1
            },
        },
        addc!(ADC_A_A, 0x8F, a),
        addc!(ADC_A_B, 0x88, b),
        addc!(ADC_A_C, 0x89, c),
        addc!(ADC_A_D, 0x8A, d),
        addc!(ADC_A_E, 0x8B, e),
        addc!(ADC_A_H, 0x8C, h),
        addc!(ADC_A_L, 0x8D, l),
        sub!(SUB_B, 0x90, b),
        sub!(SUB_C, 0x91, c),
        sub!(SUB_D, 0x92, d),
        sub!(SUB_E, 0x93, e),
        sub!(SUB_H, 0x94, h),
        sub!(SUB_L, 0x95, l),
        subc!(SBC_A_A, 0x9F, a),
        subc!(SBC_A_B, 0x98, b),
        subc!(SBC_A_C, 0x99, c),
        subc!(SBC_A_D, 0x9A, d),
        subc!(SBC_A_E, 0x9B, e),
        subc!(SBC_A_H, 0x9C, h),
        subc!(SBC_A_L, 0x9D, l),
        Instruction {
            mnemonic: "ADC A, d8",
            opcode: 0xCE,
            cycles: 2,
            length: 2,
            handler: |cpu| {
                cpu.add(cpu.read_next_byte(), true);
                2
            },
        },
        Instruction {
            mnemonic: "AND d8",
            opcode: 0xE6,
            cycles: 2,
            length: 2,
            handler: |cpu| {
                cpu.reg.a &= cpu.read_next_byte();
                2
            },
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::CPU;
    use crate::reg::FlagBit::*;

    #[test]
    fn test_add() {
        let mut cpu = CPU::new();
        let instructions = get();
        let instruction = instructions
            .iter()
            .find(|i| i.mnemonic == "ADD_A_B")
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
        let instruction = instructions.iter().find(|i| i.mnemonic == "SUB_B").unwrap();

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
    fn test_xor() {
        let mut cpu = CPU::new();
        let instructions = get();
        let instruction = instructions.iter().find(|i| i.mnemonic == "XOR_A").unwrap();

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
