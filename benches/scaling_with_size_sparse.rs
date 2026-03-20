//! Criterion benchmark comparing LAPMOD, SparseLAPJV, SparseHungarian,
//! Crouse, and Jaqaman on sparse matrices of increasing size at fixed density.

use std::hint::black_box;
use std::time::Duration;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use geometric_traits::prelude::*;
use weighted_matching_benchmark::{
    compute_jaqaman_cost, compute_max_cost, compute_padding, matrix_label, sparse_square_matrix,
};

macro_rules! bench_sparse_five {
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
        $group.bench_with_input(BenchmarkId::new("Crouse", &$label), csr, |b, m| {
            b.iter(|| black_box(m.crouse(black_box(padding), black_box(max_cost)).ok()));
        });
        let jaqaman_cost = compute_jaqaman_cost(max_cost);
        $group.bench_with_input(BenchmarkId::new("Jaqaman", &$label), csr, |b, m| {
            b.iter(|| black_box(m.jaqaman(black_box(jaqaman_cost), black_box(max_cost)).ok()));
        });
    }};
}

fn bench_sparse_5pct(c: &mut Criterion) {
    eprintln!("[1/2] Running size_sparse_5pct benchmarks...");
    let mut group = c.benchmark_group("size_sparse_5pct");
    let density = 0.05;

    for n in [20usize, 50, 100, 200, 500, 1000, 2000, 3000, 5000, 10000] {
        if n >= 2000 {
            group
                .sample_size(10)
                .measurement_time(Duration::from_secs(60));
        } else if n >= 500 {
            group
                .sample_size(30)
                .measurement_time(Duration::from_secs(20));
        } else {
            group
                .sample_size(100)
                .measurement_time(Duration::from_secs(10));
        }

        eprintln!("  Generating sparse_square_matrix(n={n}, d={density})...");
        let csr = sparse_square_matrix(0xDEAD_BEEF_u64.wrapping_mul(n as u64), n, density);
        let lbl = matrix_label(n, density);
        bench_sparse_five!(group, lbl, csr);
    }

    group.finish();
}

fn bench_sparse_20pct(c: &mut Criterion) {
    eprintln!("[2/2] Running size_sparse_20pct benchmarks...");
    let mut group = c.benchmark_group("size_sparse_20pct");
    let density = 0.20;

    for n in [20usize, 50, 100, 200, 500, 1000, 2000, 3000, 5000, 10000] {
        if n >= 2000 {
            group
                .sample_size(10)
                .measurement_time(Duration::from_secs(60));
        } else if n >= 500 {
            group
                .sample_size(30)
                .measurement_time(Duration::from_secs(20));
        } else {
            group
                .sample_size(100)
                .measurement_time(Duration::from_secs(10));
        }

        eprintln!("  Generating sparse_square_matrix(n={n}, d={density})...");
        let csr = sparse_square_matrix(0xDEAD_BEEF_u64.wrapping_mul(n as u64), n, density);
        let lbl = matrix_label(n, density);
        bench_sparse_five!(group, lbl, csr);
    }

    group.finish();
}

criterion_group!(benches, bench_sparse_5pct, bench_sparse_20pct,);
criterion_main!(benches);
