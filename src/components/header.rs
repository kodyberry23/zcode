use ratatui::{layout::Rect, Frame};

use crate::components::Component;
use crate::model::AppModel;

/// Header component rendering the logo + session/status.
pub struct Header;

impl Header {
    pub fn new() -> Self {
        Self
    }
}

impl Component for Header {
    fn view(&self, frame: &mut Frame, area: Rect, model: &AppModel) {
        crate::ui::header::render_header(frame, area, &model.state, &model.theme);
    }
}
