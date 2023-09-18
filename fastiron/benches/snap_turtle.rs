use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use fastiron::constants::Tuple3;

// Define the routines that are tested

fn manual_snap(times: usize) {
    let bounds: (usize, usize, usize) = (10, 10, 10);
    fn snap(bounds: (usize, usize, usize), tt: (i32, i32, i32)) -> Tuple3 {
        (
            (tt.0.max(0) as usize).min(bounds.0 - 1),
            (tt.1.max(0) as usize).min(bounds.1 - 1),
            (tt.2.max(0) as usize).min(bounds.2 - 1),
        )
    }
    let tt1 = (1, 3, 2); // nothing to snap
    let tt2 = (-1, 3, 2); // 1 element
    let tt3 = (-1, 11, 2); // 2 elements
    let tt4 = (-1, 11, -1); // 3 elements
    (0..times).for_each(|_| {
        let res = snap(bounds, tt1);
        black_box(res);
    });
    (0..times).for_each(|_| {
        let res = snap(bounds, tt2);
        black_box(res);
    });
    (0..times).for_each(|_| {
        let res = snap(bounds, tt3);
        black_box(res);
    });
    (0..times).for_each(|_| {
        let res = snap(bounds, tt4);
        black_box(res);
    });
}

fn clamp_snap(times: usize) {
    let bounds: (usize, usize, usize) = (10, 10, 10);
    fn snap(bounds: (usize, usize, usize), tt: (i32, i32, i32)) -> Tuple3 {
        (
            tt.0.clamp(0, (bounds.0 - 1) as i32) as usize,
            tt.1.clamp(0, (bounds.1 - 1) as i32) as usize,
            tt.2.clamp(0, (bounds.2 - 1) as i32) as usize,
        )
    }
    let tt1 = (1, 3, 2); // nothing to snap
    let tt2 = (-1, 3, 2); // 1 element
    let tt3 = (-1, 11, 2); // 2 elements
    let tt4 = (-1, 11, -1); // 3 elements
    (0..times).for_each(|_| {
        let res = snap(bounds, tt1);
        black_box(res);
    });
    (0..times).for_each(|_| {
        let res = snap(bounds, tt2);
        black_box(res);
    });
    (0..times).for_each(|_| {
        let res = snap(bounds, tt3);
        black_box(res);
    });
    (0..times).for_each(|_| {
        let res = snap(bounds, tt4);
        black_box(res);
    });
}

pub fn criterion_benchmark(c: &mut Criterion) {
    // Generate/Define the input
    let n_iter: usize = 1000;

    let mut group = c.benchmark_group("snap turtle implementation");
    group.bench_with_input(
        BenchmarkId::new("manual snap", "number of iterations/4"),
        &n_iter,
        |b, &n| b.iter(|| manual_snap(n)),
    );
    group.bench_with_input(
        BenchmarkId::new("clamp snap", "number of iterations/4"),
        &n_iter,
        |b, &n| b.iter(|| clamp_snap(n)),
    );
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
