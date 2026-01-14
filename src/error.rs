// src/error.rs - Error types for ZCode

use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ZCodeError {
    #[error("Provider '{0}' is not installed")]
    ProviderNotInstalled(String),

    #[error("Provider '{0}' failed: {1}")]
    ProviderExecutionFailed(String, String),

    #[error("Failed to parse provider output: {0}")]
    ParseError(String),

    #[error("File operation failed for '{0}': {1}")]
    FileError(PathBuf, String),

    #[error("No changes detected in provider response")]
    NoChangesDetected,

    #[error("Permission denied: {0}")]
    PermissionDenied(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(String),
}

#[derive(Debug, Clone)]
pub struct ErrorDisplay {
    pub title: String,
    pub message: String,
    pub help_url: Option<String>,
}

pub fn get_install_url(provider: &str) -> String {
    match provider.to_lowercase().as_str() {
        "claude" | "claude code" => "https://claude.ai/code".to_string(),
        "aider" => "https://aider.chat/docs/install.html".to_string(),
        "copilot" | "github copilot" => "https://docs.github.com/copilot/using-github-copilot/using-github-copilot-in-the-command-line".to_string(),
        "amazon q" | "q" => "https://docs.aws.amazon.com/amazonq/latest/qdeveloper-ug/command-line.html".to_string(),
        _ => format!("https://www.google.com/search?q={} CLI install", provider),
    }
}
