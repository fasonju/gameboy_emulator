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

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Cond {
    Zero,
    NotZero,
    Carry,
    NotCarry,
}

impl From<u8> for Cond {
    fn from(c: u8) -> Cond {
        match c {
            0 => Cond::NotZero,
            1 => Cond::Zero,
            2 => Cond::NotCarry,
            3 => Cond::Carry,
            _ => panic!("Invalid condition")
        }
    }
}


