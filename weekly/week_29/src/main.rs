mod day_1;
mod day_2;

fn main() {
    bitwise_operations();
}

fn bitwise_operations() {
    let b1 = 0b1010_1010;
    let m1 = 0b1101_0110;
    
    assert_eq!(b1 & m1, 0b1000_0010);
    assert_eq!(b1 | m1, 0b1111_1110);
    assert_eq!(b1 ^ m1, 0b0111_1100);

    let b2 = 0b1101_1101;
    let m2 = 0b0101_1101;

    assert_eq!(b2 & m2, 0b0101_1101);
    assert_eq!(b2 | m2, 0b1101_1101);
    assert_eq!(b2 ^ m2, 0b1000_0000);

    let b3 = 0b0010_1110;
    let m3 = 0b0101_1101;

    assert_eq!(b3 & m3, 0b0000_1100);
    assert_eq!(b3 | m3, 0b0111_1111);
    assert_eq!(b3 ^ m3, 0b0111_0011);

    
}