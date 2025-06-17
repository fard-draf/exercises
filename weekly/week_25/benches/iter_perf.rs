use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use week_25::day_2::ex1_bench::{sum_iterator_fold, sum_iterator_sum, sum_manual_loop, stack_vec_by_fold, stack_vec_by_loop, stack_vec_by_loop_with_capacity, stack_vec_by_map};

// Supposons que vos fonctions sont dans votre crate principal (lib.rs ou main.rs)
// et qu'il faut les rendre publiques pour le benchmark.
// Exemple de ce que vous pourriez avoir dans src/lib.rs :
/*
pub fn sum_manual_loop(data: &[u64]) -> u64 {
    let mut sum = 0;
    for &item in data {
        sum += item;
    }
    sum
}

pub fn sum_iterator_sum(data: &[u64]) -> u64 {
    data.iter().sum()
}

pub fn sum_iterator_fold(data: &[u64]) -> u64 {
    data.iter().fold(0, |acc, &x| acc + x)
}
*/
// N'oubliez pas d'importer votre crate. Remplacez `mon_projet` par le nom de votre crate.
// use crate::wee::{sum_iterator_fold, sum_iterator_sum, sum_manual_loop};

fn benchmark_sum(c: &mut Criterion) {
    // Préparons des données de test.
    // `black_box` empêche le compilateur d'optimiser les données en amont.
    let data = black_box((0..1000).collect::<Vec<u64>>());
    let mut group = c.benchmark_group("Summation Comparison");

    // for strings
    // let owned_strings: Vec<String> = (0..1000).map(|i| i.to_string()).collect();
    // let string_slices: Vec<&str> = owned_strings.iter().map(|s| s.as_str()).collect();
    // let data = black_box(&string_slices);



    group.bench_function("Manual For Loop", |b| {
        b.iter(|| stack_vec_by_loop(black_box(&data)));
    });

    group.bench_function("Manual Optimized For Loop", |b| {
        b.iter(|| stack_vec_by_loop_with_capacity(black_box(&data)));
    });

    group.bench_function("Fold Vec", |b| {
        b.iter(|| stack_vec_by_fold(black_box(&data)));
    });

    group.bench_function("Map Vec", |b| {
        b.iter(|| stack_vec_by_map(black_box(&data)));
    });
    // group.bench_function("Manual For Loop", |b| {
    //     b.iter(|| sum_manual_loop(black_box(&data)))
    // });

    // group.bench_function("Iterator::sum()", |b| {
    //     b.iter(|| sum_iterator_sum(black_box(&data)))
    // });

    // group.bench_function("Iterator::fold()", |b| {
    //     b.iter(|| sum_iterator_fold(black_box(&data)))
    // });

    group.finish();
}

criterion_group!(benches, benchmark_sum);
criterion_main!(benches);