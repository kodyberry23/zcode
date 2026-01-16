use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Rect},
    style::{Color, Style},
    widgets::{Paragraph, Widget},
};

/// Lightweight spinner widget.
pub struct Spinner<'a> {
    pub text: &'a str,
    pub color: Color,
}

impl<'a> Spinner<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            color: Color::Yellow,
        }
    }
}

impl<'a> Widget for Spinner<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let spinner_chars = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
        let frame_idx = (std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as usize)
            / 80
            % spinner_chars.len();
        let text = format!("{} {}", spinner_chars[frame_idx], self.text);
        let paragraph = Paragraph::new(text)
            .style(Style::default().fg(self.color))
            .alignment(Alignment::Center);
        paragraph.render(area, buf);
    }
}
