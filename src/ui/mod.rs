//! User interface components and rendering
//!
//! This module provides all UI elements and rendering logic for the application.
//! It uses Ratatui widgets and ANSI escape codes to render:
//!
//! - Diff viewer with scrolling and hunk navigation
//! - Prompt entry dialog
//! - Provider selection menu
//! - Confirmation dialogs
//! - Error messages
//! - ASCII logo splash screen
//!
//! # Submodules
//!
//! - [`renderers`]: Ratatui-based rendering functions
//! - [`colors`]: Color schemes and style definitions
//! - [`layout`]: Layout helper functions
//! - [`logo`]: ASCII logo rendering

pub mod chat_history;
pub mod colors;
pub mod editor;
pub mod header;
pub mod help;
pub mod layout;
pub mod logo;
pub mod overlay_diff;
pub mod prompt_input;
pub mod renderers;
pub mod search;
pub mod session_turn;
pub mod sidebar;
pub mod status_bar;
pub mod theme;
pub mod widgets;

pub use colors::Colors;
// Legacy ANSI constants retained for any remaining non-ratatui render paths
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

// Note: Ratatui handles terminal clearing and cursor positioning automatically
