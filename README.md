# Remote Sensing and Public Data Earth Observation

This project demonstrates remote sensing analysis using public earth observation data.

## Business context

From orbit, Barcelona is surprising green. This is the view of Sentinel-2 sensors; allowing us to see what we normally couldn't. The Copernicus program delivers multi-spectral imagery that allows us to observe, quantify, and understand cities like Barcelona in unprecedented detail.

Europe's commitment to open data means access to earth observation data is no longer the limiting factor (at least at some level of aggregation).

On February 5, 2025, two Sentinel-2 images captured Barcelona and its surrounding metropolitan region. One uses the Normalized Difference Vegetation Index (NDVI) to visualize plant health. The other presents a false-color composite using infrared light to highlight vegetation and urban features.

## Article

Medium article: [How Remote Sensing and Public Data are Powering a New Era of Earth Observation](https://medium.com/@kylejones_47003/how-remote-sensing-and-public-data-are-powering-a-new-era-of-earth-observation-8002f2b1e04d)

## Project Structure

```
.
├── README.md           # This file
├── main.py            # Main entry point
├── config.yaml        # Configuration file
├── requirements.txt   # Python dependencies
├── src/               # Core functions
│   ├── core.py        # Remote sensing functions
│   └── plotting.py    # Tufte-style plotting utilities
├── tests/             # Unit tests
├── data/              # Data files
├── images/            # Generated plots and figures
├── rust/                   # Rust port (core + PyO3 + CLI bench)
├── benchmark_rust.py       # Python vs Rust benchmark
├── src/compute_kernel.py   # Python/numpy reference kernel
```

## Configuration

Edit `config.yaml` to customize:
- Data source or synthetic generation
- Number of spectral bands
- NDVI calculation options
- Output settings

## Remote Sensing Features

Analysis includes:
- Multi-spectral bands: Red, Green, Blue, NIR, SWIR
- NDVI calculation: Vegetation index
- Time series analysis: Temporal patterns
- Public data integration: Earth observation datasets

## Caveats

- By default, generates synthetic satellite data.
- Real remote sensing data available from USGS, NASA, ESA.
- NDVI requires Red and NIR bands.

## Rust performance port

Side-by-side **Python vs Rust** implementation of the numeric hot loop — NDVI tile statistics. Reference PyO3 benchmark: **see `benchmark_rust.py`** on a release build (local machine; run `benchmark_rust.py` to reproduce).

| Path | Role |
|------|------|
| `src/compute_kernel.py` | Python/numpy reference kernel |
| `rust/core/` | Pure Rust library |
| `rust/py/` | PyO3 bindings |
| `rust/bench/` | Standalone CLI benchmark |
| `benchmark_rust.py` | Python vs Rust timing + correctness check |

```bash
# Rust-only CLI benchmark
cd rust && cargo run --release -p how_remote_sensing_and_public_data_are_powering_a_new_era_of_earth_observation_bench

# Python vs Rust (PyO3)
pip install maturin numpy
maturin develop --release -m rust/py/Cargo.toml
python benchmark_rust.py
```

Python ML training, solvers, and orchestration stay in Python; Rust targets the numeric hot loops. Stochastic generators validate output shapes; deterministic kernels match at tight floating-point tolerance.


## Disclaimer

Educational/demo code only. Not financial, safety, or engineering advice. Use at your own risk. Verify results independently before any production or operational use.

## License

MIT — see [LICENSE](LICENSE).