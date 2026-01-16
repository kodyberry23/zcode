use ratatui::{layout::Rect, Frame};

use crate::components::Component;
use crate::model::AppModel;

pub struct DiffView;

impl DiffView {
    pub fn new() -> Self {
        Self
    }
}

impl Component for DiffView {
    fn view(&self, frame: &mut Frame, area: Rect, model: &AppModel) {
        crate::ui::overlay_diff::render_overlay_diff(
            frame,
            area,
            &model.state.overlay_diff_state,
            &model.theme,
        );
    }
}
