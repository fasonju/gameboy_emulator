/// This module contains the Registers struct which holds the values of the CPU registers.
/// It also contains the Register16, Register8 and Flag enums which are used to represent the different registers and flags.
/// The Registers struct has methods to read and write the values of the registers and flags.
/// The Register16 and Register8 enums have methods to convert the instruction variables to the corresponding register.
use crate::utils::{get_bit_u16, get_hi, get_lo, set_bit_u16, set_hi, set_lo};

use super::instruction_variables::{R16, R16MEM, R8};

#[derive(Debug, Copy, Clone)]
pub enum Register16 {
    AF,
    BC,
    DE,
    HL,
    SP,
    PC,
}

impl From<R16> for Register16 {
    fn from(register: R16) -> Self {
        match register {
            R16::BC => Register16::BC,
            R16::DE => Register16::DE,
            R16::HL => Register16::HL,
            R16::SP => Register16::SP,
        }
    }
}

impl From<R16MEM> for Register16 {
    fn from(register: R16MEM) -> Self {
        match register {
            R16MEM::BC => Register16::BC,
            R16MEM::DE => Register16::DE,
            R16MEM::HLI => Register16::HL,
            R16MEM::HLD => Register16::HL,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Register8 {
    A,
    F,
    B,
    C,
    D,
    E,
    H,
    L,
}

impl From<R8> for Register8 {
    fn from(register: R8) -> Register8 {
        match register {
            R8::A => Register8::A,
            R8::B => Register8::B,
            R8::C => Register8::C,
            R8::D => Register8::D,
            R8::E => Register8::E,
            R8::H => Register8::H,
            R8::L => Register8::L,
            _ => panic!("Invalid R8 register"),
        }
    }
}

pub enum Flag {
    Z,
    N,
    H,
    C,
}

pub struct Registers {
    af: u16, // f is flags
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pub pc: u16,
}

impl Registers {
    /// Create a new Registers object
    pub fn new(af: u16, bc: u16, de: u16, hl: u16, sp: u16, pc: u16) -> Registers {
        Registers {
            af,
            bc,
            de,
            hl,
            sp,
            pc,
        }
    }

    pub fn read_16(&self, register: Register16) -> u16 {
        match register {
            Register16::AF => self.af,
            Register16::BC => self.bc,
            Register16::DE => self.de,
            Register16::HL => self.hl,
            Register16::SP => self.sp,
            Register16::PC => self.pc,
        }
    }

    pub fn write_16(&mut self, register: Register16, value: u16) {
        match register {
            Register16::AF => self.af = value,
            Register16::BC => self.bc = value,
            Register16::DE => self.de = value,
            Register16::HL => self.hl = value,
            Register16::SP => self.sp = value,
            Register16::PC => self.pc = value,
        }
    }

    pub fn read_8(&self, register: Register8) -> u8 {
        match register {
            Register8::A => get_hi(self.af),
            Register8::F => get_lo(self.af),
            Register8::B => get_hi(self.bc),
            Register8::C => get_lo(self.bc),
            Register8::D => get_hi(self.de),
            Register8::E => get_lo(self.de),
            Register8::H => get_hi(self.hl),
            Register8::L => get_lo(self.hl),
        }
    }

    pub fn write_8(&mut self, register: Register8, value: u8) {
        match register {
            Register8::A => set_hi(&mut self.af, value),
            Register8::F => set_lo(&mut self.af, value),
            Register8::B => set_hi(&mut self.bc, value),
            Register8::C => set_lo(&mut self.bc, value),
            Register8::D => set_hi(&mut self.de, value),
            Register8::E => set_lo(&mut self.de, value),
            Register8::H => set_hi(&mut self.hl, value),
            Register8::L => set_lo(&mut self.hl, value),
        }
    }

    pub fn read_flag(&self, flag: Flag) -> u8 {
        match flag {
            Flag::Z => get_bit_u16(self.af, 0),
            Flag::N => get_bit_u16(self.af, 1),
            Flag::H => get_bit_u16(self.af, 2),
            Flag::C => get_bit_u16(self.af, 3),
        }
    }

    pub fn write_flag(&mut self, flag: Flag, value: u8) {
        match flag {
            Flag::Z => set_bit_u16(&mut self.af, 0, value),
            Flag::N => set_bit_u16(&mut self.af, 1, value),
            Flag::H => set_bit_u16(&mut self.af, 2, value),
            Flag::C => set_bit_u16(&mut self.af, 3, value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_16() {
        let registers = Registers::new(0x1234, 0x5678, 0x9ABC, 0xDEF0, 0x1357, 0x2468);
        assert_eq!(registers.read_16(Register16::AF), 0x1234);
        assert_eq!(registers.read_16(Register16::BC), 0x5678);
        assert_eq!(registers.read_16(Register16::DE), 0x9ABC);
        assert_eq!(registers.read_16(Register16::HL), 0xDEF0);
        assert_eq!(registers.read_16(Register16::SP), 0x1357);
        assert_eq!(registers.read_16(Register16::PC), 0x2468);
    }

    #[test]
    fn test_write_16() {
        let mut registers = Registers::new(0, 0, 0, 0, 0, 0);
        registers.write_16(Register16::AF, 0x1234);
        registers.write_16(Register16::BC, 0x5678);
        registers.write_16(Register16::DE, 0x9ABC);
        registers.write_16(Register16::HL, 0xDEF0);
        registers.write_16(Register16::SP, 0x1357);
        registers.write_16(Register16::PC, 0x2468);
        assert_eq!(registers.af, 0x1234);
        assert_eq!(registers.bc, 0x5678);
        assert_eq!(registers.de, 0x9ABC);
        assert_eq!(registers.hl, 0xDEF0);
        assert_eq!(registers.sp, 0x1357);
        assert_eq!(registers.pc, 0x2468);
    }

    #[test]
    fn test_read_8() {
        let registers = Registers::new(0x1234, 0x5678, 0x9ABC, 0xDEF0, 0x1357, 0x2468);
        assert_eq!(registers.read_8(Register8::A), 0x12);
        assert_eq!(registers.read_8(Register8::F), 0x34);
        assert_eq!(registers.read_8(Register8::B), 0x56);
        assert_eq!(registers.read_8(Register8::C), 0x78);
        assert_eq!(registers.read_8(Register8::D), 0x9A);
        assert_eq!(registers.read_8(Register8::E), 0xBC);
        assert_eq!(registers.read_8(Register8::H), 0xDE);
        assert_eq!(registers.read_8(Register8::L), 0xF0);
    }

    #[test]
    fn test_write_8() {
        let mut registers = Registers::new(0, 0, 0, 0, 0, 0);
        registers.write_8(Register8::A, 0x12);
        registers.write_8(Register8::F, 0x34);
        registers.write_8(Register8::B, 0x56);
        registers.write_8(Register8::C, 0x78);
        registers.write_8(Register8::D, 0x9A);
        registers.write_8(Register8::E, 0xBC);
        registers.write_8(Register8::H, 0xDE);
        registers.write_8(Register8::L, 0xF0);
        assert_eq!(registers.af, 0x1234);
        assert_eq!(registers.bc, 0x5678);
        assert_eq!(registers.de, 0x9ABC);
        assert_eq!(registers.hl, 0xDEF0);
    }

    #[test]
    fn test_read_flag() {
        let registers = Registers::new(0b1010_1010_1010_1010, 0, 0, 0, 0, 0);
        assert_eq!(registers.read_flag(Flag::Z), 0);
        assert_eq!(registers.read_flag(Flag::N), 1);
        assert_eq!(registers.read_flag(Flag::H), 0);
        assert_eq!(registers.read_flag(Flag::C), 1);
    }

    #[test]
    fn test_write_flag() {
        let mut registers = Registers::new(0, 0, 0, 0, 0, 0);
        registers.write_flag(Flag::Z, 1);
        registers.write_flag(Flag::N, 0);
        registers.write_flag(Flag::H, 1);
        registers.write_flag(Flag::C, 0);

        assert_eq!(registers.read_8(Register8::F), 0x5);
    }
}
