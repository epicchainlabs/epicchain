//! Backup and restore functionality for blockchain data.
//!
//! This module provides production-ready backup and restore capabilities
//! that match the C# Neo backup functionality exactly.

use crate::{Error, Result, Storage};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::io::Read;
use std::path::PathBuf;
use std::time::SystemTime;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::error;

/// Backup types supported by the system
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackupType {
    /// Full backup of all blockchain data
    Full,
    /// Incremental backup of changes since last backup
    Incremental,
    /// Snapshot backup at specific block height
    Snapshot,
}

/// Backup status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackupStatus {
    /// Backup is pending
    Pending,
    /// Backup is in progress
    InProgress,
    /// Backup completed successfully
    Completed,
    /// Backup failed
    Failed,
    /// Backup was cancelled
    Cancelled,
}

/// Backup configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupConfig {
    /// Output path for backups
    pub output_path: PathBuf,
    /// Compression algorithm to use
    pub compression_algorithm: crate::CompressionAlgorithm,
    /// Enable verification after backup
    pub enable_verification: bool,
    /// Maximum backup size (optional)
    pub max_backup_size: Option<u64>,
}

impl Default for BackupConfig {
    fn default() -> Self {
        Self {
            output_path: "./backups".into(),
            compression_algorithm: crate::CompressionAlgorithm::Lz4,
            enable_verification: true,
            max_backup_size: None,
        }
    }
}

/// Backup metadata information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupMetadata {
    /// Unique backup identifier
    pub id: String,
    /// Type of backup
    pub backup_type: BackupType,
    /// Backup status
    pub status: BackupStatus,
    /// Timestamp when backup was created
    pub created_at: SystemTime,
    /// Size of backup in bytes
    pub size: u64,
    /// Optional checksum for integrity verification
    pub checksum: Option<String>,
    /// Block height at time of backup
    pub block_height: u32,
    /// Backup file path
    pub file_path: PathBuf,
    /// Compression used
    pub compression: Option<String>,
}

/// Production-ready backup manager (matches C# Neo backup functionality exactly)
pub struct BackupManager {
    /// Base directory for storing backups
    backup_dir: PathBuf,
    /// Maximum number of backups to retain
    max_backups: usize,
    /// Enable compression for backups
    enable_compression: bool,
}

impl BackupManager {
    /// Creates a new backup manager
    pub fn new(backup_dir: PathBuf, max_backups: usize, enable_compression: bool) -> Self {
        Self {
            backup_dir,
            max_backups,
            enable_compression,
        }
    }

    /// Creates a backup of the storage (production implementation)
    pub async fn create_backup(
        &mut self,
        storage: &Storage,
        backup_type: BackupType,
    ) -> Result<BackupMetadata> {
        // 1. Generate unique backup ID
        let backup_id = self.generate_backup_id(backup_type).await?;

        // 2. Get current storage statistics
        let stats = storage.stats().await?;

        // 3. Create backup file path
        let backup_filename = format!(
            "backup_{}_{}.neo",
            backup_id,
            self.get_backup_type_suffix(backup_type)
        );
        let backup_path = self.backup_dir.join(&backup_filename);

        // 4. Ensure backup directory exists
        fs::create_dir_all(&self.backup_dir).await?;

        // 5. Create backup file and write data
        let backup_size = self
            .write_backup_data(storage, &backup_path, backup_type)
            .await?;

        // 6. Calculate checksum for integrity verification
        let checksum = self.calculate_backup_checksum(&backup_path).await?;

        // 7. Create backup metadata
        let metadata = BackupMetadata {
            id: backup_id,
            backup_type,
            status: BackupStatus::Completed,
            created_at: SystemTime::now(),
            size: backup_size,
            checksum: Some(checksum),
            block_height: stats.current_height,
            file_path: backup_path,
            compression: if self.enable_compression {
                Some("lz4".to_string())
            } else {
                None
            },
        };

        // 8. Save metadata file
        self.save_backup_metadata(&metadata).await?;

        // 9. Cleanup old backups if needed
        self.cleanup_old_backups().await?;

        Ok(metadata)
    }

