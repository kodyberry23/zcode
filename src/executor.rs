// src/executor.rs - Async command execution using tokio

use anyhow::Result;
use std::collections::BTreeMap;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

/// Result of a command execution
#[derive(Debug, Clone)]
pub struct CommandResult {
    pub exit_code: Option<i32>,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub context: BTreeMap<String, String>,
}

/// Execute a command asynchronously and return the result
pub async fn execute_command(
    command: &str,
    args: &[String],
    context: BTreeMap<String, String>,
) -> Result<CommandResult> {
    let mut child = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| {
            // Provide better context for command not found errors
            if e.kind() == std::io::ErrorKind::NotFound {
                anyhow::anyhow!("Command '{}' not found in PATH", command)
            } else {
                anyhow::anyhow!("Failed to execute '{}': {}", command, e)
            }
        })?;

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let stderr = child.stderr.take().expect("Failed to capture stderr");

    // Read stdout
    let stdout_handle = tokio::spawn(async move {
        let reader = BufReader::new(stdout);
        let mut lines = reader.lines();
        let mut output = Vec::new();
        while let Ok(Some(line)) = lines.next_line().await {
            output.extend_from_slice(line.as_bytes());
            output.push(b'\n');
        }
        output
    });

    // Read stderr
    let stderr_handle = tokio::spawn(async move {
        let reader = BufReader::new(stderr);
        let mut lines = reader.lines();
        let mut output = Vec::new();
        while let Ok(Some(line)) = lines.next_line().await {
            output.extend_from_slice(line.as_bytes());
            output.push(b'\n');
        }
        output
    });

    // Wait for process to complete
    let status = child.wait().await?;
    let stdout_bytes = stdout_handle.await?;
    let stderr_bytes = stderr_handle.await?;

    Ok(CommandResult {
        exit_code: status.code(),
        stdout: stdout_bytes,
        stderr: stderr_bytes,
        context,
    })
}

/// Execute provider detection command
pub async fn execute_provider_detection(
    command: &str,
    provider_id: &str,
    display_name: &str,
    config_key: &str,
) -> Result<CommandResult> {
    let mut context = BTreeMap::new();
    context.insert("provider_id".to_string(), provider_id.to_string());
    context.insert("display_name".to_string(), display_name.to_string());
    context.insert("cli_command".to_string(), command.to_string());
    context.insert("config_key".to_string(), config_key.to_string());

    execute_command(command, &["--version".to_string()], context).await
}

/// Execute AI provider prompt command
pub async fn execute_provider_prompt(
    command: &str,
    args: Vec<String>,
    provider_name: &str,
) -> Result<CommandResult> {
    let mut context = BTreeMap::new();
    context.insert("request_type".to_string(), "prompt_execution".to_string());
    context.insert("provider".to_string(), provider_name.to_string());

    execute_command(command, &args, context).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_execute_command() {
        let result = execute_command("echo", &["hello".to_string()], BTreeMap::new())
            .await
            .unwrap();

        assert_eq!(result.exit_code, Some(0));
        assert!(String::from_utf8_lossy(&result.stdout).contains("hello"));
    }

    #[tokio::test]
    async fn test_provider_detection() {
        // Test with a command that should exist
        let result = execute_provider_detection("echo", "test", "Test Provider", "test")
            .await
            .unwrap();

        assert_eq!(result.exit_code, Some(0));
        assert_eq!(result.context.get("provider_id").unwrap(), "test");
    }
}
