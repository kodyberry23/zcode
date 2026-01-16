use ratatui::{layout::Rect, Frame};

use crate::components::Component;
use crate::model::AppModel;

pub struct ProviderSelect;

impl ProviderSelect {
    pub fn new() -> Self {
        Self
    }
}

impl Component for ProviderSelect {
    fn view(&self, frame: &mut Frame, area: Rect, model: &AppModel) {
        // Reuse existing renderer for provider selection
        frame.render_widget(ratatui::widgets::Clear, area);
        crate::ui::renderers::render_provider_select(frame, &model.state, &model.theme);
    }
}
