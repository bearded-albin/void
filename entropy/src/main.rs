//! Entropy - Terminal UI Dashboard for VOID Simulations
//!
//! This binary currently provides a **visual shell** around the
//! low-level lattice-based VOID simulation. The high-level
//! `Simulation` orchestration API in `evolution.rs` is still a
//! TODO layer; to keep the project in a running state, the TUI
//! uses only the *implemented* primitives:
//!
//! - `generate_lattice` – create a 3D lattice with random energy
//! - `simulate_tick` – advance the lattice by one parallel update
//!
//! Once the `Simulation` type is fully implemented, this TUI can
//! be wired directly to it with minimal changes.
//!
//! # Usage
//!
//! ```bash
//! cargo run --release -p entropy
//! ```
//!
//! The view currently focuses on:
//! - A status/header bar
//! - A placeholder lattice panel
//! - A simple energy history sparkline

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
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Sparkline},
    Frame, Terminal,
};
use std::{
    io::{stdout, Stdout},
    time::{Duration, Instant},
};

// Import the currently working low-level API from entropic_void
use entropic_void::{generate_lattice, simulate_tick, Lattice};

// ============================================================================
// Application State
// ============================================================================

/// Main application state holding simulation and UI state
struct App {
    /// The core lattice state (low-level simulation primitive)
    lattice: Lattice,

    /// Whether simulation is currently running
    is_running: bool,

    /// Logical timestep size used for time accounting (not in physics yet)
    dt: f64,

    /// Target FPS for rendering
    target_fps: u32,

    /// Current Z-slice being displayed (0 to size.2-1)
    current_z_slice: usize,

    /// History of total energy for plotting
    energy_history: Vec<f64>,

    /// Maximum history length for charts
    max_history: usize,

    /// Timestamp of last simulation step
    last_step_time: Instant,

    /// Logical simulation time (dt * steps)
    sim_time: f64,

    /// Number of ticks performed
    steps: u64,

    /// Base RNG seed used for simulate_tick
    seed_base: u64,
}

impl App {
    /// Create a new application with default simulation parameters
    fn new() -> Result<Self> {
        // Initialize a 32x32x32 lattice with some total energy budget
        let size = (32, 32, 32);
        let total_energy = 1.0e6;
        let lattice = generate_lattice(42, size, total_energy);

        let current_z_slice = size.2 / 2;

        Ok(Self {
            lattice,
            is_running: true,
            dt: 0.01,
            target_fps: 30,
            current_z_slice,
            energy_history: Vec::new(),
            max_history: 500,
            last_step_time: Instant::now(),
            sim_time: 0.0,
            steps: 0,
            seed_base: 1234,
        })
    }

    /// Handle keyboard input
    fn handle_key(&mut self, key: KeyEvent) -> bool {
        match key.code {
            KeyCode::Char('q') => return false, // Quit
            KeyCode::Char(' ') => self.is_running = !self.is_running, // Toggle pause
            KeyCode::Char('s') => {
                // Single step when paused
                if !self.is_running {
                    self.step_simulation();
                }
            }
            KeyCode::Char('r') => {
                // Reset simulation
                let size = self.lattice.size;
                let total_energy = 1.0e6;
                self.lattice = generate_lattice(42, size, total_energy);
                self.energy_history.clear();
                self.sim_time = 0.0;
                self.steps = 0;
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
                let (_, _, sz) = self.lattice.size;
                if sz > 0 {
                    self.current_z_slice = (self.current_z_slice + 1).min(sz - 1);
                }
            }
            KeyCode::Down => {
                // Navigate down through Z-slices
                if self.current_z_slice > 0 {
                    self.current_z_slice -= 1;
                }
            }
            _ => {}
        }
        true // Continue running
    }

    /// Advance simulation by one lattice tick and update diagnostics
    fn step_simulation(&mut self) {
        // Use a changing seed so each tick is slightly different
        simulate_tick(&mut self.lattice, self.seed_base + self.steps);

        self.steps += 1;
        self.sim_time += self.dt;

        // Compute total energy for diagnostics
        let total_energy: f64 = self
            .lattice
            .cells
            .iter()
            .map(|c| c.total_energy)
            .sum();
        self.energy_history.push(total_energy);

        // Trim history if too long
        if self.energy_history.len() > self.max_history {
            let excess = self.energy_history.len() - self.max_history;
            self.energy_history.drain(0..excess);
        }
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

    // Layout:
    // - Top: Title/status bar
    // - Middle: Left (lattice placeholder), Right (metrics)
    // - Bottom: Controls
    let main_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(10),    // Main content
            Constraint::Length(3),  // Footer
        ])
        .split(size);

    render_header(frame, app, main_layout[0]);

    let content_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(main_layout[1]);

    render_lattice_view(frame, app, content_layout[0]);
    render_metrics_panel(frame, app, content_layout[1]);

    render_footer(frame, main_layout[2]);
}

