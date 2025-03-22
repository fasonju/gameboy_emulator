use super::registers::Registers;

const STARTUP_AF: u16 = 0x0;
const STARTUP_BC: u16 = 0x0;
const STARTUP_DE: u16 = 0x0;
const STARTUP_HL: u16 = 0x0;
const STARTUP_SP: u16 = 0x0;
const STARTUP_PC: u16 = 0x0;

pub struct Cpu {
    registers: Registers
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: Registers::new(STARTUP_AF, 
                STARTUP_BC, 
                STARTUP_DE, 
                STARTUP_HL, 
                STARTUP_SP, 
                STARTUP_PC
            )
        }
    }

    pub fn get_af(&self) -> u16 {
        self.registers.get_af()
    }
}