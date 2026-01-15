//! User interface components and rendering
//!
//! This module provides all UI elements and rendering logic for the plugin.
//! It uses ANSI escape codes and Zellij's native UI components to render:
//!
//! - Diff viewer with scrolling and hunk navigation
//! - Prompt entry dialog
//! - Provider selection menu
//! - Confirmation dialogs
//! - Error messages
//!
//! # Submodules
//!
//! - [`renderer`]: The `Renderable` trait and rendering context
//! - [`components`]: Reusable UI components
//! - [`diff_view`]: Diff-specific rendering logic
//! - [`colors`]: Color schemes and ANSI escape codes

pub mod colors;
pub mod components;
pub mod diff_view;
pub mod renderer;

pub use colors::Colors;
pub use renderer::{RenderContext, Renderer};

pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";

pub fn truncate_line(line: &str, max_width: usize) -> String {
    use unicode_width::UnicodeWidthStr;

    if line.width() <= max_width {
        line.to_string()
    } else {
        let mut result = String::new();
        let mut width = 0;

        for ch in line.chars() {
            let ch_width = unicode_width::UnicodeWidthChar::width(ch).unwrap_or(0);
            if width + ch_width + 1 > max_width {
                result.push('…');
                break;
            }
            result.push(ch);
            width += ch_width;
        }

        result
    }
}

pub fn center_text(text: &str, width: usize) -> String {
    use unicode_width::UnicodeWidthStr;
    let text_width = text.width();
    if text_width >= width {
        text.to_string()
    } else {
        let padding = (width - text_width) / 2;
        format!(
            "{}{}{}",
            " ".repeat(padding),
            text,
            " ".repeat(width - padding - text_width)
        )
    }
}

/// Clear the screen and move cursor to top-left
pub fn clear_screen() {
    print!("\x1b[2J\x1b[H");
}

/// Move cursor to a specific position (1-indexed)
pub fn move_cursor(row: usize, col: usize) {
    print!("\x1b[{};{}H", row, col);
}
