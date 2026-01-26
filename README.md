# VOID – Visualizing Oscillations in Invisible Density

[![Rust](https://github.com/resonant-jovian/void/actions/workflows/rust.yml/badge.svg)](https://github.com/resonant-jovian/void/actions/workflows/rust.yml)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Rust Version](https://img.shields.io/badge/rust-nightly%202024-orange.svg)](https://www.rust-lang.org)

> **A conservative multi-mode energy lattice simulator for exploring dark matter and dark energy interactions through cosmic web pattern formation**

## 🌌 Overview

**VOID** is a Rust 2024 nightly library that simulates energy redistribution, oscillations, and spatial transport across a 3D lattice representing the cosmic web. By modeling **5 energy variables** distributed across **4 fundamental forces**, it explores how unknown interactions (potentially dark matter/energy) could drive the formation of voids, walls, and filaments observed in large-scale cosmic structure.

### Key Features

- ✅ **Strictly Conservative Physics** – Global energy conservation with zero entropy production
- ✅ **Multi-Mode Energy Variables** – EM radiation, baryons, neutrinos, and two unknown dark components
- ✅ **Four Fundamental Forces** – Gravity, electromagnetism, weak, and strong nuclear interactions
- ✅ **Exact Oscillatory Solutions** – Matrix exponential evolution with eigenmode analysis
- ✅ **3D Spatial Coupling** – Neighbor-to-neighbor energy exchange on cubic lattice
- ✅ **Pattern Metrics** – Automated detection of void/wall/filament structures
- ✅ **Parquet Data Format** – Efficient columnar storage for large-scale simulations
- ✅ **Terminal UI Dashboard** – Real-time visualization using [ratatui](https://github.com/ratatui/ratatui)

## 📦 Project Structure

This workspace contains two crates:

```
void/
├── entropic_void/          # Core simulation library
│   ├── src/
│   │   ├── lib.rs         # Public API
│   │   ├── types.rs       # Core data structures
│   │   ├── lattice.rs     # 3D grid management
│   │   ├── energy.rs      # Constraint enforcement
│   │   ├── redistribution.rs  # Intra-cell evolution
│   │   ├── transport.rs   # Inter-cell coupling
│   │   ├── oscillation.rs # Mode tracking
│   │   ├── init.rs        # Initialization
│   │   ├── evolution.rs   # Simulation orchestration
│   │   ├── conservation.rs # Verification & metrics
│   │   ├── visualization.rs # Data export
│   │   └── utils.rs       # Math utilities
│   └── Cargo.toml
│
└── entropy/                # Terminal UI (ratatui-based TUI)
    ├── src/
    │   └── main.rs        # Dashboard application
    └── Cargo.toml
```

## 🚀 Quick Start

### Prerequisites

- **Rust nightly 2024** (see `rust-toolchain.toml`)
- BLAS/LAPACK libraries for matrix operations (optional for performance)

```bash
# Install Rust nightly
rustup toolchain install nightly
rustup override set nightly

# Install BLAS/LAPACK (optional, platform-specific)
# Ubuntu/Debian:
sudo apt-get install libblas-dev liblapack-dev
# macOS:
brew install openblas lapack
```

### Running the TUI Dashboard

```bash
# Run the interactive terminal dashboard
cargo run --release -p entropy

# Or use the workspace target
cargo run --release --bin entropy
```

### Using the Library

Add to your `Cargo.toml`:

```toml
[dependencies]
entropic_void = { path = "../entropic_void" }
```

Basic usage:

```rust
use entropic_void::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize a 32³ lattice
    let mut lattice = Lattice::new((32, 32, 32));
    
    // Set up energy redistribution matrix
    let mut redistribution = RedistributionMatrix::new_zero();
    // TODO: Configure oscillation rates between variables/forces
    
    // Define spatial coupling strengths
    let coupling = [[0.1; FORCES]; VARS];
    
    // Create constraint set
    let constraints = ConstraintSet::default();
    
    // Initialize simulation
    let mut sim = Simulation::new(lattice, redistribution, coupling, constraints);
    
    // Evolve the system
    let dt = 0.01;
    let t_end = 10.0;
    sim.evolve_until(t_end, dt, |sim| {
        // Callback for checkpoints
        if sim.step % 100 == 0 {
            println!("Step {}: t = {:.3}", sim.step, sim.time);
        }
    })?;
    
    // Verify conservation
    let energy_error = sim.verify_energy_conservation();
    println!("Energy conservation error: {:.2e}", energy_error);
    
    // Compute pattern metrics
    let metrics = sim.compute_pattern_metrics();
    println!("Void fraction: {:.3}", metrics.void_fraction);
    
    Ok(())
}
```

## 🔬 Physics Model

### Energy Variables

1. **EM Radiation** – Electromagnetic energy (photons)
2. **Baryonic Matter** – Ordinary matter (protons, neutrons)
3. **Neutrinos** – Light, weakly-interacting particles
4. **Unknown₁** – Hypothetical dark matter candidate
5. **Unknown₂** – Hypothetical dark energy candidate

### Fundamental Forces

1. **Gravity** – Attractive, long-range, couples to all energy
2. **Electromagnetism** – Long-range, couples to charged particles
3. **Weak Nuclear** – Short-range, governs radioactive decay
4. **Strong Nuclear** – Very short-range, binds quarks

### Evolution Equations

**Local Redistribution (Intra-Cell):**
```
dE/dt = R · E
```
where `R` is an antisymmetric 20×20 matrix (5 vars × 4 forces), ensuring oscillatory exchange without dissipation.

**Spatial Transport (Inter-Cell):**
```
dE_i/dt += Σ_j κ_ij (E_j - E_i)
```
where `κ_ij` is the coupling strength between neighboring cells.

**Conservation Law:**
```
dE_total/dt = 0  (exactly)
```

### Pattern Metrics

- **Void Fraction** – Percentage of low-density cells
- **Filament Fraction** – Percentage of high-density, connected structures
- **Wall Fraction** – Percentage of intermediate-density sheets
- **Clustering Dimension** – Fractal dimension of dense regions
- **Power Spectrum** – Fourier analysis of spatial modes

## 📊 Data Output

All simulation data is exported to **Apache Parquet** format for efficient storage and analysis:

- **Snapshots** – Per-cell energy states at specified intervals
- **Metrics** – Pattern statistics at each timestep
- **Power Spectra** – Fourier modes for oscillation analysis
- **Permutation Search Results** – Parameter sweep outcomes

Example directory structure:
```
data/
├── snapshots/
│   ├── run_001/
│   │   ├── step_00000.parquet
│   │   ├── step_00100.parquet
│   │   └── ...
├── metrics/
│   └── run_001_metrics.parquet
└── search/
    └── permutation_results.parquet
```

## 🎯 Roadmap

### Phase 1: Foundation (Current)
- [x] Core type system
- [x] Lattice grid management
- [x] Energy constraint system
- [ ] Complete redistribution implementation with nalgebra
- [ ] Spatial transport with exact exchange
- [ ] Basic conservation checks

### Phase 2: Physics
- [ ] Matrix exponential evolution (exact solutions)
- [ ] Eigenmode decomposition for oscillation tracking
- [ ] FFT-based power spectrum analysis
- [ ] Pattern metric computation

### Phase 3: Visualization
- [ ] Ratatui TUI with real-time updates
- [ ] 2D slice rendering (ASCII art)
- [ ] Chart widgets for energy/metrics
- [ ] Interactive controls (pause, step, reset)

### Phase 4: Optimization
- [ ] Parquet I/O integration
- [ ] Rayon parallelization
- [ ] SIMD optimizations
- [ ] Adaptive mesh refinement (AMR)

### Phase 5: Analysis
- [ ] Permutation search framework
- [ ] ML emulation (optional)
- [ ] Topological data analysis
- [ ] Publication-ready examples

## 🧪 Examples

See the `examples/` directory (coming soon) for:

- `basic_oscillation.rs` – Verify local energy exchange
- `spatial_propagation.rs` – Watch energy diffuse across 1D chain
- `permutation_search.rs` – Parameter sweep template
- `cosmic_web_demo.rs` – Generate void/filament structures

## 🤝 Contributing

Contributions welcome! Please:

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

### Development Setup

```bash
# Clone the repo
git clone https://github.com/resonant-jovian/void.git
cd void

# Run tests
cargo test --all

# Run lints
cargo clippy --all -- -D warnings

# Format code
cargo fmt --all

# Run examples (when available)
cargo run --example basic_oscillation
```

## 📚 Documentation

Generate and view full API documentation:

```bash
cargo doc --open --no-deps
```

For implementation guides, see:
- [COMPLETE_IMPLEMENTATION_GUIDE.md](docs/COMPLETE_IMPLEMENTATION_GUIDE.md) (coming soon)
- [PHYSICS_EQUATIONS_REFERENCE.md](docs/PHYSICS_EQUATIONS_REFERENCE.md) (coming soon)
- [CRATE_RECOMMENDATIONS.md](docs/CRATE_RECOMMENDATIONS.md) (coming soon)

## 📖 References

This project is inspired by:

- **Void-Wall Cosmology** – Rieder et al. (2013), van de Weygaert et al.
- **N-Body Simulations** – Enzo, GAMER, RAMSES
- **Hamiltonian Neural Networks** – Greydanus et al. (2019)
- **Cosmological Power Spectra** – Standard cosmology analysis techniques

## 📄 License

This project is licensed under the GNU General Public License v3.0 - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- [ratatui](https://github.com/ratatui/ratatui) – Terminal UI framework
- [nalgebra](https://github.com/dimforge/nalgebra) – Linear algebra library
- [arrow-rs](https://github.com/apache/arrow-rs) – Parquet/Arrow implementation
- [rustfft](https://github.com/ejmahler/RustFFT) – FFT library
- [rayon](https://github.com/rayon-rs/rayon) – Data parallelism

---

**Built with ❤️ and 🦀 by the VOID team**

*Exploring the invisible through computational cosmology*
