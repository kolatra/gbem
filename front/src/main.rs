#![allow(unused)]
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;
use std::process::exit;
use tracing::{debug, info, error};

use hardware::{cpu::CPU, MMU, GPU, load_boot_rom, LOG_LINES};

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
        let Ok(new_state) = r_cpu.recv() else {
            error!("cpu thread died");
            error!("goodbye :(");
            exit(1);
        };
        debug!("received");

        // graphics

        let b = false;
        if b {
            break;
        }
    }
}

fn setup_logs() {
    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_line_number(LOG_LINES)
        .with_file(LOG_LINES)
        .with_max_level(tracing::Level::TRACE)
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
