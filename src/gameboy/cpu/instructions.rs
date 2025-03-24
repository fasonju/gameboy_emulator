use crate::gameboy::Memory;

use super::{instruction_variables::{B3, COND, R16, R16MEM, R16STK, R8, TGT3}, registers::{Flag, Register16, Register8}, Cpu};



#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Instruction {
    // Block 0
    Nop,
    LdR16Imm16(R16, u16),
    LdR16MemA(R16MEM),
    LdAR16Mem(R16MEM),
    LdMemImm16SP(u16),

    IncR16(R16),
    DecR16(R16),
    AddHlR16(R16),

    IncR8(R8),
    DecR8(R8),
    LdR8Imm8(R8, u8),
    
    Rlca,
    Rrca,
    Rla,
    Rra,
    Daa,
    Cpl,
    Scf,
    Ccf,

    JrImm8(u8),
    JrCondImm8(COND, u8),

    Stop,

    // Block 1
    LdR8R8(R8, R8),
    Halt,

    // Block 2
    AddAR8(R8),
    AdcAR8(R8),
    SubAR8(R8),
    SbcAR8(R8),
    AndAR8(R8),
    XorAR8(R8),
    OrAR8(R8),
    CpAR8(R8),


    // Block 3
    AddAImm8(u8),
    AdcAImm8(u8),
    SubAImm8(u8),
    SbcAImm8(u8),
    AndAImm8(u8),
    XorAImm8(u8),
    OrAImm8(u8),
    CpAImm8(u8),

    RetCond(COND),
    Ret,
    Reti,
    JpCondImm16(COND, u16),
    JpImm16(u16),
    JpHl,
    CallCondImm16(COND, u16),
    CallImm16(u16),
    RstTgt3(TGT3),

    PopR16Stk(R16STK),
    PushR16Stk(R16STK),

    LdMemCA,
    LdhMemImm8A(u8),
    LdMemImm16A(u16),
    LdAMemC,
    LdhAMemImm8(u8),
    LdAMemImm16(u16),

    AddSpImm8(u8),
    LdHlSpImm8(u8),
    LdSpHl,

    Di,
    Ei,

    // Prefix CB
    RlcR8(R8),
    RrcR8(R8),
    RlR8(R8),
    RrR8(R8),
    SlaR8(R8),
    SraR8(R8),
    SwapR8(R8),
    SrlR8(R8),

    BitB3R8(B3, R8),
    ResB3R8(B3, R8),
    SetB3R8(B3, R8),
}

