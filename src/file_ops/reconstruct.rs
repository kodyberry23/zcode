// src/file_ops/reconstruct.rs - File content reconstruction from hunks

use crate::state::{ChangeTag, Hunk, HunkStatus};
use anyhow::{anyhow, Result};

/// Reconstruct file content by applying accepted hunks to original content
pub fn reconstruct_file_content(original: &str, hunks: &[&Hunk]) -> Result<String> {
    if hunks.is_empty() {
        return Ok(original.to_string());
    }

    let mut lines: Vec<String> = original.lines().map(|s| s.to_string()).collect();

    // Filter to only accepted hunks and sort by start_line in descending order
    // (so we apply from bottom to top to avoid line number shifting)
    let mut accepted_hunks: Vec<_> = hunks
        .iter()
        .filter(|h| h.status == HunkStatus::Accepted)
        .collect();

    accepted_hunks.sort_by(|a, b| b.start_line.cmp(&a.start_line));

    // Apply each hunk
    for hunk in accepted_hunks {
        apply_hunk(&mut lines, hunk)?;
    }

    Ok(lines.join("\n"))
}

/// Apply a single hunk to the lines
fn apply_hunk(lines: &mut Vec<String>, hunk: &Hunk) -> Result<()> {
    let mut insertions = Vec::new();
    let mut deletions = Vec::new();

    // Separate insertions and deletions
    for change in &hunk.changes {
        match change.tag {
            ChangeTag::Insert => insertions.push(change.content.clone()),
            ChangeTag::Delete => deletions.push(change.old_line_num.unwrap_or(0)),
            ChangeTag::Equal => {}
        }
    }

    // Apply deletions first (from high line numbers to low to avoid offset issues)
    for old_line_num in deletions.iter().rev() {
        if *old_line_num > 0 && *old_line_num <= lines.len() {
            lines.remove(old_line_num - 1);
        }
    }

    // Then apply insertions at the hunk's start line
    let insert_pos = if hunk.start_line > 0 {
        hunk.start_line - 1
    } else if lines.is_empty() {
        0
    } else {
        lines.len() // Append to end
    };

    // Clamp insert position to valid range
    let insert_pos = insert_pos.min(lines.len());

    for (idx, insertion) in insertions.iter().enumerate() {
        lines.insert(insert_pos + idx, insertion.clone());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ChangeTag, LineChange};

    fn create_test_hunk(
        start_line: usize,
        changes: Vec<(ChangeTag, String, Option<usize>, Option<usize>)>,
    ) -> Hunk {
        Hunk {
            id: 0,
            file_path: std::path::PathBuf::from("test.txt"),
            start_line,
            end_line: start_line + 5,
            changes: changes
                .into_iter()
                .map(|(tag, content, old_line, new_line)| LineChange {
                    tag,
                    content,
                    old_line_num: old_line,
                    new_line_num: new_line,
                })
                .collect(),
            status: HunkStatus::Accepted,
        }
    }

    #[test]
    fn test_no_hunks() {
        let original = "line 1\nline 2\nline 3";
        let result = reconstruct_file_content(original, &[]).unwrap();
        assert_eq!(result, original);
    }

    #[test]
    fn test_single_insertion() {
        let original = "line 1\nline 3";
        let hunk = create_test_hunk(
            2,
            vec![(ChangeTag::Insert, "line 2".to_string(), None, Some(2))],
        );

        let result = reconstruct_file_content(original, &[&hunk]).unwrap();
        assert_eq!(result, "line 1\nline 2\nline 3");
    }

    #[test]
    fn test_single_deletion() {
        let original = "line 1\nline 2\nline 3";
        let hunk = create_test_hunk(
            2,
            vec![(ChangeTag::Delete, "line 2".to_string(), Some(2), None)],
        );

        let result = reconstruct_file_content(original, &[&hunk]).unwrap();
        assert_eq!(result, "line 1\nline 3");
    }

    #[test]
    fn test_multiple_changes_in_hunk() {
        let original = "line 1\nline 3";
        let hunk = create_test_hunk(
            2,
            vec![
                (ChangeTag::Insert, "line 2a".to_string(), None, Some(2)),
                (ChangeTag::Insert, "line 2b".to_string(), None, Some(3)),
            ],
        );

        let result = reconstruct_file_content(original, &[&hunk]).unwrap();
        assert_eq!(result, "line 1\nline 2a\nline 2b\nline 3");
    }

    #[test]
    fn test_multiple_hunks() {
        // Start with 4 lines and remove one in the middle, then add two back in different places
        let original = "line 1\nline 2\nline 4\nline 5";

        // Hunk to insert "line 3" after "line 2"
        let hunk1 = create_test_hunk(
            3,
            vec![(ChangeTag::Insert, "line 3".to_string(), None, Some(3))],
        );

        let result = reconstruct_file_content(original, &[&hunk1]).unwrap();
        assert_eq!(result, "line 1\nline 2\nline 3\nline 4\nline 5");
    }

    #[test]
    fn test_empty_file_insertion() {
        let original = "";
        let hunk = create_test_hunk(
            0,
            vec![(ChangeTag::Insert, "line 1".to_string(), None, Some(1))],
        );

        let result = reconstruct_file_content(original, &[&hunk]).unwrap();
        assert_eq!(result, "line 1");
    }
}
