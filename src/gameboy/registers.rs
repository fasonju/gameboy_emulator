#[derive(Debug, Copy, Clone)]

/// Registers module
pub struct Registers {
    pub af: u16, // f is flags
    pub bc: u16,
    pub de: u16,
    pub hl: u16,
    pub sp: u16,
    pub pc: u16
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
}

