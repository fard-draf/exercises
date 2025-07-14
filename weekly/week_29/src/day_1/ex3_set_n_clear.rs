pub fn set_bit(data: &mut u8, position: u8) {
    assert!(position < 8, "Position out of bounds!");
    let mask = 1 << position;

    *data |= mask;
}

pub fn clear_bit(data: &mut u8, position: u8) {
    assert!(position < 8, "Position out of bounds!");
    let mask = 1 << position;

    *data &= !mask;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_bit() {
        let mut byte = 0b0000_0000;
        set_bit(&mut byte, 3);
        assert_eq!(byte, 0b0000_1000);
        set_bit(&mut byte, 7);
        assert_eq!(byte, 0b1000_1000);
        // Tenter de set un bit déjà à 1 ne doit rien changer
        set_bit(&mut byte, 7);
        assert_eq!(byte, 0b1000_1000);
    }

    #[test]
    fn test_clear_bit() {
        let mut byte = 0b1111_1111;
        clear_bit(&mut byte, 2);
        assert_eq!(byte, 0b1111_1011);
        clear_bit(&mut byte, 5);
        assert_eq!(byte, 0b1101_1011);
        // Tenter de clear un bit déjà à 0 ne doit rien changer
        clear_bit(&mut byte, 5);
        assert_eq!(byte, 0b1101_1011);
    }

    #[test]
    fn test_set_and_clear() {
        let mut byte = 0b1010_0101;
        set_bit(&mut byte, 1); // 0 -> 1
        assert_eq!(byte, 0b1010_0111);
        clear_bit(&mut byte, 7); // 1 -> 0
        assert_eq!(byte, 0b0010_0111);
    }
}