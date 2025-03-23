use super::{registers::{Flag, Register16, Register8}, Cpu};


// A generic instruction for 8-bit registers
pub enum Instruction {
    // Ld instruction
    LdR8FromR8(Register8, Register8),
    LdR8FromN8(Register8, u8),
    LdR16FromN16(Register16, u16),
    LdMemHlFromR8(Register8),
    LdMemHlFromN8(u8),
    LdR8FromMemHl(Register8),
    LdMemR16FromA(Register16),
    LdMemN16FromA(u16),
    LdhMemN16FromA(u16),
    LdhMemCFromA,
    LdAFromMemR16(Register16),
    LdAFromMemN16(u16),
    LdhAFromMemN16(u16),
    LdhAFromMemC,
    LdMemHliFromA,
    LdMemHldFromA,
    LdAFromMemHli,
    LdAFromMemHld,

    // Arithmetic instructions
    AdcR8(Register8),
    AdcMemHl,
    AdcN8(u8),
    AddR8(Register8),
    AddMemHl,
    AddN8(u8),
    CpR8,
    CpMemHl,
    CpN8(u8),
    DecR8(Register8),
    DecMemHl,
    IncR8(Register8),
    IncMemHl,
    SbcR8(Register8),
    SbcMemHl,
    SbcN8(u8),
    SubR8(Register8),
    SubMemHl,
    SubN8(u8),

    // Bitwise logic instructions
    AndR8(Register8),
    AndMemHl,
    AndN8(u8),
    Cpl,
    OrR8(Register8),
    OrMemHl,
    OrN8(u8),
    XorR8(Register8),
    XorMemHl,
    XorN8(u8),

    // Bitwise flag instructions
    BitU3R8(Register8),
    BitU3MemHl,
    ResU3R8(Register8),
    ResU3MemHl,
    SetU3R8(Register8),
    SetU3MemHl,

    // Rotate and shift instructions
    RlR8(Register8),
    RlMemHl,
    Rla,
    RlcR8(Register8),
    RlcMemHl,
    Rlca,
    RrR8(Register8),
    RrMemHl,
    Rra,
    RrcR8(Register8),
    RrcMemHl,
    Rrca,
    SlaR8(Register8),
    SlaMemHl,
    SraR8(Register8),
    SraMemHl,
    SrlR8(Register8),
    SrlMemHl,
    SwapR8(Register8),
    SwapMemHl,

    // Jump and subroutine instructions
    CallN16(u16),
    CallCcN16(Flag, u16),
    JpHl,
    JpN16(u16),
    JpCcN16(Flag, u16),
    JrN16(u16),
    JrCcN16(Flag, u16),
    RetCc(Flag),
    Ret,
    Reti,
    RstN8(u8),
    CallVec(u16),

    // Stack instructions
    AddHlSp,
    AddSpE8(i8),
    DecSp,
    IncSp,
    LdSpFromN16(u16),
    LdMemN16FromSp(u16),
    LdHlFromSpPlusE8(i8),
    LdSpFromHl,
    PopAf,
    PopR16(Register16),
    PushAf,
    PushR16(Register16),

    // Interrupt related instructions
    Di,
    Ei,
    Halt,

    // Misc instructions
    Daa,
    Nop,
    Stop,
}

