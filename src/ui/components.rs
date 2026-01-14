// src/ui/components.rs - Reusable UI components

use super::{Colors, BOLD, DIM, RESET};
use crate::state::{ChangeTag, Hunk, HunkStatus, LineChange, ProviderInfo, State};

/// Header component showing plugin name and current context
pub struct Header;

impl Header {
    pub fn render(state: &State, colors: &Colors, width: usize) {
        let provider_name = state
            .provider
            .as_ref()
            .map(|p| p.name())
            .unwrap_or("No Provider");

        let file_count = state.pending_changes.len();
        let hunk_count = state.hunks.len();

        let info = format!(
            "  ZCode │ {} │ {} files │ {} hunks",
            provider_name, file_count, hunk_count
        );

        let padding = width.saturating_sub(info.len());
        println!(
            "{}{}{}{}{}",
            colors.header_bg,
            colors.header_fg,
            info,
            " ".repeat(padding),
            RESET
        );
    }
}

/// Footer component showing keyboard shortcuts
pub struct Footer;

impl Footer {
    pub fn render(colors: &Colors, width: usize, help_text: &str) {
        let padding_start = (width.saturating_sub(help_text.len())) / 2;
        let padding_end = width.saturating_sub(help_text.len() + padding_start);

        println!(
            "{}{}{}{}{}{}",
            colors.header_bg,
            colors.header_fg,
            " ".repeat(padding_start),
            help_text,
            " ".repeat(padding_end),
            RESET
        );
    }
}

/// Status line showing current operation
pub struct StatusLine;

impl StatusLine {
    pub fn render_processing(colors: &Colors, width: usize, message: &str) {
        let status = format!("▶ {}", message);
        let padding = width.saturating_sub(status.len());
        println!(
            "{}{}{}{}{}",
            colors.prompt_fg,
            BOLD,
            status,
            " ".repeat(padding),
            RESET
        );
    }

    pub fn render_info(colors: &Colors, width: usize, message: &str) {
        let status = format!("ℹ {}", message);
        let padding = width.saturating_sub(status.len());
        println!(
            "{}{}{}{}",
            colors.context_fg,
            status,
            " ".repeat(padding),
            RESET
        );
    }
}

/// Provider selection menu
pub struct ProviderMenu;

impl ProviderMenu {
    pub fn render(
        providers: &[ProviderInfo],
        selected_idx: usize,
        colors: &Colors,
        rows: usize,
        cols: usize,
    ) {
        if providers.is_empty() {
            println!(
                "{}No AI providers detected. Please install Claude Code, Aider, GitHub Copilot, or Amazon Q.{}",
                colors.error_fg, RESET
            );
            return;
        }

        println!(
            "{}{}Select an AI Provider:{}\\n",
            colors.header_fg, BOLD, RESET
        );

        for (idx, provider) in providers.iter().enumerate() {
            let is_selected = idx == selected_idx;
            let marker = if is_selected { "▶" } else { " " };
            let status_icon = if provider.available { "✓" } else { "✗" };

            let bg = if is_selected { colors.selected_bg } else { "" };
            let fg = if provider.available {
                colors.status_accepted
            } else {
                colors.status_rejected
            };

            println!(
                "{}{} {} {} {} {}{}",
                bg, marker, status_icon, provider.name, provider.cli_command, fg, RESET
            );
        }
    }
}

/// Text input component with cursor
pub struct PromptInput;

impl PromptInput {
    pub fn render(prompt: &str, text: &str, cursor_pos: usize, colors: &Colors, width: usize) {
        println!("{}{}{}:{}", colors.prompt_fg, BOLD, prompt, RESET);

        // Render text with cursor
        let mut display = String::new();
        for (idx, ch) in text.chars().enumerate() {
            if idx == cursor_pos {
                display.push('│');
            }
            display.push(ch);
        }
        if cursor_pos >= text.len() {
            display.push('│');
        }

        let truncated = if display.len() > width.saturating_sub(2) {
            format!("...{}", &display[display.len() - width.saturating_sub(5)..])
        } else {
            display
        };

        println!("{}{}{}", colors.context_fg, truncated, RESET);
    }
}

/// Confirmation dialog
pub struct ConfirmDialog;

impl ConfirmDialog {
    pub fn render(message: &str, colors: &Colors, width: usize) {
        println!();
        Self::render_box(colors, width, |_| {
            println!("{}  {}  {}", " ".repeat(3), message, " ".repeat(3));
        });
        println!();
        println!(
            "{}{}  y/Yes │ n/No │ Esc/Cancel  {}",
            " ".repeat(5),
            colors.prompt_fg,
            RESET
        );
    }

