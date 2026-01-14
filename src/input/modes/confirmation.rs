// src/input/modes/confirmation.rs - Keyboard input handling for confirmation dialogs

use crate::input::handler::{key_helpers, Action, InputHandler, InputResult};
use crate::state::{Mode, State};
use zellij_tile::prelude::{BareKey, KeyWithModifier};

/// Input handler for confirmation mode
pub struct ConfirmationHandler;

impl InputHandler for ConfirmationHandler {
    fn handle_key(&mut self, key: &KeyWithModifier, state: &mut State) -> InputResult {
        use key_helpers::*;

        if is_char(key, 'y') || is_key(key, BareKey::Enter) {
            return InputResult::Action(Action::Confirm);
        }

        if is_char(key, 'n') || is_key(key, BareKey::Esc) {
            return InputResult::Action(Action::Deny);
        }

        InputResult::Ignored
    }

    fn keybindings(&self) -> Vec<String> {
        vec!["y/Enter: confirm".to_string(), "n/Esc: deny".to_string()]
    }
}

impl ConfirmationHandler {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_creation() {
        let handler = ConfirmationHandler::new();
        assert_eq!(handler.keybindings().len(), 2);
    }
}
