//! Entropy - Terminal UI Dashboard for VOID Simulations
//!
//! This binary provides a real-time terminal interface for visualizing and controlling
//! energy lattice simulations. Built with ratatui for high-performance rendering.
//!
//! # Features
//!
//! - Real-time energy distribution visualization
//! - 2D slice views of the 3D lattice
//! - Live charts for conservation metrics and pattern statistics
//! - Interactive controls (pause/resume, step-through, parameter adjustment)
//! - ASCII-art rendering of energy density fields
//!
//! # Usage
//!
//! ```bash
//! cargo run --release -p entropy
//! ```
//!
//! # Controls
//!
//! - `Space`: Pause/Resume simulation
//! - `s`: Single step forward
//! - `r`: Reset simulation
//! - `[/]`: Decrease/increase time step
//! - `Up/Down`: Navigate through lattice Z-slices
//! - `Left/Right`: Cycle through energy variables
//! - `q`: Quit

use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{
        Axis, Block, Borders, Chart, Dataset, GraphType, Paragraph, Sparkline,
    },
    Frame, Terminal,
};
use std::{
    io::{stdout, Stdout},
    time::{Duration, Instant},
};

// Import the core simulation library
use entropic_void::*;

// ============================================================================
// Application State
// ============================================================================

/// Main application state holding simulation and UI state
struct App {
    /// The core simulation instance
    simulation: Simulation,
    
    /// Whether simulation is currently running
    is_running: bool,
    
    /// Current timestep size
    dt: f64,
    
    /// Target FPS for rendering
    target_fps: u32,
    
    /// Current Z-slice being displayed (0 to size.2-1)
    current_z_slice: usize,
    
    /// Which energy variable to display (0-4)
    current_variable: usize,
    
    /// History of total energy for plotting
    energy_history: Vec<f64>,
    
    /// History of pattern metrics
    void_fraction_history: Vec<f64>,
    filament_fraction_history: Vec<f64>,
    
    /// Maximum history length for charts
    max_history: usize,
    
    /// Timestamp of last simulation step
    last_step_time: Instant,
}

impl App {
    /// Create a new application with default simulation parameters
    fn new() -> Result<Self> {
        // Initialize a 32x32x32 lattice
        let lattice = Lattice::new((32, 32, 32));
        
        // TODO: Configure redistribution matrix with oscillation rates
        // For now, create a zero matrix (no redistribution)
        let redistribution = RedistributionMatrix::new_zero();
        
        // TODO: Set up spatial coupling strengths
        // Example: weak coupling for all variables/forces
        let coupling = [[0.01; FORCES]; VARS];
        
        // TODO: Define constraints
        // For now, use default (all variables free)
        let constraints = ConstraintSet::default();
        
        // Create simulation
        let simulation = Simulation::new(lattice, redistribution, coupling, constraints);
        
        Ok(Self {
            simulation,
            is_running: true,
            dt: 0.01,
            target_fps: 30,
            current_z_slice: 16, // Middle slice
            current_variable: 0,  // EM radiation
            energy_history: Vec::new(),
            void_fraction_history: Vec::new(),
            filament_fraction_history: Vec::new(),
            max_history: 500,
            last_step_time: Instant::now(),
        })
    }
    
    /// Handle keyboard input
    fn handle_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') => return false, // Quit
            KeyCode::Char(' ') => self.is_running = !self.is_running, // Toggle pause
            KeyCode::Char('s') => {
                // Single step
                if !self.is_running {
                    self.step_simulation();
                }
            }
            KeyCode::Char('r') => {
                // Reset simulation
                // TODO: Implement reset logic
            }
            KeyCode::Char('[') => {
                // Decrease timestep
                self.dt = (self.dt * 0.9).max(0.001);
            }
            KeyCode::Char(']') => {
                // Increase timestep
                self.dt = (self.dt * 1.1).min(0.1);
            }
            KeyCode::Up => {
                // Navigate up through Z-slices
                let (_, _, sz) = self.simulation.lattice.size();
                self.current_z_slice = (self.current_z_slice + 1).min(sz - 1);
            }
            KeyCode::Down => {
                // Navigate down through Z-slices
                self.current_z_slice = self.current_z_slice.saturating_sub(1);
            }
            KeyCode::Left => {
                // Cycle to previous variable
                self.current_variable = (self.current_variable + VARS - 1) % VARS;
            }
            KeyCode::Right => {
                // Cycle to next variable
                self.current_variable = (self.current_variable + 1) % VARS;
            }
            _ => {}
        }
        true // Continue running
    }
    
    /// Advance simulation by one timestep
    fn step_simulation(&mut self) {
        // Step the simulation
        if let Err(e) = self.simulation.step(self.dt, false) {
            eprintln!("Simulation error: {}", e);
            return;
        }
        
        // Update history
        // TODO: Get actual metrics from simulation
        let total_energy = 0.0; // Placeholder
        self.energy_history.push(total_energy);
        
        // Trim history if too long
        if self.energy_history.len() > self.max_history {
            self.energy_history.remove(0);
        }
        
        // TODO: Compute pattern metrics
        // let metrics = self.simulation.compute_pattern_metrics();
        // self.void_fraction_history.push(metrics.void_fraction);
        // self.filament_fraction_history.push(metrics.filament_fraction);
    }
    
    /// Main update loop called each frame
    fn update(&mut self) {
        if self.is_running {
            // Calculate if enough time has passed for next step
            let elapsed = self.last_step_time.elapsed();
            let step_duration = Duration::from_secs_f64(self.dt);
            
            if elapsed >= step_duration {
                self.step_simulation();
                self.last_step_time = Instant::now();
            }
        }
    }
}

