// src/providers/aider.rs - Aider provider implementation

use anyhow::Result;

use super::{AIProvider, ParserType};
use crate::config::ProviderConfig;
use crate::parsers::parse_unified_diff;
use crate::state::{FileChange, PromptRequest};

#[derive(Debug, Clone)]
pub struct AiderProvider {
    pub model: String,
    pub edit_format: String,
    /// Custom CLI path (if specified in config)
    pub cli_path: Option<String>,
}

impl Default for AiderProvider {
    fn default() -> Self {
        Self {
            model: "gpt-4".to_string(),
            edit_format: "diff".to_string(),
            cli_path: None,
        }
    }
}

impl AiderProvider {
    pub fn new(config: Option<&ProviderConfig>) -> Self {
        Self {
            model: "gpt-4".to_string(),
            edit_format: "diff".to_string(),
            cli_path: config.and_then(|c| c.path.clone()),
        }
    }
}

impl AIProvider for AiderProvider {
    fn name(&self) -> &str {
        "Aider"
    }

    fn cli_command(&self) -> &str {
        self.cli_path.as_deref().unwrap_or("aider")
    }

    fn build_execute_args(&self, request: &PromptRequest) -> Vec<String> {
        let mut args = vec![
            "--model".to_string(),
            self.model.clone(),
            "--edit-format".to_string(),
            self.edit_format.clone(),
            "--yes".to_string(),    // Auto-confirm
            "--no-git".to_string(), // Don't auto-commit
            "--message".to_string(),
            request.prompt.clone(),
        ];

        // Add files to context
        for file in &request.context_files {
            args.push(file.to_string_lossy().to_string());
        }

        args
    }

    fn parse_file_changes(&self, output: &str) -> Result<Vec<FileChange>> {
        parse_unified_diff(output)
    }

    fn parser_type(&self) -> ParserType {
        ParserType::UnifiedDiff
    }

    fn supports_sessions(&self) -> bool {
        false
    }
}
