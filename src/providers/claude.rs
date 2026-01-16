// src/providers/claude.rs - Claude Code provider implementation

use anyhow::Result;

use super::{AIProvider, ParserType};
use crate::config::ProviderConfig;
use crate::parsers::parse_claude_json;
use crate::state::{FileChange, PromptRequest};

#[derive(Debug, Clone, Default)]
pub struct ClaudeProvider {
    pub session_id: Option<String>,
    /// Custom CLI path (if specified in config)
    pub cli_path: Option<String>,
}

impl ClaudeProvider {
    pub fn new(config: Option<&ProviderConfig>) -> Self {
        Self {
            session_id: None,
            cli_path: config.and_then(|c| c.path.clone()),
        }
    }

    pub fn with_session(mut self, session_id: String) -> Self {
        self.session_id = Some(session_id);
        self
    }
}

impl AIProvider for ClaudeProvider {
    fn name(&self) -> &str {
        "Claude Code"
    }

    fn cli_command(&self) -> &str {
        self.cli_path.as_deref().unwrap_or("claude")
    }

    fn build_execute_args(&self, request: &PromptRequest) -> Vec<String> {
        let mut args = vec![
            "-p".to_string(),
            request.prompt.clone(),
            "--output-format".to_string(),
            "json".to_string(),
            "--allowedTools".to_string(),
            "Read,Edit,Write".to_string(),
        ];

        if let Some(ref session) = request.session_id.as_ref().or(self.session_id.as_ref()) {
            args.push("--resume".to_string());
            args.push(session.to_string());
        }

        args
    }

    fn parse_file_changes(&self, output: &str) -> Result<Vec<FileChange>> {
        parse_claude_json(output)
    }

    fn parser_type(&self) -> ParserType {
        ParserType::ClaudeJson
    }

    fn supports_sessions(&self) -> bool {
        true
    }

    fn extract_session_id(&self, stdout: &str) -> Option<String> {
        // Parse session ID from Claude's JSON response
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(stdout) {
            json.get("session_id")
                .and_then(|v| v.as_str())
                .map(|s| s.to_string())
        } else {
            None
        }
    }
}
