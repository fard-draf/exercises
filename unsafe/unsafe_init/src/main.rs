use std::alloc::{self, Layout};

fn main() {
    explore_layout();
}

fn explore_layout() {
    let layout1 = Layout::from_size_align(8, 1).unwrap();
    let layout2 = Layout::from_size_align(8, 8).unwrap();

    unsafe {
        let ptr: *mut u8 = std::alloc::alloc(layout1);
        *ptr = 0;
        let value = *ptr;
    }
    println!(
        "Lay1 - Size: {}, Align: {}",
        layout1.size(),
        layout1.align()
    );
    println!(
        "Lay2 - Size: {}, Align: {}",
        layout2.size(),
        layout2.align()
    );
}
