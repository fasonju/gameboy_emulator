
use super::{instruction_variables::{Cond, B3, R16, R16MEM, R16STK, R8}, instructions::Instruction, memory::Memory, registers::Registers};

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
            ((0x3, 0x0, 0x6), _, _) => Instruction::AddAImm8(self.fetch_byte()), // ADD A, imm8
            ((0x3, 0x0, 0xE), _, _) => Instruction::AdcAImm8(self.fetch_byte()), // ADC A, imm8
            ((0x3, 0x1, 0x6), _, _) => Instruction::SubAImm8(self.fetch_byte()), // SUB A, imm8
            ((0x3, 0x1, 0xE), _, _) => Instruction::SbcAImm8(self.fetch_byte()), // SBC A, imm8
            ((0x3, 0x2, 0x6), _, _) => Instruction::AndAImm8(self.fetch_byte()), // AND A, imm8
            ((0x3, 0x2, 0xE), _, _) => Instruction::XorAImm8(self.fetch_byte()), // XOR A, imm8
            ((0x3, 0x3, 0x6), _, _) => Instruction::OrAImm8(self.fetch_byte()), // OR A, imm8
            ((0x3, 0x3, 0xE), _, _) => Instruction::CpAImm8(self.fetch_byte()), // CP A, imm8

            (_, _, (0x6, _, 0x0)) => Instruction::RetCond(Cond::from(jj)), // RET cond
            (_, _, (0x6, 0x1, 0x1)) => Instruction::Ret, // RET
            (_, _, (0x6, 0x3, 0x1)) => Instruction::Reti, // RETI
            (_, _, (0x6, _, 0x2)) => Instruction::JpCondImm16(Cond::from(jj), self.fetch_word()), // JP cond, imm16
            (_, _, (0x6, 0x0, 0x3)) => Instruction::JpImm16(self.fetch_word()), // JP imm16
            (_, _, (0x7, 0x1, 0x1)) => Instruction::JpHl, // JP HL
            (_, _, (0x6, _, 0x4)) => Instruction::CallCondImm16(Cond::from(jj), self.fetch_word()), // CALL cond, imm16
            (_, _, (0x6, 0x1, 0x5)) => Instruction::CallImm16(self.fetch_word()), // CALL imm16
            (_, (0x3, _, 0x7), _) => Instruction::RstTgt3(B3::from(aaa)), // RST tgt3

            ((0x3, _, 0x1), _, _) => Instruction::PopR16Stk(R16STK::from(yy)), // POP R16
            ((0x3, _, 0x5), _, _) => Instruction::PushR16Stk(R16STK::from(yy)), // PUSH R16

            ((0x3, 0x0, 0xB), _, _) => map_prefixed_instruction(self.fetch_byte()), // CB

            ((0x3, 0x2, 0x2), _, _) => Instruction::LdMemCA, // LD (C), A
            ((0x3, 0x2, 0x0), _, _) => Instruction::LdhMemImm8A(self.fetch_byte()), // LDH (imm8), A
            ((0x3, 0x2, 0xA), _, _) => Instruction::LdMemImm16A(self.fetch_word()), // LD (imm16), A
            ((0x3, 0x3, 0x2), _, _) => Instruction::LdAMemC, // LD A, (C)
            ((0x3, 0x3, 0x0), _, _) => Instruction::LdhAMemImm8(self.fetch_byte()), // LDH A, (imm8)
            ((0x3, 0x3, 0xA), _, _) => Instruction::LdAMemImm16(self.fetch_word()), // LD A, (imm16)

            ((0x3, 0x2, 0x8), _, _) => Instruction::AddSpImm8(self.fetch_byte()),
            ((0x3, 0x3, 0x8), _, _) => Instruction::LdHlSpImm8(self.fetch_byte()),
            ((0x3, 0x3, 0x9), _, _) => Instruction::LdSpHl,

            ((0x3, 0x3, 0x3), _, _) => Instruction::Di,
            ((0x3, 0x3, 0xB), _, _) => Instruction::Ei,

            _ => panic!("Unknown instruction: {:#04X}", opcode)
            

        }
    }
}

