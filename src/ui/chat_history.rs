// src/ui/chat_history.rs - Chat history UI component

use crate::state::{ChatHistory, ChatMessage, MessageStatus};
use crate::ui::colors::Theme;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

/// Render chat history panel
pub fn render_chat_history(
    frame: &mut Frame,
    area: Rect,
    chat_history: &ChatHistory,
    theme: &Theme,
) {
    let messages = chat_history.filtered_messages();

    let items: Vec<ListItem> = messages
        .iter()
        .map(|msg| {
            let prefix = if msg.is_user { "> " } else { "◆ " };

            let default_color = if msg.is_user {
                theme.prompt_style.fg.unwrap_or(Color::Cyan)
            } else {
                theme.normal_style.fg.unwrap_or(Color::Gray)
            };

            let status_icon = match msg.status {
                MessageStatus::Success => "✓ ",
                MessageStatus::Error => "✗ ",
                MessageStatus::Working => "⧳ ",
                MessageStatus::Pending => "○ ",
            };

            let status_color = match msg.status {
                MessageStatus::Success => theme.status_accepted.fg,
                MessageStatus::Error => theme.error_style.fg,
                MessageStatus::Working | MessageStatus::Pending => theme.status_pending.fg,
            };

            let timestamp = msg.timestamp.format("%H:%M:%S").to_string();
            let token_info = if let Some(tokens) = msg.token_count {
                format!(" ({} tokens)", tokens)
            } else {
                String::new()
            };

            // Format message with timestamp and status
            let header = format!("[{}] {}{}", timestamp, status_icon, prefix);
            let content = format!("{}{}", msg.content, token_info);

            // Truncate long messages for display (can be expanded later)
            let display_content = if content.len() > (area.width as usize).saturating_sub(10) {
                format!(
                    "{}...",
                    &content[..(area.width as usize).saturating_sub(13)]
                )
            } else {
                content
            };

            let line = Line::from(vec![
                Span::styled(
                    header,
                    Style::default().fg(status_color.unwrap_or(default_color)),
                ),
                Span::styled(display_content, Style::default().fg(default_color)),
            ]);

            ListItem::new(line)
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.border_style)
                .title(" Chat History "),
        )
        .style(theme.normal_style);

    // Note: We need mutable access to scroll_state, but this function takes &ChatHistory
    // The caller should pass &mut ChatHistory or we need to restructure
    // For now, we'll render without state (non-interactive)
    // TODO: Fix this to properly use stateful rendering
    frame.render_widget(list, area);
}
