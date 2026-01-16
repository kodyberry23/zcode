use ratatui::{layout::Rect, Frame};

use crate::components::Component;
use crate::model::AppModel;
use crate::state::MessageStatus;

pub struct ChatPanel;

impl ChatPanel {
    pub fn new() -> Self {
        Self
    }
}

impl Component for ChatPanel {
    fn view(&self, frame: &mut Frame, area: Rect, model: &AppModel) {
        let messages = &model.state.chat_history.messages;
        if messages.is_empty() {
            crate::ui::session_turn::render_empty_chat(frame, area, &model.theme);
        } else {
            crate::ui::session_turn::render_session_turns(frame, area, messages, &model.theme);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{backend::TestBackend, Terminal};

    #[test]
    fn renders_empty_chat() {
        let backend = TestBackend::new(40, 10);
        let mut terminal = Terminal::new(backend).unwrap();
        let component = ChatPanel::new();
        let model = AppModel::default();

        terminal
            .draw(|f| {
                let area = f.area();
                component.view(f, area, &model);
            })
            .unwrap();
    }
}