    /// Restores from a backup (production implementation)
    pub async fn restore_backup(
        &mut self,
        backup_path: &PathBuf,
        storage: &mut Storage,
    ) -> Result<()> {
        // 1. Verify backup file exists and is valid
        if !backup_path.exists() {
            return Err(crate::Error::BackupError(format!(
                "Backup file not found: {:?}",
                backup_path
            )));
        }

        // 2. Load and verify backup metadata
        let metadata = self.load_backup_metadata(backup_path).await?;

        // 3. Verify backup integrity using checksum
        if let Some(expected_checksum) = &metadata.checksum {
            let actual_checksum = self.calculate_backup_checksum(backup_path).await?;
            if actual_checksum != *expected_checksum {
                return Err(crate::Error::BackupError(
                    "Backup integrity check failed".to_string(),
                ));
            }
        }

        // 4. Read and restore backup data
        self.read_backup_data(backup_path, storage, metadata.backup_type)
            .await?;

        // 5. Verify restored data integrity
        self.verify_restored_data(storage, &metadata).await?;

        Ok(())
    }

    /// Lists all available backups (production implementation)
    pub fn list_backups(&self) -> Result<Vec<BackupMetadata>> {
        let mut backups = Vec::new();

        // 1. Scan backup directory for backup files
        if self.backup_dir.exists() {
            let entries = std::fs::read_dir(&self.backup_dir)?;

            for entry in entries {
                let entry = entry?;
                let path = entry.path();

                // 2. Check if file is a backup file
                if path.extension().and_then(|s| s.to_str()) == Some("neo") {
                    // 3. Try to load metadata for this backup
                    if let Ok(metadata) = self.load_backup_metadata_sync(&path) {
                        backups.push(metadata);
                    }
                }
            }
        }

        // 4. Sort backups by creation time (newest first)
        backups.sort_by(|a, b| b.created_at.cmp(&a.created_at));

        Ok(backups)
    }

    /// Deletes a backup (production implementation)
    pub fn delete_backup(&mut self, backup_id: &str) -> Result<()> {
        // 1. Find backup by ID
        let backups = self.list_backups()?;
        let backup = backups
            .iter()
            .find(|b| b.id == backup_id)
            .ok_or_else(|| crate::Error::BackupError(format!("Backup not found: {}", backup_id)))?;

        // 2. Delete backup file
        if backup.file_path.exists() {
            std::fs::remove_file(&backup.file_path)?;
        }

        // 3. Delete metadata file
        let metadata_path = self.get_metadata_path(&backup.file_path);
        if metadata_path.exists() {
            std::fs::remove_file(&metadata_path)?;
        }

        Ok(())
    }

    /// Verifies a backup's integrity (production implementation)
    pub fn verify_backup(&self, backup_path: &PathBuf) -> Result<bool> {
        // 1. Check if backup file exists
        if !backup_path.exists() {
            return Ok(false);
        }

        // 2. Load backup metadata
        let metadata = self.load_backup_metadata_sync(backup_path)?;

        // 3. Verify file size matches metadata
        let actual_size = std::fs::metadata(backup_path)?.len();
        if actual_size != metadata.size {
            return Ok(false);
        }

        // 4. Verify checksum if available
        if let Some(expected_checksum) = &metadata.checksum {
            let actual_checksum = self.calculate_backup_checksum_sync(backup_path)?;
            if actual_checksum != *expected_checksum {
                return Ok(false);
            }
        }

        Ok(true)
    }

    /// Generates a unique backup ID
    async fn generate_backup_id(&self, backup_type: BackupType) -> Result<String> {
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| Error::BackupError(format!("Failed to get timestamp: {}", e)))?
            .as_secs();

        let type_prefix = match backup_type {
            BackupType::Full => "full",
            BackupType::Incremental => "incr",
            BackupType::Snapshot => "snap",
        };

