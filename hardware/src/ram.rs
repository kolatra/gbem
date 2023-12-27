use std::{
    ops::Deref,
    sync::{Arc, RwLock},
};

use crate::mem::Memory;

use RamSize::*;
pub enum RamSize {
    KB2,
    KB8,
    KB32,
    KB64,
    KB128,
}

impl Deref for RamSize {
    type Target = usize;

    fn deref(&self) -> &Self::Target {
        match self {
            KB2 => &2048,
            KB8 => &8192,
            KB32 => &32768,
            KB64 => &65536,
            KB128 => &131072,
        }
    }
}

use RamStart::*;
pub enum RamStart {
    Cart,
    VRam,
    WRam,
    ERam,
    HRam,
}

impl Deref for RamStart {
    type Target = u16;

    fn deref(&self) -> &Self::Target {
        match self {
            Cart => &0x0000,
            VRam => &0x8000,
            WRam => &0xC000,
            ERam => &0xE000,
            HRam => &0xFF80,
        }
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
    pub fn new(size: usize, start: u16) -> Arc<RwLock<Self>> {
        let region = Self {
            start,
            mem: vec![0; size],
        };

        Arc::new(RwLock::new(region))
    }
}
