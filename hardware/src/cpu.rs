use crate::{
    FlagBit, Registers, GPU, MMU, SPAMMY_LOGS, instructions::{self, Instruction},
};
use tracing::{debug, info, trace, warn};

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
        let opcode = self.mmu.read(pc);

        match instructions::get().iter().find(|i| i.opcode == opcode.into()) {
            Some(i) => {
                self.dbg_print_bytes(i);
                debug!("opcode: {:#04x}", opcode);
                i.clone()
            },
            None => panic!("Unknown opcode: {:#04x}", opcode),
        }
    }

    fn dbg_print_bytes(&self, i: &Instruction) {
        let pc = self.reg.pc as usize;
        let ins_bytes = &self.mmu.cartridge[pc..pc + i.length as usize];
        let out = ins_bytes.iter().fold(String::new(), |s, b| s + &format!("{:#02x} ", b));
        debug!(out);
    }

    pub fn cycle(&mut self) {
        trace!("cycle");
        let opcode = self.fetch();

        debug!("{:?}", opcode);
        opcode.run(self);

        if opcode.length == 1 {
            self.reg.pc += 2;
        } else {
            self.reg.pc += opcode.length;
        }

        self.print_reg()
    }

    pub fn reset(&mut self) {
        trace!("reset");
        self.reg = Registers::new();
    }

    pub fn set_flag(&mut self, flag: FlagBit, value: bool) {
        trace!("set_flag");
        let bit = flag as u8;
        let mask = 1 << bit;
        self.reg.f = (self.reg.f & !mask) | ((value as u8) << bit);
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
        trace!("add");
        let a   = self.reg.a;
        let c   = if use_carry && self.is_set(C) { 1 } else { 0 };
        let hc  = (((a & 0xF) + (b & 0xF)) & 0x10) == 0x10;
        let r   = a.wrapping_add(b).wrapping_add(c);
        let a16 = a as u16;
        let b16 = b as u16;
        let c16 = c as u16;

        use FlagBit::*;
        self.set_flag(Z, r == 0);
        self.set_flag(N, false);
        self.set_flag(H, hc);
        self.set_flag(C, (a16 + b16 + c16) > 0xFF);
        self.reg.a = r;
    }

    #[rustfmt::skip]
    pub fn sub(&mut self, b: u8, use_carry: bool) {
        trace!("sub");
        let a   = self.reg.a;
        let c   = if use_carry && self.is_set(C) { 1 } else { 0 };
        let hc  = (((a & 0xF) + (b & 0xF)) & 0x10) == 0x10;
        let r   = a.wrapping_sub(b).wrapping_sub(c);
        let a16 = a as u16;
        let b16 = b as u16;
        let c16 = c as u16;

        use FlagBit::*;
        self.set_flag(Z, r == 0);
        self.set_flag(N, true);
        self.set_flag(H, hc);
        let result = a16.checked_sub(b16).and_then(|b16| b16.checked_sub(c16));
        self.set_flag(C, result.is_none());
        self.reg.a = r;
    }

    pub fn print_reg(&self) {
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
