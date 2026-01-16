// src/ui/header.rs - Compact header with logo, session info, and status

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::state::State;
use crate::ui::colors::Theme;
use crate::ui::logo::render_logo_compact;
use crate::ui::status_bar::format_tokens_cost;

/// Render the top header bar inspired by OpenCode's layout.
pub fn render_header(frame: &mut Frame, area: Rect, state: &State, theme: &Theme) {
    // Split into left (logo + session) and right (status indicators)
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Min(30),    // Logo + session (flexible)
            Constraint::Length(45), // Status (fixed width for metrics)
        ])
        .split(area);

    // Left: logo + session
    let left = chunks[0];
    let left_inner = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(18), Constraint::Min(10)])
        .split(left);

    render_logo_compact(frame, left_inner[0]);

    let session_title = state
        .sessions
        .current_session_id
        .as_ref()
        .and_then(|id| state.sessions.sessions.get(id))
        .map(|s| s.description.clone())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| {
            state
                .sessions
                .current_session_id
                .clone()
                .unwrap_or_else(|| "Session".into())
        });

    let provider = state
        .provider
        .as_ref()
        .map(|p| p.name().to_string())
        .unwrap_or_else(|| "No provider".into());

    let session_line = Line::from(vec![
        Span::styled(
            session_title,
            Style::default()
                .fg(theme.normal_style.fg.unwrap_or(Color::White))
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(provider, Style::default().fg(Color::Rgb(120, 170, 255))),
    ]);

    let session_block = Paragraph::new(session_line)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(theme.border_style)
                .title(" Project / Session "),
        )
        .alignment(ratatui::layout::Alignment::Left);
    frame.render_widget(session_block, left_inner[1]);

    // Right: status indicators
    let right = chunks[1];
    let status_text = format!(
        "{} | {} | ${:.4}",
        state.status_info.current_task.clone(),
        state.status_info.tokens_sent,
        state.status_info.session_cost
    );

    let status = Paragraph::new(status_text)
        .style(theme.normal_style)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(theme.border_style)
                .title(" Status "),
        )
        .alignment(ratatui::layout::Alignment::Right);

    frame.render_widget(status, right);
}
