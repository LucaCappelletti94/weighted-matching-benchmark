//! Criterion benchmark comparing LAPJV and Hungarian on dense matrices
//! (VecMatrix2D) and LAPMOD on equivalent sparse matrices (ValuedCSR2D at
//! 100% density).

use std::hint::black_box;
use std::time::Duration;

use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};
use geometric_traits::prelude::*;
use weighted_matching_benchmark::{
    compute_dense_max_cost, compute_max_cost, dense_square_matrix, sparse_square_matrix,
};

fn bench_dense(c: &mut Criterion) {
    eprintln!("[1/1] Running size_dense benchmarks...");
    let mut group = c.benchmark_group("size_dense");

    for n in [20usize, 50, 100, 200, 300, 400, 500, 750, 1000] {
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

        let lbl = format!("n={n}");

        // Dense matrix for LAPJV and Hungarian (native dense representation).
        eprintln!("  Generating dense_square_matrix n={n}...");
        let dense = dense_square_matrix(0xDEAD_BEEF_u64.wrapping_mul(n as u64), n);
        let dense_max_cost = compute_dense_max_cost(&dense);

        group.bench_with_input(BenchmarkId::new("LAPJV", &lbl), &dense, |b, m| {
            b.iter(|| black_box(m.lapjv(black_box(dense_max_cost)).ok()));
        });
        group.bench_with_input(BenchmarkId::new("Hungarian", &lbl), &dense, |b, m| {
            b.iter(|| black_box(m.hungarian(black_box(dense_max_cost)).ok()));
        });

        // Sparse CSR at 100% density for LAPMOD.
        eprintln!("  Generating sparse_square_matrix n={n} d=1.0...");
        let csr = sparse_square_matrix(0xDEAD_BEEF_u64.wrapping_mul(n as u64), n, 1.0);
        let csr_max_cost = compute_max_cost(&csr);

        group.bench_with_input(BenchmarkId::new("LAPMOD", &lbl), &csr, |b, m| {
            b.iter(|| black_box(m.lapmod(black_box(csr_max_cost)).ok()));
        });
    }

    group.finish();
}

criterion_group!(benches, bench_dense);
criterion_main!(benches);
