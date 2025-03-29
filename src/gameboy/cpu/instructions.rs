use crate::{gameboy::Memory, utils::get_bit_u8};

use super::{
    instruction_variables::{Cond, B3, R16, R16MEM, R16STK, R8, TGT3},
    registers::{Flag, Register16, Register8},
    Cpu,
};

/// Instructions for the Gameboy CPU
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
    IncMemHl,
    DecR8(R8),
    DecMemHl,
    LdR8Imm8(R8, u8),
    LdMemHlImm8(u8),

    Rlca,
    Rrca,
    Rla,
    Rra,
    Daa,
    Cpl,
    Scf,
    Ccf,

    JrImm8(u8),
    JrCondImm8(Cond, u8),

    Stop,

    // Block 1
    LdR8R8(R8, R8),
    LdR8MemHl(R8),
    LdMemHlR8(R8),
    Halt,

    // Block 2
    AddAR8(R8),
    AddAMemHl,
    AdcAR8(R8),
    AdcAMemHl,
    SubAR8(R8),
    SubAMemHl,
    SbcAR8(R8),
    SbcAMemHl,
    AndAR8(R8),
    AndAMemHl,
    XorAR8(R8),
    XorAMemHl,
    OrAR8(R8),
    OrAMemHl,
    CpAR8(R8),
    CpAMemHl,

    // Block 3
    AddAImm8(u8),
    AdcAImm8(u8),
    SubAImm8(u8),
    SbcAImm8(u8),
    AndAImm8(u8),
    XorAImm8(u8),
    OrAImm8(u8),
    CpAImm8(u8),

    RetCond(Cond),
    Ret,
    Reti,
    JpCondImm16(Cond, u16),
    JpImm16(u16),
    JpHl,
    CallCondImm16(Cond, u16),
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
    RlcMemHl,
    RlcR8(R8),
    RrcMemHl,
    RrcR8(R8),
    RlMemHl,
    RlR8(R8),
    RrMemHl,
    RrR8(R8),
    SlaMemHl,
    SlaR8(R8),
    SraMemHl,
    SraR8(R8),
    SwapMemHl,
    SwapR8(R8),
    SrlMemHl,
    SrlR8(R8),

    BitB3MemHl(B3),
    BitB3R8(B3, R8),
    ResB3MemHl(B3),
    ResB3R8(B3, R8),
    SetB3MemHl(B3),
    SetB3R8(B3, R8),
}

