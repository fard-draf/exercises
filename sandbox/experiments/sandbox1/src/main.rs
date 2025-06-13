fn main() {
    let value = 200;
    let op = get_high_bit(value);
    println!("{:b}, 0x{:2X}, {:b}", value, value, op);
    let input = "bonjour";
    let reverse = reverse(input);
    println!("{}", reverse)
}

// Pattern 1: Masquage du bit de poids faible
fn is_even(n: u8) -> bool {
    (n & 0b1) == 0
}

// Pattern 2: Masquage du bit de poids fort (3 bits)
fn get_high_bit(n: u8) -> u8 {
    (n & 0b10000000) >> 0
    // equivaut a     (n & 0x80) >> 7
}

// Pattern 3: Extraction de bits du milieu
fn get_middle_bit(n: u8) -> u8 {
    (n & 0b010) >> 1
}

pub fn reverse(input: &str) -> String {
    input.chars().rev().fold(String::new(), |mut string, i| {
        string.push(i);
        string
    })
}
