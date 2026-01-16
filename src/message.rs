use crate::input::modes::InputMode;
use crate::state::Mode;
use std::path::PathBuf;

/// Basic navigation directions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// Messages drive updates to the application model (Elm-style update).
#[derive(Debug, Clone)]
pub enum Message {
    // Navigation
    Navigate(Direction),
    ScrollTo(usize),

    // Modes
    SetMode(Mode),
    SetInputMode(InputMode),
    PushInputMode(InputMode),
    PopInputMode,

    // Provider actions
    SelectProvider(usize),
    DetectProviders,

    // Prompt actions
    SubmitPrompt(String),
    CancelPrompt,

    // Diff actions
    AcceptHunk(usize),
    RejectHunk(usize),
    AcceptAll,
    RejectAll,
    ApplyChanges,

    // UI actions
    ToggleSidebar,
    ToggleHelp,
    Search(String),

    // Editor actions
    OpenEditor { path: PathBuf, line: Option<usize> },

    // System
    Quit,
    Resize(u16, u16),
    Tick,
}
