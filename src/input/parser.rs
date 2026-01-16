use std::time::{Duration, Instant};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::input::keymap::KeymapRegistry;
use crate::input::modes::InputMode;
use crate::message::Message;

/// Result of processing a key against the keymap.
pub enum KeyParseOutcome {
    Matched(Message),
    Pending,
    NoMatch,
}

/// Parses multi-key sequences (e.g., `gg`, `<C-b>`) with a timeout.
pub struct KeySequenceParser {
    buffer: Vec<String>,
    last_key_time: Instant,
    timeout: Duration,
}

impl KeySequenceParser {
    pub fn new(timeout: Duration) -> Self {
        Self {
            buffer: Vec::new(),
            last_key_time: Instant::now(),
            timeout,
        }
    }

    pub fn process(
        &mut self,
        key: KeyEvent,
        keymap: &KeymapRegistry,
        mode: InputMode,
    ) -> KeyParseOutcome {
        // Clear buffer on timeout
        if self.last_key_time.elapsed() > self.timeout {
            self.buffer.clear();
        }

        self.last_key_time = Instant::now();
        self.buffer.push(key_to_token(key));

        // Exact match?
        if let Some(msg) = keymap.lookup(mode, &self.buffer) {
            self.buffer.clear();
            return KeyParseOutcome::Matched(msg);
        }

        // Prefix match?
        if keymap.has_prefix(mode, &self.buffer) {
            return KeyParseOutcome::Pending;
        }

        // No match: clear buffer
        self.buffer.clear();
        KeyParseOutcome::NoMatch
    }
}

fn key_to_token(key: KeyEvent) -> String {
    let modifiers = key.modifiers;
    match key.code {
        KeyCode::Char(c) => {
            if modifiers.contains(KeyModifiers::CONTROL) {
                format!("<C-{}>", c.to_ascii_uppercase())
            } else if modifiers.contains(KeyModifiers::ALT) {
                format!("<A-{}>", c)
            } else {
                c.to_string()
            }
        }
        KeyCode::Enter => "<Enter>".to_string(),
        KeyCode::Esc => "<Esc>".to_string(),
        KeyCode::Tab => "<Tab>".to_string(),
        KeyCode::Backspace => "<Backspace>".to_string(),
        KeyCode::Left => "<Left>".to_string(),
        KeyCode::Right => "<Right>".to_string(),
        KeyCode::Up => "<Up>".to_string(),
        KeyCode::Down => "<Down>".to_string(),
        other => format!("<{:?}>", other),
    }
}
