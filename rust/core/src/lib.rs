//! Per-tile NDVI summary statistics (mean, std, p95).

pub fn ndvi_tile_statistics(ndvi: &[f64]) -> (f64, f64, f64) {
    if ndvi.is_empty() {
        return (0.0, 0.0, 0.0);
    }
    let n = ndvi.len() as f64;
    let mean = ndvi.iter().sum::<f64>() / n;
    let var = ndvi.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n;
    let mut sorted = ndvi.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let idx = ((sorted.len() as f64) * 0.95).floor() as usize;
    let p95 = sorted[idx.min(sorted.len() - 1)];
    (mean, var.sqrt(), p95)
}
