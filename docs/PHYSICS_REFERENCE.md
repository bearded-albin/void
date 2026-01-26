# Physics Equations Reference

This document provides the complete mathematical foundation for the VOID simulation.

## Core Constants

```rust
const VARS: usize = 5;   // Energy variables
const FORCES: usize = 4; // Fundamental forces
const N_FLATTENED: usize = VARS * FORCES; // 20 degrees of freedom per cell
```

## Energy Variables

| Index | Name | Symbol | Physical Interpretation |
|-------|------|--------|-------------------------|
| 0 | EM Radiation | \(E_{\gamma}\) | Photons, electromagnetic waves |
| 1 | Baryonic Matter | \(E_b\) | Protons, neutrons, atoms |
| 2 | Neutrinos | \(E_{\nu}\) | Weakly-interacting leptons |
| 3 | Unknown₁ | \(E_{u1}\) | Dark matter candidate |
| 4 | Unknown₂ | \(E_{u2}\) | Dark energy candidate |

## Fundamental Forces

| Index | Name | Symbol | Range | Couples To |
|-------|------|--------|-------|------------|
| 0 | Gravity | \(F_G\) | Infinite | All energy |
| 1 | Electromagnetism | \(F_{EM}\) | Infinite | Charged particles |
| 2 | Weak Nuclear | \(F_W\) | ~10⁻¹⁸ m | Leptons, quarks |
| 3 | Strong Nuclear | \(F_S\) | ~10⁻¹⁵ m | Quarks, gluons |

## Cell State Representation

Each lattice cell contains energy distributed across variables and forces:

```
E[i][f] = energy of variable i coupled to force f
```

Total energy per cell:
```
E_total = ∑_{i=0}^{4} ∑_{f=0}^{3} E[i][f]
```

## Local Redistribution (Intra-Cell)

### Evolution Equation

Energy redistribution within a cell:

```
d𝐬/dt = R · 𝐬
```

where:
- \(𝐬\) is the flattened energy vector (length 20)
- \(R\) is the 20×20 redistribution matrix

### Antisymmetric Matrix

For **strict conservation**, \(R\) must be antisymmetric:

```
R = -Rᵀ
```

This ensures:
```
d/dt (𝐬ᵀ · 𝐬) = 𝐬ᵀ · Rᵀ · 𝐬 + 𝐬ᵀ · R · 𝐬
              = 𝐬ᵀ · (-R) · 𝐬 + 𝐬ᵀ · R · 𝐬
              = 0
```

### Exact Solution

For antisymmetric \(R\), the solution is:

```
𝐬(t) = exp(R · t) · 𝐬(0)
```

where \(exp(R · t)\) is a **rotation matrix** (orthogonal), preserving energy exactly.

### Matrix Exponential

Compute via Taylor series:

```
exp(R · t) = I + (R · t) + (R · t)²/2! + (R · t)³/3! + ...
```

Or via eigendecomposition:
```
R = Q · Λ · Q⁻¹
exp(R · t) = Q · exp(Λ · t) · Q⁻¹
```

For antisymmetric \(R\), eigenvalues are **purely imaginary**:
```
λ_k = i · ω_k
```

leading to oscillatory solutions:
```
exp(i · ω_k · t) = cos(ω_k · t) + i · sin(ω_k · t)
```

## Oscillation Modes

### Eigenmode Decomposition

For antisymmetric \(R\):

```
R · v_k = i · ω_k · v_k
```

where:
- \(v_k\) is eigenvector (mode shape)
- \(ω_k\) is oscillation frequency

### Mode Projection

Project current state onto mode:

```
A_k(t) = 𝐬(t) · v_k
```

Time evolution:
```
A_k(t) = A_k(0) · exp(i · ω_k · t)
```

### Physical Interpretation

- **Low-frequency modes** (ω ≈ 0): Slow energy transfer between variables
- **High-frequency modes** (ω ≫ 1): Rapid oscillations between forces

## Spatial Transport (Inter-Cell)

### Conservative Exchange

Energy exchange between neighboring cells:

```
dE_i/dt = κ · (E_j - E_i)
```

where:
- \(κ\) is coupling strength
- \(E_i, E_j\) are energies in cells \(i, j\)

This conserves total energy:
```
d(E_i + E_j)/dt = κ · (E_j - E_i) + κ · (E_i - E_j) = 0
```

### Exact Two-Cell Solution

For initial conditions \(E_i(0)\), \(E_j(0)\):

```
E_i(t) = E_avg + (E_i(0) - E_avg) · exp(-2κ · t)
E_j(t) = E_avg + (E_j(0) - E_avg) · exp(-2κ · t)
```

