use ratatui::{layout::Rect, Frame};

use crate::components::Component;
use crate::model::AppModel;

pub struct StatusBar;

impl StatusBar {
    pub fn new() -> Self {
        Self
    }
}

impl Component for StatusBar {
    fn view(&self, frame: &mut Frame, area: Rect, model: &AppModel) {
        crate::ui::status_bar::render_status_bar(
            frame,
            area,
            &model.state.status_info,
            &model.theme,
        );
    }
}
