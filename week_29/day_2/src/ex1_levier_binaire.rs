fn get_bit_at(data: u64, position: u8) -> bool {
    if position >= 64 {
        panic!()
    }
    
    (data >> position) & 1 == 1

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_bit_nominal() {
        let data = 0b1010_u64;
        assert_eq!(get_bit_at(data, 0), false);
        assert_eq!(get_bit_at(data, 1), true);
        assert_eq!(get_bit_at(data, 2), false);
        assert_eq!(get_bit_at(data, 3), true);
    }

    #[test]
    fn test_get_bit_edges() {

        let data_lsb = 1_u64; 
        assert_eq!(get_bit_at(data_lsb, 0), true);
        assert_eq!(get_bit_at(data_lsb, 63), false);

        let data_msb = 1_u64 << 63; 
        assert_eq!(get_bit_at(data_msb, 0), false);
        assert_eq!(get_bit_at(data_msb, 63), true);
    }
    
    #[test]
    #[should_panic]
    fn test_get_bit_out_of_bounds() {
        get_bit_at(0, 64);
    }
}