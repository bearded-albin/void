# Development Guide

## Overview

This guide provides detailed instructions for implementing the VOID simulation library.
Follow the modules in order, building up from foundational types to complete physics.

## Implementation Order

### Phase 1: Foundation (Day 1, ~8 hours)

#### 1.1. Complete `types.rs` (2 hours)

**Current Status:** Basic structure exists

**TODO:**
- [ ] Add comprehensive doc comments to all types
- [ ] Implement `Default` for all constraint types
- [ ] Add validation methods (e.g., `is_valid()` for `CellState`)
- [ ] Implement `Display` for debugging
- [ ] Add helper methods for index conversions

**Example:**
```rust
impl CellState {
    /// Validate that all energies are non-negative and finite
    pub fn is_valid(&self) -> bool {
        self.e.iter().all(|row| {
            row.iter().all(|&val| val.is_finite() && val >= 0.0)
        })
    }
    
    /// Get total energy across all variables and forces
    pub fn total(&self) -> f64 {
        self.e.iter().flat_map(|row| row.iter()).sum()
    }
}
```

#### 1.2. Complete `lattice.rs` (2 hours)

**Current Status:** Basic grid exists

**TODO:**
- [ ] Add periodic boundary condition support
- [ ] Implement efficient neighbor iteration
- [ ] Add methods for 26-neighbor connectivity (optional)
- [ ] Create iterator adapters for parallel access
- [ ] Add visualization helpers (2D slices)

**Example:**
```rust
impl Lattice {
    /// Get periodic coordinates (wrapping at boundaries)
    pub fn periodic_coord(&self, coord: LatticeCoord) -> LatticeCoord {
        let (sx, sy, sz) = self.size;
        LatticeCoord {
            x: coord.x % sx,
            y: coord.y % sy,
            z: coord.z % sz,
        }
    }
    
    /// Extract 2D slice at given Z coordinate
    pub fn slice_xy(&self, z: usize) -> Vec<Vec<f64>> {
        // Return 2D grid of total energies
        todo!("Extract slice for visualization")
    }
}
```

#### 1.3. Complete `energy.rs` (2 hours)

**Current Status:** Basic aggregation exists

**TODO:**
- [ ] Implement all constraint enforcement functions
- [ ] Add projection algorithms for `FixedTotal` and `FixedRatio`
- [ ] Create validation suite for constraint violations
- [ ] Add helper for computing "charges" (conserved quantities)

**Example:**
```rust
pub fn project_energy(cell: &mut CellState, constraints: &ConstraintSet) {
    // Step 1: Apply expression constraints (force percentages)
    for (var_i, constraint) in constraints.expr_constraints.iter().enumerate() {
        if constraint.locked {
            let var_total: f64 = cell.e[var_i].iter().sum();
            for (force_f, &pct) in constraint.force_pct.iter().enumerate() {
                cell.e[var_i][force_f] = var_total * pct;
            }
        }
    }
    
    // Step 2: Apply variable constraints (total energy per variable)
    for (var_i, constraint) in constraints.var_constraints.iter().enumerate() {
        match constraint {
            VariableConstraint::FixedTotal(target) => {
                let current: f64 = cell.e[var_i].iter().sum();
                let scale = target / current;
                for force_f in 0..FORCES {
                    cell.e[var_i][force_f] *= scale;
                }
            }
            VariableConstraint::FixedRatio(ratios) => {
                // Implement ratio-based projection
                todo!("Scale variables to maintain ratios")
            }
            VariableConstraint::Free => {} // No action
        }
    }
}
```

#### 1.4. Implement `utils.rs` stubs (2 hours)

**TODO:**
- [ ] Create `matrix_ops` module with basic operations
- [ ] Implement `sampling` module for simplex sampling
- [ ] Add `fft` module stubs (implement in Phase 2)
- [ ] Create test suite for each utility

**Example:**
```rust
pub mod matrix_ops {
    /// Multiply two NxN matrices
    pub fn multiply<const N: usize>(a: &[[f64; N]; N], b: &[[f64; N]; N]) -> [[f64; N]; N] {
        let mut result = [[0.0; N]; N];
        for i in 0..N {
            for j in 0..N {
                for k in 0..N {
                    result[i][j] += a[i][k] * b[k][j];
                }
            }
        }
        result
    }
    
    /// Check if matrix is antisymmetric (A = -A^T)
    pub fn is_antisymmetric<const N: usize>(a: &[[f64; N]; N], tol: f64) -> bool {
        for i in 0..N {
            for j in 0..N {
                if (a[i][j] + a[j][i]).abs() > tol {
                    return false;
                }
            }
        }
        true
    }
}
```

### Phase 2: Physics Core (Day 2, ~8 hours)

#### 2.1. Complete `redistribution.rs` (4 hours)

**TODO:**
- [ ] Implement matrix exponential using Taylor/Padé approximation
- [ ] Add `evolve_exact()` for antisymmetric evolution
- [ ] Implement `extract_oscillation_modes()` using eigendecomposition
- [ ] Add builder pattern for constructing redistribution matrices
- [ ] Create validation suite ensuring antisymmetry

**Key Implementation:**
```rust
pub fn evolve_exact(cell: &mut CellState, matrix: &RedistributionMatrix, dt: f64) {
    // Flatten cell state to vector
    let mut state_vec = [0.0; N_FLATTENED];
    for var_i in 0..VARS {
        for force_f in 0..FORCES {
            state_vec[var_i * FORCES + force_f] = cell.e[var_i][force_f];
        }
    }
    
    // Compute exp(R*dt) * state_vec using utils::matrix_ops::exponential
    let exp_matrix = utils::matrix_ops::exponential(&matrix.a, dt, 10);
    let evolved = utils::matrix_ops::multiply_vec(&exp_matrix, &state_vec);
    
    // Unflatten back to cell state
    for var_i in 0..VARS {
        for force_f in 0..FORCES {
            cell.e[var_i][force_f] = evolved[var_i * FORCES + force_f];
        }
    }
}
```

