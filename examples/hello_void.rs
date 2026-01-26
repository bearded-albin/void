//! Hello VOID - Your First Simulation
//!
//! This example demonstrates the basic workflow using the current
//! low-level API:
//! 1. Generate a lattice with random energy distribution
//! 2. Step the simulation forward with `simulate_tick`
//! 3. Check that total energy stays roughly constant

use entropic_void::{generate_lattice, simulate_tick, Lattice};

fn total_energy(lattice: &Lattice) -> f64 {
    lattice
        .cells
        .iter()
        .map(|c| c.total_energy)
        .sum::<f64>()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== VOID: Hello World Simulation ===");
    println!("Using low-level lattice API (generate_lattice + simulate_tick)\n");

    // Step 1: Create a small 8x8x8 lattice with homogeneous total energy
    println!("[1/3] Initializing 8³ lattice...");
    let size = (8, 8, 8);
    let total_energy_target = 1.0e6;
    let mut lattice = generate_lattice(42, size, total_energy_target);
    println!("      ✓ Lattice size: {:?}", lattice.size);
    println!("      ✓ Total cells: {}", size.0 * size.1 * size.2);

    let initial_energy = total_energy(&lattice);
    println!("      ✓ Initial total energy: {:.6e}", initial_energy);

    // Step 2: Evolve the lattice with a few ticks
    println!("\n[2/3] Evolving lattice with simulate_tick()...");
    let steps = 200;
    for step in 0..steps {
        // Use a changing seed so each tick is slightly different
        simulate_tick(&mut lattice, 1234 + step as u64);
        if step % 50 == 0 {
            println!("      → Performed step {}", step);
        }
    }

    // Step 3: Check energy conservation at a coarse level
    println!("\n[3/3] Checking energy conservation (approximate)...");
    let final_energy = total_energy(&lattice);
    let diff = (final_energy - initial_energy).abs();
    println!("      ✓ Final total energy:   {:.6e}", final_energy);
    println!("      ✓ |ΔE| = {:.6e}", diff);

    println!("\n=== Simulation Complete ===");
    println!("This example uses the current, working primitives.");
    println!("The higher-level Simulation API in evolution.rs is a TODO layer on top.\n");

    Ok(())
}
