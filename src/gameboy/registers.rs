use crate::utils::{get_bit, get_hi, get_lo, set_hi, set_lo};


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
    // Registers for two bytes at once

    fn get_af(&self) -> u16 {
        self.af
    }

    fn get_bc(&self) -> u16 {
        self.bc
    }

    fn get_de(&self) -> u16 {
        self.de
    }

    fn get_hl(&self) -> u16 {
        self.hl
    }

    fn get_sp(&self) -> u16 {
        self.sp
    }

    fn get_pc(&self) -> u16 {
        self.pc
    }

    fn set_af(&mut self, value: u16) {
        self.af = value;
    }

    fn set_bc(&mut self, value: u16) {
        self.bc = value;
    }

    fn set_de(&mut self, value: u16) {
        self.de = value;
    }

    fn set_hl(&mut self, value: u16) {
        self.hl = value;
    }

    fn set_sp(&mut self, value: u16) {
        self.sp = value;
    }

    fn set_pc(&mut self, value: u16) {
        self.pc = value;
    }

    // Registers for one byte at a time
    fn get_a(&self) -> u8 {
        get_hi(self.af)
    }   

    fn get_flags(&self) -> u8 {
        get_lo(self.af)
    }

    fn get_zero_flag(&self) -> u8 {
        get_bit(self.af, 7)
    }

    fn get_substraction_flag(&self) -> u8 {
        get_bit(self.af, 6)
    }

    fn get_half_carry_flag(&self) -> u8 {
        get_bit(self.af, 5)
    }

    fn get_carry_flag(&self) -> u8 {
        get_bit(self.af, 4)
    }

    fn get_b(&self) -> u8 {
        get_hi(self.bc)
    }

    fn get_c(&self) -> u8 {
        get_lo(self.bc)
    }

    fn get_d(&self) -> u8 {
        get_hi(self.de)
    }

    fn get_e(&self) -> u8 {
        get_lo(self.de)
    }

    fn get_h(&self) -> u8 {
        get_hi(self.hl)
    }

    fn get_l(&self) -> u8 {
        get_lo(self.hl)
    }

    fn set_a(&mut self, value: u8) {
        set_hi(&mut self.af, value);
    }

    fn set_flags(&mut self, value: u8) {
        set_lo(&mut self.af, value);
    }

    fn set_b(&mut self, value: u8) {
        set_hi(&mut self.bc, value);
    }

    fn set_c(&mut self, value: u8) {
        set_lo(&mut self.bc, value);
    }

    fn set_d(&mut self, value: u8) {
        set_hi(&mut self.de, value);
    }

    fn set_e(&mut self, value: u8) {
        set_lo(&mut self.de, value);
    }

    fn set_h(&mut self, value: u8) {
        set_hi(&mut self.hl, value);
    }

    fn set_l(&mut self, value: u8) {
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
}



