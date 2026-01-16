use ratatui::{layout::Rect, Frame};

use crate::message::Message;
use crate::model::AppModel;

/// Base trait for all UI components.
pub trait Component {
    /// Optional initialization hook.
    fn init(&mut self, _model: &mut AppModel) -> Option<Message> {
        None
    }

    /// Update component-specific state; may emit a global Message.
    fn update(&mut self, _msg: &Message, _model: &mut AppModel) -> Option<Message> {
        None
    }

    /// Render component into the provided area.
    fn view(&self, frame: &mut Frame, area: Rect, model: &AppModel);

    /// Handle a key when this component is focused; may emit a message.
    fn handle_key(
        &mut self,
        _key: crossterm::event::KeyEvent,
        _model: &mut AppModel,
    ) -> Option<Message> {
        None
    }

    /// Whether this component is currently focused.
    fn focused(&self) -> bool {
        false
    }
}

pub mod chat_panel;
pub mod command_palette;
pub mod confirmation;
pub mod diff_view;
pub mod header;
pub mod help;
pub mod prompt_input;
pub mod provider_select;
pub mod sidebar;
pub mod status_bar;
