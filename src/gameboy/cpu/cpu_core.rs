use crate::gameboy::Memory;

use super::{
    instruction_variables::{Cond, B3, R16, R16MEM, R16STK, R8, TGT3},
    instructions::Instruction,
    registers::Registers,
};

const STARTUP_AF: u16 = 0x0;
const STARTUP_BC: u16 = 0x0;
const STARTUP_DE: u16 = 0x0;
const STARTUP_HL: u16 = 0x0;
const STARTUP_SP: u16 = 0x0;
const STARTUP_PC: u16 = 0x0;

pub struct Cpu {
    pub registers: Registers,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: Registers::new(
                STARTUP_AF, STARTUP_BC, STARTUP_DE, STARTUP_HL, STARTUP_SP, STARTUP_PC,
            ),
        }
    }

    fn fetch_byte(&mut self, memory: &Memory) -> u8 {
        let byte = memory.read_byte(self.registers.pc);
        self.registers.pc += 1;
        byte
    }

    fn fetch_word(&mut self, memory: &Memory) -> u16 {
        let word = memory.read_word(self.registers.pc);
        self.registers.pc += 2;
        word
    }

    fn fetch_instruction(&mut self, memory: &Memory) -> Instruction {
        // opcode == xxyyzzzz == xxaaabbb == iiijjbbb
        let opcode = self.fetch_byte(memory);
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

            ((0x0, _, 0x1), _, _) => {
                Instruction::LdR16Imm16(R16::from(yy), self.fetch_word(memory))
            } // LD R16, imm16
            ((0x0, _, 0x2), _, _) => Instruction::LdR16MemA(R16MEM::from(yy)), // LD (R16), A
            ((0x0, _, 0xA), _, _) => Instruction::LdAR16Mem(R16MEM::from(yy)), // LD A, (R16)
            ((0x0, 0x0, 0x8), _, _) => Instruction::LdMemImm16SP(self.fetch_word(memory)), // LD (imm16), SP

            ((0x0, _, 0x3), _, _) => Instruction::IncR16(R16::from(yy)), // INC R16
            ((0x0, _, 0xB), _, _) => Instruction::DecR16(R16::from(yy)), // DEC R16
            ((0x0, _, 0x9), _, _) => Instruction::AddHlR16(R16::from(yy)), // ADD HL, R16

            (_, (0x0, 0x6, 0x4), _) => Instruction::IncMemHl,
            (_, (0x0, _, 0x4), _) => Instruction::IncR8(R8::from(aaa)), // INC R8
            (_, (0x0, 0x6, 0x5), _) => Instruction::DecMemHl,
            (_, (0x0, _, 0x5), _) => Instruction::DecR8(R8::from(aaa)), // DEC R8

            (_, (0x0, 0x6, 0x6), _) => Instruction::LdMemHlImm8(self.fetch_byte(memory)),
            (_, (0x0, _, 0x6), _) => Instruction::LdR8Imm8(R8::from(aaa), self.fetch_byte(memory)), // LD R8, Imm8

            ((0x0, 0x0, 0x7), _, _) => Instruction::Rlca, // RLCA
            ((0x0, 0x0, 0xF), _, _) => Instruction::Rrca, // RRCA
            ((0x0, 0x1, 0x7), _, _) => Instruction::Rla,  // RLA
            ((0x0, 0x1, 0xF), _, _) => Instruction::Rra,  // RRA
            ((0x0, 0x2, 0x7), _, _) => Instruction::Daa,  // DAA
            ((0x0, 0x2, 0xF), _, _) => Instruction::Cpl,  // CPL
            ((0x0, 0x3, 0x7), _, _) => Instruction::Scf,  // SCF
            ((0x0, 0x3, 0xF), _, _) => Instruction::Ccf,  // CCF

            // Note: offset is signed
            (_, _, (0x0, 0x3, 0x0)) => Instruction::JrImm8(self.fetch_byte(memory)), // JR imm8
            (_, _, (0x1, _, 0x0)) => {
                Instruction::JrCondImm8(Cond::from(jj), self.fetch_byte(memory))
            } // JR cond, imm8

            ((0x0, 0x1, 0x0), _, _) => Instruction::Stop, // STOP

            // Block 1
            (_, (0x1, 0x6, 0x6), _) => Instruction::Halt, // HALT
            (_, (0x1, 0x6, _), _) => Instruction::LdMemHlR8(R8::from(bbb)), // LD (HL), R8
            (_, (0x1, _, 0x6), _) => Instruction::LdR8MemHl(R8::from(aaa)), // LD R8, (HL)
            (_, (0x1, _, _), _) => Instruction::LdR8R8(R8::from(aaa), R8::from(bbb)), // LD R8, R8

            // Block 2
            (_, (0x2, 0x0, 0x6), _) => Instruction::AddAMemHl,
            (_, (0x2, 0x0, _), _) => Instruction::AddAR8(R8::from(bbb)), // ADD A, R8
            (_, (0x2, 0x1, 0x6), _) => Instruction::AdcAMemHl,
            (_, (0x2, 0x1, _), _) => Instruction::AdcAR8(R8::from(bbb)), // ADC A, R8
            (_, (0x2, 0x2, 0x6), _) => Instruction::SubAMemHl,
            (_, (0x2, 0x2, _), _) => Instruction::SubAR8(R8::from(bbb)), // SUB A, R8
            (_, (0x2, 0x3, 0x6), _) => Instruction::SbcAMemHl,
            (_, (0x2, 0x3, _), _) => Instruction::SbcAR8(R8::from(bbb)), // SBC A, R8
            (_, (0x2, 0x4, 0x6), _) => Instruction::AndAMemHl,
            (_, (0x2, 0x4, _), _) => Instruction::AndAR8(R8::from(bbb)), // AND A, R8
            (_, (0x2, 0x5, 0x6), _) => Instruction::XorAMemHl,
            (_, (0x2, 0x5, _), _) => Instruction::XorAR8(R8::from(bbb)), // XOR A, R8
            (_, (0x2, 0x6, 0x6), _) => Instruction::OrAMemHl,
            (_, (0x2, 0x6, _), _) => Instruction::OrAR8(R8::from(bbb)), // OR A, R8
            (_, (0x2, 0x7, 0x6), _) => Instruction::CpAMemHl,
            (_, (0x2, 0x7, _), _) => Instruction::CpAR8(R8::from(bbb)), // CP A, R8

            // Block 3
            ((0x3, 0x0, 0x6), _, _) => Instruction::AddAImm8(self.fetch_byte(memory)), // ADD A, imm8
            ((0x3, 0x0, 0xE), _, _) => Instruction::AdcAImm8(self.fetch_byte(memory)), // ADC A, imm8
            ((0x3, 0x1, 0x6), _, _) => Instruction::SubAImm8(self.fetch_byte(memory)), // SUB A, imm8
            ((0x3, 0x1, 0xE), _, _) => Instruction::SbcAImm8(self.fetch_byte(memory)), // SBC A, imm8
            ((0x3, 0x2, 0x6), _, _) => Instruction::AndAImm8(self.fetch_byte(memory)), // AND A, imm8
            ((0x3, 0x2, 0xE), _, _) => Instruction::XorAImm8(self.fetch_byte(memory)), // XOR A, imm8
            ((0x3, 0x3, 0x6), _, _) => Instruction::OrAImm8(self.fetch_byte(memory)),  // OR A, imm8
            ((0x3, 0x3, 0xE), _, _) => Instruction::CpAImm8(self.fetch_byte(memory)),  // CP A, imm8

            (_, _, (0x6, _, 0x0)) => Instruction::RetCond(Cond::from(jj)), // RET cond
            (_, _, (0x6, 0x1, 0x1)) => Instruction::Ret,                   // RET
            (_, _, (0x6, 0x3, 0x1)) => Instruction::Reti,                  // RETI
            (_, _, (0x6, _, 0x2)) => {
                Instruction::JpCondImm16(Cond::from(jj), self.fetch_word(memory))
            } // JP cond, imm16
            (_, _, (0x6, 0x0, 0x3)) => Instruction::JpImm16(self.fetch_word(memory)), // JP imm16
            (_, _, (0x7, 0x1, 0x1)) => Instruction::JpHl,                  // JP HL
            (_, _, (0x6, _, 0x4)) => {
                Instruction::CallCondImm16(Cond::from(jj), self.fetch_word(memory))
            } // CALL cond, imm16
            (_, _, (0x6, 0x1, 0x5)) => Instruction::CallImm16(self.fetch_word(memory)), // CALL imm16
            (_, (0x3, _, 0x7), _) => Instruction::RstTgt3(TGT3::from(aaa)),             // RST tgt3

            ((0x3, _, 0x1), _, _) => Instruction::PopR16Stk(R16STK::from(yy)), // POP R16
            ((0x3, _, 0x5), _, _) => Instruction::PushR16Stk(R16STK::from(yy)), // PUSH R16

            ((0x3, 0x0, 0xB), _, _) => map_prefixed_instruction(self.fetch_byte(memory)), // CB

            ((0x3, 0x2, 0x2), _, _) => Instruction::LdhMemCA, // LD (C), A
            ((0x3, 0x2, 0x0), _, _) => Instruction::LdhMemImm8A(self.fetch_byte(memory)), // LDH (imm8), A
            ((0x3, 0x2, 0xA), _, _) => Instruction::LdMemImm16A(self.fetch_word(memory)), // LD (imm16), A
            ((0x3, 0x3, 0x2), _, _) => Instruction::LdAMemC, // LD A, (C)
            ((0x3, 0x3, 0x0), _, _) => Instruction::LdhAMemImm8(self.fetch_byte(memory)), // LDH A, (imm8)
            ((0x3, 0x3, 0xA), _, _) => Instruction::LdAMemImm16(self.fetch_word(memory)), // LD A, (imm16)

            ((0x3, 0x2, 0x8), _, _) => Instruction::AddSpImm8(self.fetch_byte(memory)),
            ((0x3, 0x3, 0x8), _, _) => Instruction::LdHlSpImm8(self.fetch_byte(memory)),
            ((0x3, 0x3, 0x9), _, _) => Instruction::LdSpHl,

            ((0x3, 0x3, 0x3), _, _) => Instruction::Di,
            ((0x3, 0x3, 0xB), _, _) => Instruction::Ei,

            _ => panic!("Unknown instruction: {:#04X}", opcode),
        }
    }

    pub fn tick(&mut self, memory: &mut Memory) -> u8 {
        let instruction = self.fetch_instruction(memory);
        instruction.execute(self, memory)
    }
}

