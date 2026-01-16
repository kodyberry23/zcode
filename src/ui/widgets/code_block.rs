use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

/// Minimal code block widget with monospace styling.
pub struct CodeBlock<'a> {
    pub title: Option<&'a str>,
    pub lines: Vec<Line<'a>>,
    pub style: Style,
}

impl<'a> CodeBlock<'a> {
    pub fn new(lines: Vec<Line<'a>>) -> Self {
        Self {
            title: None,
            lines,
            style: Style::default(),
        }
    }

    pub fn title(mut self, title: &'a str) -> Self {
        self.title = Some(title);
        self
    }

    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl<'a> Widget for CodeBlock<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .title(self.title.unwrap_or(" Code "));

        let paragraph = Paragraph::new(self.lines).block(block).style(self.style);
        paragraph.render(area, buf);
    }
}
