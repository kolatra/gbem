#![allow(unused)]
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;
use std::process::exit;
use tracing::{debug, info, error};

use hardware::{cpu::CPU, MMU, GPU, load_boot_rom, LOG_LINES, SPAMMY_LOGS};

fn main() {
    setup_logs();

    let mut cpu = CPU::new();
    // https://github.com/Hacktix/Bootix/blob/main/bootix_dmg.asm
    // Can look here for the "steps"
    load_boot_rom(&mut cpu.mmu);

    // box that bitch up for sharing
    let p_cpu = Arc::new(RwLock::new(cpu));
    let r_cpu = setup_thread(Arc::clone(&p_cpu));

    loop {
        match r_cpu.recv() {
            Ok(cpu_state) => {
                // update everything else 
                // with new memory and cpu state
            },

            Err(e) => {
                error!("cpu thread died \n{e}");
                error!("goodbye :(");
                break
            },
        }
    }
}

fn setup_logs() {
    let level = match SPAMMY_LOGS {
        true => tracing::Level::TRACE,
        false => tracing::Level::DEBUG
    };

    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
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
        info!("sending");

        thread::sleep(Duration::from_millis(500));
    });

    cpu_receiver
}
