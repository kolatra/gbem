use hardware::instructions;

fn main() {
    let bytes = include_bytes!("../DMG_ROM.bin").to_vec();
    let instructions = instructions::get();
    
    let mut skip_count = 0;
    // 0x31 0xff 0xfe LD SP, d16
    for byte in bytes {
        if skip_count > 0 {
            skip_count -= 1;
            continue;
        }

        let instruction = instructions.iter().find(|i| i.opcode == byte as u16);
        match instruction {
            Some(i) => {
                println!("{:#04x} {}", byte, i.mnemonic);
            }
            None => {
                println!("{:#04x} unknown", byte);
            }
        }

        skip_count = match instruction {
            Some(i) => i.length - 1,
            None => 0,
        };
    }
}
