#![forbid(unsafe_code)]

use void::{
    generate_lattice,
    simulate_tick,
};

fn main() {
    // Parameters
    let seed = 42;
    let lattice_size = (3, 3, 3); // 3x3x3 for quick test
    let total_energy = 1000.0;

    // Initialize lattice
    let mut lattice = generate_lattice(seed, lattice_size, total_energy);
    println!("=== Initial Lattice ===");
    for (i, cell) in lattice.cells.iter().enumerate() {
        println!("Cell {}: total_energy = {:.3}", i, cell.total_energy);
        for g in &cell.groups {
            println!("  {:?}: {:.3}", g.kind, g.total_energy);
        }
    }

    // Run one simulation tick
    simulate_tick(&mut lattice, seed);

    // Print updated lattice
    println!("\n=== Lattice After 1 Tick ===");
    for (i, cell) in lattice.cells.iter().enumerate() {
        println!("Cell {}: total_energy = {:.3}", i, cell.total_energy);
        for g in &cell.groups {
            println!("  {:?}: {:.3}", g.kind, g.total_energy);
        }
    }

    // Optional: sum total lattice energy to check conservation
    let total: f64 = lattice.cells.iter().map(|c| c.total_energy).sum();
    println!("\nTotal lattice energy: {:.3}", total);
}
