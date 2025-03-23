
use super::{instruction_variables::{R16, R16MEM, R8}, instructions::Instruction, memory::Memory, registers::Registers};

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
        
        // matching any one of the three tuples is enough to match the instruction, avoid mixing usage
        match ((xx, yy, zzzz), (xx, aaa, bbb), (iii, jj, bbb)) {
            ((0x0, 0x0, 0x0), _, _) => Instruction::Nop, // NOP

            ((0x0, _, 0x1), _, _) => Instruction::LdR16Imm16(R16::from(yy), self.fetch_word()), // LD R16, imm16
            ((0x0, _, 0x2), _, _) => Instruction::LdR16MemA(R16MEM::from(yy)), // LD (R16), A
            ((0x0, _, 0xA), _, _) => Instruction::LdAR16Mem(R16MEM::from(yy)), // LD A, (R16)
            ((0x0, 0x0, 0x8), _, _) => Instruction::LdMemImm16MemSP(self.fetch_word()), // LD (imm16), SP

            ((0x0, _, 0x3), _, _) => Instruction::IncR16(R16::from(yy)),
            ((0x0, _, 0xB), _, _) => Instruction::DecR16(R16::from(yy)),
            ((0x0, _, 0x9), _, _) => Instruction::AddHlR16(R16::from(yy)), 

            (_, (0x0, _, 0x4), _) => Instruction::IncR8(R8::from(aaa)),
            (_, (0x0, _, 0x5), _) => Instruction::DecR8(R8::from(aaa)),

            (_, (0x0, _, 0x6), _) => Instruction::LdR8Imm8(R8::from(aaa), self.fetch_byte()),

            ((0x0, 0x0, 0x7), _, _) => Instruction::Rlca,
            ((0x0, 0x0, 0xF), _, _) => Instruction::Rrca,
            ((0x0, 0x1, 0x7), _, _) => Instruction::Rla,
            ((0x0, 0x1, 0xF), _, _) => Instruction::Rra,
            ((0x0, 0x2, 0x7), _, _) => Instruction::Daa,
            ((0x0, 0x2, 0xF), _, _) => Instruction::Cpl,
            ((0x0, 0x3, 0x7), _, _) => Instruction::Scf,
            ((0x0, 0x3, 0xF), _, _) => Instruction::Ccf,

            





            _ => panic!("Unknown instruction")

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_instruction() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);

        cpu.memory.write_byte(0, 0x00);
        assert_eq!(cpu.fetch_instruction(), Instruction::Nop);

        cpu.memory.write_byte(1, 0x01);
        cpu.memory.write_byte(2, 0x12); // imm16
        assert_eq!(cpu.fetch_instruction(), Instruction::LdR16Imm16(R16::BC, 0x12));

        cpu.memory.write_byte(4, 0x02);
        assert_eq!(cpu.fetch_instruction(), Instruction::LdR16MemA(R16MEM::BC));

        cpu.memory.write_byte(5, 0x0A);
        assert_eq!(cpu.fetch_instruction(), Instruction::LdAR16Mem(R16MEM::BC));

        cpu.memory.write_byte(6, 0x08);
        cpu.memory.write_byte(7, 0x34);
        cpu.memory.write_byte(8, 0x12);
        assert_eq!(cpu.fetch_instruction(), Instruction::LdMemImm16MemSP(0x1234));

        cpu.memory.write_byte(9, 0x03);
        assert_eq!(cpu.fetch_instruction(), Instruction::IncR16(R16::BC));

        cpu.memory.write_byte(10, 0x0B);
        assert_eq!(cpu.fetch_instruction(), Instruction::DecR16(R16::BC));

        cpu.memory.write_byte(11, 0x09);
        assert_eq!(cpu.fetch_instruction(), Instruction::AddHlR16(R16::BC));

        cpu.memory.write_byte(12, 0x04);
        assert_eq!(cpu.fetch_instruction(), Instruction::IncR8(R8::B));

        cpu.memory.write_byte(13, 0x05);
        assert_eq!(cpu.fetch_instruction(), Instruction::DecR8(R8::B));

        cpu.memory.write_byte(14, 0x06);
        cpu.memory.write_byte(15, 0x12);
        assert_eq!(cpu.fetch_instruction(), Instruction::LdR8Imm8(R8::B, 0x12));

        cpu.memory.write_byte(16, 0x07);
        assert_eq!(cpu.fetch_instruction(), Instruction::Rlca);

        cpu.memory.write_byte(17, 0x0F);
        assert_eq!(cpu.fetch_instruction(), Instruction::Rrca);

        cpu.memory.write_byte(18, 0x17);
        assert_eq!(cpu.fetch_instruction(), Instruction::Rla);

        cpu.memory.write_byte(19, 0x1F);
        assert_eq!(cpu.fetch_instruction(), Instruction::Rra);

        cpu.memory.write_byte(20, 0x27);
        assert_eq!(cpu.fetch_instruction(), Instruction::Daa);

        cpu.memory.write_byte(21, 0x2F);
        assert_eq!(cpu.fetch_instruction(), Instruction::Cpl);

        cpu.memory.write_byte(22, 0x37);
        assert_eq!(cpu.fetch_instruction(), Instruction::Scf);

        cpu.memory.write_byte(23, 0x3F);
        assert_eq!(cpu.fetch_instruction(), Instruction::Ccf);
    }
}
