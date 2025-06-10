use crate::explain_copy_ref::{demonstrate_copy_semantics, explain_copy};
use crate::pointer1::function_pointer1;

mod explain_copy_ref;
mod pointer1;
fn main() {
    let vec = vec![1, 2, 3, 4];
    let ptr = &vec[1] as *const i32;
    let ptr2 = &vec[2];
    let ptr3 = &vec[3];
    println!("{:?}, {:p}, {:p}", ptr, ptr2, ptr3);

    let transaction_data = vec![
        0x01, 0x02, 0x03, 0x04, // Version (u32)
        0x10, 0x20, 0x30, 0x40, // Amount (u32)
        0xFF, 0xEE, 0xDD, 0xCC, // Fee (u32)
    ];

    // Raw pointers vers chaque champ
    let version_ptr = transaction_data.as_ptr() as *const u32;
    let amount_ptr = unsafe { version_ptr.offset(1) };
    let fee_ptr = unsafe { version_ptr.offset(2) };

    unsafe {
        let version = u32::from_le_bytes([
            *transaction_data.get_unchecked(0),
            *transaction_data.get_unchecked(1),
            *transaction_data.get_unchecked(2),
            *transaction_data.get_unchecked(3),
        ]);

        println!("   Transaction version: 0x{:08x}", version);
        println!("   Raw version via ptr: 0x{:08x}", (*version_ptr).to_le());
        println!("   Raw amount via ptr: 0x{:08x}", (*amount_ptr).to_le());
        println!("   Raw fee via ptr: 0x{:08x}", (*fee_ptr).to_le());
    }
}