impl Instruction {
    /// Execute the instruction
    ///
    /// Consumes the instruction and modifies the CPU and memory
    ///
    /// Returns the number of cycles the instruction took
    pub fn execute(self, cpu: &mut Cpu, memory: &mut Memory) -> u8 {
        match self {
            Instruction::Nop => 1,
            Instruction::LdR16Imm16(register, value) => {
                cpu.registers.write_16(Register16::from(register), value);

                3
            }
            Instruction::LdR16MemA(register) => {
                let value = cpu.registers.read_8(Register8::A);
                let address = cpu.registers.read_16(Register16::from(register));
                memory.write_byte(address, value);

                2
            }
            Instruction::LdAR16Mem(register) => {
                let address = cpu.registers.read_16(Register16::from(register));
                let value = memory.read_byte(address);
                cpu.registers.write_8(Register8::A, value);

                2
            }
            Instruction::LdMemImm16SP(adress) => {
                let value = cpu.registers.read_16(Register16::SP);
                memory.write_word(adress, value);

                5
            }
            Instruction::IncR16(register) => {
                let reg = Register16::from(register);
                let value = cpu.registers.read_16(reg);
                cpu.registers.write_16(reg, value.wrapping_add(1));

                2
            }
            Instruction::DecR16(register) => {
                let reg = Register16::from(register);
                let value = cpu.registers.read_16(reg);
                cpu.registers.write_16(reg, value.wrapping_sub(1));

                2
            }
            Instruction::AddHlR16(register) => {
                let value = cpu.registers.read_16(Register16::from(register));
                let hl = cpu.registers.read_16(Register16::HL);
                let (result, overflow) = hl.overflowing_add(value);
                cpu.registers.write_16(Register16::HL, result);
                cpu.registers.write_flag(Flag::N, 0);
                cpu.registers.write_flag(
                    Flag::H,
                    if check_half_carry_add_u16(hl, value) {
                        1
                    } else {
                        0
                    },
                );
                cpu.registers
                    .write_flag(Flag::C, if overflow { 1 } else { 0 });

                2
            }
            Instruction::IncMemHl => {
                let address = cpu.registers.read_16(Register16::HL);
                let value = memory.read_byte(address);
                let result = value.wrapping_add(1);

                memory.write_byte(address, result);
                cpu.registers
                    .write_flag(Flag::Z, if result == 0 { 1 } else { 0 });
                cpu.registers.write_flag(Flag::N, 0);
                cpu.registers.write_flag(
                    Flag::H,
                    if check_half_carry_add_u8(value, 1) {
                        1
                    } else {
                        0
                    },
                );

                3
            }
            Instruction::IncR8(register) => {
                let reg = Register8::from(register);
                let value = cpu.registers.read_8(reg);
                let result = value.wrapping_add(1);

                cpu.registers.write_8(reg, result);
                cpu.registers
                    .write_flag(Flag::Z, if result == 0 { 1 } else { 0 });
                cpu.registers.write_flag(Flag::N, 0);
                cpu.registers.write_flag(
                    Flag::H,
                    if check_half_carry_add_u8(value, 1) {
                        1
                    } else {
                        0
                    },
                );

                1
            }
            Instruction::DecMemHl => {
                let address = cpu.registers.read_16(Register16::HL);
                let value = memory.read_byte(address);
                let result = value.wrapping_sub(1);

                memory.write_byte(address, result);
                cpu.registers
                    .write_flag(Flag::Z, if result == 0 { 1 } else { 0 });
                cpu.registers.write_flag(Flag::N, 1);
                cpu.registers.write_flag(
                    Flag::H,
                    if check_half_borrow_sub_u8(value, 1) {
                        1
                    } else {
                        0
                    },
                );

                3
            }
            Instruction::DecR8(register) => {
                let reg = Register8::from(register);
                let value = cpu.registers.read_8(reg);
                let result = value.wrapping_sub(1);

                cpu.registers.write_8(reg, result);
                cpu.registers
                    .write_flag(Flag::Z, if result == 0 { 1 } else { 0 });
                cpu.registers.write_flag(Flag::N, 1);
                cpu.registers.write_flag(
                    Flag::H,
                    if check_half_borrow_sub_u8(value, 1) {
                        1
                    } else {
                        0
                    },
                );

                1
            }
            Instruction::LdMemHlImm8(value) => {
                let address = cpu.registers.read_16(Register16::HL);
                memory.write_byte(address, value);

                3
            }
            Instruction::LdR8Imm8(register, value) => {
                let reg = Register8::from(register);
                cpu.registers.write_8(reg, value);

                2
            }
            Instruction::Rlca => {
                let value = cpu.registers.read_8(Register8::A);
                let carry = get_bit_u8(value, 7);
                let result = (value << 1) | carry;

                cpu.registers.write_8(Register8::A, result);
                cpu.registers.write_flag(Flag::Z, 0);
                cpu.registers.write_flag(Flag::N, 0);
                cpu.registers.write_flag(Flag::H, 0);
                cpu.registers.write_flag(Flag::C, carry);

                1
            }
            Instruction::Rrca => {
                let value = cpu.registers.read_8(Register8::A);
                let carry = get_bit_u8(value, 0);
                let result = (value >> 1) | (carry << 7);

                cpu.registers.write_8(Register8::A, result);
                cpu.registers.write_flag(Flag::Z, 0);
                cpu.registers.write_flag(Flag::N, 0);
                cpu.registers.write_flag(Flag::H, 0);
                cpu.registers.write_flag(Flag::C, carry);

                1
            }
            Instruction::Rla => {
                let value = cpu.registers.read_8(Register8::A);
                let carry = cpu.registers.read_flag(Flag::C);

                let result = (value << 1) | carry;
                let new_carry = get_bit_u8(value, 7);

                cpu.registers.write_8(Register8::A, result);
                cpu.registers.write_flag(Flag::Z, 0);
                cpu.registers.write_flag(Flag::N, 0);
                cpu.registers.write_flag(Flag::H, 0);
                cpu.registers.write_flag(Flag::C, new_carry);

                1
            }
            Instruction::Rra => {
                let value = cpu.registers.read_8(Register8::A);
                let carry = cpu.registers.read_flag(Flag::C);

                let result = (value >> 1) | (carry << 7);
                let new_carry = get_bit_u8(value, 0);

                cpu.registers.write_8(Register8::A, result);
                cpu.registers.write_flag(Flag::Z, 0);
                cpu.registers.write_flag(Flag::N, 0);
                cpu.registers.write_flag(Flag::H, 0);
                cpu.registers.write_flag(Flag::C, new_carry);

                1
            }
            Instruction::Daa => {
                let a = cpu.registers.read_8(Register8::A);
                let n = cpu.registers.read_flag(Flag::N);
                let h = cpu.registers.read_flag(Flag::H);
                let c = cpu.registers.read_flag(Flag::C);
                let mut adjustment = 0;

                if h == 0x1 || (a & 0xF) > 9 {
                    adjustment |= 0x6;
                }

                if c == 0x1 || a > 0x99 {
                    adjustment |= 0x60;
                    cpu.registers.write_flag(Flag::C, 0x1);
                }

                let result = if n == 0x0 {
                    a.wrapping_add(adjustment)
                } else {
                    a.wrapping_sub(adjustment)
                };
                cpu.registers.write_8(Register8::A, result);
                cpu.registers
                    .write_flag(Flag::Z, if result == 0 { 0x1 } else { 0x0 });
                cpu.registers.write_flag(Flag::H, 0x0);

                1
            }
            Instruction::Cpl => {
                let a = cpu.registers.read_8(Register8::A);
                cpu.registers.write_8(Register8::A, !a);

                cpu.registers.write_flag(Flag::N, 0x1);
                cpu.registers.write_flag(Flag::H, 0x1);

                1
            }
            Instruction::Scf => {
                cpu.registers.write_flag(Flag::N, 0x0);
                cpu.registers.write_flag(Flag::H, 0x0);
                cpu.registers.write_flag(Flag::C, 0x1);

                1
            }
            Instruction::Ccf => {
                let c = cpu.registers.read_flag(Flag::C);
                cpu.registers.write_flag(Flag::N, 0x0);
                cpu.registers.write_flag(Flag::H, 0x0);
                cpu.registers
                    .write_flag(Flag::C, if c == 0x1 { 0x0 } else { 0x1 });

                1
            }
            Instruction::JrImm8(byte) => {
                let pc = cpu.registers.read_16(Register16::PC);

                let pc_new = pc.wrapping_add_signed(byte as i8 as i16); // two step casting to get the sign extension
                cpu.registers.write_16(Register16::PC, pc_new);

                3
            }
            Instruction::JrCondImm8(condition, byte) => {
                let jump = match condition {
                    Cond::NotZero => cpu.registers.read_flag(Flag::Z) == 0,
                    Cond::Zero => cpu.registers.read_flag(Flag::Z) == 1,
                    Cond::NotCarry => cpu.registers.read_flag(Flag::C) == 0,
                    Cond::Carry => cpu.registers.read_flag(Flag::C) == 1,
                };

                if jump {
                    let pc = cpu.registers.read_16(Register16::PC);
                    let pc_new = pc.wrapping_add_signed(byte as i8 as i16); // two step casting to get the sign extension
                    cpu.registers.write_16(Register16::PC, pc_new);

                    3
                } else {
                    2
                }
            }
            Instruction::Stop => todo!(),
            Instruction::LdMemHlR8(register) => {
                let value = cpu.registers.read_8(Register8::from(register));
                let adress = cpu.registers.read_16(Register16::HL);

                memory.write_byte(adress, value);

                2
            }
            Instruction::LdR8MemHl(register) => {
                let adress = cpu.registers.read_16(Register16::HL);
                let value = memory.read_byte(adress);

                cpu.registers.write_8(Register8::from(register), value);

                2
            }
            Instruction::LdR8R8(target_register, source_register) => {
                let value = cpu.registers.read_8(Register8::from(source_register));

                cpu.registers
                    .write_8(Register8::from(target_register), value);

                1
            }
            Instruction::Halt => todo!(),
            Instruction::AddAMemHl => {
                let adress = cpu.registers.read_16(Register16::HL);
                let value = memory.read_byte(adress);
                let a = cpu.registers.read_8(Register8::A);

                let (result, overflow) = a.overflowing_add(value);

                cpu.registers.write_8(Register8::A, result);
                cpu.registers
                    .write_flag(Flag::Z, if result == 0 { 1 } else { 0 });
                cpu.registers.write_flag(Flag::N, 0);
                cpu.registers.write_flag(
                    Flag::H,
                    if check_half_carry_add_u8(a, value) {
                        1
                    } else {
                        0
                    },
                );
                cpu.registers
                    .write_flag(Flag::C, if overflow { 1 } else { 0 });

                2
            }
            Instruction::AddAR8(register) => {
                let value = cpu.registers.read_8(Register8::from(register));
                let a = cpu.registers.read_8(Register8::A);

                let (result, overflow) = a.overflowing_add(value);

                cpu.registers.write_8(Register8::A, result);
                cpu.registers
                    .write_flag(Flag::Z, if result == 0 { 1 } else { 0 });
                cpu.registers.write_flag(Flag::N, 0);
                cpu.registers.write_flag(
                    Flag::H,
                    if check_half_carry_add_u8(a, value) {
                        1
                    } else {
                        0
                    },
                );
                cpu.registers
                    .write_flag(Flag::C, if overflow { 1 } else { 0 });

                1
            }
            Instruction::AdcAMemHl => {
                let adress = cpu.registers.read_16(Register16::HL);
                let value = memory.read_byte(adress);
                let a = cpu.registers.read_8(Register8::A);
                let carry = cpu.registers.read_flag(Flag::C);

                let (partial_result, overflow_add_a_carry) = a.overflowing_add(carry);
                let (result, overflow_add_sub_result_value) = partial_result.overflowing_add(value);

                let half_overflow = check_half_carry_add_u8(a, carry)
                    || check_half_carry_add_u8(partial_result, value);
                let overflow = overflow_add_a_carry || overflow_add_sub_result_value;

                cpu.registers.write_8(Register8::A, result);
                cpu.registers
                    .write_flag(Flag::Z, if result == 0 { 1 } else { 0 });
                cpu.registers.write_flag(Flag::N, 0);
                cpu.registers
                    .write_flag(Flag::H, if half_overflow { 1 } else { 0 });
                cpu.registers
                    .write_flag(Flag::C, if overflow { 1 } else { 0 });

                2
            }
            Instruction::AdcAR8(register) => {
                let value = cpu.registers.read_8(Register8::from(register));

                let a = cpu.registers.read_8(Register8::A);
                let carry = cpu.registers.read_flag(Flag::C);

                let (partial_result, overflow_add_a_carry) = a.overflowing_add(carry);
                let (result, overflow_add_sub_result_value) = partial_result.overflowing_add(value);

                let half_overflow = check_half_carry_add_u8(a, carry)
                    || check_half_carry_add_u8(partial_result, value);
                let overflow = overflow_add_a_carry || overflow_add_sub_result_value;

                cpu.registers.write_8(Register8::A, result);
                cpu.registers
                    .write_flag(Flag::Z, if result == 0 { 1 } else { 0 });
                cpu.registers.write_flag(Flag::N, 0);
                cpu.registers
                    .write_flag(Flag::H, if half_overflow { 1 } else { 0 });
                cpu.registers
                    .write_flag(Flag::C, if overflow { 1 } else { 0 });

                1
            }
            Instruction::SubAMemHl => {
                let adress = cpu.registers.read_16(Register16::HL);
                let value = memory.read_byte(adress);
                let a = cpu.registers.read_8(Register8::A);
                let (result, borrow) = a.overflowing_sub(value);

                cpu.registers.write_8(Register8::A, result);
                cpu.registers
                    .write_flag(Flag::Z, if result == 0 { 1 } else { 0 });
                cpu.registers.write_flag(Flag::N, 1);
                cpu.registers.write_flag(
                    Flag::H,
                    if check_half_borrow_sub_u8(a, value) {
                        1
                    } else {
                        0
                    },
                );
                cpu.registers
                    .write_flag(Flag::C, if borrow { 1 } else { 0 });

                2
            }
            Instruction::SubAR8(register) => {
                let value = cpu.registers.read_8(Register8::from(register));
                let a = cpu.registers.read_8(Register8::A);
                let (result, borrow) = a.overflowing_sub(value);

                cpu.registers.write_8(Register8::A, result);
                cpu.registers
                    .write_flag(Flag::Z, if result == 0 { 1 } else { 0 });
                cpu.registers.write_flag(Flag::N, 1);
                cpu.registers.write_flag(
                    Flag::H,
                    if check_half_borrow_sub_u8(a, value) {
                        1
                    } else {
                        0
                    },
                );
                cpu.registers
                    .write_flag(Flag::C, if borrow { 1 } else { 0 });

                1
            }
            Instruction::SbcAMemHl => {
                let adress = cpu.registers.read_16(Register16::HL);
                let value = memory.read_byte(adress);
                let a = cpu.registers.read_8(Register8::A);
                let carry = cpu.registers.read_flag(Flag::C);

                let (sub_result, borrow_sub_a_borrow) = a.overflowing_sub(carry);
                let (result, borrow_sub_result_value) = sub_result.overflowing_sub(value);

                let half_borrow = check_half_borrow_sub_u8(a, carry)
                    || check_half_borrow_sub_u8(sub_result, value);
                let overflow = borrow_sub_a_borrow || borrow_sub_result_value;

                cpu.registers.write_8(Register8::A, result);
                cpu.registers
                    .write_flag(Flag::Z, if result == 0 { 1 } else { 0 });
                cpu.registers.write_flag(Flag::N, 1);
                cpu.registers
                    .write_flag(Flag::H, if half_borrow { 1 } else { 0 });
                cpu.registers
                    .write_flag(Flag::C, if overflow { 1 } else { 0 });

                2
            }
            Instruction::SbcAR8(register) => {
                let a = cpu.registers.read_8(Register8::A);
                let value = cpu.registers.read_8(Register8::from(register));
                let carry = cpu.registers.read_flag(Flag::C);

                let (sub_result, borrow_sub_a_borrow) = a.overflowing_sub(carry);
                let (result, borrow_sub_result_value) = sub_result.overflowing_sub(value);

                let half_borrow = check_half_borrow_sub_u8(a, carry)
                    || check_half_borrow_sub_u8(sub_result, value);
                let overflow = borrow_sub_a_borrow || borrow_sub_result_value;

                cpu.registers.write_8(Register8::A, result);
                cpu.registers
                    .write_flag(Flag::Z, if result == 0 { 1 } else { 0 });
                cpu.registers.write_flag(Flag::N, 1);
                cpu.registers
                    .write_flag(Flag::H, if half_borrow { 1 } else { 0 });
                cpu.registers
                    .write_flag(Flag::C, if overflow { 1 } else { 0 });

                1
            }
            Instruction::AndAMemHl => {
                let adress = cpu.registers.read_16(Register16::HL);
                let value = memory.read_byte(adress);
                let a = cpu.registers.read_8(Register8::A);

                let result = a & value;

                cpu.registers.write_8(Register8::A, result);
                cpu.registers
                    .write_flag(Flag::Z, if result == 0 { 1 } else { 0 });
                cpu.registers.write_flag(Flag::N, 0);
                cpu.registers.write_flag(Flag::H, 1);

                2
            }
            Instruction::AndAR8(register) => {
                let value = cpu.registers.read_8(Register8::from(register));
                let a = cpu.registers.read_8(Register8::A);

                let result = a & value;

                cpu.registers.write_8(Register8::A, result);
                cpu.registers
                    .write_flag(Flag::Z, if result == 0 { 1 } else { 0 });
                cpu.registers.write_flag(Flag::N, 0);
                cpu.registers.write_flag(Flag::H, 1);

                1
            }
            Instruction::XorAMemHl => {
                let adress = cpu.registers.read_16(Register16::HL);
                let value = memory.read_byte(adress);
                let a = cpu.registers.read_8(Register8::A);

                let result = a ^ value;

                cpu.registers.write_8(Register8::A, result);
                cpu.registers
                    .write_flag(Flag::Z, if result == 0 { 1 } else { 0 });
                cpu.registers.write_flag(Flag::N, 0);
                cpu.registers.write_flag(Flag::H, 0);

                2
            }
            Instruction::XorAR8(register) => {
                let value = cpu.registers.read_8(Register8::from(register));
                let a = cpu.registers.read_8(Register8::A);

                let result = a ^ value;

                cpu.registers.write_8(Register8::A, result);
                cpu.registers
                    .write_flag(Flag::Z, if result == 0 { 1 } else { 0 });
                cpu.registers.write_flag(Flag::N, 0);
                cpu.registers.write_flag(Flag::H, 0);

                1
            }
            Instruction::OrAMemHl => {
                let adress = cpu.registers.read_16(Register16::HL);
                let value = memory.read_byte(adress);
                let a = cpu.registers.read_8(Register8::A);

                let result = a | value;

                cpu.registers.write_8(Register8::A, result);
                cpu.registers
                    .write_flag(Flag::Z, if result == 0 { 1 } else { 0 });
                cpu.registers.write_flag(Flag::N, 0);
                cpu.registers.write_flag(Flag::H, 0);
                cpu.registers.write_flag(Flag::C, 0);

                2
            }
            Instruction::OrAR8(register) => {
                let value = cpu.registers.read_8(Register8::from(register));
                let a = cpu.registers.read_8(Register8::A);

                let result = a | value;

                cpu.registers.write_8(Register8::A, result);
                cpu.registers
                    .write_flag(Flag::Z, if result == 0 { 1 } else { 0 });
                cpu.registers.write_flag(Flag::N, 0);
                cpu.registers.write_flag(Flag::H, 0);
                cpu.registers.write_flag(Flag::C, 0);

                1
            }
            Instruction::CpAMemHl => {
                let adress = cpu.registers.read_16(Register16::HL);
                let value = memory.read_byte(adress);
                let a = cpu.registers.read_8(Register8::A);

                let (result, borrow) = a.overflowing_sub(value);

                cpu.registers
                    .write_flag(Flag::Z, if result == 0 { 1 } else { 0 });
                cpu.registers.write_flag(Flag::N, 1);
                cpu.registers.write_flag(
                    Flag::H,
                    if check_half_borrow_sub_u8(a, value) {
                        1
                    } else {
                        0
                    },
                );
                cpu.registers
                    .write_flag(Flag::C, if borrow { 1 } else { 0 });

                2
            }
            Instruction::CpAR8(register) => {
                let value = cpu.registers.read_8(Register8::from(register));
                let a = cpu.registers.read_8(Register8::A);

                let (result, borrow) = a.overflowing_sub(value);

                cpu.registers
                    .write_flag(Flag::Z, if result == 0 { 1 } else { 0 });
                cpu.registers.write_flag(Flag::N, 1);
                cpu.registers.write_flag(
                    Flag::H,
                    if check_half_borrow_sub_u8(a, value) {
                        1
                    } else {
                        0
                    },
                );
                cpu.registers
                    .write_flag(Flag::C, if borrow { 1 } else { 0 });

                1
            }
            Instruction::AddAImm8(value) => {
                let a = cpu.registers.read_8(Register8::A);

                let result = a.wrapping_add(value);

                cpu.registers.write_8(Register8::A, result);
                cpu.registers
                    .write_flag(Flag::Z, if result == 0 { 1 } else { 0 });
                cpu.registers.write_flag(Flag::N, 0);
                cpu.registers.write_flag(
                    Flag::H,
                    if check_half_carry_add_u8(a, value) {
                        1
                    } else {
                        0
                    },
                );
                cpu.registers.write_flag(
                    Flag::C,
                    if (a as u16 + value as u16) > 0xFF {
                        1
                    } else {
                        0
                    },
                );

                2
            }
            Instruction::AdcAImm8(value) => {
                let a = cpu.registers.read_8(Register8::A);
                let c = cpu.registers.read_flag(Flag::C);

                let (sub_result, sub_result_carry) = a.overflowing_add(c);
                let (result, result_carry) = sub_result.overflowing_add(value);

                let half_carry =
                    check_half_carry_add_u8(a, c) || check_half_carry_add_u8(sub_result, value);
                let carry = sub_result_carry || result_carry;

                cpu.registers.write_8(Register8::A, result);
                cpu.registers
                    .write_flag(Flag::Z, if result == 0 { 1 } else { 0 });
                cpu.registers.write_flag(Flag::N, 0);
                cpu.registers
                    .write_flag(Flag::H, if half_carry { 1 } else { 0 });
                cpu.registers.write_flag(Flag::C, if carry { 1 } else { 0 });

                2
            }
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
            Instruction::RlcMemHl => todo!(),
            Instruction::RlcR8(register8) => todo!(),
            Instruction::RrcMemHl => todo!(),
            Instruction::RrcR8(register8) => todo!(),
            Instruction::RlMemHl => todo!(),
            Instruction::RlR8(register8) => todo!(),
            Instruction::RrMemHl => todo!(),
            Instruction::RrR8(register8) => todo!(),
            Instruction::SlaMemHl => todo!(),
            Instruction::SlaR8(register8) => todo!(),
            Instruction::SraMemHl => todo!(),
            Instruction::SraR8(register8) => todo!(),
            Instruction::SwapMemHl => todo!(),
            Instruction::SwapR8(register8) => todo!(),
            Instruction::SrlMemHl => todo!(),
            Instruction::SrlR8(register8) => todo!(),
            Instruction::BitB3MemHl(b3) => todo!(),
            Instruction::BitB3R8(b3, register8) => todo!(),
            Instruction::ResB3MemHl(b3) => todo!(),
            Instruction::ResB3R8(b3, register8) => todo!(),
            Instruction::SetB3MemHl(b3) => todo!(),
            Instruction::SetB3R8(b3, register8) => todo!(),
        }
    }
}

