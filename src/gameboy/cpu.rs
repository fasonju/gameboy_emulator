
use super::{instruction_variables::{Cond, R16, R16MEM, R8}, instructions::Instruction, memory::Memory, registers::Registers};

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
        let xx = opcode >> 6;
        let yy = (opcode >> 4) & 0x3;
        let zzzz = opcode & 0xF;
        let aaa = (opcode >> 3) & 0x7;
        let bbb = opcode & 0x7;
        let iii = opcode >> 5;
        let jj = (opcode >> 3) & 0x3;
        
        // matching any one of the three tuples is enough to match the instruction, avoid mixing usage
        match ((xx, yy, zzzz), (xx, aaa, bbb), (iii, jj, bbb)) {
            // Block 0
            ((0x0, 0x0, 0x0), _, _) => Instruction::Nop, // NOP

            ((0x0, _, 0x1), _, _) => Instruction::LdR16Imm16(R16::from(yy), self.fetch_word()), // LD R16, imm16
            ((0x0, _, 0x2), _, _) => Instruction::LdR16MemA(R16MEM::from(yy)), // LD (R16), A
            ((0x0, _, 0xA), _, _) => Instruction::LdAR16Mem(R16MEM::from(yy)), // LD A, (R16)
            ((0x0, 0x0, 0x8), _, _) => Instruction::LdMemImm16MemSP(self.fetch_word()), // LD (imm16), SP

            ((0x0, _, 0x3), _, _) => Instruction::IncR16(R16::from(yy)), // INC R16
            ((0x0, _, 0xB), _, _) => Instruction::DecR16(R16::from(yy)), // DEC R16
            ((0x0, _, 0x9), _, _) => Instruction::AddHlR16(R16::from(yy)), // ADD HL, R16

            (_, (0x0, _, 0x4), _) => Instruction::IncR8(R8::from(aaa)), // INC R8
            (_, (0x0, _, 0x5), _) => Instruction::DecR8(R8::from(aaa)), // DEC R8

            (_, (0x0, _, 0x6), _) => Instruction::LdR8Imm8(R8::from(aaa), self.fetch_byte()), // LD R8, Imm8
 
            ((0x0, 0x0, 0x7), _, _) => Instruction::Rlca, // RLCA
            ((0x0, 0x0, 0xF), _, _) => Instruction::Rrca, // RRCA
            ((0x0, 0x1, 0x7), _, _) => Instruction::Rla, // RLA
            ((0x0, 0x1, 0xF), _, _) => Instruction::Rra, // RRA
            ((0x0, 0x2, 0x7), _, _) => Instruction::Daa, // DAA
            ((0x0, 0x2, 0xF), _, _) => Instruction::Cpl, // CPL
            ((0x0, 0x3, 0x7), _, _) => Instruction::Scf, // SCF
            ((0x0, 0x3, 0xF), _, _) => Instruction::Ccf, // CCF

            (_, _, (0x0, 0x3, 0x0)) => Instruction::JrImm8(self.fetch_byte()), // JR imm8
            (_, _, (0x1, _, 0x0)) => Instruction::JrCondImm8(Cond::from(jj), self.fetch_byte()), // JR cond, imm8

            ((0x0, 0x1, 0x0), _, _) => Instruction::Stop, // STOP   

            // Block 1
            (_, (0x1, 0x6, 0x6), _) => Instruction::Halt, // HALT
            (_, (0x1, _, _), _) => Instruction::LdR8R8(R8::from(aaa), R8::from(bbb)), // LD R8, R8

            // Block 2
            (_, (0x2, 0x0, _), _) => Instruction::AddAR8(R8::from(bbb)), // ADD A, R8
            (_, (0x2, 0x1, _), _) => Instruction::AdcAR8(R8::from(bbb)), // ADC A, R8
            (_, (0x2, 0x2, _), _) => Instruction::SubAR8(R8::from(bbb)), // SUB A, R8
            (_, (0x2, 0x3, _), _) => Instruction::SbcAR8(R8::from(bbb)), // SBC A, R8
            (_, (0x2, 0x4, _), _) => Instruction::AndAR8(R8::from(bbb)), // AND A, R8
            (_, (0x2, 0x5, _), _) => Instruction::XorAR8(R8::from(bbb)), // XOR A, R8
            (_, (0x2, 0x6, _), _) => Instruction::OrAR8(R8::from(bbb)), // OR A, R8
            (_, (0x2, 0x7, _), _) => Instruction::CpAR8(R8::from(bbb)), // CP A, R8

            // Block 3

            _ => panic!("Unknown instruction: {:#04X}", opcode)

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

        cpu.memory.write_byte(24, 0x18);
        cpu.memory.write_byte(25, 0x12);
        assert_eq!(cpu.fetch_instruction(), Instruction::JrImm8(0x12));

        cpu.memory.write_byte(26, 0x20);
        cpu.memory.write_byte(27, 0x10);
        assert_eq!(cpu.fetch_instruction(), Instruction::JrCondImm8(Cond::NotZero, 0x10));

        cpu.memory.write_byte(28, 0x10);
        assert_eq!(cpu.fetch_instruction(), Instruction::Stop);

        cpu.memory.write_byte(29, 0x76);
        assert_eq!(cpu.fetch_instruction(), Instruction::Halt);

        cpu.memory.write_byte(30, 0x40);
        assert_eq!(cpu.fetch_instruction(), Instruction::LdR8R8(R8::B, R8::B));

        cpu.memory.write_byte(31, 0x80);
        assert_eq!(cpu.fetch_instruction(), Instruction::AddAR8(R8::B));

        cpu.memory.write_byte(32, 0x88);
        assert_eq!(cpu.fetch_instruction(), Instruction::AdcAR8(R8::B));

        cpu.memory.write_byte(33, 0x90);
        assert_eq!(cpu.fetch_instruction(), Instruction::SubAR8(R8::B));

        cpu.memory.write_byte(34, 0x98);
        assert_eq!(cpu.fetch_instruction(), Instruction::SbcAR8(R8::B));

        cpu.memory.write_byte(35, 0xA0);
        assert_eq!(cpu.fetch_instruction(), Instruction::AndAR8(R8::B));

        cpu.memory.write_byte(36, 0xA8);
        assert_eq!(cpu.fetch_instruction(), Instruction::XorAR8(R8::B));

        cpu.memory.write_byte(37, 0xB0);
        assert_eq!(cpu.fetch_instruction(), Instruction::OrAR8(R8::B));

        cpu.memory.write_byte(38, 0xB8);
        assert_eq!(cpu.fetch_instruction(), Instruction::CpAR8(R8::B));
    }
}
