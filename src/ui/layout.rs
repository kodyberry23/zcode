// src/ui/layout.rs - Layout helper functions

use ratatui::layout::{Constraint, Direction, Layout, Rect};

/// Responsive layout breakpoints (in columns).
pub struct LayoutBreakpoints {
    pub compact: u16,
    pub wide: u16,
}

impl Default for LayoutBreakpoints {
    fn default() -> Self {
        Self {
            compact: 80,
            wide: 120,
        }
    }
}

/// High-level application layouts based on available width.
pub enum AppLayout {
    Compact {
        header: Rect,
        content: Rect,
        input: Rect,
        status: Rect,
    },
    Normal {
        header: Rect,
        content: Rect,
        input: Rect,
        status: Rect,
        sidebar: Option<Rect>,
    },
    Wide {
        header: Rect,
        chat: Rect,
        diff: Rect,
        input: Rect,
        status: Rect,
        sidebar: Rect,
    },
}

/// Computes responsive layouts for the app.
pub struct LayoutManager {
    breakpoints: LayoutBreakpoints,
}

impl LayoutManager {
    pub fn new(breakpoints: LayoutBreakpoints) -> Self {
        Self { breakpoints }
    }

    pub fn compute(&self, area: Rect, sidebar_visible: bool) -> AppLayout {
        // Base vertical split: header, content, input, status with spacing
        let vertical = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header (compact)
                Constraint::Min(8),    // Content (flexible, min 8)
                Constraint::Min(3),    // Input (flexible 3-6 lines)
                Constraint::Max(6),    // Cap multiline input
                Constraint::Length(1), // Status bar
            ])
            .split(area);

        let header = vertical[0];
        let content_area = vertical[1];
        let input = vertical[2];
        let status = vertical[4];

        if area.width < self.breakpoints.compact {
            AppLayout::Compact {
                header,
                content: content_area,
                input,
                status,
            }
        } else if area.width < self.breakpoints.wide {
            let sidebar = if sidebar_visible {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Min(50),    // Content (flexible, min 50 cols)
                        Constraint::Length(25), // Sidebar (fixed 25 cols)
                    ])
                    .split(content_area);
                Some(chunks[1])
            } else {
                None
            };

            AppLayout::Normal {
                header,
                content: content_area,
                input,
                status,
                sidebar,
            }
        } else {
            let horizontal = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Min(40),    // Chat (flexible, min 40 cols)
                    Constraint::Min(30),    // Diff (flexible, min 30 cols)
                    Constraint::Length(25), // Sidebar (fixed width)
                ])
                .split(content_area);

            let chat = horizontal[0];
            let diff = horizontal[1];
            let sidebar = horizontal[2];

            AppLayout::Wide {
                header,
                chat,
                diff,
                input,
                status,
                sidebar,
            }
        }
    }
}

/// Create a standard three-section layout: header, content, footer
pub fn main_layout(area: Rect) -> (Rect, Rect, Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Content (flexible)
            Constraint::Length(3), // Footer
        ])
        .split(area);

    (chunks[0], chunks[1], chunks[2])
}

/// Create a centered dialog/modal with specified width and height
pub fn centered_dialog(area: Rect, width: u16, height: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((area.height.saturating_sub(height)) / 2),
            Constraint::Length(height),
            Constraint::Min(0),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((area.width.saturating_sub(width)) / 2),
            Constraint::Length(width),
            Constraint::Min(0),
        ])
        .split(popup_layout[1])[1]
}

/// Create a centered rect with max-width constraint (better than percentage for wide terminals)
pub fn max_width_centered(area: Rect, max_width: u16) -> Rect {
    if area.width > max_width {
        let padding = (area.width - max_width) / 2;
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(padding),
                Constraint::Length(max_width),
                Constraint::Min(0),
            ])
            .split(area)[1]
    } else {
        // On narrow terminals, use small margins
        Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(2),
                Constraint::Min(10),
                Constraint::Length(2),
            ])
            .split(area)[1]
    }
}

/// Create a centered rect with percentage-based dimensions
pub fn centered_rect_percent(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

/// Split content area into two columns
pub fn two_column_layout(area: Rect, left_width: u16) -> (Rect, Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(left_width), Constraint::Min(0)])
        .split(area);

    (chunks[0], chunks[1])
}

/// Split content area into two rows
pub fn two_row_layout(area: Rect, top_height: u16) -> (Rect, Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(top_height), Constraint::Min(0)])
        .split(area);

    (chunks[0], chunks[1])
}

/// Create a layout for a prompt input with label and input box
pub fn prompt_input_layout(area: Rect) -> (Rect, Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Label
            Constraint::Min(3),    // Input box
        ])
        .split(area);

    (chunks[0], chunks[1])
}

/// Create responsive chat layout with optional sidebar
pub fn responsive_chat_layout(area: Rect, sidebar_visible: bool) -> (Rect, Option<Rect>) {
    if area.width > 120 && sidebar_visible {
        // Side-by-side for wide terminals
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
            .split(area);
        (chunks[0], Some(chunks[1]))
    } else if area.width > 80 && sidebar_visible {
        // Stacked for medium terminals
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Min(10), Constraint::Length(10)])
            .split(area);
        (chunks[0], Some(chunks[1]))
    } else {
        // No sidebar for narrow terminals
        (area, None)
    }
}

/// Create layout with chat, status bar, and input
pub fn chat_layout_with_status(area: Rect) -> (Rect, Rect, Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(5),    // Chat (flexible)
            Constraint::Length(1), // Status bar (fixed)
            Constraint::Length(3), // Input area (fixed)
        ])
        .split(area);

    (chunks[0], chunks[1], chunks[2])
}
