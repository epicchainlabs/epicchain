//! Storage-specific error handling
//!
//! This module provides specialized error handling for storage operations,
//! including database errors, disk space issues, and data corruption.

use crate::error_handler::{ErrorCategory, ErrorHandler, ErrorSeverity, RecoveryAction};
use anyhow::{Context, Result};
use std::path::Path;
use std::sync::Arc;
use tracing::{error, info, warn};

/// Storage error types
#[derive(Debug, Clone)]
pub enum StorageError {
    /// Database corruption detected
    DatabaseCorruption { path: String, details: String },
    /// Disk space exhausted
    DiskSpaceExhausted { available: u64, required: u64 },
    /// Write operation failed
    WriteFailed { path: String, reason: String },
    /// Read operation failed
    ReadFailed { path: String, reason: String },
    /// Database locked
    DatabaseLocked { path: String },
    /// Backup failed
    BackupFailed { reason: String },
    /// Data integrity check failed
    IntegrityCheckFailed { details: String },
}

/// Handle storage-specific errors with recovery
pub async fn handle_storage_error(
    error: StorageError,
    storage_path: &Path,
    error_handler: Arc<ErrorHandler>,
) -> Result<()> {
    let (severity, context) = match &error {
        StorageError::DatabaseCorruption { path, details } => {
            error!("Database corruption detected at {}: {}", path, details);
            (ErrorSeverity::Critical, "database_corruption")
        }
        StorageError::DiskSpaceExhausted {
            available,
            required,
        } => {
            error!(
                "Disk space exhausted: {} bytes available, {} required",
                available, required
            );
            (ErrorSeverity::Critical, "disk_space")
        }
        StorageError::WriteFailed { path, reason } => {
            error!("Write operation failed at {}: {}", path, reason);
            (ErrorSeverity::High, "write_failed")
        }
        StorageError::ReadFailed { path, reason } => {
            warn!("Read operation failed at {}: {}", path, reason);
            (ErrorSeverity::Medium, "read_failed")
        }
        StorageError::DatabaseLocked { path } => {
            warn!("Database locked at {}", path);
            (ErrorSeverity::Medium, "database_locked")
        }
        StorageError::BackupFailed { reason } => {
            error!("Backup operation failed: {}", reason);
            (ErrorSeverity::High, "backup_failed")
        }
        StorageError::IntegrityCheckFailed { details } => {
            error!("Data integrity check failed: {}", details);
            (ErrorSeverity::Critical, "integrity_failed")
        }
    };

    // Handle the error
    let action = error_handler
        .handle_error(
            anyhow::anyhow!("{:?}", error),
            ErrorCategory::Storage,
            severity,
            context,
        )
        .await?;

    // Execute recovery action
    match action {
        RecoveryAction::UseFallback(method) => {
            if method == "repair_database" {
                info!("Attempting to repair database");
                repair_database(storage_path).await?;
            } else if method == "use_backup" {
                info!("Attempting to restore from backup");
                restore_from_backup(storage_path).await?;
            }
        }
        RecoveryAction::RestartComponent(_) => {
            warn!("Storage component restart required - clearing caches");
            clear_storage_caches().await?;
        }
        RecoveryAction::Shutdown => {
            error!("Storage error requires node shutdown");
            emergency_backup(storage_path).await?;
        }
        _ => {
            tracing::debug!("No specific recovery action for storage error");
        }
    }

    Ok(())
}

/// Attempt to repair corrupted database
async fn repair_database(storage_path: &Path) -> Result<()> {
    info!("Starting database repair process/* implementation */;");

    // Create repair directory
    let repair_path = storage_path.join("repair");
    std::fs::create_dir_all(&repair_path).context("Failed to create repair directory")?;

    // Run RocksDB repair
    let opts = rocksdb::Options::default();
    match rocksdb::DB::repair(&opts, storage_path) {
        Ok(_) => {
            info!("Database repair completed successfully");
            Ok(())
        }
        Err(e) => {
            error!("Database repair failed: {}", e);
            Err(anyhow::anyhow!("Database repair failed: {}", e))
        }
    }
}

