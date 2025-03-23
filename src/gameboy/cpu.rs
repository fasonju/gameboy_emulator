
use super::{ memory::Memory, registers::Registers};

const STARTUP_AF: u16 = 0x0;
const STARTUP_BC: u16 = 0x0;
const STARTUP_DE: u16 = 0x0;
const STARTUP_HL: u16 = 0x0;
const STARTUP_SP: u16 = 0x0;
const STARTUP_PC: u16 = 0x0;

pub enum Register8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    F, // Flags register
}

pub enum Register16 {
    AF,
    BC,
    DE,
    HL,
    SP, // Stack Pointer
    PC, // Program Counter
}

pub struct Cpu<'a> {
    registers: Registers,
    memory: &'a Memory
}

impl<'a> Cpu<'a> {
    pub fn new(memory: &Memory) -> Cpu {
        Cpu {
            registers: Registers::new(STARTUP_AF, 
                STARTUP_BC, 
                STARTUP_DE, 
                STARTUP_HL, 
                STARTUP_SP, 
                STARTUP_PC
            ),
            memory
        }
    }

    /// Fetches the next byte from memory and increments the program counter
    fn fetch_byte(&mut self) -> u8 {
        let pc = self.registers.get_register_16(Register16::PC);
        let byte = self.memory.read_byte(pc);
        self.registers.set_register_16(Register16::PC, pc + 1);
        byte
    }

    /// Fetches the next word from memory and increments the program counter
    fn fetch_word(&mut self) -> u16 {
        let pc = self.registers.get_register_16(Register16::PC);
        let word = self.memory.read_word(pc);
        self.registers.set_register_16(Register16::PC, pc + 2);
        word
    }
}
