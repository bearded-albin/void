use rayon::prelude::*;
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant}
};
use rand::{
    rngs::{ThreadRng, SmallRng},
    Rng,
    SeedableRng
};

use eframe::{App, egui};
use egui_plotter::EguiBackend;
use plotters::prelude::*;

// ===================== Directions =====================
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction { PosX, NegX, PosY, NegY, PosZ, NegZ }

impl Direction {
    pub const ALL: [Direction; 6] = [Self::PosX, Self::NegX, Self::PosY, Self::NegY, Self::PosZ, Self::NegZ];
}

// ===================== Energy Model =====================
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnergyGroupKind { Light, Matter, Neutrino, OppositeMatter, OppositeLight }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Interaction { Electromagnetic, Strong, Weak, Gravitational, Expansion }

#[derive(Debug, Clone)]
pub struct EnergyPacket { pub energy: f64, pub direction: Option<Direction> }

#[derive(Debug, Clone)]
pub struct Subgroup { pub interaction: Interaction, pub packets: Vec<EnergyPacket> }

#[derive(Debug, Clone)]
pub struct EnergyGroup { pub kind: EnergyGroupKind, pub total_energy: f64, pub subgroups: Vec<Subgroup> }

#[derive(Debug, Clone)]
pub struct Voxel { pub total_energy: f64, pub density: f64, pub groups: Vec<EnergyGroup> }

// ===================== Lattice =====================
#[derive(Clone)]
pub struct Lattice { pub size: (usize, usize, usize), pub voxels: Vec<Voxel> }

impl Lattice {
    pub fn index(&self, x: usize, y: usize, z: usize) -> usize {
        let (sx, sy, _) = self.size;
        x + y * sx + z * sx * sy
    }
}

// ===================== Initialization =====================
fn random_partition(total: f64, n: usize, rng: &mut impl Rng) -> Vec<f64> {
    let mut vals: Vec<f64> = (0..n).map(|_| rng.random_range(0.5..1.5)).collect();
    let sum: f64 = vals.iter().sum();
    vals.iter_mut().for_each(|v| *v = *v / sum * total);
    vals
}

fn init_group(kind: EnergyGroupKind, energy: f64, rng: &mut impl Rng) -> EnergyGroup {
    let interactions = match kind {
        EnergyGroupKind::Light => vec![Interaction::Electromagnetic],
        EnergyGroupKind::Matter => vec![Interaction::Strong, Interaction::Weak, Interaction::Gravitational],
        EnergyGroupKind::Neutrino => vec![Interaction::Weak, Interaction::Gravitational],
        EnergyGroupKind::OppositeMatter => vec![Interaction::Gravitational],
        EnergyGroupKind::OppositeLight => vec![Interaction::Expansion],
    };
    let energies = random_partition(energy, interactions.len(), rng);
    let subgroups = interactions.into_iter().zip(energies)
        .map(|(i, e)| {
            let packets = Direction::ALL.iter().map(|&d| EnergyPacket { energy: e / 6.0, direction: Some(d) }).collect();
            Subgroup { interaction: i, packets }
        }).collect();
    EnergyGroup { kind, total_energy: energy, subgroups }
}

fn init_voxel(base_energy: f64, density: f64, rng: &mut impl Rng) -> Voxel {
    let kinds = [EnergyGroupKind::Light, EnergyGroupKind::Matter, EnergyGroupKind::Neutrino,
        EnergyGroupKind::OppositeMatter, EnergyGroupKind::OppositeLight];
    let energies = random_partition(base_energy, kinds.len(), rng);
    let groups = kinds.iter().zip(energies).map(|(&k, e)| init_group(k, e, rng)).collect();
    Voxel { total_energy: base_energy, density, groups }
}

fn init_lattice(size: (usize, usize, usize), base_energy: f64, density: f64) -> Lattice {
    let mut rng = ThreadRng::default(); // <-- use ThreadRng
    let count = size.0 * size.1 * size.2;
    let voxels = (0..count).map(|_| {
        let e = base_energy * rng.random_range(0.995..1.005);
        init_voxel(e, density, &mut rng)
    }).collect();
    Lattice { size, voxels }
}

