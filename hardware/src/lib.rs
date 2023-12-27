#![warn(clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::module_name_repetitions,
    clippy::match_overlapping_arm,
    clippy::self_assignment,
    clippy::must_use_candidate,
    clippy::enum_glob_use,
    clippy::missing_panics_doc,
    clippy::missing_errors_doc,
    clippy::similar_names,
    clippy::too_many_lines
)]
#![feature(lazy_cell)]
use tracing::warn;

pub mod cpu;
pub mod instructions;
pub mod mem;
pub mod ram;
pub mod reg;

// TODO have a config file or CLI input for these?
pub const SPAMMY_LOGS: bool = true;
pub const LOG_REGISTERS: bool = true;
pub const LOG_LINES: bool = true;

pub const CLOCK_FREQ: usize = 4_194_304; // 4.194304 MHz
pub const MACHINE_FREQ: usize = 1_048_576; // 1.048576 MHz - 1/4 of the clock frequency
pub const FPS: usize = 60;
pub const RAM_SIZE: usize = 0x2000;
pub const MAX_ROM_SIZE: usize = 0x8000; // Assumed from the region size in memory

pub const NINTENDO_HEADER: [u8; 48] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

pub static BOOT_ROM: &[u8] = include_bytes!("../../disassembler/DMG_ROM.bin");

pub const JUMP_VECTORS: [u8; 12] = [
    0x00, 0x08, 0x10, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58, 0x60,
];

/// A shorter to use equivalent of `Default::default()`
/// Useful for intializing and updating structs
#[inline]
pub fn default<T: Default>() -> T {
    Default::default()
}

#[derive(Debug, Clone, Default)]
pub struct GPU {}
impl GPU {
    pub fn new() -> Self {
        warn!("nice gpu nerd");
        Self {}
    }
}

#[allow(unused)]
#[derive(Debug, Default, Clone, Copy)]
struct Timer {
    counter: u8,
    modulo: u8,
    control: u8,
}

#[allow(unused)]
#[derive(Debug, Default, Clone, Copy)]
struct Interrupts {
    enable: u8,
    flag: u8,
}
