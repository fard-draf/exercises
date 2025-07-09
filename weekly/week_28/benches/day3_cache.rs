// benches/my_benchmark.rs

use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

const VEC_SIZE: usize = 1_000_000;

// Votre struct optimisée pour le cache
#[derive(Clone, Copy, Default)]
#[repr(C)]
struct CacheFriendly {
    a: u64,
    b: u64,
    c: u64,
    d: u64,
    e: u64,
    f: u64,
    g: u64,
    next_idx: usize,
}

// Votre struct hostile au cache
#[derive(Clone, Copy)]
#[repr(C)]
struct CacheHostile {
    a: u64,
    _padding: [u8; 56], // Sépare 'a' et 'g' dans des lignes de cache différentes
    g: u64,
    next_idx: usize,
}

impl Default for CacheHostile {
    fn default() -> Self {
        let a = 0;
        let _padding: [u8; 56] = [0u8; 56];
        let g = 0;
        let next_idx = 1;
        Self {
            a,
            _padding,
            g,
            next_idx,
        }
    }
}

fn friendly_benchmark(c: &mut Criterion) {
    // TODO 1 : Créer un Vec<CacheFriendly> de VEC_SIZE éléments.
    // Initialisez les champs `a` et `g` avec des valeurs simples (ex: 1).
    let mut indices: Vec<usize> = (0..VEC_SIZE).collect();
    indices.reverse();
    let mut data: Vec<CacheFriendly> = vec![CacheFriendly::default(); VEC_SIZE];
    for i in 0..VEC_SIZE {
        data[indices[i]].next_idx = indices[(i + 1) % VEC_SIZE];
    }

    println!("Friendly size {}", std::mem::size_of::<CacheFriendly>());
    c.bench_function("Cache Friendly Access", |b| {
        b.iter(|| {
            let mut sum = 0u64;
            let mut current_idx = 0; // On commence toujours à l'indice 0

            // On boucle VEC_SIZE fois en suivant la chaîne
            for _ in 0..data.len() {
                let item = &data[current_idx];
                sum = sum.wrapping_add(black_box(item.a) + black_box(item.g));

                // On passe à l'élément suivant de la chaîne
                current_idx = black_box(item.next_idx);
            }
            black_box(sum);
        })
    });
}

fn hostile_benchmark(c: &mut Criterion) {
    // TODO 2 : Créer un Vec<CacheHostile> de VEC_SIZE éléments.
    // Initialisez `a` et `g` de la même manière.
    let mut indices: Vec<usize> = (0..VEC_SIZE).collect();
    indices.reverse();
    let mut data: Vec<CacheHostile> = vec![CacheHostile::default(); VEC_SIZE];
    for i in 0..VEC_SIZE {
        data[indices[i]].next_idx = indices[(i + 1) % VEC_SIZE];
    }

    println!("Hostile size {}", std::mem::size_of::<CacheHostile>());

    c.bench_function("Cache Hostile Access", |b| {
        b.iter(|| {
            let mut sum = 0u64;
            let mut current_idx = 0; // On commence toujours à l'indice 0

            // On boucle VEC_SIZE fois en suivant la chaîne
            for _ in 0..data.len() {
                let item = &data[current_idx];
                sum = sum.wrapping_add(black_box(item.a) + black_box(item.g));

                // On passe à l'élément suivant de la chaîne
                current_idx = black_box(item.next_idx);
            }
            black_box(sum);
        })
    });
}

criterion_group!(benches, friendly_benchmark, hostile_benchmark);
criterion_main!(benches);
