// src/providers/custom.rs - Custom/user-configurable provider

use super::AIProvider;
use crate::state::{FileChange, PromptRequest, ProviderResponse};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct CustomProvider {
    pub name: String,
    pub command: String,
    pub args_template: Vec<String>,
}

impl AIProvider for CustomProvider {
    fn name(&self) -> &str {
        &self.name
    }

    fn cli_command(&self) -> &str {
        &self.command
    }

    fn execute(&self, _request: &PromptRequest) -> Result<ProviderResponse> {
        // Placeholder - to be implemented
        anyhow::bail!("Custom provider execution not yet implemented")
    }

    fn parse_file_changes(&self, _output: &str) -> Result<Vec<FileChange>> {
        // Placeholder - to be implemented
        anyhow::bail!("Custom provider parsing not yet implemented")
    }

    fn supports_sessions(&self) -> bool {
        false
    }

    fn is_available(&self) -> bool {
        false
    }
}
