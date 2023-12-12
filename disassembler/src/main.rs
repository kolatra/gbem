use std::{io::{Write, Read}, collections::HashMap};

use clap::Parser;
use hardware::instructions::INSTRUCTIONS;
use tracing::{error, info, warn};

fn main() {
    setup_logs();
    let args = Args::parse();
    let bytes = match args.file {
        Some(file) => read_from_file(file).expect("Failed to read file"),
        None => {
            warn!("No file specified, using default");
            hardware::BOOT_ROM.to_vec()
        }
    };
    info!("Saving bytes to file");
    save_bytes(&bytes).expect("not sure how we got here");

    disassemble(bytes);
}

#[derive(Debug, Parser)]
struct Args {
    #[clap(short, long)]
    file: Option<String>,
}

fn setup_logs() {
    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        // .with_line_number(true)
        .without_time()
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

fn read_from_file(file: String) -> std::io::Result<Vec<u8>> {
    let mut f = std::fs::File::open(file)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn disassemble(bytes: Vec<u8>) {
    let mut skip_count = 0;
    let mut nop_count = 0;
    let mut unknown_map = HashMap::new();
    let mut in_nop = false;
    for (i, byte) in bytes.iter().enumerate() {
        if skip_count > 0 {
            skip_count -= 1;
            continue;
        }

        if *byte == 0x00 {
            if in_nop {
                nop_count += 1;
            } else {
                in_nop = true;
                nop_count = 1;
            }
            continue;
        } else if in_nop {
            info!("NOP x{}", nop_count);
            info!("--------");
            in_nop = false;
            continue;
        }

        if *byte == 0xCB {
            info!("CB prefix");
            info!("{:#04x} {:#04x}", byte, &bytes[i + 1]);
            info!("--------");
            skip_count = 1;
            continue;
        }

        let instruction = INSTRUCTIONS.iter().find(|i| i.opcode == *byte as u32);

        match instruction {
            Some(ins) => {
                info!("{}", ins.mnemonic);
                let length = ins.length as usize;

                if length == 1 {
                    info!("{:#04x}", byte);
                } else {
                    skip_count = length - 1;

                    let ins_bytes = &bytes[i..i + length];
                    let out = ins_bytes
                        .iter()
                        .fold(String::new(), |s, b| s + &format!("{:#02x} ", b));

                    info!("{}", out);
                }

                info!("--------");
            }

            None => {
                error!("{:#04x}: Unknown", byte);
                let entry = unknown_map.entry(byte).or_insert(0);
                *entry += 1;
            }
        }
    }

    warn!("Unknown bytes");
    let mut counter = 0;
    let mut out = String::new();
    for (byte, count) in unknown_map {
        counter += 1;
        out += &format!("{:#04x}: {: <4} ", byte, count);
        if counter == 15 {
            counter = 0;
            warn!("{}", out);
            out.clear();
        }
    }
}

fn save_bytes(bytes: &[u8]) -> std::io::Result<()> {
    let mut f = std::fs::File::create("./disassembler/DMG_ROM.txt")?;
    let mut counter = 0;

    let output = bytes
        .iter()
        .map(|b| {
            counter += 1;
            format!("{:#04x}", b)
                + if counter == 16 {
                    counter = 0;
                    "\n"
                } else {
                    " "
                }
        })
        .fold(String::new(), |new_s, byte| new_s + &byte);

    f.write_all(output.as_bytes())
}
