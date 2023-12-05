use hardware::instructions;

fn main() {
    let bytes = include_bytes!("../DMG_ROM.bin").to_vec();
    let instructions = instructions::get();

    // 0x31 0xff 0xfe LD SP, d16
    let mut skip_count = 0;
    for (i, byte) in bytes.iter().enumerate() {
        if skip_count > 0 {
            skip_count -= 1;
            continue;
        }

        let instruction = instructions.iter().find(|i| i.opcode == *byte as u16);
        match instruction {
            Some(i) => {
                println!("{:#04x} {}", byte, i.mnemonic);
            }
            None => {
                println!("{:#04x}", byte);
            }
        }

        let length = match instruction {
            Some(i) => i.length,
            None => 0,
        };

        if length == 0 || length == 1 {
            continue;
        }

        skip_count = length;

        let ins_bytes = &bytes[i..i + length as usize];
        let out = ins_bytes.iter().fold(String::new(), |s, b| s + &format!("{:#02x} ", b));
        println!("{}", out);
    }
}
