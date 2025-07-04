use std::mem::size_of;

fn main() {
    // --- Cas 1 : Types Pointeur ---
    println!("--- Analyse des Pointeurs ---");
    // Un Box<T> est un pointeur vers le tas (heap).
    println!("Taille de Box<u8>:         {}", size_of::<Box<u8>>());
    // Comment la taille d'une Option contenant un pointeur peut-elle être la même ?
    println!("Taille de Option<Box<u8>>:   {}", size_of::<Option<Box<u8>>>());

    println!(); // Saut de ligne

    // Une référence &T est aussi un pointeur.
    println!("Taille de &u8:             {}", size_of::<&u8>());
    println!("Taille de Option<&u8>:       {}", size_of::<Option<&u8>>());
    println!();

    // --- Cas 2 : Type Primitif ---
    println!("--- Analyse des Primitifs ---");
    println!("Taille de u64:             {}", size_of::<u64>());
    // L'optimisation fonctionne-t-elle ici ?
    println!("Taille de Option<u64>:      {}", size_of::<Option<u64>>());
    
}


