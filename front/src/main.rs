#![allow(dead_code)]
#![feature(lazy_cell)]
#![warn(clippy::nursery, clippy::pedantic)]

use std::sync::LazyLock;

use clap::Parser;
use tracing::{error, trace, warn};

use hardware::instructions::INSTRUCTIONS;
use hardware::emu::run_emulation;

static DEFAULT_ROM: &str = "./gbem/roms/Tetris.gb";

#[derive(Debug, Parser)]
struct Args {
    #[clap(short, long)]
    rom: Option<String>,
    
    #[clap(short)]
    spam: bool
}

static ARGS: LazyLock<Args> = LazyLock::new(Args::parse);

fn dbg_check_instructions() -> Result<(), &'static str> {
    let mut buf = Vec::new();

    for instruction in INSTRUCTIONS.iter() {
        if buf.contains(&instruction.opcode) {
            error!("{instruction:?} Defined twice");
            return Err("Instruction already exists!");
        }

        buf.push(instruction.opcode);
    }

    trace!("All good");
    Ok(())
}

fn main() -> Result<(), &'static str> {
    setup_logs();

    dbg_check_instructions()?;

    let default = DEFAULT_ROM.to_string();
    let rom = ARGS.rom.as_ref().unwrap_or_else(|| {
        warn!("Using default rom: {}", DEFAULT_ROM);
        &default
    });

    match run_emulation(rom) {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("{e}");
            Err("")
        }
    }
}

fn setup_logs() {
    let level = if ARGS.spam {
        tracing::Level::TRACE
    } else {
        tracing::Level::DEBUG
    };

    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .compact()
        .without_time()
        .with_line_number(true)
        .with_file(false)
        .with_max_level(level)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();
}
