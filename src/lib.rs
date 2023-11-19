#![allow(unused)]

use std::thread;
use std::cell::{Cell, RefCell};
use std::sync::{mpsc, Arc, RwLock};
use std::fs;
use std::ops::Range;
use std::rc::Rc;
use std::time::Duration;

const CLOCK_FREQ: usize = 4194304; // 4.194304 MHz
const MACHINE_FREQ: usize = 1048576; // 1.048576 MHz - 1/4 of the clock frequency
const FPS: usize = 60;

const RAM_SIZE: usize = 0x2000;
const MAX_ROM_SIZE: usize = 0x8000;

pub const NINTENDO_HEADER: [u8; 48] = [
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B, 0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E, 0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC, 0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,
];

pub const BOOT_ROM: [u8; 256] = [
    0x31, 0xFE, 0xFF, 0xAF, 0x21, 0xFF, 0x9F, 0x32, 0xCB, 0x7C, 0x20, 0xFB, 0x21, 0x26, 0xFF, 0x0E,
    0x11, 0x3E, 0x80, 0x32, 0xE2, 0x0C, 0x3E, 0xF3, 0xE2, 0x32, 0x3E, 0x77, 0x77, 0x3E, 0xFC, 0xE0,
    0x47, 0x11, 0x04, 0x01, 0x21, 0x10, 0x80, 0x1A, 0xCD, 0x95, 0x00, 0xCD, 0x96, 0x00, 0x13, 0x7B,
    0xFE, 0x34, 0x20, 0xF3, 0x11, 0xD8, 0x00, 0x06, 0x08, 0x1A, 0x13, 0x22, 0x23, 0x05, 0x20, 0xF9,
    0x3E, 0x19, 0xEA, 0x10, 0x99, 0x21, 0x2F, 0x99, 0x0E, 0x0C, 0x3D, 0x28, 0x08, 0x32, 0x0D, 0x20,
    0xF9, 0x2E, 0x0F, 0x18, 0xF3, 0x67, 0x3E, 0x64, 0x57, 0xE0, 0x42, 0x3E, 0x91, 0xE0, 0x40, 0x04,
    0x1E, 0x02, 0x0E, 0x0C, 0xF0, 0x44, 0xFE, 0x90, 0x20, 0xFA, 0x0D, 0x20, 0xF7, 0x1D, 0x20, 0xF2,
    0x0E, 0x13, 0x24, 0x7C, 0x1E, 0x83, 0xFE, 0x62, 0x28, 0x06, 0x1E, 0xC1, 0xFE, 0x64, 0x20, 0x06,
    0x7B, 0xE2, 0x0C, 0x3E, 0x87, 0xE2, 0xF0, 0x42, 0x90, 0xE0, 0x42, 0x15, 0x20, 0xD2, 0x05, 0x20,
    0x4F, 0x16, 0x20, 0x18, 0xCB, 0x4F, 0x06, 0x04, 0xC5, 0xCB, 0x11, 0x17, 0xC1, 0xCB, 0x11, 0x17,
    0x05, 0x20, 0xF5, 0x22, 0x23, 0x22, 0x23, 0xC9, 0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
    0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D, 0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
    0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99, 0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
    0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E, 0x3C, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x3C,
    0x21, 0x04, 0x01, 0x11, 0xA8, 0x00, 0x1A, 0x13, 0xBE, 0x20, 0xFE, 0x23, 0x7D, 0xFE, 0x34, 0x20,
    0xF5, 0x06, 0x19, 0x78, 0x86, 0x23, 0x05, 0x20, 0xFB, 0x86, 0x20, 0xFE, 0x3E, 0x01, 0xE0, 0x50,
];

pub const JUMP_VECTORS: [u8; 12] = [
    0x00, 0x08, 0x10, 0x20, 0x28, 0x30, 0x38, 0x40, 0x48, 0x50, 0x58, 0x60,
];

enum CounterResult {
    Next,
    Advance(u8),
    Pause
}

struct Instruction {
    mnemonic: &'static str,
    opcode: u32,
    cycles: i8,
    length: i8,
    handler: fn (mmu: &mut MMU, cpu: &mut CPU) -> CounterResult
}

#[derive(Debug, Default, Clone, Copy)]
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
#[derive(Debug, Clone, Default)]
struct CPU {
    registers: Registers,
}

impl CPU {
    fn reset(&mut self) {
        self.registers = Registers::default();
    }
}

#[derive(Debug, Default)]
struct Timer {
    counter: u8,
    modulo: u8,
    control: u8,
}

#[derive(Debug, Default)]
struct Interrupts {
    enable: u8,
    flag: u8,
}

