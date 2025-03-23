
use crate::utils::{carry_u8_add, half_carry_u8_add};

use super::{registers::{Flag, Register16, Register8}, Cpu};


// A generic instruction for 8-bit registers
pub enum Instruction {
    // Ld instruction

    /// LD r8, r8
    LdR8FromR8(Register8, Register8),
    /// LD r8, n8
    LdR8FromN8(Register8, u8),
    /// LD r16, n16
    LdR16FromN16(Register16, u16),
    /// LD (HL), r8
    LdMemHlFromR8(Register8),
    /// LD (HL), n8
    LdMemHlFromN8(u8),
    /// LD r8, (HL)
    LdR8FromMemHl(Register8),
    /// LD (r16), A
    LdMemR16FromA(Register16),
    /// LD (n16), A
    LdMemN16FromA(u16),
    /// LDH (n16), A
    LdhMemN16FromA(u16),
    /// LDH (C), A
    LdhMemCFromA,
    /// LD A, (r16)
    LdAFromMemR16(Register16),
    /// LD A, (n16)
    LdAFromMemN16(u16),
    /// LDH A, (n16)
    LdhAFromMemN16(u16),
    /// LDH A, (C)
    LdhAFromMemC,
    /// LD (HL+), A
    LdMemHliFromA,
    /// LD (HL-), A
    LdMemHldFromA,
    /// LD A, (HL+)
    LdAFromMemHli,
    /// LD A, (HL-)
    LdAFromMemHld,

    // Arithmetic instructions

    /// ADC A, r8
    AdcR8(Register8),
    /// ADC A, (HL)
    AdcMemHl,
    /// ADC A, n8
    AdcN8(u8),
    /// ADD A, r8
    AddR8(Register8),
    /// ADD A, (HL)
    AddMemHl,
    /// ADD A, n8
    AddN8(u8),
    /// CP A, r8
    CpR8,
    /// CP A, (HL)
    CpMemHl,
    /// CP A, n8
    CpN8(u8),
    /// DEC r8
    DecR8(Register8),
    /// DEC (HL)
    DecMemHl,
    /// INC r8
    IncR8(Register8),
    /// INC (HL)
    IncMemHl,
    /// SBC A, r8
    SbcR8(Register8),
    /// SBC A, (HL)
    SbcMemHl,
    /// SBC A, n8
    SbcN8(u8),
    /// SUB A, r8
    SubR8(Register8),
    /// SUB A, (HL)
    SubMemHl,
    /// SUB A, n8
    SubN8(u8),

    // Bitwise logic instructions

    /// AND A, r8
    AndR8(Register8),
    /// AND A, (HL)
    AndMemHl,
    /// AND A, n8
    AndN8(u8),
    /// CPL
    Cpl,
    /// OR A, r8
    OrR8(Register8),
    /// OR A, (HL)
    OrMemHl,
    /// OR A, n8
    OrN8(u8),
    /// XOR A, r8
    XorR8(Register8),
    /// XOR A, (HL)
    XorMemHl,
    /// XOR A, n8
    XorN8(u8),

    // Bitwise flag instructions

    /// BIT u3, r8
    BitU3R8(u8, Register8),
    /// BIT u3, (HL)
    BitU3MemHl(u8),
    /// RES u3, r8
    ResU3R8(u8, Register8),
    /// RES u3, (HL)
    ResU3MemHl(u8),
    /// SET u3, r8
    SetU3R8(Register8),
    /// SET u3, (HL)
    SetU3MemHl(u8),

    // Rotate and shift instructions

