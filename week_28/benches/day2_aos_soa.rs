use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

const NUM_PARTICLES: usize = 150_000;
// we have to increase the stack limit -> ulimit -s 32768

// --- Approche AoS (Array of Structs) ---
#[derive(Clone, Copy)]
struct Particle {
    position: [f32; 3],
    velocity: [f32; 3],
    _dummy_data: [u8; 128],
    mass: f32,
}

fn update_aos(particles: &mut [Particle]) {
    // TODO: Itérer et mettre à jour la position de chaque particule
    // en utilisant sa vélocité.
    // ex: position.x += velocity.x;
    particles.iter_mut().for_each(|e| {
        e.position[0] += e.velocity[0];
        e.position[1] += e.velocity[1];
        e.position[2] += e.velocity[2];
    });
}

// --- Approche SoA (Struct of Arrays) ---
#[derive(Clone)]
struct ParticleSystem<const N: usize> {
    positions: [[f32; 3]; N],
    velocities: [[f32; 3]; N],
    _dummy_data: [[u8; 128]; N],
    masses: [f32; N],
}

fn update_soa<const N: usize>(system: &mut ParticleSystem<N>) {
    // TODO: Itérer sur les positions et les mettre à jour
    // en utilisant les vélocités correspondantes.
    // Utilisez zip pour itérer sur plusieurs slices en même temps.
    system
        .positions
        .iter_mut()
        .zip(&system.velocities)
        .for_each(|(pos, vel)| {
            for index in 0..3 {
                pos[index] += vel[index];
            }
        });
}

// --- Benchmark ---
fn bench(c: &mut Criterion) {
    let mut group = c.benchmark_group("AoS_vs_SoA");

    // Benchmark pour AoS
    group.bench_function("AoS", |b| {
        // TODO: Initialiser le Vec<Particle> ici
        let mut aos_vec = vec![
            Particle {
                position: [2.25; 3],
                velocity: [5.22; 3],
                _dummy_data: [0u8; 128],
                mass: 55.5
            };
            NUM_PARTICLES
        ];

        b.iter(|| update_aos(black_box(&mut aos_vec)))
    });

    // Benchmark pour SoA
    group.bench_function("SoA", |b| {
        // TODO: Initialiser la struct ParticleSystem ici
        let mut soa_vec = ParticleSystem {
            positions: [[2.25; 3]; NUM_PARTICLES],
            velocities: [[5.22; 3]; NUM_PARTICLES],
            _dummy_data: [[0u8; 128]; NUM_PARTICLES],
            masses: [55.5; NUM_PARTICLES],
        };
        b.iter(|| update_soa(black_box(&mut soa_vec)))
    });

    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
