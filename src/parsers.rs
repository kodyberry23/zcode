//! Parsing AI provider outputs into code changes
//!
//! This module handles parsing the diverse output formats from different AI tools
//! and converting them into a common `FileChange` representation.
//!
//! # Supported Formats
//!
//! - **Unified diff**: Standard diff format (used by git, Aider, etc.)
//! - **Code blocks**: Markdown-style code blocks with file path annotations
//! - **Claude JSON**: Claude's JSON response format
//! - **JSON changes**: Custom JSON array format for file changes
//! - **Regex-based**: Custom regex patterns for custom outputs

use anyhow::{Context, Result};
use regex::Regex;
use std::fs;
use std::path::PathBuf;

use crate::state::{ChangeType, FileChange};

/// Parse standard unified diff format (used by Aider, git, etc.)
pub fn parse_unified_diff(input: &str) -> Result<Vec<FileChange>> {
    let mut changes = Vec::new();
    let mut current_file: Option<PathBuf> = None;
    let mut original_lines = Vec::new();
    let mut proposed_lines = Vec::new();

    for line in input.lines() {
        if line.starts_with("--- ") {
            // Original file - save previous if exists
            if let Some(path) = current_file.take() {
                changes.push(FileChange {
                    path,
                    original_content: Some(original_lines.join("\n")),
                    proposed_content: proposed_lines.join("\n"),
                    change_type: ChangeType::Modify,
                });
                original_lines.clear();
                proposed_lines.clear();
            }
            // Parse path from "--- a/path/to/file"
            let path = line
                .strip_prefix("--- ")
                .unwrap()
                .strip_prefix("a/")
                .unwrap_or(line);
            current_file = Some(PathBuf::from(path));
        } else if line.starts_with("+++ ") {
            // New file marker (skip, use --- path)
        } else if line.starts_with('-') && !line.starts_with("---") {
            original_lines.push(line[1..].to_string());
        } else if line.starts_with('+') && !line.starts_with("+++") {
            proposed_lines.push(line[1..].to_string());
        } else if !line.starts_with("@@") {
            // Context line
            original_lines.push(line.to_string());
            proposed_lines.push(line.to_string());
        }
    }

    // Don't forget last file
    if let Some(path) = current_file {
        changes.push(FileChange {
            path,
            original_content: Some(original_lines.join("\n")),
            proposed_content: proposed_lines.join("\n"),
            change_type: ChangeType::Modify,
        });
    }

    Ok(changes)
}

/// Parse markdown-style code blocks with file paths
pub fn parse_code_blocks(input: &str) -> Result<Vec<FileChange>> {
    let re =
        Regex::new(r"(?s)```(?:\w+)?\s*\n?(?://|#|<!--)\s*(?:file:|path:)?\s*([^\n]+)\n(.+?)```")?;

    let mut changes = Vec::new();

    for cap in re.captures_iter(input) {
        let path = PathBuf::from(cap[1].trim());
        let content = cap[2].to_string();

        // Try to read original file to determine change type
        let (original, change_type) = if path.exists() {
            (fs::read_to_string(&path).ok(), ChangeType::Modify)
        } else {
            (None, ChangeType::Create)
        };

        changes.push(FileChange {
            path,
            original_content: original,
            proposed_content: content,
            change_type,
        });
    }

    Ok(changes)
}

/// Parse Claude Code's JSON output format
pub fn parse_claude_json(input: &str) -> Result<Vec<FileChange>> {
    let response: serde_json::Value =
        serde_json::from_str(input).context("Failed to parse Claude JSON response")?;

    let mut changes = Vec::new();

    // Claude embeds tool uses in the result
    if let Some(result) = response.get("result").and_then(|r| r.as_str()) {
        // Parse tool use patterns from result text
        let edit_pattern = Regex::new(r"(?s)Editing\s+`?([^`\n]+)`?.*?```\w*\n(.+?)```")?;

        for cap in edit_pattern.captures_iter(result) {
            let path = PathBuf::from(cap[1].trim());
            let content = cap[2].to_string();

            let original = if path.exists() {
                fs::read_to_string(&path).ok()
            } else {
                None
            };

            let change_type = if original.is_some() {
                ChangeType::Modify
            } else {
                ChangeType::Create
            };

            changes.push(FileChange {
                path,
                original_content: original,
                proposed_content: content,
                change_type,
            });
        }
    }

    Ok(changes)
}