fn map_prefixed_instruction(byte: u8) -> Instruction {
    let xx = byte >> 6;
    let aaa = (byte >> 3) & 0x7;
    let bbb = byte & 0x7;
    match (xx, aaa, bbb) {
            (0x0, 0x0, _) => Instruction::RlcR8(R8::from(bbb)),
            (0x0, 0x1, _) => Instruction::RrcR8(R8::from(bbb)),
            (0x0, 0x2, _) => Instruction::RlR8(R8::from(bbb)),
            (0x0, 0x3, _) => Instruction::RrR8(R8::from(bbb)),
            (0x0, 0x4, _) => Instruction::SlaR8(R8::from(bbb)),
            (0x0, 0x5, _) => Instruction::SraR8(R8::from(bbb)),
            (0x0, 0x6, _) => Instruction::SwapR8(R8::from(bbb)),
            (0x0, 0x7, _) => Instruction::SrlR8(R8::from(bbb)),
        _ => panic!("Unknown prefixed instruction: {:#04X}", byte)
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

        cpu.memory.write_byte(39, 0xC6);
        cpu.memory.write_byte(40, 0x12);
        assert_eq!(cpu.fetch_instruction(), Instruction::AddAImm8(0x12));

        cpu.memory.write_byte(41, 0xCE);
        cpu.memory.write_byte(42, 0x12);
        assert_eq!(cpu.fetch_instruction(), Instruction::AdcAImm8(0x12));

        cpu.memory.write_byte(43, 0xD6);
        cpu.memory.write_byte(44, 0x12);
        assert_eq!(cpu.fetch_instruction(), Instruction::SubAImm8(0x12));

        cpu.memory.write_byte(45, 0xDE);
        cpu.memory.write_byte(46, 0x12);
        assert_eq!(cpu.fetch_instruction(), Instruction::SbcAImm8(0x12));

        cpu.memory.write_byte(47, 0xE6);
        cpu.memory.write_byte(48, 0x12);
        assert_eq!(cpu.fetch_instruction(), Instruction::AndAImm8(0x12));

        cpu.memory.write_byte(49, 0xEE);
        cpu.memory.write_byte(50, 0x12);
        assert_eq!(cpu.fetch_instruction(), Instruction::XorAImm8(0x12));

        cpu.memory.write_byte(51, 0xF6);
        cpu.memory.write_byte(52, 0x12);
        assert_eq!(cpu.fetch_instruction(), Instruction::OrAImm8(0x12));

        cpu.memory.write_byte(53, 0xFE);
        cpu.memory.write_byte(54, 0x12);
        assert_eq!(cpu.fetch_instruction(), Instruction::CpAImm8(0x12));

        cpu.memory.write_byte(55, 0xC0);
        assert_eq!(cpu.fetch_instruction(), Instruction::RetCond(Cond::NotZero));

        cpu.memory.write_byte(56, 0xC9);
        assert_eq!(cpu.fetch_instruction(), Instruction::Ret);

        cpu.memory.write_byte(57, 0xD9);
        assert_eq!(cpu.fetch_instruction(), Instruction::Reti);
        
        cpu.memory.write_byte(58, 0xC2);
        cpu.memory.write_byte(59, 0x12);
        cpu.memory.write_byte(60, 0x34);
        assert_eq!(cpu.fetch_instruction(), Instruction::JpCondImm16(Cond::NotZero, 0x3412));

        cpu.memory.write_byte(61, 0xC3);
        cpu.memory.write_byte(62, 0x12);
        cpu.memory.write_byte(63, 0x34);
        assert_eq!(cpu.fetch_instruction(), Instruction::JpImm16(0x3412));

        cpu.memory.write_byte(64, 0xE9);
        assert_eq!(cpu.fetch_instruction(), Instruction::JpHl);
        
        cpu.memory.write_byte(65, 0xC4);
        cpu.memory.write_byte(66, 0x12);
        cpu.memory.write_byte(67, 0x34);
        assert_eq!(cpu.fetch_instruction(), Instruction::CallCondImm16(Cond::NotZero, 0x3412));

        cpu.memory.write_byte(68, 0xCD);
        cpu.memory.write_byte(69, 0x12);
        cpu.memory.write_byte(70, 0x34);
        assert_eq!(cpu.fetch_instruction(), Instruction::CallImm16(0x3412));

        cpu.memory.write_byte(71, 0xC7);
        assert_eq!(cpu.fetch_instruction(), Instruction::RstTgt3(B3::Zero));

        cpu.memory.write_byte(72, 0xC1);
        assert_eq!(cpu.fetch_instruction(), Instruction::PopR16Stk(R16STK::BC));

        cpu.memory.write_byte(73, 0xC5);
        assert_eq!(cpu.fetch_instruction(), Instruction::PushR16Stk(R16STK::BC));

        cpu.memory.write_byte(74, 0xCB);
        cpu.memory.write_byte(75, 0x00);
        assert_eq!(cpu.fetch_instruction(), Instruction::RlcR8(R8::B));

        cpu.memory.write_byte(76, 0xCB);
        cpu.memory.write_byte(77, 0x08);
        assert_eq!(cpu.fetch_instruction(), Instruction::RrcR8(R8::B));

        cpu.memory.write_byte(78, 0xCB);
        cpu.memory.write_byte(79, 0x10);
        assert_eq!(cpu.fetch_instruction(), Instruction::RlR8(R8::B));

        cpu.memory.write_byte(80, 0xCB);
        cpu.memory.write_byte(81, 0x18);
        assert_eq!(cpu.fetch_instruction(), Instruction::RrR8(R8::B));

        cpu.memory.write_byte(82, 0xCB);
        cpu.memory.write_byte(83, 0x20);
        assert_eq!(cpu.fetch_instruction(), Instruction::SlaR8(R8::B));

        cpu.memory.write_byte(84, 0xCB);
        cpu.memory.write_byte(85, 0x28);
        assert_eq!(cpu.fetch_instruction(), Instruction::SraR8(R8::B));

        cpu.memory.write_byte(86, 0xCB);
        cpu.memory.write_byte(87, 0x30);
        assert_eq!(cpu.fetch_instruction(), Instruction::SwapR8(R8::B));

        cpu.memory.write_byte(88, 0xCB);
        cpu.memory.write_byte(89, 0x38);
        assert_eq!(cpu.fetch_instruction(), Instruction::SrlR8(R8::B));

        cpu.memory.write_byte(90, 0xE2);
        assert_eq!(cpu.fetch_instruction(), Instruction::LdMemCA);

        cpu.memory.write_byte(91, 0xE0);
        cpu.memory.write_byte(92, 0x12);
        assert_eq!(cpu.fetch_instruction(), Instruction::LdhMemImm8A(0x12));

        cpu.memory.write_byte(93, 0xEA);
        cpu.memory.write_byte(94, 0x34);
        cpu.memory.write_byte(95, 0x12);
        assert_eq!(cpu.fetch_instruction(), Instruction::LdMemImm16A(0x1234));

        cpu.memory.write_byte(96, 0xF2);
        assert_eq!(cpu.fetch_instruction(), Instruction::LdAMemC);

        cpu.memory.write_byte(97, 0xF0);
        cpu.memory.write_byte(98, 0x12);
        assert_eq!(cpu.fetch_instruction(), Instruction::LdhAMemImm8(0x12));

        cpu.memory.write_byte(99, 0xFA);
        cpu.memory.write_byte(100, 0x34);
        cpu.memory.write_byte(101, 0x12);
        assert_eq!(cpu.fetch_instruction(), Instruction::LdAMemImm16(0x1234));

        cpu.memory.write_byte(102, 0xE8);
        cpu.memory.write_byte(103, 0x12);
        assert_eq!(cpu.fetch_instruction(), Instruction::AddSpImm8(0x12));

        cpu.memory.write_byte(104, 0xF8);
        cpu.memory.write_byte(105, 0x12);
        assert_eq!(cpu.fetch_instruction(), Instruction::LdHlSpImm8(0x12));

        cpu.memory.write_byte(106, 0xF9);
        assert_eq!(cpu.fetch_instruction(), Instruction::LdSpHl);

        cpu.memory.write_byte(107, 0xF3);
        assert_eq!(cpu.fetch_instruction(), Instruction::Di);

        cpu.memory.write_byte(108, 0xFB);
        assert_eq!(cpu.fetch_instruction(), Instruction::Ei);

    }
}
