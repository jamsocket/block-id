pub fn add_mod(lhs: u8, rhs: u8, base: u8) -> u8 {
    let mut result = lhs.wrapping_add(rhs);

    if result < lhs {
        result = result.wrapping_sub(base);
    }

    result % base
}

pub fn sub_mod(lhs: u8, rhs: u8, base: u8) -> u8 {
    let mut result = lhs.wrapping_sub(rhs);

    if result > lhs {
        result = result.wrapping_add(base);
    }

    result % base
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_add_mod() {
        assert_eq!(1, add_mod(4, 5, 8));

        assert_eq!(4, add_mod(200, 58, 254));
        assert_eq!(5, add_mod(200, 58, 253));

        assert_eq!(8, add_mod(200, 58, 250));

        assert_eq!(222, add_mod(233, 233, 244));
    }

    #[test]
    fn test_sub_mod() {
        assert_eq!(1, sub_mod(6, 5, 10));
        assert_eq!(9, sub_mod(5, 6, 10));
    }
}