// helpers

// utils

fn check_half_carry_add_u8(left: u8, right: u8) -> bool {
    (((left & 0xF) + (right & 0xF)) & 0x10) != 0x0
}

fn check_half_carry_add_u16(left: u16, right: u16) -> bool {
    (((left & 0xFFF) + (right & 0xFFF)) & 0x1000) != 0x0
}

fn check_half_borrow_sub_u8(left: u8, right: u8) -> bool {
    (left & 0xF) < (right & 0xF)
}

fn check_half_borrow_sub_u16(left: u16, right: u16) -> bool {
    (left & 0xFFF) < (right & 0xFFF)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_half_carry_add_u8() {
        assert!(check_half_carry_add_u8(0x0F, 0x01));
        assert!(check_half_carry_add_u8(0x0F, 0x0F));
        assert!(!check_half_carry_add_u8(0x0F, 0x00));
        assert!(!check_half_carry_add_u8(0x00, 0x00));
    }

    #[test]
    fn test_check_half_carry_add_u16() {
        assert!(check_half_carry_add_u16(0x0FFF, 0x0001));
        assert!(check_half_carry_add_u16(0x0FFF, 0x0FFF));
        assert!(!check_half_carry_add_u16(0x0FFF, 0x0000));
        assert!(!check_half_carry_add_u16(0x0000, 0x0000));
    }

    #[test]
    fn test_check_half_borrow_sub_u8() {
        assert!(!check_half_borrow_sub_u8(0x01, 0x01));
        assert!(check_half_borrow_sub_u8(0x01, 0x02));
        assert!(check_half_borrow_sub_u8(0x00, 0x01));
        assert!(!check_half_borrow_sub_u8(0x0F, 0x01));
    }

    #[test]
    fn test_check_half_borrow_sub_u16() {
        assert!(!check_half_borrow_sub_u16(0x0001, 0x0001));
        assert!(check_half_borrow_sub_u16(0x0001, 0x0002));
        assert!(check_half_borrow_sub_u16(0x0000, 0x0001));
        assert!(!check_half_borrow_sub_u16(0x0FFF, 0x0001));
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
    fn test_inc_memhl() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::IncMemHl;
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0x0);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 3);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(memory.read_byte(0x1234), 0x1);
    }

    #[test]
    fn test_inc_memhl_half_overflow() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::IncMemHl;
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0xF);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 3);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(memory.read_byte(0x1234), 0x10);
    }

    #[test]
    fn test_inc_memhl_zero() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::IncMemHl;
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0xFF);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 3);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 1);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(memory.read_byte(0x1234), 0x00);
    }

    #[test]
    fn test_dec_r8() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::DecR8(R8::B);
        cpu.registers.write_8(Register8::B, 0x2);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_8(Register8::B), 0x1);
    }

    #[test]
    fn test_dec_r8_half_borrow() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::DecR8(R8::B);
        cpu.registers.write_8(Register8::B, 0x10);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(cpu.registers.read_8(Register8::B), 0xF);
    }

    #[test]
    fn test_dec_r8_zero() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::DecR8(R8::B);
        cpu.registers.write_8(Register8::B, 0x1);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 1);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_8(Register8::B), 0x0);
    }

    #[test]
    fn test_dec_memhl() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::DecMemHl;
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0x2);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 3);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(memory.read_byte(0x1234), 0x1);
    }

    #[test]
    fn test_dec_memhl_half_borrow() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::DecMemHl;
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0x10);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 3);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(memory.read_byte(0x1234), 0xF);
    }

    #[test]
    fn test_dec_memhl_zero() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::DecMemHl;
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0x1);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 3);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 1);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(memory.read_byte(0x1234), 0x0);
    }

    #[test]
    fn test_ld_r8_imm8() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::LdR8Imm8(R8::B, 0xAB);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::B), 0xAB);
    }

    #[test]
    fn test_ld_memhl_imm8() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::LdMemHlImm8(0xAB);
        cpu.registers.write_16(Register16::HL, 0x1234);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 3);
        assert_eq!(memory.read_byte(0x1234), 0xAB);
    }

    #[test]
    fn test_rlca() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::Rlca;

        cpu.registers.write_8(Register8::A, 0b1001_1010);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0b0011_0101);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 1);
    }

    #[test]
    fn test_rrca() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::Rrca;
        cpu.registers.write_8(Register8::A, 0b1001_1011);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0b1100_1101);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 1);
    }

    #[test]
    fn test_rla() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::Rla;
        cpu.registers.write_8(Register8::A, 0b0001_1010);
        cpu.registers.write_flag(Flag::C, 1);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0b0011_0101);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_rra() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::Rra;
        cpu.registers.write_8(Register8::A, 0b0001_1010);
        cpu.registers.write_flag(Flag::C, 1);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0b1000_1101);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_daa_no_change() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::Daa;
        cpu.registers.write_8(Register8::A, 0x45);
        cpu.registers.write_flag(Flag::N, 0);
        cpu.registers.write_flag(Flag::H, 0);
        cpu.registers.write_flag(Flag::C, 0);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x45);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_daa_n_true_half() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::Daa;

        cpu.registers.write_8(Register8::A, 0x45);

        cpu.registers.write_flag(Flag::N, 1);
        cpu.registers.write_flag(Flag::H, 1); // adjustment of 6
        cpu.registers.write_flag(Flag::C, 0);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.registers.read_8(Register8::A), 0x3F);
        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_daa_n_true_carry() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::Daa;

        cpu.registers.write_8(Register8::A, 0x61);

        cpu.registers.write_flag(Flag::N, 1);
        cpu.registers.write_flag(Flag::H, 0);
        cpu.registers.write_flag(Flag::C, 1); // adjustment of 60

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x01);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 1);
    }

    #[test]
    fn test_daa_n_false_half() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::Daa;

        cpu.registers.write_8(Register8::A, 0x45);

        cpu.registers.write_flag(Flag::N, 0);
        cpu.registers.write_flag(Flag::H, 1); // adjustment of 6
        cpu.registers.write_flag(Flag::C, 0);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x4B);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_daa_n_false_carry() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::Daa;

        cpu.registers.write_8(Register8::A, 0x60);

        cpu.registers.write_flag(Flag::N, 0);
        cpu.registers.write_flag(Flag::H, 0);
        cpu.registers.write_flag(Flag::C, 1); // adjustment of 60

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0xC0);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 1);
    }

    /// Tests for alternative DAA trigger to apply offset: target is larger than 0x90
    #[test]
    fn test_daa_n_false_half_alternative_large() {
        // if A 0xF > 0x9
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::Daa;

        cpu.registers.write_8(Register8::A, 0xA0);

        cpu.registers.write_flag(Flag::N, 0);
        cpu.registers.write_flag(Flag::H, 0);
        cpu.registers.write_flag(Flag::C, 0);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x0);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 1);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 1);
    }

    #[test]
    fn test_daa_n_false_half_alternative_small() {
        // if A 0x6 < 0x9
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::Daa;

        cpu.registers.write_8(Register8::A, 0x4A);

        cpu.registers.write_flag(Flag::N, 0);
        cpu.registers.write_flag(Flag::H, 0);
        cpu.registers.write_flag(Flag::C, 0);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x50);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_cpl() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::Cpl;
        cpu.registers.write_8(Register8::A, 0x45);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0xBA);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
    }

    #[test]
    fn test_scf() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::Scf;

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 1);
    }

    #[test]
    fn test_ccf() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::Ccf;
        cpu.registers.write_flag(Flag::C, 1);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_jr_imm8() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::JrImm8(10i8 as u8);
        let old_pc = cpu.registers.read_16(Register16::PC);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 3);
        assert_eq!(cpu.registers.read_16(Register16::PC), old_pc + 10);
    }

    #[test]
    fn test_jr_imm8_negative() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::JrImm8(-2i8 as u8);
        cpu.registers.write_16(Register16::PC, 0x1000);
        let old_pc = cpu.registers.read_16(Register16::PC);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 3);
        assert_eq!(cpu.registers.read_16(Register16::PC), old_pc - 2);
    }

    #[test]
    fn test_jr_cond_imm8() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::JrCondImm8(Cond::Zero, 10i8 as u8);
        cpu.registers.write_flag(Flag::Z, 1);
        let old_pc = cpu.registers.read_16(Register16::PC);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 3);
        assert_eq!(cpu.registers.read_16(Register16::PC), old_pc + 10);
    }

    #[test]
    fn test_jr_cond_imm8_false() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        cpu.registers.pc = 0x1000;
        let instruction = Instruction::JrCondImm8(Cond::Zero, -10i8 as u8);
        let old_pc = cpu.registers.read_16(Register16::PC);
        cpu.registers.write_flag(Flag::Z, 1);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 3);
        assert_eq!(cpu.registers.read_16(Register16::PC), old_pc - 10);
    }

    #[test]
    fn test_jr_cond_imm8_untaken() {
        let mut cpu: Cpu = Cpu::new();
        let mut memory: Memory = Memory::new();
        let instruction = Instruction::JrCondImm8(Cond::Zero, 10i8 as u8);
        let old_pc = cpu.registers.read_16(Register16::PC);
        cpu.registers.write_flag(Flag::Z, 0);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_16(Register16::PC), old_pc);
    }

    #[test]
    fn test_ld_r8_r8() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::LdR8R8(R8::A, R8::B);
        cpu.registers.write_8(Register8::A, 0x12);
        cpu.registers.write_8(Register8::B, 0x34);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x34);
    }

    #[test]
    fn test_ld_r8_memhl() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::LdR8MemHl(R8::A);
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0x56);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x56);
    }

    #[test]
    fn test_ld_memhl_r8() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::LdMemHlR8(R8::A);
        cpu.registers.write_16(Register16::HL, 0x1234);
        cpu.registers.write_8(Register8::A, 0x56);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(memory.read_byte(0x1234), 0x56);
    }

    #[test]
    fn test_ld_r8_r8_no_op() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::LdR8R8(R8::A, R8::A);
        cpu.registers.write_8(Register8::A, 0x12);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x12);
    }

    #[test]
    fn test_add_a_r8() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::AddAR8(R8::B);
        cpu.registers.write_8(Register8::A, 0x12);
        cpu.registers.write_8(Register8::B, 0x34);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x46);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_add_a_r8_zero() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::AddAR8(R8::B);
        cpu.registers.write_8(Register8::A, 0x00);
        cpu.registers.write_8(Register8::B, 0x00);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x00);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 1);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_add_a_r8_half_carry() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::AddAR8(R8::B);
        cpu.registers.write_8(Register8::A, 0x0F);
        cpu.registers.write_8(Register8::B, 0x01);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x10);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_add_a_r8_carry() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::AddAR8(R8::B);
        cpu.registers.write_8(Register8::A, 0xFF);
        cpu.registers.write_8(Register8::B, 0x01);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x00);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 1);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(cpu.registers.read_flag(Flag::C), 1);
    }

    #[test]
    fn test_add_a_memhl() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::AddAMemHl;
        cpu.registers.write_8(Register8::A, 0x12);
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0x34);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x46);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_add_a_memhl_zero() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::AddAMemHl;
        cpu.registers.write_8(Register8::A, 0x00);
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0x00);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x00);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 1);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_add_a_memhl_half_carry() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::AddAMemHl;
        cpu.registers.write_8(Register8::A, 0x0F);
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0x01);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x10);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_add_a_memhl_carry() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::AddAMemHl;
        cpu.registers.write_8(Register8::A, 0xFF);
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0x01);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x00);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 1);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(cpu.registers.read_flag(Flag::C), 1);
    }

    #[test]
    fn test_adc_a_r8() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::AdcAR8(R8::B);
        cpu.registers.write_8(Register8::A, 0x12);
        cpu.registers.write_8(Register8::B, 0x34);
        cpu.registers.write_flag(Flag::C, 1);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x47);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_adc_a_r8_zero() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::AdcAR8(R8::B);
        cpu.registers.write_8(Register8::A, 0x00);
        cpu.registers.write_8(Register8::B, 0x00);
        cpu.registers.write_flag(Flag::C, 0);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x00);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 1);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_adc_a_r8_half_carry() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::AdcAR8(R8::B);
        cpu.registers.write_8(Register8::A, 0x0E);
        cpu.registers.write_8(Register8::B, 0x01);
        cpu.registers.write_flag(Flag::C, 1);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x10);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_adc_a_r8_carry() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::AdcAR8(R8::B);
        cpu.registers.write_8(Register8::A, 0xFE);
        cpu.registers.write_8(Register8::B, 0x01);
        cpu.registers.write_flag(Flag::C, 1);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x00);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 1);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(cpu.registers.read_flag(Flag::C), 1);
    }

    #[test]
    fn test_adc_a_memhl() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::AdcAMemHl;
        cpu.registers.write_8(Register8::A, 0x12);
        cpu.registers.write_16(Register16::HL, 0x1234);
        cpu.registers.write_flag(Flag::C, 1);
        memory.write_byte(0x1234, 0x34);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x47);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_adc_a_memhl_zero() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::AdcAMemHl;
        cpu.registers.write_8(Register8::A, 0x00);
        cpu.registers.write_16(Register16::HL, 0x1234);
        cpu.registers.write_flag(Flag::C, 0);
        memory.write_byte(0x1234, 0x00);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x00);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 1);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_adc_a_memhl_half_carry() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::AdcAMemHl;
        cpu.registers.write_8(Register8::A, 0x0E);
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0x01);
        cpu.registers.write_flag(Flag::C, 1);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x10);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_adc_a_memhl_carry() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::AdcAMemHl;
        cpu.registers.write_8(Register8::A, 0xFE);
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0x01);
        cpu.registers.write_flag(Flag::C, 1);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x00);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 1);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(cpu.registers.read_flag(Flag::C), 1);
    }

    #[test]
    fn test_sub_a_r8() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::SubAR8(R8::B);
        cpu.registers.write_8(Register8::A, 0x34);
        cpu.registers.write_8(Register8::B, 0x12);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x22);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_sub_a_r8_zero() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::SubAR8(R8::B);
        cpu.registers.write_8(Register8::A, 0x00);
        cpu.registers.write_8(Register8::B, 0x00);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x00);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 1);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_sub_a_r8_half_carry() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::SubAR8(R8::B);
        cpu.registers.write_8(Register8::A, 0x10);
        cpu.registers.write_8(Register8::B, 0x01);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x0F);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_sub_a_r8_carry() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::SubAR8(R8::B);
        cpu.registers.write_8(Register8::A, 0x00);
        cpu.registers.write_8(Register8::B, 0x01);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0xFF);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(cpu.registers.read_flag(Flag::C), 1);
    }

    #[test]
    fn test_sub_a_memhl() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::SubAMemHl;
        cpu.registers.write_8(Register8::A, 0x34);
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0x12);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x22);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_sub_a_memhl_zero() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::SubAMemHl;
        cpu.registers.write_8(Register8::A, 0x00);
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0x00);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x00);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 1);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_sub_a_memhl_half_carry() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::SubAMemHl;
        cpu.registers.write_8(Register8::A, 0x10);
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0x01);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x0F);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_sub_a_memhl_carry() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::SubAMemHl;
        cpu.registers.write_8(Register8::A, 0x00);
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0x01);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0xFF);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(cpu.registers.read_flag(Flag::C), 1);
    }

    #[test]
    fn test_sbc_a_r8() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::SbcAR8(R8::B);
        cpu.registers.write_8(Register8::A, 0x34);
        cpu.registers.write_8(Register8::B, 0x32);
        cpu.registers.write_flag(Flag::C, 1);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 1);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_sbc_a_r8_zero() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::SbcAR8(R8::B);
        cpu.registers.write_8(Register8::A, 0x00);
        cpu.registers.write_8(Register8::B, 0x00);
        cpu.registers.write_flag(Flag::C, 0);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x00);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 1);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_sbc_a_r8_half_carry() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::SbcAR8(R8::B);
        cpu.registers.write_8(Register8::A, 0x10);
        cpu.registers.write_8(Register8::B, 0x01);
        cpu.registers.write_flag(Flag::C, 1);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x0E);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_sbc_a_r8_carry() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::SbcAR8(R8::B);
        cpu.registers.write_8(Register8::A, 0x00);
        cpu.registers.write_8(Register8::B, 0x0);
        cpu.registers.write_flag(Flag::C, 1);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0xFF);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
    }

    #[test]
    fn test_sbc_a_memhl() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::SbcAMemHl;
        cpu.registers.write_8(Register8::A, 0x34);
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0x32);
        cpu.registers.write_flag(Flag::C, 1);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 1);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_sbc_a_memhl_zero() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::SbcAMemHl;
        cpu.registers.write_8(Register8::A, 0x00);
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0x00);
        cpu.registers.write_flag(Flag::C, 0);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x00);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 1);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_sbc_a_memhl_half_carry() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::SbcAMemHl;
        cpu.registers.write_8(Register8::A, 0x10);
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0x01);
        cpu.registers.write_flag(Flag::C, 1);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x0E);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_sbc_a_memhl_carry() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::SbcAMemHl;
        cpu.registers.write_8(Register8::A, 0x00);
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0x01);
        cpu.registers.write_flag(Flag::C, 1);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0xFE);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(cpu.registers.read_flag(Flag::C), 1);
    }

    #[test]
    fn test_and_a_r8() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::AndAR8(R8::B);
        cpu.registers.write_8(Register8::A, 0b1010_1010);
        cpu.registers.write_8(Register8::B, 0b1100_1100);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0b1000_1000);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_and_a_r8_zero() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::AndAR8(R8::B);
        cpu.registers.write_8(Register8::A, 0b1010_1010);
        cpu.registers.write_8(Register8::B, 0b0101_0101);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 1);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
    }

    #[test]
    fn test_and_a_memhl() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::AndAMemHl;
        cpu.registers.write_8(Register8::A, 0b1010_1010);
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0b1100_1100);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0b1000_1000);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
    }

    #[test]
    fn test_and_a_memhl_zero() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::AndAMemHl;
        cpu.registers.write_8(Register8::A, 0b1010_1010);
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0b0101_0101);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 1);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
    }

    #[test]
    fn test_xor_a_r8() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::XorAR8(R8::B);
        cpu.registers.write_8(Register8::A, 0b1010_1010);
        cpu.registers.write_8(Register8::B, 0b1100_1100);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0b0110_0110);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_xor_a_r8_zero() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::XorAR8(R8::B);
        cpu.registers.write_8(Register8::A, 0b1010_1010);
        cpu.registers.write_8(Register8::B, 0b1010_1010);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 1);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
    }

    #[test]
    fn test_xor_a_memhl() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::XorAMemHl;
        cpu.registers.write_8(Register8::A, 0b1010_1010);
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0b1100_1100);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0b0110_0110);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_xor_a_memhl_zero() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::XorAMemHl;
        cpu.registers.write_8(Register8::A, 0b1010_1010);
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0b1010_1010);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 1);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
    }

    #[test]
    fn test_or_a_r8() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::OrAR8(R8::B);
        cpu.registers.write_8(Register8::A, 0b1010_1010);
        cpu.registers.write_8(Register8::B, 0b1100_1100);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0b1110_1110);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_or_a_r8_zero() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::OrAR8(R8::B);
        cpu.registers.write_8(Register8::A, 0x0);
        cpu.registers.write_8(Register8::B, 0x0);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_8(Register8::A), 0);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 1);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
    }

    #[test]
    fn test_or_a_memhl() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::OrAMemHl;
        cpu.registers.write_8(Register8::A, 0b1010_1010);
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0b1100_1100);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0b1110_1110);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_or_a_memhl_zero() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::OrAMemHl;
        cpu.registers.write_8(Register8::A, 0x0);
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0x0);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 1);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
    }

    #[test]
    fn test_cp_a_r8() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::CpAR8(R8::B);
        cpu.registers.write_8(Register8::A, 0x34);
        cpu.registers.write_8(Register8::B, 0x32);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_cp_a_r8_zero() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::CpAR8(R8::B);
        cpu.registers.write_8(Register8::A, 0x0);
        cpu.registers.write_8(Register8::B, 0x0);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 1);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_cp_a_r8_half_carry() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::CpAR8(R8::B);
        cpu.registers.write_8(Register8::A, 0x10);
        cpu.registers.write_8(Register8::B, 0x01);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_cp_a_r8_carry() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::CpAR8(R8::B);
        cpu.registers.write_8(Register8::A, 0x00);
        cpu.registers.write_8(Register8::B, 0x01);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 1);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(cpu.registers.read_flag(Flag::C), 1);
    }

    #[test]
    fn test_cp_a_memhl() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::CpAMemHl;
        cpu.registers.write_8(Register8::A, 0x34);
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0x32);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_cp_a_memhl_zero() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::CpAMemHl;
        cpu.registers.write_8(Register8::A, 0x0);
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0x0);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 1);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_cp_a_memhl_half_carry() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::CpAMemHl;
        cpu.registers.write_8(Register8::A, 0x10);
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0x01);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_cp_a_memhl_carry() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::CpAMemHl;
        cpu.registers.write_8(Register8::A, 0x00);
        cpu.registers.write_16(Register16::HL, 0x1234);
        memory.write_byte(0x1234, 0x01);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 1);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(cpu.registers.read_flag(Flag::C), 1);
    }

    #[test]
    fn test_add_a_imm8() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::AddAImm8(0x12);
        cpu.registers.write_8(Register8::A, 0x0);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x12);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_add_a_imm8_zero() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::AddAImm8(0x0);
        cpu.registers.write_8(Register8::A, 0x0);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x0);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 1);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_add_a_imm8_half_carry() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::AddAImm8(0x0F);
        cpu.registers.write_8(Register8::A, 0x0F);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x1E);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_add_a_imm8_carry() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::AddAImm8(0x01);
        cpu.registers.write_8(Register8::A, 0xFF);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x0);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 1);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(cpu.registers.read_flag(Flag::C), 1);
    }

    #[test]
    fn test_adc_a_imm8() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::AdcAImm8(0x12);
        cpu.registers.write_8(Register8::A, 0x1);
        cpu.registers.write_flag(Flag::C, 1);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x14);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_adc_a_imm8_zero() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::AdcAImm8(0x0);
        cpu.registers.write_8(Register8::A, 0x0);
        cpu.registers.write_flag(Flag::C, 0);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x0);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 1);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 0);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_adc_a_imm8_half_carry() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::AdcAImm8(0x0E);
        cpu.registers.write_8(Register8::A, 0x01);
        cpu.registers.write_flag(Flag::C, 1);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x10);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(cpu.registers.read_flag(Flag::C), 0);
    }

    #[test]
    fn test_adc_a_imm8_carry() {
        let mut cpu = Cpu::new();
        let mut memory = Memory::new();
        let instruction = Instruction::AdcAImm8(0x01);
        cpu.registers.write_8(Register8::A, 0xFF);
        cpu.registers.write_flag(Flag::C, 1);

        let cycles = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cycles, 2);
        assert_eq!(cpu.registers.read_8(Register8::A), 0x01);
        assert_eq!(cpu.registers.read_flag(Flag::Z), 0);
        assert_eq!(cpu.registers.read_flag(Flag::N), 0);
        assert_eq!(cpu.registers.read_flag(Flag::H), 1);
        assert_eq!(cpu.registers.read_flag(Flag::C), 1);
    }
}