    /// RL r8
    RlR8(Register8),
    /// RL (HL)
    RlMemHl,
    /// RLA
    Rla,
    /// RLC r8
    RlcR8(Register8),
    /// RLC (HL)
    RlcMemHl,
    /// RLCA
    Rlca,
    /// RR r8
    RrR8(Register8),
    /// RR (HL)
    RrMemHl,
    /// RRA
    Rra,
    /// RRC r8
    RrcR8(Register8),
    /// RRC (HL)
    RrcMemHl,
    /// RRCA
    Rrca,
    /// SLA r8
    SlaR8(Register8),
    /// SLA (HL)
    SlaMemHl,
    /// SRA r8
    SraR8(Register8),
    /// SRA (HL)
    SraMemHl,
    /// SRL r8
    SrlR8(Register8),
    /// SRL (HL)
    SrlMemHl,
    /// SWAP r8
    SwapR8(Register8),
    /// SWAP (HL)
    SwapMemHl,

    // Jump and subroutine instructions

    /// CALL n16
    CallN16(u16),
    /// CALL cc, n16
    CallCcN16(Flag, u16),
    /// JP HL
    JpHl,
    /// JP n16
    JpN16(u16),
    /// JP cc, n16
    JpCcN16(Flag, u16),
    /// JR n16
    JrN16(u16),
    /// JR cc, n16
    JrCcN16(Flag, u16),
    /// RET cc
    RetCc(Flag),
    /// RET
    Ret,
    /// RETI
    Reti,
    /// RST n8
    RstN8(u8),
    /// CALL vec
    CallVec(u16),

    // Stack instructions

    /// ADD HL, SP
    AddHlSp,
    /// ADD SP, e8
    AddSpE8(i8),
    /// DEC SP
    DecSp,
    /// INC SP
    IncSp,
    /// LD SP, n16
    LdSpFromN16(u16),
    /// LD (n16), SP
    LdMemN16FromSp(u16),
    /// LD HL, SP + e8
    LdHlFromSpPlusE8(i8),
    /// LD SP, HL
    LdSpFromHl,
    /// POP AF
    PopAf,
    /// POP r16
    PopR16(Register16),
    /// PUSH AF
    PushAf,
    /// PUSH r16
    PushR16(Register16),

    // Interrupt related instructions

    /// DI
    Di,
    /// EI
    Ei,
    /// HALT
    Halt,

    // Misc instructions

    /// DAA
    Daa,
    /// NOP
    Nop,
    /// STOP
    Stop,
}

