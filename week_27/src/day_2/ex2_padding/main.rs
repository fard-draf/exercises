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


fn main() {
    println!("Analyse de 'Disordered':");
    println!("  - Alignement prédit: 8 | Réel: {}", align_of::<Disordered>());
    println!("  - Taille prédite:    24 | Réelle: {}", size_of::<Disordered>());
    println!();
    
    println!("Analyse de 'Ordered':");
    println!("  - Alignement prédit: 8 | Réel: {}", align_of::<Ordered>());
    println!("  - Taille prédite:    16 | Réelle: {}", size_of::<Ordered>());
    println!();
    
    println!("Analyse de 'AlsoOrdered':");
    println!("  - Alignement prédit: 8 | Réel: {}", align_of::<AlsoOrdered>());
    println!("  - Taille prédite:    16 | Réelle: {}", size_of::<AlsoOrdered>());
}