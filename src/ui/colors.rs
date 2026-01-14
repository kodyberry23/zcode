// src/ui/colors.rs - Color scheme definitions

#[derive(Debug, Clone)]
pub struct Colors {
    // Diff colors
    pub added_bg: &'static str,
    pub added_fg: &'static str,
    pub removed_bg: &'static str,
    pub removed_fg: &'static str,
    pub context_fg: &'static str,

    // UI colors
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
            added_bg: "\x1b[48;5;22m",     // Dark green
            added_fg: "\x1b[38;5;114m",    // Light green
            removed_bg: "\x1b[48;5;52m",   // Dark red
            removed_fg: "\x1b[38;5;210m",  // Light red
            context_fg: "\x1b[38;5;250m",  // Light gray
            header_bg: "\x1b[48;5;236m",   // Dark gray
            header_fg: "\x1b[38;5;75m",    // Cyan
            selected_bg: "\x1b[48;5;238m", // Slightly lighter gray
            status_accepted: "\x1b[38;5;114m",
            status_rejected: "\x1b[38;5;210m",
            status_pending: "\x1b[38;5;220m",
            error_fg: "\x1b[38;5;196m",  // Bright red
            prompt_fg: "\x1b[38;5;141m", // Purple
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
