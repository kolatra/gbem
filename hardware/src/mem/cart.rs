use crate::mem::Memory;

const MAX_ROM_SIZE: usize = 0x8000; // Assumed from the region size in memory

#[derive(Debug, Clone, Default)]
pub struct Cartridge {
    mem: Vec<u8>,
}

impl Cartridge {
    pub fn new() -> Self {
        Self {
            mem: Vec::with_capacity(MAX_ROM_SIZE),
        }
    }
}

impl Memory for Cartridge {
    fn read(&self, address: u16) -> u8 {
        self.mem[address as usize]
    }

    fn write(&mut self, address: u16, value: u8) {
        self.mem[address as usize] = value;
    }

    fn read_range(&self, start: u16, end: u16) -> &[u8] {
        &self.mem[start as usize..end as usize]
    }

    fn write_range(&mut self, start: u16, end: u16, value: &[u8]) {
        self.mem[start as usize..end as usize].copy_from_slice(value);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_range() {
        let mut cart = Cartridge::new();
        cart.write(0x0000, 0x01);
        cart.write(0x0001, 0x02);
        cart.write(0x0002, 0x03);
        cart.write(0x0003, 0x04);
        cart.write(0x0004, 0x05);
        cart.write(0x0005, 0x06);
        cart.write(0x0006, 0x07);
        cart.write(0x0007, 0x08);

        let range = cart.read_range(0x0000, 0x0008);
        assert_eq!(range, &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);
    }

    #[test]
    fn test_read_range_mut() {
        let mut cart = Cartridge::new();
        cart.write(0x0000, 0x01);
        cart.write(0x0001, 0x02);
        cart.write(0x0002, 0x03);
        cart.write(0x0003, 0x04);
        cart.write(0x0004, 0x05);
        cart.write(0x0005, 0x06);
        cart.write(0x0006, 0x07);
        cart.write(0x0007, 0x08);

        let range = cart.read_range(0x0000, 0x0008);
        assert_eq!(range, &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08]);

        cart.write_range(0, 7, &[0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10]);

        let range = cart.read_range(0x0000, 0x0008);
        assert_eq!(range, &[0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F, 0x10]);
    }
}
