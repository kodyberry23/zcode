// src/file_ops/apply.rs - Hunk application logic with transaction model

use anyhow::{anyhow, Context, Result};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::PathBuf;

use super::{atomic_write, reconstruct_file_content, BackupSet};
use crate::config::Config;
use crate::state::{ChangeType, FileChange, Hunk, HunkStatus};

/// Result of applying hunks to files
#[derive(Debug, Clone)]
pub struct ApplyResult {
    /// Files that were successfully modified
    pub files_modified: Vec<PathBuf>,
    /// Backup files that were created
    pub backups_created: Vec<PathBuf>,
    /// Number of hunks applied
    pub hunks_applied: usize,
}

/// Apply all accepted hunks to their respective files
pub fn apply_accepted_hunks(
    hunks: &[&Hunk],
    pending_changes: &HashMap<PathBuf, FileChange>,
    config: &Config,
) -> Result<ApplyResult> {
    // Filter to only accepted hunks
    let accepted_hunks: Vec<_> = hunks
        .iter()
        .filter(|h| h.status == HunkStatus::Accepted)
        .copied()
        .collect();

    if accepted_hunks.is_empty() {
        return Err(anyhow!("No accepted hunks to apply"));
    }

    // Group hunks by file
    let mut hunks_by_file: BTreeMap<PathBuf, Vec<&Hunk>> = BTreeMap::new();
    for hunk in &accepted_hunks {
        hunks_by_file
            .entry(hunk.file_path.clone())
            .or_default()
            .push(hunk);
    }

    // Prepare files to modify
    let files_to_modify: Vec<PathBuf> = hunks_by_file.keys().cloned().collect();

    // Create backups for all files (transaction model)
    let backup_set = if config.general.create_backups {
        BackupSet::create(&files_to_modify).context("Failed to create backups")?
    } else {
        BackupSet {
            backups: HashMap::new(),
            timestamp: String::new(),
        }
    };

    let backups_created = backup_set.backup_paths();

    // Apply changes to all files
    let files_modified = match apply_all_files(&hunks_by_file, pending_changes) {
        Ok(modified) => modified,
        Err(e) => {
            // Rollback on failure
            if config.general.create_backups {
                let _ = backup_set.restore_all();
            }
            return Err(e).context("Failed to apply hunks");
        }
    };

    Ok(ApplyResult {
        files_modified,
        backups_created,
        hunks_applied: accepted_hunks.len(),
    })
}

/// Apply changes to all affected files
fn apply_all_files(
    hunks_by_file: &BTreeMap<PathBuf, Vec<&Hunk>>,
    pending_changes: &HashMap<PathBuf, FileChange>,
) -> Result<Vec<PathBuf>> {
    let mut files_modified = Vec::new();

    for (file_path, hunks) in hunks_by_file {
        // Reconstruct file content
        let new_content = if hunks.iter().any(|h| {
            pending_changes
                .get(file_path)
                .map(|c| c.change_type == ChangeType::Create)
                .unwrap_or(false)
        }) {
            // New file creation
            reconstruct_file_content("", hunks).context(format!(
                "Failed to reconstruct new file: {}",
                file_path.display()
            ))?
        } else {
            // Existing file modification
            let original = fs::read_to_string(file_path)
                .context(format!("Failed to read file: {}", file_path.display()))?;

            reconstruct_file_content(&original, hunks).context(format!(
                "Failed to reconstruct file: {}",
                file_path.display()
            ))?
        };

        // Write file atomically
        atomic_write(file_path, &new_content)
            .context(format!("Failed to write file: {}", file_path.display()))?;

        files_modified.push(file_path.clone());
    }

    Ok(files_modified)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::{ChangeTag, LineChange};

    fn create_test_hunk(
        file_path: PathBuf,
        start_line: usize,
        changes: Vec<(ChangeTag, String)>,
    ) -> Hunk {
        Hunk {
            id: 0,
            file_path,
            start_line,
            end_line: start_line + 5,
            changes: changes
                .into_iter()
                .enumerate()
                .map(|(idx, (tag, content))| {
                    let old_line = if tag == ChangeTag::Delete {
                        Some(start_line + idx)
                    } else {
                        None
                    };
                    let new_line = if tag != ChangeTag::Delete {
                        Some(start_line + idx)
                    } else {
                        None
                    };
                    LineChange {
                        tag,
                        content,
                        old_line_num: old_line,
                        new_line_num: new_line,
                    }
                })
                .collect(),
            status: HunkStatus::Accepted,
        }
    }

    #[test]
    fn test_apply_result_structure() {
        let result = ApplyResult {
            files_modified: vec![PathBuf::from("test.txt")],
            backups_created: vec![PathBuf::from("/backup/test.txt")],
            hunks_applied: 1,
        };

        assert_eq!(result.files_modified.len(), 1);
        assert_eq!(result.backups_created.len(), 1);
        assert_eq!(result.hunks_applied, 1);
    }

    #[test]
    fn test_no_accepted_hunks() {
        let mut hunk = create_test_hunk(
            PathBuf::from("test.txt"),
            1,
            vec![(ChangeTag::Insert, "line".to_string())],
        );
        hunk.status = HunkStatus::Pending; // Mark as not accepted

        let config = Config::default();
        let pending_changes = HashMap::new();

        let result = apply_accepted_hunks(&[&hunk], &pending_changes, &config);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("No accepted hunks"));
    }
}
