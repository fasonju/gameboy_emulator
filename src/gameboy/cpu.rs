
use super::{instruction_variables::R16, instructions::Instruction, memory::Memory, registers::Registers};

const STARTUP_AF: u16 = 0x0;
const STARTUP_BC: u16 = 0x0;
const STARTUP_DE: u16 = 0x0;
const STARTUP_HL: u16 = 0x0;
const STARTUP_SP: u16 = 0x0;
const STARTUP_PC: u16 = 0x0;



pub struct Cpu<'a> {
    memory: &'a Memory,
    registers: Registers, 
}

impl<'a> Cpu<'a> {
    pub fn new(memory: &Memory) -> Cpu {
        Cpu {
            registers: Registers::new(STARTUP_AF, STARTUP_BC, STARTUP_DE, STARTUP_HL, STARTUP_SP, STARTUP_PC),
            memory
        }
    }

    fn fetch_byte(&mut self) -> u8 {
        let byte = self.memory.read_byte(self.registers.pc as u16);
        self.registers.pc += 1;
        byte
    }

    fn fetch_word(&mut self) -> u16 {
        let word = self.memory.read_word(self.registers.pc as u16);
        self.registers.pc += 2;
        word
    }

    fn fetch_instruction(&mut self) -> Instruction {
        // opcode == xxyyzzzz == xxaaabbb == iiijjbbb
        let opcode = self.fetch_byte(); 
    
        let xx = opcode >> 7;
        let yy = (opcode >> 4) & 0x3;
        let zzzz = opcode & 0xF;
        let aaa = (opcode >> 3) & 0x7;
        let bbb = opcode & 0x7;
        let iii = (opcode >> 6) & 0x7;
        let jj = (opcode >> 3) & 0x3;
        
        match ((xx, yy, zzzz), (xx, aaa, bbb), (iii, jj, bbb)) {
            ((0x0, 0x0, 0x0), _, _) => Instruction::Nop,
            ((0x0, _, 0x1), _, _) => Instruction::LdR16Imm16(R16::from(yy), self.fetch_word()),



            _ => panic!("Unknown instruction")

        }
    }
}
