// src/ui/renderer.rs - Renderer trait and rendering infrastructure

use super::colors::Colors;
use crate::state::{Mode, State};

/// Context for rendering operations
#[derive(Debug, Clone)]
pub struct RenderContext {
    pub rows: usize,
    pub cols: usize,
    pub colors: Colors,
    pub show_line_numbers: bool,
}

impl RenderContext {
    pub fn new(rows: usize, cols: usize, colors: Colors) -> Self {
        Self {
            rows,
            cols,
            colors,
            show_line_numbers: true,
        }
    }

    /// Remaining rows after accounting for header and footer
    pub fn content_rows(&self) -> usize {
        self.rows.saturating_sub(3) // 1 header + 2 footer
    }

    /// Remaining rows for scrollable content
    pub fn scrollable_rows(&self) -> usize {
        self.rows.saturating_sub(4) // 1 header + 1 status + 2 footer
    }
}

/// Main renderer trait for all UI modes
pub trait Renderer {
    fn render(&self, state: &State, ctx: &RenderContext);
}

/// Provider selection UI renderer
pub struct ProviderSelectRenderer;

impl Renderer for ProviderSelectRenderer {
    fn render(&self, state: &State, ctx: &RenderContext) {
        use crate::ui::components::{Footer, ProviderMenu};

        // Clear screen and move cursor to top
        print!("\x1b[2J\x1b[H");

        // Header
        let title = "  ZCode │ Select AI Provider";
        let padding = ctx.cols.saturating_sub(title.len());
        println!(
            "{}{}{}{}{}",
            ctx.colors.header_bg,
            ctx.colors.header_fg,
            title,
            " ".repeat(padding),
            crate::ui::RESET
        );

        println!(); // Spacer

        // Provider menu
        ProviderMenu::render(
            &state.available_providers,
            state.selected_provider_idx,
            &ctx.colors,
            ctx.rows,
            ctx.cols,
        );

        // Move to bottom for footer
        let footer_row = ctx.rows.saturating_sub(1);
        print!("\x1b[{};1H", footer_row);

        // Footer with keybindings
        Footer::render(
            &ctx.colors,
            ctx.cols,
            "j/k: navigate │ Enter: select │ q: quit",
        );
    }
}

/// Prompt entry UI renderer
pub struct PromptEntryRenderer;

impl Renderer for PromptEntryRenderer {
    fn render(&self, state: &State, _ctx: &RenderContext) {
        print!("PromptEntry mode not yet implemented");
    }
}

/// Diff review UI renderer
pub struct DiffReviewRenderer;

impl Renderer for DiffReviewRenderer {
    fn render(&self, state: &State, _ctx: &RenderContext) {
        print!("DiffReview mode not yet implemented");
    }
}

/// Confirmation dialog renderer
pub struct ConfirmationRenderer;

impl Renderer for ConfirmationRenderer {
    fn render(&self, state: &State, _ctx: &RenderContext) {
        print!("Confirmation mode not yet implemented");
    }
}

/// Error display renderer
pub struct ErrorRenderer;

impl Renderer for ErrorRenderer {
    fn render(&self, state: &State, ctx: &RenderContext) {
        if let Some(error) = &state.last_error {
            let width = ctx.cols;
            println!(
                "\n{}{}╔{}╗{}",
                ctx.colors.error_fg,
                " ".repeat(3),
                "═".repeat(width.saturating_sub(8)),
                crate::ui::RESET
            );
            println!(
                "{}{}║ Error: {}{}",
                ctx.colors.error_fg,
                " ".repeat(2),
                error.title,
                crate::ui::RESET
            );
            println!(
                "{}{}╠{}╣{}",
                ctx.colors.error_fg,
                " ".repeat(3),
                "═".repeat(width.saturating_sub(8)),
                crate::ui::RESET
            );
            println!(
                "{}{}║ {}{}",
                ctx.colors.error_fg,
                " ".repeat(2),
                error.message,
                crate::ui::RESET
            );
            println!(
                "{}{}╚{}╝{}",
                ctx.colors.error_fg,
                " ".repeat(3),
                "═".repeat(width.saturating_sub(8)),
                crate::ui::RESET
            );
            println!(
                "{}Press any key to continue...{}",
                ctx.colors.prompt_fg,
                crate::ui::RESET
            );
        }
    }
}

/// Get the appropriate renderer for a mode
pub fn get_renderer_for_mode(mode: &Mode) -> Box<dyn Renderer> {
    match mode {
        Mode::ProviderSelect => Box::new(ProviderSelectRenderer),
        Mode::PromptEntry => Box::new(PromptEntryRenderer),
        Mode::DiffReview => Box::new(DiffReviewRenderer),
        Mode::Confirmation => Box::new(ConfirmationRenderer),
        Mode::Error => Box::new(ErrorRenderer),
        Mode::Processing => Box::new(ProcessingRenderer),
    }
}

/// Processing/loading state renderer
pub struct ProcessingRenderer;

impl Renderer for ProcessingRenderer {
    fn render(&self, _state: &State, ctx: &RenderContext) {
        let spinner_chars = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let frame = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as usize)
            / 80
            % spinner_chars.len();

        println!(
            "{}{}{}Processing... Please wait{}",
            " ".repeat((ctx.cols / 2).saturating_sub(10)),
            ctx.colors.prompt_fg,
            spinner_chars[frame],
            crate::ui::RESET
        );
    }
}
