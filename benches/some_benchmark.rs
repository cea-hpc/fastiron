use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use fastiron::mc::mc_vector::MCVector;

// Define the routines that are tested

fn add_vectors(times: usize) {
    let uu = MCVector {
        x: 1.0,
        y: 3.0,
        z: 5.0,
    };

    let vv = MCVector {
        x: 2.0,
        y: 4.0,
        z: 6.0,
    };
    (0..times).for_each(|_| {
        let res = uu + vv;
        black_box(res);
    });
}

pub fn criterion_benchmark(c: &mut Criterion) {
    // Generate/Define the input
    let n_iter: usize = 1000;

    // Creates a bench using
    // - an Id
    // - a reference to the arguments
    // - the routine put inside an iterator
    c.bench_with_input(
        BenchmarkId::new("add", "number of iterations"),
        &n_iter,
        |b, &n| b.iter(|| add_vectors(n)),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
