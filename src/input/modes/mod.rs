// src/input/modes/mod.rs - Mode-specific input handlers

pub mod confirmation;
pub mod diff_review;
pub mod prompt_entry;
pub mod provider_select;

pub use confirmation::ConfirmationHandler;
pub use diff_review::DiffReviewHandler;
pub use prompt_entry::PromptEntryHandler;
pub use provider_select::ProviderSelectHandler;

/// Input editing modes (vim-style).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InputMode {
    Normal,
    Insert,
    Visual,
    Command,
    Search,
    ProviderSelect,
    DiffReview,
    Confirmation,
}

/// Simple stack for modal input modes.
#[derive(Debug, Default)]
pub struct ModeStack {
    stack: Vec<InputMode>,
}

impl ModeStack {
    pub fn push(&mut self, mode: InputMode) {
        self.stack.push(mode);
    }

    pub fn pop(&mut self) -> Option<InputMode> {
        self.stack.pop()
    }

    pub fn current(&self) -> Option<InputMode> {
        self.stack.last().copied()
    }
}
