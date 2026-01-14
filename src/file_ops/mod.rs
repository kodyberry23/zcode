//! Safe file operations with atomic writes and rollback support
//!
//! This module provides robust file modification capabilities:
//!
//! - **Atomic writes**: Files are written safely using temp files and atomic renames
//! - **Backup management**: Automatic backups before any modifications
//! - **Rollback support**: Restore from backups if any operation fails
//! - **Transaction semantics**: All-or-nothing guarantees across multiple files
//! - **Hunk application**: Intelligent application of code hunks to files
//!
//! # Submodules
//!
//! - [`apply`]: Orchestrates the complete file modification pipeline
//! - [`backup`]: Manages backup creation and restoration
//! - [`reconstruct`]: Applies hunks to file content

pub mod apply;
pub mod backup;
pub mod reconstruct;

pub use apply::{apply_accepted_hunks, ApplyResult};
pub use backup::BackupSet;
pub use reconstruct::reconstruct_file_content;

// Re-export common utilities
use anyhow::{Context, Result};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use tempfile::NamedTempFile;

/// Atomic file write using temp file + rename
pub fn atomic_write(path: &Path, content: &str) -> Result<()> {
    let dir = path.parent().unwrap_or(Path::new("."));

    // Ensure directory exists
    fs::create_dir_all(dir)?;

    // Create temp file in same directory (required for atomic rename)
    let mut temp = NamedTempFile::new_in(dir).context("Failed to create temp file")?;

    // Write content
    temp.write_all(content.as_bytes())
        .context("Failed to write to temp file")?;

    // Fsync to ensure data is on disk
    temp.as_file()
        .sync_all()
        .context("Failed to sync temp file")?;

    // Atomic rename
    temp.persist(path).context("Failed to persist temp file")?;

    // Fsync directory for metadata durability
    if let Ok(dir_file) = fs::File::open(dir) {
        let _ = dir_file.sync_all();
    }

    Ok(())
}

/// Create a backup file
pub fn create_backup(original_path: &Path, content: &str) -> Result<PathBuf> {
    let backup_dir = dirs::cache_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("zcode")
        .join("backups");

    fs::create_dir_all(&backup_dir)?;

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let filename = original_path
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "unknown".to_string());

    let backup_path = backup_dir.join(format!("{}_{}", timestamp, filename));

    atomic_write(&backup_path, content)?;

    Ok(backup_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_atomic_write_creates_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let test_file = temp_dir.path().join("test.txt");

        atomic_write(&test_file, "test content").unwrap();

        assert!(test_file.exists());
        let content = fs::read_to_string(&test_file).unwrap();
        assert_eq!(content, "test content");
    }
}
