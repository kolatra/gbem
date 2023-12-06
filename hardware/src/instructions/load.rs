use crate::{ld_a8_a, load_16bit, load_8bit, load_a_8bit, load_imm, load_r_into_r, reg::Pair};

use super::Instruction;

pub fn get() -> Vec<Instruction> {
    vec![
        ld_a8_a!(LDH_A8_A, 0xE0),
        load_imm!(LD_A_D8, 0x3E, a),
        load_imm!(LD_B_D8, 0x06, b),
        load_imm!(LD_C_D8, 0x0E, c),
        load_imm!(LD_L_D8, 0x2E, e),
        load_r_into_r!(LD_B_A, 0x47, b, a),
        // load_8bit!(LD_A_A, 0x7F, a),
        load_a_8bit!(LD_A_B, 0x78, b),
        load_a_8bit!(LD_A_C, 0x79, c),
        load_a_8bit!(LD_A_D, 0x7A, d),
        load_a_8bit!(LD_A_E, 0x7B, e),
        load_a_8bit!(LD_A_H, 0x7C, h),
        load_a_8bit!(LD_A_L, 0x7D, l),
        load_8bit!(LD_H_A, 0x67, a, h),
        load_16bit!(LD_SP_D16, 0x31, sp),
        Instruction {
            mnemonic: "LD (a16), A",
            opcode: 0xEA,
            cycles: 4,
            length: 3,
            handler: |_| todo!(),
        },
        Instruction {
            mnemonic: "LD (C), A",
            opcode: 0xE2,
            cycles: 2,
            length: 1,
            handler: |cpu| {
                cpu.mmu.write(0xFF00 + cpu.reg.c as u16, cpu.reg.a);
            },
        },
        Instruction {
            mnemonic: "LD DE, d16",
            opcode: 0x11,
            cycles: 3,
            length: 3,
            handler: |cpu| {
                let d16 = cpu.mmu.read_word(cpu.reg.pc + 1);
                cpu.reg.d = (d16 >> 8) as u8;
                cpu.reg.e = d16 as u8;
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
            handler: |cpu| {
                cpu.reg.h = cpu.mmu.read(cpu.reg.pc + 1);
            },
        },
        Instruction {
            mnemonic: "LD (HL-), A",
            opcode: 0x32,
            cycles: 2,
            length: 1,
            handler: |_cpu| todo!(),
        },
        Instruction {
            mnemonic: "LD (HL), A",
            opcode: 0x77,
            cycles: 2,
            length: 1,
            handler: |_cpu| todo!(),
        },
        Instruction {
            mnemonic: "LD (HL+), A",
            opcode: 0x22,
            cycles: 2,
            length: 1,
            handler: |_cpu| todo!(),
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
}
