// src/providers/custom.rs - Custom/user-configurable provider

use super::{AIProvider, ParserType};
use crate::config::ProviderConfig;
use crate::parsers::{parse_code_blocks, parse_unified_diff};
use crate::state::{FileChange, PromptRequest};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct CustomProvider {
    /// Display name for the provider
    pub display_name: String,
    /// CLI command or path to execute
    pub command: String,
    /// Template for command arguments. Use {prompt} as placeholder.
    pub args_template: Vec<String>,
    /// Parser type to use for output
    pub parser: ParserType,
}

impl CustomProvider {
    /// Create a custom provider from configuration
    ///
    /// # Arguments
    /// * `name` - The config key name (used as fallback display name)
    /// * `path` - The CLI command or path
    /// * `config` - Additional configuration options
    pub fn from_config(name: &str, path: &str, config: &ProviderConfig) -> Self {
        let display_name = config.name.clone().unwrap_or_else(|| name.to_string());

        let args_template = config
            .args_template
            .clone()
            .unwrap_or_else(|| vec!["{prompt}".to_string()]);

        let parser = match config.parser.as_deref() {
            Some("unified_diff") | Some("diff") => ParserType::UnifiedDiff,
            Some("json") | Some("claude_json") => ParserType::ClaudeJson,
            _ => ParserType::CodeBlocks, // Default to code blocks
        };

        Self {
            display_name,
            command: path.to_string(),
            args_template,
            parser,
        }
    }
}

impl AIProvider for CustomProvider {
    fn name(&self) -> &str {
        &self.display_name
    }

    fn cli_command(&self) -> &str {
        &self.command
    }

    fn build_execute_args(&self, request: &PromptRequest) -> Vec<String> {
        self.args_template
            .iter()
            .map(|arg| {
                if arg.contains("{prompt}") {
                    arg.replace("{prompt}", &request.prompt)
                } else {
                    arg.clone()
                }
            })
            .collect()
    }

    fn parse_file_changes(&self, output: &str) -> Result<Vec<FileChange>> {
        match self.parser {
            ParserType::UnifiedDiff => parse_unified_diff(output),
            ParserType::CodeBlocks => parse_code_blocks(output),
            ParserType::ClaudeJson => {
                // For JSON, try to parse as Claude JSON format
                crate::parsers::parse_claude_json(output)
            }
        }
    }

    fn parser_type(&self) -> ParserType {
        self.parser.clone()
    }

    fn supports_sessions(&self) -> bool {
        false
    }
}
