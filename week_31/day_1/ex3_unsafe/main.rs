#[repr(C)] // Repr(C) permet de dire au LLVM de ne pas optimiser le layout de cette struct. Comme nous utilisons un raw pointer, il est esssentiel de garder
           // chaque field au bon emplacement memoire.
struct Point {
    x: i32,
    y: i32,
}

unsafe fn move_point(p: *mut Point, dx: i32, dy: i32) {
    if p.is_null() {
        core::panic!("Pointeur Null");
    }
    // SAFETY: le rawpointer est valide, on peut l utiliser en tant que &mut.
    let point_ref = unsafe { &mut *p };

    point_ref.x += dx;
    point_ref.y += dy;
}

fn main() {
    let mut point = Point { x: 10, y: 20 };
    let p_mut: *mut Point = &mut point;

    unsafe {
        move_point(p_mut, 5, -5);
    }

    let p_mut2 = unsafe { &mut *p_mut };
    unsafe {
        move_point(p_mut2, 5, -5);
    }

    assert_eq!(point.x, 20);
    assert_eq!(point.y, 10);
    println!("Déplacement réussi: ({}, {})", point.x, point.y);
}

//  cargo +nightly miri run --bin ex3_unsafe
