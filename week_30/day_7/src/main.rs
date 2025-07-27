#[derive(Debug)]
pub struct RawVec<T> {
    pub ptr: *mut T,
    pub cap: usize,
    pub len: usize,
}

impl<T> RawVec<T> {
    fn new() -> Self {
        Self {
            ptr: std::ptr::dangling_mut(),
            cap: 0,
            len: 0,
        }
    }

    fn with_capacity(capacity: usize) -> Self {
        if capacity == 0 {
            return Self {
                ptr: std::ptr::dangling_mut(),
                cap: 0,
                len: 0,
            };
        }
        let layout = match std::alloc::Layout::array::<T>(capacity) {
            Ok(l) => l,
            Err(_) => core::panic!("Unvalid layout"),
        };

        let ptr = unsafe { std::alloc::alloc(layout) };
        let ptr = ptr.cast::<T>();
        if ptr.is_null() {
            std::alloc::handle_alloc_error(layout);
        }

        Self {
            ptr,
            cap: capacity,
            len: 0,
        }
    }

    fn push(&mut self, value: T) {
        if self.cap == self.len {
            core::panic!("OOM");
        }

        let addr = unsafe { self.ptr.add(self.len) };
        unsafe { std::ptr::write(addr, value) };
        self.len += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        }

        self.len -= 1;
        let addr = unsafe { self.ptr.add(self.len) };
        let value = unsafe { std::ptr::read(addr) };
        Some(value)
    }
}

impl<T> Drop for RawVec<T> {
    fn drop(&mut self) {
        if self.cap > 0 {
            if self.len > 0 {
                let slice = unsafe { std::slice::from_raw_parts_mut(self.ptr, self.len) };
                unsafe { std::ptr::drop_in_place(slice) };
            }
            let layout = match std::alloc::Layout::array::<T>(self.cap) {
                Ok(l) => l,
                Err(_) => core::panic!("Unvalid layout"),
            };

            unsafe { std::alloc::dealloc(self.ptr as *mut u8, layout) };
        }
    }
}

fn main() {
    let mut raw_vec = RawVec::<u32>::with_capacity(1000);
    for i in 0..1000 {
        raw_vec.push(i);
    }
    dbg!(&raw_vec);
    for _ in 0..5 {
        raw_vec.pop();
    }
    dbg!(&raw_vec);
    for i in 0..2 {
        raw_vec.push(i);
    }
    dbg!(&raw_vec);
    while let Some(val) = raw_vec.pop() {
        println!("VALUE: {}", val);
    }
}
