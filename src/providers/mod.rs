//! AI provider abstraction and implementations
//!
//! This module defines the `AIProvider` trait that all AI tools must implement,
//! along with concrete implementations for popular tools:
//!
//! - **Claude**: Anthropic's Claude AI via the official CLI
//! - **Aider**: The Aider AI code assistant
//! - **Copilot**: GitHub Copilot CLI
//! - **Kiro**: AWS's Kiro CLI (formerly Amazon Q Developer)
//! - **Custom**: Custom parsers for new tools
//!
//! # Command Execution
//!
//! Providers build command arguments via `build_execute_args()` which are then
//! executed asynchronously using `tokio::process::Command` by the application's
//! executor module.
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

use crate::config::ProviderConfig;
use crate::state::{FileChange, PromptRequest};

/// Parser type for interpreting provider output
#[derive(Debug, Clone, PartialEq)]
pub enum ParserType {
    /// Claude's JSON output format
    ClaudeJson,
    /// Standard unified diff format (used by Aider)
    UnifiedDiff,
    /// Markdown code blocks (used by Copilot, Kiro)
    CodeBlocks,
}

/// Core trait that all AI providers must implement.
///
/// Providers specify command arguments via `build_execute_args()` which are then
/// executed asynchronously by the application's executor module.
pub trait AIProvider: Send + Sync {
    /// Human-readable name for the provider
    fn name(&self) -> &str;

    /// Get the CLI command name
    fn cli_command(&self) -> &str;

    /// Build command arguments for execution
    ///
    /// Returns a vector of arguments (NOT including the command itself).
    /// The executor will prepend the cli_command() when executing.
    fn build_execute_args(&self, request: &PromptRequest) -> Vec<String>;

    /// Parse file changes from provider output (stdout)
    fn parse_file_changes(&self, output: &str) -> Result<Vec<FileChange>>;

    /// Get the parser type for this provider
    fn parser_type(&self) -> ParserType;

    /// Whether this provider supports session continuity
    fn supports_sessions(&self) -> bool;

    /// Extract session ID from stdout (if supported)
    fn extract_session_id(&self, _stdout: &str) -> Option<String> {
        None
    }
}

/// Factory function to create a provider by name
///
/// For built-in providers, config is optional. For custom providers,
/// config must be provided with at least a `path` specified.
pub fn create_provider(name: &str, config: Option<&ProviderConfig>) -> Option<Box<dyn AIProvider>> {
    match name.to_lowercase().as_str() {
        "claude" | "claude code" => Some(Box::new(claude::ClaudeProvider::new(config))),
        "aider" => Some(Box::new(aider::AiderProvider::new(config))),
        "copilot" | "github copilot" | "github copilot cli" => {
            Some(Box::new(copilot::CopilotProvider::new(config)))
        }
        "amazon q" | "amazon q developer" | "q" | "kiro" | "kiro cli" => {
            Some(Box::new(amazon_q::AmazonQProvider::new(config)))
        }
        _ => {
            // Try to create custom provider from config
            config.and_then(|c| {
                c.path.as_ref().map(|path| {
                    Box::new(custom::CustomProvider::from_config(name, path, c))
                        as Box<dyn AIProvider>
                })
            })
        }
    }
}
