use ratatui::{layout::Rect, Frame};

use crate::components::Component;
use crate::model::AppModel;

pub struct Sidebar;

impl Sidebar {
    pub fn new() -> Self {
        Self
    }
}

impl Component for Sidebar {
    fn view(&self, frame: &mut Frame, area: Rect, model: &AppModel) {
        crate::ui::sidebar::render_sidebar(frame, area, &model.state.sidebar_state, &model.theme);
    }
}
