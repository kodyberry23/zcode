// src/ui/status_bar.rs - Real-time status bar rendering

use crate::state::StatusInfo;
use crate::ui::colors::Theme;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Gauge, Paragraph},
    Frame,
};

/// Render status bar with real-time information - minimal OpenCode style
pub fn render_status_bar(frame: &mut Frame, area: Rect, status: &StatusInfo, theme: &Theme) {
    let status_text = if status.is_working {
        let spinner_chars = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let frame_idx = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as usize)
            / 80
            % spinner_chars.len();

        let progress = if let Some(percent) = status.progress_percent {
            format!(" {}%", percent)
        } else {
            String::new()
        };

        format!(
            "{} Working{} | Tokens: {} | Cost ${:.4}",
            spinner_chars[frame_idx], progress, status.tokens_sent, status.session_cost
        )
    } else {
        // Ready state - minimal format matching the images
        let provider_display = if status.provider.is_empty() {
            "()".to_string()
        } else {
            status.provider.clone()
        };

        format!(
            "Ready | provider {} | Tokens: {} | Cost ${:.4}",
            provider_display, status.tokens_sent, status.session_cost
        )
    };

    let style = Style::default().fg(Color::DarkGray);

    let paragraph = Paragraph::new(status_text)
        .style(style)
        .alignment(Alignment::Left)
        .block(Block::default().borders(Borders::NONE));

    frame.render_widget(paragraph, area);
}

/// Render cost estimate before sending message
pub fn render_cost_estimate(
    frame: &mut Frame,
    area: Rect,
    estimated_tokens: usize,
    estimated_cost: f64,
    theme: &Theme,
) {
    let text = format!(
        "Est. cost: ~${:.4} ({} tokens) │ Press Enter to send",
        estimated_cost, estimated_tokens
    );

    let paragraph = Paragraph::new(text)
        .block(
            Block::default()
                .borders(Borders::TOP)
                .border_style(theme.border_style),
        )
        .style(theme.prompt_style);

    frame.render_widget(paragraph, area);
}

/// Helper to format tokens and cost succinctly for header/status usage.
pub fn format_tokens_cost(tokens: usize, cost: f64) -> String {
    if tokens == 0 {
        "0".into()
    } else {
        format!("{} • ${:.4}", tokens, cost)
    }
}
