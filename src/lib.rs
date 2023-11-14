#![allow(unused)]

use std::ops::Range;

const CLOCK_FREQ: usize = 4194304; // 4.194304 MHz
const MACHINE_FREQ: usize = 1048576; // 1.048576 MHz - 1/4 of the clock frequency
const FPS: usize = 60;

const RAM_SIZE: usize = 0x2000;
const ADDRESS_SPACE: Range<u16> = 0x0000..0xFFFF;

#[derive(Debug, Default)]
struct Registers {
    af: u16,
    bc: u16,
    de: u16,
    hl: u16,

    pc: u16,
    sp: u16,
}

impl Registers {
    fn read_high(&self) -> u16 {
        (self.af >> 8) & 0xFF
    }

    fn read_low(&self) -> u16 {
        self.af & 0xFF
    }
}

struct CPU {
    registers: Registers,
}

struct System {
    cpu: CPU,
    memory: [u8; RAM_SIZE],
    video_memory: [u8; RAM_SIZE],
}
