// src/providers/copilot.rs - GitHub Copilot CLI provider

use anyhow::{Context, Result};
use std::process::Command;

use super::AIProvider;
use crate::parsers::parse_code_blocks;
use crate::state::{FileChange, PromptRequest, ProviderResponse};

#[derive(Debug, Default, Clone)]
pub struct CopilotProvider;

impl AIProvider for CopilotProvider {
    fn name(&self) -> &str {
        "GitHub Copilot"
    }

    fn cli_command(&self) -> &str {
        "gh copilot"
    }

    fn execute(&self, request: &PromptRequest) -> Result<ProviderResponse> {
        let mut cmd = Command::new("gh");
        cmd.args(["copilot", "suggest", "-t", "shell", &request.prompt]);

        let output = cmd
            .output()
            .context("Failed to execute GitHub Copilot CLI")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("GitHub Copilot CLI failed: {}", stderr);
        }

        Ok(ProviderResponse {
            raw_output: String::from_utf8_lossy(&output.stdout).to_string(),
            exit_code: output.status.code().unwrap_or(-1),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }

    fn parse_file_changes(&self, output: &str) -> Result<Vec<FileChange>> {
        parse_code_blocks(output)
    }

    fn supports_sessions(&self) -> bool {
        false
    }

    fn is_available(&self) -> bool {
        Command::new("gh")
            .args(["copilot", "--help"])
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}