fn map_prefixed_instruction(byte: u8) -> Instruction {
    let xx = byte >> 6;
    let aaa = (byte >> 3) & 0x7;
    let bbb = byte & 0x7;
    match (xx, aaa, bbb) {
        (0x0, 0x0, 0x6) => Instruction::RlcMemHl,
        (0x0, 0x0, _) => Instruction::RlcR8(R8::from(bbb)),
        (0x0, 0x1, 0x6) => Instruction::RrcMemHl,
        (0x0, 0x1, _) => Instruction::RrcR8(R8::from(bbb)),
        (0x0, 0x2, 0x6) => Instruction::RlMemHl,
        (0x0, 0x2, _) => Instruction::RlR8(R8::from(bbb)),
        (0x0, 0x3, 0x6) => Instruction::RrMemHl,
        (0x0, 0x3, _) => Instruction::RrR8(R8::from(bbb)),
        (0x0, 0x4, 0x6) => Instruction::SlaMemHl,
        (0x0, 0x4, _) => Instruction::SlaR8(R8::from(bbb)),
        (0x0, 0x5, 0x6) => Instruction::SraMemHl,
        (0x0, 0x5, _) => Instruction::SraR8(R8::from(bbb)),
        (0x0, 0x6, 0x6) => Instruction::SwapMemHl,
        (0x0, 0x6, _) => Instruction::SwapR8(R8::from(bbb)),
        (0x0, 0x7, 0x6) => Instruction::SrlMemHl,
        (0x0, 0x7, _) => Instruction::SrlR8(R8::from(bbb)),

        (0x1, _, 0x6) => Instruction::BitB3MemHl(B3::from(aaa)),
        (0x1, _, _) => Instruction::BitB3R8(B3::from(aaa), R8::from(bbb)),
        (0x2, _, 0x6) => Instruction::ResB3MemHl(B3::from(aaa)),
        (0x2, _, _) => Instruction::ResB3R8(B3::from(aaa), R8::from(bbb)),
        (0x3, _, 0x6) => Instruction::SetB3MemHl(B3::from(aaa)),
        (0x3, _, _) => Instruction::SetB3R8(B3::from(aaa), R8::from(bbb)),
        _ => panic!("Unknown prefixed instruction: {:#04X}", byte),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_instruction() {
        let memory = Memory::new();
        let mut cpu = Cpu::new();

        memory.write_byte(0, 0x00);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::Nop);

        memory.write_byte(1, 0x01);
        memory.write_byte(2, 0x34);
        memory.write_byte(3, 0x12);
        assert_eq!(
            cpu.fetch_instruction(&memory),
            Instruction::LdR16Imm16(R16::BC, 0x1234)
        );

        memory.write_byte(4, 0x02);
        assert_eq!(
            cpu.fetch_instruction(&memory),
            Instruction::LdR16MemA(R16MEM::BC)
        );

        memory.write_byte(5, 0x0A);
        assert_eq!(
            cpu.fetch_instruction(&memory),
            Instruction::LdAR16Mem(R16MEM::BC)
        );

        memory.write_byte(6, 0x08);
        memory.write_byte(7, 0x34);
        memory.write_byte(8, 0x12);
        assert_eq!(
            cpu.fetch_instruction(&memory),
            Instruction::LdMemImm16SP(0x1234)
        );

        memory.write_byte(9, 0x03);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::IncR16(R16::BC));

        memory.write_byte(10, 0x0B);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::DecR16(R16::BC));

        memory.write_byte(11, 0x09);
        assert_eq!(
            cpu.fetch_instruction(&memory),
            Instruction::AddHlR16(R16::BC)
        );

        memory.write_byte(12, 0x04);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::IncR8(R8::B));

        memory.write_byte(13, 0x34);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::IncMemHl);

        memory.write_byte(14, 0x05);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::DecR8(R8::B));

        memory.write_byte(15, 0x35);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::DecMemHl);

        memory.write_byte(16, 0x06);
        memory.write_byte(17, 0x12);
        assert_eq!(
            cpu.fetch_instruction(&memory),
            Instruction::LdR8Imm8(R8::B, 0x12)
        );
        println!("reached");

        memory.write_byte(18, 0x36);
        memory.write_byte(19, 0x12);
        assert_eq!(
            cpu.fetch_instruction(&memory),
            Instruction::LdMemHlImm8(0x12)
        );
        println!("reached");

        memory.write_byte(20, 0x07);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::Rlca);

        memory.write_byte(21, 0x0F);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::Rrca);

        memory.write_byte(22, 0x17);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::Rla);

        memory.write_byte(23, 0x1F);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::Rra);

        memory.write_byte(24, 0x27);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::Daa);

        memory.write_byte(25, 0x2F);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::Cpl);

        memory.write_byte(26, 0x37);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::Scf);

        memory.write_byte(27, 0x3F);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::Ccf);

        memory.write_byte(28, 0x18);
        memory.write_byte(29, 0x12);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::JrImm8(0x12));

        memory.write_byte(30, 0x20);
        memory.write_byte(31, 0x10);
        assert_eq!(
            cpu.fetch_instruction(&memory),
            Instruction::JrCondImm8(Cond::NotZero, 0x10)
        );

        memory.write_byte(32, 0x10);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::Stop);

        memory.write_byte(33, 0x76);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::Halt);

        memory.write_byte(34, 0x40);
        assert_eq!(
            cpu.fetch_instruction(&memory),
            Instruction::LdR8R8(R8::B, R8::B)
        );

        memory.write_byte(35, 0x46);
        assert_eq!(
            cpu.fetch_instruction(&memory),
            Instruction::LdR8MemHl(R8::B)
        );

        memory.write_byte(36, 0x70);
        assert_eq!(
            cpu.fetch_instruction(&memory),
            Instruction::LdMemHlR8(R8::B)
        );

        memory.write_byte(37, 0x80);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::AddAR8(R8::B));

        memory.write_byte(38, 0x86);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::AddAMemHl);

        memory.write_byte(39, 0x88);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::AdcAR8(R8::B));

        memory.write_byte(40, 0x8E);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::AdcAMemHl);

        memory.write_byte(41, 0x90);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::SubAR8(R8::B));

        memory.write_byte(42, 0x96);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::SubAMemHl);

        memory.write_byte(43, 0x98);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::SbcAR8(R8::B));

        memory.write_byte(44, 0x9E);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::SbcAMemHl);

        memory.write_byte(45, 0xA0);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::AndAR8(R8::B));

        memory.write_byte(46, 0xA6);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::AndAMemHl);

        memory.write_byte(47, 0xA8);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::XorAR8(R8::B));

        memory.write_byte(48, 0xAE);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::XorAMemHl);

        memory.write_byte(49, 0xB0);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::OrAR8(R8::B));

        memory.write_byte(50, 0xB6);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::OrAMemHl);

        memory.write_byte(51, 0xB8);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::CpAR8(R8::B));

        memory.write_byte(52, 0xBE);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::CpAMemHl);

        memory.write_byte(53, 0xC6);
        memory.write_byte(54, 0x12);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::AddAImm8(0x12));

        memory.write_byte(55, 0xCE);
        memory.write_byte(56, 0x12);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::AdcAImm8(0x12));

        memory.write_byte(57, 0xD6);
        memory.write_byte(58, 0x12);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::SubAImm8(0x12));

        memory.write_byte(59, 0xDE);
        memory.write_byte(60, 0x12);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::SbcAImm8(0x12));

        memory.write_byte(61, 0xE6);
        memory.write_byte(62, 0x12);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::AndAImm8(0x12));

        memory.write_byte(63, 0xEE);
        memory.write_byte(64, 0x12);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::XorAImm8(0x12));

        memory.write_byte(65, 0xF6);
        memory.write_byte(66, 0x12);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::OrAImm8(0x12));

        memory.write_byte(67, 0xFE);
        memory.write_byte(68, 0x12);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::CpAImm8(0x12));

        memory.write_byte(69, 0xC0);
        assert_eq!(
            cpu.fetch_instruction(&memory),
            Instruction::RetCond(Cond::NotZero)
        );

        memory.write_byte(70, 0xC9);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::Ret);

        memory.write_byte(71, 0xD9);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::Reti);

        memory.write_byte(72, 0xC2);
        memory.write_byte(73, 0x12);
        memory.write_byte(74, 0x34);
        assert_eq!(
            cpu.fetch_instruction(&memory),
            Instruction::JpCondImm16(Cond::NotZero, 0x3412)
        );

        memory.write_byte(75, 0xC3);
        memory.write_byte(76, 0x12);
        memory.write_byte(77, 0x34);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::JpImm16(0x3412));

        memory.write_byte(78, 0xE9);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::JpHl);

        memory.write_byte(79, 0xC4);
        memory.write_byte(80, 0x12);
        memory.write_byte(81, 0x34);
        assert_eq!(
            cpu.fetch_instruction(&memory),
            Instruction::CallCondImm16(Cond::NotZero, 0x3412)
        );

        memory.write_byte(82, 0xCD);
        memory.write_byte(83, 0x12);
        memory.write_byte(84, 0x34);
        assert_eq!(
            cpu.fetch_instruction(&memory),
            Instruction::CallImm16(0x3412)
        );

        memory.write_byte(85, 0xC7);
        assert_eq!(
            cpu.fetch_instruction(&memory),
            Instruction::RstTgt3(TGT3::Zero)
        );

        memory.write_byte(86, 0xC1);
        assert_eq!(
            cpu.fetch_instruction(&memory),
            Instruction::PopR16Stk(R16STK::BC)
        );

        memory.write_byte(87, 0xC5);
        assert_eq!(
            cpu.fetch_instruction(&memory),
            Instruction::PushR16Stk(R16STK::BC)
        );

        memory.write_byte(88, 0xCB);
        memory.write_byte(89, 0x00);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::RlcR8(R8::B));

        memory.write_byte(90, 0xCB);
        memory.write_byte(91, 0x06);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::RlcMemHl);

        memory.write_byte(92, 0xCB);
        memory.write_byte(93, 0x08);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::RrcR8(R8::B));

        memory.write_byte(94, 0xCB);
        memory.write_byte(95, 0x0E);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::RrcMemHl);

        memory.write_byte(96, 0xCB);
        memory.write_byte(97, 0x10);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::RlR8(R8::B));

        memory.write_byte(98, 0xCB);
        memory.write_byte(99, 0x16);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::RlMemHl);

        memory.write_byte(100, 0xCB);
        memory.write_byte(101, 0x18);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::RrR8(R8::B));

        memory.write_byte(102, 0xCB);
        memory.write_byte(103, 0x1E);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::RrMemHl);

        memory.write_byte(104, 0xCB);
        memory.write_byte(105, 0x20);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::SlaR8(R8::B));

        memory.write_byte(106, 0xCB);
        memory.write_byte(107, 0x26);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::SlaMemHl);

        memory.write_byte(108, 0xCB);
        memory.write_byte(109, 0x28);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::SraR8(R8::B));

        memory.write_byte(110, 0xCB);
        memory.write_byte(111, 0x2E);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::SraMemHl);

        memory.write_byte(112, 0xCB);
        memory.write_byte(113, 0x30);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::SwapR8(R8::B));

        memory.write_byte(114, 0xCB);
        memory.write_byte(115, 0x36);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::SwapMemHl);

        memory.write_byte(116, 0xCB);
        memory.write_byte(117, 0x38);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::SrlR8(R8::B));

        memory.write_byte(118, 0xCB);
        memory.write_byte(119, 0x3E);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::SrlMemHl);

        memory.write_byte(120, 0xCB);
        memory.write_byte(121, 0x40);
        assert_eq!(
            cpu.fetch_instruction(&memory),
            Instruction::BitB3R8(B3::Zero, R8::B)
        );

        memory.write_byte(122, 0xCB);
        memory.write_byte(123, 0x46);
        assert_eq!(
            cpu.fetch_instruction(&memory),
            Instruction::BitB3MemHl(B3::Zero)
        );

        memory.write_byte(124, 0xCB);
        memory.write_byte(125, 0x80);
        assert_eq!(
            cpu.fetch_instruction(&memory),
            Instruction::ResB3R8(B3::Zero, R8::B)
        );

        memory.write_byte(126, 0xCB);
        memory.write_byte(127, 0x86);
        assert_eq!(
            cpu.fetch_instruction(&memory),
            Instruction::ResB3MemHl(B3::Zero)
        );

        memory.write_byte(128, 0xCB);
        memory.write_byte(129, 0xC0);
        assert_eq!(
            cpu.fetch_instruction(&memory),
            Instruction::SetB3R8(B3::Zero, R8::B)
        );

        memory.write_byte(130, 0xCB);
        memory.write_byte(131, 0xC6);
        assert_eq!(
            cpu.fetch_instruction(&memory),
            Instruction::SetB3MemHl(B3::Zero)
        );

        memory.write_byte(132, 0xE2);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::LdhMemCA);

        memory.write_byte(133, 0xE0);
        memory.write_byte(134, 0x12);
        assert_eq!(
            cpu.fetch_instruction(&memory),
            Instruction::LdhMemImm8A(0x12)
        );

        memory.write_byte(135, 0xEA);
        memory.write_byte(136, 0x34);
        memory.write_byte(137, 0x12);
        assert_eq!(
            cpu.fetch_instruction(&memory),
            Instruction::LdMemImm16A(0x1234)
        );

        memory.write_byte(138, 0xF2);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::LdAMemC);

        memory.write_byte(139, 0xF0);
        memory.write_byte(140, 0x12);
        assert_eq!(
            cpu.fetch_instruction(&memory),
            Instruction::LdhAMemImm8(0x12)
        );

        memory.write_byte(141, 0xFA);
        memory.write_byte(142, 0x34);
        memory.write_byte(143, 0x12);
        assert_eq!(
            cpu.fetch_instruction(&memory),
            Instruction::LdAMemImm16(0x1234)
        );

        memory.write_byte(144, 0xE8);
        memory.write_byte(145, 0x12);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::AddSpImm8(0x12));

        memory.write_byte(146, 0xF8);
        memory.write_byte(147, 0x12);
        assert_eq!(
            cpu.fetch_instruction(&memory),
            Instruction::LdHlSpImm8(0x12)
        );

        memory.write_byte(148, 0xF9);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::LdSpHl);

        memory.write_byte(149, 0xF3);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::Di);

        memory.write_byte(150, 0xFB);
        assert_eq!(cpu.fetch_instruction(&memory), Instruction::Ei);
    }

    #[test]
    fn test_map_prefixed_instruction() {
        assert_eq!(map_prefixed_instruction(0x00), Instruction::RlcR8(R8::B));
        assert_eq!(map_prefixed_instruction(0x06), Instruction::RlcMemHl);
        assert_eq!(map_prefixed_instruction(0x08), Instruction::RrcR8(R8::B));
        assert_eq!(map_prefixed_instruction(0x0E), Instruction::RrcMemHl);
        assert_eq!(map_prefixed_instruction(0x10), Instruction::RlR8(R8::B));
        assert_eq!(map_prefixed_instruction(0x16), Instruction::RlMemHl);
        assert_eq!(map_prefixed_instruction(0x18), Instruction::RrR8(R8::B));
        assert_eq!(map_prefixed_instruction(0x1E), Instruction::RrMemHl);
        assert_eq!(map_prefixed_instruction(0x20), Instruction::SlaR8(R8::B));
        assert_eq!(map_prefixed_instruction(0x26), Instruction::SlaMemHl);
        assert_eq!(map_prefixed_instruction(0x28), Instruction::SraR8(R8::B));
        assert_eq!(map_prefixed_instruction(0x2E), Instruction::SraMemHl);
        assert_eq!(map_prefixed_instruction(0x30), Instruction::SwapR8(R8::B));
        assert_eq!(map_prefixed_instruction(0x36), Instruction::SwapMemHl);
        assert_eq!(map_prefixed_instruction(0x38), Instruction::SrlR8(R8::B));
        assert_eq!(map_prefixed_instruction(0x3E), Instruction::SrlMemHl);
        assert_eq!(
            map_prefixed_instruction(0x40),
            Instruction::BitB3R8(B3::Zero, R8::B)
        );
        assert_eq!(
            map_prefixed_instruction(0x46),
            Instruction::BitB3MemHl(B3::Zero)
        );
        assert_eq!(
            map_prefixed_instruction(0x80),
            Instruction::ResB3R8(B3::Zero, R8::B)
        );
        assert_eq!(
            map_prefixed_instruction(0x86),
            Instruction::ResB3MemHl(B3::Zero)
        );
        assert_eq!(
            map_prefixed_instruction(0xC0),
            Instruction::SetB3R8(B3::Zero, R8::B)
        );
        assert_eq!(
            map_prefixed_instruction(0xC6),
            Instruction::SetB3MemHl(B3::Zero)
        );
    }
}