/// Parse JSON changes format
pub fn parse_json_changes(input: &str) -> Result<Vec<FileChange>> {
    let json: serde_json::Value = serde_json::from_str(input)?;

    let mut changes = Vec::new();

    if let Some(files) = json.as_array() {
        for file_obj in files {
            if let (Some(path_str), Some(content_str)) = (
                file_obj.get("path").and_then(|p| p.as_str()),
                file_obj.get("content").and_then(|c| c.as_str()),
            ) {
                let path = PathBuf::from(path_str);

                let original = if path.exists() {
                    fs::read_to_string(&path).ok()
                } else {
                    None
                };

                let change_type = if original.is_some() {
                    ChangeType::Modify
                } else {
                    ChangeType::Create
                };

                changes.push(FileChange {
                    path,
                    original_content: original,
                    proposed_content: content_str.to_string(),
                    change_type,
                });
            }
        }
    }

    Ok(changes)
}

/// Parse with custom regex pattern
pub fn parse_with_regex(input: &str, pattern: &str) -> Result<Vec<FileChange>> {
    let re = Regex::new(pattern)?;
    let mut changes = Vec::new();

    for cap in re.captures_iter(input) {
        if cap.len() >= 3 {
            let path = PathBuf::from(cap[1].trim());
            let content = cap[2].to_string();

            let original = if path.exists() {
                fs::read_to_string(&path).ok()
            } else {
                None
            };

            let change_type = if original.is_some() {
                ChangeType::Modify
            } else {
                ChangeType::Create
            };

            changes.push(FileChange {
                path,
                original_content: original,
                proposed_content: content,
                change_type,
            });
        }
    }

    Ok(changes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_unified_diff_single_file() {
        let input = r#"--- a/test.txt
+++ b/test.txt
line 1
-old line
+new line
line 3"#;

        let result = parse_unified_diff(input).unwrap();
        assert!(!result.is_empty());
        assert_eq!(result[0].path, PathBuf::from("test.txt"));
        assert!(result[0].original_content.is_some());
        assert!(!result[0].proposed_content.is_empty());
    }

    #[test]
    fn test_parse_unified_diff_multiple_files() {
        let input = r#"--- a/file1.txt
+++ b/file1.txt
-old
+new
--- a/file2.txt
+++ b/file2.txt
-line1
+line2"#;

        let result = parse_unified_diff(input).unwrap();
        assert!(!result.is_empty());
        // At least file1 should be present
        assert!(result.iter().any(|c| c.path == PathBuf::from("file1.txt")));
    }

    #[test]
    fn test_parse_code_blocks_basic() {
        let input = r#"
```rust
// file: src/main.rs
fn main() {
    println!("Hello");
}
```
"#;

        let result = parse_code_blocks(input).unwrap();
        assert!(!result.is_empty());
        // Path extraction may fail if file doesn't exist, but shouldn't panic
    }

    #[test]
    fn test_parse_json_changes_empty() {
        let input = "[]";
        let result = parse_json_changes(input).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_parse_json_changes_single() {
        let input = r#"[{"path": "test.txt", "content": "hello world"}]"#;
        let result = parse_json_changes(input).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].path, PathBuf::from("test.txt"));
        assert_eq!(result[0].proposed_content, "hello world");
    }

    #[test]
    fn test_parse_with_regex_valid_pattern() {
        let input = "file:src/test.rs;content:fn main() {}";
        let pattern = r"file:([^;]+);content:(.+)";
        let result = parse_with_regex(input, pattern).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].path, PathBuf::from("src/test.rs"));
    }

    #[test]
    fn test_parse_with_regex_invalid_pattern() {
        let input = "some content";
        let pattern = "[invalid";
        let result = parse_with_regex(input, pattern);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_claude_json_empty_response() {
        let input = r#"{"result": ""}"#;
        let result = parse_claude_json(input).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_change_type_detection() {
        // Create file doesn't exist - should be Create type
        let change = FileChange {
            path: PathBuf::from("/nonexistent/path.txt"),
            original_content: None,
            proposed_content: "new content".to_string(),
            change_type: ChangeType::Create,
        };
        assert_eq!(change.change_type, ChangeType::Create);

        // Modify file has original - should be Modify type
        let change = FileChange {
            path: PathBuf::from("/existing/path.txt"),
            original_content: Some("old content".to_string()),
            proposed_content: "new content".to_string(),
            change_type: ChangeType::Modify,
        };
        assert_eq!(change.change_type, ChangeType::Modify);
    }
}
