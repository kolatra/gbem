use crate::{
    get_instructions, FlagBit, Instruction, ProgramCounter, Registers, GPU, MMU, SPAMMY_LOGS,
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

    pub fn fetch(&self) -> u8 {
        trace!("fetch");
        debug!("pc: {:#04x}", self.reg.pc);
        debug!("sp: {:#04x}", self.reg.sp);
        let pc = self.reg.pc;
        self.mmu.read(pc)
    }

    fn dbg_print_bytes(&self, i: &Instruction) {
        let pc = self.reg.pc as usize;
        let a = &self.mmu.cartridge[pc..pc + i.length as usize];
        #[allow(clippy::format_collect)]
        let instr_bytes: String = a.iter().map(|b| format!("{:#02x} ", b)).collect();
        debug!(instr_bytes);
    }

    pub fn cycle(&mut self) {
        trace!("cycle");
        let opcode = self.fetch();
        debug!("opcode: {:#04x}", opcode);
        let instructions = get_instructions();
        let instruction = instructions.iter().find(|i| i.opcode == opcode as u16);

        debug!("{:?}", instruction);
        let pc = match instruction {
            Some(i) => {
                self.dbg_print_bytes(i);
                i.run(self)
            }

            None => panic!("Unknown opcode: {:#04x}", opcode),
        };

        debug!("pc: {:?}", pc);
        match pc {
            ProgramCounter::Next => self.reg.pc += 2,
            ProgramCounter::Skip(i) => self.reg.pc += i as u16,
            ProgramCounter::Pause => warn!(opcode, self.reg.pc, "paused"),
        };

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
