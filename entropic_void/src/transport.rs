#![forbid(unsafe_code)]

/*
Purpose: Spatial coupling between neighboring cells.

Uses Lattice, CellState, SpatialMode from types and lattice.
Uses utils::fft for spectral analysis.
Called by: evolution::step_transport, oscillation for global modes, visualization.
*/

use crate::lattice::Lattice;
use crate::types::{CellState, FORCES, SpatialMode, VARS};

/*
Effect:
Take E_a = cell_a.e[var_i][force_f], E_b = cell_b.e[var_i][force_f].
Evolve as conservative two-oscillator exchange.
*/
pub fn exchange_exact(
    cell_a: &mut CellState,
    cell_b: &mut CellState,
    var_i: usize,
    force_f: usize,
    coupling: f64,
    dt: f64,
) {
    todo!();
}

/*
Effect:
For each cell and neighbor (likely via neighbors_6):
For each (var_i, force_f):
Call exchange_exact with coupling = coupling_matrix[var_i][force_f].
*/
pub fn distribute_to_neighbors(
    lattice: &mut Lattice,
    coupling_matrix: &[[f64; FORCES]; VARS],
    dt: f64,
) {
    todo!();
}

/**/
pub fn fourier_mode_frequency(k: (isize, isize, isize), size: (usize, usize, usize)) -> f64 {
    todo!();
}

/*
Uses utils::fft::fft_3d to compute FFT of E[c][var_i][force_f].
Converts to SpatialMode list.
*/
pub fn compute_spatial_modes(lattice: &Lattice, var_i: usize, force_f: usize) -> Vec<SpatialMode> {
    todo!();
}
