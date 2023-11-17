#![allow(unused)]

use std::fs;
use std::ops::Range;

const CLOCK_FREQ: usize = 4194304; // 4.194304 MHz
const MACHINE_FREQ: usize = 1048576; // 1.048576 MHz - 1/4 of the clock frequency
const FPS: usize = 60;

const RAM_SIZE: usize = 0x2000;
const ADDRESS_SPACE: Range<u16> = 0x0000..0xFFFF;

mod big_ass_arrays;
pub use big_ass_arrays::*;

#[derive(Debug, Default)]
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

/// Decode and execute all instructions
#[derive(Debug)]
struct CPU {
    registers: Registers,
}

#[derive(Debug, Default)]
struct Timer {
    counter: u8,
    modulo: u8,
    control: u8,
}

#[derive(Debug, Default)]
struct Interrupts {
    enable: u8,
    flag: u8,
}

/// Listen for memory management orders from the CPU
#[derive(Debug)]
struct MMU {
    w_ram: Vec<u8>, // Work RAM
    v_ram: Vec<u8>, // Video RAM
    cartridge: Vec<u8>,
    timer: Timer,
    // https://gbdev.io/pandocs/Joypad_Input.html#ff00--p1joyp-joypad
    joypad: u8,
    // https://gbdev.io/pandocs/Timer_and_Divider_Registers.html#ff04--div-divider-register
    divider_reg: u8,
    // https://gbdev.io/pandocs/Interrupts.html#ff0f--if-interrupt-flag
    interrupts: Interrupts,
}

impl MMU {
    fn read(&self, address: u16) -> u8 {
        const VRAM_START: u16 = 0x8000;
        const WRAM_START: u16 = 0xC000;

        match address {
            0x0000..=0x7FFF => self.cartridge[address as usize],
            0x8000..=0x9FFF => self.v_ram[(address - VRAM_START) as usize],
            0xA000..=0xBFFF => 1, // Cartridge external RAM
            0xC000..=0xDFFF => self.w_ram[(address - WRAM_START) as usize],
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
}

fn load_rom(mmu: &mut MMU) -> std::io::Result<()> {
    let rom = "./DMG_ROM.bin";
    let mut bytes = fs::read(rom)?;

    let logo = &bytes[0x0104..0x0133];
    if logo != NINTENDO_HEADER {
        eprintln!("Invalid Gameboy ROM!");
    } else {
        println!("Loading ROM");
        let mem = &mut mmu.cartridge;
        mem.clear();
        mem.append(&mut bytes);
    }

    Ok(())
}

pub fn its_a_gameboy() {
    let mut mmu = MMU {
        w_ram: Vec::with_capacity(RAM_SIZE),
        v_ram: Vec::with_capacity(RAM_SIZE),
        cartridge: Vec::new(),
        timer: Timer::default(),
        joypad: 0,
        divider_reg: 0,
        interrupts: Interrupts { enable: 0, flag: 0 },
    };

    let cpu = CPU {
        registers: Registers::default(),
    };

    dbg!(load_rom(&mut mmu));
}
