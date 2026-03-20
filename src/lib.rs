use geometric_traits::{
    impls::{ValuedCSR2D, VecMatrix2D},
    prelude::{MatrixMut, SparseMatrixMut, SparseValuedMatrix},
    traits::algorithms::randomized_graphs::XorShift64,
};
use num_traits::ToPrimitive;

/// Type alias for the sparse cost matrix used across all benchmarks.
pub type SparseCostMatrix = ValuedCSR2D<usize, usize, usize, f64>;

/// Type alias for the dense cost matrix used in dense benchmarks.
pub type DenseCostMatrix = VecMatrix2D<f64>;

/// Compute the target number of edges for an `n x n` matrix at a given density.
pub fn target_edge_count(n: usize, density: f64) -> usize {
    let Some(total_cells) = n.checked_mul(n).and_then(|value| value.to_f64()) else {
        return usize::MAX;
    };
    (total_cells * density)
        .floor()
        .to_usize()
        .unwrap_or(usize::MAX)
}

/// Compute the target edge count for an `n_rows x n_cols` rectangular matrix.
pub fn target_edge_count_rect(n_rows: usize, n_cols: usize, density: f64) -> usize {
    let Some(total_cells) = n_rows.checked_mul(n_cols).and_then(|value| value.to_f64()) else {
        return usize::MAX;
    };
    (total_cells * density)
        .floor()
        .to_usize()
        .unwrap_or(usize::MAX)
}

/// Generate a random index in `[0, n)` from the RNG.
pub fn random_index(rng: &mut XorShift64, n: usize) -> usize {
    let n_u64 = u64::try_from(n).expect("usize values always fit into u64");
    let raw = rng.next().expect("XorShift64 produces infinite values") % n_u64;
    usize::try_from(raw).expect("raw index is modulo n and always fits usize")
}

/// Generate a random cost in `[0.01, 9.99]` from the RNG.
pub fn random_cost(rng: &mut XorShift64) -> f64 {
    let raw = rng.next().expect("XorShift64 produces infinite values") % 999 + 1;
    let cents = u32::try_from(raw).expect("bounded to the range 1..=999");
    f64::from(cents) / 100.0
}

/// Generate a random permutation of `[0, n)` using Fisher-Yates shuffle.
fn random_permutation(rng: &mut XorShift64, n: usize) -> Vec<usize> {
    let mut perm: Vec<usize> = (0..n).collect();
    for i in (1..n).rev() {
        let j = random_index(rng, i + 1);
        perm.swap(i, j);
    }
    perm
}

/// Generate a random `n x n` sparse cost matrix with the given density.
///
/// Embeds a random permutation (perfect matching) to guarantee feasibility,
/// then fills additional random edges to reach the target density.
pub fn sparse_square_matrix(seed: u64, n: usize, density: f64) -> SparseCostMatrix {
    let mut rng = XorShift64::from(seed);
    let total_edges = target_edge_count(n, density).max(n);
    let mut csr: SparseCostMatrix = ValuedCSR2D::with_sparse_shaped_capacity((n, n), total_edges);

    // Embed a random permutation to guarantee a feasible perfect matching.
    let perm = random_permutation(&mut rng, n);
    for (row, &col) in perm.iter().enumerate() {
        let cost = random_cost(&mut rng);
        let _ = csr.add((row, col, cost));
    }

    // Fill additional random edges to reach target density.
    let extra = total_edges.saturating_sub(n);
    for _ in 0..extra {
        let row = random_index(&mut rng, n);
        let col = random_index(&mut rng, n);
        let cost = random_cost(&mut rng);
        let _ = csr.add((row, col, cost));
    }

    csr
}

