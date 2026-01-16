//! Main plugin state and data structures
//!
//! This module defines the core data types and state machine for the ZCode plugin.
//! It manages the application's modes, user interactions, and the flow between
//! prompting, diff review, and file application.
use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::PathBuf;

use crate::config::Config;
use crate::error::ErrorDisplay;
use crate::providers::AIProvider;
use crate::session::SessionManager;
use chrono::{DateTime, Utc};
use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// State of provider detection process
#[derive(Debug, Clone, PartialEq, Default)]
pub enum DetectionState {
    /// Detection has not started yet
    #[default]
    NotStarted,
    /// Detection is in progress
    InProgress,
    /// Detection has completed
    Completed,
}

/// State of prompt execution
#[derive(Debug, Clone, PartialEq, Default)]
pub enum ExecutionState {
    /// No prompt is being executed
    #[default]
    Idle,
    /// Waiting for async command result
    WaitingForResult,
}

/// Request to send to an AI provider
#[derive(Debug, Clone)]
pub struct PromptRequest {
    pub prompt: String,
    pub context_files: Vec<PathBuf>,
    pub session_id: Option<String>,
    pub working_directory: PathBuf,
}

/// Raw response from an AI provider
#[derive(Debug, Clone)]
pub struct ProviderResponse {
    pub raw_output: String,
    pub exit_code: i32,
    pub stderr: String,
}

