use crate::utils::{get_bit, get_hi, get_lo, set_bit, set_hi, set_lo};

#[derive(Debug, Copy, Clone)]
pub enum Register8 {
    A,
    B,
    C,
    D,
    E,
    H,
    L,
    F, // Flags register
}

#[derive(Debug, Copy, Clone)]
pub enum Register16 {
    AF,
    BC,
    DE,
    HL,
    SP, // Stack Pointer
    PC, // Program Counter
}

#[derive(Debug, Copy, Clone)]
pub enum Flag {
    Zero,
    Substraction,
    HalfCarry,
    Carry,
}

/// Registers module
pub struct Registers {
    af: u16, // f is flags
    bc: u16,
    de: u16,
    hl: u16,
    sp: u16,
    pc: u16
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
            pc
        }
    }

    pub fn get_register_16(&self, register: Register16) -> u16 {
        match register {
            Register16::AF => self.af,
            Register16::BC => self.bc,
            Register16::DE => self.de,
            Register16::HL => self.hl,
            Register16::SP => self.sp,
            Register16::PC => self.pc,
        }
    }

    pub fn set_register_16(&mut self, register: Register16, value: u16) {
        match register {
            Register16::AF => self.af = value,
            Register16::BC => self.bc = value,
            Register16::DE => self.de = value,
            Register16::HL => self.hl = value,
            Register16::SP => self.sp = value,
            Register16::PC => self.pc = value,
        }
    }

    pub fn get_register_8(&self, register: Register8) -> u8 {
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

    pub fn set_register_8(&mut self, register: Register8, value: u8) {
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

    pub fn get_flag(&self, flag: Flag) -> u8 {
        match flag {
            Flag::Zero => get_bit(self.get_register_8(Register8::F) as u16, 7),
            Flag::Substraction => get_bit(self.get_register_8(Register8::F) as u16, 6),
            Flag::HalfCarry => get_bit(self.get_register_8(Register8::F) as u16, 5),
            Flag::Carry => get_bit(self.get_register_8(Register8::F) as u16, 4),
        }
    }

    pub fn set_flag(&mut self, flag: Flag, value: u8) {
        match flag {
            Flag::Zero => set_bit(&mut self.af, 7, value),
            Flag::Substraction => set_bit(&mut self.af, 6, value),
            Flag::HalfCarry => set_bit(&mut self.af, 5, value),
            Flag::Carry => set_bit(&mut self.af, 4, value),
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_getters_16() {

        let af = 0xABCD;
        let bc = 0x1234;
        let de = 0x5678;
        let hl = 0x9ABC;
        let sp = 0xDEF0;
        let pc = 0x1234;

        let registers = Registers {
            af,
            bc,
            de,
            hl,
            sp,
            pc
        };
        assert_eq!(registers.get_register_16(Register16::AF), af);
        assert_eq!(registers.get_register_16(Register16::BC), bc);
        assert_eq!(registers.get_register_16(Register16::DE), de);
        assert_eq!(registers.get_register_16(Register16::HL), hl);
        assert_eq!(registers.get_register_16(Register16::SP), sp);
        assert_eq!(registers.get_register_16(Register16::PC), pc);
    }

    #[test]
    fn test_getters_8() {
        let af = 0xABCD;
        let bc = 0x1234;
        let de = 0x5678;
        let hl = 0x9ABC;
        let sp = 0xDEF0;
        let pc = 0x1234;

        let registers = Registers {
            af,
            bc,
            de,
            hl,
            sp,
            pc
        };
        assert_eq!(registers.get_register_8(Register8::A), 0xAB);
        assert_eq!(registers.get_register_8(Register8::F), 0xCD);
        assert_eq!(registers.get_register_8(Register8::B), 0x12);
        assert_eq!(registers.get_register_8(Register8::C), 0x34);
        assert_eq!(registers.get_register_8(Register8::D), 0x56);
        assert_eq!(registers.get_register_8(Register8::E), 0x78);
        assert_eq!(registers.get_register_8(Register8::H), 0x9A);
        assert_eq!(registers.get_register_8(Register8::L), 0xBC);
    }

    #[test]
    fn test_setters_16() {
        let mut registers = Registers {
            af: 0,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0,
            pc: 0
        };

        let af = 0xABCD;
        let bc = 0x1234;
        let de = 0x5678;
        let hl = 0x9ABC;
        let sp = 0xDEF0;
        let pc = 0x1234;

        registers.set_register_16(Register16::AF, af);
        registers.set_register_16(Register16::BC, bc);
        registers.set_register_16(Register16::DE, de);
        registers.set_register_16(Register16::HL, hl);
        registers.set_register_16(Register16::SP, sp);
        registers.set_register_16(Register16::PC, pc);
    }

    #[test]
    fn test_setters_8() {
        let mut registers = Registers {
            af: 0,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0,
            pc: 0
        };

        let a = 0xAB;
        let f = 0xCD;
        let b = 0x12;
        let c = 0x34;
        let d = 0x56;
        let e = 0x78;
        let h = 0x9A;
        let l = 0xBC;

        
        registers.set_register_8(Register8::A, a);
        registers.set_register_8(Register8::F, f);
        registers.set_register_8(Register8::B, b);
        registers.set_register_8(Register8::C, c);
        registers.set_register_8(Register8::D, d);
        registers.set_register_8(Register8::E, e);
        registers.set_register_8(Register8::H, h);
        registers.set_register_8(Register8::L, l);

        assert_eq!(registers.get_register_8(Register8::A), a);
        assert_eq!(registers.get_register_8(Register8::F), f);
        assert_eq!(registers.get_register_8(Register8::B), b);
        assert_eq!(registers.get_register_8(Register8::C), c);
        assert_eq!(registers.get_register_8(Register8::D), d);
        assert_eq!(registers.get_register_8(Register8::E), e);
        assert_eq!(registers.get_register_8(Register8::H), h);
        assert_eq!(registers.get_register_8(Register8::L), l);
    }

    #[test]
    fn test_getters_flags() {
        let af = 0b1101_1010_1010_1010;
        let registers = Registers {
            af,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0,
            pc: 0
        };

        assert_eq!(registers.get_flag(Flag::Zero), 1);
        assert_eq!(registers.get_flag(Flag::Substraction), 0);
        assert_eq!(registers.get_flag(Flag::HalfCarry), 1);
        assert_eq!(registers.get_flag(Flag::Carry), 0);
    }

    #[test]
    fn test_setters_flags() {
        let mut registers = Registers {
            af: 0,
            bc: 0,
            de: 0,
            hl: 0,
            sp: 0,
            pc: 0
        };

        registers.set_flag(Flag::Zero, 1);
        registers.set_flag(Flag::Substraction, 0);
        registers.set_flag(Flag::HalfCarry, 1);
        registers.set_flag(Flag::Carry, 0);

        assert_eq!(registers.get_flag(Flag::Zero), 1);
        assert_eq!(registers.get_flag(Flag::Substraction), 0);
        assert_eq!(registers.get_flag(Flag::HalfCarry), 1);
        assert_eq!(registers.get_flag(Flag::Carry), 0);
    }
}



