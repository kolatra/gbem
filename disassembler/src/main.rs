#![feature(let_chains)]
#![warn(clippy::pedantic, clippy::nursery)]
#![allow(dead_code, clippy::similar_names)]
use std::{
    collections::HashMap,
    io::{Read, Write}, fs::File, sync::Arc,
};

use clap::Parser;
use hardware::instructions::INSTRUCTIONS;
use tracing::{error, info, warn};

fn main() {
    setup_logs();
    let args = Args::parse();

    let bytes = args.file.map_or_else(
        || {
            warn!("No file specified, using default");
            hardware::BOOT_ROM.to_vec()
        },
        |file| read_from_file(file).expect("Failed to read file"),
    );

    if args.save {
        info!("Saving bytes to file");
        save_bytes(&bytes).expect("not sure how we got here");
    }

    let header = parse_header(&bytes);
    info!("{:#?}", header);
    disassemble(&bytes);
}

#[derive(Debug, Parser)]
struct Args {
    #[clap(short, long)]
    file: Option<String>,

    #[clap(short, long, default_value = "false")]
    save: bool,
}

#[derive(Debug)]
struct DMGHeader {
    entry_point: Vec<u8>,
    logo: Vec<u8>,
    title: Vec<u8>,
    licensee: Vec<u8>,
    sgb_flag: u8,
    cartridge_type: u8,
    rom_size: u8,
    ram_size: u8,
    destination_code: u8,
    old_licensee_code: u8,
    mask_rom_version_number: u8,
    header_checksum: u8,
    global_checksum: u16,
}

fn parse_header(bytes: &[u8]) -> DMGHeader {
    let entry_point = bytes[0x0100..0x0104].to_vec();
    let logo = bytes[0x0104..0x0134].to_vec();
    let title = bytes[0x0134..0x0143].to_vec();
    let licensee = bytes[0x0144..0x0146].to_vec();
    let sgb_flag = bytes[0x0146];
    let cartridge_type = bytes[0x0147];
    let rom_size = bytes[0x0148];
    let ram_size = bytes[0x0149];
    let destination_code = bytes[0x014A];
    let old_licensee_code = bytes[0x014B];
    let mask_rom_version_number = bytes[0x014C];
    let header_checksum = bytes[0x014D];

    let global_checksum = bytes[0x014E..0x0150]
        .iter()
        .fold(0, |acc, n| acc + u16::from(*n));

    DMGHeader {
        entry_point,
        logo,
        title,
        licensee,
        sgb_flag,
        cartridge_type,
        rom_size,
        ram_size,
        destination_code,
        old_licensee_code,
        mask_rom_version_number,
        header_checksum,
        global_checksum,
    }
}

fn setup_logs() {
    let file = File::create("debug.log");
    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Error: {error:?}"),
    };

    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        .with_writer(Arc::new(file))
        .with_line_number(true)
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

fn disassemble(bytes: &[u8]) {
    info!("Ending address: {:#04x}", bytes.len());

    let mut skip_count = 0;
    let mut filtered_count = 0;
    let mut unknown_map = HashMap::new();

    let minify = false;
    let mut minifying = false;

    for (i, byte) in bytes.iter().enumerate() {
        if skip_count > 0 {
            skip_count -= 1;
            continue;
        }

        if minify {
            if i == bytes.len() - 1 && minifying {
                info!("{:#04x}: {:#04x} x{}", i, bytes[i-1], filtered_count);
            }

            if *byte == 0x00 || *byte == 0xFF {
                if minifying {
                    if bytes[i-1] != *byte {
                        info!("{:#04x}: {:#04x} x{}", i, bytes[i-1], filtered_count);
                        filtered_count = 0;
                    }

                    filtered_count += 1;
                } else {
                    minifying = true;
                    filtered_count = 1;
                }

                continue;
            } else if minifying {
                info!("{:#04x}: {:#04x} x{}", i, bytes[i-1], filtered_count);
                minifying = false;
                filtered_count = 0;
            }
        }

        if *byte == 0xCB {
            info!("{:#04x} {:#04x}", byte, &bytes[i + 1]);
            skip_count = 1;
            continue;
        }

        let instruction = INSTRUCTIONS.iter().find(|i| i.opcode == u32::from(*byte));

        if let Some(ins) = instruction {
            let length = ins.length as usize;

            let operands = if length > 1 {
                skip_count = length - 1;

                let ins_bytes = &bytes[i..i + length];
                let out = ins_bytes
                    .iter()
                    .fold(String::new(), |s, byte| s + &format!("{byte:#02x} "));

                out
            } else {
                String::new()
            };

            info!(
                "{:#04x}: {} {}",
                i, ins.mnemonic, operands
            );
        } else {
            error!("{:#04x}: {:#04x} ?", i, byte);
            let entry = unknown_map.entry(byte).or_insert(0);
            *entry += 1;
        }
    }

    warn!("Unknown bytes: {}", unknown_map.len());
    let mut counter = 0;
    let mut out = String::new();
    for (i, (byte, count)) in unknown_map.iter().enumerate() {
        counter += 1;
        out += &format!("{byte:#04x}: {count: <3} ");

        if counter == 4 {
            counter = 0;
            //warn!("{}", out);
            out.clear();
            continue;
        }

        // print extras
        if i == unknown_map.len() - 1 {
            //warn!("{}", out);
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
            format!("{b:#04x}")
                + if counter == 16 {
                    counter = 0;
                    "\n"
                } else {
                    " "
                }
        })
        .fold(String::new(), |new_s, b| new_s + &b);

    f.write_all(output.as_bytes())
}
