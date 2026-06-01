use how_remote_sensing_and_public_data_are_powering_a_new_era_of_earth_observation_core::ndvi_tile_statistics;
use numpy::PyReadonlyArray1;
use pyo3::prelude::*;

#[pyfunction]
fn ndvi_tile_statistics_py(ndvi: PyReadonlyArray1<f64>) -> PyResult<(f64, f64, f64)> {
    Ok(ndvi_tile_statistics(ndvi.as_slice()?))
}

#[pyfunction]
#[pyo3(signature = (ndvi, iterations=5_000))]
fn bench_kernel_py(ndvi: PyReadonlyArray1<f64>, iterations: usize) -> PyResult<f64> {
    let buf = ndvi.as_slice()?.to_vec();
    let start = std::time::Instant::now();
    for _ in 0..iterations {
        let _ = ndvi_tile_statistics(&buf);
    }
    Ok(start.elapsed().as_secs_f64())
}

#[pymodule]
fn how_remote_sensing_and_public_data_are_powering_a_new_era_of_earth_observation_rs(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ndvi_tile_statistics_py, m)?)?;
    m.add_function(wrap_pyfunction!(bench_kernel_py, m)?)?;
    Ok(())
}
