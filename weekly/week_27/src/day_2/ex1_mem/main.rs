use std::mem::{align_of, size_of};

struct Simple { //pred -> 2 bytes
    a: u8,
    b: u16,
}

struct TupleStruct(u8, u32); //pred -> 4 bytes

struct Complex { //pred -> 8 bytes
    a: u16,
    b: u64,
    c: bool,
}

struct Nested { //pred -> 4 bytes
    a: u8,
    b: Simple,
    c: u32,
}

struct ZeroSized { //pred -> 4 bytes
    a: u32,
    b: [u8; 0], // Tableau de taille zéro
    c: (),      // Unit type
}

fn main() {
    // Nous allons remplir cette partie avec nos prédictions et vérifications.
    println!("Struct Simple: aligne of {}", align_of::<Simple>());
    println!("Struct TupleStruct: aligne of {}", align_of::<TupleStruct>());
    println!("Struct Complex: aligne of {}", align_of::<Complex>());
    println!("Struct Nested: aligne of {}", align_of::<Nested>());
    println!("Struct Zero Sized: aligne of {}", align_of::<ZeroSized>());
    
}