#### 2.2. Complete `transport.rs` (2 hours)

**TODO:**
- [ ] Implement `exchange_exact()` for conservative two-cell exchange
- [ ] Add `distribute_to_neighbors()` with Rayon parallelization
- [ ] Implement spatial mode analysis (FFT-based)
- [ ] Add configurable coupling matrices

#### 2.3. Complete `init.rs` (2 hours)

**TODO:**
- [ ] Implement `initialize_homogeneous()` with noise
- [ ] Add `initialize_structured()` for spatial modes
- [ ] Create `EnergyDistribution` builder
- [ ] Add simplex sampling for random distributions

### Phase 3: Simulation Loop (Day 3, ~8 hours)

#### 3.1. Complete `evolution.rs` (4 hours)

**TODO:**
- [ ] Finalize `Simulation::step()` with error handling
- [ ] Implement `evolve_until()` with callback support
- [ ] Add checkpoint/restore functionality
- [ ] Create adaptive timestep logic (optional)

#### 3.2. Complete `conservation.rs` (2 hours)

**TODO:**
- [ ] Implement all pattern metric computations
- [ ] Add Fourier-based power spectrum analysis
- [ ] Create void/wall/filament classification
- [ ] Add clustering dimension calculation

#### 3.3. Complete `visualization.rs` (2 hours)

**TODO:**
- [ ] Implement Parquet snapshot writing
- [ ] Add metrics export functions
- [ ] Create 2D slice extraction for TUI
- [ ] Add ASCII art rendering helpers

### Phase 4: Advanced Features (Day 4-5)

#### 4.1. Oscillation Tracking
- [ ] Implement mode projection and tracking
- [ ] Add frequency extraction from time series
- [ ] Create coupling matrix analysis

#### 4.2. Parquet I/O
- [ ] Set up Arrow schema for snapshots
- [ ] Implement streaming writes
- [ ] Add partitioning by run_id/step

#### 4.3. TUI Integration
- [ ] Connect simulation to entropy TUI
- [ ] Implement real-time slice rendering
- [ ] Add chart data extraction
- [ ] Create interactive parameter controls

## Testing Strategy

### Unit Tests

Each module should have `#[cfg(test)]` blocks with:

1. **types.rs:** Test all constructors, validators
2. **lattice.rs:** Test indexing, boundary conditions, neighbors
3. **energy.rs:** Test constraint enforcement, projection accuracy
4. **redistribution.rs:** Test antisymmetry, energy conservation
5. **transport.rs:** Test two-cell exchange conservation

**Example:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    
    #[test]
    fn test_energy_conservation_after_redistribution() {
        let mut cell = CellState::default();
        // Initialize with known energy
        cell.e[0][0] = 1.0;
        let initial_energy = energy::total_energy(&cell);
        
        let matrix = RedistributionMatrix::new_zero();
        redistribution::evolve_exact(&mut cell, &matrix, 0.1);
        
        let final_energy = energy::total_energy(&cell);
        assert_relative_eq!(initial_energy, final_energy, epsilon = 1e-10);
    }
}
```

### Integration Tests

Create `tests/integration_test.rs`:

```rust
use entropic_void::*;

#[test]
fn test_full_simulation_conserves_energy() {
    let lattice = Lattice::new((4, 4, 4));
    let redistribution = RedistributionMatrix::new_zero();
    let coupling = [[0.01; FORCES]; VARS];
    let constraints = ConstraintSet::default();
    
    let mut sim = Simulation::new(lattice, redistribution, coupling, constraints);
    
    // Get initial energy
    let initial_energy = sim.lattice.cells.iter()
        .map(|c| energy::total_energy(c))
        .sum::<f64>();
    
    // Evolve
    sim.evolve_until(1.0, 0.01, |_| {}).unwrap();
    
    // Check conservation
    let final_energy = sim.lattice.cells.iter()
        .map(|c| energy::total_energy(c))
        .sum::<f64>();
    
    assert!((initial_energy - final_energy).abs() < 1e-9);
}
```

## Performance Optimization

### Profiling

Use `cargo flamegraph` to identify hotspots:

```bash
cargo install flamegraph
sudo cargo flamegraph --bin entropy
```

### Optimization Checklist

- [ ] Use `rayon` for parallel lattice updates
- [ ] Profile matrix operations, use BLAS if needed
- [ ] Cache FFT plans for repeated use
- [ ] Consider SIMD for energy aggregation
- [ ] Use `inline` hints for hot paths

## Documentation

### Doc Comments

Every public item needs doc comments:

```rust
/// Represents the energy state of a single lattice cell.
///
/// Energy is distributed across 5 variables and 4 forces,
/// stored as a 2D array `e[var][force]`.
///
/// # Examples
///
/// ```
/// use entropic_void::{CellState, VARS, FORCES};
///
/// let cell = CellState::default();
/// assert_eq!(cell.e.len(), VARS);
/// ```
pub struct CellState {
    pub e: [[f64; FORCES]; VARS],
}
```

### Run `cargo doc`

Generate and check docs:

```bash
cargo doc --open --no-deps
```

## Continuous Integration

Ensure CI passes:

```bash
# All tests
cargo test --all

# Lints
cargo clippy --all -- -D warnings

# Format
cargo fmt --all -- --check

# Docs
cargo doc --no-deps
```

## Next Steps

Once core library is complete:

1. Implement example binaries (`examples/`)
2. Create benchmark suite
3. Write tutorial documentation
4. Publish to crates.io
