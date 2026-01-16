// src/ui/renderers.rs - Ratatui-based rendering functions

use crate::state::{DetectionState, State};
use crate::ui::colors::Theme;
use crate::ui::layout::{centered_dialog, main_layout};
use crate::ui::logo::{centered_rect, render_logo_text};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

/// Render the splash screen with logo (full screen, centered)
pub fn render_splash(frame: &mut Frame, theme: &Theme) {
    let area = frame.area();

    // Clear the screen with dark background
    frame.render_widget(Clear, area);

    // Center the logo
    let logo_height = 10u16;
    let logo_width = 70u16;
    let logo_area = centered_rect(area, logo_width, logo_height);

    render_logo_text(frame, logo_area);
}

/// Render provider selection screen - OpenCode style with centered logo and dialog
pub fn render_provider_select(frame: &mut Frame, state: &State, theme: &Theme) {
    let area = frame.area();

    // Clear background
    frame.render_widget(Clear, area);

    // Layout: Logo at top (centered), provider dialog in center, status at bottom
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(12), // Logo area (larger for full logo)
            Constraint::Min(8),     // Provider selection area
            Constraint::Length(1),  // Status bar
        ])
        .split(area);

    let logo_area = chunks[0];
    let content_area = chunks[1];
    let status_area = chunks[2];

    // Render centered ASCII logo
    render_logo_text(frame, logo_area);

    // Provider selection dialog - centered with rounded border style
    let dialog_width = 50u16;
    let dialog_height = if state.detection_state == DetectionState::InProgress {
        5
    } else if state.available_providers.is_empty() {
        12
    } else {
        (state.available_providers.len() as u16 + 5).min(12)
    };

    let dialog_rect = centered_dialog(content_area, dialog_width, dialog_height);

    if state.detection_state == DetectionState::InProgress {
        // Loading state with spinner
        let spinner_chars = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let frame_idx = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as usize)
            / 80
            % spinner_chars.len();

        let loading_text = format!("{} Detecting AI providers...", spinner_chars[frame_idx]);
        let loading = Paragraph::new(loading_text)
            .style(Style::default().fg(Color::Gray))
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .border_style(Style::default().fg(Color::DarkGray))
                    .title(" Select Provider ")
                    .title_style(Style::default().fg(Color::White)),
            );
        frame.render_widget(loading, dialog_rect);
    } else if state.available_providers.is_empty() {
        let no_providers = vec![
            Line::from(""),
            Line::from(Span::styled(
                "No AI providers detected.",
                Style::default().fg(Color::Gray),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "Please install one of:",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(Span::styled(
                "  • Claude Code (claude)",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(Span::styled(
                "  • Aider (aider)",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(Span::styled(
                "  • GitHub Copilot CLI (copilot)",
                Style::default().fg(Color::DarkGray),
            )),
            Line::from(Span::styled(
                "  • Kiro CLI (kiro)",
                Style::default().fg(Color::DarkGray),
            )),
        ];
        let text = Paragraph::new(no_providers)
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .border_style(Style::default().fg(Color::DarkGray))
                    .title(" Select Provider ")
                    .title_style(Style::default().fg(Color::White)),
            );
        frame.render_widget(text, dialog_rect);
    } else {
        // Provider list with clean styling
        let items: Vec<ListItem> = state
            .available_providers
            .iter()
            .enumerate()
            .map(|(idx, provider)| {
                let is_selected = idx == state.selected_provider_idx;
                let marker = if is_selected { "▷ " } else { "  " };

                let style = if is_selected {
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(Color::Gray)
                };

                // Show provider name and command in parentheses
                let cmd_suffix = format!(" ({})", provider.cli_command);
                let line = Line::from(vec![
                    Span::styled(marker, style),
                    Span::styled(&provider.name, style),
                    Span::styled(cmd_suffix, Style::default().fg(Color::DarkGray)),
                ]);
                ListItem::new(line)
            })
            .collect();

        let list = List::new(items).block(
            Block::default()
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray))
                .title(" Select Provider ")
                .title_style(Style::default().fg(Color::White)),
        );

        frame.render_widget(list, dialog_rect);
    }

    // Status bar - minimal style at bottom
    let provider_name = state
        .provider
        .as_ref()
        .map(|p| p.name().to_string())
        .unwrap_or_default();

    let status_text = format!(
        "Ready | provider {} | Tokens: {} | Cost ${:.4}",
        if provider_name.is_empty() {
            "()"
        } else {
            &provider_name
        },
        state.status_info.tokens_sent,
        state.status_info.session_cost
    );
    let status = Paragraph::new(status_text)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Left);
    frame.render_widget(status, status_area);
}

/// Render prompt entry screen
pub fn render_prompt_entry(frame: &mut Frame, state: &State, theme: &Theme) {
    let area = frame.area();
    let (header, content, footer) = main_layout(area);

    // Header
    let provider_name = state
        .provider
        .as_ref()
        .map(|p| p.name())
        .unwrap_or("No Provider");
    let header_text = Paragraph::new(format!("ZCode - {} - Enter Prompt", provider_name))
        .style(theme.header_style)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.border_style),
        );
    frame.render_widget(header_text, header);

    // Content - Prompt input
    let input_area = centered_dialog(content, content.width.saturating_sub(4), 8);

    // Render prompt text with cursor
    let mut display_text = state.prompt_buffer.clone();
    if state.cursor_position < display_text.len() {
        display_text.insert(state.cursor_position, '│');
    } else {
        display_text.push('│');
    }

    let input_paragraph = Paragraph::new(display_text)
        .style(theme.normal_style)
        .wrap(Wrap { trim: false })
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.prompt_style)
                .title(" Enter your prompt "),
        );

    frame.render_widget(input_paragraph, input_area);

    // Footer
    let footer_text = Paragraph::new("Enter: submit │ Esc: back │ Ctrl+C: quit")
        .style(theme.header_style)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.border_style),
        );
    frame.render_widget(footer_text, footer);
}

