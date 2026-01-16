// src/input/handler.rs - Input handler trait and result types

use crate::state::{Mode, State};
use crossterm::event::KeyEvent;

/// Result of handling a key event
#[derive(Debug, Clone, PartialEq)]
pub enum InputResult {
    /// Key was handled and consumed
    Consumed,
    /// Key was not relevant to this handler
    Ignored,
    /// Switch to a different mode
    ModeChange(Mode),
    /// Execute an action
    Action(Action),
}

/// Actions triggered by keyboard input
#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    /// Navigate to next item
    Next,
    /// Navigate to previous item
    Previous,
    /// Accept current hunk
    AcceptCurrent,
    /// Reject current hunk
    RejectCurrent,
    /// Accept all hunks
    AcceptAll,
    /// Reject all hunks
    RejectAll,
    /// Apply all accepted changes
    ApplyChanges,
    /// Select a provider (by index)
    SelectProvider(usize),
    /// Submit prompt text
    SubmitPrompt(String),
    /// Confirm action (yes)
    Confirm,
    /// Deny/cancel action
    Deny,
    /// Switch to next file
    NextFile,
    /// Switch to previous file
    PreviousFile,
    /// Toggle line numbers
    ToggleLineNumbers,
    /// Jump to beginning
    Beginning,
    /// Jump to end
    End,
    /// Scroll view up
    ScrollUp,
    /// Scroll view down
    ScrollDown,
    /// Page up
    PageUp,
    /// Page down
    PageDown,
    /// Quit application
    Quit,
}

/// Trait for handling keyboard input in different modes
pub trait InputHandler {
    /// Handle a key event and return the result
    fn handle_key(&mut self, key: &KeyEvent, state: &mut State) -> InputResult;

    /// Get available keybindings for this handler
    fn keybindings(&self) -> Vec<String>;
}

// Legacy trait for backward compatibility during migration
#[allow(dead_code)]
pub trait InputHandlerLegacy {
    fn handle_key_legacy(&mut self, key: &KeyEvent, state: &mut State) -> InputResult;
    fn keybindings(&self) -> Vec<String>;
}

/// Helper functions for key matching with Crossterm
pub mod key_helpers {
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    /// Check if a key matches a specific character
    pub fn is_char(key: &KeyEvent, c: char) -> bool {
        if let KeyCode::Char(ch) = key.code {
            ch.eq_ignore_ascii_case(&c)
        } else {
            false
        }
    }

    /// Check if a key matches a specific key code
    pub fn is_key(key: &KeyEvent, target: KeyCode) -> bool {
        key.code == target
    }

    /// Check if a key has a specific modifier
    pub fn has_modifier(key: &KeyEvent, modifier: KeyModifiers) -> bool {
        key.modifiers.contains(modifier)
    }

    /// Check if a key has any modifiers
    pub fn has_any_modifier(key: &KeyEvent) -> bool {
        !key.modifiers.is_empty()
    }

    /// Get all modifiers as a readable string
    pub fn format_modifiers(modifiers: KeyModifiers) -> String {
        let mut parts = Vec::new();
        if modifiers.contains(KeyModifiers::CONTROL) {
            parts.push("CTRL");
        }
        if modifiers.contains(KeyModifiers::ALT) {
            parts.push("ALT");
        }
        if modifiers.contains(KeyModifiers::SHIFT) {
            parts.push("SHIFT");
        }
        parts.join("+")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_action_debug() {
        let action = Action::AcceptCurrent;
        assert_eq!(format!("{:?}", action), "AcceptCurrent");
    }

    #[test]
    fn test_input_result_equality() {
        assert_eq!(InputResult::Consumed, InputResult::Consumed);
        assert_ne!(InputResult::Consumed, InputResult::Ignored);
    }
}
