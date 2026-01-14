//! Main plugin state and data structures
//!
//! This module defines the core data types and state machine for the ZCode plugin.
//! It manages the application's modes, user interactions, and the flow between
//! prompting, diff review, and file application.
use std::collections::HashMap;
use std::path::PathBuf;

use crate::config::Config;
use crate::error::ErrorDisplay;
use crate::providers::AIProvider;
use crate::session::SessionManager;

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

#[derive(Debug, Clone, PartialEq)]
pub enum Mode {
    ProviderSelect,
    PromptEntry,
    Processing,
    DiffReview,
    Confirmation,
    Error,
}

#[derive(Debug, Clone)]
pub struct ProviderInfo {
    pub name: String,
    pub available: bool,
    pub cli_command: String,
}

/// Main plugin state
pub struct State {
    // Provider management
    pub provider: Option<Box<dyn AIProvider>>,
    pub available_providers: Vec<ProviderInfo>,
    pub selected_provider_idx: usize,

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
}

impl Default for State {
    fn default() -> Self {
        Self {
            provider: None,
            available_providers: Vec::new(),
            selected_provider_idx: 0,
            mode: Mode::ProviderSelect,
            hunks: Vec::new(),
            selected_hunk: 0,
            scroll_offset: 0,
            viewport_rows: 24,
            viewport_cols: 80,
            prompt_buffer: String::new(),
            cursor_position: 0,
            sessions: SessionManager::default(),
            pending_changes: HashMap::new(),
            last_error: None,
            permissions_granted: false,
            config: Config::default(),
            last_apply_result: None,
        }
    }
}

impl State {
    pub fn initialize(
        &mut self,
        _configuration: &std::collections::BTreeMap<String, String>,
    ) -> anyhow::Result<()> {
        // Load configuration
        self.config = Config::load().unwrap_or_default();

        // Load sessions
        self.sessions = SessionManager::load().unwrap_or_default();

        // Detect available providers
        self.detect_available_providers();

        Ok(())
    }

    fn detect_available_providers(&mut self) {
        self.available_providers.clear();

        // Check for Claude
        if is_cli_available("claude") {
            self.available_providers.push(ProviderInfo {
                name: "Claude Code".to_string(),
                available: true,
                cli_command: "claude".to_string(),
            });
        }

        // Check for Aider
        if is_cli_available("aider") {
            self.available_providers.push(ProviderInfo {
                name: "Aider".to_string(),
                available: true,
                cli_command: "aider".to_string(),
            });
        }

        // Check for GitHub Copilot
        if is_cli_available_with_args("gh", &["copilot", "--help"]) {
            self.available_providers.push(ProviderInfo {
                name: "GitHub Copilot".to_string(),
                available: true,
                cli_command: "gh copilot".to_string(),
            });
        }

        // Check for Amazon Q
        if is_cli_available("q") {
            self.available_providers.push(ProviderInfo {
                name: "Amazon Q Developer".to_string(),
                available: true,
                cli_command: "q".to_string(),
            });
        }
    }

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

    pub fn handle_key(&mut self, key: &zellij_tile::prelude::KeyWithModifier) -> bool {
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
                    // TODO: Send prompt to AI provider
                    self.mode = Mode::Processing;
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

fn is_cli_available(cmd: &str) -> bool {
    std::process::Command::new(cmd)
        .arg("--version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn is_cli_available_with_args(cmd: &str, args: &[&str]) -> bool {
    std::process::Command::new(cmd)
        .args(args)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}
