// src/input/modes/prompt_entry.rs - Keyboard input handling for prompt entry mode

use crate::input::handler::{key_helpers, Action, InputHandler, InputResult};
use crate::state::{Mode, State};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

/// Input handler for prompt entry mode
pub struct PromptEntryHandler;

impl InputHandler for PromptEntryHandler {
    fn handle_key(&mut self, key: &KeyEvent, state: &mut State) -> InputResult {
        use key_helpers::*;

        // Text input - regular characters
        if let KeyCode::Char(ch) = key.code {
            if !has_any_modifier(key) || is_printable(ch) {
                state.prompt_buffer.insert(state.cursor_position, ch);
                state.cursor_position += 1;
                return InputResult::Consumed;
            }
        }

        // Handle special keys
        if is_key(key, KeyCode::Backspace) && state.cursor_position > 0 {
            state.cursor_position -= 1;
            state.prompt_buffer.remove(state.cursor_position);
            return InputResult::Consumed;
        }

        if is_key(key, KeyCode::Delete) && state.cursor_position < state.prompt_buffer.len() {
            state.prompt_buffer.remove(state.cursor_position);
            return InputResult::Consumed;
        }

        if is_key(key, KeyCode::Enter) {
            let prompt = state.prompt_buffer.clone();
            state.prompt_buffer.clear();
            state.cursor_position = 0;
            return InputResult::Action(Action::SubmitPrompt(prompt));
        }

        if is_key(key, KeyCode::Esc) {
            state.prompt_buffer.clear();
            state.cursor_position = 0;
            return InputResult::ModeChange(Mode::ProviderSelect);
        }

        // Arrow keys for cursor movement
        if is_key(key, KeyCode::Left) && state.cursor_position > 0 {
            state.cursor_position -= 1;
            return InputResult::Consumed;
        }

        if is_key(key, KeyCode::Right) && state.cursor_position < state.prompt_buffer.len() {
            state.cursor_position += 1;
            return InputResult::Consumed;
        }

        if is_key(key, KeyCode::Home) {
            state.cursor_position = 0;
            return InputResult::Consumed;
        }

        if is_key(key, KeyCode::End) {
            state.cursor_position = state.prompt_buffer.len();
            return InputResult::Consumed;
        }

        // Ctrl+U to clear line
        if is_char(key, 'u') && has_modifier(key, KeyModifiers::CONTROL) {
            state.prompt_buffer.clear();
            state.cursor_position = 0;
            return InputResult::Consumed;
        }

        InputResult::Ignored
    }

    fn keybindings(&self) -> Vec<String> {
        vec![
            "Enter: submit prompt".to_string(),
            "Esc: cancel".to_string(),
            "Ctrl+U: clear line".to_string(),
        ]
    }
}

impl PromptEntryHandler {
    pub fn new() -> Self {
        Self
    }
}

/// Check if a character is printable and should be inserted
fn is_printable(ch: char) -> bool {
    !ch.is_control() && ch != '\n' && ch != '\r'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_printable_chars() {
        assert!(is_printable('a'));
        assert!(is_printable('1'));
        assert!(is_printable(' '));
        assert!(!is_printable('\n'));
        assert!(!is_printable('\r'));
    }
}
