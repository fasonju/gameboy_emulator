use super::registers::{Register16, Register8};


// A generic instruction for 8-bit registers
pub enum Instruction {
    Nop,

    // Ld instruction
    LdR8fromR8(Register8, Register8),
    LdR8fromN8(Register8, u8),
    LdR16fromN16(Register16, u16),
    LdMemHlfromR8(Register8),
    LdMemHlfromN8(u8),
    LdR8fromMemHl(Register8),
    LdMemR16fromA(Register16),
    LdMemN16fromA(u16),
    LdhMemN16fromA(u16),
    LdhMemCfromA,
    LdAfromMemR16(Register16),
    LdAfromMemN16(u16),
    LdhAfromMemN16(u16),
    LdhAfromMemC,
    LdMemHlIfromA,
    LdMemHldfromA,
    LdAfromMemHli,
    LdAfromMemHld,

    // Arithmetic instructions
    AdcR8(Register8),
    AdcMemHL,
    AdcN8(u8),
    AddR8(Register8),
    AddMemHL,
    AddN8(u8),
    CpR8,
    CpMemHL,
    CpN8(u8),
    DecR8(Register8),
    DecMemHL,
    IncR8(Register8),
    IncMemHL,
    SbcR8(Register8),
    SbcMemHL,
    SbcN8(u8),
    SubR8(Register8),
    SubMemHL,
    SubN8(u8),
}