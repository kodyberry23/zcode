// src/providers/amazon_q.rs - Amazon Q Developer provider

use anyhow::{Context, Result};
use std::process::Command;

use super::AIProvider;
use crate::parsers::parse_code_blocks;
use crate::state::{FileChange, PromptRequest, ProviderResponse};

#[derive(Debug, Default, Clone)]
pub struct AmazonQProvider;

impl AIProvider for AmazonQProvider {
    fn name(&self) -> &str {
        "Amazon Q Developer"
    }

    fn cli_command(&self) -> &str {
        "q"
    }

    fn execute(&self, request: &PromptRequest) -> Result<ProviderResponse> {
        let mut cmd = Command::new("q");
        cmd.args(["chat", "--no-interactive", &request.prompt]);

        let output = cmd.output().context("Failed to execute Amazon Q CLI")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Amazon Q CLI failed: {}", stderr);
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
        true
    }

    fn is_available(&self) -> bool {
        Command::new("q")
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}
