use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use fastiron::mc::mc_vector::MCVector;
use num::Float;

// Define the routines that are tested

fn add_vectors<T: Float>(uu: MCVector<T>, vv: MCVector<T>) {
    _ = uu + vv;
}

fn add_assign_vectors<T: Float>(mut uu: MCVector<T>, vv: MCVector<T>) {
    uu += vv;
}

fn mul_vector<T: Float>(uu: MCVector<T>, f: T) {
    _ = uu * f;
}

pub fn criterion_benchmark(c: &mut Criterion) {
    // Generate/Define the input

    let uu = MCVector {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };

    let vv = MCVector {
        x: 2.0,
        y: 2.0,
        z: 2.0,
    };

    let f = 2.243;

    // Creates a bench using
    // - an Id
    // - a reference to the arguments
    // - the routine put inside an iterator
    c.bench_with_input(BenchmarkId::new("add", "2 vectors"), &(uu, vv), |b, &t| {
        b.iter(|| add_vectors(t.0, t.1))
    });

    c.bench_with_input(
        BenchmarkId::new("add_assign", "2 vectors"),
        &(uu, vv),
        |b, &t| b.iter(|| add_assign_vectors(t.0, t.1)),
    );

    c.bench_with_input(
        BenchmarkId::new("mul_vector", "1 vector & 1 scalar"),
        &(uu, f),
        |b, &t| b.iter(|| mul_vector(t.0, t.1)),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
