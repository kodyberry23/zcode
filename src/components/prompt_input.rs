use ratatui::{layout::Rect, Frame};

use crate::components::Component;
use crate::model::AppModel;

pub struct PromptInput;

impl PromptInput {
    pub fn new() -> Self {
        Self
    }
}

impl Component for PromptInput {
    fn view(&self, frame: &mut Frame, area: Rect, model: &AppModel) {
        crate::ui::prompt_input::render_prompt_input(frame, area, &model.state, &model.theme);
    }
}
