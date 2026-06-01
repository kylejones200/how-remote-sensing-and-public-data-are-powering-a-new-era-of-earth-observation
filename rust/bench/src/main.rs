use how_remote_sensing_and_public_data_are_powering_a_new_era_of_earth_observation_core::ndvi_tile_statistics;

fn main() {
    let ndvi: Vec<f64> = (0..4096).map(|i| 0.2 + (i as f64 * 0.001).sin() * 0.3).collect();
    for _ in 0..5000 {
        let _ = ndvi_tile_statistics(&ndvi);
    }
}
