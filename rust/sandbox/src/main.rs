fn main() {
    let id: u8 = 52; // 8bit
    let value: u8 = 253; // 8 bits
    let owner: u16 = 5124; // 13 bits 
    // total of 29 bits -> u32 is ok

    let mut storage = 0u32;

    //let clean storage
    storage &= !(0xFFFF << 0);
    storage |= (id as u32) << 0;
    storage |= ((value as u32) & 0xFF) << 8;
    storage |= ((owner as u32) & 0x1FFF) << 16;
    // storage &= !(0xFFFF << 0);

    println!("{:b}", storage);

    let extracted_id = (storage & 0xFF) as u8;
    println!("{}", extracted_id);

    let extracted_value = ((storage >> 8) & 0xFF) as u8;

    println!("{}", extracted_value);

    let extracted_owner = ((storage >> 16) & 0x1FFF) as u16;
    println!("{}", extracted_owner)
}
