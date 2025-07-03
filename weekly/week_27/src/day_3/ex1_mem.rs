use std::mem::{align_of, size_of};

// La version "naïve" ou désordonnée
#[repr(C)]
struct Disordered {
    a: u8,
    b: u64,
    c: u8,
}

#[repr(C)]
// La version optimisée
struct Ordered {
    a: u8,
    c: u8,
    b: u64,
}

// Une autre version optimisée ?
#[repr(C)]
struct AlsoOrdered {
    b: u64,
    a: u8,
    c: u8,
}