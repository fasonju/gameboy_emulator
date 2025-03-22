use crate::utils::{get_bit, get_hi, get_lo, set_bit, set_hi, set_lo};


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

    pub fn get_af(&self) -> u16 {
        self.af
    }

    pub fn get_bc(&self) -> u16 {
        self.bc
    }

    pub fn get_de(&self) -> u16 {
        self.de
    }

    pub fn get_hl(&self) -> u16 {
        self.hl
    }

    pub fn get_sp(&self) -> u16 {
        self.sp
    }

    pub fn get_pc(&self) -> u16 {
        self.pc
    }

    pub fn set_af(&mut self, value: u16) {
        self.af = value;
    }

    pub fn set_bc(&mut self, value: u16) {
        self.bc = value;
    }

    pub fn set_de(&mut self, value: u16) {
        self.de = value;
    }

    pub fn set_hl(&mut self, value: u16) {
        self.hl = value;
    }

    pub fn set_sp(&mut self, value: u16) {
        self.sp = value;
    }

    pub fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }

    // Registers for one byte at a time
    pub fn get_a(&self) -> u8 {
        get_hi(self.af)
    }   

    pub fn get_flags(&self) -> u8 {
        get_lo(self.af)
    }

    pub fn get_zero_flag(&self) -> u8 {
        get_bit(self.af, 7)
    }

    pub fn get_substraction_flag(&self) -> u8 {
        get_bit(self.af, 6)
    }

    pub fn get_half_carry_flag(&self) -> u8 {
        get_bit(self.af, 5)
    }

    pub fn get_carry_flag(&self) -> u8 {
        get_bit(self.af, 4)
    }

    pub fn get_b(&self) -> u8 {
        get_hi(self.bc)
    }

    pub fn get_c(&self) -> u8 {
        get_lo(self.bc)
    }

    pub fn get_d(&self) -> u8 {
        get_hi(self.de)
    }

    pub fn get_e(&self) -> u8 {
        get_lo(self.de)
    }

    pub fn get_h(&self) -> u8 {
        get_hi(self.hl)
    }

    pub fn get_l(&self) -> u8 {
        get_lo(self.hl)
    }

    pub fn set_a(&mut self, value: u8) {
        set_hi(&mut self.af, value);
    }

    pub fn set_flags(&mut self, value: u8) {
        set_lo(&mut self.af, value);
    }

    pub fn set_zero_flag(&mut self, value: u8) {
        set_bit(&mut self.af, 7, value);
    }

    pub fn set_substraction_flag(&mut self, value: u8) {
        set_bit(&mut self.af, 6, value);
    }

    pub fn set_half_carry_flag(&mut self, value: u8) {
        set_bit(&mut self.af, 5, value);
    }

    pub fn set_carry_flag(&mut self, value: u8) {
        set_bit(&mut self.af, 4, value);
    }
    
    pub fn set_b(&mut self, value: u8) {
        set_hi(&mut self.bc, value);
    }

    pub fn set_c(&mut self, value: u8) {
        set_lo(&mut self.bc, value);
    }

    pub fn set_d(&mut self, value: u8) {
        set_hi(&mut self.de, value);
    }

    pub fn set_e(&mut self, value: u8) {
        set_lo(&mut self.de, value);
    }

    pub fn set_h(&mut self, value: u8) {
        set_hi(&mut self.hl, value);
    }

    pub fn set_l(&mut self, value: u8) {
        set_lo(&mut self.hl, value);
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
        assert_eq!(registers.get_af(), af);
        assert_eq!(registers.get_bc(), bc);
        assert_eq!(registers.get_de(), de);
        assert_eq!(registers.get_hl(), hl);
        assert_eq!(registers.get_sp(), sp);
        assert_eq!(registers.get_pc(), pc);
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
        assert_eq!(registers.get_a(), get_hi(af));
        assert_eq!(registers.get_flags(), get_lo(af));
        assert_eq!(registers.get_b(), get_hi(bc));
        assert_eq!(registers.get_c(), get_lo(bc));
        assert_eq!(registers.get_d(), get_hi(de));
        assert_eq!(registers.get_e(), get_lo(de));
        assert_eq!(registers.get_h(), get_hi(hl));
        assert_eq!(registers.get_l(), get_lo(hl));
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

        registers.set_af(af);
        registers.set_bc(bc);
        registers.set_de(de);
        registers.set_hl(hl);
        registers.set_sp(sp);
        registers.set_pc(pc);

        assert_eq!(registers.get_af(), af);
        assert_eq!(registers.get_bc(), bc);
        assert_eq!(registers.get_de(), de);
        assert_eq!(registers.get_hl(), hl);
        assert_eq!(registers.get_sp(), sp);
        assert_eq!(registers.get_pc(), pc);
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

        registers.set_a(a);
        registers.set_flags(f);
        registers.set_b(b);
        registers.set_c(c);
        registers.set_d(d);
        registers.set_e(e);
        registers.set_h(h);
        registers.set_l(l);

        assert_eq!(registers.get_a(), a);
        assert_eq!(registers.get_flags(), f);
        assert_eq!(registers.get_b(), b);
        assert_eq!(registers.get_c(), c);
        assert_eq!(registers.get_d(), d);
        assert_eq!(registers.get_e(), e);
        assert_eq!(registers.get_h(), h);
        assert_eq!(registers.get_l(), l);
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

        assert_eq!(registers.get_zero_flag(), 1);
        assert_eq!(registers.get_substraction_flag(), 0);
        assert_eq!(registers.get_half_carry_flag(), 1);
        assert_eq!(registers.get_carry_flag(), 0);
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

        registers.set_zero_flag(1);
        registers.set_substraction_flag(0);
        registers.set_half_carry_flag(1);
        registers.set_carry_flag(0);

        assert_eq!(registers.get_zero_flag(), 1);
        assert_eq!(registers.get_substraction_flag(), 0);
        assert_eq!(registers.get_half_carry_flag(), 1);
        assert_eq!(registers.get_carry_flag(), 0);
    }
}