// ============================================================================
// UI Rendering
// ============================================================================

/// Main UI rendering function
fn ui(frame: &mut Frame, app: &mut App) {
    let size = frame.area();
    
    // Split terminal into main areas:
    // - Top: Title/status bar
    // - Middle: Main visualization area (split left/right)
    // - Bottom: Controls help
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(10),     // Main content
            Constraint::Length(3),  // Footer
        ])
        .split(size);
    
    // Render header
    render_header(frame, app, main_layout[0]);
    
    // Split main content area into left (lattice view) and right (charts)
    let content_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(main_layout[1]);
    
    // Render lattice visualization
    render_lattice_view(frame, app, content_layout[0]);
    
    // Render metrics charts
    render_metrics_panel(frame, app, content_layout[1]);
    
    // Render footer with controls
    render_footer(frame, main_layout[2]);
}

/// Render the header with simulation status
fn render_header(frame: &mut Frame, app: &App, area: Rect) {
    let status = if app.is_running { "RUNNING" } else { "PAUSED" };
    let status_color = if app.is_running { Color::Green } else { Color::Yellow };
    
    let var_names = ["EM", "Baryons", "Neutrinos", "Unknown₁", "Unknown₂"];
    
    let header_text = vec![
        Line::from(vec![
            Span::styled("VOID", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" – Visualizing Oscillations in Invisible Density"),
        ]),
        Line::from(vec![
            Span::raw("Status: "),
            Span::styled(status, Style::default().fg(status_color)),
            Span::raw(format!(" | Step: {} | Time: {:.3} | dt: {:.4} | ",
                app.simulation.step, app.simulation.time, app.dt)),
            Span::raw("Variable: "),
            Span::styled(var_names[app.current_variable], Style::default().fg(Color::Magenta)),
            Span::raw(format!(" | Z-Slice: {}", app.current_z_slice)),
        ]),
    ];
    
    let header = Paragraph::new(header_text)
        .block(Block::default().borders(Borders::ALL));
    
    frame.render_widget(header, area);
}

/// Render the 2D lattice slice view
fn render_lattice_view(frame: &mut Frame, app: &App, area: Rect) {
    // TODO: Extract 2D slice from lattice at current_z_slice
    // TODO: Convert energy values to ASCII art characters
    // For now, placeholder
    
    let placeholder_text = vec![
        Line::from("Lattice visualization will appear here"),
        Line::from(""),
        Line::from("ASCII art rendering of energy density:"),
        Line::from("  . = low energy"),
        Line::from("  : = medium energy"),
        Line::from("  # = high energy"),
        Line::from(""),
        Line::from("TODO: Implement lattice slice extraction"),
        Line::from("      and ASCII rendering from the core library"),
    ];
    
    let lattice_view = Paragraph::new(placeholder_text)
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Energy Lattice (2D Slice)"));
    
    frame.render_widget(lattice_view, area);
}

/// Render the metrics and charts panel
fn render_metrics_panel(frame: &mut Frame, app: &App, area: Rect) {
    // Split metrics panel into multiple chart areas
    let charts_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(33),
            Constraint::Percentage(33),
            Constraint::Percentage(34),
        ])
        .split(area);
    
    // Render energy conservation chart
    render_energy_chart(frame, app, charts_layout[0]);
    
    // Render pattern metrics sparklines
    render_pattern_sparklines(frame, app, charts_layout[1]);
    
    // Render statistics panel
    render_statistics(frame, app, charts_layout[2]);
}

/// Render energy conservation history chart
fn render_energy_chart(frame: &mut Frame, app: &App, area: Rect) {
    if app.energy_history.is_empty() {
        let placeholder = Paragraph::new("No data yet...")
            .block(Block::default().borders(Borders::ALL).title("Energy Conservation"));
        frame.render_widget(placeholder, area);
        return;
    }
    
    // TODO: Create actual Chart widget with energy history
    // For now, show sparkline
    let sparkline_data: Vec<u64> = app.energy_history.iter()
        .map(|&e| (e.abs() * 100.0) as u64)
        .collect();
    
    let sparkline = Sparkline::default()
        .data(&sparkline_data)
        .style(Style::default().fg(Color::Green))
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Total Energy"));
    
    frame.render_widget(sparkline, area);
}

