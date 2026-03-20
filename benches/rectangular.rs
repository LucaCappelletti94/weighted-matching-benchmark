//! Criterion benchmark profiling Crouse and Jaqaman on rectangular matrices of
//! varying shape (square, wide, tall) and density.

use std::hint::black_box;
use std::time::Duration;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use geometric_traits::prelude::*;
use weighted_matching_benchmark::{
    compute_jaqaman_cost, compute_max_cost, compute_padding, rect_label, rectangular_sparse_matrix,
};

fn bench_rect_algos(
    group: &mut criterion::BenchmarkGroup<criterion::measurement::WallTime>,
    n_rows: usize,
    n_cols: usize,
    density: f64,
) {
    let csr = rectangular_sparse_matrix(42, n_rows, n_cols, density);
    let max_cost = compute_max_cost(&csr);
    let non_edge_cost = compute_padding(max_cost);
    let lbl = rect_label(n_rows, n_cols, density);

    group.bench_with_input(BenchmarkId::new("Crouse", &lbl), &csr, |b, m| {
        b.iter(|| black_box(m.crouse(black_box(non_edge_cost), black_box(max_cost)).ok()));
    });
    let jaqaman_cost = compute_jaqaman_cost(max_cost);
    group.bench_with_input(BenchmarkId::new("Jaqaman", &lbl), &csr, |b, m| {
        b.iter(|| black_box(m.jaqaman(black_box(jaqaman_cost), black_box(max_cost)).ok()));
    });
}

fn bench_rect_square(c: &mut Criterion) {
    eprintln!("[1/7] Running rect_square benchmarks...");
    let mut group = c.benchmark_group("rect_square");
    let density = 0.10;

    for n in [20usize, 50, 100, 200, 500] {
        if n >= 500 {
            group
                .sample_size(10)
                .measurement_time(Duration::from_secs(60));
        } else if n >= 200 {
            group
                .sample_size(30)
                .measurement_time(Duration::from_secs(20));
        } else {
            group
                .sample_size(100)
                .measurement_time(Duration::from_secs(10));
        }
        bench_rect_algos(&mut group, n, n, density);
    }

    group.finish();
}

fn bench_rect_wide(c: &mut Criterion) {
    eprintln!("[2/7] Running rect_wide benchmarks...");
    let mut group = c.benchmark_group("rect_wide");
    let density = 0.10;

    for (nr, nc) in [(50, 100), (100, 200), (100, 500), (200, 500)] {
        if nr >= 200 || nc >= 500 {
            group
                .sample_size(10)
                .measurement_time(Duration::from_secs(60));
        } else {
            group
                .sample_size(30)
                .measurement_time(Duration::from_secs(20));
        }
        bench_rect_algos(&mut group, nr, nc, density);
    }

    group.finish();
}

fn bench_rect_tall(c: &mut Criterion) {
    eprintln!("[3/7] Running rect_tall benchmarks...");
    let mut group = c.benchmark_group("rect_tall");
    let density = 0.10;

    for (nr, nc) in [(100, 50), (200, 100), (500, 200)] {
        if nr >= 500 {
            group
                .sample_size(10)
                .measurement_time(Duration::from_secs(60));
        } else {
            group
                .sample_size(30)
                .measurement_time(Duration::from_secs(20));
        }
        bench_rect_algos(&mut group, nr, nc, density);
    }

    group.finish();
}

fn bench_rect_square_d30(c: &mut Criterion) {
    eprintln!("[4/7] Running rect_square_d30 benchmarks...");
    let mut group = c.benchmark_group("rect_square_d30");
    let density = 0.30;

    for n in [20usize, 50, 100, 200, 500] {
        if n >= 500 {
            group
                .sample_size(10)
                .measurement_time(Duration::from_secs(60));
        } else if n >= 200 {
            group
                .sample_size(30)
                .measurement_time(Duration::from_secs(20));
        } else {
            group
                .sample_size(100)
                .measurement_time(Duration::from_secs(10));
        }
        bench_rect_algos(&mut group, n, n, density);
    }

    group.finish();
}

fn bench_rect_wide_d30(c: &mut Criterion) {
    eprintln!("[5/7] Running rect_wide_d30 benchmarks...");
    let mut group = c.benchmark_group("rect_wide_d30");
    let density = 0.30;

    for (nr, nc) in [(50, 100), (100, 200), (100, 500), (200, 500)] {
        if nr >= 200 || nc >= 500 {
            group
                .sample_size(10)
                .measurement_time(Duration::from_secs(60));
        } else {
            group
                .sample_size(30)
                .measurement_time(Duration::from_secs(20));
        }
        bench_rect_algos(&mut group, nr, nc, density);
    }

    group.finish();
}

fn bench_rect_tall_d30(c: &mut Criterion) {
    eprintln!("[6/7] Running rect_tall_d30 benchmarks...");
    let mut group = c.benchmark_group("rect_tall_d30");
    let density = 0.30;

    for (nr, nc) in [(100, 50), (200, 100), (500, 200)] {
        if nr >= 500 {
            group
                .sample_size(10)
                .measurement_time(Duration::from_secs(60));
        } else {
            group
                .sample_size(30)
                .measurement_time(Duration::from_secs(20));
        }
        bench_rect_algos(&mut group, nr, nc, density);
    }

    group.finish();
}

fn bench_rect_extreme(c: &mut Criterion) {
    eprintln!("[7/7] Running rect_extreme benchmarks...");
    let mut group = c.benchmark_group("rect_extreme");
    let density = 0.10;

    group
        .sample_size(10)
        .measurement_time(Duration::from_secs(60));

    for (nr, nc) in [(20, 1000), (50, 1000), (1000, 20), (1000, 50)] {
        bench_rect_algos(&mut group, nr, nc, density);
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_rect_square,
    bench_rect_wide,
    bench_rect_tall,
    bench_rect_square_d30,
    bench_rect_wide_d30,
    bench_rect_tall_d30,
    bench_rect_extreme,
);
criterion_main!(benches);
