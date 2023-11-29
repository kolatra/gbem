#![allow(unused)]
use tracing::error;
use tracing::{debug, info};
use std::sync::RwLock;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;
use std::process::exit;

use hardware::{CPU, load_boot_rom, LOG_LINES};

fn main() {
    setup_logs();

    let mut cpu = CPU::new();
    load_boot_rom(&mut cpu.mmu);

    // box that bitch up for sharing
    let p_cpu = Arc::new(RwLock::new(cpu));
    let cpu_r = setup_thread(Arc::clone(&p_cpu));

    loop {
        let Ok(new_state) = cpu_r.recv() else {
            error!("cpu thread died");
            error!("exiting...");
            exit(1);
        };
        info!("received");

        // do the graphics processing shit here idk

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
        cpu.write().and_then(|mut c| Ok(c.cycle()));

        cpu_sender.send(Arc::clone(&cpu)).unwrap();
        info!("sending");

        thread::sleep(Duration::from_millis(1000));
    });

    cpu_receiver
}
