use crate::mem::Memory;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FlagBit {
    Z = 7,
    N = 6,
    H = 5,
    C = 4,
}

#[derive(Debug, Clone, Copy)]
pub struct Registers {
    pub a: u8,
    pub f: u8,
    pub b: u8,
    pub c: u8,
    pub d: u8,
    pub e: u8,
    pub h: u8,
    pub l: u8,

    pub pc: u16,
    pub sp: u16,
}

pub enum Pair {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC
}

impl Memory for Registers {
    fn read(&self, _address: u16) -> u8 {
        todo!()
    }

    fn write(&mut self, _address: u16, _value: u8) {
        todo!()
    }
}

impl Registers {
    pub fn new() -> Self {
        Self {
            a: 0x01,
            // FIXME:
            // If the header checksum is 0,
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

            pc: 0x0100,
            sp: 0xFFFE,
        }
    }
}

impl Default for Registers {
    fn default() -> Self {
        Self::new()
    }
}