/// Render the header with simulation status
fn render_header(frame: &mut Frame, app: &App, area: Rect) {
    let status = if app.is_running { "RUNNING" } else { "PAUSED" };
    let status_color = if app.is_running { Color::Green } else { Color::Yellow };

    let (_, _, sz) = app.lattice.size;
    let current_energy = app.energy_history.last().copied().unwrap_or(0.0);

    let header_text = vec![
        Line::from(vec![
            Span::styled(
                "VOID",
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
            Span::raw(" – Visualizing Oscillations in Invisible Density"),
        ]),
        Line::from(vec![
            Span::raw("Status: "),
            Span::styled(status, Style::default().fg(status_color)),
            Span::raw(format!(
                " | Step: {} | Time: {:.3} | dt: {:.4} | Z-Slice: {}/{} | E_tot: {:.3e}",
                app.steps,
                app.sim_time,
                app.dt,
                app.current_z_slice,
                sz.saturating_sub(1),
                current_energy,
            )),
        ]),
    ];

    let header = Paragraph::new(header_text).block(Block::default().borders(Borders::ALL));

    frame.render_widget(header, area);
}

/// Render the 2D lattice slice view (currently placeholder)
fn render_lattice_view(frame: &mut Frame, app: &App, area: Rect) {
    // NOTE: The new lattice module (`entropic_void::lattice`) will eventually
    // provide rich helpers for extracting 2D slices and visualizing fields.
    // For now, we render a textual placeholder while the physics tooling
    // and higher-level Simulation API are being built out.

    let placeholder_text = vec![
        Line::from("Lattice visualization placeholder"),
        Line::from(""),
        Line::from("Current design:").into(),
        Line::from("  - Using low-level lattice + simulate_tick"),
        Line::from("  - Higher-level Simulation API is still TODO"),
        Line::from(""),
        Line::from("Future plan:"),
        Line::from("  - Extract 2D slices at fixed z"),
        Line::from("  - Map energy density → ASCII gradient"),
        Line::from("  - Overlay variable/force selections"),
    ];

    let lattice_view = Paragraph::new(placeholder_text)
        .block(Block::default().borders(Borders::ALL).title("Energy Lattice (2D Slice)"));

    frame.render_widget(lattice_view, area);
}

/// Render the metrics and charts panel
fn render_metrics_panel(frame: &mut Frame, app: &App, area: Rect) {
    let charts_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    render_energy_chart(frame, app, charts_layout[0]);
    render_statistics(frame, app, charts_layout[1]);
}

/// Render energy conservation history chart (sparkline)
fn render_energy_chart(frame: &mut Frame, app: &App, area: Rect) {
    if app.energy_history.is_empty() {
        let placeholder = Paragraph::new("No energy samples yet…")
            .block(Block::default().borders(Borders::ALL).title("Total Energy"));
        frame.render_widget(placeholder, area);
        return;
    }

    let min = app
        .energy_history
        .iter()
        .cloned()
        .fold(f64::INFINITY, f64::min);
    let max = app
        .energy_history
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);

    let scale = (max - min).max(1e-9);
    let sparkline_data: Vec<u64> = app
        .energy_history
        .iter()
        .map(|&e| (((e - min) / scale) * 100.0) as u64)
        .collect();

    let sparkline = Sparkline::default()
        .data(&sparkline_data)
        .style(Style::default().fg(Color::Green))
        .block(Block::default().borders(Borders::ALL).title("Total Energy (normalized)"));

    frame.render_widget(sparkline, area);
}

/// Render current statistics (very basic for now)
fn render_statistics(frame: &mut Frame, app: &App, area: Rect) {
    let current_energy = app.energy_history.last().copied().unwrap_or(0.0);

    let stats_text = vec![
        Line::from(vec![
            Span::raw("Steps: "),
            Span::styled(app.steps.to_string(), Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::raw("Sim Time: "),
            Span::styled(format!("{:.3}", app.sim_time), Style::default().fg(Color::Green)),
        ]),
        Line::from(vec![
            Span::raw("Total Energy (last): "),
            Span::styled(format!("{:.3e}", current_energy), Style::default().fg(Color::Green)),
        ]),
        Line::from(""),
        Line::from("Pattern metrics (voids/filaments/etc.) will be wired in"),
        Line::from("once conservation.rs and evolution.rs are fully implemented."),
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
        Span::styled("r", Style::default().fg(Color::Cyan)),
        Span::raw("=Reset "),
        Span::styled("[/]", Style::default().fg(Color::Cyan)),
        Span::raw("=dt "),
        Span::styled("↑↓", Style::default().fg(Color::Cyan)),
        Span::raw("=Z-slice "),
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
