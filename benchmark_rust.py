#!/usr/bin/env python3
"""Python vs Rust kernel benchmark."""

from __future__ import annotations

import time
import sys
from pathlib import Path

import numpy as np

ROOT = Path(__file__).resolve().parent
sys.path.insert(0, str(ROOT / "src"))
from compute_kernel import ndvi_tile_statistics  # noqa: E402

def main() -> None:
    ndvi = np.ascontiguousarray(0.2 + np.sin(np.arange(4096) * 0.001) * 0.3)
    t0 = time.perf_counter()
    for _ in range(200):
        ndvi_tile_statistics(ndvi)
    py_s = time.perf_counter() - t0
    try:
        import how_remote_sensing_and_public_data_are_powering_a_new_era_of_earth_observation_rs as rs
    except ImportError:
        print("Build: maturin develop --release -m rust/py/Cargo.toml")
        print(f"Python {py_s:.3f}s")
        return
    rs_s = rs.bench_kernel_py(ndvi, 5000)
    print(f"Python {py_s:.3f}s Rust {rs_s:.3f}s speedup {py_s / max(rs_s, 1e-9):.1f}x")
    py = ndvi_tile_statistics(ndvi)
    rs_out = rs.ndvi_tile_statistics_py(ndvi)
    for i in range(3):
        np.testing.assert_allclose(py[i], rs_out[i], rtol=1e-10)
    print("Correctness: OK")

if __name__ == "__main__":
    main()
