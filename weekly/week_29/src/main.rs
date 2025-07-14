mod day_1;

fn main() {
    let bit_buffer: u64 = 0b10101010;
    println!("bit_buffer vierge: {:08b}", bit_buffer);


    let and = bit_buffer & 0b01010101;
    println!("and: {:08b}", and);

    let  or = bit_buffer | 0b01010101;
    println!("or: {:08b}", or);

    let xor = bit_buffer ^ 0b01010101;
    println!("xor: {:08b}", xor);

    let op_himself = !bit_buffer;
    println!("!: {:08b}", op_himself);

    let and_by_op_himself = bit_buffer & !bit_buffer;
    println!("& and !: {:08b}", and_by_op_himself);

    let or_by_op_himself = bit_buffer | !bit_buffer;
    println!("| and !: {:08b}", or_by_op_himself );

    let xor_by_op_himself = bit_buffer ^ !bit_buffer;
    println!("^ and !: {:08b}", xor_by_op_himself);

}

