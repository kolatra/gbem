#[macro_export]
macro_rules! inc_reg {
    ($mnemonic:ident, $opcode:expr, $reg:ident) => {
        Instruction {
            mnemonic: stringify!($mnemonic),
            opcode: $opcode,
            cycles: 1,
            length: 1,
            handler: |cpu| cpu.reg.$reg = cpu.reg.$reg.wrapping_add(1),
        }
    };
}

#[macro_export]
macro_rules! dec_reg {
    ($mnemonic:ident, $opcode:expr, $reg:ident) => {
        Instruction {
            mnemonic: stringify!($mnemonic),
            opcode: $opcode,
            cycles: 1,
            length: 1,
            handler: |cpu| cpu.reg.$reg = cpu.reg.$reg.wrapping_sub(1),
        }
    };
}

#[macro_export]
macro_rules! inc_pair {
    ($mnemonic:ident, $opcode:expr, $pair:ident) => {
        Instruction {
            mnemonic: stringify!($mnemonic),
            opcode: $opcode,
            cycles: 2,
            length: 1,
            handler: |_cpu| todo!(),
            /* handler: |cpu| {
                cpu.reg.e = cpu.reg.e.wrapping_add(1);
                if cpu.reg.e == 0 {
                    cpu.reg.d = cpu.reg.d.wrapping_add(1);
                }
            }, */
        }
    };
}

#[macro_export]
macro_rules! dec_pair {
    ($mnemonic:ident, $opcode:expr, $pair:ident) => {
        Instruction {
            mnemonic: stringify!($mnemonic),
            opcode: $opcode,
            cycles: 2,
            length: 1,
            handler: |cpu| {
                let curr = cpu.reg.read_pair(Pair::$pair);
                cpu.reg.write_pair(Pair::$pair, curr - 1);
            },
        }
    };
}

#[macro_export]
macro_rules! xor_reg {
    ($mnemonic:ident, $opcode:expr, $reg:ident) => {
        Instruction {
            mnemonic: stringify!($mnemonic),
            opcode: $opcode,
            cycles: 1,
            length: 1,
            handler: |cpu| cpu.reg.a ^= cpu.reg.$reg,
        }
    };
}

#[macro_export]
macro_rules! and_reg {
    ($mnemonic:ident, $opcode:expr, $reg:ident) => {
        Instruction {
            mnemonic: stringify!($mnemonic),
            opcode: $opcode,
            cycles: 1,
            length: 1,
            handler: |_cpu| todo!(),
        }
    };
}

#[macro_export]
macro_rules! load_imm {
    ($mnemonic:ident, $opcode:expr, $reg:ident) => {
        Instruction {
            mnemonic: stringify!($mnemonic),
            opcode: $opcode,
            cycles: 2,
            length: 2,
            handler: |cpu| cpu.reg.$reg = cpu.mmu.read(cpu.reg.pc + 1),
        }
    };
}

#[macro_export]
macro_rules! add {
    ($mnemonic:ident, $opcode:expr, $reg:ident) => {
        Instruction {
            mnemonic: stringify!($mnemonic),
            opcode: $opcode,
            cycles: 1,
            length: 1,
            handler: |cpu| cpu.add(cpu.reg.$reg, false),
        }
    };
}

#[macro_export]
macro_rules! addc {
    ($mnemonic:ident, $opcode:expr, $reg:ident) => {
        Instruction {
            mnemonic: stringify!($mnemonic),
            opcode: $opcode,
            cycles: 1,
            length: 1,
            handler: |cpu| cpu.add(cpu.reg.$reg, true),
        }
    };
}

#[macro_export]
macro_rules! sub {
    ($mnemonic:ident, $opcode:expr, $reg:ident) => {
        Instruction {
            mnemonic: stringify!($mnemonic),
            opcode: $opcode,
            cycles: 1,
            length: 1,
            handler: |cpu| cpu.sub(cpu.reg.$reg, true),
        }
    };
}

#[macro_export]
macro_rules! subc {
    ($mnemonic:ident, $opcode:expr, $reg:ident) => {
        Instruction {
            mnemonic: stringify!($mnemonic),
            opcode: $opcode,
            cycles: 1,
            length: 1,
            handler: |cpu| cpu.sub(cpu.reg.$reg, true),
        }
    };
}

#[macro_export]
macro_rules! cp_r {
    ($mnemonic:ident, $opcode:expr, $reg:ident) => {
        Instruction {
            mnemonic: stringify!($mnemonic),
            opcode: $opcode,
            cycles: 2,
            length: 1,
            handler: |cpu| {
                let cmp = cpu.reg.a.wrapping_sub(cpu.reg.$reg);
                cpu.set_flag(FlagBit::Z, cmp == 0);
            },
        }
    };
}

#[macro_export]
macro_rules! ld_a8_a {
    ($mnemonic:ident, $opcode:expr) => {
        Instruction {
            mnemonic: stringify!($mnemonic),
            opcode: $opcode,
            cycles: 3,
            length: 2,
            handler: |cpu| {
                let a8 = cpu.mmu.read(cpu.reg.pc + 1) as u16;
                cpu.mmu.write(0xFF00 + a8, cpu.reg.a);
            },
        }
    };
}

#[macro_export]
macro_rules! load_a_8bit {
    ($mnemonic:ident, $opcode:expr, $reg:ident) => {
        Instruction {
            mnemonic: stringify!($mnemonic),
            opcode: $opcode,
            cycles: 1,
            length: 1,
            handler: |cpu| cpu.reg.a = cpu.reg.$reg,
        }
    };
}

#[macro_export]
macro_rules! load_8bit {
    ($mnemonic:ident, $opcode:expr, $from:ident, $to:ident) => {
        Instruction {
            mnemonic: stringify!($mnemonic),
            opcode: $opcode,
            cycles: 1,
            length: 1,
            handler: |cpu| cpu.reg.$to = cpu.reg.$from,
        }
    };
}

#[macro_export]
macro_rules! load_r_into_r {
    ($mnemonic:ident, $opcode:expr, $reg1:ident, $reg2:ident) => {
        Instruction {
            mnemonic: stringify!($mnemonic),
            opcode: $opcode,
            cycles: 1,
            length: 1,
            handler: |cpu| cpu.reg.$reg1 = cpu.reg.$reg2,
        }
    };
}

#[macro_export]
macro_rules! load_16bit {
    ($mnemonic:ident, $opcode:expr, $reg:ident) => {
        Instruction {
            mnemonic: stringify!($mnemonic),
            opcode: $opcode,
            cycles: 3,
            length: 3,
            handler: |cpu| {
                let d16 = cpu.mmu.read_word(cpu.reg.pc + 1);
                cpu.reg.$reg = d16;
            },
        }
    };
}

#[macro_export]
macro_rules! load_16_bit {
    ($mnemonic:ident, $opcode:expr, $reg:ident) => {
        Instruction {
            mnemonic: stringify!($mnemonic),
            opcode: $opcode,
            cycles: 3,
            length: 3,
            handler: |cpu| {
                let d16 = cpu.mmu.read_word(cpu.reg.pc + 1);
                cpu.reg.write_pair(Pair::$reg, d16);
            },
        }
    };
}

#[macro_export]
macro_rules! load_16_into_8 {
    ($mnemonic:ident, $opcode:expr, $reg1:expr, $reg2:ident) => {
        Instruction {
            mnemonic: stringify!($mnemonic),
            opcode: $opcode,
            cycles: 2,
            length: 1,
            handler: |cpu| {
                cpu.reg.$reg2 = cpu.reg.read_pair($reg1) as u8;
            },
        }
    };
}
