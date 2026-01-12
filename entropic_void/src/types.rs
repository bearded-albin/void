#![forbid(unsafe_code)]

/*
Purpose: Core data types and constants used everywhere.

Pure type definitions; no external calls.
Used by all other modules.
*/

/*
Core data types and constants used everywhere.
*/
/**/
pub const VARS: usize = 5;
/**/
pub const FORCES: usize = 4;
/**/
pub const N_FLATTENED: usize = VARS * FORCES;

/*
Energy per variable per force in one cell.
*/
#[derive(Default)]
pub struct CellState {
    pub e: [[f64; FORCES]; VARS],
}

/**/
#[derive(Default)]
pub struct RedistributionMatrix {
    pub a: [[f64; N_FLATTENED]; N_FLATTENED],
}

/**/
#[derive(Default)]
pub enum VariableConstraint {
    #[default]
    Free,
    FixedTotal(f64),
    FixedRatio([f64; VARS]),
}

/**/
#[derive(Default)]
pub struct ExpressionConstraint {
    pub locked: bool,
    pub force_pct: [f64; FORCES],
}

/**/
#[derive(Default)]
pub struct TransferMask {
    pub allow_var_to_var: [[bool; VARS]; VARS],
    pub allow_force_to_force: [[bool; FORCES]; FORCES],
}

/**/
#[derive(Default)]
pub struct ConstraintSet {
    pub var_constraints: [VariableConstraint; VARS],
    pub expr_constraints: [ExpressionConstraint; VARS],
    pub transfer_mask: TransferMask,
}

/**/
#[derive(Default)]
pub struct OscillationMode {
    pub frequency: f64,
    pub amplitude: f64,
    pub phase: f64,
    pub eigenvector: [f64; N_FLATTENED],
}

/**/
#[derive(Default)]
pub struct SpatialMode {
    pub k: (isize, isize, isize),
    pub amplitude: f64,
    pub frequency: f64,
}

/**/
#[derive(Default)]
pub struct LatticeCoord {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

/**/
#[derive(Default)]
pub enum Direction {
    #[default]
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
}

/*
Extra?
*/
#[derive(Default)]
pub enum VariableKind {
    #[default]
    EMRadiation,
    Baryons,
    Neutrinos,
    Unknown1,
    Unknown2,
}

/*
Extra?
*/
#[derive(Default)]
pub enum ForceKind {
    #[default]
    Gravity,
    Electromagnetism,
    Weak,
    Strong,
}