/// Render processing/loading screen
pub fn render_processing(frame: &mut Frame, theme: &Theme) {
    let area = frame.area();
    let loading_area = centered_dialog(area, 40, 5);

    let spinner_chars = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let frame_idx = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as usize)
        / 80
        % spinner_chars.len();

    let text = format!("{} Processing... Please wait", spinner_chars[frame_idx]);
    let paragraph = Paragraph::new(text)
        .style(theme.prompt_style)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.border_style),
        );

    frame.render_widget(paragraph, loading_area);
}

/// Render error screen - clean OpenCode style
pub fn render_error(frame: &mut Frame, state: &State, theme: &Theme) {
    let area = frame.area();
    let error_area = centered_dialog(area, 60, 10);

    // Clear background
    frame.render_widget(Clear, error_area);

    if let Some(error) = &state.last_error {
        let error_lines = vec![
            Line::from(""),
            Line::from(Span::styled(
                &error.title,
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            )),
            Line::from(""),
            Line::from(Span::styled(
                error.message.clone(),
                Style::default().fg(Color::White),
            )),
            Line::from(""),
            Line::from(Span::styled(
                "Press any key to continue...",
                Style::default().fg(Color::DarkGray),
            )),
        ];

        let paragraph = Paragraph::new(error_lines)
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Red))
                    .title(" Error ")
                    .title_style(Style::default().fg(Color::Red)),
            );

        frame.render_widget(paragraph, error_area);
    }
}

/// Render diff review screen (stub for now)
pub fn render_diff_review(frame: &mut Frame, state: &State, theme: &Theme) {
    let area = frame.area();
    let (header, content, footer) = main_layout(area);

    // Header
    let header_text = Paragraph::new("ZCode - Diff Review")
        .style(theme.header_style)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.border_style),
        );
    frame.render_widget(header_text, header);

    // Content
    let text = if state.hunks.is_empty() {
        "No changes detected.\n\nRun an AI provider to generate code changes."
    } else {
        "Diff review not yet implemented"
    };

    let paragraph = Paragraph::new(text)
        .style(theme.normal_style)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.border_style),
        );
    frame.render_widget(paragraph, content);

    // Footer
    let footer_text = Paragraph::new("j/k: navigate │ a/r: accept/reject │ Enter: apply │ q: quit")
        .style(theme.header_style)
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.border_style),
        );
    frame.render_widget(footer_text, footer);
}

/// Render confirmation dialog - clean OpenCode style
pub fn render_confirmation(frame: &mut Frame, state: &State, theme: &Theme) {
    let area = frame.area();
    let dialog_area = centered_dialog(area, 50, 8);

    // Clear background
    frame.render_widget(Clear, dialog_area);

    let text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "Apply accepted changes?",
            Style::default().fg(Color::White),
        )),
        Line::from(""),
        Line::from(Span::styled(
            "This will modify the files on disk.",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        Line::from(vec![
            Span::styled(
                "y",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("/Yes  ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "n",
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
            ),
            Span::styled("/No  ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                "Esc",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled("/Cancel", Style::default().fg(Color::DarkGray)),
        ]),
    ];

    let paragraph = Paragraph::new(text).alignment(Alignment::Center).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(Style::default().fg(Color::DarkGray))
            .title(" Confirmation ")
            .title_style(Style::default().fg(Color::White)),
    );

    frame.render_widget(paragraph, dialog_area);
}
