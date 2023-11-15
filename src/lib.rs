#![allow(unused)]

use std::fs;
use std::ops::Range;

const CLOCK_FREQ: usize = 4194304; // 4.194304 MHz
const MACHINE_FREQ: usize = 1048576; // 1.048576 MHz - 1/4 of the clock frequency
const FPS: usize = 60;

const RAM_SIZE: usize = 0x2000;
const ADDRESS_SPACE: Range<u16> = 0x0000..0xFFFF;

fn jump_vectors() -> Vec<u8> {
    vec![
        // RST
        0x0000, 0x0008, 0x0010, 0x0020, 0x0028, 0x0030, 0x0038,
        // Interrupts
        0x0040, 0x0048, 0x0050, 0x0058, 0x0060
    ]
}

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

/// Listen for memory management orders from the CPU
#[derive(Debug)]
struct MMU {
    memory: Vec<u8>, // allocate this at the start and use it like an arena
    cartridge: Vec<u8>,
}

impl MMU {
    fn read(&self, address: u16) -> u8 {
        match address {
            0x0000..=0xDFFF => 1, // Cartridge RAM
            0xE000..=0xFDFF => 0, // Echo RAM
            0xFE00..=0xFE9F => 1, // Object attribute memory
            0xFEA0..=0xFEFF => 0, // Not usable
            0xFF00..=0xFF7F => 1, // I/O registers
            0xFF80..=0xFFFE => 1, // High RAM
            0xFFFF => 1, // Interrupt enable register
            _ => 0
        }
    }
}

fn load_boot_rom(mem: &mut Vec<u8>) -> std::io::Result<()> {
    let rom = "./DMG_ROM.bin";
    let mut bytes = fs::read(rom)?;

    mem.clear();
    mem.append(&mut bytes);

    Ok(())
}

pub fn its_a_gameboy() {
    let mut mmu = MMU {
        memory: Vec::new(),
        cartridge: Vec::new()
    };
    let cpu = CPU {
        registers: Registers::default()
    };

    load_boot_rom(&mut mmu.memory);

    println!("read in a {} size ROM", mmu.memory.len());
}
