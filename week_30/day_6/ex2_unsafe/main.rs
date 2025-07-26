use std::alloc::{self, alloc};

fn main() {
    {
        let mut ptr: RawVec<u32> = RawVec::with_capacity(10);
        dbg!(ptr);
        drop(ptr);
    }
}

#[derive(Debug)]
struct RawVec<T> {
    ptr: *mut T,
    cap: usize,
}

impl<T> RawVec<T> {
    fn new() -> Self {
        Self {
            ptr: core::ptr::dangling_mut(),
            cap: 0,
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        if capacity == 0 {
            Self {
                ptr: core::ptr::dangling_mut(),
                cap: 0,
            }
        } else {
            let layout = match alloc::Layout::array::<T>(capacity) {
                Ok(l) => l,
                Err(_) => panic!("Allocation trop grande"),
            };

            let ptr = unsafe { alloc(layout) };
            let ptr = ptr.cast::<T>();
            if ptr.is_null() {
                alloc::handle_alloc_error(layout)
            }

            Self { ptr, cap: capacity }
        }
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        if self.cap > 0 {
            let layout = match alloc::Layout::array::<T>(self.cap) {
                Ok(l) => l,
                Err(_) => panic!("Allocation trop grande"),
            };

            unsafe {
                std::alloc::dealloc(self.ptr as *mut u8, layout);
            }
        }
    }
}
