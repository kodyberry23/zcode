// src/providers/copilot.rs - GitHub Copilot CLI provider

use anyhow::Result;

use super::{AIProvider, ParserType};
use crate::config::ProviderConfig;
use crate::parsers::parse_code_blocks;
use crate::state::{FileChange, PromptRequest};

#[derive(Debug, Clone, Default)]
pub struct CopilotProvider {
    /// Custom CLI path (if specified in config)
    pub cli_path: Option<String>,
}

impl CopilotProvider {
    pub fn new(config: Option<&ProviderConfig>) -> Self {
        Self {
            cli_path: config.and_then(|c| c.path.clone()),
        }
    }
}

impl AIProvider for CopilotProvider {
    fn name(&self) -> &str {
        "GitHub Copilot CLI"
    }

    fn cli_command(&self) -> &str {
        self.cli_path.as_deref().unwrap_or("copilot")
    }

    fn build_execute_args(&self, request: &PromptRequest) -> Vec<String> {
        vec![
            "suggest".to_string(),
            "-t".to_string(),
            "shell".to_string(),
            request.prompt.clone(),
        ]
    }

    fn parse_file_changes(&self, output: &str) -> Result<Vec<FileChange>> {
        parse_code_blocks(output)
    }

    fn parser_type(&self) -> ParserType {
        ParserType::CodeBlocks
    }

    fn supports_sessions(&self) -> bool {
        false
    }
}
