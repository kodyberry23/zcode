// src/input/modes/provider_select.rs - Keyboard input handling for provider selection mode

use crate::input::handler::{key_helpers, Action, InputHandler, InputResult};
use crate::state::{Mode, State};
use zellij_tile::prelude::{BareKey, KeyWithModifier};

/// Input handler for provider selection mode
pub struct ProviderSelectHandler;

impl InputHandler for ProviderSelectHandler {
    fn handle_key(&mut self, key: &KeyWithModifier, state: &mut State) -> InputResult {
        use key_helpers::*;

        // Navigation
        if is_char(key, 'j') {
            if state.selected_provider_idx < state.available_providers.len().saturating_sub(1) {
                state.selected_provider_idx += 1;
                return InputResult::Consumed;
            }
        }

        if is_char(key, 'k') {
            if state.selected_provider_idx > 0 {
                state.selected_provider_idx -= 1;
                return InputResult::Consumed;
            }
        }

        // Select provider
        if is_key(key, BareKey::Enter) {
            return InputResult::Action(Action::SelectProvider(state.selected_provider_idx));
        }

        // Jump to first/last
        if is_char(key, 'g') {
            state.selected_provider_idx = 0;
            return InputResult::Consumed;
        }

        if is_char(key, 'G') && has_modifier(key, zellij_tile::prelude::KeyModifier::Shift) {
            state.selected_provider_idx = state.available_providers.len().saturating_sub(1);
            return InputResult::Consumed;
        }

        // Quit
        if is_char(key, 'q') {
            return InputResult::Action(Action::Quit);
        }

        InputResult::Ignored
    }

    fn keybindings(&self) -> Vec<String> {
        vec![
            "j/k: navigate".to_string(),
            "Enter: select".to_string(),
            "q: quit".to_string(),
        ]
    }
}

impl ProviderSelectHandler {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_creation() {
        let handler = ProviderSelectHandler::new();
        let keybindings = handler.keybindings();
        assert_eq!(keybindings.len(), 3);
        assert!(keybindings[0].contains("navigate"));
    }
}
