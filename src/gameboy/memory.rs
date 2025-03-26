use std::sync::Mutex;

use crate::utils::{combine, split};

const ROM_00_START: usize = 0x0000;
const ROM_00_END: usize = 0x3FFF;
const ROM_00_SIZE: usize = ROM_00_END - ROM_00_START + 1;

const ROM_NN_START: usize = 0x4000;
const ROM_NN_END: usize = 0x7FFF;
const ROM_NN_SIZE: usize = ROM_NN_END - ROM_NN_START + 1;

const VRAM_START: usize = 0x8000;
const VRAM_END: usize = 0x9FFF;
const VRAM_SIZE: usize = VRAM_END - VRAM_START + 1;

const EXRAM_START: usize = 0xA000;
const EXRAM_END: usize = 0xBFFF;
const EXRAM_SIZE: usize = EXRAM_END - EXRAM_START + 1;

const WRAM_0_START: usize = 0xC000;
const WRAM_0_END: usize = 0xCFFF;
const WRAM_0_SIZE: usize = WRAM_0_END - WRAM_0_START + 1;

const WRAM_NN_START: usize = 0xD000;
const WRAM_NN_END: usize = 0xDFFF;
const WRAM_NN_SIZE: usize = WRAM_NN_END - WRAM_NN_START + 1;

const ECHO_RAM_START: usize = 0xE000;
const ECHO_RAM_END: usize = 0xFDFF;
const ECHO_RAM_SIZE: usize = ECHO_RAM_END - ECHO_RAM_START + 1;

const OAM_START: usize = 0xFE00;
const OAM_END: usize = 0xFE9F;
const OAM_SIZE: usize = OAM_END - OAM_START + 1;

const IO_START: usize = 0xFF00;
const IO_END: usize = 0xFF7F;
const IO_SIZE: usize = IO_END - IO_START + 1;

const HRAM_START: usize = 0xFF80;
const HRAM_END: usize = 0xFFFE;
const HRAM_SIZE: usize = HRAM_END - HRAM_START + 1;

const IE_START: usize = 0xFFFF;
const IE_END: usize = 0xFFFF;
const IE_SIZE: usize = IE_END - IE_START + 1;

pub struct Memory {
    rom_00: Mutex<[u8; ROM_00_SIZE]>,
    rom_nn: Mutex<[u8; ROM_NN_SIZE]>,
    vram: Mutex<[u8; VRAM_SIZE]>,
    exram: Mutex<[u8; EXRAM_SIZE]>,
    wram_0: Mutex<[u8; WRAM_0_SIZE]>,
    wram_nn: Mutex<[u8; WRAM_NN_SIZE]>,
    echo: Mutex<[u8; ECHO_RAM_SIZE]>,
    oam: Mutex<[u8; OAM_SIZE]>,
    io: Mutex<[u8; IO_SIZE]>,
    hram: Mutex<[u8; HRAM_SIZE]>,
    ie: Mutex<[u8; IE_SIZE]>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            rom_00: Mutex::new([0; ROM_00_SIZE]),
            rom_nn: Mutex::new([0; ROM_NN_SIZE]),
            vram: Mutex::new([0; VRAM_SIZE]),
            exram: Mutex::new([0; EXRAM_SIZE]),
            wram_0: Mutex::new([0; WRAM_0_SIZE]),
            wram_nn: Mutex::new([0; WRAM_NN_SIZE]),
            echo: Mutex::new([0; ECHO_RAM_SIZE]),
            oam: Mutex::new([0; OAM_SIZE]),
            io: Mutex::new([0; IO_SIZE]),
            hram: Mutex::new([0; HRAM_SIZE]),
            ie: Mutex::new([0; IE_SIZE]),
        }
    }

    pub fn read_byte(&self, adress: u16) -> u8 {
        let adress_as_index = usize::from(adress);
        match adress_as_index {
            ROM_00_START..=ROM_00_END => {
                self.rom_00.lock().unwrap()[adress_as_index - ROM_00_START]
            }
            ROM_NN_START..=ROM_NN_END => {
                self.rom_nn.lock().unwrap()[adress_as_index - ROM_NN_START]
            }
            VRAM_START..=VRAM_END => self.vram.lock().unwrap()[adress_as_index - VRAM_START],
            EXRAM_START..=EXRAM_END => self.exram.lock().unwrap()[adress_as_index - EXRAM_START],
            WRAM_0_START..=WRAM_0_END => {
                self.wram_0.lock().unwrap()[adress_as_index - WRAM_0_START]
            }
            WRAM_NN_START..=WRAM_NN_END => {
                self.wram_nn.lock().unwrap()[adress_as_index - WRAM_NN_START]
            }
            ECHO_RAM_START..=ECHO_RAM_END => panic!("Echo RAM not implemented"),
            OAM_START..=OAM_END => self.oam.lock().unwrap()[adress_as_index - OAM_START],
            IO_START..=IO_END => self.io.lock().unwrap()[adress_as_index - IO_START],
            HRAM_START..=HRAM_END => self.hram.lock().unwrap()[adress_as_index - HRAM_START],
            IE_START..=IE_END => self.ie.lock().unwrap()[adress_as_index - IE_START],
            _ => panic!("Invalid adress: {:#06X}", adress),
        }
    }

    pub fn write_byte(&self, adress: u16, value: u8) {
        let adress_as_index = usize::from(adress);
        match adress_as_index {
            ROM_00_START..=ROM_00_END => {
                self.rom_00.lock().unwrap()[adress_as_index - ROM_00_START] = value
            }
            ROM_NN_START..=ROM_NN_END => {
                self.rom_nn.lock().unwrap()[adress_as_index - ROM_NN_START] = value
            }
            VRAM_START..=VRAM_END => {
                self.vram.lock().unwrap()[adress_as_index - VRAM_START] = value
            }
            EXRAM_START..=EXRAM_END => {
                self.exram.lock().unwrap()[adress_as_index - EXRAM_START] = value
            }
            WRAM_0_START..=WRAM_0_END => {
                self.wram_0.lock().unwrap()[adress_as_index - WRAM_0_START] = value
            }
            WRAM_NN_START..=WRAM_NN_END => {
                self.wram_nn.lock().unwrap()[adress_as_index - WRAM_NN_START] = value
            }
            ECHO_RAM_START..=ECHO_RAM_END => panic!("Echo RAM not implemented"),
            OAM_START..=OAM_END => self.oam.lock().unwrap()[adress_as_index - OAM_START] = value,
            IO_START..=IO_END => self.io.lock().unwrap()[adress_as_index - IO_START] = value,
            HRAM_START..=HRAM_END => {
                self.hram.lock().unwrap()[adress_as_index - HRAM_START] = value
            }
            IE_START..=IE_END => self.ie.lock().unwrap()[adress_as_index - IE_START] = value,
            _ => panic!("Invalid adress: {:#06X}", adress),
        }
    }

    pub fn read_word(&self, adress: u16) -> u16 {
        let lo = self.read_byte(adress);
        let hi = self.read_byte(adress + 1);
        combine(hi, lo)
    }

    pub fn write_word(&self, adress: u16, value: u16) {
        let (hi, lo) = split(value);
        self.write_byte(adress, lo);
        self.write_byte(adress + 1, hi);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_write() {
        let memory = Memory::new();
        memory.write_byte(0x0000, 0xAB);
        assert_eq!(memory.read_byte(0x0000), 0xAB);
    }

    #[test]
    fn test_read_write_word() {
        let memory = Memory::new();
        memory.write_word(0x0000, 0xABCD);
        assert_eq!(memory.read_word(0x0000), 0xABCD);
    }
}
