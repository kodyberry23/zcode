// src/ui/session_turn.rs - Message/turn rendering for chat stream

use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::state::{ChatMessage, MessageStatus};
use crate::ui::colors::Theme;

/// Render chat messages as a vertical list styled like OpenCode's SessionTurn.
pub fn render_session_turns(
    frame: &mut Frame,
    area: Rect,
    messages: &[ChatMessage],
    theme: &Theme,
) {
    let items: Vec<ListItem> = messages
        .iter()
        .map(|msg| {
            let prefix = if msg.is_user { "› " } else { "◆ " };
            let prefix_color = if msg.is_user {
                Color::Rgb(120, 170, 255)
            } else {
                Color::Rgb(200, 200, 200)
            };

            let status_icon = match msg.status {
                MessageStatus::Success => "✓ ",
                MessageStatus::Error => "✗ ",
                MessageStatus::Working => "… ",
                MessageStatus::Pending => "○ ",
            };

            let status_color = match msg.status {
                MessageStatus::Success => theme.status_accepted.fg.unwrap_or(Color::Green),
                MessageStatus::Error => theme.error_style.fg.unwrap_or(Color::Red),
                MessageStatus::Working | MessageStatus::Pending => {
                    theme.status_pending.fg.unwrap_or(Color::Yellow)
                }
            };

            let timestamp = msg.timestamp.format("%H:%M:%S").to_string();

            let header = Line::from(vec![
                Span::styled(
                    format!("[{}] ", timestamp),
                    Style::default().fg(Color::Rgb(90, 90, 90)),
                ),
                Span::styled(status_icon, Style::default().fg(status_color)),
                Span::styled(
                    prefix,
                    Style::default()
                        .fg(prefix_color)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(msg.content.clone(), theme.normal_style),
            ]);

            ListItem::new(header).style(theme.normal_style)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(theme.border_style)
                .title(" Chat "),
        )
        .style(theme.normal_style);

    frame.render_widget(list, area);
}

/// Empty state when there are no messages yet.
pub fn render_empty_chat(frame: &mut Frame, area: Rect, theme: &Theme) {
    let block = Block::default()
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(theme.border_style)
        .title(" Chat ");

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Center content vertically
    let vertical_padding = inner.height.saturating_sub(8) / 2;
    let content_area = Rect::new(inner.x, inner.y + vertical_padding, inner.width, 8);

    let lines = vec![
        Line::from(vec![
            Span::raw("👋 "),
            Span::styled(
                "Welcome to zcode!",
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Get started:",
            Style::default().fg(Color::Gray),
        )),
        Line::from(vec![
            Span::raw("  1. "),
            Span::styled("Select a provider", Style::default().fg(Color::Yellow)),
            Span::raw(" (Esc to choose)"),
        ]),
        Line::from(vec![
            Span::raw("  2. "),
            Span::styled("Type your question", Style::default().fg(Color::Yellow)),
            Span::raw(" below"),
        ]),
        Line::from(vec![
            Span::raw("  3. "),
            Span::styled("Press Enter", Style::default().fg(Color::Yellow)),
            Span::raw(" to send"),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "Press ? for help",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let paragraph = Paragraph::new(lines)
        .alignment(ratatui::layout::Alignment::Center)
        .style(theme.normal_style);

    frame.render_widget(paragraph, content_area);
}