impl Instruction {
    /// Execute the instruction
    /// 
    /// Returns the number of cycles the instruction took
    pub fn execute(self, cpu: &mut Cpu, memory: &mut Memory) -> u8 {
        match self {
            Instruction::Nop => {
                1
            },
            Instruction::LdR16Imm16(register, value) =>  {
                cpu.registers.write_16(Register16::from(register), value);

                3
            },
            Instruction::LdR16MemA(register) => {
                let value = cpu.registers.read_8(Register8::A);
                let address = cpu.registers.read_16(Register16::from(register));
                memory.write_byte(address, value);

                2
            },
            Instruction::LdAR16Mem(register) => {
                let address = cpu.registers.read_16(Register16::from(register));
                let value = memory.read_byte(address);
                cpu.registers.write_8(Register8::A, value);

                2
            },
            Instruction::LdMemImm16SP(adress) => {
                let value = cpu.registers.read_16(Register16::SP);
                memory.write_word(adress, value);

                5
            },
            Instruction::IncR16(register) => {
                let reg = Register16::from(register);
                let value = cpu.registers.read_16(reg);
                cpu.registers.write_16(Register16::from(reg), value.wrapping_add(1));

                2
            },
            Instruction::DecR16(register) => {
                let reg = Register16::from(register);
                let value = cpu.registers.read_16(reg);
                cpu.registers.write_16(Register16::from(reg), value.wrapping_sub(1));

                2
            },
            Instruction::AddHlR16(register) => {
                let value = cpu.registers.read_16(Register16::from(register));
                let hl = cpu.registers.read_16(Register16::HL);
                let (result, overflow) = hl.overflowing_add(value);
                cpu.registers.write_16(Register16::HL, result);
                cpu.registers.write_flag(Flag::N, 0);
                cpu.registers.write_flag(Flag::H, if check_half_carry_add_u16(hl, value) { 1 } else { 0 });
                cpu.registers.write_flag(Flag::C, if overflow { 1 } else { 0 });

                2
            },
            Instruction::IncR8(register) => {
                match register {
                    R8::MemHl => {
                        let address = cpu.registers.read_16(Register16::HL);
                        let value = memory.read_byte(address);
                        let result = value.wrapping_add(1);

                        memory.write_byte(address, result);
                        cpu.registers.write_flag(Flag::Z, if result == 0 { 1 } else { 0 });
                        cpu.registers.write_flag(Flag::N, 0);
                        cpu.registers.write_flag(Flag::H, if check_half_carry_add_u8(value, 1) { 1 } else { 0 });
                    },
                    _ => {
                        let reg = Register8::from(register);
                        let value = cpu.registers.read_8(reg);
                        let result = value.wrapping_add(1);

                        cpu.registers.write_8(reg, result);
                        cpu.registers.write_flag(Flag::Z, if result == 0 { 1 } else { 0 });
                        cpu.registers.write_flag(Flag::N, 0);
                        cpu.registers.write_flag(Flag::H, if check_half_carry_add_u8(value, 1) { 1 } else { 0 });
                    }
                }

                1
            },
            Instruction::DecR8(register8) => todo!(),
            Instruction::LdR8Imm8(register8, _) => todo!(),
            Instruction::Rlca => todo!(),
            Instruction::Rrca => todo!(),
            Instruction::Rla => todo!(),
            Instruction::Rra => todo!(),
            Instruction::Daa => todo!(),
            Instruction::Cpl => todo!(),
            Instruction::Scf => todo!(),
            Instruction::Ccf => todo!(),
            Instruction::JrImm8(_) => todo!(),
            Instruction::JrCondImm8(condition, _) => todo!(),
            Instruction::Stop => todo!(),
            Instruction::LdR8R8(register8, register9) => todo!(),
            Instruction::Halt => todo!(),
            Instruction::AddAR8(register8) => todo!(),
            Instruction::AdcAR8(register8) => todo!(),
            Instruction::SubAR8(register8) => todo!(),
            Instruction::SbcAR8(register8) => todo!(),
            Instruction::AndAR8(register8) => todo!(),
            Instruction::XorAR8(register8) => todo!(),
            Instruction::OrAR8(register8) => todo!(),
            Instruction::CpAR8(register8) => todo!(),
            Instruction::AddAImm8(_) => todo!(),
            Instruction::AdcAImm8(_) => todo!(),
            Instruction::SubAImm8(_) => todo!(),
            Instruction::SbcAImm8(_) => todo!(),
            Instruction::AndAImm8(_) => todo!(),
            Instruction::XorAImm8(_) => todo!(),
            Instruction::OrAImm8(_) => todo!(),
            Instruction::CpAImm8(_) => todo!(),
            Instruction::RetCond(condition) => todo!(),
            Instruction::Ret => todo!(),
            Instruction::Reti => todo!(),
            Instruction::JpCondImm16(condition, _) => todo!(),
            Instruction::JpImm16(_) => todo!(),
            Instruction::JpHl => todo!(),
            Instruction::CallCondImm16(condition, _) => todo!(),
            Instruction::CallImm16(_) => todo!(),
            Instruction::RstTgt3(b3) => todo!(),
            Instruction::PopR16Stk(register16_stk) => todo!(),
            Instruction::PushR16Stk(register16_stk) => todo!(),
            Instruction::LdMemCA => todo!(),
            Instruction::LdhMemImm8A(_) => todo!(),
            Instruction::LdMemImm16A(_) => todo!(),
            Instruction::LdAMemC => todo!(),
            Instruction::LdhAMemImm8(_) => todo!(),
            Instruction::LdAMemImm16(_) => todo!(),
            Instruction::AddSpImm8(_) => todo!(),
            Instruction::LdHlSpImm8(_) => todo!(),
            Instruction::LdSpHl => todo!(),
            Instruction::Di => todo!(),
            Instruction::Ei => todo!(),
            Instruction::RlcR8(register8) => todo!(),
            Instruction::RrcR8(register8) => todo!(),
            Instruction::RlR8(register8) => todo!(),
            Instruction::RrR8(register8) => todo!(),
            Instruction::SlaR8(register8) => todo!(),
            Instruction::SraR8(register8) => todo!(),
            Instruction::SwapR8(register8) => todo!(),
            Instruction::SrlR8(register8) => todo!(),
            Instruction::BitB3R8(b3, register8) => todo!(),
            Instruction::ResB3R8(b3, register8) => todo!(),
            Instruction::SetB3R8(b3, register8) => todo!(),
        }
    }
}

