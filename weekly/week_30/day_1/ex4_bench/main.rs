use std::cell::{Cell, RefCell};
use std::time::Instant;

const ITERATIONS: u32 = 100_000_000;

fn benchmark_cell() {
    let cell = Cell::new(0u32);
    let start = Instant::now();

    for _ in 0..ITERATIONS {
        // La méthode .get() lit la valeur, la méthode .set() l'écrit.
        // C'est un déplacement (move) de la valeur, pas un emprunt.
        cell.set(cell.get() + 1);
    }

    let duration = start.elapsed();
    println!("Cell: {:?}", duration);
}

fn benchmark_refcell() {
    let ref_cell = RefCell::new(0u32);
    let start = Instant::now();

    for _ in 0..ITERATIONS {
        // À chaque itération, on demande un emprunt mutable.
        *ref_cell.borrow_mut() += 1;
    }

    let duration = start.elapsed();
    println!("RefCell: {:?}", duration);
}

fn main() {
    benchmark_cell();
    benchmark_refcell();
}