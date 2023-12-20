#![allow(unused)]
use std::io::ErrorKind::InvalidData;
use std::sync::{Mutex, RwLock};
use std::{fs, sync::Arc};

use tracing::{debug, error, info, trace, warn};

use crate::{Interrupts, Timer, BOOT_ROM, MAX_ROM_SIZE, NINTENDO_HEADER, RAM_SIZE};

use self::cart::Cartridge;

pub mod cart;

pub type Device<T> = Arc<RwLock<T>>;

struct GenericDevice {
    mem: Vec<u8>,
}

impl Memory for GenericDevice {
    fn read(&self, address: u16) -> u8 {
        self.mem[address as usize]
    }

    fn write(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }

    fn read_range(&self, start: u16, end: u16) -> &[u8] {
        &self.mem[start as usize..end as usize]
    }

    fn write_range(&mut self, start: u16, end: u16, value: &[u8]) {
        self.mem[start as usize..end as usize].copy_from_slice(value);
    }
}

impl Default for GenericDevice {
    fn default() -> Self {
        Self { mem: vec![0; 4096] }
    }
}

pub trait Memory {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
    fn read_range(&self, start: u16, end: u16) -> &[u8];
    fn write_range(&mut self, start: u16, end: u16, value: &[u8]);

    fn read_word(&self, address: u16) -> u16 {
        let upper = self.read(address);
        let lower = self.read(address + 1);

        (lower as u16) << 8 | upper as u16
    }

    fn write_word(&mut self, address: u16, value: u16) {
        let upper = (value >> 8) as u8;
        let lower = value as u8;
        self.write(address, lower);
        self.write(address + 1, upper);
    }
}

// TODO: Look at using arenas instead of Vec?
// https://crates.io/crates/generational-arena
#[derive(Debug, Clone, Default)]
pub struct MMU {
    w_ram: Vec<u8>, // Work RAM
    v_ram: Vec<u8>, // Video RAM
    pub cartridge: Device<Cartridge>,
    stack: Vec<u8>, // HRAM
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
            cartridge: Arc::new(RwLock::new(Cartridge::new())),
            // 126 bytes for the size of the stack
            stack: vec![0; 0x7E],
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
    const STACK_START: usize = 0xFF80;

    pub fn read(&self, address: u16) -> u8 {
        debug!("read: {:#04x}", address);
        match self.get_device(address) {
            Some(d) => {
                let lock = d.read().unwrap();
                lock.read(address)
            }
            None => 0,
        }
    }

    pub fn write(&mut self, address: u16, value: u8) {
        debug!("write: {:#04x} {:#04x}", address, value);
        if let Some(device) = self.get_device(address) {
            let mut lock = device.write().unwrap();
            lock.write(address, value);
        }
    }

    // TODO: This won't have to return Option once we have all the devices implemented.
    #[rustfmt::skip]
    fn get_device(&self, address: u16) -> Option<Arc<RwLock<dyn Memory>>> {
        match address {
            0x0000..=0x7FFF => Some(self.cartridge.clone()),
            0x8000..=0x9FFF => { error!("Video RAM is not implemented"); None }
            0xA000..=0xBFFF => { error!("Cartridge external RAM is not implemented"); None }
            0xC000..=0xDFFF => { error!("Work RAM is not implemented"); None }
            0xE000..=0xFDFF => { error!("Echo RAM is not implemented"); None }
            0xFE00..=0xFE9F => { error!("Object attribute memory is not implemented"); None }
            0xFEA0..=0xFEFF => { error!("Not usable is not implemented"); None }
            0xFF00 => { error!("Joypad is not implemented"); None }
            0xFF04 => { error!("Divider register is not implemented"); None }
            0xFF05 => { error!("Timer counter is not implemented"); None }
            0xFF06 => { error!("Timer modulo is not implemented"); None }
            0xFF07 => { error!("Timer control is not implemented"); None }
            0xFF0F => { error!("Interrupt flag is not implemented"); None }
            0xFF10..=0xFF26 => { error!("Sound control registers are not implemented"); None }
            0xFF00..=0xFF7F => { error!("I/O registers are not implemented"); None }
            0xFF80..=0xFFFE => { error!("HRAM is not implemented"); None }
            0xFFFF => { error!("Interrupt enable is not implemented"); None }
            _ => panic!(
                "Tried to get device at {:x} (outside of address space)",
                address
            ),
        }
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let upper = self.read(address);
        let lower = self.read(address + 1);

        // In 2-byte instructions, the first byte of immediate
        // data is the lower byte and the second byte is
        // the upper byte.
        (lower as u16) << 8 | upper as u16
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        let upper = (value >> 8) as u8;
        let lower = value as u8;
        self.write(address, lower);
        self.write(address + 1, upper);
    }
}

pub fn load_rom(mmu: &mut MMU) -> std::io::Result<()> {
    // FIXME: The ROM is hardcoded for now, but we can make it more dynamic.
    // The ROM "check" is also disabled to make the test load.
    let rom = "./disassembler/DMG_ROM.bin";
    // let rom = "./test-roms/blargg/mem_timing/mem_timing.gb";
    let mut bytes = fs::read(rom)?;

    if bytes.len() < 0x0133 || bytes[0x0104..0x0133] != NINTENDO_HEADER {
        error!("Invalid ROM");
        // return Err(std::io::Error::new(InvalidData, "Invalid ROM"));
    }

    info!("Loading ROM");
    let arc = mmu.cartridge.clone();
    let mut cart = arc.write().unwrap();
    cart.write_range(0, bytes.len() as u16, &bytes);

    Ok(())
}

pub fn load_boot_rom(mmu: &mut MMU) {
    trace!("Loading boot ROM");
    let arc = mmu.cartridge.clone();
    let mut cart = arc.write().unwrap();
    cart.write_range(0, BOOT_ROM.len() as u16, &BOOT_ROM);
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
