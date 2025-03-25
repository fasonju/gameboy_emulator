/// This module contains the enums for the different variables used in the instructions.
/// They are not the same as the ones in the registers module, as they are used to represent
/// the different variables in the instructions, not the registers themselves.

/// The R8 enum is used to represent the 8-bit registers in the instructions.
/// 
/// Note: MemHl is used to represent the memory at the address in HL.
#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum R8 {
    B,
    C,
    D,
    E,
    H,
    L,
    MemHl, // Memory at HL
    A,
}

impl From<u8> for R8 {
    fn from(r: u8) -> R8 {
        match r {
            0 => R8::B,
            1 => R8::C,
            2 => R8::D,
            3 => R8::E,
            4 => R8::H,
            5 => R8::L,
            6 => R8::MemHl,
            7 => R8::A,
            _ => panic!("Invalid R8 register")
        }
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum R16 {
    BC,
    DE,
    HL,
    SP, // Stack Pointer
}

impl From<u8> for R16 {
    fn from(r: u8) -> R16 {
        match r {
            0 => R16::BC,
            1 => R16::DE,
            2 => R16::HL,
            3 => R16::SP,
            _ => panic!("Invalid R16 register")
        }
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum R16STK {
    BC,
    DE,
    HL,
    AF,
}

impl From<u8> for R16STK {
    fn from(r: u8) -> R16STK {
        match r {
            0 => R16STK::BC,
            1 => R16STK::DE,
            2 => R16STK::HL,
            3 => R16STK::AF,
            _ => panic!("Invalid R16STK register")
        }
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum R16MEM {
    BC,
    DE,
    HLI,
    HLD,
}

impl From<u8> for R16MEM {
    fn from(r: u8) -> R16MEM {
        match r {
            0 => R16MEM::BC,
            1 => R16MEM::DE,
            2 => R16MEM::HLI,
            3 => R16MEM::HLD,
            _ => panic!("Invalid R16MEM register")
        }
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum B3 {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
}

impl From<u8> for B3 {
    fn from(b: u8) -> B3 {
        match b {
            0 => B3::Zero,
            1 => B3::One,
            2 => B3::Two,
            3 => B3::Three,
            4 => B3::Four,
            5 => B3::Five,
            6 => B3::Six,
            7 => B3::Seven,
            _ => panic!("Invalid B3")
        }
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum COND {
    Zero,
    NotZero,
    Carry,
    NotCarry,
}

impl From<u8> for COND {
    fn from(c: u8) -> COND {
        match c {
            0 => COND::NotZero,
            1 => COND::Zero,
            2 => COND::NotCarry,
            3 => COND::Carry,
            _ => panic!("Invalid condition")
        }
    }
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum TGT3 {
    ZERO = 0x0,
    ONE = 0x8,
    TWO = 0x10,
    THREE = 0x18,
    FOUR = 0x20,
    FIVE = 0x28,
    SIX = 0x30,
    SEVEN = 0x38,
}

impl From<u8> for TGT3 {
    fn from(t: u8) -> TGT3 {
        match t {
            0x0 => TGT3::ZERO,
            0x1 => TGT3::ONE,
            0x2 => TGT3::TWO,
            0x3 => TGT3::THREE,
            0x4 => TGT3::FOUR,
            0x5 => TGT3::FIVE,
            0x6 => TGT3::SIX,
            0x7 => TGT3::SEVEN,
            _ => panic!("Invalid TGT3")
        }
    }
}
