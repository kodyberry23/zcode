// src/ui/diff_view.rs - Diff view rendering with scrolling and navigation

use super::components::{Footer, Header, HunkDisplay, StatusLine};
use super::renderer::{RenderContext, Renderer};
use super::{DIM, RESET};
use crate::state::State;

/// Diff view renderer with scrollable hunks and file navigation
pub struct DiffViewRenderer;

impl Renderer for DiffViewRenderer {
    fn render(&self, state: &State, ctx: &RenderContext) {
        // Clear screen (Zellij handles this, but we'll be explicit)
        print!("\x1b[2J\x1b[H");

        // Header with provider and file info
        Header::render(state, &ctx.colors, ctx.cols);

        // Status line showing current position
        let total_hunks = state.hunks.len();
        let accepted = state
            .hunks
            .iter()
            .filter(|h| h.status == crate::state::HunkStatus::Accepted)
            .count();
        let rejected = state
            .hunks
            .iter()
            .filter(|h| h.status == crate::state::HunkStatus::Rejected)
            .count();

        let status_msg = format!(
            "Hunk {}/{} │ {} accepted │ {} rejected",
            state.selected_hunk + 1,
            total_hunks,
            accepted,
            rejected
        );
        StatusLine::render_info(&ctx.colors, ctx.cols, &status_msg);

        // Diff content
        self.render_diff_content(state, ctx);

        // Footer with keybindings
        let keybinds = "j/k: navigate │ a/r: accept/reject │ A/R: all │ Space: review │ Enter: apply │ q: quit";
        Footer::render(&ctx.colors, ctx.cols, keybinds);
    }
}

impl DiffViewRenderer {
    fn render_diff_content(&self, state: &State, ctx: &RenderContext) {
        if state.hunks.is_empty() {
            println!("{}No changes detected{}\\n", ctx.colors.prompt_fg, RESET);
            println!(
                "{}Run an AI provider to generate code changes.{}",
                ctx.colors.context_fg, RESET
            );
            return;
        }

        let content_rows = ctx.scrollable_rows();
        let visible_hunks = state
            .hunks
            .iter()
            .skip(state.scroll_offset)
            .take(content_rows);

        for (idx, hunk) in visible_hunks.enumerate() {
            let actual_idx = idx + state.scroll_offset;
            let is_selected = actual_idx == state.selected_hunk;

            HunkDisplay::render(
                hunk,
                is_selected,
                &ctx.colors,
                ctx.show_line_numbers,
                ctx.cols,
            );

            println!(); // Blank line between hunks
        }

        // Show if more hunks exist below
        if state.scroll_offset + content_rows < state.hunks.len() {
            println!(
                "{}{}...more hunks below (press 'j' to scroll){}",
                DIM, ctx.colors.context_fg, RESET
            );
        }
    }
}

/// File tabs component for multi-file diffs
pub struct FileTabs;

impl FileTabs {
    pub fn render(state: &State, colors: &crate::ui::Colors, width: usize) {
        if state.pending_changes.len() <= 1 {
            return;
        }

        print!("{}Files: ", colors.context_fg);
        for (idx, (path, _)) in state.pending_changes.iter().enumerate() {
            let filename = path
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "unknown".to_string());

            // Simple indicator for which file we're viewing
            let prefix = if idx == 0 { "▶ " } else { "  " };

            print!("{}{} │ ", prefix, filename);
        }
        println!("{}", RESET);
    }
}

/// Inline review mode for focused hunk editing
pub struct InlineReviewRenderer;

impl Renderer for InlineReviewRenderer {
    fn render(&self, state: &State, ctx: &RenderContext) {
        if state.hunks.is_empty() {
            return;
        }

        Header::render(state, &ctx.colors, ctx.cols);

        if let Some(hunk) = state.hunks.get(state.selected_hunk) {
            println!(
                "{}Reviewing Hunk {}:{}",
                ctx.colors.header_fg, hunk.id, RESET
            );
            println!(
                "{}File: {}{}",
                ctx.colors.context_fg,
                hunk.file_path.display(),
                RESET
            );
            println!();

            // Full hunk display
            HunkDisplay::render(hunk, true, &ctx.colors, ctx.show_line_numbers, ctx.cols);

            println!();
            println!(
                "{}Options: [a]ccept │ [r]eject │ [q]uit review│ [↑↓] scroll{}",
                ctx.colors.prompt_fg, RESET
            );
        }

        Footer::render(&ctx.colors, ctx.cols, "");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_context_calculations() {
        let colors = crate::ui::Colors::dark();
        let ctx = RenderContext::new(24, 80, colors);

        assert_eq!(ctx.content_rows(), 21);
        assert_eq!(ctx.scrollable_rows(), 20);
    }
}