impl Instruction {
    /// Execute the instruction
    /// 
    /// Returns the number of cycles the instruction took
    pub fn execute(&self, cpu: &mut Cpu) -> u8 {
        match self {
            // Load instructions

            Instruction::LdR8FromR8(target, source) => {
                load_register8_to_register8(*target, *source, cpu);
                1
            },
            Instruction::LdR8FromN8(register, value) => { 
                cpu.registers.set_register_8(*register, *value);
                2
            },
            Instruction::LdR16FromN16(register, value) => {
                cpu.registers.set_register_16(*register, *value);
                3
            },
            Instruction::LdMemHlFromR8(register) => {
                let adress = cpu.registers.get_register_16(Register16::HL);
                load_register8_to_memory(*register, adress, cpu);
                2
            },
            Instruction::LdMemHlFromN8(n) => {
                let adress = cpu.registers.get_register_16(Register16::HL);
                cpu.memory.write_byte(adress, *n);
                3
            }, 
            Instruction::LdR8FromMemHl(register) => {
                let adress = cpu.registers.get_register_16(Register16::HL);
                load_memory_to_register8(*register, adress, cpu);
                2
            },
            Instruction::LdMemR16FromA(register16) => {
                let adress = cpu.registers.get_register_16(*register16);
                load_register8_to_memory(Register8::A, adress, cpu);
                2
            },
            Instruction::LdMemN16FromA(n) => {
                load_register8_to_memory(Register8::A, *n, cpu);
                4
            },
            Instruction::LdhMemN16FromA(n) => {
                let adress = 0xFF00 + n;
                load_register8_to_memory(Register8::A, adress, cpu);
                4
            },
            Instruction::LdhMemCFromA => {
                let adress = 0xFF00 + cpu.registers.get_register_8(Register8::C) as u16;
                load_register8_to_memory(Register8::A, adress, cpu);
                2
            },
            Instruction::LdAFromMemR16(register) => {
                let adress = cpu.registers.get_register_16(*register);
                load_memory_to_register8(Register8::A, adress, cpu);
                2
            },
            Instruction::LdAFromMemN16(n) => {
                load_memory_to_register8(Register8::A, *n, cpu);
                4
            },
            Instruction::LdhAFromMemN16(n) => {
                let adress = 0xFF00 + n;
                load_memory_to_register8(Register8::A, adress, cpu);
                4
            },
            Instruction::LdhAFromMemC => {
                let adress = 0xFF00 + cpu.registers.get_register_8(Register8::C) as u16;
                load_memory_to_register8(Register8::A, adress, cpu);
                2
            },
            Instruction::LdMemHliFromA => {
                let adress = cpu.registers.get_register_16(Register16::HL);
                load_register8_to_memory(Register8::A, adress, cpu);
                cpu.registers.set_register_16(Register16::HL, adress + 1);
                2
            },
            Instruction::LdMemHldFromA => {
                let adress = cpu.registers.get_register_16(Register16::HL);
                load_register8_to_memory(Register8::A, adress, cpu);
                cpu.registers.set_register_16(Register16::HL, adress - 1);
                2
            },
            Instruction::LdAFromMemHli => {
                let adress = cpu.registers.get_register_16(Register16::HL);
                load_memory_to_register8(Register8::A, adress, cpu);
                cpu.registers.set_register_16(Register16::HL, adress + 1);
                2
            },
            Instruction::LdAFromMemHld => {
                let adress = cpu.registers.get_register_16(Register16::HL);
                load_memory_to_register8(Register8::A, adress, cpu);
                cpu.registers.set_register_16(Register16::HL, adress - 1);
                2
            },

            // Arithmetic instructions
            Instruction::AdcR8(register) => {
                let left = cpu.registers.get_register_8(Register8::A);
                let right = cpu.registers.get_flag(Flag::Carry)
                    .wrapping_add(cpu.registers.get_register_8(*register));

                let result = left.wrapping_add(right);
                
                let z: u8 = if result == 0 { 1 } else { 0 };
                let n: u8 = 0;
                let h: u8 = half_carry_u8_add(left, right);
                let c: u8 = carry_u8_add(left, right);

                cpu.registers.set_flags(Some(z), Some(n), Some(h), Some(c));
                cpu.registers.set_register_8(Register8::A, result);
                1
            },
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
            Instruction::BitU3R8(index, register8) => todo!(),
            Instruction::BitU3MemHl(index) => todo!(),
            Instruction::ResU3R8(index, register8) => todo!(),
            Instruction::ResU3MemHl(index) => todo!(),
            Instruction::SetU3R8(register8) => todo!(),
            Instruction::SetU3MemHl(index) => todo!(),
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

fn load_register8_to_register8(target: Register8, source: Register8, cpu: &mut Cpu) {
    let value = cpu.registers.get_register_8(source);
    cpu.registers.set_register_8(target, value);
}

fn load_register8_to_memory(source: Register8, adress: u16, cpu: &mut Cpu) {
    let value = cpu.registers.get_register_8(source);
    cpu.memory.write_byte(adress, value);
}

fn load_memory_to_register8(target: Register8, adress: u16, cpu: &mut Cpu) {
    let value = cpu.memory.read_byte(adress);
    cpu.registers.set_register_8(target, value);
}

fn load_memory_to_register16(target: Register16, adress: u16, cpu: &mut Cpu) {
    let value = cpu.memory.read_word(adress);
    cpu.registers.set_register_16(target, value);
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::gameboy::{registers::{Register16, Register8}, Memory};

    #[test]
    fn test_load_register8_to_register8() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        cpu.registers.set_register_8(Register8::A, 0x12);
        cpu.registers.set_register_8(Register8::B, 0x34);

        load_register8_to_register8(Register8::A, Register8::B, &mut cpu);

        assert_eq!(cpu.registers.get_register_8(Register8::A), 0x34);
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
    fn test_load_memory_to_register8() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        cpu.memory.write_byte(0x1234, 0x12);
        cpu.registers.set_register_8(Register8::A, 0x34);

        load_memory_to_register8(Register8::A, 0x1234, &mut cpu);

        assert_eq!(cpu.registers.get_register_8(Register8::A), 0x12);
    }

    #[test]
    fn test_load_memory_to_register16() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        cpu.memory.write_word(0x1234, 0x1234);

        load_memory_to_register16(Register16::BC, 0x1234, &mut cpu);

        assert_eq!(cpu.registers.get_register_16(Register16::BC), 0x1234);
    }

    #[test]
    fn test_ld_r8_from_r8() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        cpu.registers.set_register_8(Register8::A, 0x12);
        cpu.registers.set_register_8(Register8::B, 0x34);

        let instruction = Instruction::LdR8FromR8(Register8::A, Register8::B);
        let cycles = instruction.execute(&mut cpu);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.get_register_8(Register8::A), 0x34);
    }

    #[test]
    fn test_ld_r8_from_n8() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        cpu.registers.set_register_8(Register8::A, 0x12);

        let instruction = Instruction::LdR8FromN8(Register8::A, 0x34);
        let cycles = instruction.execute(&mut cpu);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.get_register_8(Register8::A), 0x34);
    }

    #[test]
    fn test_ld_r16_from_n16() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        cpu.registers.set_register_16(Register16::BC, 0x1234);

        let instruction = Instruction::LdR16FromN16(Register16::BC, 0x5678);
        let cycles = instruction.execute(&mut cpu);

        assert_eq!(cycles, 3);
        assert_eq!(cpu.registers.get_register_16(Register16::BC), 0x5678);
    }

    #[test]
    fn test_ld_mem_hl_from_r8() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        cpu.registers.set_register_8(Register8::A, 0x12);
        cpu.registers.set_register_16(Register16::HL, 0x1234);

        let instruction = Instruction::LdMemHlFromR8(Register8::A);
        let cycles = instruction.execute(&mut cpu);

        let cycles = instruction.execute(&mut cpu);
        assert_eq!(memory.read_byte(0x1234), 0x12);
    }

    #[test]
    fn test_ld_mem_hl_from_n8() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        cpu.registers.set_register_16(Register16::HL, 0x1234);

        let instruction = Instruction::LdMemHlFromN8(0x12);
        let cycles = instruction.execute(&mut cpu);

        assert_eq!(cycles, 3);
        assert_eq!(memory.read_byte(0x1234), 0x12);
    }

    #[test]
    fn test_ld_r8_from_mem_hl() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        cpu.memory.write_byte(0x1234, 0x12);
        cpu.registers.set_register_16(Register16::HL, 0x1234);

        let instruction = Instruction::LdR8FromMemHl(Register8::A);
        let cycles = instruction.execute(&mut cpu);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.get_register_8(Register8::A), 0x12);
    }

    #[test]
    fn test_ld_r16_from_a() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        cpu.registers.set_register_8(Register8::A, 0x12);
        cpu.registers.set_register_16(Register16::BC, 0x1234);

        let instruction = Instruction::LdMemR16FromA(Register16::BC);
        let cycles = instruction.execute(&mut cpu);

        assert_eq!(cycles, 2);
        assert_eq!(memory.read_byte(0x1234), 0x12);
    }

    #[test]
    fn test_ld_mem_n16_from_a() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        cpu.registers.set_register_8(Register8::A, 0x12);

        let instruction = Instruction::LdMemN16FromA(0x1234);
        let cycles = instruction.execute(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(memory.read_byte(0x1234), 0x12);
    }

    #[test]
    fn test_ldh_n16_from_a() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        cpu.registers.set_register_8(Register8::A, 0x12);

        let instruction = Instruction::LdhMemN16FromA(0x12);
        let cycles = instruction.execute(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(memory.read_byte(0xFF12), 0x12);
    }

    #[test]
    fn test_ldh_mem_c_from_a() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        let adress = 0xFF00;
        cpu.registers.set_register_8(Register8::A, 0x12);
        cpu.registers.set_register_8(Register8::C, 0x34);

        let instruction = Instruction::LdhMemCFromA;
        let cycles = instruction.execute(&mut cpu);

        assert_eq!(cycles, 2);
        assert_eq!(memory.read_byte(0xFF34), 0x12);
    }

    #[test]
    fn test_ld_a_from_mem_n16() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        memory.write_byte(0x1234, 0x12);

        let instruction = Instruction::LdAFromMemN16(0x1234);
        let cycles = instruction.execute(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.get_register_8(Register8::A), 0x12);
    }

    #[test]
    fn test_ldh_a_from_mem_n16() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        memory.write_byte(0xFF12, 0x12);

        let instruction = Instruction::LdhAFromMemN16(0x12);
        let cycles = instruction.execute(&mut cpu);

        assert_eq!(cycles, 4);
        assert_eq!(cpu.registers.get_register_8(Register8::A), 0x12);
    }

    #[test]
    fn test_ldh_a_from_mem_c() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        memory.write_byte(0xFF34, 0x12);
        cpu.registers.set_register_8(Register8::C, 0x34);

        let instruction = Instruction::LdhAFromMemC;
        let cycles = instruction.execute(&mut cpu);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.get_register_8(Register8::A), 0x12);
    }

    #[test]
    fn test_ld_mem_hli_from_a() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        cpu.registers.set_register_8(Register8::A, 0x12);
        cpu.registers.set_register_16(Register16::HL, 0x1234);

        let instruction = Instruction::LdMemHliFromA;
        let cycles = instruction.execute(&mut cpu);

        assert_eq!(cycles, 2);
        assert_eq!(memory.read_byte(0x1234), 0x12);
        assert_eq!(cpu.registers.get_register_16(Register16::HL), 0x1235);
    }

    #[test]
    fn test_ld_mem_hld_from_a() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        cpu.registers.set_register_8(Register8::A, 0x12);
        cpu.registers.set_register_16(Register16::HL, 0x1234);

        let instruction = Instruction::LdMemHldFromA;
        let cycles = instruction.execute(&mut cpu);

        assert_eq!(cycles, 2);
        assert_eq!(memory.read_byte(0x1234), 0x12);
        assert_eq!(cpu.registers.get_register_16(Register16::HL), 0x1233);
    }

    #[test]
    fn test_ld_a_from_mem_hli() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        memory.write_byte(0x1234, 0x12);
        cpu.registers.set_register_16(Register16::HL, 0x1234);

        let instruction = Instruction::LdAFromMemHli;
        let cycles = instruction.execute(&mut cpu);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.get_register_8(Register8::A), 0x12);
        assert_eq!(cpu.registers.get_register_16(Register16::HL), 0x1235);
    }

    #[test]
    fn test_ld_a_from_mem_hld() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        memory.write_byte(0x1234, 0x12);
        cpu.registers.set_register_16(Register16::HL, 0x1234);

        let instruction = Instruction::LdAFromMemHld;
        let cycles = instruction.execute(&mut cpu);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.get_register_8(Register8::A), 0x12);
        assert_eq!(cpu.registers.get_register_16(Register16::HL), 0x1233);
    }

    #[test]
    fn test_adc_r8_no_carry() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        cpu.registers.set_register_8(Register8::A, 0x12);
        cpu.registers.set_register_8(Register8::B, 0x34);

        let instruction = Instruction::AdcR8(Register8::B);
        let flag = cpu.registers.get_flag(Flag::Carry);
        let cycles = instruction.execute(&mut cpu);

        assert_eq!(cycles, 1);
        assert_eq!(flag, 0);
        assert_eq!(cpu.registers.get_register_8(Register8::A), 0x46);
    }

    #[test]
    fn test_adc_r8_with_carry_register() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        cpu.registers.set_register_8(Register8::A, 0xFF);
        cpu.registers.set_register_8(Register8::B, 0x01);
        cpu.registers.set_flags(None, None, None, None);

        let instruction = Instruction::AdcR8(Register8::B);
        let flag = cpu.registers.get_flag(Flag::Carry);
        let cycles = instruction.execute(&mut cpu);

        assert_eq!(cycles, 1);
        assert_eq!(flag, 0);
        assert_eq!(cpu.registers.get_register_8(Register8::A), 0x0);
    }

    #[test]
    fn test_adc_r8_with_carry_flag() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        cpu.registers.set_register_8(Register8::A, 0xFF);
        cpu.registers.set_register_8(Register8::B, 0x0);
        cpu.registers.set_flags(None, None, None, Some(1));

        let instruction = Instruction::AdcR8(Register8::B);
        let flag = cpu.registers.get_flag(Flag::Carry);
        let cycles = instruction.execute(&mut cpu);

        assert_eq!(cycles, 1);
        assert_eq!(flag, 1);
        assert_eq!(cpu.registers.get_register_8(Register8::A), 0x00);
    }

    fn test_adc_r8_half_carry_register() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        cpu.registers.set_register_8(Register8::A, 0x0F);
        cpu.registers.set_register_8(Register8::B, 0x01);
        cpu.registers.set_flags(None, None, None, None);

        let instruction = Instruction::AdcR8(Register8::B);
        let flag = cpu.registers.get_flag(Flag::HalfCarry);
        let cycles = instruction.execute(&mut cpu);

        assert_eq!(cycles, 1);
        assert_eq!(flag, 1);
        assert_eq!(cpu.registers.get_register_8(Register8::A), 0x10);
    }

    #[test]
    fn test_adc_r8_half_carry_flag() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        cpu.registers.set_register_8(Register8::A, 0x0F);
        cpu.registers.set_register_8(Register8::B, 0x01);
        cpu.registers.set_flags(None, None, None, None);
        let instruction = Instruction::AdcR8(Register8::B);

        let cycles = instruction.execute(&mut cpu);
        let flag = cpu.registers.get_flag(Flag::HalfCarry);

        assert_eq!(cycles, 1);
        assert_eq!(flag, 1);
        assert_eq!(cpu.registers.get_register_8(Register8::A), 0x10);
    }

    fn test_adc_r8_zero() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        cpu.registers.set_register_8(Register8::A, 0xFF);
        cpu.registers.set_register_8(Register8::B, 0x01);
        cpu.registers.set_flags(None, None, None, None);

        let instruction = Instruction::AdcR8(Register8::B);
        let cycles = instruction.execute(&mut cpu);
        let flag = cpu.registers.get_flag(Flag::Zero);

        assert_eq!(cycles, 1);
        assert_eq!(flag, 1);
        assert_eq!(cpu.registers.get_register_8(Register8::A), 0x00);
    }

    #[test]
    fn test_adc_r8_non_zero() {
        let memory = Memory::new();
        let mut cpu = Cpu::new(&memory);
        cpu.registers.set_register_8(Register8::A, 0x12);
        cpu.registers.set_register_8(Register8::B, 0x34);
        cpu.registers.set_flags(None, None, None, None);

        let instruction = Instruction::AdcR8(Register8::B);
        let cycles = instruction.execute(&mut cpu);
        let flag = cpu.registers.get_flag(Flag::Zero);

        assert_eq!(cycles, 1);
        assert_eq!(flag, 0);
        assert_eq!(cpu.registers.get_register_8(Register8::A), 0x46);
    }
}