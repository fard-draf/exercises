use std::ptr;

fn main() {
    let data = [10i32, 20, 30, 40];

    let p: *const i32 = data.as_ptr();

    for e in 0..4 {
        unsafe {
            println!("{:?}", p.add(e).read());
        };
    }

    for e in 0..4 {
        unsafe {
            let p: *mut i32 = p as *mut i32;
            std::ptr::write(p.add(e), p.add(e).read() * 2);

            println!("{:?}", p.add(e).read());
        }
    }

    println!("[TABLEAU]: {:#?}", data);
    assert_eq!(data, [20, 40, 60, 80]);
}
