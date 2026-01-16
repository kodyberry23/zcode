use std::collections::HashMap;

use crate::input::modes::InputMode;
use crate::message::Message;

/// Registry mapping key sequences (vim-style) to messages per input mode.
pub struct KeymapRegistry {
    bindings: HashMap<InputMode, HashMap<Vec<String>, Message>>,
}

impl KeymapRegistry {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    pub fn bind(&mut self, mode: InputMode, sequence: &[&str], message: Message) {
        let entry = self.bindings.entry(mode).or_default();
        entry.insert(sequence.iter().map(|s| s.to_string()).collect(), message);
    }

    pub fn lookup(&self, mode: InputMode, sequence: &[String]) -> Option<Message> {
        self.bindings
            .get(&mode)
            .and_then(|m| m.get(sequence))
            .cloned()
    }

    pub fn has_prefix(&self, mode: InputMode, sequence: &[String]) -> bool {
        self.bindings
            .get(&mode)
            .map(|m| m.keys().any(|k| k.starts_with(sequence)))
            .unwrap_or(false)
    }

    /// Default vim-like bindings across modes.
    pub fn default_vim() -> Self {
        use Message::*;

        let mut registry = Self::new();

        // Normal mode navigation
        registry.bind(
            InputMode::Normal,
            &["j"],
            Navigate(crate::message::Direction::Down),
        );
        registry.bind(
            InputMode::Normal,
            &["k"],
            Navigate(crate::message::Direction::Up),
        );
        registry.bind(
            InputMode::Normal,
            &["h"],
            Navigate(crate::message::Direction::Left),
        );
        registry.bind(
            InputMode::Normal,
            &["l"],
            Navigate(crate::message::Direction::Right),
        );
        registry.bind(InputMode::Normal, &["g", "g"], ScrollTo(0));
        registry.bind(InputMode::Normal, &["G"], ScrollTo(usize::MAX));
        registry.bind(InputMode::Normal, &["/"], Search(String::new()));
        registry.bind(
            InputMode::Normal,
            &[":"],
            SetMode(crate::state::Mode::CommandMode),
        );
        registry.bind(InputMode::Normal, &["?"], ToggleHelp);
        registry.bind(InputMode::Normal, &["q"], Quit);
        registry.bind(InputMode::Normal, &["<C-b>"], ToggleSidebar);

        // Insert mode exits
        registry.bind(
            InputMode::Insert,
            &["<Esc>"],
            SetInputMode(InputMode::Normal),
        );
        registry.bind(
            InputMode::Insert,
            &["<C-c>"],
            SetInputMode(InputMode::Normal),
        );

        // Diff review mode actions
        registry.bind(InputMode::DiffReview, &["y"], AcceptHunk(0));
        registry.bind(InputMode::DiffReview, &["n"], RejectHunk(0));
        registry.bind(InputMode::DiffReview, &["Y"], AcceptAll);
        registry.bind(InputMode::DiffReview, &["N"], RejectAll);
        registry.bind(InputMode::DiffReview, &["<Enter>"], ApplyChanges);

        // Command/help escape
        registry.bind(
            InputMode::Command,
            &["<Esc>"],
            SetInputMode(InputMode::Normal),
        );
        registry.bind(
            InputMode::Search,
            &["<Esc>"],
            SetInputMode(InputMode::Normal),
        );

        registry
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::message::Direction;

    #[test]
    fn test_default_vim_navigation() {
        let km = KeymapRegistry::default_vim();
        let seq = vec!["j".to_string()];
        assert!(matches!(
            km.lookup(InputMode::Normal, &seq),
            Some(Message::Navigate(Direction::Down))
        ));
    }

    #[test]
    fn test_prefix_recognition() {
        let km = KeymapRegistry::default_vim();
        let seq = vec!["g".to_string()];
        assert!(km.has_prefix(InputMode::Normal, &seq));
    }
}
