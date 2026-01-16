// src/ui/colors.rs - Color scheme definitions using Ratatui

use ratatui::style::{Color, Modifier, Style};

#[derive(Debug, Clone)]
pub struct Theme {
    // Diff colors
    pub added_style: Style,
    pub removed_style: Style,
    pub context_style: Style,

    // UI colors
    pub header_style: Style,
    pub selected_style: Style,
    pub status_accepted: Style,
    pub status_rejected: Style,
    pub status_pending: Style,
    pub error_style: Style,
    pub prompt_style: Style,
    pub normal_style: Style,
    pub border_style: Style,
}

impl Theme {
    /// Dark theme optimized for OpenCode-style minimal look
    pub fn dark() -> Self {
        // Palette inspired by OpenCode's flat dark UI
        // Background: near-black, surfaces: dark gray, text: soft whites
        // Accents: subtle blue for interactive, green for success, red for errors
        let surface_border = Color::Rgb(68, 68, 68); // #444
        let text_primary = Color::Rgb(230, 230, 230); // #e6e6e6
        let text_muted = Color::Rgb(150, 150, 150); // #969696
        let accent_blue = Color::Rgb(120, 170, 255); // soft blue
        let success_green = Color::Rgb(120, 200, 140);
        let error_red = Color::Rgb(240, 120, 120);
        let warning_yellow = Color::Rgb(235, 200, 120);

        Self {
            added_style: Style::default()
                .fg(success_green)
                .bg(Color::Rgb(18, 36, 26)),
            removed_style: Style::default().fg(error_red).bg(Color::Rgb(44, 20, 20)),
            context_style: Style::default().fg(text_muted),
            header_style: Style::default()
                .fg(text_primary)
                .add_modifier(Modifier::DIM),
            selected_style: Style::default()
                .fg(text_primary)
                .bg(Color::Rgb(32, 32, 32))
                .add_modifier(Modifier::BOLD),
            status_accepted: Style::default()
                .fg(success_green)
                .add_modifier(Modifier::BOLD),
            status_rejected: Style::default().fg(error_red).add_modifier(Modifier::BOLD),
            status_pending: Style::default()
                .fg(warning_yellow)
                .add_modifier(Modifier::BOLD),
            error_style: Style::default().fg(error_red).add_modifier(Modifier::BOLD),
            prompt_style: Style::default()
                .fg(text_primary)
                .bg(Color::Rgb(22, 22, 22))
                .add_modifier(Modifier::BOLD),
            normal_style: Style::default().fg(text_primary),
            border_style: Style::default().fg(surface_border),
        }
    }

    /// Style for deletion decorations (strikethrough, dimmed)
    pub fn deletion_style(&self) -> Style {
        self.removed_style.add_modifier(Modifier::CROSSED_OUT)
    }

    /// Style for addition decorations (green background)
    pub fn addition_style(&self) -> Style {
        self.added_style
    }

    /// Style for pending markers
    pub fn pending_marker(&self) -> Style {
        self.status_pending
    }

    /// Style for accepted markers
    pub fn accepted_marker(&self) -> Style {
        self.status_accepted
    }

    /// Style for rejected markers
    pub fn rejected_marker(&self) -> Style {
        self.status_rejected
    }

    pub fn light() -> Self {
        Self {
            added_style: Style::default()
                .fg(Color::Indexed(22)) // Dark green
                .bg(Color::Indexed(194)), // Light green
            removed_style: Style::default()
                .fg(Color::Indexed(124)) // Dark red
                .bg(Color::Indexed(224)), // Light red
            context_style: Style::default().fg(Color::Indexed(240)), // Dark gray
            header_style: Style::default()
                .fg(Color::Indexed(25)) // Dark blue
                .bg(Color::Indexed(254)) // Light gray
                .add_modifier(Modifier::BOLD),
            selected_style: Style::default()
                .bg(Color::Indexed(252)) // Light gray
                .add_modifier(Modifier::BOLD),
            status_accepted: Style::default()
                .fg(Color::Indexed(28)) // Dark green
                .add_modifier(Modifier::BOLD),
            status_rejected: Style::default()
                .fg(Color::Indexed(160)) // Dark red
                .add_modifier(Modifier::BOLD),
            status_pending: Style::default()
                .fg(Color::Indexed(136)) // Brown/orange
                .add_modifier(Modifier::BOLD),
            error_style: Style::default()
                .fg(Color::Indexed(160)) // Dark red
                .add_modifier(Modifier::BOLD),
            prompt_style: Style::default()
                .fg(Color::Indexed(91)) // Dark purple
                .add_modifier(Modifier::BOLD),
            normal_style: Style::default().fg(Color::Black),
            border_style: Style::default().fg(Color::Indexed(240)),
        }
    }
}

// Keep old Colors struct for backward compatibility during migration
#[derive(Debug, Clone)]
pub struct Colors {
    pub added_bg: &'static str,
    pub added_fg: &'static str,
    pub removed_bg: &'static str,
    pub removed_fg: &'static str,
    pub context_fg: &'static str,
    pub header_bg: &'static str,
    pub header_fg: &'static str,
    pub selected_bg: &'static str,
    pub status_accepted: &'static str,
    pub status_rejected: &'static str,
    pub status_pending: &'static str,
    pub error_fg: &'static str,
    pub prompt_fg: &'static str,
}

impl Colors {
    pub fn dark() -> Self {
        Self {
            added_bg: "\x1b[48;5;22m",
            added_fg: "\x1b[38;5;114m",
            removed_bg: "\x1b[48;5;52m",
            removed_fg: "\x1b[38;5;210m",
            context_fg: "\x1b[38;5;250m",
            header_bg: "\x1b[48;5;236m",
            header_fg: "\x1b[38;5;75m",
            selected_bg: "\x1b[48;5;238m",
            status_accepted: "\x1b[38;5;114m",
            status_rejected: "\x1b[38;5;210m",
            status_pending: "\x1b[38;5;220m",
            error_fg: "\x1b[38;5;196m",
            prompt_fg: "\x1b[38;5;141m",
        }
    }

    pub fn light() -> Self {
        Self {
            added_bg: "\x1b[48;5;194m",
            added_fg: "\x1b[38;5;22m",
            removed_bg: "\x1b[48;5;224m",
            removed_fg: "\x1b[38;5;124m",
            context_fg: "\x1b[38;5;240m",
            header_bg: "\x1b[48;5;254m",
            header_fg: "\x1b[38;5;25m",
            selected_bg: "\x1b[48;5;252m",
            status_accepted: "\x1b[38;5;28m",
            status_rejected: "\x1b[38;5;160m",
            status_pending: "\x1b[38;5;136m",
            error_fg: "\x1b[38;5;160m",
            prompt_fg: "\x1b[38;5;91m",
        }
    }
}
