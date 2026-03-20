//! Criterion benchmark comparing all sparse-input algorithms head-to-head
//! on the same square sparse matrices: LAPMOD, SparseLAPJV, SparseHungarian,
//! Crouse, and Jaqaman.

use std::hint::black_box;
use std::time::Duration;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use geometric_traits::prelude::*;
use weighted_matching_benchmark::{
    compute_jaqaman_cost, compute_max_cost, compute_padding, matrix_label, sparse_square_matrix,
};

macro_rules! bench_all_sparse {
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

fn bench_cross_n100(c: &mut Criterion) {
    eprintln!("[1/3] Running cross_n100 benchmarks...");
    let mut group = c.benchmark_group("cross_n100");
    let n = 100;

    for density in [0.05, 0.20, 0.50, 1.0] {
        if density >= 0.50 {
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
        bench_all_sparse!(group, lbl, csr);
    }

    group.finish();
}

fn bench_cross_n300(c: &mut Criterion) {
    eprintln!("[2/3] Running cross_n300 benchmarks...");
    let mut group = c.benchmark_group("cross_n300");
    let n = 300;

    for density in [0.05, 0.20, 0.50] {
        if density >= 0.20 {
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
        bench_all_sparse!(group, lbl, csr);
    }

    group.finish();
}

fn bench_cross_n500(c: &mut Criterion) {
    eprintln!("[3/3] Running cross_n500 benchmarks...");
    let mut group = c.benchmark_group("cross_n500");
    let n = 500;

    for density in [0.05, 0.20, 0.50] {
        group
            .sample_size(10)
            .measurement_time(Duration::from_secs(60));

        eprintln!("  n={n}, density={density}...");
        let csr = sparse_square_matrix(42, n, density);
        let lbl = matrix_label(n, density);
        bench_all_sparse!(group, lbl, csr);
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_cross_n100,
    bench_cross_n300,
    bench_cross_n500
);
criterion_main!(benches);
