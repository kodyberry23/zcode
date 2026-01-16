use crate::input::modes::{InputMode, ModeStack};
use crate::state::State;
use crate::ui::colors::Theme;

/// Central application model holding state and UI/input modes.
pub struct AppModel {
    pub state: State,
    pub input_mode: InputMode,
    pub mode_stack: ModeStack,
    pub theme: Theme,
    pub should_quit: bool,
}

impl AppModel {
    pub fn new() -> anyhow::Result<Self> {
        let mut state = State::default();
        state.initialize(&Default::default())?;

        let theme = if state.config.display.color_scheme == "light" {
            Theme::light()
        } else {
            Theme::dark()
        };

        Ok(Self {
            state,
            input_mode: InputMode::Normal,
            mode_stack: ModeStack::default(),
            theme,
            should_quit: false,
        })
    }
}

impl Default for AppModel {
    fn default() -> Self {
        Self {
            state: State::default(),
            input_mode: InputMode::Normal,
            mode_stack: ModeStack::default(),
            theme: Theme::dark(),
            should_quit: false,
        }
    }
}
