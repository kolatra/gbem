#![allow(dead_code)]
#![warn(clippy::nursery, clippy::pedantic)]
use clap::Parser;
use hardware::mem::load_rom;
use tracing::warn;
use std::process::exit;
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;
use tracing::{error, trace};

use hardware::{cpu::CPU, mem::load_boot_rom, LOG_LINES, SPAMMY_LOGS};

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
    trace!("Starting up with rom: {}", rom);

    let cpu = CPU::new();
    // https://github.com/Hacktix/Bootix/blob/main/bootix_dmg.asm
    // Can look here for the "steps"
    load_boot_rom(&cpu.mmu);
    if let Err(e) = load_rom(&rom, &cpu.mmu) {
        error!("Failed to load ROM: {}", e);
        exit(1);
    }

    // box that bitch up for sharing
    let p_cpu = Arc::new(RwLock::new(cpu));
    let r_cpu = setup_thread(Arc::clone(&p_cpu));

    loop {
        match r_cpu.recv() {
            Ok(_cpu_state) => {
                // update everything else
                // with new memory and cpu state
                trace!("received new cpu state");
            }

            Err(e) => {
                error!("cpu thread died \n{e}");
                error!("goodbye :(");
                break;
            }
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

fn setup_thread(cpu: Arc<RwLock<CPU>>) -> Receiver<Arc<RwLock<CPU>>> {
    let (cpu_sender, cpu_receiver) = mpsc::channel();
    let _t_cpu = thread::spawn(move || loop {
        let _ = cpu.write().map(|mut c| c.cycle());

        cpu_sender.send(Arc::clone(&cpu)).unwrap();
        trace!("sent new cpu state");

        thread::sleep(Duration::from_millis(500));
    });

    cpu_receiver
}
