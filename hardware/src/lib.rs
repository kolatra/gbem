#![allow(unused)]
use std::fs;
use core::fmt::Display;
use std::io::ErrorKind::InvalidData;

use tracing::{info, warn, debug, error};

// Debug consts
pub const SPAMMY_LOGS: bool = false;
pub const LOG_LINES: bool = true;

#[cfg(test)]
mod tests;

const CLOCK_FREQ: usize = 4194304; // 4.194304 MHz
const MACHINE_FREQ: usize = 1048576; // 1.048576 MHz - 1/4 of the clock frequency
const FPS: usize = 60;

const RAM_SIZE: usize = 0x2000;
const MAX_ROM_SIZE: usize = 0x8000; // Assumed from the region size in memory

pub const NINTENDO_HEADER: [u8; 48] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

pub const BOOT_ROM: [u8; 256] = [
    0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26, 0xFF, 0x0E,
    0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E, 0xFC, 0xE0,
    0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0xCD, 0x95, 0x00, 0xCD, 0x96, 0x00, 0x13, 0x7B,
    0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0x00, 0x06, 0x08, 0x1A, 0x13, 0x22, 0x23, 0x05, 0x20, 0xF9,
    0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x3D, 0x28, 0x08, 0x32, 0x0D, 0x20,
    0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64, 0x57, 0xE0, 0x42, 0x3E, 0x91, 0xE0, 0x40, 0x04,
    0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90, 0x20, 0xFA, 0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2,
    0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62, 0x28, 0x06, 0x1E, 0xC1, 0xFE, 0x64, 0x20, 0x06,
    0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xE2, 0xF0, 0x42, 0x90, 0xE0, 0x42, 0x15, 0x20, 0xD2, 0x05, 0x20,
    0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F, 0x06, 0x04, 0xC5, 0xCB, 0x11, 0x17, 0xC1, 0xCB, 0x11, 0x17,
    0x05, 0x20, 0xF5, 0x22, 0x23, 0x22, 0x23, 0xC9, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
    0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
    0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
    0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x3C, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x3C,
    0x21, 0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13, 0xBE, 0x20, 0xFE, 0x23, 0x7D, 0xFE, 0x34, 0x20,
    0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x20, 0xFE, 0x3E, 0x01, 0xE0, 0x50,
];

pub const JUMP_VECTORS: [u8; 12] = [
    0x00, 0x08, 0x10, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58, 0x60,
];

#[derive(Debug, Clone, Copy)]
enum ProgramCounter {
    Next,
    Skip(u8),
    Pause,
}

#[derive(Debug, Clone)]
struct Instruction {
    mnemonic: &'static str,
    opcode: u32,
    cycles: i8,
    length: i8,
    handler: fn(cpu: &mut CPU) -> ProgramCounter,
}

