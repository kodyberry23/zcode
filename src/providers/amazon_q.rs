// src/providers/amazon_q.rs - Kiro CLI provider (formerly Amazon Q Developer)

use anyhow::Result;

use super::{AIProvider, ParserType};
use crate::config::ProviderConfig;
use crate::parsers::parse_code_blocks;
use crate::state::{FileChange, PromptRequest};

#[derive(Debug, Clone, Default)]
pub struct AmazonQProvider {
    /// Custom CLI path (if specified in config)
    pub cli_path: Option<String>,
}

impl AmazonQProvider {
    pub fn new(config: Option<&ProviderConfig>) -> Self {
        Self {
            cli_path: config.and_then(|c| c.path.clone()),
        }
    }
}

impl AIProvider for AmazonQProvider {
    fn name(&self) -> &str {
        "Kiro CLI"
    }

    fn cli_command(&self) -> &str {
        self.cli_path.as_deref().unwrap_or("kiro")
    }

    fn build_execute_args(&self, request: &PromptRequest) -> Vec<String> {
        vec![
            "chat".to_string(),
            "--no-interactive".to_string(),
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
        true
    }
}
