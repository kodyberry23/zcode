// src/providers/claude.rs - Claude Code provider implementation

use anyhow::{Context, Result};
use std::process::Command;

use super::AIProvider;
use crate::parsers::parse_claude_json;
use crate::state::{FileChange, PromptRequest, ProviderResponse};

#[derive(Debug, Default, Clone)]
pub struct ClaudeProvider {
    pub session_id: Option<String>,
}

impl AIProvider for ClaudeProvider {
    fn name(&self) -> &str {
        "Claude Code"
    }

    fn cli_command(&self) -> &str {
        "claude"
    }

    fn execute(&self, request: &PromptRequest) -> Result<ProviderResponse> {
        let mut cmd = Command::new("claude");
        cmd.args(["-p", &request.prompt, "--output-format", "json"]);

        if let Some(ref session) = self.session_id {
            cmd.args(["--resume", session]);
        }

        cmd.args(["--allowedTools", "Read,Edit,Write"]);

        let output = cmd.output().context("Failed to execute Claude CLI")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("Claude CLI failed: {}", stderr);
        }

        Ok(ProviderResponse {
            raw_output: String::from_utf8_lossy(&output.stdout).to_string(),
            exit_code: output.status.code().unwrap_or(-1),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        })
    }

    fn parse_file_changes(&self, output: &str) -> Result<Vec<FileChange>> {
        parse_claude_json(output)
    }

    fn supports_sessions(&self) -> bool {
        true
    }

    fn extract_session_id(&self, response: &ProviderResponse) -> Option<String> {
        // Parse session ID from Claude's JSON response
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&response.raw_output) {
            json.get("session_id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        } else {
            None
        }
    }

    fn is_available(&self) -> bool {
        Command::new("claude")
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}
