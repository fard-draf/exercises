use std::alloc::{dealloc, handle_alloc_error};

fn main() {
    let layout = std::alloc::Layout::new::<i32>();
    let alloc = unsafe { std::alloc::alloc(layout) };

    {
        let data = {
            if alloc.is_null() {
                handle_alloc_error(layout);
            }
            let data = alloc.cast::<i32>();
            unsafe { &mut *data }
        };

        println!("{:p}", data);
    }
    unsafe {
        dealloc(alloc as *mut u8, layout);
    }
}
