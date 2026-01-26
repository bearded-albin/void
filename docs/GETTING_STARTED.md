# Getting Started with VOID

This guide will walk you through installing, running, and understanding your first VOID simulation.

## Prerequisites

- **Rust nightly 2024** (see [rust-toolchain.toml](../rust-toolchain.toml))
- **BLAS/LAPACK** libraries (optional, for performance)

### Installing Rust Nightly

```bash
# Install rustup if you haven't already
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install nightly toolchain
rustup toolchain install nightly

# Set nightly as default (or use rustup override)
rustup default nightly
```

### Installing BLAS/LAPACK (Optional)

**Ubuntu/Debian:**
```bash
sudo apt-get install libblas-dev liblapack-dev
```

**macOS:**
```bash
brew install openblas lapack
```

**Windows:**
```powershell
# Using vcpkg
vcpkg install openblas:x64-windows
```

## Installation

### Clone the Repository

```bash
git clone https://github.com/resonant-jovian/void.git
cd void
```

### Build the Project

```bash
# Build all workspace members
cargo build --release --all

# Or build just the library
cargo build --release -p entropic_void

# Or build just the TUI
cargo build --release -p entropy
```

## Your First Simulation

### Option 1: Using the Library

Create a new binary:

```bash
mkdir -p examples
touch examples/hello_void.rs
```

Add this code to `examples/hello_void.rs`:

```rust
use entropic_void::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing VOID simulation...");
    
    // Create a small 8³ lattice
    let mut lattice = Lattice::new((8, 8, 8));
    println!("Lattice size: {:?}", lattice.size());
    
    // Set up a simple redistribution matrix (zero for now)
    let redistribution = RedistributionMatrix::new_zero();
    
    // Define spatial coupling (weak)
    let coupling = [[0.01; FORCES]; VARS];
    
    // Use default constraints
    let constraints = ConstraintSet::default();
    
    // Create simulation
    let mut sim = Simulation::new(lattice, redistribution, coupling, constraints);
    println!("Simulation initialized at t = {}", sim.time);
    
    // Run for 100 steps
    println!("\nEvolving simulation...");
    let dt = 0.01;
    for step in 0..100 {
        sim.step(dt, false)?;
        
        if step % 10 == 0 {
            println!("Step {}: t = {:.3}", sim.step, sim.time);
        }
    }
    
    println!("\nSimulation complete!");
    println!("Final time: {:.3}", sim.time);
    println!("Total steps: {}", sim.step);
    
    Ok(())
}
```

Run it:

```bash
cargo run --example hello_void
```

### Option 2: Using the TUI Dashboard

Run the interactive terminal interface:

```bash
cargo run --release -p entropy
```

**Controls:**
- `Space`: Pause/Resume
- `s`: Single step
- `[/]`: Adjust timestep
- `↑↓`: Navigate Z-slices
- `←→`: Cycle energy variables
- `q`: Quit

## Understanding the Output

### Energy Conservation

VOID simulations strictly conserve total energy. You can verify this:

```rust
let initial_energy = /* compute from lattice */;
sim.evolve_until(10.0, 0.01, |_| {})?;
let final_energy = /* compute again */;

assert!((initial_energy - final_energy).abs() < 1e-9);
```

### Pattern Metrics

Compute cosmic web statistics:

```rust
let metrics = sim.compute_pattern_metrics();
println!("Void fraction: {:.3}", metrics.void_fraction);
println!("Filament fraction: {:.3}", metrics.filament_fraction);
```

## Next Steps

### Customize Your Simulation

1. **Modify energy variables:** Adjust initial distributions
2. **Configure oscillations:** Set up redistribution matrix
3. **Tune coupling:** Change spatial transport rates
4. **Add constraints:** Fix energy ratios or totals

### Explore Examples

Once examples are implemented:

```bash
cargo run --example basic_oscillation
cargo run --example spatial_propagation
cargo run --example cosmic_web_demo
```

### Read the Documentation

- [Physics Reference](PHYSICS_REFERENCE.md) - Understand the equations
- [Development Guide](DEVELOPMENT.md) - Implement new features
- [API Docs](../target/doc/entropic_void/index.html) - Function references

## Troubleshooting

### Build Errors

**Problem:** `error: linker 'cc' not found`

**Solution:** Install a C compiler:
```bash
# Ubuntu/Debian
sudo apt-get install build-essential

# macOS
xcode-select --install
```

**Problem:** `undefined reference to 'dgeev_'` (LAPACK)

**Solution:** Disable LAPACK feature:
```bash
cargo build --no-default-features
```

### Runtime Issues

**Problem:** Simulation crashes or produces NaN

**Solution:** Check timestep is not too large:
```rust
let dt = 0.001; // Use smaller timestep
```

**Problem:** TUI doesn't render correctly

**Solution:** Ensure terminal supports 256 colors:
```bash
export TERM=xterm-256color
```

### Performance

**Problem:** Simulation is slow

**Solution:** 
1. Use release mode: `cargo build --release`
2. Enable LAPACK: Ensure BLAS/LAPACK installed
3. Reduce lattice size for testing
4. Profile with `cargo flamegraph`

## Community

- **Issues:** Report bugs on [GitHub Issues](https://github.com/resonant-jovian/void/issues)
- **Discussions:** Ask questions in [GitHub Discussions](https://github.com/resonant-jovian/void/discussions)
- **Contributing:** See [DEVELOPMENT.md](DEVELOPMENT.md)

## License

GNU General Public License v3.0 - see [LICENSE](../LICENSE)
