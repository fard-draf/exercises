fn main() {
    let mut packet: [u32; 4] = [0xAAAAAAAA, 0xBBBBBBBB, 0xCCCCCCCC, 0xDDDDDDDD];
    println!("Paquet initial: {:X?}", packet);

    let ptr1 = packet.as_ptr();
    println!("[PTR1]: {:?}", ptr1);

    for e in 0..packet.len() {
        let val = unsafe { ptr1.offset(e as isize) };
        println!("[VAL{}]: {:?}", e + 1, unsafe { val.read() })
    }

    let ptr2 = unsafe { ptr1.offset(2) as *mut u32 };
    println!("[PTR2]: {:?}", ptr2);
    unsafe { std::ptr::write(ptr2, 0x12345678) };

    let ptr3 = unsafe { ptr1.offset(3) as *mut u32 };
    unsafe {
        std::ptr::write(ptr3, 0xFFFFFFFF);
    }

    println!("[PACKET]: {:X?}", packet);

    assert_eq!(packet, [0xAAAAAAAA, 0xBBBBBBBB, 0x12345678, 0xFFFFFFFF]);
}
