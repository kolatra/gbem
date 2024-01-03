#![allow(dead_code)]
use std::io::ErrorKind::InvalidData;
use std::sync::RwLock;
use std::{fs, sync::Arc};

use tracing::{debug, error, info, trace};

use crate::ram::{MemoryRegion, Region};
use crate::{ram::RamSize::*, ram::RamStart::*, Interrupts, Timer, BOOT_ROM, NINTENDO_HEADER};

pub trait Memory {
    fn read(&self, address: u16) -> u8;
    fn write(&mut self, address: u16, value: u8);
    fn read_range(&self, start: u16, end: u16) -> &[u8];
    fn write_range(&mut self, start: u16, end: u16, value: &[u8]);

    fn read_word(&self, address: u16) -> u16 {
        let upper = self.read(address);
        let lower = self.read(address + 1);

        u16::from(lower) << 8 | u16::from(upper)
    }

    fn write_word(&mut self, address: u16, value: u16) {
        let upper = (value >> 8) as u8;
        let lower = value as u8;
        self.write(address, lower);
        self.write(address + 1, upper);
    }
}

#[derive(Debug, Clone, Default)]
pub struct MMU {
    cart: Region,
    vram: Region,
    wram: Region,
    hram: Region,
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
            cart: MemoryRegion::new(u16::from(KB32), u16::from(Cart)),
            vram: MemoryRegion::new(u16::from(KB8), u16::from(VRam)),
            wram: MemoryRegion::new(u16::from(KB8), u16::from(WRam)),
            hram: MemoryRegion::new(126, u16::from(HRam)),
            timer: Timer::default(),
            joypad: 0,
            divider_reg: 0,
            interrupts: Interrupts::default(),
        }
    }
}

#[allow(unused)]
impl MMU {
    pub fn read(&self, address: u16) -> u8 {
        debug!("read: {:#04x}", address);
        self.get_region(address).map_or(0, |lock| {
            let region = lock.read().unwrap();
            region.read(address)
        })
    }

    pub fn write(&mut self, address: u16, value: u8) {
        debug!("write: {:#04x} {:#04x}", address, value);
        if let Some(lock) = self.get_region(address) {
            let mut region = lock.write().unwrap();
            region.write(address, value);
        }
    }

    // TODO: This won't have to return Option once we have all the devices implemented.
    #[rustfmt::skip]
    fn get_region(&self, address: u16) -> Option<Arc<RwLock<MemoryRegion>>> {
        match address {
            0x0000..=0x7FFF => Some(Arc::clone(&self.cart)),
            0x8000..=0x9FFF => Some(Arc::clone(&self.vram)),
            0xA000..=0xBFFF => { error!(address, "Cartridge external RAM is not implemented"); None }
            0xC000..=0xDFFF => Some(Arc::clone(&self.wram)),
            0xE000..=0xFDFF => { error!(address, "Echo RAM is not implemented"); None }
            0xFE00..=0xFE9F => { error!(address, "Object attribute memory is not implemented"); None }
            0xFEA0..=0xFEFF => { error!(address, "Not usable"); None }
            0xFF00 => { error!(address, "Joypad is not implemented"); None }
            0xFF04 => { error!(address, "Divider register is not implemented"); None }
            0xFF05..=0xFF07 => { error!(address, "Timer is not implemented"); None }
            0xFF0F => { error!(address, "Interrupt flag is not implemented"); None }
            0xFF10..=0xFF26 => { error!(address, "Sound control registers are not implemented"); None }
            0xFF00..=0xFF7F => { error!(address, "I/O registers are not implemented"); None }
            0xFF80..=0xFFFE => Some(Arc::clone(&self.hram)),
            0xFFFF => { error!(address, "Interrupt enable is not implemented"); None }
            _ => panic!(
                "Tried to get device at {address:x} (outside of address space)"
            ),
        }
    }

    pub fn read_word(&self, address: u16) -> u16 {
        let upper = self.read(address);
        let lower = self.read(address + 1);

        // In 2-byte instructions, the LSB is first.
        u16::from(lower) << 8 | u16::from(upper)
    }

    pub fn write_word(&mut self, address: u16, value: u16) {
        let upper = (value >> 8) as u8;
        let lower = value as u8;
        self.write(address, lower);
        self.write(address + 1, upper);
    }

    pub fn read_range(&self, start: u16, end: u16) -> Option<Vec<u8>> {
        let region = self.get_region(start);

        region.map(|region| {
            let region = region.read().unwrap();
            region
                .read_range(start - region.start, end - region.start)
                .to_owned()
        })
    }
}

pub fn load_rom(rom: &str, mmu: &MMU) -> std::io::Result<()> {
    // let rom = "./test-roms/blargg/mem_timing/mem_timing.gb";
    let bytes = fs::read(rom)?;

    if bytes.len() < 0x0133 || bytes[0x0104..=0x0133] != NINTENDO_HEADER {
        error!("Invalid ROM");
        return Err(std::io::Error::new(InvalidData, "Invalid ROM"));
    }

    info!("Loading ROM {rom}");
    let arc = mmu.cart.clone();
    arc.write()
        .unwrap()
        .write_range(0, bytes.len() as u16, &bytes);

    Ok(())
}

pub fn load_boot_rom(mmu: &MMU) {
    trace!("Loading boot ROM");
    let arc = mmu.cart.clone();
    let mut cart = arc.write().unwrap();
    cart.write_range(0, BOOT_ROM.len() as u16, BOOT_ROM);
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