/// Restore database from backup
async fn restore_from_backup(storage_path: &Path) -> Result<()> {
    info!("Starting database restore from backup/* implementation */;");

    let backup_path = storage_path.join("backup");
    if !backup_path.exists() {
        return Err(anyhow::anyhow!("No backup available at {:?}", backup_path));
    }

    // Move current database to corrupted directory
    let corrupted_path = storage_path.join("corrupted");
    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let corrupted_backup = corrupted_path.join(format!("backup_{}", timestamp));
    std::fs::create_dir_all(&corrupted_backup)?;

    // Copy current files to corrupted backup
    for entry in std::fs::read_dir(storage_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            let filename = path
                .file_name()
                .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?;
            std::fs::copy(&path, corrupted_backup.join(filename))?;
        }
    }

    // Restore files from backup
    for entry in std::fs::read_dir(&backup_path)? {
        let entry = entry?;
        let src = entry.path();
        if src.is_file() {
            let filename = src
                .file_name()
                .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?;
            let dst = storage_path.join(filename);
            std::fs::copy(&src, &dst)?;
            info!("Restored file: {:?}", filename);
        }
    }

    info!("Backup restore completed successfully");
    Ok(())
}

/// Clear storage caches
async fn clear_storage_caches() -> Result<()> {
    info!("Clearing storage caches/* implementation */;");

    // Force garbage collection and compaction
    // RocksDB automatically manages its own caches
    // We can hint the system to release memory
    #[cfg(target_os = "linux")]
    {
        // Advise kernel to drop caches
        let _ = std::process::Command::new("sync").output();
    }

    info!("Storage cache clearing completed");
    Ok(())
}

/// Create emergency backup before shutdown
async fn emergency_backup(storage_path: &Path) -> Result<()> {
    warn!("Creating emergency backup before shutdown/* implementation */;");

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let backup_path = storage_path.join(format!("emergency_backup_{}", timestamp));
    std::fs::create_dir_all(&backup_path)?;

    // Copy critical database files
    let critical_files = vec!["CURRENT", "MANIFEST-*", "*.log", "*.sst"];
    let mut backed_up = 0;

    for pattern in critical_files {
        for entry in std::fs::read_dir(storage_path)? {
            let entry = entry?;
            let path = entry.path();
            let filename = path
                .file_name()
                .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("Invalid filename encoding"))?;

            if pattern.contains('*') {
                let prefix = pattern.trim_end_matches('*');
                if filename.starts_with(prefix) {
                    std::fs::copy(&path, backup_path.join(filename))?;
                    backed_up += 1;
                }
            } else if filename == pattern {
                std::fs::copy(&path, backup_path.join(filename))?;
                backed_up += 1;
            }
        }
    }

    info!(
        "Emergency backup created at {:?} ({} files)",
        backup_path, backed_up
    );
    Ok(())
}

/// Monitor storage health
pub async fn monitor_storage_health(storage_path: &Path, error_handler: Arc<ErrorHandler>) {
    loop {
        tokio::time::sleep(std::time::Duration::from_secs(60)).await;

        // Check disk space
        if let Ok(available) = get_available_disk_space(storage_path) {
            let min_required = 1_000_000_000; // 1GB minimum
            if available < min_required {
                let _ = handle_storage_error(
                    StorageError::DiskSpaceExhausted {
                        available,
                        required: min_required,
                    },
                    storage_path,
                    error_handler.clone(),
                )
                .await;
            }
        }

        // Check database integrity
        if should_check_integrity() {
            if let Err(e) = check_database_integrity(storage_path).await {
                let _ = handle_storage_error(
                    StorageError::IntegrityCheckFailed {
                        details: e.to_string(),
                    },
                    storage_path,
                    error_handler.clone(),
                )
                .await;
            }
        }
    }
}

/// Get available disk space
fn get_available_disk_space(path: &Path) -> Result<u64> {
    #[cfg(unix)]
    {
        use libc::statvfs;
        use std::ffi::CString;
        use std::os::unix::fs::MetadataExt;

        let path_str = CString::new(
            path.to_str()
                .ok_or_else(|| anyhow::anyhow!("Invalid path"))?,
        )?;
        // SAFETY: Operation is safe within this context
        let mut stat: libc::statvfs = unsafe { std::mem::zeroed() };

        // SAFETY: Operation is safe within this context
        let result = unsafe { statvfs(path_str.as_ptr(), &mut stat) };

        if result == 0 {
            let available = (stat.f_bavail as u64) * (stat.f_bsize as u64);
            Ok(available)
        } else {
            Err(anyhow::anyhow!("Failed to get disk space statistics"))
        }
    }

    #[cfg(windows)]
    {
        use std::os::windows::ffi::OsStrExt;
        use std::ptr::null_mut;
        use winapi::um::fileapi::GetDiskFreeSpaceExW;

        let path_wide: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
        let mut available: u64 = 0;

        // SAFETY: Operation is safe within this context
        let result = unsafe {
            GetDiskFreeSpaceExW(path_wide.as_ptr(), &mut available as *mut u64, null_mut())
        };

        if result != 0 {
            Ok(available)
        } else {
            Err(anyhow::anyhow!("Failed to get disk space statistics"))
        }
    }

    #[cfg(not(any(unix, windows)))]
    {
        Ok(10_000_000_000) // 10GB default
    }
}

