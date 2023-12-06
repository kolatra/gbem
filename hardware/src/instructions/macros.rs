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
macro_rules! load_8bit {
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
