/// Sets the bit at `position` to 1.
fn set_bit(data: &mut u64, position: u8) {
    *data |= 1 << position;  
    println!("data: {:0b}", data);
}

/// Clears the bit at `position` to 0.
fn clear_bit(data: &mut u64, position: u8) {
    *data &= !(1 << position);
}

/// Toggles the bit at `position`.
fn toggle_bit(data: &mut u64, position: u8) {
    *data ^= 1 << position;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_bit() {
        let mut data = 0b1010_u64;
        set_bit(&mut data, 0); // Set a bit that is 0
        assert_eq!(data, 0b1011);
        set_bit(&mut data, 1); // Set a bit that is already 1
        assert_eq!(data, 0b1011);
    }

    #[test]
    fn test_clear_bit() {
        let mut data = 0b1010_u64;
        clear_bit(&mut data, 1); // Clear a bit that is 1
        assert_eq!(data, 0b1000);
        clear_bit(&mut data, 2); // Clear a bit that is already 0
        assert_eq!(data, 0b1000);
    }

    #[test]
    fn test_toggle_bit() {
        let mut data = 0b1010_u64;
        toggle_bit(&mut data, 0); // Toggle a 0 to a 1
        assert_eq!(data, 0b1011);
        toggle_bit(&mut data, 1); // Toggle a 1 to a 0
        assert_eq!(data, 0b1001);
    }

    #[test]
    fn test_toggle_idempotency() {
        let original_data = 0b1010_u64;
        let mut data = original_data;
        toggle_bit(&mut data, 2); // Toggle
        assert_ne!(data, original_data);
        toggle_bit(&mut data, 2); // Toggle back
        assert_eq!(data, original_data);
    }

    #[test]
    #[should_panic]
    fn test_set_bit_out_of_bounds() {
        let mut data = 0;
        set_bit(&mut data, 64);
    }
}