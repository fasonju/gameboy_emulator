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

pub enum R16STK {
    BC,
    DE,
    HL,
    AF,
}

pub enum R16MEM {
    BC,
    DE,
    HLI,
    HLD,
}

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

pub enum Condition {
    Zero,
    NotZero,
    Carry,
    NotCarry,
}


