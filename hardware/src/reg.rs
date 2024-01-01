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
}

#[allow(clippy::needless_pass_by_value)]
impl Registers {
    pub const fn read_pair(&self, pair: Pair) -> u16 {
        match pair {
            Pair::AF => (self.a as u16) << 8 | self.f as u16,
            Pair::BC => (self.b as u16) << 8 | self.c as u16,
            Pair::DE => (self.d as u16) << 8 | self.e as u16,
            Pair::HL => (self.h as u16) << 8 | self.l as u16,
        }
    }

    
    pub fn write_pair(&mut self, pair: Pair, value: u16) {
        let higher = (value >> 8) as u8;
        let lower = value as u8;

        match pair {
            Pair::AF => {
                self.a = higher;
                self.f = lower;
            }
            Pair::BC => {
                self.b = higher;
                self.c = lower;
            }
            Pair::DE => {
                self.d = higher;
                self.e = lower;
            }
            Pair::HL => {
                self.h = higher;
                self.l = lower;
            }
        }
    }

    pub const fn is_set(&self, flag: FlagBit) -> bool {
        self.f >> flag as u16 == 1
    }
}

impl Registers {
    pub const fn new() -> Self {
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
