//! Diff generation and hunk extraction
//!
//! This module handles generating diffs between original and proposed code,
//! then extracting them into discrete hunks that can be individually accepted or rejected.
//!
//! # Algorithm
//!
//! Uses the Patience algorithm for high-quality diffs that are similar to what git produces.
//! This algorithm is especially good for code because it identifies moving blocks efficiently.

use similar::{Algorithm, TextDiff};
use std::time::Duration;

use crate::state::{ChangeTag, Hunk, HunkStatus, LineChange};

/// Generate a diff between two texts
pub fn generate_diff<'a>(original: &'a str, proposed: &'a str) -> TextDiff<'a, 'a, 'a, str> {
    TextDiff::configure()
        .algorithm(Algorithm::Patience)
        .timeout(Duration::from_secs(5))
        .diff_lines(original, proposed)
}

/// Extract hunks from a diff
pub fn extract_hunks<'a>(
    file_path: &std::path::PathBuf,
    diff: &TextDiff<'a, 'a, 'a, str>,
) -> Vec<Hunk> {
    let mut hunks = Vec::new();

    for (hunk_idx, ops) in diff.grouped_ops(3).iter().enumerate() {
        let mut changes = Vec::new();
        let mut start_line = 0;
        let mut end_line = 0;

        for op in ops {
            for change in diff.iter_changes(op) {
                let tag = match change.tag() {
                    similar::ChangeTag::Delete => ChangeTag::Delete,
                    similar::ChangeTag::Insert => ChangeTag::Insert,
                    similar::ChangeTag::Equal => ChangeTag::Equal,
                };

                let old_line = change.old_index();
                let new_line = change.new_index();

                if old_line.is_some() {
                    start_line = old_line.unwrap();
                }
                if new_line.is_some() {
                    end_line = new_line.unwrap();
                }

                changes.push(LineChange {
                    tag,
                    content: change.value().to_string(),
                    old_line_num: old_line,
                    new_line_num: new_line,
                });
            }
        }

        if !changes.is_empty() {
            hunks.push(Hunk {
                id: hunk_idx,
                file_path: file_path.clone(),
                start_line,
                end_line,
                changes,
                status: HunkStatus::Pending,
            });
        }
    }

    hunks
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_generate_diff_identical() {
        let original = "line 1\nline 2\nline 3";
        let proposed = "line 1\nline 2\nline 3";

        let diff = generate_diff(original, proposed);
        let hunks = extract_hunks(&PathBuf::from("test.txt"), &diff);

        // Identical content should produce no hunks
        assert_eq!(hunks.len(), 0);
    }

    #[test]
    fn test_generate_diff_single_change() {
        let original = "line 1\nline 2\nline 3";
        let proposed = "line 1\nmodified line\nline 3";

        let diff = generate_diff(original, proposed);
        let hunks = extract_hunks(&PathBuf::from("test.txt"), &diff);

        assert!(!hunks.is_empty());
        // Should have changes (at least insert and/or delete)
        let has_changes = hunks[0]
            .changes
            .iter()
            .any(|c| c.tag == ChangeTag::Insert || c.tag == ChangeTag::Delete);
        assert!(has_changes);
    }

    #[test]
    fn test_generate_diff_addition() {
        let original = "line 1\nline 3";
        let proposed = "line 1\nline 2\nline 3";

        let diff = generate_diff(original, proposed);
        let hunks = extract_hunks(&PathBuf::from("test.txt"), &diff);

        assert!(!hunks.is_empty());
        // Should have an insertion
        let has_insert = hunks[0].changes.iter().any(|c| c.tag == ChangeTag::Insert);
        assert!(has_insert);
    }

    #[test]
    fn test_generate_diff_deletion() {
        let original = "line 1\nline 2\nline 3";
        let proposed = "line 1\nline 3";

        let diff = generate_diff(original, proposed);
        let hunks = extract_hunks(&PathBuf::from("test.txt"), &diff);

        assert!(!hunks.is_empty());
        // Should have a deletion
        let has_delete = hunks[0].changes.iter().any(|c| c.tag == ChangeTag::Delete);
        assert!(has_delete);
    }

    #[test]
    fn test_hunk_has_correct_file_path() {
        let original = "line 1";
        let proposed = "line 1\nline 2";

        let file_path = PathBuf::from("src/main.rs");
        let diff = generate_diff(original, proposed);
        let hunks = extract_hunks(&file_path, &diff);

        assert!(!hunks.is_empty());
        assert_eq!(hunks[0].file_path, file_path);
    }

    #[test]
    fn test_hunk_status_is_pending() {
        let original = "a";
        let proposed = "b";

        let diff = generate_diff(original, proposed);
        let hunks = extract_hunks(&PathBuf::from("test.txt"), &diff);

        assert!(!hunks.is_empty());
        assert_eq!(hunks[0].status, HunkStatus::Pending);
    }
}
