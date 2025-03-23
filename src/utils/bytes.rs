pub fn get_hi(n: u16) -> u8 {
    (n >> 8) as u8
}

pub fn get_lo(n: u16) -> u8 {
    n as u8
}

pub fn set_hi(n: &mut u16, hi: u8) {
   *n = (*n & 0x00FF) | ((hi as u16) << 8)
}

pub fn set_lo(n: &mut u16, lo: u8) {
    *n = (*n & 0xFF00) | lo as u16
}

/// combine two u8s into a u16
pub fn combine(hi: u8, lo: u8) -> u16 {
    ((hi as u16) << 8) | lo as u16
}

pub fn split(n: u16) -> (u8, u8) {
    (get_hi(n), get_lo(n))
}

/// indexed from right to left
pub fn get_bit(n: u16, index: u8) -> u8 {
    assert!(index < 16);

    (n >> index) as u8 & 1
}

/// Indexed from right to left
/// set the bit at the given index
pub fn set_bit(n: &mut u16, index: u8, value: u8) {
    assert!(index < 16);
    assert!(value == 0 || value == 1);

    if value == 1 {
        *n |= 1 << index;
    } else {
        *n &= !(1 << index);
    }
}

pub fn half_carry_u8_add(left: u8, right: u8) -> u8 {
    if (left & 0xF) + (right & 0xF) > 0xF {
        1
    } else {
        0
    }
}

pub fn carry_u8_add(left: u8, right: u8) -> u8 {
    if (left as u16) + (right as u16) > 0xFF {
        1
    } else {
        0
    }
}

pub fn carry_u16_add(left: u16, right: u16) -> u16 {
    if (left as u32) + (right as u32) > 0xFFFF {
        1
    } else {
        0
    }
}

pub fn half_carry_u16_add(left: u16, right: u16) -> u16 {
    if (left & 0xFFF) + (right & 0xFFF) > 0xFFF {
        1
    } else {
        0
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_lo() {
        assert_eq!(get_lo(0xABCD), 0xCD);
    }

    #[test]
    fn test_get_hi() {
        assert_eq!(get_hi(0xABCD), 0xAB);
    }

    #[test]
    fn test_set_lo() {
        let mut n = 0xABCD;
        set_lo(&mut n, 0xEF);
        assert_eq!(n, 0xABEF);
    }

    #[test]
    fn test_set_hi() {
        let mut n = 0xABCD;
        set_hi(&mut n, 0xEF);
        assert_eq!(n, 0xEFCD);
    }

    #[test]
    fn test_get_bit() {
        assert_eq!(get_bit(0b1010, 0), 0);
        assert_eq!(get_bit(0b1010, 1), 1);
        assert_eq!(get_bit(0b1010, 2), 0);
        assert_eq!(get_bit(0b1010, 3), 1);
    }

    #[test]
    fn test_set_bit() {
        let mut n = 0b1010;
        set_bit(&mut n, 0, 1);
        assert_eq!(n, 0b1011);
        set_bit(&mut n, 0, 0);
        assert_eq!(n, 0b1010);
    }

    #[test]
    fn test_combine() {
        assert_eq!(combine(0xAB, 0xCD), 0xABCD);
    }

    #[test]
    fn test_split() {
        assert_eq!(split(0xABCD), (0xAB, 0xCD));
    }

    #[test]
    fn test_half_carry_u8_add() {
        assert_eq!(half_carry_u8_add(0x0F, 0x01), 1);
        assert_eq!(half_carry_u8_add(0x0F, 0x0F), 1);
        assert_eq!(half_carry_u8_add(0x0F, 0x00), 0);
    }

    #[test]
    fn test_carry_u8_add() {
        assert_eq!(carry_u8_add(0xFF, 0x01), 1);
        assert_eq!(carry_u8_add(0xFF, 0x00), 0);
    }

    #[test]
    fn test_carry_u16_add() {
        assert_eq!(carry_u16_add(0xFFFF, 0x0001), 1);
        assert_eq!(carry_u16_add(0xFFFF, 0x0000), 0);
    }

    #[test]
    fn test_half_carry_u16_add() {
        assert_eq!(half_carry_u16_add(0xFFF0, 0x0010), 1);
        assert_eq!(half_carry_u16_add(0xFFF0, 0x0000), 0);
    }
}