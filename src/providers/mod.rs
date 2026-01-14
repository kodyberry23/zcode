//! AI provider abstraction and implementations
//!
//! This module defines the `AIProvider` trait that all AI tools must implement,
//! along with concrete implementations for popular tools:
//!
//! - **Claude**: Anthropic's Claude AI via the official CLI
//! - **Aider**: The Aider AI code assistant
//! - **Copilot**: GitHub Copilot CLI
//! - **Amazon Q**: AWS's AI assistant
//! - **Custom**: Custom parsers for new tools
//!
//! # Adding a New Provider
//!
//! To add support for a new AI tool:
//! 1. Create a new module (e.g., `custom.rs`)
//! 2. Implement the `AIProvider` trait
//! 3. Add it to the match statement in `create_provider()`

pub mod aider;
pub mod amazon_q;
pub mod claude;
pub mod copilot;
pub mod custom;

use anyhow::Result;

use crate::state::{FileChange, PromptRequest, ProviderResponse};

/// Core trait that all AI providers must implement
pub trait AIProvider: Send + Sync {
    /// Human-readable name for the provider
    fn name(&self) -> &str;

    /// Execute a prompt and return raw output
    fn execute(&self, request: &PromptRequest) -> Result<ProviderResponse>;

    /// Parse file changes from provider output
    fn parse_file_changes(&self, output: &str) -> Result<Vec<FileChange>>;

    /// Whether this provider supports session continuity
    fn supports_sessions(&self) -> bool;

    /// Get session ID from response (if supported)
    fn extract_session_id(&self, _response: &ProviderResponse) -> Option<String> {
        None
    }

    /// Check if the CLI tool is installed and accessible
    fn is_available(&self) -> bool;

    /// Get the CLI command name for error messages
    fn cli_command(&self) -> &str;
}

/// Factory function to create a provider by name
pub fn create_provider(name: &str) -> Option<Box<dyn AIProvider>> {
    match name.to_lowercase().as_str() {
        "claude" | "claude code" => Some(Box::new(claude::ClaudeProvider::default())),
        "aider" => Some(Box::new(aider::AiderProvider::default())),
        "copilot" | "github copilot" => Some(Box::new(copilot::CopilotProvider::default())),
        "amazon q" | "q" => Some(Box::new(amazon_q::AmazonQProvider::default())),
        _ => None,
    }
}
