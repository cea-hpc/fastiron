use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use fastiron::utils::mc_rng_state::rng_sample;
use rand::prelude::*;

// Define the routines that are tested

fn custom_sample(times: usize) {
    let mut seed: u64 = 9017476812930;
    (0..times).for_each(|_| {
        let res: f64 = rng_sample(&mut seed);
        black_box(res);
    });
}

fn rand_std_sample(times: usize) {
    let mut rng: StdRng = rand::SeedableRng::seed_from_u64(9017476812930);
    (0..times).for_each(|_| {
        let res: f64 = rng.gen();
        black_box(res);
    });
}

fn rand_small_sample(times: usize) {
    let mut rng: SmallRng = rand::SeedableRng::seed_from_u64(9017476812930);
    (0..times).for_each(|_| {
        let res: f64 = rng.gen();
        black_box(res);
    });
}

fn rand_thread_sample(times: usize) {
    let mut rng: ThreadRng = rand::thread_rng();
    (0..times).for_each(|_| {
        let res: f64 = rng.gen();
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
        BenchmarkId::new("rand crate StdRng", "number of iterations"),
        &n_iter,
        |b, &n| b.iter(|| rand_std_sample(n)),
    );
    group.bench_with_input(
        BenchmarkId::new("rand crate SmallRng", "number of iterations"),
        &n_iter,
        |b, &n| b.iter(|| rand_small_sample(n)),
    );
    group.bench_with_input(
        BenchmarkId::new("rand crate ThreadRng", "number of iterations"),
        &n_iter,
        |b, &n| b.iter(|| rand_thread_sample(n)),
    );
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