/// Parsed file change from provider output
#[derive(Debug, Clone)]
pub struct FileChange {
    pub path: PathBuf,
    pub original_content: Option<String>,
    pub proposed_content: String,
    pub change_type: ChangeType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChangeType {
    Create,
    Modify,
    Delete,
}

/// A diff hunk for review
#[derive(Debug, Clone)]
pub struct Hunk {
    pub id: usize,
    pub file_path: PathBuf,
    pub start_line: usize,
    pub end_line: usize,
    pub changes: Vec<LineChange>,
    pub status: HunkStatus,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HunkStatus {
    Pending,
    Accepted,
    Rejected,
}

#[derive(Debug, Clone)]
pub struct LineChange {
    pub tag: ChangeTag,
    pub content: String,
    pub old_line_num: Option<usize>,
    pub new_line_num: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChangeTag {
    Equal,
    Insert,
    Delete,
}

/// A proposed change to a file (NOT yet applied)
#[derive(Debug, Clone)]
pub struct ProposedChange {
    pub id: usize,
    pub file_path: PathBuf,
    pub original_content: String, // What's currently on disk
    pub proposed_content: String, // What AI suggests
    pub line_decorations: Vec<LineDecoration>,
    pub status: ChangeStatus,
}

/// Visual decoration for a single line
#[derive(Debug, Clone)]
pub struct LineDecoration {
    pub line_number: usize,
    pub decoration_type: DecorationType,
    pub original_text: Option<String>, // For deletions/modifications
    pub new_text: Option<String>,      // For additions/modifications
    pub accepted: Option<bool>, // None = pending, Some(true) = accepted, Some(false) = rejected
}

#[derive(Debug, Clone, PartialEq)]
pub enum DecorationType {
    Addition,     // New line (green background, virtual text)
    Deletion,     // Removed line (strikethrough, dimmed)
    Modification, // Changed line (strikethrough old + virtual text new)
    Context,      // Unchanged line (for context)
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChangeStatus {
    Pending,       // Waiting for user review
    PartialAccept, // Some lines accepted, some pending
    Accepted,      // All accepted, ready to apply
    Rejected,      // All rejected
    Applied,       // Written to disk
}

/// State for the overlay diff viewer
pub struct OverlayDiffState {
    pub proposed_changes: Vec<ProposedChange>,
    pub current_change_idx: usize,
    pub current_line_idx: usize,
    pub show_context_lines: usize,
    pub folded_unchanged: bool, // Collapse unchanged regions
}

impl Default for OverlayDiffState {
    fn default() -> Self {
        Self {
            proposed_changes: Vec::new(),
            current_change_idx: 0,
            current_line_idx: 0,
            show_context_lines: 3,
            folded_unchanged: false,
        }
    }
}

/// Status information for real-time feedback
pub struct StatusInfo {
    pub is_working: bool,
    pub current_task: String,
    pub progress_percent: Option<u8>,
    pub tokens_sent: usize,
    pub tokens_received: usize,
    pub cost_estimate: f64,
    pub total_cost: f64,
    pub session_cost: f64,
    pub provider: String,
    pub model: String,
    pub eta_seconds: Option<u64>,
    pub can_cancel: bool,
    pub start_time: Option<Instant>,
}

impl Default for StatusInfo {
    fn default() -> Self {
        Self {
            is_working: false,
            current_task: String::new(),
            progress_percent: None,
            tokens_sent: 0,
            tokens_received: 0,
            cost_estimate: 0.0,
            total_cost: 0.0,
            session_cost: 0.0,
            provider: String::new(),
            model: String::new(),
            eta_seconds: None,
            can_cancel: false,
            start_time: None,
        }
    }
}

/// Sidebar state for file preview
pub struct SidebarState {
    pub visible: bool,
    pub pinned_file: Option<PathBuf>,
    pub scroll_offset: usize,
    pub highlighted_lines: Vec<usize>,
    pub syntax_highlighting: bool,
    pub current_file_indicator: Option<String>,
}

impl Default for SidebarState {
    fn default() -> Self {
        Self {
            visible: false,
            pinned_file: None,
            scroll_offset: 0,
            highlighted_lines: Vec::new(),
            syntax_highlighting: true,
            current_file_indicator: None,
        }
    }
}

/// UI display preferences
pub struct UIPreferences {
    /// Whether to show the chat history panel
    pub show_chat_history: bool,
    /// Whether to show the sidebar
    pub show_sidebar: bool,
}

impl Default for UIPreferences {
    fn default() -> Self {
        Self {
            // Show chat history by default so the conversation is front and center.
            show_chat_history: true,
            show_sidebar: false,
        }
    }
}

/// A chat message in the conversation history
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: usize,
    pub timestamp: DateTime<Utc>,
    pub is_user: bool,
    pub content: String,
    pub token_count: Option<usize>,
    pub cost: Option<f64>,
    pub status: MessageStatus,
    pub associated_files: Vec<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MessageStatus {
    Success,
    Error,
    Pending,
    Working,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessageFilter {
    Error,
    Success,
    All,
}

/// Chat history with navigation state
pub struct ChatHistory {
    pub messages: Vec<ChatMessage>,
    pub next_id: usize,
    pub scroll_state: ListState,
    pub search_query: Option<String>,
    pub filter: Option<MessageFilter>,
}

impl Default for ChatHistory {
    fn default() -> Self {
        Self {
            messages: Vec::new(),
            next_id: 1,
            scroll_state: ListState::default(),
            search_query: None,
            filter: None,
        }
    }
}

impl ChatHistory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_message(&mut self, message: ChatMessage) {
        self.messages.push(message);
    }

    pub fn get_message(&self, id: usize) -> Option<&ChatMessage> {
        self.messages.iter().find(|m| m.id == id)
    }

    pub fn filtered_messages(&self) -> Vec<&ChatMessage> {
        let mut filtered: Vec<&ChatMessage> = self.messages.iter().collect();

        // Apply filter
        if let Some(ref filter) = self.filter {
            filtered = filtered
                .into_iter()
                .filter(|msg| match filter {
                    MessageFilter::Error => msg.status == MessageStatus::Error,
                    MessageFilter::Success => msg.status == MessageStatus::Success,
                    MessageFilter::All => true,
                })
                .collect();
        }

        // Apply search query
        if let Some(ref query) = self.search_query {
            let query_lower = query.to_lowercase();
            filtered = filtered
                .into_iter()
                .filter(|msg| msg.content.to_lowercase().contains(&query_lower))
                .collect();
        }

        filtered
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    ProviderSelect,
    PromptEntry,
    Processing,
    DiffReview,
    Confirmation,
    Error,
    ChatHistory,
    CommandMode,
    Help,
}

#[derive(Debug, Clone)]
pub struct ProviderInfo {
    pub name: String,
    pub available: bool,
    pub cli_command: String,
    /// Config key for looking up provider config (e.g., "claude", "q", or custom key)
    pub config_key: String,
}

/// Main plugin state
pub struct State {
    // Provider management
    pub provider: Option<Box<dyn AIProvider>>,
    pub available_providers: Vec<ProviderInfo>,
    pub selected_provider_idx: usize,
    pub pending_detections: HashSet<String>,
    pub detection_state: DetectionState,
    pub execution_state: ExecutionState,

    // UI state
    pub mode: Mode,
    pub hunks: Vec<Hunk>,
    pub selected_hunk: usize,
    pub scroll_offset: usize,
    pub viewport_rows: usize,
    pub viewport_cols: usize,

    // Input
    pub prompt_buffer: String,
    pub cursor_position: usize,
    pub command_buffer: String,

    // Session management
    pub sessions: SessionManager,

    // Pending changes
    pub pending_changes: HashMap<PathBuf, FileChange>,

    // Error handling
    pub last_error: Option<ErrorDisplay>,

    // Permissions
    pub permissions_granted: bool,

    // Configuration
    pub config: Config,

    // File operations result
    pub last_apply_result: Option<crate::file_ops::ApplyResult>,

    // Chat history
    pub chat_history: ChatHistory,

    // Overlay diff state
    pub overlay_diff_state: OverlayDiffState,

    // Status tracking
    pub status_info: StatusInfo,

    // Sidebar state
    pub sidebar_state: SidebarState,

    // UI preferences
    pub ui_prefs: UIPreferences,
}

impl Default for State {
    fn default() -> Self {
        Self {
            provider: None,
            available_providers: Vec::new(),
            selected_provider_idx: 0,
            pending_detections: HashSet::new(),
            detection_state: DetectionState::default(),
            execution_state: ExecutionState::default(),
            mode: Mode::ProviderSelect,
            hunks: Vec::new(),
            selected_hunk: 0,
            scroll_offset: 0,
            viewport_rows: 24,
            viewport_cols: 80,
            prompt_buffer: String::new(),
            cursor_position: 0,
            command_buffer: String::new(),
            sessions: SessionManager::default(),
            pending_changes: HashMap::new(),
            last_error: None,
            permissions_granted: false,
            config: Config::default(),
            last_apply_result: None,
            chat_history: ChatHistory::new(),
            overlay_diff_state: OverlayDiffState::default(),
            status_info: StatusInfo::default(),
            sidebar_state: SidebarState::default(),
            ui_prefs: UIPreferences::default(),
        }
    }
}

impl State {
    pub fn initialize(&mut self, _configuration: &BTreeMap<String, String>) -> anyhow::Result<()> {
        // Load configuration with error reporting
        self.config = match Config::load() {
            Ok(config) => config,
            Err(e) => {
                self.last_error = Some(ErrorDisplay {
                    title: "Config Warning".to_string(),
                    message: format!("Using defaults: {}", e),
                    help_url: None,
                });
                Config::default()
            }
        };

        // Load sessions with error reporting
        self.sessions = match SessionManager::load() {
            Ok(sessions) => sessions,
            Err(e) => {
                // Don't overwrite config error, just log to stderr if we can
                if self.last_error.is_none() {
                    self.last_error = Some(ErrorDisplay {
                        title: "Session Warning".to_string(),
                        message: format!("Using defaults: {}", e),
                        help_url: None,
                    });
                }
                SessionManager::default()
            }
        };

        Ok(())
    }

    // Provider detection is now handled by App struct

    // Command result handling is now in App struct

    /// Apply accepted hunks to files
    pub fn apply_changes(&mut self) -> anyhow::Result<crate::file_ops::ApplyResult> {
        let accepted_hunks: Vec<_> = self
            .hunks
            .iter()
            .filter(|h| h.status == HunkStatus::Accepted)
            .collect();

        if accepted_hunks.is_empty() {
            return Err(anyhow::anyhow!("No accepted hunks to apply"));
        }

        crate::file_ops::apply_accepted_hunks(&accepted_hunks, &self.pending_changes, &self.config)
    }

    // Prompt execution is now handled by App struct

    // Key handling is now in App struct
    #[allow(dead_code)]
    pub fn handle_key_legacy(&mut self, key: &crossterm::event::KeyEvent) -> bool {
        use crate::input::modes::*;
        use crate::input::{Action, InputHandler, InputResult};

        let result = match self.mode {
            Mode::ProviderSelect => {
                let mut handler = ProviderSelectHandler::new();
                handler.handle_key(key, self)
            }
            Mode::PromptEntry => {
                let mut handler = PromptEntryHandler::new();
                handler.handle_key(key, self)
            }
            Mode::DiffReview => {
                let mut handler = DiffReviewHandler::new();
                handler.handle_key(key, self)
            }
            Mode::Confirmation => {
                let mut handler = ConfirmationHandler::new();
                handler.handle_key(key, self)
            }
            Mode::ChatHistory => {
                // For now, handle like PromptEntry
                let mut handler = PromptEntryHandler::new();
                handler.handle_key(key, self)
            }
            Mode::CommandMode => {
                // Special handling for command mode - consume the key
                InputResult::Consumed
            }
            Mode::Help => {
                // Press any key to close help - return to PromptEntry
                self.mode = Mode::PromptEntry;
                return true;
            }
            Mode::Error => {
                self.last_error = None;
                self.mode = Mode::PromptEntry;
                return true;
            }
            Mode::Processing => return false,
        };

        // Handle the result
        match result {
            InputResult::Consumed => true,
            InputResult::Ignored => false,
            InputResult::ModeChange(new_mode) => {
                self.mode = new_mode;
                true
            }
            InputResult::Action(action) => self.handle_action(&action),
        }
    }

    fn handle_action(&mut self, action: &crate::input::Action) -> bool {
        use crate::input::modes::DiffReviewHandler;
        use crate::input::Action;

        match action {
            Action::Next
            | Action::Previous
            | Action::AcceptCurrent
            | Action::RejectCurrent
            | Action::AcceptAll
            | Action::RejectAll
            | Action::Beginning
            | Action::End
            | Action::ScrollUp
            | Action::ScrollDown
            | Action::PageUp
            | Action::PageDown
            | Action::ToggleLineNumbers => DiffReviewHandler::apply_action(action, self),
            Action::NextFile => {
                // TODO: Implement multi-file navigation
                false
            }
            Action::PreviousFile => {
                // TODO: Implement multi-file navigation
                false
            }
            Action::ApplyChanges => {
                self.mode = Mode::Confirmation;
                true
            }
            Action::SelectProvider(idx) => {
                if *idx < self.available_providers.len() {
                    let provider_info = &self.available_providers[*idx];
                    let config = self.config.providers.get(&provider_info.config_key);
                    self.provider = crate::providers::create_provider(&provider_info.name, config);
                    self.selected_provider_idx = *idx;
                    self.mode = Mode::PromptEntry;
                    true
                } else {
                    false
                }
            }
            Action::SubmitPrompt(text) => {
                if text.is_empty() {
                    self.last_error = Some(crate::error::ErrorDisplay {
                        title: "Empty Prompt".to_string(),
                        message: "Please enter a prompt text".to_string(),
                        help_url: None,
                    });
                    self.mode = Mode::Error;
                    true
                } else {
                    // Prompt execution is now handled by App struct
                    true
                }
            }
            Action::Confirm => match self.apply_changes() {
                Ok(result) => {
                    self.last_apply_result = Some(result);
                    self.mode = Mode::DiffReview;
                    self.hunks.clear();
                    self.pending_changes.clear();
                    self.last_error = Some(crate::error::ErrorDisplay {
                        title: "Changes Applied Successfully".to_string(),
                        message: "Files have been updated with the accepted hunks".to_string(),
                        help_url: None,
                    });
                    true
                }
                Err(e) => {
                    self.last_error = Some(crate::error::ErrorDisplay {
                        title: "Failed to Apply Changes".to_string(),
                        message: e.to_string(),
                        help_url: None,
                    });
                    self.mode = Mode::Error;
                    true
                }
            },
            Action::Deny => {
                self.mode = Mode::DiffReview;
                true
            }
            Action::Quit => {
                // TODO: Graceful shutdown
                true
            }
        }
    }
}
