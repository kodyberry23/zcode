use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, Borders, Paragraph, Widget},
};

/// Simple diff hunk widget (placeholder for richer view).
pub struct DiffHunk<'a> {
    pub title: &'a str,
    pub lines: Vec<Line<'a>>,
    pub style: Style,
}

impl<'a> Widget for DiffHunk<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default().borders(Borders::ALL).title(self.title);
        let paragraph = Paragraph::new(self.lines).block(block).style(self.style);
        paragraph.render(area, buf);
    }
}
