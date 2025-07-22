pub fn toggle_bit(data: &mut u8, position: u8){
    // La position doit être entre 0 et 7.
    assert!(position < 8, "Position out of bounds!");

    let mask = 1 << position;

    *data ^= mask;
    println!("data: {:08b}", data);


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_toggle_from_zero_to_one() {
        let mut byte = 0b0000_0000;
        toggle_bit(&mut byte, 0);
        assert_eq!(byte, 0b0000_0001);
        toggle_bit(&mut byte, 7);
        assert_eq!(byte, 0b1000_0001);
        toggle_bit(&mut byte, 3);
        assert_eq!(byte, 0b1000_1001);
    }

    #[test]
    fn test_toggle_from_one_to_zero() {
        let mut byte = 0b1111_1111;
        toggle_bit(&mut byte, 1);
        assert_eq!(byte, 0b1111_1101);
        toggle_bit(&mut byte, 6);
        assert_eq!(byte, 0b1011_1101);
    }

    #[test]
    fn test_toggle_back_and_forth() {
        let mut byte = 0b1010_0101;
        toggle_bit(&mut byte, 2); // 0 -> 1
        assert_eq!(byte, 0b1010_0001 );
        toggle_bit(&mut byte, 2); // 1 -> 0
        assert_eq!(byte, 0b1010_0101);
    }

    #[test]
    #[should_panic]
    fn test_panic_on_out_of_bounds() {
        let mut byte = 0;
        toggle_bit(&mut byte, 8);
    }
}