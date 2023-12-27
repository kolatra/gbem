use crate::{
    instructions::{Instruction, INSTRUCTIONS},
    mem::{Memory, MMU},
    reg::FlagBit,
    reg::Registers,
    GPU,
};
use tracing::{debug, info, trace};

#[derive(Debug, Clone, Default)]
pub struct CPU {
    pub reg: Registers,
    pub mmu: MMU,
    pub gpu: GPU,
}

impl CPU {
    pub fn new() -> Self {
        Self {
            reg: Registers::new(),
            mmu: MMU::new(),
            gpu: GPU::new(),
        }
    }
}

impl CPU {
    pub fn push_stack(&mut self, value: u8) {
        trace!("push_stack");
        self.reg.sp -= 1;
        self.mmu.write(self.reg.sp, value);
    }

    pub fn pop_stack(&mut self) -> u8 {
        trace!("pop_stack");
        let value = self.mmu.read(self.reg.sp);
        self.reg.sp += 1;
        value
    }

    pub fn fetch(&self) -> Instruction {
        trace!("fetch");
        debug!("pc: {:#04x}", self.reg.pc);
        debug!("sp: {:#04x}", self.reg.sp);

        let pc = self.reg.pc;
        let mut opcode = self.mmu.read(pc);

        if opcode == 0xCB {
            info!("CB prefix");
            opcode = self.mmu.read(pc + 1);
        }

        INSTRUCTIONS
            .iter()
            .find(|i| i.opcode == opcode.into())
            .map_or_else(
                || panic!("Unknown opcode: {opcode:#04x}"),
                |i| {
                    self.dbg_print_bytes(i);
                    debug!("opcode: {:#04x}", opcode);
                    *i
                },
            )
    }

    pub fn read_byte(&self) -> u8 {
        trace!("read_byte");
        self.mmu.read(self.reg.pc)
    }

    pub fn read_next_byte(&self) -> u8 {
        trace!("read_next_byte");
        self.mmu.read(self.reg.pc + 1)
    }

    #[allow(clippy::significant_drop_tightening)] // It doesn't actually apply here
    fn dbg_print_bytes(&self, i: &Instruction) {
        let pc = self.reg.pc;
        let cartridge = &self.mmu.cartridge.read().unwrap();
        let ins_bytes = cartridge.read_range(pc, pc + i.length);
        let out = ins_bytes
            .iter()
            .fold(String::new(), |s, b| s + &format!("{b:#02x} "));
        debug!(out);
    }

    pub fn cycle(&mut self) {
        trace!("cycle");
        let instruction = self.fetch();

        debug!(
            "{} - cycles: {} length: {}",
            instruction.mnemonic, instruction.cycles, instruction.length
        );
        instruction.run(self);

        if instruction.length == 1 {
            self.reg.pc += 2;
        } else {
            self.reg.pc += instruction.length;
        }

        self.print_reg();
    }

    pub fn reset(&mut self) {
        trace!("reset");
        self.reg = Registers::new();
    }

    pub fn set_flag(&mut self, flag: FlagBit, value: bool) {
        trace!("set_flag");
        let bit = flag as u8;
        let mask = 1 << bit;
        self.reg.f = (self.reg.f & !mask) | (u8::from(value) << bit);
    }

    pub fn is_set(&self, flag: FlagBit) -> bool {
        trace!("is_set");
        let bit = flag as u8;
        let mask = 1 << bit;
        self.reg.f & mask > 0
    }

    /// https://robdor.com/2016/08/10/gameboy-emulator-half-carry-flag/
    #[rustfmt::skip]
    pub fn add(&mut self, b: u8, use_carry: bool) {
        use FlagBit::*;
        trace!("add");
        let a   = self.reg.a;
        let c   = u8::from(use_carry && self.is_set(C));
        let hc  = (((a & 0xF) + (b & 0xF)) & 0x10) == 0x10;
        let r   = a.wrapping_add(b).wrapping_add(c);
        let a16 = u16::from(a);
        let b16 = u16::from(b);
        let c16 = u16::from(c);

        self.set_flag(Z, r == 0);
        self.set_flag(N, false);
        self.set_flag(H, hc);
        self.set_flag(C, (a16 + b16 + c16) > 0xFF);
        self.reg.a = r;
    }

    #[rustfmt::skip]
    pub fn sub(&mut self, b: u8, use_carry: bool) {
        use FlagBit::*;
        trace!("sub");
        let a   = self.reg.a;
        let c   = u8::from(use_carry && self.is_set(C));
        let hc  = (((a & 0xF) + (b & 0xF)) & 0x10) == 0x10;
        let r   = a.wrapping_sub(b).wrapping_sub(c);
        let a16 = u16::from(a);
        let b16 = u16::from(b);
        let c16 = u16::from(c);

        self.set_flag(Z, r == 0);
        self.set_flag(N, true);
        self.set_flag(H, hc);
        let result = a16.checked_sub(b16).and_then(|b16| b16.checked_sub(c16));
        self.set_flag(C, result.is_none());
        self.reg.a = r;
    }

    pub fn print_reg(&self) {
        if crate::LOG_REGISTERS {
            trace!("Registers (hex):");
            trace!("A: {:#04x}     F: {:#04x}", self.reg.a, self.reg.f);
            trace!("B: {:#04x}     C: {:#04x}", self.reg.b, self.reg.c);
            trace!("D: {:#04x}     E: {:#04x}", self.reg.d, self.reg.e);
            trace!("H: {:#04x}     L: {:#04x}", self.reg.h, self.reg.l);

            trace!("Registers (bin):");
            trace!("A: {:08b} F: {:08b}", self.reg.a, self.reg.f);
            trace!("B: {:08b} C: {:08b}", self.reg.b, self.reg.c);
            trace!("D: {:08b} E: {:08b}", self.reg.d, self.reg.e);
            trace!("H: {:08b} L: {:08b}", self.reg.h, self.reg.l);
        }
    }
}