/// Render pattern metrics as sparklines
fn render_pattern_sparklines(frame: &mut Frame, app: &App, area: Rect) {
    let metrics_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);
    
    // Void fraction sparkline
    if !app.void_fraction_history.is_empty() {
        let data: Vec<u64> = app.void_fraction_history.iter()
            .map(|&f| (f * 100.0) as u64)
            .collect();
        
        let sparkline = Sparkline::default()
            .data(&data)
            .style(Style::default().fg(Color::Blue))
            .block(Block::default().borders(Borders::ALL).title("Void Fraction"));
        
        frame.render_widget(sparkline, metrics_layout[0]);
    }
    
    // Filament fraction sparkline
    if !app.filament_fraction_history.is_empty() {
        let data: Vec<u64> = app.filament_fraction_history.iter()
            .map(|&f| (f * 100.0) as u64)
            .collect();
        
        let sparkline = Sparkline::default()
            .data(&data)
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Filament Fraction"));
        
        frame.render_widget(sparkline, metrics_layout[1]);
    }
}

/// Render current statistics
fn render_statistics(frame: &mut Frame, app: &App, area: Rect) {
    // TODO: Get actual metrics from simulation
    let stats_text = vec![
        Line::from(vec![
            Span::raw("Total Energy: "),
            Span::styled("0.000", Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::raw("Conservation Error: "),
            Span::styled("< 1e-12", Style::default().fg(Color::Green)),
        ]),
        Line::from(""),
        Line::from("Pattern Metrics:"),
        Line::from(vec![
            Span::raw("  Void Fraction: "),
            Span::styled("N/A", Style::default().fg(Color::Gray)),
        ]),
        Line::from(vec![
            Span::raw("  Filament Fraction: "),
            Span::styled("N/A", Style::default().fg(Color::Gray)),
        ]),
        Line::from(vec![
            Span::raw("  Clustering Dim: "),
            Span::styled("N/A", Style::default().fg(Color::Gray)),
        ]),
    ];
    
    let stats_panel = Paragraph::new(stats_text)
        .block(Block::default().borders(Borders::ALL).title("Statistics"));
    
    frame.render_widget(stats_panel, area);
}

/// Render the footer with keyboard controls
fn render_footer(frame: &mut Frame, area: Rect) {
    let help_text = Line::from(vec![
        Span::raw("Controls: "),
        Span::styled("Space", Style::default().fg(Color::Cyan)),
        Span::raw("=Play/Pause "),
        Span::styled("s", Style::default().fg(Color::Cyan)),
        Span::raw("=Step "),
        Span::styled("[/]", Style::default().fg(Color::Cyan)),
        Span::raw("=dt "),
        Span::styled("↑↓", Style::default().fg(Color::Cyan)),
        Span::raw("=Z-slice "),
        Span::styled("←→", Style::default().fg(Color::Cyan)),
        Span::raw("=Variable "),
        Span::styled("q", Style::default().fg(Color::Red)),
        Span::raw("=Quit"),
    ]);
    
    let footer = Paragraph::new(help_text)
        .block(Block::default().borders(Borders::ALL))
        .centered();
    
    frame.render_widget(footer, area);
}

// ============================================================================
// Terminal Management
// ============================================================================

/// Initialize the terminal
fn init_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}

/// Restore the terminal to normal mode
fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}

// ============================================================================
// Main Event Loop
// ============================================================================

fn main() -> Result<()> {
    // Initialize error handling
    color_eyre::install()?;
    
    // Create application state
    let mut app = App::new()?;
    
    // Initialize terminal
    let mut terminal = init_terminal()?;
    
    // Main event loop
    let result = run_app(&mut terminal, &mut app);
    
    // Restore terminal
    restore_terminal(&mut terminal)?;
    
    // Return result
    result
}

/// Run the main application loop
fn run_app(
    terminal: &mut Terminal<CrosstermBackend<Stdout>>,
    app: &mut App,
) -> Result<()> {
    let frame_duration = Duration::from_millis(1000 / app.target_fps as u64);
    let mut last_frame = Instant::now();
    
    loop {
        // Handle input events (non-blocking)
        if event::poll(Duration::from_millis(1))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if !app.handle_key(key) {
                        break; // User requested quit
                    }
                }
            }
        }
        
        // Update simulation state
        app.update();
        
        // Render UI at target FPS
        if last_frame.elapsed() >= frame_duration {
            terminal.draw(|frame| ui(frame, app))?;
            last_frame = Instant::now();
        }
    }
    
    Ok(())
}
