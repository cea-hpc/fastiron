use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

// Define the routines that are tested

fn custom_sample(times: usize) {
    (0..times).for_each(|_| {
        let res = 0;
        black_box(res);
    });
}

fn rand_sample(times: usize) {
    (0..times).for_each(|_| {
        let res = 0;
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
    let mut group = c.benchmark_group("RNG");
    group.bench_with_input(
        BenchmarkId::new("Custom RNG", "number of iterations"),
        &n_iter,
        |b, &n| b.iter(|| custom_sample(n)),
    );
    group.bench_with_input(
        BenchmarkId::new("rand crate RNG", "number of iterations"),
        &n_iter,
        |b, &n| b.iter(|| rand_sample(n)),
    );
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
