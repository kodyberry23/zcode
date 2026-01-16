// src/ui/prompt_input.rs - Floating prompt input component
//
// This mirrors the OpenCode-style floating bar with a subtle backdrop, rounded
// borders, and compact metadata row (provider/model + send hint).

use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};

use crate::state::State;
use crate::ui::colors::Theme;

pub fn render_prompt_input(frame: &mut Frame, area: Rect, state: &State, theme: &Theme) {
    // Apply a subtle gradient-like backdrop by layering a translucent block.
    frame.render_widget(Clear, area);

    // Use max-width centering for better use of space on wide terminals
    let container = crate::ui::layout::max_width_centered(area, 100);

    // Split container into input and footer row
    let vertical = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(2)])
        .split(container);

    // Build display text with cursor marker
    let mut display_text = state.prompt_buffer.clone();
    if state.cursor_position <= display_text.len() {
        display_text.insert(state.cursor_position, '│');
    } else {
        display_text.push('│');
    }

    let show_placeholder = state.prompt_buffer.is_empty();
    let placeholder = "Ask anything… (Shift+Enter for newline)";

    let paragraph = Paragraph::new(if show_placeholder {
        placeholder.into()
    } else {
        display_text
    })
    .wrap(Wrap { trim: false })
    .style(if show_placeholder {
        Style::default().fg(Color::Rgb(120, 120, 120))
    } else {
        theme.prompt_style
    })
    .alignment(Alignment::Left)
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(theme.border_style)
            .title(" Prompt ")
            .title_alignment(Alignment::Left),
    );

    frame.render_widget(paragraph, vertical[0]);

    // Footer row: provider/model info + send hint
    let provider = state
        .provider
        .as_ref()
        .map(|p| p.name().to_string())
        .unwrap_or_else(|| "No provider".into());
    let model = if state.status_info.model.is_empty() {
        "model?".to_string()
    } else {
        state.status_info.model.clone()
    };

    let footer_line = Line::from(vec![
        Span::styled("Agent ", Style::default().fg(Color::Rgb(160, 160, 160))),
        Span::styled(provider, Style::default().fg(Color::Rgb(120, 170, 255))),
        Span::raw(" • "),
        Span::styled(model, Style::default().fg(Color::Rgb(160, 200, 255))),
        Span::raw("   "),
        Span::styled(
            "Ctrl+Enter send",
            Style::default().fg(Color::Rgb(140, 140, 140)),
        ),
    ]);

    let footer = Paragraph::new(footer_line)
        .style(theme.normal_style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(theme.border_style)
                .title(" Compose ")
                .title_style(Style::default().fg(Color::Rgb(160, 160, 160))),
        )
        .alignment(Alignment::Left);

    frame.render_widget(footer, vertical[1]);
}