fn check_half_carry_add_u8(left: u8, right: u8) -> bool {
    (left & 0xF) + (right & 0xF) & 0x10 != 0x0
}

fn check_half_carry_add_u16(left: u16, right: u16) -> bool {
    (left & 0xFFF) + (right & 0xFFF) & 0x1000 != 0x0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_half_carry_add_u8() {
        assert_eq!(check_half_carry_add_u8(0x0F, 0x01), true);
        assert_eq!(check_half_carry_add_u8(0x0F, 0x0F), true);
        assert_eq!(check_half_carry_add_u8(0x0F, 0x00), false);
        assert_eq!(check_half_carry_add_u8(0x00, 0x00), false);
    }

    #[test]
    fn test_check_half_carry_add_u16() {
        assert_eq!(check_half_carry_add_u16(0x0FFF, 0x0001), true);
        assert_eq!(check_half_carry_add_u16(0x0FFF, 0x0FFF), true);
        assert_eq!(check_half_carry_add_u16(0x0FFF, 0x0000), false);
        assert_eq!(check_half_carry_add_u16(0x0000, 0x0000), false);
    }

    #[test]
    fn test_nop() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::Nop;

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
    }

    #[test]
    fn test_ld_r16_imm16() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::LdR16Imm16(R16::BC, 0xABCD);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 3);
        assert_eq!(cpu.registers.read_16(Register16::BC), 0xABCD);
    }

    #[test]
    fn test_ld_r16mem_a() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::LdR16MemA(R16MEM::BC);
        
        cpu.registers.write_16(Register16::BC, 0x1234);
        cpu.registers.write_8(Register8::A, 0xAB);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(memory.read_byte(0x1234), 0xAB);
    }

    #[test]
    fn test_ld_a_r16mem() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::LdAR16Mem(R16MEM::BC);
        
        cpu.registers.write_16(Register16::BC, 0x1234);
        memory.write_byte(0x1234, 0xAB);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0xAB);
    }

    #[test]
    fn test_ld_memimm16_sp() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::LdMemImm16SP(0x1234);
        cpu.registers.write_16(Register16::SP, 0xABCD);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 5);
        assert_eq!(memory.read_word(0x1234), 0xABCD);
    }

    #[test]
    fn test_inc_r16() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::IncR16(R16::BC);
        cpu.registers.write_16(Register16::BC, 0x1234);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_16(Register16::BC), 0x1235);
    }

    #[test]
    fn test_dec_r16() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::DecR16(R16::BC);
        cpu.registers.write_16(Register16::BC, 0x1234);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_16(Register16::BC), 0x1233);
    }

    #[test]
    fn test_add_hl_r16() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::AddHlR16(R16::BC);
        cpu.registers.write_16(Register16::HL, 0x1234);
        cpu.registers.write_16(Register16::BC, 0x5678);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
        assert_eq!(cpu.registers.read_16(Register16::HL), 0x68AC);
    }

    #[test]
    fn test_add_hl_r16_flags() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::AddHlR16(R16::BC);
        cpu.registers.write_16(Register16::HL, 0xFFFF);
        cpu.registers.write_16(Register16::BC, 0xFFFF);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(cpu.registers.read_flag(Flag::C), 1);
        assert_eq!(cpu.registers.read_16(Register16::HL), 0xFFFE);
    }

    #[test]
    fn test_inc_r8() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::IncR8(R8::B);
        cpu.registers.write_8(Register8::B, 0x0);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_8(Register8::B), 0x1);
    }

    #[test]
    fn test_inc_r8_half_overflow() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::IncR8(R8::B);
        cpu.registers.write_8(Register8::B, 0xF);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(cpu.registers.read_8(Register8::B), 0x10);
    }

    #[test]
    fn test_inc_r8_zero() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::IncR8(R8::B);
        cpu.registers.write_8(Register8::B, 0xFF);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 1);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(cpu.registers.read_8(Register8::B), 0x00);
    }

    #[test]
    fn test_inc_r8_memhl() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::IncR8(R8::MemHl);
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0x0);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(memory.read_byte(0x1234), 0x1);
    }
}

