#![allow(dead_code)]
#![warn(clippy::nursery, clippy::pedantic)]

use clap::Parser;
use tracing::{error, trace, warn};

use hardware::instructions::INSTRUCTIONS;
use hardware::{emu::run_emulation, SPAMMY_LOGS};

static DEFAULT_ROM: &str = "/home/tyler/dev/gbem/roms/Tetris.gb";

#[derive(Debug, Parser)]
struct Args {
    #[clap(short, long)]
    rom: Option<String>,
}

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

    let args = Args::parse();
    let rom = args.rom.unwrap_or_else(|| {
        warn!("Using default rom: {}", DEFAULT_ROM);
        DEFAULT_ROM.to_string()
    });

    match run_emulation(&rom) {
        Ok(()) => Ok(()),
        Err(e) => {
            eprintln!("{e}");
            Err("")
        }
    }
}

fn setup_logs() {
    let level = if SPAMMY_LOGS {
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
