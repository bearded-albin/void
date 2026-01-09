#![forbid(unsafe_code)]
use rayon::prelude::*;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;

//
// =======================
// Core Types
// =======================
//

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    PosX, NegX,
    PosY, NegY,
    PosZ, NegZ,
}

impl Direction {
    pub const ALL: [Direction; 6] = [
        Self::PosX, Self::NegX,
        Self::PosY, Self::NegY,
        Self::PosZ, Self::NegZ,
    ];

    pub fn offset(self) -> (isize, isize, isize) {
        match self {
            Self::PosX => ( 1, 0, 0),
            Self::NegX => (-1, 0, 0),
            Self::PosY => ( 0, 1, 0),
            Self::NegY => ( 0,-1, 0),
            Self::PosZ => ( 0, 0, 1),
            Self::NegZ => ( 0, 0,-1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnergyGroupKind {
    Light,
    Matter,
    Neutrino,
    OppositeMatter,
    OppositeLight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Interaction {
    Electromagnetic,
    Strong,
    Weak,
    Gravitational,
    Expansion,
}

//
// =======================
// Tensor-Based Flux
// =======================
//

#[derive(Clone, Debug)]
struct FluxTensor {
    flux: [f64; 6],
}

impl FluxTensor {
    fn zero() -> Self {
        Self { flux: [0.0; 6] }
    }
}

//
// =======================
// Energy Structures
// =======================
//

#[derive(Debug, Clone)]
pub struct EnergyPacket { pub energy: f64 }

#[derive(Debug, Clone)]
pub struct Subgroup {
    pub interaction: Interaction,
    pub packets: Vec<EnergyPacket>,
}

#[derive(Debug, Clone)]
pub struct EnergyGroup {
    pub kind: EnergyGroupKind,
    pub total_energy: f64,
    pub subgroups: Vec<Subgroup>,
}

#[derive(Debug, Clone)]
pub struct Cell {
    pub total_energy: f64,
    pub groups: Vec<EnergyGroup>,
}

//
// =======================
// Lattice
// =======================
//

#[derive(Clone)]
pub struct Lattice {
    pub size: (usize, usize, usize),
    pub cells: Vec<Cell>,
}

impl Lattice {
    pub fn index(&self, x: usize, y: usize, z: usize) -> usize {
        let (sx, sy, _) = self.size;
        x + y * sx + z * sx * sy
    }

    fn in_bounds(&self, x: isize, y: isize, z: isize) -> bool {
        x >= 0 && y >= 0 && z >= 0 &&
            (x as usize) < self.size.0 &&
            (y as usize) < self.size.1 &&
            (z as usize) < self.size.2
    }
}

//
// =======================
// Initialization
// =======================
//

fn random_partition(total: f64, n: usize, rng: &mut SmallRng) -> Vec<f64> {
    let mut v: Vec<f64> = (0..n).map(|_| rng.random_range(0.5..1.5)).collect();
    let s: f64 = v.iter().sum();
    for x in &mut v { *x = *x / s * total; }
    v
}

fn init_group(kind: EnergyGroupKind, energy: f64, rng: &mut SmallRng) -> EnergyGroup {
    let interactions = match kind {
        EnergyGroupKind::Light => vec![Interaction::Electromagnetic],
        EnergyGroupKind::Matter => vec![Interaction::Strong, Interaction::Weak, Interaction::Gravitational],
        EnergyGroupKind::Neutrino => vec![Interaction::Weak, Interaction::Gravitational],
        EnergyGroupKind::OppositeMatter => vec![Interaction::Gravitational],
        EnergyGroupKind::OppositeLight => vec![Interaction::Expansion],
    };

    let parts = random_partition(energy, interactions.len(), rng);
    let subgroups = interactions.into_iter().zip(parts).map(|(i,e)| Subgroup {
        interaction: i,
        packets: vec![EnergyPacket{ energy: e }],
    }).collect();

    EnergyGroup { kind, total_energy: energy, subgroups }
}

fn init_cell(energy: f64, rng: &mut SmallRng) -> Cell {
    let kinds = [
        EnergyGroupKind::Light,
        EnergyGroupKind::Matter,
        EnergyGroupKind::Neutrino,
        EnergyGroupKind::OppositeMatter,
        EnergyGroupKind::OppositeLight,
    ];

    let parts = random_partition(energy, kinds.len(), rng);
    let groups = kinds.iter().zip(parts)
        .map(|(&k,e)| init_group(k,e,rng))
        .collect();

    Cell { total_energy: energy, groups }
}

pub fn generate_lattice(seed: u64, size: (usize, usize, usize), total_energy: f64) -> Lattice {
    let mut rng = SmallRng::seed_from_u64(seed);
    let n = size.0 * size.1 * size.2;
    let per = total_energy / n as f64;
    let cells = (0..n).map(|_| init_cell(per, &mut rng)).collect();
    Lattice { size, cells }
}

//
// =======================
// FORCE TENSORS
// =======================
//

fn electromagnetic_tensor(cell: &Cell) -> FluxTensor {
    let mut t = FluxTensor::zero();
    if let Some(g) = cell.groups.iter().find(|g| g.kind == EnergyGroupKind::Light) {
        let e = g.total_energy * 0.05;
        t.flux[0] = e / 4.0;
        t.flux[1] = e / 4.0;
        t.flux[2] = e / 4.0;
        t.flux[3] = e / 4.0;
    }
    t
}

fn gravitational_tensor(cell: &Cell, neighbors: &[(Direction, &Cell)]) -> FluxTensor {
    let mut t = FluxTensor::zero();
    for (d, n) in neighbors {
        if n.total_energy > cell.total_energy {
            t.flux[*d as usize] = (n.total_energy - cell.total_energy) * 0.01;
        }
    }
    t
}

fn expansion_tensor(cell: &Cell) -> FluxTensor {
    let mut t = FluxTensor::zero();
    if let Some(g) = cell.groups.iter().find(|g| g.kind == EnergyGroupKind::OppositeLight) {
        let e = g.total_energy * 0.03;
        for i in 0..6 { t.flux[i] = e / 6.0; }
    }
    t
}

//
// =======================
// INTRA-CELL FORCES
// =======================
//

fn strong_force(cell: &mut Cell) {
    if let Some(g) = cell.groups.iter_mut().find(|g| g.kind == EnergyGroupKind::Matter) {
        let total: f64 = g.subgroups.iter().flat_map(|sg| sg.packets.iter()).map(|p| p.energy).sum();
        let count = g.subgroups.iter().map(|sg| sg.packets.len()).sum::<usize>().max(1);
        let per = total / count as f64;
        for sg in &mut g.subgroups { for p in &mut sg.packets { p.energy = per; } }
    }
}

fn weak_force(cell: &mut Cell, rng: &mut SmallRng) {
    for g in &mut cell.groups {
        for sg in &mut g.subgroups {
            if sg.interaction == Interaction::Weak {
                let f = rng.random_range(0.9..1.1);
                for p in &mut sg.packets { p.energy *= f; }
            }
        }
    }
}

//
// =======================
// PARALLEL SIMULATION STEP
// =======================
//

pub fn simulate_tick(lattice: &mut Lattice, seed: u64) {
    let n_cells = lattice.cells.len();

    // Step 1: compute fluxes per cell in parallel
    let fluxes: Vec<Vec<(usize, f64)>> = (0..n_cells).into_par_iter().map(|idx| {
        let (sx, sy, _sz) = lattice.size;
        let x = idx % sx;
        let y = (idx / sx) % sy;
        let z = idx / (sx * sy);
        let mut rng = SmallRng::seed_from_u64(seed + idx as u64);

        let cell = &lattice.cells[idx];

        // Compute neighbors
        let neighbors: Vec<(Direction, &Cell)> = Direction::ALL.iter().filter_map(|&d| {
            let (dx, dy, dz) = d.offset();
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            let nz = z as isize + dz;
            if lattice.in_bounds(nx, ny, nz) {
                Some((d, &lattice.cells[lattice.index(nx as usize, ny as usize, nz as usize)]))
            } else { None }
        }).collect();

        // Compute tensors
        let tensors = [
            electromagnetic_tensor(cell),
            gravitational_tensor(cell, &neighbors),
            expansion_tensor(cell),
        ];

        // Compute net fluxes for this cell
        let mut cell_flux: Vec<(usize, f64)> = Vec::new();

        for t in &tensors {
            for (i, &e) in t.flux.iter().enumerate() {
                if e > 0.0 {
                    let d = Direction::ALL[i];
                    let (dx, dy, dz) = d.offset();
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;
                    let nz = z as isize + dz;
                    if lattice.in_bounds(nx, ny, nz) {
                        let nidx = lattice.index(nx as usize, ny as usize, nz as usize);
                        // outgoing flux from this cell
                        cell_flux.push((nidx, e));
                        // negative flux to self
                        cell_flux.push((idx, -e));
                    }
                }
            }
        }

        // Apply intra-cell forces immediately to a clone
        let mut cell_clone = cell.clone();
        strong_force(&mut cell_clone);
        weak_force(&mut cell_clone, &mut rng);

        // Replace old cell with clone
        // Actually we only mutate total_energy here later
        cell_flux.push((idx, 0.0)); // dummy to keep indexing consistent
        cell_flux
    }).collect();

    // Step 2: flatten fluxes into delta array
    let mut deltas = vec![0.0; n_cells];
    for cell_flux in fluxes {
        for (idx, delta) in cell_flux {
            deltas[idx] += delta;
        }
    }

    // Step 3: apply deltas to lattice
    lattice.cells.iter_mut().zip(deltas.iter()).for_each(|(c, &d)| {
        c.total_energy += d;
    });
}

