
use super::{instruction_variables::{Condition, R16, R16MEM, R16STK, R8, B3}, Cpu};

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Instruction {
    // Block 0
    Nop,
    LdR16Imm16(R16, u16),
    LdR16MemA(R16MEM),
    LdAR16Mem(R16MEM),
    LdMemImm16MemSP(u16),

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
    JrCondImm8(Condition, u8),

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

    RetCond(Condition),
    Ret,
    Reti,
    JpCondImm16(Condition, u16),
    JpImm16(u16),
    JpHl,
    CallCondImm16(Condition, u16),
    CallImm16(u16),
    RstTgt3(B3),

    PopR16Stk(R16STK),
    PushR16Stk(R16STK),

    // Prefix?

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
    pub fn execute(self, cpu: &mut Cpu) -> u8 {
        match self {
            Instruction::Nop => todo!(),
            Instruction::LdR16Imm16(register16, _) => todo!(),
            Instruction::LdR16MemA(register16_mem) => todo!(),
            Instruction::LdAR16Mem(register16_mem) => todo!(),
            Instruction::LdMemImm16MemSP(_) => todo!(),
            Instruction::IncR16(register16) => todo!(),
            Instruction::DecR16(register16) => todo!(),
            Instruction::AddHlR16(register16) => todo!(),
            Instruction::IncR8(register8) => todo!(),
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
