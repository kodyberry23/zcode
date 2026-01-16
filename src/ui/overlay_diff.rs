// src/ui/overlay_diff.rs - Overlay-based diff rendering (VSCode/Neovim style)

use crate::state::{DecorationType, LineDecoration, OverlayDiffState, ProposedChange};
use crate::ui::colors::Theme;
use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
    Frame,
};
use std::path::PathBuf;

/// Render overlay-style diff preview
pub fn render_overlay_diff(
    frame: &mut Frame,
    area: Rect,
    diff_state: &OverlayDiffState,
    theme: &Theme,
) {
    if diff_state.proposed_changes.is_empty() {
        let text = Paragraph::new("No changes to review")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(theme.border_style)
                    .title(" Diff Review "),
            )
            .style(theme.normal_style)
            .alignment(ratatui::layout::Alignment::Center);
        frame.render_widget(text, area);
        return;
    }

    let current_change = &diff_state.proposed_changes[diff_state.current_change_idx];
    let file_name = current_change
        .file_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown");

    // Build lines for display
    let mut lines = Vec::new();

    // File header
    lines.push(Line::from(vec![Span::styled(
        format!("┌─ {} ─", file_name),
        theme.header_style,
    )]));

    // Render each line decoration
    for (idx, dec) in current_change.line_decorations.iter().enumerate() {
        let is_selected = idx == diff_state.current_line_idx;
        let line_num = dec.line_number;

        match dec.decoration_type {
            DecorationType::Deletion => {
                // Show original text with strikethrough
                let original = dec.original_text.as_deref().unwrap_or("");
                let marker = match dec.accepted {
                    Some(true) => "✓",
                    Some(false) => "✗",
                    None => "○",
                };
                let marker_style = match dec.accepted {
                    Some(true) => theme.status_accepted,
                    Some(false) => theme.status_rejected,
                    None => theme.status_pending,
                };

                let line = Line::from(vec![
                    Span::styled(
                        format!("{:4} ", line_num),
                        Style::default().fg(Color::Indexed(242)),
                    ),
                    Span::styled(format!("{} ", marker), marker_style),
                    Span::styled(
                        format!("-{}", original),
                        theme.removed_style.add_modifier(Modifier::CROSSED_OUT),
                    ),
                ]);

                lines.push(line);
            }
            DecorationType::Addition => {
                // Show new text with green background
                let new_text = dec.new_text.as_deref().unwrap_or("");
                let marker = match dec.accepted {
                    Some(true) => "✓",
                    Some(false) => "✗",
                    None => "○",
                };
                let marker_style = match dec.accepted {
                    Some(true) => theme.status_accepted,
                    Some(false) => theme.status_rejected,
                    None => theme.status_pending,
                };

                let line = Line::from(vec![
                    Span::styled(format!("    "), Style::default()),
                    Span::styled(format!("{} ", marker), marker_style),
                    Span::styled(format!("+{}", new_text), theme.added_style),
                ]);

                lines.push(line);
            }
            DecorationType::Modification => {
                // Show both old (strikethrough) and new (green) on consecutive lines
                let original = dec.original_text.as_deref().unwrap_or("");
                let new_text = dec.new_text.as_deref().unwrap_or("");
                let marker = match dec.accepted {
                    Some(true) => "✓",
                    Some(false) => "✗",
                    None => "○",
                };
                let marker_style = match dec.accepted {
                    Some(true) => theme.status_accepted,
                    Some(false) => theme.status_rejected,
                    None => theme.status_pending,
                };

                // Old line (strikethrough)
                lines.push(Line::from(vec![
                    Span::styled(
                        format!("{:4} ", line_num),
                        Style::default().fg(Color::Indexed(242)),
                    ),
                    Span::styled(format!("{} ", marker), marker_style),
                    Span::styled(
                        format!("-{}", original),
                        theme.removed_style.add_modifier(Modifier::CROSSED_OUT),
                    ),
                ]));

                // New line (green)
                lines.push(Line::from(vec![
                    Span::styled(format!("    "), Style::default()),
                    Span::styled(format!("  "), Style::default()),
                    Span::styled(format!("+{}", new_text), theme.added_style),
                ]));
            }
            DecorationType::Context => {
                // Unchanged line - only show if not folded
                if !diff_state.folded_unchanged {
                    let content = dec.original_text.as_deref().unwrap_or("");
                    let line = Line::from(vec![
                        Span::styled(
                            format!("{:4} ", line_num),
                            Style::default().fg(Color::Indexed(242)),
                        ),
                        Span::styled("  ", Style::default()),
                        Span::styled(format!(" {}", content), theme.context_style),
                    ]);
                    lines.push(line);
                }
            }
        }
    }

    // Footer with keybindings
    lines.push(Line::from(""));
    lines.push(Line::from(vec![Span::styled(
        "[y] Accept line │ [n] Reject line │ [a] Accept all │ [r] Reject all │ ",
        theme.prompt_style,
    )]));
    lines.push(Line::from(vec![Span::styled(
        "[j/k] Navigate │ [J/K] Next/Prev file │ [Enter] Apply accepted",
        theme.prompt_style,
    )]));

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.border_style)
                .title(format!(" Diff Review - {} ", file_name)),
        )
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

/// Convert hunks to overlay decorations
pub fn convert_hunks_to_overlay(
    file_path: PathBuf,
    original_content: String,
    proposed_content: String,
    hunks: &[crate::state::Hunk],
) -> ProposedChange {
    use crate::diff::{extract_hunks, generate_diff};
    use crate::state::ChangeTag;

    let diff = generate_diff(&original_content, &proposed_content);
    let mut line_decorations = Vec::new();

    // Process each hunk
    for hunk in hunks {
        for change in &hunk.changes {
            let decoration_type = match change.tag {
                ChangeTag::Insert => DecorationType::Addition,
                ChangeTag::Delete => DecorationType::Deletion,
                ChangeTag::Equal => DecorationType::Context,
            };

            let line_num = change.new_line_num.or(change.old_line_num).unwrap_or(0);

            let decoration = LineDecoration {
                line_number: line_num,
                decoration_type,
                original_text: if matches!(change.tag, ChangeTag::Delete | ChangeTag::Equal) {
                    Some(change.content.clone())
                } else {
                    None
                },
                new_text: if matches!(change.tag, ChangeTag::Insert | ChangeTag::Equal) {
                    Some(change.content.clone())
                } else {
                    None
                },
                accepted: None, // Start as pending
            };

            line_decorations.push(decoration);
        }
    }

    ProposedChange {
        id: 0,
        file_path,
        original_content,
        proposed_content,
        line_decorations,
        status: crate::state::ChangeStatus::Pending,
    }
}
