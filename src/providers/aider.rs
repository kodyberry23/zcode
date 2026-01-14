// src/providers/aider.rs - Aider provider implementation

use anyhow::{Context, Result};
use std::process::Command;

use super::AIProvider;
use crate::parsers::parse_unified_diff;
use crate::state::{FileChange, PromptRequest, ProviderResponse};

#[derive(Debug, Clone)]
pub struct AiderProvider {
    pub model: String,
    pub edit_format: String,
}

impl Default for AiderProvider {
    fn default() -> Self {
        Self {
            model: "gpt-4".to_string(),
            edit_format: "diff".to_string(),
        }
    }
}

impl AIProvider for AiderProvider {
    fn name(&self) -> &str {
        "Aider"
    }

    fn cli_command(&self) -> &str {
        "aider"
    }

    fn execute(&self, request: &PromptRequest) -> Result<ProviderResponse> {
        let mut cmd = Command::new("aider");
        cmd.args([
            "--model",
            &self.model,
            "--edit-format",
            &self.edit_format,
            "--yes",    // Auto-confirm
            "--no-git", // Don't auto-commit
            "--message",
            &request.prompt,
        ]);

        // Add files to context
        for file in &request.context_files {
            cmd.arg(file);
        }

        let output = cmd.output().context("Failed to execute Aider CLI")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Aider CLI failed: {}", stderr);
        }

        Ok(ProviderResponse {
            raw_output: String::from_utf8_lossy(&output.stdout).to_string(),
            exit_code: output.status.code().unwrap_or(-1),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }

    fn parse_file_changes(&self, output: &str) -> Result<Vec<FileChange>> {
        parse_unified_diff(output)
    }

    fn supports_sessions(&self) -> bool {
        false
    }

    fn is_available(&self) -> bool {
        Command::new("aider")
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}