/// Generate a random `n_rows x n_cols` rectangular sparse cost matrix.
///
/// Embeds a matching of size `min(n_rows, n_cols)` to guarantee feasibility.
pub fn rectangular_sparse_matrix(
    seed: u64,
    n_rows: usize,
    n_cols: usize,
    density: f64,
) -> SparseCostMatrix {
    let mut rng = XorShift64::from(seed);
    let min_dim = n_rows.min(n_cols);
    let total_edges = target_edge_count_rect(n_rows, n_cols, density).max(min_dim);
    let mut csr: SparseCostMatrix =
        ValuedCSR2D::with_sparse_shaped_capacity((n_rows, n_cols), total_edges);

    // Embed a matching: pair row i with a random column for i in [0, min_dim).
    let col_perm = random_permutation(&mut rng, n_cols);
    for (row, &col) in col_perm.iter().enumerate().take(min_dim) {
        let cost = random_cost(&mut rng);
        let _ = csr.add((row, col, cost));
    }

    // Fill additional random edges.
    let extra = total_edges.saturating_sub(min_dim);
    for _ in 0..extra {
        let row = random_index(&mut rng, n_rows);
        let col = random_index(&mut rng, n_cols);
        let cost = random_cost(&mut rng);
        let _ = csr.add((row, col, cost));
    }

    csr
}

/// Generate a diagonal-dominant cost matrix.
///
/// Diagonal entries have low costs (~0.1), off-diagonal entries have high costs (~5.0).
/// A permutation along the diagonal is embedded for feasibility.
pub fn diagonal_dominant_matrix(seed: u64, n: usize, density: f64) -> SparseCostMatrix {
    let mut rng = XorShift64::from(seed);
    let total_edges = target_edge_count(n, density).max(n);
    let mut csr: SparseCostMatrix = ValuedCSR2D::with_sparse_shaped_capacity((n, n), total_edges);

    // Embed the identity permutation with low costs.
    for i in 0..n {
        // Cost in [0.01, 0.20]
        let raw = rng.next().expect("XorShift64 produces infinite values") % 20 + 1;
        let cost = f64::from(u32::try_from(raw).unwrap()) / 100.0;
        let _ = csr.add((i, i, cost));
    }

    // Fill off-diagonal entries with high costs.
    let extra = total_edges.saturating_sub(n);
    for _ in 0..extra {
        let row = random_index(&mut rng, n);
        let col = random_index(&mut rng, n);
        // High cost in [4.00, 9.99]
        let raw = rng.next().expect("XorShift64 produces infinite values") % 600 + 400;
        let cost = f64::from(u32::try_from(raw).unwrap()) / 100.0;
        let _ = csr.add((row, col, cost));
    }

    csr
}

/// Generate a block-structured cost matrix with `n_blocks` blocks.
///
/// Within-block edges have low costs (~0.5), across-block edges have high costs (~8.0).
/// A permutation is embedded within each block for feasibility.
pub fn block_structured_matrix(
    seed: u64,
    n: usize,
    n_blocks: usize,
    density: f64,
) -> SparseCostMatrix {
    let mut rng = XorShift64::from(seed);
    let total_edges = target_edge_count(n, density).max(n);
    let mut csr: SparseCostMatrix = ValuedCSR2D::with_sparse_shaped_capacity((n, n), total_edges);

    let block_size = n / n_blocks;

    // Embed a permutation within each block for feasibility.
    for b in 0..n_blocks {
        let start = b * block_size;
        let end = if b == n_blocks - 1 {
            n
        } else {
            start + block_size
        };
        let len = end - start;
        let perm = random_permutation(&mut rng, len);
        for (i, &j) in perm.iter().enumerate() {
            // Low within-block cost [0.01, 1.00]
            let raw = rng.next().expect("XorShift64 produces infinite values") % 100 + 1;
            let cost = f64::from(u32::try_from(raw).unwrap()) / 100.0;
            let _ = csr.add((start + i, start + j, cost));
        }
    }

    // Fill additional edges: within-block cheap, across-block expensive.
    let extra = total_edges.saturating_sub(n);
    for _ in 0..extra {
        let row = random_index(&mut rng, n);
        let col = random_index(&mut rng, n);
        let row_block = row / block_size;
        let col_block = col / block_size;
        let cost = if row_block == col_block {
            // Within-block: [0.01, 1.00]
            let raw = rng.next().expect("XorShift64 produces infinite values") % 100 + 1;
            f64::from(u32::try_from(raw).unwrap()) / 100.0
        } else {
            // Across-block: [6.00, 9.99]
            let raw = rng.next().expect("XorShift64 produces infinite values") % 400 + 600;
            f64::from(u32::try_from(raw).unwrap()) / 100.0
        };
        let _ = csr.add((row, col, cost));
    }

    csr
}

