use ratatui::{layout::Rect, Frame};

use crate::components::Component;
use crate::model::AppModel;
use crate::state::Mode;

pub struct HelpOverlay;

impl HelpOverlay {
    pub fn new() -> Self {
        Self
    }
}

impl Component for HelpOverlay {
    fn view(&self, frame: &mut Frame, area: Rect, model: &AppModel) {
        crate::ui::help::render_help(frame, area, &Mode::Help, &model.theme);
    }
}
