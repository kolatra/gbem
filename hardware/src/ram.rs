use std::sync::{Arc, RwLock};

use crate::mem::Memory;

pub enum RamSize {
    KB2 = 2048,
    KB8 = 8192,
    KB32 = 32_768,
    KB64 = 65_536,
    KB128 = 131_072,
}

impl From<RamSize> for u16 {
    #[allow(clippy::cast_possible_truncation)]
    fn from(size: RamSize) -> Self {
        size as Self
    }
}

pub enum RamStart {
    Cart = 0x0000,
    VRam = 0x8000,
    WRam = 0xC000,
    ERam = 0xE000,
    HRam = 0xFF80,
}

impl From<RamStart> for u16 {
    #[allow(clippy::cast_possible_truncation)]
    fn from(start: RamStart) -> Self {
        start as Self
    }
}

pub type Region = Arc<RwLock<MemoryRegion>>;

#[derive(Debug, Clone, Default)]
pub struct MemoryRegion {
    pub start: u16,
    mem: Vec<u8>,
}

impl Memory for MemoryRegion {
    fn read(&self, address: u16) -> u8 {
        let address = address - self.start;
        self.mem[address as usize]
    }

    fn write(&mut self, address: u16, value: u8) {
        let address = address - self.start;
        self.mem[address as usize] = value;
    }

    fn read_range(&self, start: u16, end: u16) -> &[u8] {
        &self.mem[start as usize..end as usize]
    }

    fn write_range(&mut self, start: u16, end: u16, value: &[u8]) {
        self.mem[start as usize..end as usize].copy_from_slice(value);
    }
}

impl MemoryRegion {
    pub fn new(size: u16, start: u16) -> Arc<RwLock<Self>> {
        let region = Self {
            start,
            mem: vec![0; size.into()],
        };

        Arc::new(RwLock::new(region))
    }
}
