#![allow(dead_code)]
#![warn(clippy::nursery, clippy::pedantic)]
use std::process::exit;

use clap::Parser;
use tracing::warn;

use hardware::{emu::new_emulation, LOG_LINES, SPAMMY_LOGS};

static DEFAULT_ROM: &str = "/home/tyler/dev/gbem/roms/Tetris.gb";

#[derive(Debug, Parser)]
struct Args {
    #[clap(short, long)]
    rom: Option<String>,
}

fn main() {
    setup_logs();

    let args = Args::parse();
    let rom = args.rom.unwrap_or_else(|| {
        warn!("Using default rom: {}", DEFAULT_ROM);
        DEFAULT_ROM.to_string()
    });

    match new_emulation(&rom) {
        Ok(()) => (),
        Err(e) => {
            eprintln!("{e}");
            exit(1)
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
        .without_time()
        .with_line_number(LOG_LINES)
        .with_file(LOG_LINES)
        .with_max_level(level)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
}
