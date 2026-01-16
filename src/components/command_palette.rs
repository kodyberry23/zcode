use ratatui::{layout::Rect, Frame};

use crate::components::Component;
use crate::model::AppModel;

pub struct CommandPalette;

impl CommandPalette {
    pub fn new() -> Self {
        Self
    }
}

impl Component for CommandPalette {
    fn view(&self, frame: &mut Frame, area: Rect, model: &AppModel) {
        // Reuse command input rendering from app
        frame.render_widget(ratatui::widgets::Clear, area);
        let text = format!(":{}", model.state.command_buffer);
        use ratatui::style::Style;
        use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
        let paragraph = Paragraph::new(text)
            .style(Style::default().fg(ratatui::style::Color::White))
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(model.theme.border_style)
                    .title(" Command "),
            );
        frame.render_widget(paragraph, area);
    }
}
