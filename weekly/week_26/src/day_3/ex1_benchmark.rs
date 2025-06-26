use std::time::Instant;

// Constante pour la taille du vecteur, facile à modifier pour les tests.
const VECTOR_SIZE: usize = 50_000_000;

fn main() {
    println!("Démarrage du benchmark avec {} éléments.", VECTOR_SIZE);

    // --- Expérience 1: Données contiguës ---
    let data_contiguous: Vec<i32> = (0..VECTOR_SIZE as i32).collect();
    let start_contiguous = Instant::now();
    
    // TODO 1: Utilise .iter().fold() pour sommer tous les éléments de `data_contiguous`.
    // L'accumulateur démarre à 0. L'opération est une simple addition.
    let sum_contiguous: i64 = data_contiguous.iter().fold(0, |acc, &x| acc + x as i64);

    let duration_contiguous = start_contiguous.elapsed();
    println!("[Contiguous] Somme: {}. Temps: {:?}", sum_contiguous, duration_contiguous);


    // --- Expérience 2: Données derrière des pointeurs ---
    let data_indirect: Vec<Box<i32>> = (0..VECTOR_SIZE as i32).map(Box::new).collect();
    let start_indirect = Instant::now();

    // TODO 2: Fais la même chose pour `data_indirect`.
    // Attention au type de l'élément dans la closure, c'est un &Box<i32>.
    // Il faudra le déréférencer deux fois pour obtenir la valeur i32.
    let sum_indirect: i64 = data_indirect.iter().fold(0, |acc, b| acc + **b as i64);

    let duration_indirect = start_indirect.elapsed();
    println!("[Indirect]   Somme: {}. Temps: {:?}", sum_indirect, duration_indirect);

    // --- Analyse ---
    // TODO 3: Calcule et affiche le ratio de performance.
    // (duration_indirect.as_nanos() / duration_contiguous.as_nanos())
    if duration_contiguous.as_nanos() > 0 {
        let ratio = duration_indirect.as_nanos() as f64 / duration_contiguous.as_nanos() as f64;
        println!("\nConclusion: L'approche indirecte est {:.2}x plus lente.", ratio);
    }
}