// ===================== Simulation Helpers =====================
fn fluctuate_voxel(voxel: &mut Voxel, rng: &mut SmallRng) {
    for group in &mut voxel.groups {
        let factor: f64 = rng.random_range(0.95..1.05);
        group.total_energy *= factor;
        for sg in &mut group.subgroups {
            for p in &mut sg.packets {
                p.energy *= rng.random_range(0.95..1.05);
            }
        }
        if group.kind == EnergyGroupKind::Matter {
            group.total_energy *= 0.999; // decay
        }
    }
}

fn diffuse_voxel(voxel: &mut Voxel) {
    if let Some(matter_group) = voxel.groups.iter_mut().find(|g| g.kind == EnergyGroupKind::Matter) {
        let diffusion = matter_group.total_energy * 0.005;
        matter_group.total_energy -= diffusion;
        let n_packets = matter_group.subgroups.iter().map(|sg| sg.packets.len()).sum::<usize>();
        if n_packets > 0 {
            for sg in &mut matter_group.subgroups {
                for p in &mut sg.packets {
                    p.energy += diffusion / n_packets as f64;
                }
            }
        }
    }
}

// ===================== Simulation Step =====================

fn simulate_step(lattice: &mut Lattice) {
    lattice.voxels.par_iter_mut().for_each(|voxel| {
        // Create a thread-local SmallRng seeded from thread_rng
        let mut seed_rng: ThreadRng = ThreadRng::default();
        let mut rng: SmallRng = SmallRng::from_rng(&mut seed_rng);

        fluctuate_voxel(voxel, &mut rng);
        diffuse_voxel(voxel);
    });
}

// ===================== Visualization =====================
fn draw_slice(ui: &mut egui::Ui, lattice: &Lattice, z: usize) {
    let backend = EguiBackend::new(ui);
    let root = backend.into_drawing_area();
    root.fill(&WHITE).ok();

    let (sx, sy, _) = lattice.size;
    let mut chart = ChartBuilder::on(&root).margin(5).build_cartesian_2d(0..sx as i32, 0..sy as i32).unwrap();
    chart.configure_mesh().disable_mesh().draw().unwrap();

    for y in 0..sy {
        for x in 0..sx {
            let v = &lattice.voxels[lattice.index(x, y, z)];
            let matter_energy = v.groups.iter()
                .find(|g| g.kind == EnergyGroupKind::Matter)
                .map(|g| g.total_energy)
                .unwrap_or(0.0);

            let intensity = (matter_energy / 1.0).max(0.2); // ensures at least faint red

            chart.draw_series(std::iter::once(Rectangle::new(
                [(x as i32, y as i32), ((x + 1) as i32, (y + 1) as i32)],
                RGBColor((255.0 * intensity) as u8, 0, 0).filled()
            ))).ok();
        }
    }
}

// ===================== App =====================
pub struct SimApp {
    lattice: Arc<Mutex<Lattice>>,
    last_tick: Instant,
    tick_interval: Duration,
}

impl App for SimApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if Instant::now().duration_since(self.last_tick) >= self.tick_interval {
            if let Ok(mut lat) = self.lattice.lock() {
                simulate_step(&mut lat);
            }
            self.last_tick = Instant::now();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Auto-updating simulation every 200ms");
            let lat = self.lattice.lock().unwrap();
            draw_slice(ui, &lat, lat.size.2 / 2);
        });

        ctx.request_repaint();
    }
}

// ===================== Main =====================
fn main() {
    let lattice = init_lattice((126, 126, 126), 1.0, 1.0); // larger lattice

    let app = SimApp {
        lattice: Arc::new(Mutex::new(lattice)),
        last_tick: Instant::now(),
        tick_interval: Duration::from_millis(200),
    };

    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "3D Lattice Energy Simulation",
        options,
        Box::new(|_cc| Ok(Box::new(app) as Box<dyn App>))
    ).unwrap();
}