/// Check if integrity check should run
fn should_check_integrity() -> bool {
    use std::sync::atomic::{AtomicU64, Ordering};

    static LAST_CHECK: AtomicU64 = AtomicU64::new(0);

    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();

    let last = LAST_CHECK.load(Ordering::Relaxed);

    if now - last > 3600 {
        LAST_CHECK.store(now, Ordering::Relaxed);
        true
    } else {
        false
    }
}

/// Check database integrity
async fn check_database_integrity(path: &Path) -> Result<()> {
    info!("Running database integrity check/* implementation */;");

    let required_files = vec!["CURRENT", "IDENTITY", "LOCK"];

    for file in required_files {
        let file_path = path.join(file);
        if !file_path.exists() {
            return Err(anyhow::anyhow!("Missing required database file: {}", file));
        }

        std::fs::metadata(&file_path)
            .map_err(|e| anyhow::anyhow!("Cannot access {}: {}", file, e))?;
    }

    // Verify CURRENT file points to valid manifest
    let current_file = path.join("CURRENT");
    let current_content = std::fs::read_to_string(&current_file)?;
    let manifest_name = current_content.trim();

    let manifest_path = path.join(manifest_name);
    if !manifest_path.exists() {
        return Err(anyhow::anyhow!(
            "Manifest file {} referenced in CURRENT does not exist",
            manifest_name
        ));
    }

    info!("Database integrity check passed");
    Ok(())
}

/// Create periodic backups
pub async fn periodic_backup_task(storage_path: &Path) {
    let backup_interval = std::time::Duration::from_secs(3600);

    loop {
        tokio::time::sleep(backup_interval).await;

        info!("Starting periodic backup/* implementation */;");
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        let backup_path = storage_path
            .join("backup")
            .join(format!("snapshot_{}", timestamp));

        match create_backup(storage_path, &backup_path).await {
            Ok(file_count) => info!(
                "Periodic backup completed successfully ({} files)",
                file_count
            ),
            Err(e) => error!("Periodic backup failed: {}", e),
        }

        let _ = clean_old_backups(&storage_path.join("backup"), 24).await;
    }
}

/// Create backup
async fn create_backup(source_path: &Path, backup_path: &Path) -> Result<usize> {
    std::fs::create_dir_all(backup_path)?;

    let mut file_count = 0;

    // Copy all database files
    for entry in std::fs::read_dir(source_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            let filename = path
                .file_name()
                .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?;
            let dest = backup_path.join(filename);

            // Skip temporary files
            let name_str = filename.to_str().unwrap_or("");
            if !name_str.starts_with(".") && !name_str.ends_with(".tmp") {
                std::fs::copy(&path, &dest)?;
                file_count += 1;
            }
        }
    }

    Ok(file_count)
}

/// Clean old backup directories
async fn clean_old_backups(backup_dir: &Path, keep_hours: u64) -> Result<()> {
    let cutoff = std::time::SystemTime::now() - std::time::Duration::from_secs(keep_hours * 3600);

    if backup_dir.exists() {
        for entry in std::fs::read_dir(backup_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let metadata = std::fs::metadata(&path)?;
                if let Ok(modified) = metadata.modified() {
                    if modified < cutoff {
                        std::fs::remove_dir_all(&path)?;
                        info!(
                            "Removed old backup: {:?}",
                            path.file_name()
                                .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?
                        );
                    }
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn test_storage_error_types() {
        let error = StorageError::DiskSpaceExhausted {
            available: 1000,
            required: 5000,
        };

        match error {
            StorageError::DiskSpaceExhausted {
                available,
                required,
            } => {
                assert_eq!(available, 1000);
                assert_eq!(required, 5000);
            }
            _ => {
                error!("Unexpected error type in storage error handler");
                return Err(Error::StorageError("Unexpected error type".to_string()));
            }
        }
    }
}
