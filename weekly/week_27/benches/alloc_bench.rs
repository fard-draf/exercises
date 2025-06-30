// benches/my_benchmark.rs

use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use week_27::MyData;
// Remplace 'heap_cost_benchmark' par le nom de ton crate si différent

fn benchmark_allocations(c: &mut Criterion) {
    // --- Benchmark 1: Création sur le Stack ---
    c.bench_function("creation_stack", |b| {
        b.iter(|| {
            // TON CODE ICI: Crée une instance de MyData.
            // Utilise black_box pour que le compilateur n'optimise pas l'opération.
            let data = MyData {
                value1: 42,
                value2: 100,
            };
            black_box(data);
        })
    });

    // --- Benchmark 2: Création sur le Heap ---
    c.bench_function("creation_heap", |b| {
        b.iter(|| {
            // TON CODE ICI: Crée une instance de MyData enveloppée dans un Box.
            let boxed_data = Box::new(MyData {
                value1: 42,
                value2: 100,
            });
            black_box(boxed_data);
        })
    });

    // --- Setup pour les benchmarks d'accès ---
    let stack_data = MyData {
        value1: 42,
        value2: 100,
    };
    let heap_data = Box::new(MyData {
        value1: 42,
        value2: 100,
    });

    // --- Benchmark 3: Accès sur le Stack ---
    c.bench_function("access_stack", |b| {
        b.iter(|| {
            // TON CODE ICI: Accède au champ value1 de stack_data.
            black_box(stack_data.value1);
        })
    });

    // --- Benchmark 4: Accès sur le Heap ---
    c.bench_function("access_heap", |b| {
        b.iter(|| {
            // TON CODE ICI: Accède au champ value1 de heap_data.
            // Attention, heap_data est un Box !
            black_box(heap_data.value1);
        })
    });
}

criterion_group!(benches, benchmark_allocations);
criterion_main!(benches);