/// Generate a near-degenerate cost matrix where all costs are ~5.0 ± 0.01.
///
/// Tests numerical stability and worst-case pivoting behavior.
/// A permutation is embedded for feasibility.
pub fn near_degenerate_matrix(seed: u64, n: usize, density: f64) -> SparseCostMatrix {
    let mut rng = XorShift64::from(seed);
    let total_edges = target_edge_count(n, density).max(n);
    let mut csr: SparseCostMatrix = ValuedCSR2D::with_sparse_shaped_capacity((n, n), total_edges);

    let base_cost = 5.0;

    // Embed a random permutation.
    let perm = random_permutation(&mut rng, n);
    for (row, &col) in perm.iter().enumerate() {
        let noise = rng.next().expect("XorShift64 produces infinite values") % 3;
        let cost = base_cost - 0.01 + f64::from(u32::try_from(noise).unwrap()) * 0.01;
        let _ = csr.add((row, col, cost));
    }

    // Fill additional edges with near-identical costs.
    let extra = total_edges.saturating_sub(n);
    for _ in 0..extra {
        let row = random_index(&mut rng, n);
        let col = random_index(&mut rng, n);
        let noise = rng.next().expect("XorShift64 produces infinite values") % 3;
        let cost = base_cost - 0.01 + f64::from(u32::try_from(noise).unwrap()) * 0.01;
        let _ = csr.add((row, col, cost));
    }

    csr
}

/// Generate a random `n x n` dense cost matrix as a `VecMatrix2D<f64>`.
///
/// Every cell gets a random cost in [0.01, 9.99].
pub fn dense_square_matrix(seed: u64, n: usize) -> DenseCostMatrix {
    let mut rng = XorShift64::from(seed);
    let mut data = Vec::with_capacity(n * n);
    for _ in 0..n * n {
        data.push(random_cost(&mut rng));
    }
    VecMatrix2D::new(n, n, data)
}

/// Compute `max_cost` from a dense matrix.
pub fn compute_dense_max_cost(m: &DenseCostMatrix) -> f64 {
    use geometric_traits::prelude::DenseValuedMatrix;
    m.max_value().unwrap_or(100.0) * 2.0 + 1.0
}

/// Compute `max_cost` from a matrix following the existing convention.
pub fn compute_max_cost(csr: &SparseCostMatrix) -> f64 {
    csr.max_sparse_value().unwrap_or(100.0) * 2.0 + 1.0
}

/// Compute `padding_cost` from `max_cost` following the existing convention.
///
/// Used for SparseLAPJV, SparseHungarian, and Crouse.
pub fn compute_padding(max_cost: f64) -> f64 {
    max_cost * 0.9
}

/// Compute `non_edge_cost` for Jaqaman.
///
/// Jaqaman requires `non_edge_cost / 2 > max_sparse_value` and
/// `non_edge_cost < max_cost`. Since `max_cost = 2 * max_sparse_value + 1`,
/// we use `max_cost - 0.5 = 2 * max_sparse_value + 0.5`, giving
/// `non_edge_cost / 2 = max_sparse_value + 0.25 > max_sparse_value`.
pub fn compute_jaqaman_cost(max_cost: f64) -> f64 {
    max_cost - 0.5
}

/// Format a matrix label for benchmark identification.
pub fn matrix_label(n: usize, density: f64) -> String {
    format!("n={n}_d={density:.2}")
}

/// Format a rectangular matrix label for benchmark identification.
pub fn rect_label(n_rows: usize, n_cols: usize, density: f64) -> String {
    format!("{n_rows}x{n_cols}_d={density:.2}")
}
