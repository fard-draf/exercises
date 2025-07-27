use core::panic;
use std::alloc::{alloc, dealloc, handle_alloc_error};

#[derive(Debug)]
pub struct RawPointer<T> {
    pub ptr: *mut T,
    pub cap: usize,
}

impl<T> RawPointer<T> {
    fn new() -> Self {
        Self {
            ptr: core::ptr::dangling_mut(),
            cap: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        if capacity == 0 {
            Self {
                ptr: core::ptr::dangling_mut(),
                cap: 0,
            }
        } else {
            let layout = match std::alloc::Layout::array::<T>(capacity) {
                Ok(l) => l,
                Err(_) => panic!("Unvalid Layout"),
            };

            let ptr = unsafe { alloc(layout) };

            if ptr.is_null() {
                handle_alloc_error(layout)
            }

            Self {
                ptr: ptr.cast::<T>(),
                cap: capacity,
            }
        }
    }
}

impl<T> Drop for RawPointer<T> {
    fn drop(&mut self) {
        let layout = match std::alloc::Layout::array::<T>(self.cap) {
            Ok(l) => l,
            Err(_) => panic!("Unvalid Layout"),
        };

        unsafe { dealloc(self.ptr as *mut u8, layout) };
    }
}