impl Instruction {
    /// Execute the instruction
    /// Returns the number of cycles the instruction took
    pub fn execute(&self, cpu: &mut Cpu) -> u8 {
        match self {
            Instruction::LdR8FromR8(target, source) => {
                let value = cpu.registers.get_register_8(*source);
                load_value_to_register8(*target, value, cpu);
                1
            },
            Instruction::LdR8FromN8(target, value) => { 
                load_value_to_register8(*target, *value, cpu);
                2
            },
            Instruction::LdR16FromN16(target, value) => {
                load_value_to_register16(*target, *value, cpu);
                3
            },
            Instruction::LdMemHlFromR8(register8) => {
                let adress = cpu.registers.get_register_16(Register16::HL);
                load_register8_to_memory(*register8, adress, cpu);
                2
            },
            Instruction::LdMemHlFromN8(n) => todo!(),
            Instruction::LdR8FromMemHl(register8) => todo!(),
            Instruction::LdMemR16FromA(register16) => todo!(),
            Instruction::LdMemN16FromA(n) => todo!(),
            Instruction::LdhMemN16FromA(n) => todo!(),
            Instruction::LdhMemCFromA => todo!(),
            Instruction::LdAFromMemR16(register16) => todo!(),
            Instruction::LdAFromMemN16(n) => todo!(),
            Instruction::LdhAFromMemN16(n) => todo!(),
            Instruction::LdhAFromMemC => todo!(),
            Instruction::LdMemHliFromA => todo!(),
            Instruction::LdMemHldFromA => todo!(),
            Instruction::LdAFromMemHli => todo!(),
            Instruction::LdAFromMemHld => todo!(),
            Instruction::AdcR8(register8) => todo!(),
            Instruction::AdcMemHl => todo!(),
            Instruction::AdcN8(n) => todo!(),
            Instruction::AddR8(register8) => todo!(),
            Instruction::AddMemHl => todo!(),
            Instruction::AddN8(n) => todo!(),
            Instruction::CpR8 => todo!(),
            Instruction::CpMemHl => todo!(),
            Instruction::CpN8(_) => todo!(),
            Instruction::DecR8(register8) => todo!(),
            Instruction::DecMemHl => todo!(),
            Instruction::IncR8(register8) => todo!(),
            Instruction::IncMemHl => todo!(),
            Instruction::SbcR8(register8) => todo!(),
            Instruction::SbcMemHl => todo!(),
            Instruction::SbcN8(_) => todo!(),
            Instruction::SubR8(register8) => todo!(),
            Instruction::SubMemHl => todo!(),
            Instruction::SubN8(_) => todo!(),
            Instruction::AndR8(register8) => todo!(),
            Instruction::AndMemHl => todo!(),
            Instruction::AndN8(_) => todo!(),
            Instruction::Cpl => todo!(),
            Instruction::OrR8(register8) => todo!(),
            Instruction::OrMemHl => todo!(),
            Instruction::OrN8(_) => todo!(),
            Instruction::XorR8(register8) => todo!(),
            Instruction::XorMemHl => todo!(),
            Instruction::XorN8(_) => todo!(),
            Instruction::BitU3R8(register8) => todo!(),
            Instruction::BitU3MemHl => todo!(),
            Instruction::ResU3R8(register8) => todo!(),
            Instruction::ResU3MemHl => todo!(),
            Instruction::SetU3R8(register8) => todo!(),
            Instruction::SetU3MemHl => todo!(),
            Instruction::RlR8(register8) => todo!(),
            Instruction::RlMemHl => todo!(),
            Instruction::Rla => todo!(),
            Instruction::RlcR8(register8) => todo!(),
            Instruction::RlcMemHl => todo!(),
            Instruction::Rlca => todo!(),
            Instruction::RrR8(register8) => todo!(),
            Instruction::RrMemHl => todo!(),
            Instruction::Rra => todo!(),
            Instruction::RrcR8(register8) => todo!(),
            Instruction::RrcMemHl => todo!(),
            Instruction::Rrca => todo!(),
            Instruction::SlaR8(register8) => todo!(),
            Instruction::SlaMemHl => todo!(),
            Instruction::SraR8(register8) => todo!(),
            Instruction::SraMemHl => todo!(),
            Instruction::SrlR8(register8) => todo!(),
            Instruction::SrlMemHl => todo!(),
            Instruction::SwapR8(register8) => todo!(),
            Instruction::SwapMemHl => todo!(),
            Instruction::CallN16(_) => todo!(),
            Instruction::CallCcN16(flag, _) => todo!(),
            Instruction::JpHl => todo!(),
            Instruction::JpN16(_) => todo!(),
            Instruction::JpCcN16(flag, _) => todo!(),
            Instruction::JrN16(_) => todo!(),
            Instruction::JrCcN16(flag, _) => todo!(),
            Instruction::RetCc(flag) => todo!(),
            Instruction::Ret => todo!(),
            Instruction::Reti => todo!(),
            Instruction::RstN8(_) => todo!(),
            Instruction::CallVec(_) => todo!(),
            Instruction::AddHlSp => todo!(),
            Instruction::AddSpE8(_) => todo!(),
            Instruction::DecSp => todo!(),
            Instruction::IncSp => todo!(),
            Instruction::LdSpFromN16(_) => todo!(),
            Instruction::LdMemN16FromSp(_) => todo!(),
            Instruction::LdHlFromSpPlusE8(_) => todo!(),
            Instruction::LdSpFromHl => todo!(),
            Instruction::PopAf => todo!(),
            Instruction::PopR16(register16) => todo!(),
            Instruction::PushAf => todo!(),
            Instruction::PushR16(register16) => todo!(),
            Instruction::Di => todo!(),
            Instruction::Ei => todo!(),
            Instruction::Halt => todo!(),
            Instruction::Daa => todo!(),
            Instruction::Nop => todo!(),
            Instruction::Stop => todo!(),
        }
    }
}
fn load_value_to_register8(target: Register8, value: u8, cpu: &mut Cpu) {
    cpu.registers.set_register_8(target, value);
}

