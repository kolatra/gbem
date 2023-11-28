#![allow(unused)]
use tracing::{debug, info};
use std::sync::RwLock;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;

use hardware::{CPU, load_rom};

fn main() {
    setup_logs();

    let mut cpu = CPU::default();
    load_rom(&mut cpu.mmu);

    let p_cpu = Arc::new(RwLock::new(cpu));
    let cpu_r = setup_thread(Arc::clone(&p_cpu));

    loop {
        // Send this new state to the main thread
        // which is doing graphics
        let new_state = cpu_r.recv().unwrap();
        info!("recv");

        let b = false;
        if b {
            break;
        }
    }
}

fn setup_logs() {
    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

fn setup_thread(cpu: Arc<RwLock<CPU>>) -> Receiver<Arc<RwLock<CPU>>> {
    let (cpu_sender, cpu_receiver) = mpsc::channel();
    let _t_cpu = thread::spawn(move || loop {
    {   // Only hold the lock for this block
        cpu.write().and_then(|mut c| Ok(c.cycle()));
    }

        // Send updated state
        cpu_sender.send(cpu.clone()).unwrap();

        // Wait for clock time
        thread::sleep(Duration::from_millis(1000));
    });

    cpu_receiver
}
