use core::panic;
use std::{
    alloc::{self, alloc},
    fmt::Debug,
};

use unsafe3::RawPointer;
mod unsafe2;
mod unsafe3;
fn main() {
    {
        let mut vec: RawVec<u32> = RawVec::with_capacity(500);
        for i in 0..=499 {
            vec.push(i);
        }
        dbg!(&vec);

        for i in 0..=1000 {
            vec.pop();
        }
        dbg!(vec);
    }
}

#[derive(Debug)]
struct RawVec<T> {
    ptr: *mut T,
    cap: usize,
    len: usize,
}

impl<T> RawVec<T> {
    fn new() -> Self {
        Self {
            ptr: core::ptr::dangling_mut(),
            cap: 0,
            len: 0,
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        if capacity == 0 {
            Self {
                ptr: core::ptr::dangling_mut(),
                cap: 0,
                len: 0,
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

            Self {
                ptr,
                cap: capacity,
                len: 0,
            }
        }
    }

    fn push(&mut self, value: T) {
        if self.cap == self.len {
            panic!("Capacity is too low");
        }

        let new_ptr = unsafe { self.ptr.add(self.len) };

        unsafe { std::ptr::write(new_ptr, value) };
        self.len += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        if self.len > 0 {
            self.len -= 1;
            let addr = unsafe { self.ptr.add(self.len) };
            let value = unsafe { std::ptr::read(addr) };
            Some(value)
        } else {
            None
        }
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        if self.cap > 0 {
            if self.len > 0 {
                for e in 0..self.len {
                    let ptr = unsafe { self.ptr.add(e) };

                    unsafe { std::ptr::drop_in_place(ptr) };
                }
            }
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
