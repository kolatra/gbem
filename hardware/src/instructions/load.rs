use crate::{
    ld_a8_a, load_16_into_8, load_16bit, load_16_bit, load_8bit, load_a_8bit, load_imm, load_r_into_r, reg::Pair,
};

use super::Instruction;

pub fn get() -> Vec<Instruction> {
    vec![
        ld_a8_a!(LDH_A8_A, 0xE0),
        Instruction {
            mnemonic: "LD A, (a8)",
            opcode: 0xF0,
            cycles: 3,
            length: 2,
            handler: |_| todo!(),
        },
        load_imm!(LD_A_D8, 0x3E, a),
        load_imm!(LD_B_D8, 0x06, b),
        load_imm!(LD_C_D8, 0x0E, c),
        load_imm!(LD_D_D8, 0x16, d),
        load_imm!(LD_E_D8, 0x1E, e),
        load_imm!(LD_L_D8, 0x2E, l),
        load_r_into_r!(LD_B_A, 0x47, b, a),
        load_r_into_r!(LD_C_A, 0x4F, c, a),
        load_r_into_r!(LD_B_B, 0x50, b, b),
        load_a_8bit!(LD_A_A, 0x7F, a),
        load_a_8bit!(LD_A_B, 0x78, b),
        load_a_8bit!(LD_A_C, 0x79, c),
        load_a_8bit!(LD_A_D, 0x7A, d),
        load_a_8bit!(LD_A_E, 0x7B, e),
        load_a_8bit!(LD_A_H, 0x7C, h),
        load_a_8bit!(LD_A_L, 0x7D, l),
        load_8bit!(LD_H_A, 0x67, a, h),
        load_8bit!(LD_D_A, 0x57, a, d),
        load_8bit!(LD_H_E, 0x63, e, h),
        load_8bit!(LD_B_C, 0x41, c, b),
        load_8bit!(LD_B_D, 0x42, d, b),
        load_16bit!(LD_SP_D16, 0x31, sp),
        load_16_bit!(LD_BC_D16, 0x01, BC),
        load_16_into_8!(LD_L_HL, 0x6E, Pair::HL, l),
        load_16_into_8!(LD_HL_A, 0x77, Pair::HL, a),
        Instruction {
            mnemonic: "LD (a16), A",
            opcode: 0xEA,
            cycles: 4,
            length: 3,
            handler: |cpu| {
                let nn = cpu.read_next_word();
                cpu.mmu.write(nn, cpu.reg.a);
            },
        },
        Instruction {
            mnemonic: "LD (a16), SP",
            opcode: 0x08,
            cycles: 5,
            length: 3,
            handler: |cpu| {
                let nn = cpu.read_next_word();
                cpu.mmu.write_word(nn, cpu.reg.sp);
            },
        },
        Instruction {
            mnemonic: "LD (C), A",
            opcode: 0xE2,
            cycles: 2,
            length: 1,
            handler: |cpu| {
                cpu.mmu.write(0xFF00 + u16::from(cpu.reg.c), cpu.reg.a);
            },
        },
        Instruction {
            mnemonic: "LD DE, d16",
            opcode: 0x11,
            cycles: 3,
            length: 3,
            handler: |cpu| {
                let d16 = cpu.mmu.read_word(cpu.reg.pc + 1);
                cpu.reg.write_pair(Pair::DE, d16);
            },
        },
        Instruction {
            mnemonic: "LD A, (DE)",
            opcode: 0x1A,
            cycles: 2,
            length: 1,
            handler: |cpu| {
                let to_write = cpu.reg.read_pair(Pair::DE);
                cpu.reg.a = to_write as u8;
            },
        },
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
            mnemonic: "LD H, d8",
            opcode: 0x26,
            cycles: 2,
            length: 2,
            handler: |cpu| cpu.reg.h = cpu.mmu.read(cpu.reg.pc + 1),
        },
        Instruction {
            mnemonic: "LD (HL-), A",
            opcode: 0x32,
            cycles: 2,
            length: 1,
            handler: |cpu| {
                let hl = cpu.reg.read_pair(Pair::HL);
                cpu.mmu.write(hl, cpu.reg.a);
                cpu.reg.write_pair(Pair::HL, hl - 1);
            },
        },
        Instruction {
            mnemonic: "LD (HL+), A",
            opcode: 0x22,
            cycles: 2,
            length: 1,
            handler: |cpu| {
                let hl = cpu.reg.read_pair(Pair::HL);
                cpu.mmu.write(hl, cpu.reg.a);
                cpu.reg.write_pair(Pair::HL, hl + 1);
            },
        },
        Instruction {
            mnemonic: "PUSH BC",
            opcode: 0xC5,
            cycles: 2,
            length: 1,
            handler: |_cpu| todo!(),
        },
        Instruction {
            mnemonic: "POP BC",
            opcode: 0xC1,
            cycles: 3,
            length: 1,
            handler: |_cpu| todo!(),
        },
        Instruction {
            mnemonic: "LD H, (HL)",
            opcode: 0x66,
            cycles: 2,
            length: 1,
            handler: |cpu| cpu.reg.h = cpu.reg.read_pair(Pair::HL) as u8,
        },
        Instruction {
            mnemonic: "LD (HL), E",
            opcode: 0x73,
            cycles: 2,
            length: 1,
            handler: |cpu| {
                let hl = cpu.reg.read_pair(Pair::HL);
                cpu.mmu.write(hl, cpu.reg.e);
            },
        },
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cpu::CPU;

    #[test]
    fn test_load_8bit() {
        let mut cpu = CPU::new();
        cpu.reg.b = 0x42;
        let instruction = load_a_8bit!(LD_A_B, 0x78, b);
        instruction.run(&mut cpu);
        assert_eq!(cpu.reg.a, 0x42);
    }

    #[test]
    fn test_load_16bit() {
        let mut cpu = CPU::new();
        let instruction = load_16bit!(LD_SP_D16, 0x31, sp);
        cpu.mmu.write_word(0x0101, 0x1234);
        instruction.run(&mut cpu);
        assert_eq!(cpu.reg.sp, 0x1234);
    }

    #[test]
    fn test_load_16_into_8() {
        let mut cpu = CPU::new();
        cpu.reg.h = 0x12;
        cpu.reg.l = 0x34;
        let instruction = load_16_into_8!(LD_L_HL, 0x6E, Pair::HL, l);
        instruction.run(&mut cpu);
        assert_eq!(cpu.reg.l, 0x34);
    }
}
