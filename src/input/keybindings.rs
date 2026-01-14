// src/input/keybindings.rs - Keybinding definitions and configuration

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A single keybinding combo
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct KeyCombo {
    pub key: String,
    pub modifiers: Vec<String>,
}

impl KeyCombo {
    pub fn new(key: &str) -> Self {
        Self {
            key: key.to_string(),
            modifiers: Vec::new(),
        }
    }

    pub fn with_ctrl(mut self) -> Self {
        self.modifiers.push("ctrl".to_string());
        self
    }

    pub fn with_shift(mut self) -> Self {
        self.modifiers.push("shift".to_string());
        self
    }

    pub fn with_alt(mut self) -> Self {
        self.modifiers.push("alt".to_string());
        self
    }

    pub fn to_display_string(&self) -> String {
        if self.modifiers.is_empty() {
            self.key.clone()
        } else {
            format!(
                "{} {}",
                self.modifiers.join("+").to_uppercase(),
                self.key.to_uppercase()
            )
        }
    }
}

/// Map of keybindings for a specific mode
#[derive(Debug, Clone)]
pub struct KeybindingMap {
    pub bindings: HashMap<String, String>,
}

impl KeybindingMap {
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
        }
    }

    pub fn insert(&mut self, key: String, action: String) {
        self.bindings.insert(key, action);
    }

    pub fn get(&self, key: &str) -> Option<&String> {
        self.bindings.get(key)
    }
}

impl Default for KeybindingMap {
    fn default() -> Self {
        Self::new()
    }
}

/// Default keybindings for diff review mode
pub fn default_diff_review_bindings() -> KeybindingMap {
    let mut map = KeybindingMap::new();
    map.insert("j".to_string(), "next_hunk".to_string());
    map.insert("k".to_string(), "prev_hunk".to_string());
    map.insert("J".to_string(), "next_file".to_string());
    map.insert("K".to_string(), "prev_file".to_string());
    map.insert("g".to_string(), "first_hunk".to_string());
    map.insert("G".to_string(), "last_hunk".to_string());
    map.insert("a".to_string(), "accept_hunk".to_string());
    map.insert("r".to_string(), "reject_hunk".to_string());
    map.insert("A".to_string(), "accept_all".to_string());
    map.insert("R".to_string(), "reject_all".to_string());
    map.insert("Enter".to_string(), "apply_changes".to_string());
    map.insert("Space".to_string(), "toggle_details".to_string());
    map.insert("p".to_string(), "switch_provider".to_string());
    map.insert("q".to_string(), "quit".to_string());
    map
}

/// Default keybindings for prompt entry mode
pub fn default_prompt_entry_bindings() -> KeybindingMap {
    let mut map = KeybindingMap::new();
    map.insert("Enter".to_string(), "submit".to_string());
    map.insert("Esc".to_string(), "cancel".to_string());
    map.insert("ctrl+u".to_string(), "clear_line".to_string());
    map
}

/// Default keybindings for provider selection mode
pub fn default_provider_select_bindings() -> KeybindingMap {
    let mut map = KeybindingMap::new();
    map.insert("j".to_string(), "next_provider".to_string());
    map.insert("k".to_string(), "prev_provider".to_string());
    map.insert("Enter".to_string(), "select".to_string());
    map.insert("q".to_string(), "quit".to_string());
    map
}

/// Default keybindings for confirmation dialog
pub fn default_confirmation_bindings() -> KeybindingMap {
    let mut map = KeybindingMap::new();
    map.insert("y".to_string(), "confirm".to_string());
    map.insert("n".to_string(), "deny".to_string());
    map.insert("Enter".to_string(), "confirm".to_string());
    map.insert("Esc".to_string(), "deny".to_string());
    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_combo_display() {
        let combo = KeyCombo::new("j");
        assert_eq!(combo.to_display_string(), "j");

        let combo = KeyCombo::new("c").with_ctrl();
        assert_eq!(combo.to_display_string(), "CTRL C");
    }

    #[test]
    fn test_default_bindings() {
        let bindings = default_diff_review_bindings();
        assert_eq!(bindings.get("j"), Some(&"next_hunk".to_string()));
        assert_eq!(bindings.get("k"), Some(&"prev_hunk".to_string()));
        assert_eq!(bindings.get("q"), Some(&"quit".to_string()));
    }
}
