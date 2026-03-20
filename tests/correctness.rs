//! Correctness validation: verify that all algorithms produce assignments
//! with the same total cost on the same input matrix.

use geometric_traits::prelude::*;
use weighted_matching_benchmark::{
    compute_max_cost, compute_padding, rectangular_sparse_matrix, sparse_square_matrix,
};

/// Sum the costs of an assignment given the sparse matrix.
fn assignment_cost(
    csr: &weighted_matching_benchmark::SparseCostMatrix,
    assignment: &[(usize, usize)],
) -> f64 {
    assignment
        .iter()
        .map(|&(row, col)| {
            csr.sparse_row(row)
                .zip(csr.sparse_row_values(row))
                .find(|&(c, _)| c == col)
                .map(|(_, v)| v)
                .expect("assigned pair must exist as an edge")
        })
        .sum()
}

#[test]
fn sparse_algorithms_agree_on_cost() {
    for &(n, density) in &[(20, 0.20), (50, 0.10), (100, 0.05)] {
        let csr = sparse_square_matrix(42, n, density);
        let max_cost = compute_max_cost(&csr);
        let padding = compute_padding(max_cost);

        let lapmod_result = csr.lapmod(max_cost).expect("LAPMOD should succeed");
        let sparse_lapjv_result = csr
            .sparse_lapjv(padding, max_cost)
            .expect("SparseLAPJV should succeed");
        let sparse_hungarian_result = csr
            .sparse_hungarian(padding, max_cost)
            .expect("SparseHungarian should succeed");

        let cost_lapmod = assignment_cost(&csr, &lapmod_result);
        let cost_lapjv = assignment_cost(&csr, &sparse_lapjv_result);
        let cost_hungarian = assignment_cost(&csr, &sparse_hungarian_result);

        assert!(
            (cost_lapmod - cost_lapjv).abs() < 1e-6,
            "LAPMOD and SparseLAPJV disagree on cost at n={n}, d={density}: {cost_lapmod} vs {cost_lapjv}"
        );
        assert!(
            (cost_lapmod - cost_hungarian).abs() < 1e-6,
            "LAPMOD and SparseHungarian disagree on cost at n={n}, d={density}: {cost_lapmod} vs {cost_hungarian}"
        );
    }
}

#[test]
fn crouse_produces_valid_assignment() {
    for &(nr, nc) in &[(20, 30), (50, 100), (30, 20)] {
        let csr = rectangular_sparse_matrix(42, nr, nc, 0.20);
        let max_cost = compute_max_cost(&csr);
        let non_edge_cost = compute_padding(max_cost);

        let result = csr
            .crouse(non_edge_cost, max_cost)
            .expect("Crouse should succeed");

        // Verify all assigned pairs are valid edges and no row/col is repeated.
        let mut used_rows = std::collections::HashSet::new();
        let mut used_cols = std::collections::HashSet::new();
        for &(row, col) in &result {
            assert!(row < nr, "row {row} out of bounds for {nr}x{nc} matrix");
            assert!(col < nc, "col {col} out of bounds for {nr}x{nc} matrix");
            assert!(used_rows.insert(row), "row {row} assigned twice");
            assert!(used_cols.insert(col), "col {col} assigned twice");
        }
    }
}