fn load_value_to_register16(target: Register16, value: u16, cpu: &mut Cpu) {
    cpu.registers.set_register_16(target, value);
}

fn load_register8_to_memory(source: Register8, adress: u16, cpu: &mut Cpu) {
    let value = cpu.registers.get_register_8(source);
    cpu.memory.write_byte(adress, value);
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::gameboy::{registers::{Register16, Register8}, Memory};

    #[test]
    fn test_load_value_to_register8() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        cpu.registers.set_register_8(Register8::A, 0x12);

        load_value_to_register8(Register8::A, 0x34, &mut cpu);

        assert_eq!(cpu.registers.get_register_8(Register8::A), 0x34);
    }

    #[test]
    fn test_load_value_to_register16() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        cpu.registers.set_register_16(Register16::BC, 0x1234);

        load_value_to_register16(Register16::BC, 0x5678, &mut cpu);

        assert_eq!(cpu.registers.get_register_16(Register16::BC), 0x5678);
    }

    #[test]
    fn test_load_register8_to_memory() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        cpu.registers.set_register_8(Register8::A, 0x12);
        cpu.registers.set_register_16(Register16::HL, 0x1234);

        load_register8_to_memory(Register8::A, 0x1234, &mut cpu);

        assert_eq!(memory.read_byte(0x1234), 0x12);
    }

    #[test]
    fn test_ld_r8_from_r8() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        cpu.registers.set_register_8(Register8::A, 0x12);
        cpu.registers.set_register_8(Register8::B, 0x34);

        let instruction = Instruction::LdR8FromR8(Register8::A, Register8::B);
        instruction.execute(&mut cpu);

        assert_eq!(cpu.registers.get_register_8(Register8::A), 0x34);
    }

    #[test]
    fn test_ld_r8_from_n8() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        cpu.registers.set_register_8(Register8::A, 0x12);

        let instruction = Instruction::LdR8FromN8(Register8::A, 0x34);
        instruction.execute(&mut cpu);

        assert_eq!(cpu.registers.get_register_8(Register8::A), 0x34);
    }

    #[test]
    fn test_ld_r16_from_n16() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        cpu.registers.set_register_16(Register16::BC, 0x1234);

        let instruction = Instruction::LdR16FromN16(Register16::BC, 0x5678);
        instruction.execute(&mut cpu);

        assert_eq!(cpu.registers.get_register_16(Register16::BC), 0x5678);
    }

    #[test]
    fn test_ld_mem_hl_from_r8() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        cpu.registers.set_register_8(Register8::A, 0x12);
        cpu.registers.set_register_16(Register16::HL, 0x1234);

        let instruction = Instruction::LdMemHlFromR8(Register8::A);
        instruction.execute(&mut cpu);

        assert_eq!(memory.read_byte(0x1234), 0x12);
    }
}