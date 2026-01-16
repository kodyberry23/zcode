// src/ui/search.rs - Search functionality for chat history

use crate::state::{ChatHistory, MessageFilter};
use ratatui::{
    layout::Rect,
    style::Style,
    text::Line,
    widgets::{Block, Borders, ListState, Paragraph},
    Frame,
};

/// Search mode state
pub struct SearchState {
    pub query: String,
    pub cursor_pos: usize,
    pub current_match: Option<usize>,
    pub matches: Vec<usize>,
}

impl Default for SearchState {
    fn default() -> Self {
        Self {
            query: String::new(),
            cursor_pos: 0,
            current_match: None,
            matches: Vec::new(),
        }
    }
}

impl SearchState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Update search query and find matches
    pub fn update_query(&mut self, query: String, chat_history: &ChatHistory) {
        self.query = query;
        self.cursor_pos = self.query.len();
        self.find_matches(chat_history);
    }

    /// Find all message indices that match the search query
    fn find_matches(&mut self, chat_history: &ChatHistory) {
        if self.query.is_empty() {
            self.matches.clear();
            self.current_match = None;
            return;
        }

        let query_lower = self.query.to_lowercase();
        self.matches = chat_history
            .messages
            .iter()
            .enumerate()
            .filter(|(_, msg)| msg.content.to_lowercase().contains(&query_lower))
            .map(|(idx, _)| idx)
            .collect();

        if !self.matches.is_empty() {
            self.current_match = Some(0);
        } else {
            self.current_match = None;
        }
    }

    /// Move to next match
    pub fn next_match(&mut self) {
        if self.matches.is_empty() {
            return;
        }

        if let Some(current) = self.current_match {
            let next = (current + 1) % self.matches.len();
            self.current_match = Some(next);
        } else if !self.matches.is_empty() {
            self.current_match = Some(0);
        }
    }

    /// Move to previous match
    pub fn prev_match(&mut self) {
        if self.matches.is_empty() {
            return;
        }

        if let Some(current) = self.current_match {
            let prev = if current == 0 {
                self.matches.len() - 1
            } else {
                current - 1
            };
            self.current_match = Some(prev);
        } else if !self.matches.is_empty() {
            self.current_match = Some(self.matches.len() - 1);
        }
    }

    /// Get the message index of the current match
    pub fn current_match_index(&self) -> Option<usize> {
        self.current_match
            .and_then(|idx| self.matches.get(idx).copied())
    }

    /// Clear search
    pub fn clear(&mut self) {
        self.query.clear();
        self.cursor_pos = 0;
        self.matches.clear();
        self.current_match = None;
    }
}

/// Render search input overlay
pub fn render_search_input(
    frame: &mut Frame,
    area: Rect,
    search_state: &SearchState,
    theme: &crate::ui::colors::Theme,
) {
    let prompt = if search_state.matches.is_empty() && !search_state.query.is_empty() {
        format!("/{} (no matches)", search_state.query)
    } else if !search_state.query.is_empty() {
        let match_info = if let Some(current) = search_state.current_match {
            format!(" {}/{}", current + 1, search_state.matches.len())
        } else {
            String::new()
        };
        format!("/{}{}", search_state.query, match_info)
    } else {
        "/".to_string()
    };

    let line = Line::from(prompt);
    let paragraph = Paragraph::new(line)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(theme.border_style)
                .title(" Search "),
        )
        .style(theme.prompt_style);

    frame.render_widget(paragraph, area);
}

/// Apply filter to chat history
pub fn apply_filter(chat_history: &mut ChatHistory, filter: MessageFilter) {
    chat_history.filter = Some(filter);
}

/// Clear filter
pub fn clear_filter(chat_history: &mut ChatHistory) {
    chat_history.filter = None;
}

/// Jump to a specific message by ID
pub fn jump_to_message(chat_history: &mut ChatHistory, message_id: usize) -> bool {
    if let Some(pos) = chat_history
        .messages
        .iter()
        .position(|msg| msg.id == message_id)
    {
        chat_history.scroll_state.select(Some(pos));
        true
    } else {
        false
    }
}

/// Update scroll state for chat history
pub fn update_chat_scroll_state(chat_history: &mut ChatHistory, selected: Option<usize>) {
    chat_history.scroll_state.select(selected);
}
