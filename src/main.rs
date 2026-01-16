// src/main.rs - Standalone Ratatui terminal application

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod app;
mod components;
mod config;
mod diff;
mod error;
mod events;
mod executor;
mod file_ops;
mod input;
mod message;
mod model;
mod neovim;
mod parsers;
mod providers;
mod session;
mod state;
mod ui;

use anyhow::Result;
use crossterm::{
    cursor,
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{self, Write};
use std::panic;
use std::time::Duration;

use app::App;

/// Restore terminal to normal state
/// This is called on normal exit and on panic
fn restore_terminal() {
    // Best effort to restore terminal - ignore errors
    let _ = disable_raw_mode();
    let _ = execute!(
        io::stdout(),
        LeaveAlternateScreen,
        DisableMouseCapture,
        cursor::Show
    );
    // Flush stdout to ensure all escape sequences are sent
    let _ = io::stdout().flush();
}

/// Install panic hook that restores terminal before printing panic info
fn install_panic_hook() {
    let original_hook = panic::take_hook();
    panic::set_hook(Box::new(move |panic_info| {
        // Restore terminal first
        restore_terminal();
        // Then call the original panic handler
        original_hook(panic_info);
    }));
}

#[tokio::main]
async fn main() -> Result<()> {
    // Install panic hook early to catch any panics during setup
    install_panic_hook();

    // Check if we're running in a terminal
    if !atty::is(atty::Stream::Stdout) {
        eprintln!("Error: zcode must be run in a terminal (TTY)");
        eprintln!("stdout is not a terminal. Are you piping output?");
        eprintln!("\nHow to run zcode:");
        eprintln!("  cargo run                    (direct run)");
        eprintln!("  ./target/release/zcode       (run binary)");
        eprintln!("  cargo run 2> debug.log       (save logs to file, keep UI)");
        eprintln!("\nNote: TUI applications require a real terminal to display.");
        std::process::exit(1);
    }

    // Run the application with proper terminal handling
    let result = run().await;

    // Always restore terminal on exit
    restore_terminal();

    // Report any errors after terminal is restored
    if let Err(err) = result {
        eprintln!("Error: {:?}", err);
        std::process::exit(1);
    }

    Ok(())
}

async fn run() -> Result<()> {
    // Setup terminal
    enable_raw_mode().map_err(|e| {
        anyhow::anyhow!(
            "Failed to enable raw mode: {}. Make sure you're running in a proper terminal.",
            e
        )
    })?;

    let mut stdout = io::stdout();
    execute!(
        stdout,
        EnterAlternateScreen,
        EnableMouseCapture,
        cursor::Hide
    )?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Clear the screen on startup for a clean slate
    terminal.clear()?;

    // Create app
    let mut app = App::new()?;

    // Run the application
    app.run(&mut terminal).await
}