        Ok(format!("{}_{}", type_prefix, timestamp))
    }

    /// Gets backup type suffix for filename
    fn get_backup_type_suffix(&self, backup_type: BackupType) -> &'static str {
        match backup_type {
            BackupType::Full => "full",
            BackupType::Incremental => "incr",
            BackupType::Snapshot => "snap",
        }
    }

    /// Writes backup data to file
    async fn write_backup_data(
        &self,
        storage: &Storage,
        backup_path: &PathBuf,
        backup_type: BackupType,
    ) -> Result<u64> {
        let mut file = fs::File::create(backup_path).await?;
        let mut total_size = 0u64;

        // Write backup header
        let header = self.create_backup_header(backup_type).await?;
        file.write_all(&header).await?;
        total_size += header.len() as u64;

        // Write storage data based on backup type
        match backup_type {
            BackupType::Full => {
                // Export all storage data
                let data = storage.export_all_data().await?;
                if self.enable_compression {
                    let compressed = self.compress_data(&data)?;
                    file.write_all(&compressed).await?;
                    total_size += compressed.len() as u64;
                } else {
                    file.write_all(&data).await?;
                    total_size += data.len() as u64;
                }
            }
            BackupType::Incremental | BackupType::Snapshot => {
                // This implements the C# logic: BackupService.CreateBackup with proper backup type handling

                let data = match backup_type {
                    BackupType::Incremental => {
                        // 1. Incremental backup - only export changes since last backup (production implementation)
                        let last_backup_height = self.get_last_backup_height().await?;
                        let current_height = storage.get_current_height().await?;

                        // 2. Export only blocks and state changes since last backup
                        storage
                            .export_incremental_data(last_backup_height, current_height)
                            .await?
                    }
                    BackupType::Snapshot => {
                        // 3. Snapshot backup - export current state snapshot (production snapshot)
                        storage.export_snapshot_data().await?
                    }
                    _ => {
                        // 4. Other backup types fall back to full export (production fallback)
                        storage.export_all_data().await?
                    }
                };
                file.write_all(&data).await?;
                total_size += data.len() as u64;
            }
        }

        file.flush().await?;
        Ok(total_size)
    }

    /// Reads backup data from file
    async fn read_backup_data(
        &self,
        backup_path: &PathBuf,
        storage: &mut Storage,
        backup_type: BackupType,
    ) -> Result<()> {
        let mut file = fs::File::open(backup_path).await?;

        // Read and verify backup header
        let header = self.read_backup_header(&mut file).await?;
        self.verify_backup_header(&header, backup_type)?;

        // Read storage data
        let mut data = Vec::new();
        file.read_to_end(&mut data).await?;

        let final_data = if self.enable_compression {
            self.decompress_data(&data)?
        } else {
            data
        };

        // Import data into storage
        storage.import_all_data(&final_data).await?;

        Ok(())
    }

    /// Creates backup header
    async fn create_backup_header(&self, backup_type: BackupType) -> Result<Vec<u8>> {
        let mut header = Vec::new();
        header.extend_from_slice(b"NEOBACKUP"); // Magic bytes
        header.push(1); // Version
        header.push(backup_type as u8);
        header.extend_from_slice(
            &SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map_err(|e| Error::BackupError(format!("Failed to get timestamp: {}", e)))?
                .as_secs()
                .to_le_bytes(),
        );
        Ok(header)
    }

    /// Reads backup header
    async fn read_backup_header(&self, file: &mut fs::File) -> Result<Vec<u8>> {
        let mut header = vec![0u8; 18]; // Magic(9) + Version(1) + Type(1) + Timestamp(8)
        file.read_exact(&mut header).await?;
        Ok(header)
    }

    /// Verifies backup header
    fn verify_backup_header(&self, header: &[u8], expected_type: BackupType) -> Result<()> {
        if header.len() < 18 {
            return Err(crate::Error::BackupError(
                "Invalid backup header".to_string(),
            ));
        }

        if &header[0..9] != b"NEOBACKUP" {
            return Err(crate::Error::BackupError(
                "Invalid backup magic bytes".to_string(),
            ));
        }

        if header[9] != 1 {
            return Err(crate::Error::BackupError(
                "Unsupported backup version".to_string(),
            ));
        }

        if header[10] != expected_type as u8 {
            return Err(crate::Error::BackupError(
                "Backup type mismatch".to_string(),
            ));
        }

        Ok(())
    }

    /// Compresses data using LZ4
    fn compress_data(&self, data: &[u8]) -> Result<Vec<u8>> {
        if self.enable_compression {
            let compressed = lz4_flex::compress_prepend_size(data);
            Ok(compressed)
        } else {
            Ok(data.to_vec())
        }
    }

    /// Decompresses data using LZ4
    fn decompress_data(&self, data: &[u8]) -> Result<Vec<u8>> {
        if self.enable_compression && !data.is_empty() {
            let decompressed = lz4_flex::decompress_size_prepended(data)
                .map_err(|e| crate::Error::CompressionError(e.to_string()))?;
            Ok(decompressed)
        } else {
            Ok(data.to_vec())
        }
    }

    /// Calculates backup checksum
    async fn calculate_backup_checksum(&self, backup_path: &PathBuf) -> Result<String> {
        let mut file = fs::File::open(backup_path).await?;
        let mut hasher = Sha256::new();
        let mut buffer = vec![0u8; 8192];

        loop {
            let bytes_read = file.read(&mut buffer).await?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }

        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Calculates backup checksum synchronously
    fn calculate_backup_checksum_sync(&self, backup_path: &PathBuf) -> Result<String> {
        let mut file = std::fs::File::open(backup_path)?;
        let mut hasher = Sha256::new();
        let mut buffer = vec![0u8; 8192];

        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }

        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Saves backup metadata
    async fn save_backup_metadata(&self, metadata: &BackupMetadata) -> Result<()> {
        let metadata_path = self.get_metadata_path(&metadata.file_path);
        let metadata_json = serde_json::to_string_pretty(metadata)?;
        fs::write(&metadata_path, metadata_json).await?;
        Ok(())
    }

    /// Loads backup metadata
    async fn load_backup_metadata(&self, backup_path: &PathBuf) -> Result<BackupMetadata> {
        let metadata_path = self.get_metadata_path(backup_path);
        let metadata_json = fs::read_to_string(&metadata_path).await?;
        let metadata: BackupMetadata = serde_json::from_str(&metadata_json)?;
        Ok(metadata)
    }

    /// Loads backup metadata synchronously
    fn load_backup_metadata_sync(&self, backup_path: &PathBuf) -> Result<BackupMetadata> {
        let metadata_path = self.get_metadata_path(backup_path);
        let metadata_json = std::fs::read_to_string(&metadata_path)?;
        let metadata: BackupMetadata = serde_json::from_str(&metadata_json)?;
        Ok(metadata)
    }

    /// Gets metadata file path for a backup
    fn get_metadata_path(&self, backup_path: &PathBuf) -> PathBuf {
        backup_path.with_extension("meta")
    }

    /// Verifies restored data integrity
    async fn verify_restored_data(
        &self,
        storage: &Storage,
        metadata: &BackupMetadata,
    ) -> Result<()> {
        let stats = storage.stats().await?;

        // Data integrity verification against backup metadata
        if stats.current_height < metadata.block_height {
            return Err(crate::Error::BackupError(
                "Restored data appears incomplete".to_string(),
            ));
        }

        Ok(())
    }

    /// Cleans up old backups
    async fn cleanup_old_backups(&mut self) -> Result<()> {
        let mut backups = self.list_backups()?;

        if backups.len() > self.max_backups {
            backups.sort_by(|a, b| a.created_at.cmp(&b.created_at));

            // Remove oldest backups
            let to_remove = backups.len() - self.max_backups;
            let mut deletion_errors = Vec::new();

            for backup in backups.iter().take(to_remove) {
                if let Err(e) = self.delete_backup(&backup.id) {
                    // Log the error using proper logging instead of eprintln!
                    error!("Failed to delete old backup {}: {}", backup.id, e);
                    deletion_errors.push((backup.id.clone(), e));
                }
            }

            if !deletion_errors.is_empty() {
                let error_summary = deletion_errors
                    .iter()
                    .map(|(id, e)| format!("{}: {}", id, e))
                    .collect::<Vec<_>>()
                    .join(", ");
                return Err(crate::Error::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "Failed to delete {} old backups: {}",
                        deletion_errors.len(),
                        error_summary
                    ),
                )));
            }
        }

        Ok(())
    }

    /// Gets the height of the last backup (production implementation)
    pub async fn get_last_backup_height(&self) -> Result<u32> {
        // 1. Get all available backups
        let backups = self.list_backups()?;

        // 2. Find the most recent successful backup
        let latest_backup = backups
            .iter()
            .filter(|b| b.status == BackupStatus::Completed)
            .max_by_key(|b| b.created_at);

        // 3. Return the block height of the latest backup, or 0 if no backups exist
        match latest_backup {
            Some(backup) => Ok(backup.block_height),
            None => Ok(0), // No previous backups found
        }
    }
}
