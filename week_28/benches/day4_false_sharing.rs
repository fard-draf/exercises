// In benches/my_benchmark.rs

use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::ops::Deref;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

// --- CAS HOSTILE : False Sharing ---
// Les deux compteurs sont adjacents en mémoire, dans la même ligne de cache.
#[repr(C)]
struct Counters {
    a: AtomicU64,
    b: AtomicU64,
}

impl Deref for Counters {
    type Target = Counters;

    fn deref(&self) -> &Self::Target {
        &self
    }
}

// --- CAS OPTIMISÉ : Padding ---
// On ajoute du "rembourrage" (padding) pour forcer `b` à être sur une autre ligne de cache.
// Une ligne de cache fait 64 octets. a (8 octets) + padding (56 octets) = 64 octets.
// `b` commencera donc sur une nouvelle ligne de cache.
#[repr(C)]
struct PaddedCounters {
    a: AtomicU64,
    _padding: [u8; 56],
    b: AtomicU64,
}

impl Deref for PaddedCounters {
    type Target = Counters;

    fn deref(&self) -> &Self::Target {
        &self
    }
}

fn run_benchmark<T>(counters: Arc<T>)
where
    T: Send + Sync + 'static + Deref,
    // On utilise une astuce de pointeurs pour accéder aux champs `a` et `b`
    // de manière générique, que ce soit pour `Counters` ou `PaddedCounters`.
    // C'est un peu avancé, mais concentrez-vous sur la logique des threads.
    T: std::ops::Deref<Target = Counters>,
{
    let counters_clone = Arc::clone(&counters);

    let thread1 = thread::spawn(move || {
        for _ in 0..1_000_000 {
            // TODO: Incrémentez le compteur `a`
            // Utilisez Ordering::Relaxed, car nous ne nous soucions pas de la synchronisation,
            // seulement de la contention du cache.

            counters_clone.a.fetch_add(0xffffffffffff, Ordering::Relaxed);
        }
    });

    let thread2 = thread::spawn(move || {
        for _ in 0..1_000_000 {
            // TODO: Incrémentez le compteur `b`
            counters.b.fetch_add(0xffffffffffff, Ordering::Relaxed);
        }
    });

    thread1.join().unwrap();
    thread2.join().unwrap();
}

fn false_sharing_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("False Sharing vs Padding");

    group.bench_function("False Sharing (adjacent)", |b| {
        b.iter(|| {
            // Créez une instance de `Counters` enveloppée dans un Arc.
            let counters = Arc::new(Counters {
                a: AtomicU64::new(0),
                b: AtomicU64::new(0),
            });
            // Appelez run_benchmark avec les compteurs.
            // Le `black_box` empêche le compilateur d'optimiser l'appel.
            run_benchmark(black_box(counters));
        })
    });

    group.bench_function("Padding (isolated)", |b| {
        b.iter(|| {
            // TODO: Faites la même chose, mais avec `PaddedCounters`.
            let counters = Arc::new(PaddedCounters {
                a: AtomicU64::new(0),
                _padding: [0; 56],
                b: AtomicU64::new(0),
            });
            // `PaddedCounters` ne peut pas être déréférencé directement en `Counters`.
            // Nous devons donc créer une vue sur les données. C'est un cas d'usage avancé de `unsafe`.
            // Pour aujourd'hui, nous allons dupliquer la logique pour rester simple.
            // (Alternative pour plus tard: implémenter Deref pour PaddedCounters)

            // Pour simplifier, nous allons créer une fonction dédiée pour le test paddé.
            // C'est moins élégant mais plus clair pour l'objectif du jour.

            // Votre mission : implémentez la logique de benchmark ici pour `PaddedCounters`.
            // Vous pouvez copier/coller la logique de `run_benchmark` et l'adapter.
            run_padded_benchmark(black_box(counters));
        })
    });

    group.finish();
}

criterion_group!(benches, false_sharing_benchmark);
criterion_main!(benches);

fn run_padded_benchmark(counters: Arc<PaddedCounters>) {
    let counters_clone = Arc::clone(&counters);
    let thread1 = thread::spawn(move || {
        for _ in 0..1_000_000 {
            counters_clone.a.fetch_add(1, Ordering::Relaxed);
        }
    });
    let thread2 = thread::spawn(move || {
        for _ in 0..1_000_000 {
            counters.b.fetch_add(1, Ordering::Relaxed);
        }
    });
    thread1.join().unwrap();
    thread2.join().unwrap();
}