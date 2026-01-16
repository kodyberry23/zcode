// src/ui/editor.rs - External editor integration with TUI suspend/resume
//
// This module implements the Ratatui best practice for spawning external
// editors like Neovim/Vim. The pattern is:
// 1. Suspend TUI (disable raw mode, leave alternate screen)
// 2. Launch editor in blocking mode
// 3. Wait for editor to exit
// 4. Resume TUI (enable raw mode, enter alternate screen, redraw)

use anyhow::Result;
use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

/// Open a file in an external editor (Neovim or $EDITOR).
/// Suspends the TUI, launches the editor, waits for exit, then resumes the TUI.
pub fn open_file_in_editor(
    terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    path: &Path,
    line: Option<usize>,
) -> Result<()> {
    // Step 1: Suspend TUI
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;

    // Step 2: Launch editor
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nvim".to_string());

    // Check if editor exists
    if !command_exists(&editor) {
        // Resume TUI first before showing error
        enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen)?;
        terminal.clear()?;
        anyhow::bail!(
            "Editor '{}' not found. Please install Neovim or set $EDITOR",
            editor
        );
    }

    let mut cmd = Command::new(&editor);
    cmd.arg(path);

    // Open at specific line if provided
    if let Some(line_num) = line {
        // Neovim: +{line} opens at specific line
        cmd.arg(format!("+{}", line_num));
    }

    // Step 3: Wait for editor to exit
    let status = cmd.status()?;

    // Step 4: Resume TUI
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    terminal.clear()?;

    if !status.success() {
        anyhow::bail!("Editor exited with code: {:?}", status.code());
    }

    Ok(())
}

/// Check if a command exists in PATH
fn command_exists(cmd: &str) -> bool {
    Command::new("which")
        .arg(cmd)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Suspend the TUI temporarily (for spawning any external process)
pub fn suspend_tui() -> Result<()> {
    disable_raw_mode()?;
    execute!(io::stdout(), LeaveAlternateScreen)?;
    Ok(())
}

/// Resume the TUI after suspension
pub fn resume_tui(
    terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
) -> Result<()> {
    enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen)?;
    terminal.clear()?;
    Ok(())
}
