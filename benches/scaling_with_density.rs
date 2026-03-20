//! Criterion benchmark comparing LAPMOD, SparseLAPJV, and SparseHungarian
//! at fixed matrix sizes with varying density.

use std::hint::black_box;
use std::time::Duration;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use geometric_traits::prelude::*;
use weighted_matching_benchmark::{
    compute_max_cost, compute_padding, matrix_label, sparse_square_matrix,
};

macro_rules! bench_sparse_three {
    ($group:expr, $label:expr, $csr:expr) => {{
        let csr = &$csr;
        let max_cost = compute_max_cost(csr);
        let padding = compute_padding(max_cost);

        $group.bench_with_input(BenchmarkId::new("LAPMOD", &$label), csr, |b, m| {
            b.iter(|| black_box(m.lapmod(black_box(max_cost)).ok()));
        });
        $group.bench_with_input(BenchmarkId::new("SparseLAPJV", &$label), csr, |b, m| {
            b.iter(|| black_box(m.sparse_lapjv(black_box(padding), black_box(max_cost)).ok()));
        });
        $group.bench_with_input(BenchmarkId::new("SparseHungarian", &$label), csr, |b, m| {
            b.iter(|| {
                black_box(
                    m.sparse_hungarian(black_box(padding), black_box(max_cost))
                        .ok(),
                )
            });
        });
    }};
}

fn bench_density_n100(c: &mut Criterion) {
    eprintln!("[1/3] Running density_n100 benchmarks...");
    let mut group = c.benchmark_group("density_n100");
    let n = 100;

    for density in [0.01, 0.02, 0.05, 0.10, 0.20, 0.30, 0.50, 0.70, 1.0] {
        group
            .sample_size(100)
            .measurement_time(Duration::from_secs(10));

        eprintln!("  n={n}, density={density}...");
        let csr = sparse_square_matrix(42, n, density);
        let lbl = matrix_label(n, density);
        bench_sparse_three!(group, lbl, csr);
    }

    group.finish();
}

fn bench_density_n200(c: &mut Criterion) {
    eprintln!("[2/3] Running density_n200 benchmarks...");
    let mut group = c.benchmark_group("density_n200");
    let n = 200;

    for density in [0.01, 0.02, 0.05, 0.10, 0.20, 0.30, 0.50, 0.70, 1.0] {
        if density >= 0.30 {
            group
                .sample_size(30)
                .measurement_time(Duration::from_secs(20));
        } else {
            group
                .sample_size(100)
                .measurement_time(Duration::from_secs(10));
        }

        eprintln!("  n={n}, density={density}...");
        let csr = sparse_square_matrix(42, n, density);
        let lbl = matrix_label(n, density);
        bench_sparse_three!(group, lbl, csr);
    }

    group.finish();
}

fn bench_density_n500(c: &mut Criterion) {
    eprintln!("[3/3] Running density_n500 benchmarks...");
    let mut group = c.benchmark_group("density_n500");
    let n = 500;

    for density in [0.01, 0.02, 0.05, 0.10, 0.20, 0.30, 0.50] {
        if density >= 0.10 {
            group
                .sample_size(10)
                .measurement_time(Duration::from_secs(60));
        } else {
            group
                .sample_size(30)
                .measurement_time(Duration::from_secs(20));
        }

        eprintln!("  n={n}, density={density}...");
        let csr = sparse_square_matrix(42, n, density);
        let lbl = matrix_label(n, density);
        bench_sparse_three!(group, lbl, csr);
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_density_n100,
    bench_density_n200,
    bench_density_n500,
);
criterion_main!(benches);