impl Instruction {
    fn run(&self, cpu: &mut CPU) -> ProgramCounter {
        (self.handler)(cpu)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FlagBit {
    Z = 7,
    N = 6,
    H = 5,
    C = 4,
}

#[derive(Debug, Clone, Copy)]
pub struct Registers {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,

    pc: u16,
    sp: u16,
}

impl Default for Registers {
    fn default() -> Self {
        Self {
            a: 0x01,
            // FIXME:
            // If the header checksum is $00, 
            // then the carry and half-carry flags are clear;
            // otherwise, they are both set. 
            // Always set the Z flag.
            f: 0x80,
            b: 0x00,
            c: 0x13,
            d: 0x00,
            e: 0xD8,
            h: 0x01,
            l: 0x4D,

            pc: 0x0000,
            sp: 0x0000,
            // sp: 0xFFFE,
        }
    }
}

const STACK_SIZE: usize = 65536; // TODO find a real stack size
#[derive(Debug, Clone)]
pub struct CPU {
    reg: Registers,
    pub mmu: MMU,
    stack: [u16; STACK_SIZE],
}

impl CPU {
    pub fn new() -> Self {
        Self {
            reg: Registers::default(),
            mmu: MMU::default(),
            stack: [0; STACK_SIZE],
        }
    }

    fn push_stack(&mut self, value: u16) {
        self.stack[self.reg.sp as usize] = value;
        self.reg.sp -= 1;
    }

    fn pop_stack(&mut self) -> u16 {
        let v = self.stack[self.reg.sp as usize];
        self.reg.sp += 1;
        v
    }

    fn fetch(&self) -> u8 {
        debug!("fetch");
        debug!("pc: {:#04x}", self.reg.pc);
        debug!("sp: {:#04x}", self.reg.sp);
        let pc = self.reg.pc;
        let opcode = self.mmu.read(pc);
        opcode
    }

    pub fn cycle(&mut self) {
        debug!("Cycle");
        let opcode = self.fetch();
        debug!("opcode: {:#04x}", opcode);
        let instructions = get_instructions();
        let instruction = instructions
            .iter()
            .find(|i| i.opcode == opcode as u32);

        
        debug!("{:?}", instruction);
        let pc = match instruction {
            Some(i) => {
                // debug stuff
                let a = &self.mmu.cartridge[self.reg.pc as usize..self.reg.pc as usize + i.length as usize];
                let out: String = a.iter().map(|b| format!("{:#02x} ", b)).collect();
                debug!(out);

                i.run(self)
            },
            None => panic!("Unknown opcode: {:#04x}", opcode)
        };

        match pc {
            ProgramCounter::Next => self.reg.pc += 2,
            ProgramCounter::Skip(i) => self.reg.pc += i as u16,
            ProgramCounter::Pause => warn!(opcode, self.reg.pc, "paused"),
        };

        if SPAMMY_LOGS {
            self.print_reg();
        }
    }

    fn reset(&mut self) {
        self.reg = Registers::default();
    }

    fn set_flag(&mut self, flag: FlagBit, value: bool) {
        let bit = flag as u8;
        let mask = 1 << bit;
        self.reg.f = (self.reg.f & !mask) | ((value as u8) << bit);
    }

    fn is_set(&self, flag: FlagBit) -> bool {
        let bit = flag as u8;
        let mask = 1 << bit;
        self.reg.f & mask > 0
    }

    /// https://robdor.com/2016/08/10/gameboy-emulator-half-carry-flag/
    fn add(&mut self, b: u8, use_carry: bool) {
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

    fn sub(&mut self, b: u8, use_carry: bool) {
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

    fn print_reg(&self) {
        info!("[****************************************************]");
        info!("Registers (hex):");
        info!(
            "A: {:#04x} F: {:#04x} B: {:#04x} C: {:#04x} D: {:#04x} E: {:#04x} H: {:#04x} L: {:#04x}",
            self.reg.a,
            self.reg.f,
            self.reg.b,
            self.reg.c,
            self.reg.d,
            self.reg.e,
            self.reg.h,
            self.reg.l
        );

        info!("Registers (bin):");
        info!(
            "A: {:#010b} F: {:#010b} B: {:#010b} C: {:#010b} D: {:#010b} E: {:#010b} H: {:#010b} L: {:#010b}",
            self.reg.a,
            self.reg.f,
            self.reg.b,
            self.reg.c,
            self.reg.d,
            self.reg.e,
            self.reg.h,
            self.reg.l
        );
        info!("[****************************************************]");
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct Timer {
    counter: u8,
    modulo: u8,
    control: u8,
}

#[derive(Debug, Default, Clone, Copy)]
struct Interrupts {
    enable: u8,
    flag: u8,
}

#[derive(Debug, Clone)]
pub struct MMU {
    w_ram: Vec<u8>, // Work RAM
    v_ram: Vec<u8>, // Video RAM
    pub cartridge: Vec<u8>,
    timer: Timer,
    // https://gbdev.io/pandocs/Joypad_Input.html#ff00--p1joyp-joypad
    joypad: u8,
    // https://gbdev.io/pandocs/Timer_and_Divider_Registers.html#ff04--div-divider-register
    divider_reg: u8,
    // https://gbdev.io/pandocs/Interrupts.html#ff0f--if-interrupt-flag
    interrupts: Interrupts,
}

impl Default for MMU {
    fn default() -> Self {
        Self {
            w_ram: vec![0; RAM_SIZE],
            v_ram: vec![0; RAM_SIZE],
            cartridge: vec![0; MAX_ROM_SIZE],
            timer: Timer::default(),
            joypad: 0,
            divider_reg: 0,
            interrupts: Interrupts::default(),
        }
    }
}

impl MMU {
    const VRAM_START: usize = 0x8000;
    const WRAM_START: usize = 0xC000;
    const ERAM_START: usize = 0xE000;

    fn read(&self, address: u16) -> u8 {
        let address = address as usize;
        debug!("read: {:#04x}", address);

        match address {
            0x0000..=0x7FFF => self.cartridge[address],
            0x8000..=0x9FFF => self.v_ram[address - Self::VRAM_START],
            0xA000..=0xBFFF => 1, // Cartridge external RAM
            0xC000..=0xDFFF => self.w_ram[address - Self::WRAM_START],
            0xE000..=0xFDFF => 1, // Echo RAM
            0xFE00..=0xFE9F => 1, // Object attribute memory
            0xFEA0..=0xFEFF => 1, // Not usable
            0xFF00 => self.joypad,
            0xFF04 => self.divider_reg,
            0xFF05 => self.timer.counter,
            0xFF06 => self.timer.modulo,
            0xFF07 => self.timer.control,
            0xFF0F => self.interrupts.flag,
            0xFF10..=0xFF26 => 1, // Sound control registers
            0xFF00..=0xFF7F => 1, // I/O registers
            0xFF80..=0xFFFE => 1, // High RAM
            0xFFFF => self.interrupts.enable,
            _ => 0,
        }
    }

    fn write(&mut self, address: u16, value: u8) {
        let address = address as usize;

        match address {
            0x0000..=0x7FFF => self.cartridge[address] = value,
            0x8000..=0x9FFF => self.v_ram[address - Self::VRAM_START] = value,
            0xA000..=0xBFFF => info!("Wrote {:x} to cartridge external RAM at {:x}", value, address),
            0xC000..=0xDFFF => self.w_ram[address - Self::WRAM_START] = value,
            0xE000..=0xFDFF => info!("Wrote {:x} to echo RAM at {:x}", value, address),
            0xFE00..=0xFE9F => info!("Wrote {:x} to OAM at {:x}", value, address),
            0xFEA0..=0xFEFF => warn!("Tried to write into unusable memory at {:x}", address),
            0xFF00 => self.joypad = value,
            0xFF04 => self.divider_reg = value,
            0xFF05 => self.timer.counter = value,
            0xFF06 => self.timer.modulo = value,
            0xFF07 => self.timer.control = value,
            0xFF0F => self.interrupts.flag = value,
            0xFF10..=0xFF26 => info!("Wrote {:x} to sound control registers at {:x}", value, address),
            0xFF00..=0xFF7F => info!("Wrote {:x} to i/o registers at {:x}", value, address),
            0xFF80..=0xFFFE => info!("Wrote {:x} to high RAM at {:x}", value, address),
            0xFFFF => self.interrupts.enable = value,
            _ => warn!("Tried to write {:x} to {:x} (outside of address space)", value, address),
        }
    }

    fn read_word(&self, address: u16) -> u16 {
        let upper = self.read(address);
        let lower = self.read(address + 1);
        let word  = (upper as u16) << 8 | lower as u16;
        word
    }

    fn write_word(&mut self, address: u16, value: u16) {
        let upper = (value >> 8) as u8;
        let lower = value as u8;
        self.write(address, upper);
        self.write(address + 1, lower);
    }
}

// https://meganesu.github.io/generate-gb-opcodes/
// https://gekkio.fi/files/gb-docs/gbctr.pdf
fn get_instructions() -> Vec<Instruction> {
    vec![
        Instruction {
            mnemonic: "NOP",
            opcode: 0x00,
            cycles: 1,
            length: 1,
            handler: |_| ProgramCounter::Next,
        },
        Instruction {
            mnemonic: "LD SP, d16",
            opcode: 0x31,
            cycles: 3,
            length: 3,
            handler: |cpu| {
                let d16 = cpu.mmu.read_word(cpu.reg.pc + 1);
                cpu.reg.sp = d16;
                ProgramCounter::Skip(3)
            },
        },  
        Instruction {
            mnemonic: "ADD A,B",
            opcode: 0x80,
            cycles: 1,
            length: 1,
            handler: |cpu| {
                cpu.add(cpu.reg.b, false);
                ProgramCounter::Next
            },
        },
        Instruction {
            mnemonic: "SUB A,B",
            opcode: 0x90,
            cycles: 1,
            length: 1,
            handler: |cpu| {
                cpu.sub(cpu.reg.b, false);
                ProgramCounter::Next
            },
        },
        Instruction {
            mnemonic: "XOR A",
            opcode: 0xAF,
            cycles: 1,
            length: 1,
            handler: |cpu| {
                cpu.reg.a = (cpu.reg.a ^ cpu.reg.a);
                ProgramCounter::Next
            },
        },
        Instruction {
            mnemonic: "STOP",
            opcode: 0x1000,
            cycles: 1,
            length: 2,
            handler: |_| {
                // The system clock/oscillator stops until either:
                // A reset
                // Joypad input - resume execution at pc+1
                ProgramCounter::Pause
            },
        },
        Instruction {
            mnemonic: "HALT",
            opcode: 0x76,
            cycles: 1,
            length: 1,
            handler: |_| {
                // The clock stops but the oscillator and LCD controller continue to operate
                // until an interrupt occurs.
                ProgramCounter::Pause
            },
        },
        Instruction {
            mnemonic: "RST 7",
            opcode: 0xFF,
            cycles: 4,
            length: 1,
            handler: |cpu| {
                // push the program counter onto the stack
                cpu.push_stack(cpu.reg.pc);
                cpu.reg.pc = 0x38;

                ProgramCounter::Pause
            },
        },
    ]
}

pub fn load_rom(mmu: &mut MMU) -> std::io::Result<()> {
    let rom = "SOME PATH";
    let mut bytes = fs::read(rom)?;

    if bytes.len() < 0x0133 || &bytes[0x0104..0x0133] != NINTENDO_HEADER {
        return Err(std::io::Error::new(
            InvalidData,
            "Invalid ROM",
        ));
    }

    info!("Loading ROM");
    let mem = &mut mmu.cartridge;
    mem.clear();
    mem.append(&mut bytes);

    Ok(())
}

pub fn load_boot_rom(mmu: &mut MMU) {
    let mem = &mut mmu.cartridge;
    info!("Loading boot ROM");
    mem[..BOOT_ROM.len()].copy_from_slice(&BOOT_ROM);
}
