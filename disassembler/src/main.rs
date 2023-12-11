use std::io::Write;

use hardware::instructions::INSTRUCTIONS;
use tracing::{error, info};

fn main() {
    setup_logs();
    let bytes = include_bytes!("../DMG_ROM.bin").to_vec();
    info!("Saving bytes to file");
    save_bytes(&bytes).expect("not sure how we got here");

    // 0x31 0xff 0xfe LD SP, d16
    let mut skip_count = 0;
    for (i, byte) in bytes.iter().enumerate() {
        if skip_count > 0 {
            skip_count -= 1;
            continue;
        }

        if byte.eq(&0xCB) {
            info!("CB prefix");
            info!("{:#04x} {:#04x}", byte, &bytes[i + 1]);
            info!("--------");
            skip_count = 1;
            continue;
        }

        let instruction = INSTRUCTIONS.iter().find(|i| i.opcode == *byte as u32);

        let length = match instruction {
            Some(i) => {
                info!("{}", i.mnemonic);
                i.length as usize
            }

            None => {
                error!("{} {:#04x}: Unknown", i, byte);
                break;
            }
        };

        if length > 1 {
            skip_count = length - 1;

            let ins_bytes = &bytes[i..i + length];
            let out = ins_bytes
                .iter()
                .fold(String::new(), |s, b| s + &format!("{:#02x} ", b));
            info!("{}", out);
        } else {
            info!("{:#04x}", byte);
        }
        info!("--------");
    }
}

fn setup_logs() {
    let subscriber = tracing_subscriber::fmt::Subscriber::builder()
        // .with_line_number(true)
        .without_time()
        .with_max_level(tracing::Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();
}

fn save_bytes(bytes: &[u8]) -> std::io::Result<()> {
    let mut f = std::fs::File::create("./disassembler/DMG_ROM.txt").unwrap();
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
