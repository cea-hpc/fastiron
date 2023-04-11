use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use fastiron::data::mc_vector::MCVector;

// Define the routines that are tested

fn full_cross_product(times: usize) {
    // init
    let v0: MCVector<f64> = MCVector {
        x: 1.923,
        y: -2.45,
        z: 5.013,
    };
    let v1: MCVector<f64> = MCVector {
        x: 3.041,
        y: 1.368,
        z: 9.143,
    };
    let v2: MCVector<f64> = MCVector {
        x: 6.235,
        y: 0.325,
        z: 2.502,
    };
    let facet_coords = [v0, v1, v2];
    let intersection_pt: MCVector<f64> = MCVector {
        x: 1.634,
        y: -1.34,
        z: 3.873,
    };

    // computation loop
    (0..times).for_each(|_| {
        let v01 = facet_coords[1] - facet_coords[0];
        let v12 = facet_coords[2] - facet_coords[1];
        let v20 = facet_coords[0] - facet_coords[2];
        let v0i = intersection_pt - facet_coords[0];
        let v1i = intersection_pt - facet_coords[1];
        let v2i = intersection_pt - facet_coords[2];
        let crosses = [v01.cross(&v0i), v12.cross(&v1i), v20.cross(&v2i)];
        black_box(crosses);
    });
}

fn partial_cross_product(times: usize) {
    // init
    let v0: MCVector<f64> = MCVector {
        x: 1.923,
        y: -2.45,
        z: 5.013,
    };
    let v1: MCVector<f64> = MCVector {
        x: 3.041,
        y: 1.368,
        z: 9.143,
    };
    let v2: MCVector<f64> = MCVector {
        x: 6.235,
        y: 0.325,
        z: 2.502,
    };
    let facet_coords = [v0, v1, v2];
    let intersection_pt: MCVector<f64> = MCVector {
        x: 1.634,
        y: -1.34,
        z: 3.873,
    };
    macro_rules! ab_cross_ac {
        ($ax: expr, $ay: expr, $bx: expr, $by: expr, $cx: expr, $cy: expr) => {
            ($bx - $ax) * ($cy - $ay) - ($by - $ay) * ($cx - $ax)
        };
    }

    (0..times).for_each(|_| {
        let cross1 = ab_cross_ac!(
            facet_coords[0].x,
            facet_coords[0].y,
            facet_coords[1].x,
            facet_coords[1].y,
            intersection_pt.x,
            intersection_pt.y
        );
        let cross2 = ab_cross_ac!(
            facet_coords[1].x,
            facet_coords[1].y,
            facet_coords[2].x,
            facet_coords[2].y,
            intersection_pt.x,
            intersection_pt.y
        );
        let cross0 = ab_cross_ac!(
            facet_coords[2].x,
            facet_coords[2].y,
            facet_coords[0].x,
            facet_coords[0].y,
            intersection_pt.x,
            intersection_pt.y
        );
        black_box(cross0);
        black_box(cross1);
        black_box(cross2);
    });
}

pub fn criterion_benchmark(c: &mut Criterion) {
    // Generate/Define the input
    let n_iter: usize = 1000;

    // Creates a bench using
    // - an Id
    // - a reference to the arguments
    // - the routine put inside an iterator
    let mut group = c.benchmark_group("MCT-Cross-Product");
    group.bench_with_input(
        BenchmarkId::new("Full Cross Product", "number of iterations"),
        &n_iter,
        |b, &n| b.iter(|| full_cross_product(n)),
    );
    group.bench_with_input(
        BenchmarkId::new("Partial Cross Product", "number of iterations"),
        &n_iter,
        |b, &n| b.iter(|| partial_cross_product(n)),
    );
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
