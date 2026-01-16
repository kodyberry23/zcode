// src/ui/help.rs - Context-sensitive help overlay

use crate::state::Mode;
use crate::ui::colors::Theme;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, Paragraph, Wrap},
    Frame,
};

/// Render help overlay with modern styling
pub fn render_help(frame: &mut Frame, area: Rect, mode: &Mode, theme: &Theme) {
    let help_text = get_help_text(mode);

    let paragraph = Paragraph::new(help_text)
        .block(
            Block::default()
                .title(" Help ")
                .title_style(Style::default().fg(Color::White))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::DarkGray)),
        )
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: false });

    frame.render_widget(Clear, area); // Clear background
    frame.render_widget(paragraph, area);
}

fn get_help_text(mode: &Mode) -> Vec<Line<'static>> {
    let key_style = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD);
    let header_style = Style::default()
        .fg(Color::Yellow)
        .add_modifier(Modifier::BOLD);
    let dim_style = Style::default().fg(Color::DarkGray);

    // Global keybindings section
    let mut lines = vec![
        Line::from(Span::styled("Global Keybindings", header_style)),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Tab     ", key_style),
            Span::raw("Toggle chat history"),
        ]),
        Line::from(vec![
            Span::styled("  ?       ", key_style),
            Span::raw("Show this help"),
        ]),
        Line::from(vec![
            Span::styled("  :       ", key_style),
            Span::raw("Enter command mode"),
        ]),
        Line::from(vec![
            Span::styled("  Ctrl+C  ", key_style),
            Span::raw("Quit"),
        ]),
        Line::from(""),
    ];

    // Mode-specific keybindings
    match mode {
        Mode::ProviderSelect => {
            lines.extend(vec![
                Line::from(Span::styled("Provider Selection", header_style)),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  j/k     ", key_style),
                    Span::raw("Navigate up/down"),
                ]),
                Line::from(vec![
                    Span::styled("  g/G     ", key_style),
                    Span::raw("Jump to first/last"),
                ]),
                Line::from(vec![
                    Span::styled("  Enter   ", key_style),
                    Span::raw("Select provider"),
                ]),
                Line::from(vec![
                    Span::styled("  q/Esc   ", key_style),
                    Span::raw("Quit"),
                ]),
            ]);
        }
        Mode::PromptEntry | Mode::Help => {
            lines.extend(vec![
                Line::from(Span::styled("Prompt Entry", header_style)),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  Enter   ", key_style),
                    Span::raw("Submit prompt"),
                ]),
                Line::from(vec![
                    Span::styled("  Esc     ", key_style),
                    Span::raw("Back to provider selection"),
                ]),
                Line::from(""),
                Line::from(Span::styled("Vi Navigation", header_style)),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  h/l     ", key_style),
                    Span::raw("Move cursor left/right"),
                ]),
                Line::from(vec![
                    Span::styled("  w/b     ", key_style),
                    Span::raw("Word forward/backward"),
                ]),
                Line::from(vec![
                    Span::styled("  0/$     ", key_style),
                    Span::raw("Start/end of line"),
                ]),
                Line::from(vec![
                    Span::styled("  Ctrl+U  ", key_style),
                    Span::raw("Clear line"),
                ]),
            ]);
        }
        Mode::DiffReview => {
            lines.extend(vec![
                Line::from(Span::styled("Diff Review", header_style)),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  j/k     ", key_style),
                    Span::raw("Next/previous line"),
                ]),
                Line::from(vec![
                    Span::styled("  J/K     ", key_style),
                    Span::raw("Next/previous file"),
                ]),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  y       ", key_style),
                    Span::raw("Accept current line"),
                ]),
                Line::from(vec![
                    Span::styled("  n       ", key_style),
                    Span::raw("Reject current line"),
                ]),
                Line::from(vec![
                    Span::styled("  a       ", key_style),
                    Span::raw("Accept all"),
                ]),
                Line::from(vec![
                    Span::styled("  r       ", key_style),
                    Span::raw("Reject all"),
                ]),
                Line::from(vec![
                    Span::styled("  Enter   ", key_style),
                    Span::raw("Apply accepted changes"),
                ]),
                Line::from(vec![
                    Span::styled("  q/Esc   ", key_style),
                    Span::raw("Cancel and go back"),
                ]),
            ]);
        }
        Mode::Confirmation => {
            lines.extend(vec![
                Line::from(Span::styled("Confirmation", header_style)),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  y/Enter ", key_style),
                    Span::raw("Confirm action"),
                ]),
                Line::from(vec![
                    Span::styled("  n/Esc   ", key_style),
                    Span::raw("Cancel"),
                ]),
            ]);
        }
        Mode::ChatHistory => {
            lines.extend(vec![
                Line::from(Span::styled("Chat History", header_style)),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  j/k     ", key_style),
                    Span::raw("Scroll up/down"),
                ]),
                Line::from(vec![
                    Span::styled("  g/G     ", key_style),
                    Span::raw("Jump to top/bottom"),
                ]),
                Line::from(vec![
                    Span::styled("  /       ", key_style),
                    Span::raw("Search messages"),
                ]),
                Line::from(vec![
                    Span::styled("  n/N     ", key_style),
                    Span::raw("Next/prev search result"),
                ]),
                Line::from(vec![
                    Span::styled("  Esc     ", key_style),
                    Span::raw("Close chat history"),
                ]),
            ]);
        }
        Mode::CommandMode => {
            lines.extend(vec![
                Line::from(Span::styled("Command Mode", header_style)),
                Line::from(""),
                Line::from(vec![
                    Span::styled("  Enter   ", key_style),
                    Span::raw("Execute command"),
                ]),
                Line::from(vec![
                    Span::styled("  Esc     ", key_style),
                    Span::raw("Cancel"),
                ]),
                Line::from(""),
                Line::from(Span::styled("Commands:", dim_style)),
                Line::from(vec![
                    Span::styled("  :config ", key_style),
                    Span::raw("Show configuration"),
                ]),
                Line::from(vec![
                    Span::styled("  :help   ", key_style),
                    Span::raw("Show help"),
                ]),
                Line::from(vec![
                    Span::styled("  :quit   ", key_style),
                    Span::raw("Quit application"),
                ]),
            ]);
        }
        _ => {
            lines.extend(vec![
                Line::from(""),
                Line::from(Span::styled("Press ? to close help", dim_style)),
            ]);
        }
    }

    // Footer
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "Press ? or Esc to close",
        Style::default().fg(Color::DarkGray),
    )));

    lines
}
