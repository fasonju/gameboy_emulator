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
    fn test_combine() {
        assert_eq!(combine(0xAB, 0xCD), 0xABCD);
    }
}