use core::panic;
use std::alloc::{alloc, dealloc, handle_alloc_error};

fn main() {
    #[derive(Debug)]
    struct Arr {
        len: usize,
        ptr: *mut i32,
        cap: usize,
        layout: core::alloc::Layout,
    }

    impl Arr {
        fn with_capacity(capacity: usize) -> Self {
            let layout = match core::alloc::Layout::array::<i32>(capacity) {
                Ok(l) => l,
                Err(_) => panic!("LAYOUT OOM"),
            };
            let ptr = unsafe { alloc(layout) };
            if ptr.is_null() {
                handle_alloc_error(layout)
            };
            let ptr = ptr.cast::<i32>();
            Self {
                ptr,
                cap: capacity,
                len: 0,
                layout,
            }
        }

        fn push(&mut self, val: i32) {
            if self.ptr.is_null() {
                panic!("NULL");
            }
            let new_ptr = unsafe { self.ptr.add(self.len) };
            unsafe { std::ptr::write(new_ptr, val) };
            self.len += 1;
        }
    }

    let mut arr = Arr::with_capacity(1000);

    for e in 0..1000 {
        arr.push(e);
    }
    dbg!(&arr);

    unsafe {
        arr.ptr.with_addr(arr.ptr.add(999) as usize).write(10000);
    }

    for e in 0..1000 {
        println!("{} -> {}", e, unsafe { arr.ptr.add(e).read() })
    }

    for e in 0..arr.len {
        let ptr = unsafe { arr.ptr.add(e) };
        unsafe {
            ptr.drop_in_place();
        }
    }
    unsafe { dealloc(arr.ptr as *mut u8, arr.layout) };

    dbg!(arr);
}
