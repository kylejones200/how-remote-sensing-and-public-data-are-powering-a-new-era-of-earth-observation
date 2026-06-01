"""Per-tile NDVI summary statistics (mean, std, p95)."""

from __future__ import annotations

import numpy as np


def ndvi_tile_statistics(ndvi: np.ndarray) -> tuple[float, float, float]:
    a = np.asarray(ndvi, dtype=float)
    if len(a) == 0:
        return 0.0, 0.0, 0.0
    n = len(a)
    mean = float(a.sum()) / n
    var = float(((a - mean) ** 2).sum()) / n
    sorted_a = np.sort(a)
    idx = int(np.floor(n * 0.95))
    p95 = float(sorted_a[min(idx, n - 1)])
    return mean, var**0.5, p95
