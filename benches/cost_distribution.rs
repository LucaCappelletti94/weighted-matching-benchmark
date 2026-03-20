//! Criterion benchmark testing how cost distribution patterns affect
//! LAPMOD, SparseLAPJV, and SparseHungarian performance.

use std::hint::black_box;
use std::time::Duration;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use geometric_traits::prelude::*;
use weighted_matching_benchmark::{
    block_structured_matrix, compute_max_cost, compute_padding, diagonal_dominant_matrix,
    near_degenerate_matrix, sparse_square_matrix,
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

fn bench_cost_n100(c: &mut Criterion) {
    eprintln!("[1/2] Running cost_n100 benchmarks...");
    let mut group = c.benchmark_group("cost_n100");
    let n = 100;
    let density = 0.20;

    group
        .sample_size(100)
        .measurement_time(Duration::from_secs(10));

    let uniform = sparse_square_matrix(42, n, density);
    bench_sparse_three!(group, "uniform", uniform);

    let diag = diagonal_dominant_matrix(42, n, density);
    bench_sparse_three!(group, "diagonal_dominant", diag);

    let block = block_structured_matrix(42, n, 5, density);
    bench_sparse_three!(group, "block_structured", block);

    let degen = near_degenerate_matrix(42, n, density);
    bench_sparse_three!(group, "near_degenerate", degen);

    group.finish();
}

fn bench_cost_n200(c: &mut Criterion) {
    eprintln!("[2/2] Running cost_n200 benchmarks...");
    let mut group = c.benchmark_group("cost_n200");
    let n = 200;
    let density = 0.10;

    group
        .sample_size(30)
        .measurement_time(Duration::from_secs(20));

    let uniform = sparse_square_matrix(42, n, density);
    bench_sparse_three!(group, "uniform", uniform);

    let diag = diagonal_dominant_matrix(42, n, density);
    bench_sparse_three!(group, "diagonal_dominant", diag);

    let block = block_structured_matrix(42, n, 5, density);
    bench_sparse_three!(group, "block_structured", block);

    let degen = near_degenerate_matrix(42, n, density);
    bench_sparse_three!(group, "near_degenerate", degen);

    group.finish();
}

criterion_group!(benches, bench_cost_n100, bench_cost_n200);
criterion_main!(benches);
