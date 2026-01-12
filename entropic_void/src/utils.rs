#![forbid(unsafe_code)]

/*
Uses Lattice from lattice for 3D FFT.
Used by: redistribution, oscillation, visualization, conservation.
*/

use num_complex::Complex64;
use rand::rngs::SmallRng;
use crate::lattice::Lattice;
/*
matrix_ops submodule
*/

/**/
pub fn multiply<const N: usize>(a: &[[f64; N]; N], b: &[[f64; N]; N]) -> [[f64; N]; N] {
    todo!();
}

/**/
pub fn exponential<const N: usize>(a: &[[f64; N]; N], t: f64, terms: usize) -> [[f64; N]; N] {
    todo!();
}

/**/
pub fn eigenvalues<const N: usize>(a: &[[f64; N]; N]) -> Vec<Complex64> {
    todo!();
}

/**/
pub fn eigenvectors<const N: usize>(a: &[[f64; N]; N]) -> (Vec<Complex64>, Vec<[Complex64; N]>) {
    todo!();
}

/**/
pub fn is_antisymmetric<const N: usize>(a: &[[f64; N]; N], tol: f64) -> bool {
    todo!();
}

/**/
pub fn column_sum<const N: usize>(a: &[[f64; N]; N], col: usize) -> f64 {
    todo!();
}

/**/
pub fn row_sum<const N: usize>(a: &[[f64; N]; N], row: usize) -> f64 {
    todo!();
}

/*
sampling submodule
*/

/**/
pub fn sample_simplex(n: usize, rng: &mut SmallRng) -> Vec<f64> {
    todo!();
}

/**/
pub fn sample_normal(mean: f64, std: f64, rng: &mut SmallRng) -> f64 {
    todo!();
}

/**/
pub fn add_noise(value: f64, noise_fraction: f64, rng: &mut SmallRng) -> f64 {
    todo!();
}

/*
fft submodule
*/

/**/
pub fn fft_1d(signal: &[f64]) -> Vec<Complex64> {
    todo!();
}

/**/
pub fn fft_3d(lattice: &Lattice, var_i: usize, force_f: usize) -> Vec<Complex64> {
    todo!();
}

/**/
pub fn power_spectrum(fft: &[Complex64]) -> Vec<f64> {
    todo!();
}

/*
hilbert submodule
*/

/**/
pub fn instantaneous_phase(signal: &[f64]) -> Vec<f64> {
    todo!();
}