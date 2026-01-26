# VOID Examples

This directory contains example binaries demonstrating various features of the VOID simulation library.

## Running Examples

```bash
# Run a specific example
cargo run --example hello_void

# Run in release mode for better performance
cargo run --release --example basic_oscillation

# List all examples
cargo run --example 2>&1 | grep "Available"
```

## Available Examples

### 1. `hello_void.rs` ✅
**Your first simulation**

- Creates a minimal 8³ lattice
- Demonstrates basic workflow
- Verifies energy conservation
- Good starting point for new users

```bash
cargo run --example hello_void
```

### 2. `basic_oscillation.rs` 🚧 TODO
**Intra-cell energy oscillations**

- Single cell with non-zero redistribution matrix
- Tracks energy exchange between variables
- Extracts oscillation frequencies
- Verifies exact periodicity

**What you'll learn:**
- How to configure redistribution matrices
- Eigenmode analysis
- Energy conservation in isolated systems

### 3. `spatial_propagation.rs` 🚧 TODO
**Energy diffusion on 1D chain**

- 1D lattice (N×1×1) with one hot spot
- Watch energy diffuse along chain
- Compare to analytical solution
- Visualize with ASCII art

**What you'll learn:**
- Spatial transport mechanics
- Conservative exchange between cells
- Diffusion timescales

### 4. `permutation_search.rs` 🚧 TODO
**Parameter space exploration**

- Sweep redistribution matrix entries
- Compute pattern metrics for each config
- Export results to Parquet
- Identify configurations that produce cosmic web structures

**What you'll learn:**
- Automated parameter sweeps
- Pattern metric computation
- Data export for analysis

### 5. `cosmic_web_demo.rs` 🚧 TODO
**Generate void/filament structures**

- 32³ lattice with structured initialization
- Evolve until patterns emerge
- Export snapshots at multiple timesteps
- Compute void fraction, filament fraction, power spectrum

**What you'll learn:**
- Initialization strategies
- Long-term evolution
- Pattern metric interpretation
- Data visualization workflow

### 6. `benchmark.rs` 🚧 TODO
**Performance profiling**

- Benchmark different lattice sizes
- Test parallel vs serial execution
- Profile hot paths
- Generate flamegraph data

**What you'll learn:**
- Performance characteristics
- Scaling behavior
- Optimization opportunities

## Implementation Priority

1. ✅ `hello_void.rs` (complete)
2. ⚡ `basic_oscillation.rs` (next)
3. ⚡ `spatial_propagation.rs` (next)
4. 🔒 `permutation_search.rs` (blocked: needs pattern metrics)
5. 🔒 `cosmic_web_demo.rs` (blocked: needs visualization)
6. 🔒 `benchmark.rs` (blocked: needs full implementation)

## Contributing Examples

When adding a new example:

1. Create `examples/your_example.rs`
2. Add doc comments explaining the purpose
3. Include inline comments for key steps
4. Add entry to this README
5. Test with `cargo run --example your_example`

## Tips

- Use `--release` for realistic performance
- Start with small lattices (8³-16³) for testing
- Check `cargo doc --open` for API details
- See `docs/GETTING_STARTED.md` for tutorials
