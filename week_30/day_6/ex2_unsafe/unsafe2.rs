use core::panic;
use std::alloc::{self, dealloc, handle_alloc_error};

pub struct RawVec<T> {
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
            let layout = match core::alloc::Layout::array::<T>(capacity) {
                Ok(l) => l,
                Err(_) => panic!("Layout error"),
            };

            let ptr = unsafe { std::alloc::alloc(layout) };
            let ptr = ptr.cast::<T>();
            if ptr.is_null() {
                handle_alloc_error(layout);
            }

            Self { ptr, cap: capacity }
        }
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        if self.cap > 0 {
            let layout = match core::alloc::Layout::array::<T>(self.cap) {
                Ok(l) => l,
                Err(_) => panic!("Layout error"),
            };

            unsafe { dealloc(self.ptr as *mut u8, layout) };
        }
    }
}
