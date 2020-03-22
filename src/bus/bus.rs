// use crate::cpu::Cpu6502;

pub struct Bus {
    // cpu: Cpu6502,
    ram: [u8; 64 * 1024], // temp ram
}

impl Default for Bus {
    fn default() -> Self {
        Bus {
            // cpu: Default::default(),
            ram: [0; 64 * 1024],
        }
    }
}

impl Bus {
    pub fn write(&mut self, address: u16, data: u8) {
        match address {
            0x0000..=0xFFFF => {
                self.ram[address as usize] = data;
            }
        }
    }

    pub fn read(&self, address: u16, read_only: bool) -> u8 {
        match address {
            0x0000..=0xFFFF => self.ram[address as usize],
            _ => 0x00,
        }
    }
}
