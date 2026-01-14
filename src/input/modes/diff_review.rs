// src/input/modes/diff_review.rs - Keyboard input handling for diff review mode

use crate::input::handler::{key_helpers, Action, InputHandler, InputResult};
use crate::state::{HunkStatus, Mode, State};
use zellij_tile::prelude::{BareKey, KeyWithModifier};

/// Input handler for diff review mode
pub struct DiffReviewHandler;

impl InputHandler for DiffReviewHandler {
    fn handle_key(&mut self, key: &KeyWithModifier, state: &mut State) -> InputResult {
        use key_helpers::*;

        // Navigation
        if is_char(key, 'j') {
            return InputResult::Action(Action::Next);
        }
        if is_char(key, 'k') {
            return InputResult::Action(Action::Previous);
        }
        if is_char(key, 'J') && has_modifier(key, zellij_tile::prelude::KeyModifier::Shift) {
            return InputResult::Action(Action::NextFile);
        }
        if is_char(key, 'K') && has_modifier(key, zellij_tile::prelude::KeyModifier::Shift) {
            return InputResult::Action(Action::PreviousFile);
        }

        // Jump to beginning/end
        if is_char(key, 'g') {
            return InputResult::Action(Action::Beginning);
        }
        if is_char(key, 'G') && has_modifier(key, zellij_tile::prelude::KeyModifier::Shift) {
            return InputResult::Action(Action::Beginning);
        }

        // Accept/Reject
        if is_char(key, 'a') {
            return InputResult::Action(Action::AcceptCurrent);
        }
        if is_char(key, 'r') {
            return InputResult::Action(Action::RejectCurrent);
        }
        if is_char(key, 'A') && has_modifier(key, zellij_tile::prelude::KeyModifier::Shift) {
            return InputResult::Action(Action::AcceptAll);
        }
        if is_char(key, 'R') && has_modifier(key, zellij_tile::prelude::KeyModifier::Shift) {
            return InputResult::Action(Action::RejectAll);
        }

        // Apply changes
        if is_key(key, BareKey::Enter) {
            return InputResult::Action(Action::ApplyChanges);
        }

        // Switch provider
        if is_char(key, 'p') {
            return InputResult::ModeChange(Mode::ProviderSelect);
        }

        // Scroll
        if is_char(key, ' ') {
            return InputResult::Action(Action::ScrollDown);
        }

        // Toggle line numbers
        if is_char(key, 'l') {
            return InputResult::Action(Action::ToggleLineNumbers);
        }

        // Quit
        if is_char(key, 'q') {
            return InputResult::Action(Action::Quit);
        }

        InputResult::Ignored
    }

    fn keybindings(&self) -> Vec<String> {
        vec![
            "j/k: navigate hunks".to_string(),
            "a/r: accept/reject".to_string(),
            "A/R: accept/reject all".to_string(),
            "Enter: apply changes".to_string(),
            "p: switch provider".to_string(),
            "q: quit".to_string(),
        ]
    }
}

impl DiffReviewHandler {
    pub fn new() -> Self {
        Self
    }

    /// Apply navigation actions to state
    pub fn apply_action(action: &Action, state: &mut State) -> bool {
        match action {
            Action::Next => {
                if state.selected_hunk < state.hunks.len().saturating_sub(1) {
                    state.selected_hunk += 1;
                    // Scroll to keep selected hunk visible
                    let content_rows = state.viewport_rows.saturating_sub(4);
                    if state.selected_hunk >= state.scroll_offset + content_rows {
                        state.scroll_offset = state.selected_hunk.saturating_sub(content_rows / 2);
                    }
                    true
                } else {
                    false
                }
            }
            Action::Previous => {
                if state.selected_hunk > 0 {
                    state.selected_hunk -= 1;
                    if state.selected_hunk < state.scroll_offset {
                        state.scroll_offset = state.selected_hunk;
                    }
                    true
                } else {
                    false
                }
            }
            Action::AcceptCurrent => {
                if let Some(hunk) = state.hunks.get_mut(state.selected_hunk) {
                    hunk.status = HunkStatus::Accepted;
                    true
                } else {
                    false
                }
            }
            Action::RejectCurrent => {
                if let Some(hunk) = state.hunks.get_mut(state.selected_hunk) {
                    hunk.status = HunkStatus::Rejected;
                    true
                } else {
                    false
                }
            }
            Action::AcceptAll => {
                for hunk in &mut state.hunks {
                    hunk.status = HunkStatus::Accepted;
                }
                true
            }
            Action::RejectAll => {
                for hunk in &mut state.hunks {
                    hunk.status = HunkStatus::Rejected;
                }
                true
            }
            Action::Beginning => {
                state.selected_hunk = 0;
                state.scroll_offset = 0;
                true
            }
            Action::End => {
                state.selected_hunk = state.hunks.len().saturating_sub(1);
                let content_rows = state.viewport_rows.saturating_sub(4);
                state.scroll_offset = state.hunks.len().saturating_sub(content_rows);
                true
            }
            Action::ScrollDown => {
                let content_rows = state.viewport_rows.saturating_sub(4);
                if state.scroll_offset + content_rows < state.hunks.len() {
                    state.scroll_offset += 1;
                    true
                } else {
                    false
                }
            }
            Action::ScrollUp => {
                if state.scroll_offset > 0 {
                    state.scroll_offset -= 1;
                    true
                } else {
                    false
                }
            }
            Action::PageDown => {
                let content_rows = state.viewport_rows.saturating_sub(4);
                state.scroll_offset = (state.scroll_offset + content_rows)
                    .min(state.hunks.len().saturating_sub(content_rows));
                true
            }
            Action::PageUp => {
                let content_rows = state.viewport_rows.saturating_sub(4);
                state.scroll_offset = state.scroll_offset.saturating_sub(content_rows);
                true
            }
            Action::ToggleLineNumbers => {
                // This would be set in DisplayConfig
                true
            }
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_creation() {
        let handler = DiffReviewHandler::new();
        let keybindings = handler.keybindings();
        assert!(!keybindings.is_empty());
        assert!(keybindings[0].contains("navigate"));
    }
}
