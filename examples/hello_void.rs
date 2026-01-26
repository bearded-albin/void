//! Hello VOID - Your First Simulation
//!
//! This example demonstrates the basic workflow:
//! 1. Create a lattice
//! 2. Configure physics (redistribution + coupling)
//! 3. Run simulation
//! 4. Check conservation

use entropic_void::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== VOID: Hello World Simulation ===");
    println!("Creating a minimal cosmic web simulator...\n");
    
    // Step 1: Create a small 8x8x8 lattice
    println!("[1/5] Initializing 8³ lattice...");
    let lattice = Lattice::new((8, 8, 8));
    println!("      ✓ Lattice size: {:?}", lattice.size());
    println!("      ✓ Total cells: {}", 8 * 8 * 8);
    
    // Step 2: Set up redistribution matrix (zero = no oscillations)
    println!("\n[2/5] Configuring energy redistribution...");
    let redistribution = RedistributionMatrix::new_zero();
    println!("      ✓ Using zero matrix (no intra-cell oscillations)");
    println!("      ✓ All energy will remain in initial modes");
    
    // Step 3: Define spatial coupling (very weak for demo)
    println!("\n[3/5] Setting up spatial coupling...");
    let coupling = [[0.005; FORCES]; VARS];
    println!("      ✓ Coupling strength: 0.005");
    println!("      ✓ Energy will slowly diffuse between cells");
    
    // Step 4: Create constraint set (all energy free)
    println!("\n[4/5] Applying constraints...");
    let constraints = ConstraintSet::default();
    println!("      ✓ All variables free to evolve");
    
    // Step 5: Create simulation and evolve
    println!("\n[5/5] Running simulation...");
    let mut sim = Simulation::new(lattice, redistribution, coupling, constraints);
    
    let t_end = 1.0;
    let dt = 0.01;
    let mut progress_steps = 0;
    
    sim.evolve_until(t_end, dt, |_| {
        progress_steps += 1;
        if progress_steps % 20 == 0 {
            print!(".");
            use std::io::{self, Write};
            io::stdout().flush().unwrap();
        }
    })?;
    
    println!("\n      ✓ Evolved to t = {:.2}", sim.time);
    println!("      ✓ Total steps: {}", sim.step);
    
    // Check conservation (TODO: implement this method)
    println!("\n=== Simulation Complete ===");
    println!("\nNext steps:");
    println!("  - See examples/spatial_propagation.rs for diffusion demo");
    println!("  - See examples/basic_oscillation.rs for mode analysis");
    println!("  - Run 'cargo run -p entropy' for interactive TUI\n");
    
    Ok(())
}
