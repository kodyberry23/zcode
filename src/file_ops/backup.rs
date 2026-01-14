// src/file_ops/backup.rs - Backup set management

use anyhow::{anyhow, Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use super::atomic_write;

/// A set of backups created at a specific time
#[derive(Debug, Clone)]
pub struct BackupSet {
    /// Mapping from original file path to backup path
    pub backups: HashMap<PathBuf, PathBuf>,
    /// Timestamp when backups were created
    pub timestamp: String,
}

impl BackupSet {
    /// Create backups for the given files
    pub fn create(files: &[PathBuf]) -> Result<Self> {
        let mut backups = HashMap::new();
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S_%3f").to_string();

        for file_path in files {
            // Read original content
            let original_content = fs::read_to_string(file_path).context(format!(
                "Failed to read file for backup: {}",
                file_path.display()
            ))?;

            // Create backup
            let backup_path = Self::backup_path(file_path, &timestamp)?;
            atomic_write(&backup_path, &original_content)?;

            backups.insert(file_path.clone(), backup_path);
        }

        Ok(BackupSet { backups, timestamp })
    }

    /// Generate backup path for a file
    fn backup_path(original_path: &PathBuf, timestamp: &str) -> Result<PathBuf> {
        let backup_dir = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("zcode")
            .join("backups");

        fs::create_dir_all(&backup_dir)?;

        let filename = original_path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "unknown".to_string());

        Ok(backup_dir.join(format!("{}_{}", timestamp, filename)))
    }

    /// Restore all files from backups
    pub fn restore_all(&self) -> Result<()> {
        let mut errors = Vec::new();

        for (original_path, backup_path) in &self.backups {
            match Self::restore_single(original_path, backup_path) {
                Ok(_) => {}
                Err(e) => errors.push((original_path.clone(), e)),
            }
        }

        if !errors.is_empty() {
            let error_msg = errors
                .iter()
                .map(|(path, e)| format!("{}: {}", path.display(), e))
                .collect::<Vec<_>>()
                .join("; ");
            return Err(anyhow!("Failed to restore some files: {}", error_msg));
        }

        Ok(())
    }

    /// Restore a single file from its backup
    fn restore_single(original_path: &PathBuf, backup_path: &PathBuf) -> Result<()> {
        let backup_content = fs::read_to_string(backup_path)
            .context(format!("Failed to read backup: {}", backup_path.display()))?;

        atomic_write(original_path, &backup_content)
            .context(format!("Failed to restore: {}", original_path.display()))?;

        Ok(())
    }

    /// List all backups in this set
    pub fn backup_paths(&self) -> Vec<PathBuf> {
        self.backups.values().cloned().collect()
    }

    /// Get the backup path for a specific file
    pub fn get_backup(&self, original_path: &PathBuf) -> Option<&PathBuf> {
        self.backups.get(original_path)
    }

    /// Clean up backup files (optional for user)
    pub fn cleanup(&self) -> Result<()> {
        for backup_path in self.backup_paths() {
            if backup_path.exists() {
                fs::remove_file(&backup_path).context(format!(
                    "Failed to delete backup: {}",
                    backup_path.display()
                ))?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backup_set_creation() {
        let temp_dir = tempfile::tempdir().unwrap();
        let test_file = temp_dir.path().join("test.txt");

        // Create a test file
        fs::write(&test_file, "original content").unwrap();

        // Create backup set
        let backup_set = BackupSet::create(&[test_file.clone()]).unwrap();

        // Verify backup was created
        assert_eq!(backup_set.backups.len(), 1);
        assert!(backup_set.get_backup(&test_file).is_some());

        let backup_path = backup_set.get_backup(&test_file).unwrap();
        assert!(backup_path.exists());

        let backup_content = fs::read_to_string(backup_path).unwrap();
        assert_eq!(backup_content, "original content");
    }

    #[test]
    fn test_backup_set_restore() {
        let temp_dir = tempfile::tempdir().unwrap();
        let test_file = temp_dir.path().join("test.txt");

        // Create original file
        fs::write(&test_file, "original content").unwrap();

        // Create backup
        let backup_set = BackupSet::create(&[test_file.clone()]).unwrap();

        // Modify the original file
        fs::write(&test_file, "modified content").unwrap();

        // Restore from backup
        backup_set.restore_all().unwrap();

        // Verify restoration
        let restored_content = fs::read_to_string(&test_file).unwrap();
        assert_eq!(restored_content, "original content");
    }
}
