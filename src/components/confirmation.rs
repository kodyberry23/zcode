use ratatui::{layout::Rect, Frame};

use crate::components::Component;
use crate::model::AppModel;

pub struct Confirmation;

impl Confirmation {
    pub fn new() -> Self {
        Self
    }
}

impl Component for Confirmation {
    fn view(&self, frame: &mut Frame, area: Rect, model: &AppModel) {
        crate::ui::renderers::render_confirmation(frame, &model.state, &model.theme);
    }
}
