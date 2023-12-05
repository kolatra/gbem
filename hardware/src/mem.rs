use std::fs;
use std::io::ErrorKind::InvalidData;

use tracing::{info, trace, warn};

use crate::{Interrupts, Timer, BOOT_ROM, MAX_ROM_SIZE, NINTENDO_HEADER, RAM_SIZE};

trait Memory {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);

    fn read_word(&self, address: u16) -> u16 {
        let upper = self.read(address);
        let lower = self.read(address + 1);

        (upper as u16) << 8 | lower as u16
    }

    fn write_word(&mut self, address: u16, value: u16) {
        let upper = (value >> 8) as u8;
        let lower = value as u8;
        self.write(address, upper);
        self.write(address + 1, lower);
    }
}

#[derive(Debug, Clone, Default)]
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

impl MMU {
    pub fn new() -> Self {
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

#[allow(unused)]
impl MMU {
    const VRAM_START: usize = 0x8000;
    const WRAM_START: usize = 0xC000;
    const ERAM_START: usize = 0xE000;
    const HRAM_START: usize = 0xFF80;

    pub fn read(&self, address: u16) -> u8 {
        let address = address as usize;
        trace!("read: {:#04x}", address);

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

    pub fn write(&mut self, address: u16, value: u8) {
        let address = address as usize;

        match address {
            0x0000..=0x7FFF => self.cartridge[address] = value,
            0x8000..=0x9FFF => self.v_ram[address - Self::VRAM_START] = value,
            0xA000..=0xBFFF => info!(
                "Wrote {:x} to cartridge external RAM at {:x}",
                value, address
            ),
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
            0xFF10..=0xFF26 => info!(
                "Wrote {:x} to sound control registers at {:x}",
                value, address
            ),
            0xFF00..=0xFF7F => info!("Wrote {:x} to i/o registers at {:x}", value, address),
            0xFF80..=0xFFFE => info!("Wrote {:x} to high RAM at {:x}", value, address),
            0xFFFF => self.interrupts.enable = value,
            _ => warn!(
                "Tried to write {:x} to {:x} (outside of address space)",
                value, address
            ),
        }
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let upper = self.read(address);
        let lower = self.read(address + 1);

        (upper as u16) << 8 | lower as u16
    }

    fn write_word(&mut self, address: u16, value: u16) {
        let upper = (value >> 8) as u8;
        let lower = value as u8;
        self.write(address, upper);
        self.write(address + 1, lower);
    }
}

pub fn load_rom(mmu: &mut MMU) -> std::io::Result<()> {
    let rom = "SOME PATH";
    let mut bytes = fs::read(rom)?;

    if bytes.len() < 0x0133 || bytes[0x0104..0x0133] != NINTENDO_HEADER {
        return Err(std::io::Error::new(InvalidData, "Invalid ROM"));
    }

    info!("Loading ROM");
    let mem = &mut mmu.cartridge;
    mem.clear();
    mem.append(&mut bytes);

    Ok(())
}

/// Normally this is mapped into 0x000 but
/// for simplicity we'll just load it into memory
pub fn load_boot_rom(mmu: &mut MMU) {
    let mem = &mut mmu.cartridge;
    info!("Loading boot ROM");
    mem[0x100..0x100 + BOOT_ROM.len()].copy_from_slice(&BOOT_ROM);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mmu() {
        let mut mmu = MMU::new();

        mmu.write_word(0x0000, 0x0001);
        mmu.write_word(0x0002, 0x0203);

        mmu.write_word(0x8000, 0x0405);
        mmu.write_word(0x8002, 0x0607);

        mmu.write_word(0xC000, 0x0809);
        mmu.write_word(0xC002, 0x0A0B);

        assert_eq!(mmu.read_word(0x0000), 0x0001);
        assert_eq!(mmu.read_word(0x0002), 0x0203);

        assert_eq!(mmu.read_word(0x8000), 0x0405);
        assert_eq!(mmu.read_word(0x8002), 0x0607);

        assert_eq!(mmu.read_word(0xC000), 0x0809);
        assert_eq!(mmu.read_word(0xC002), 0x0A0B);
    }
}
