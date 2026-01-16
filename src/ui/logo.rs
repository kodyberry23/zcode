// src/ui/logo.rs - ASCII logo rendering

use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

/// The ZCode ASCII logo - main logo for splash and provider select screens
pub const LOGO: &str = r#" ________  ________  ________  ________  _______      
|\_____  \|\   ____\|\   __  \|\   ___ \|\  ___ \     
 \|___/  /\ \  \___|\ \  \|\  \ \  \_|\ \ \   __/|    
     /  / /\ \  \    \ \  \\\  \ \  \ \\ \ \  \_|/__  
    /  /_/__\ \  \____\ \  \\\  \ \  \_\\ \ \  \_|\ \ 
   |\________\ \_______\ \_______\ \_______\ \_______\
    \|_______|\|_______|\|_______|\|_______|\|_______|"#;

/// Ultra-minimal single-line logo
pub const LOGO_MINIMAL: &str = "ZCODE";

/// Compact box-drawing logo for header
pub const LOGO_HEADER: &str = r#" ________  ________  ________  ________  _______      
|\_____  \|\   ____\|\   __  \|\   ___ \|\  ___ \     
 \|___/  /\ \  \___|\ \  \|\  \ \  \_|\ \ \   __/|    
     /  / /\ \  \    \ \  \\\  \ \  \ \\ \ \  \_|/__  
    /  /_/__\ \  \____\ \  \\\  \ \  \_\\ \ \  \_|\ \ 
   |\________\ \_______\ \_______\ \_______\ \_______\
    \|_______|\|_______|\|_______|\|_______|\|_______|"#;

/// Render the ZCode logo in a centered block (no border, clean look)
pub fn render_logo(frame: &mut Frame, area: Rect) {
    let logo_lines: Vec<Line> = LOGO
        .lines()
        .map(|line| {
            Line::from(Span::styled(
                line,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::DIM),
            ))
        })
        .collect();

    let logo_paragraph = Paragraph::new(logo_lines).alignment(Alignment::Center);

    frame.render_widget(logo_paragraph, area);
}

/// Render the main ZCODE logo for splash/provider select screens
pub fn render_logo_text(frame: &mut Frame, area: Rect) {
    let lines: Vec<Line> = LOGO
        .lines()
        .map(|line| {
            Line::from(Span::styled(
                line,
                Style::default().fg(Color::Gray).add_modifier(Modifier::DIM),
            ))
        })
        .collect();

    let logo_paragraph = Paragraph::new(lines).alignment(Alignment::Center);
    frame.render_widget(logo_paragraph, area);
}

/// Render the logo with a subtitle (e.g., version info)
pub fn render_logo_with_subtitle(frame: &mut Frame, area: Rect, subtitle: &str) {
    let mut lines: Vec<Line> = LOGO
        .lines()
        .map(|line| {
            Line::from(Span::styled(
                line,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::DIM),
            ))
        })
        .collect();

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        subtitle,
        Style::default().fg(Color::DarkGray),
    )));

    let logo_paragraph = Paragraph::new(lines).alignment(Alignment::Center);
    frame.render_widget(logo_paragraph, area);
}

/// Calculate centered rect for logo display
pub fn centered_rect(area: Rect, width: u16, height: u16) -> Rect {
    let x = area.x + (area.width.saturating_sub(width)) / 2;
    let y = area.y + (area.height.saturating_sub(height)) / 2;
    Rect {
        x,
        y,
        width: width.min(area.width),
        height: height.min(area.height),
    }
}

/// Render compact logo in top-left corner (for main UI header)
/// Clean, minimal design that matches OpenCode style
pub fn render_logo_compact(frame: &mut Frame, area: Rect) {
    // Simple bold text for narrow areas
    if area.width < 15 || area.height < 4 {
        let logo_paragraph = Paragraph::new(LOGO_MINIMAL)
            .style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD | Modifier::DIM),
            )
            .alignment(Alignment::Left);
        frame.render_widget(logo_paragraph, area);
    } else {
        let lines: Vec<Line> = LOGO_HEADER
            .lines()
            .map(|line| {
                Line::from(Span::styled(
                    line,
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::DIM),
                ))
            })
            .collect();

        let logo_paragraph = Paragraph::new(lines)
            .alignment(Alignment::Left)
            .block(Block::default().borders(Borders::NONE));
        frame.render_widget(logo_paragraph, area);
    }
}

/// Render a minimal inline logo for very small spaces
pub fn render_logo_inline(frame: &mut Frame, area: Rect) {
    let logo = Paragraph::new("ZCODE")
        .style(
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD | Modifier::DIM),
        )
        .alignment(Alignment::Left);
    frame.render_widget(logo, area);
}
