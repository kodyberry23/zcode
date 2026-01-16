// src/ui/sidebar.rs - File preview sidebar

use crate::state::SidebarState;
use crate::ui::colors::Theme;
use ratatui::{
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use std::fs;
use std::path::PathBuf;

/// Render file preview sidebar
pub fn render_sidebar(frame: &mut Frame, area: Rect, sidebar: &SidebarState, theme: &Theme) {
    if !sidebar.visible {
        return;
    }

    let file_path = match &sidebar.pinned_file {
        Some(path) => path,
        None => {
            // Show empty sidebar
            let paragraph = Paragraph::new("No file pinned")
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(theme.border_style)
                        .title(" File Preview "),
                )
                .style(theme.normal_style)
                .alignment(ratatui::layout::Alignment::Center);
            frame.render_widget(paragraph, area);
            return;
        }
    };

    // Read file content
    let content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(_) => {
            let paragraph = Paragraph::new("Failed to read file")
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(theme.border_style)
                        .title(" File Preview "),
                )
                .style(theme.error_style)
                .alignment(ratatui::layout::Alignment::Center);
            frame.render_widget(paragraph, area);
            return;
        }
    };

    // Build lines with syntax highlighting (basic for now, can enhance with syntect later)
    let lines: Vec<Line> = content
        .lines()
        .enumerate()
        .map(|(line_num, line)| {
            let line_num_str = format!("{:4} ", line_num + 1);
            let is_highlighted = sidebar.highlighted_lines.contains(&(line_num + 1));

            let style = if is_highlighted {
                theme.selected_style
            } else {
                theme.normal_style
            };

            Line::from(vec![
                Span::styled(line_num_str, theme.context_style),
                Span::styled(line.to_string(), style),
            ])
        })
        .collect();

    let file_name = file_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.border_style)
                .title(format!(" {} ", file_name)),
        )
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

/// Pin a file to the sidebar
pub fn pin_file(sidebar: &mut SidebarState, file_path: PathBuf) {
    sidebar.pinned_file = Some(file_path);
    sidebar.visible = true;
}

/// Unpin the current file
pub fn unpin_file(sidebar: &mut SidebarState) {
    sidebar.pinned_file = None;
    sidebar.visible = false;
}

/// Toggle sidebar visibility
pub fn toggle_sidebar(sidebar: &mut SidebarState) {
    sidebar.visible = !sidebar.visible;
}