/// Listen for memory management orders from the CPU
#[derive(Debug)]
struct MMU {
    w_ram: Vec<u8>, // Work RAM
    v_ram: Vec<u8>, // Video RAM
    cartridge: Vec<u8>,
    timer: Timer,
    // https://gbdev.io/pandocs/Joypad_Input.html#ff00--p1joyp-joypad
    joypad: u8,
    // https://gbdev.io/pandocs/Timer_and_Divider_Registers.html#ff04--div-divider-register
    divider_reg: u8,
    // https://gbdev.io/pandocs/Interrupts.html#ff0f--if-interrupt-flag
    interrupt_enable: u8,
    interrupt_flag: u8,
}

impl MMU {
    fn read(&self, address: u16) -> u8 {
        const VRAM_START: u16 = 0x8000;
        const WRAM_START: u16 = 0xC000;

        match address {
            0x0000..=0x7FFF => self.cartridge[address as usize],
            0x8000..=0x9FFF => self.v_ram[(address - VRAM_START) as usize],
            0xA000..=0xBFFF => 1, // Cartridge external RAM
            0xC000..=0xDFFF => self.w_ram[(address - WRAM_START) as usize],
            0xE000..=0xFDFF => 1, // Echo RAM
            0xFE00..=0xFE9F => 1, // Object attribute memory
            0xFEA0..=0xFEFF => 1, // Not usable
            0xFF00 => self.joypad,
            0xFF04 => self.divider_reg,
            0xFF05 => self.timer.counter,
            0xFF06 => self.timer.modulo,
            0xFF07 => self.timer.control,
            0xFF0F => self.interrupt_flag,
            0xFF10..=0xFF26 => 1, // Sound control registers
            0xFF00..=0xFF7F => 1, // I/O registers
            0xFF80..=0xFFFE => 1, // High RAM
            0xFFFF => self.interrupt_enable,
            _ => 0,
        }
    }
}

fn get_instructions() -> Vec<Instruction> {
    vec![
        Instruction {
            mnemonic: "NOP",
            opcode: 0x00,
            cycles: 1,
            length: 1,
            handler: |_, _| { CounterResult::Next }
        },

        Instruction {
            mnemonic: "ADD A,B",
            opcode: 0x80,
            cycles: 1,
            length: 1,
            handler: |mmu, cpu| {
                let (n, overflow) = cpu.registers.a.overflowing_add(cpu.registers.b);
                let mut res = 0;
                res |= ((n == 0) as u8) >> 6;
                res |= (overflow as u8) >> 4;
                res |= (overflow as u8) >> 3;
                cpu.registers.f = res;
                CounterResult::Next
            }
        },

        Instruction {
            mnemonic: "STOP",
            opcode: 0x1000,
            cycles: 1,
            length: 2,
            handler: |_, _| {
                // The system clock/oscillator stops until either:
                // A reset 
                // Joypad input - resume execution at pc+1
                CounterResult::Pause
            }
        },

        Instruction {
            mnemonic: "HALT",
            opcode: 0x76,
            cycles: 1,
            length: 1,
            handler: |_, _| {
                // The clock stops but the oscillator and LCD controller continue to operate
                // until an interrupt occurs.
                CounterResult::Pause
            }
        },
    ]
}

fn load_rom(mmu: &mut MMU) -> std::io::Result<()> {
    let rom = "";
    let mut bytes = fs::read(rom)?;

    if bytes.len() < 0x0133 {
        println!("short ass ROM");
        return Ok(())
    }    

    let logo = &bytes[0x0104..0x0133];
    if logo != NINTENDO_HEADER {
        eprintln!("Invalid Gameboy ROM!");
    } else {
        println!("Loading ROM");
        let mem = &mut mmu.cartridge;
        mem.clear();
        mem.append(&mut bytes);
    }

    Ok(())
}

pub fn its_a_gameboy() {
    let mut mmu = MMU {
        w_ram: Vec::with_capacity(RAM_SIZE),
        v_ram: Vec::with_capacity(RAM_SIZE),
        cartridge: Vec::with_capacity(MAX_ROM_SIZE),
        timer: Timer::default(),
        joypad: 0,
        divider_reg: 0,
        interrupt_enable: 0,
        interrupt_flag: 0,
    };

    let cpu = CPU {
        registers: Registers::default(),
    };

    dbg!(load_rom(&mut mmu));

    let (cpu_sender, cpu_receiver) = mpsc::channel::<CPU>();
    let _t_cpu = std::thread::spawn(move || {
        loop {
            cpu_sender.send(CPU::default()).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });

    let (gfx_sender, gfx_receiver) = mpsc::channel::<u8>();
    let _t_gfx = std::thread::spawn(move || {
        loop {
            gfx_sender.send(1).unwrap();
            thread::sleep(Duration::from_millis(1000));
        }
    });

    loop {
        let new_state = cpu_receiver.recv().unwrap();
        dbg!(new_state);

        let new_state = gfx_receiver.recv().unwrap();
        dbg!(new_state);

        if new_state == 0 {
            break;
        }
    }
}
