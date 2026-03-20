# /// script
# requires-python = ">=3.11"
# dependencies = ["matplotlib"]
# ///
"""Generate benchmark summary plot (two panels: sparse + dense scaling)."""

import matplotlib.pyplot as plt

plt.rcParams["svg.fonttype"] = "none"

# --- Data (transcribed from benchmark results) ---

# Left panel: Sparse Size Scaling (5% density)
sparse_ns = [20, 50, 100, 200, 500, 1000, 2000, 3000, 5000, 10000]

lapmod_sparse = [0.423, 0.939, 2.11, 3.69, 8.92, 17.2, 35.3, 52.1, 90.9, 181.0]
sparse_lapjv = [
    1.89,
    11.0,
    39.7,
    150.0,
    843.0,
    3470.0,
    13600.0,
    29700.0,
    84600.0,
    333000.0,
]
sparse_hungarian = [
    4.23,
    29.3,
    91.6,
    364.0,
    2330.0,
    11200.0,
    37100.0,
    75300.0,
    201000.0,
    804000.0,
]
crouse_sparse = [
    1.98,
    7.57,
    24.8,
    84.6,
    513.0,
    1980.0,
    9730.0,
    56000.0,
    151000.0,
    582000.0,
]
jaqaman_sparse = [1.69, 4.86, 9.71, 19.0, 46.9, 92.1, 188.0, 284.0, 492.0, 1053.0]

# Right panel: Dense Size Scaling (100% density)
dense_ns = [20, 50, 100, 200, 300, 400, 500, 750, 1000]

lapmod_dense = [0.379, 0.866, 1.76, 3.32, 4.82, 6.45, 8.94, 11.9, 15.6]
lapjv_dense = [1.50, 12.2, 54.3, 228.0, 655.0, 17200.0, 1880.0, 4490.0, 168000.0]
hungarian_dense = [1.92, 16.0, 156.0, 696.0, 1890.0, 3390.0, 6290.0, 12700.0, 20500.0]

# --- Colors ---
COLOR_LAPMOD = "#2E8B57"
COLOR_LAPJV = "#4169E1"
COLOR_HUNGARIAN = "#DC143C"
COLOR_CROUSE = "#FF8C00"
COLOR_JAQAMAN = "#9932CC"

# --- Plot ---
fig, (ax_left, ax_right) = plt.subplots(1, 2, figsize=(14, 5.5), sharey=True)

# Left panel: Sparse
ax_left.plot(
    sparse_ns,
    lapmod_sparse,
    "o-",
    color=COLOR_LAPMOD,
    label="LAPMOD",
    markersize=6,
    linewidth=2,
)
ax_left.plot(
    sparse_ns,
    sparse_lapjv,
    "s-",
    color=COLOR_LAPJV,
    label="SparseLAPJV",
    markersize=6,
    linewidth=2,
)
ax_left.plot(
    sparse_ns,
    sparse_hungarian,
    "^-",
    color=COLOR_HUNGARIAN,
    label="SparseHungarian",
    markersize=6,
    linewidth=2,
)
ax_left.plot(
    sparse_ns,
    crouse_sparse,
    "D-",
    color=COLOR_CROUSE,
    label="Crouse (rect.)",
    markersize=6,
    linewidth=2,
)
ax_left.plot(
    sparse_ns,
    jaqaman_sparse,
    "P-",
    color=COLOR_JAQAMAN,
    label="Jaqaman (rect.)",
    markersize=6,
    linewidth=2,
)

ax_left.set_xscale("log")
ax_left.set_yscale("log")
ax_left.set_xlabel("Matrix size (n)")
ax_left.set_ylabel("Time (\u00b5s)")
ax_left.set_title("Sparse Size Scaling (5% density)")
ax_left.legend()
ax_left.grid(True, which="both", ls=":", alpha=0.5)

# Right panel: Dense
ax_right.plot(
    dense_ns,
    lapmod_dense,
    "o-",
    color=COLOR_LAPMOD,
    label="LAPMOD",
    markersize=6,
    linewidth=2,
)
ax_right.plot(
    dense_ns,
    lapjv_dense,
    "s-",
    color=COLOR_LAPJV,
    label="LAPJV",
    markersize=6,
    linewidth=2,
)
ax_right.plot(
    dense_ns,
    hungarian_dense,
    "^-",
    color=COLOR_HUNGARIAN,
    label="Hungarian",
    markersize=6,
    linewidth=2,
)

ax_right.set_xscale("log")
ax_right.set_yscale("log")
ax_right.set_xlabel("Matrix size (n)")
ax_right.set_title("Dense Size Scaling (100% density)")
ax_right.legend()
ax_right.grid(True, which="both", ls=":", alpha=0.5)

fig.tight_layout()
fig.savefig("docs/benchmark_summary.svg", bbox_inches="tight")
print("Wrote docs/benchmark_summary.svg")
