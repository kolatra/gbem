use std::{
    process::exit,
    sync::{
        mpsc::{self, Receiver},
        Arc, RwLock,
    },
    thread::{self, spawn},
    time::Duration,
};

use tracing::{error, trace};

use crate::{cpu::CPU, mem::load_rom};

pub fn run_emulation(rom: &str) -> crate::Result<()> {
    let cpu = CPU::new();

    if let Err(e) = load_rom(rom, &cpu.mmu) {
        error!("Failed to load ROM: {}", e);
        exit(1);
    }

    // box that bitch up for sharing
    let p_cpu = Arc::new(RwLock::new(cpu));
    let r_cpu = setup_thread(Arc::clone(&p_cpu));

    loop {
        match r_cpu.recv() {
            Ok(_cpu_state) => {}

            Err(e) => {
                error!("cpu thread died \n{e}");
                error!("goodbye :(");
                anyhow::bail!("");
            }
        }
    }
}

fn setup_thread(cpu: Arc<RwLock<CPU>>) -> Receiver<Arc<RwLock<CPU>>> {
    let (cpu_sender, cpu_receiver) = mpsc::channel();

    let _cpu_thread = spawn(move || loop {
        let _ = cpu.write().map(|mut c| c.cycle());

        cpu_sender.send(Arc::clone(&cpu)).unwrap();
        trace!("cycle finished");

        thread::sleep(Duration::from_millis(1000 / crate::FPS as u64));
    });

    cpu_receiver
}
