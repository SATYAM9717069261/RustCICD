pub fn and(num1: u8, num2: u8) -> u8 {
    match (num1, num2) {
        (1, 1) => 1,
        _ => 0,
    }
}
pub fn or(num1: u8, num2: u8) -> u8 {
    match (num1, num2) {
        (0, 0) => 0,
        _ => 1,
    }
}
pub fn xor(num1: u8, num2: u8) -> u8 {
    match (num1, num2) {
        (1, 0) | (0, 1) => 1,
        _ => 0,
    }
}

#[cfg(test)]
mod test {
    use crate::{and, or, xor};
    #[test]
    fn test_and() {
        assert_eq!(1, and(1, 1));
        assert_eq!(0, and(1, 0));
        assert_eq!(0, and(0, 1));
    }

    #[test]
    fn test_or() {
        assert_eq!(1, or(1, 1));
        assert_eq!(1, or(1, 0));
        assert_eq!(1, or(0, 1));
    }
    #[test]
    fn test_xor() {
        assert_eq!(0, xor(1, 1));
        assert_eq!(1, xor(1, 0));
        assert_eq!(1, xor(0, 1));
    }
}