where \(E_avg = (E_i(0) + E_j(0))/2\) is the equilibrium energy.

### Lattice Diffusion

On a 3D cubic lattice with 6 neighbors:

```
dE(r,t)/dt = κ · ∑_{neighbors} (E(r', t) - E(r, t))
```

This is a discrete Laplacian:
```
∇² E ≈ ∑_{directions} (E(r + Δr) - E(r))
```

### Fourier Modes

Spatial Fourier transform:

```
E(k, t) = ∑_r E(r, t) · exp(-i · k · r)
```

Dispersion relation:
```
ω(k) = 2κ · (cos(k_x) + cos(k_y) + cos(k_z) - 3)
```

## Conservation Laws

### Global Energy

Total energy is **exactly conserved**:

```
E_total(t) = ∑_{cells} E_cell(t) = const
```

Numerical check:
```
|E_total(t) - E_total(0)| < ε
```

where \(ε ≈ 10⁻¹²\) is machine precision.

### Per-Variable Conservation (Optional)

If configured, conserve each variable separately:

```
E_i_total = ∑_{cells} ∑_{forces} E[i][f] = const
```

### Conserved Charges (Optional)

Define per-variable "charges":

```
Q_i = ∑_{cells} ρ_i · E[i]
```

where \(ρ_i\) is a charge coefficient (e.g., +1 for matter, -1 for antimatter).

## Pattern Metrics

### Density Distribution

Local energy density:

```
ρ(r) = E_cell(r) / V_cell
```

Histogram:
```
H(ρ) = count of cells with density in [ρ, ρ + dρ]
```

### Void/Wall/Filament Classification

Threshold-based:

```
Void: ρ(r) < ρ_mean - σ
Wall: ρ_mean - σ < ρ(r) < ρ_mean + σ
Filament: ρ(r) > ρ_mean + σ
```

### Power Spectrum

3D Fourier transform:

```
P(k) = |∑_r ρ(r) · exp(-i · k · r)|²
```

Spherically averaged:
```
P(|k|) = average over all k with same magnitude
```

### Clustering Dimension

Fractal dimension of high-density regions:

```
N(r) ∝ r^D
```

where:
- \(N(r)\) is number of cells within distance \(r\)
- \(D\) is clustering dimension
- Cosmic web: \(D ≈ 1-2\) (filamentary)

## Numerical Methods

### Timestep Selection

For stability, require:

```
Δt < 1 / max(ω_k)
```

where \(ω_k\) is the largest oscillation frequency.

For spatial diffusion:
```
Δt < Δx² / (2κ)
```

### Operator Splitting

Split evolution into:
1. **Redistribution step**: \(exp(R · Δt)\)
2. **Transport step**: \(exp(T · Δt)\)

Second-order Strang splitting:
```
U(Δt) ≈ exp(R · Δt/2) · exp(T · Δt) · exp(R · Δt/2)
```

### Energy Projection

After each step, enforce constraints:

1. **Clip negative energies** (if any arise from numerical error)
2. **Renormalize to conserve total energy**
3. **Apply variable constraints** (fixed totals, ratios)

## Implementation Formulas

### Index Flattening

Convert 2D index to 1D:

```rust
fn flatten(var_i: usize, force_f: usize) -> usize {
    var_i * FORCES + force_f
}
```

Unflatten:

```rust
fn unflatten(idx: usize) -> (usize, usize) {
    (idx / FORCES, idx % FORCES)
}
```

### Lattice Index

3D to 1D:

```rust
fn index_3d(x: usize, y: usize, z: usize, size: (usize, usize, usize)) -> usize {
    let (sx, sy, _) = size;
    x + y * sx + z * sx * sy
}
```

### Neighbor Offsets

6-connectivity:

```rust
const NEIGHBOR_OFFSETS: [(isize, isize, isize); 6] = [
    ( 1,  0,  0),  // +X
    (-1,  0,  0),  // -X
    ( 0,  1,  0),  // +Y
    ( 0, -1,  0),  // -Y
    ( 0,  0,  1),  // +Z
    ( 0,  0, -1),  // -Z
];
```

## References

### Books
- Numerical Recipes (Press et al.)
- Introduction to Computational Astrophysical Hydrodynamics (Paardekooper)

### Papers
- Rieder et al. (2013) - Void/Wall/Filament Cosmology
- Greydanus et al. (2019) - Hamiltonian Neural Networks
- Jamieson et al. (2022) - Field-Level Emulators

### Code References
- Enzo: AMR cosmological code
- RAMSES: Adaptive mesh refinement
- map2map: ML emulation framework