    fn render_box<F>(colors: &Colors, width: usize, content: F)
    where
        F: Fn(usize),
    {
        let box_width = (width.min(60)).max(30);
        let padding_left = (width.saturating_sub(box_width)) / 2;

        println!(
            "{}{}╔{}╗{}",
            " ".repeat(padding_left),
            colors.header_bg,
            "═".repeat(box_width.saturating_sub(2)),
            RESET
        );

        print!(
            "{}{}║{}│",
            " ".repeat(padding_left),
            colors.header_bg,
            RESET
        );
        content(box_width);
        println!("{}║{}", " ".repeat(padding_left - 1), RESET);

        println!(
            "{}{}╚{}╝{}",
            " ".repeat(padding_left),
            colors.header_bg,
            "═".repeat(box_width.saturating_sub(2)),
            RESET
        );
    }
}

/// Error modal dialog
pub struct ErrorModal;

impl ErrorModal {
    pub fn render(title: &str, message: &str, colors: &Colors, width: usize) {
        println!();
        println!(
            "{}{}╔{}╗{}",
            " ".repeat(3),
            colors.error_fg,
            "═".repeat(width.saturating_sub(8)),
            RESET
        );
        println!(
            "{}{}║ {} {}{}",
            " ".repeat(2),
            colors.error_fg,
            title,
            " ".repeat(width.saturating_sub(title.len() + 6)),
            RESET
        );
        println!(
            "{}{}╠{}╣{}",
            " ".repeat(3),
            colors.error_fg,
            "═".repeat(width.saturating_sub(8)),
            RESET
        );
        println!(
            "{}{}║ {} {}{}",
            " ".repeat(2),
            colors.error_fg,
            message,
            " ".repeat(width.saturating_sub(message.len() + 6)),
            RESET
        );
        println!(
            "{}{}╚{}╝{}",
            " ".repeat(3),
            colors.error_fg,
            "═".repeat(width.saturating_sub(8)),
            RESET
        );
        println!("{}Press any key to continue...{}", colors.prompt_fg, RESET);
    }
}

/// Hunk display helper
pub struct HunkDisplay;

impl HunkDisplay {
    pub fn render(
        hunk: &Hunk,
        is_selected: bool,
        colors: &Colors,
        show_line_numbers: bool,
        width: usize,
    ) {
        // Hunk header
        let status_char = match hunk.status {
            HunkStatus::Accepted => "✓",
            HunkStatus::Rejected => "✗",
            HunkStatus::Pending => "○",
        };

        let status_color = match hunk.status {
            HunkStatus::Accepted => colors.status_accepted,
            HunkStatus::Rejected => colors.status_rejected,
            HunkStatus::Pending => colors.status_pending,
        };

        let bg = if is_selected { colors.selected_bg } else { "" };

        println!(
            "{}{}@@ -{},{} +{},{} @@ {} (hunk {}){}",
            bg,
            status_color,
            hunk.start_line,
            hunk.changes
                .iter()
                .filter(|c| c.tag != ChangeTag::Insert)
                .count(),
            hunk.start_line,
            hunk.changes
                .iter()
                .filter(|c| c.tag != ChangeTag::Delete)
                .count(),
            status_char,
            hunk.id,
            RESET
        );

        // Hunk content
        for change in &hunk.changes {
            Self::render_line(change, show_line_numbers, colors, width, is_selected);
        }
    }

    fn render_line(
        change: &LineChange,
        show_line_numbers: bool,
        colors: &Colors,
        width: usize,
        is_selected: bool,
    ) {
        let (prefix, line_color, line_bg) = match change.tag {
            ChangeTag::Insert => ("+", colors.added_fg, colors.added_bg),
            ChangeTag::Delete => ("-", colors.removed_fg, colors.removed_bg),
            ChangeTag::Equal => (" ", colors.context_fg, ""),
        };

        let bg = if is_selected { colors.selected_bg } else { "" };

        let line_num = if show_line_numbers {
            match change.tag {
                ChangeTag::Insert => format!("    {:>4} ", change.new_line_num.unwrap_or(0)),
                ChangeTag::Delete => format!("{:>4}     ", change.old_line_num.unwrap_or(0)),
                ChangeTag::Equal => format!(
                    "{:>4} {:>4} ",
                    change.old_line_num.unwrap_or(0),
                    change.new_line_num.unwrap_or(0)
                ),
            }
        } else {
            String::new()
        };

        let max_content_width = width.saturating_sub(line_num.len() + 2);
        let truncated_content = if change.content.len() > max_content_width {
            format!(
                "{}…",
                &change.content[..max_content_width.saturating_sub(1)]
            )
        } else {
            change.content.clone()
        };

        println!(
            "{}{}{}{}{}{}{}{}",
            bg, line_bg, DIM, line_num, line_color, prefix, truncated_content, RESET
        );
    }
}
