use std::marker::PhantomData;

struct MonSlice<'a> {
    ptr: *const u8,
    len: usize,
    _marker: PhantomData<&'a [u8]>,
}

impl<'a> MonSlice<'a> {
    fn new(data: &'a [u8]) -> Self {
        MonSlice {
            ptr: data.as_ptr(),
            len: data.len(),
            _marker: PhantomData,
        }
    }
}

fn main() {
    let mut mon_slice;

    {
        let data = vec![1, 2, 3, 4];
        mon_slice = MonSlice::new(&data);
    }

    println!(
        "[UNVALID POINTER]: {:p}, [DATA]: {}",
        mon_slice.ptr,
        unsafe { mon_slice.ptr.read() }
    )
}
