use anyhow::{Result, anyhow};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::time::sleep;
use uuid::Uuid;
use reqwest::Client;
use std::path::PathBuf;
use crate::advanced_learning_system::{AdvancedLearningSystem, WritingMemory, SessionMemory};
use crate::config::get_learning_data_dir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulMemoryConfig {
    pub enabled: bool,
    pub cloud_endpoint: String,
    pub sync_interval_minutes: u64,
    pub max_retry_attempts: u32,
    pub corruption_check_interval_hours: u64,
    pub auto_recovery: bool,
    pub backup_generations: u8,
    pub compression_enabled: bool,
    pub encryption_key: Option<String>,
}

impl Default for SoulMemoryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            cloud_endpoint: "https://api.jsonbin.io/v3/b".to_string(), // Free cloud storage
            sync_interval_minutes: 15,
            max_retry_attempts: 3,
            corruption_check_interval_hours: 6,
            auto_recovery: true,
            backup_generations: 5,
            compression_enabled: true,
            encryption_key: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulSnapshot {
    pub snapshot_id: String,
    pub timestamp: DateTime<Utc>,
    pub learning_data: WritingMemory,
    pub checksum: String,
    pub generation: u64,
    pub compressed: bool,
    pub encrypted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoulSyncState {
    pub last_sync: Option<DateTime<Utc>>,
    pub last_corruption_check: Option<DateTime<Utc>>,
    pub current_generation: u64,
    pub sync_failures: u32,
    pub recovery_count: u32,
    pub total_syncs: u64,
}

#[derive(Debug)]
pub struct SoulMemory {
    config: SoulMemoryConfig,
    sync_state: Arc<Mutex<SoulSyncState>>,
    client: Client,
    is_syncing: Arc<Mutex<bool>>,
    last_snapshot: Arc<Mutex<Option<SoulSnapshot>>>,
    local_backup_path: PathBuf,
}

impl SoulMemory {
    pub fn new(config: SoulMemoryConfig) -> Result<Self> {
        let learning_dir = get_learning_data_dir()?;
        let local_backup_path = learning_dir.join("soul_backups");
        std::fs::create_dir_all(&local_backup_path)?;

        Ok(Self {
            config,
            sync_state: Arc::new(Mutex::new(SoulSyncState {
                last_sync: None,
                last_corruption_check: None,
                current_generation: 0,
                sync_failures: 0,
                recovery_count: 0,
                total_syncs: 0,
            })),
            client: Client::builder()
                .timeout(Duration::from_secs(30))
                .build()
                .map_err(|e| anyhow!("Failed to create HTTP client: {}", e))?,
            is_syncing: Arc::new(Mutex::new(false)),
            last_snapshot: Arc::new(Mutex::new(None)),
            local_backup_path,
        })
    }

    pub async fn start_background_sync(&self, learning_system: Arc<Mutex<AdvancedLearningSystem>>) -> Result<()> {
        if !self.config.enabled {
            println!("🧠 Soul Memory disabled in configuration");
            return Ok(());
        }

        println!("🌟 Starting Soul Memory background sync...");
        
        // Load any existing state
        self.load_local_state().await?;
        
        // Start the background sync loop
        let soul_memory = self.clone();
        let learning_ref = learning_system.clone();
        
        tokio::spawn(async move {
            soul_memory.sync_loop(learning_ref).await;
        });

        // Start corruption check loop
        let corruption_checker = self.clone();
        tokio::spawn(async move {
            corruption_checker.corruption_check_loop().await;
        });

        Ok(())
    }

    async fn sync_loop(&self, learning_system: Arc<Mutex<AdvancedLearningSystem>>) {
        let mut interval = tokio::time::interval(Duration::from_secs(self.config.sync_interval_minutes * 60));
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.perform_sync(&learning_system).await {
                eprintln!("🌩️ Soul Memory sync failed: {}", e);
                self.increment_sync_failures().await;
            } else {
                self.reset_sync_failures().await;
            }
        }
    }

    async fn corruption_check_loop(&self) {
        let mut interval = tokio::time::interval(Duration::from_secs(self.config.corruption_check_interval_hours * 3600));
        
        loop {
            interval.tick().await;
            
            if let Err(e) = self.check_and_recover_corruption().await {
                eprintln!("🔍 Soul Memory corruption check failed: {}", e);
            }
        }
    }

    async fn perform_sync(&self, learning_system: &Arc<Mutex<AdvancedLearningSystem>>) -> Result<()> {
        // Check if already syncing
        {
            let mut is_syncing = self.is_syncing.lock().unwrap();
            if *is_syncing {
                return Ok(());
            }
            *is_syncing = true;
        }

        let result = self.perform_sync_internal(learning_system).await;
        
        // Release sync lock
        {
            let mut is_syncing = self.is_syncing.lock().unwrap();
            *is_syncing = false;
        }

        result
    }

    async fn perform_sync_internal(&self, learning_system: &Arc<Mutex<AdvancedLearningSystem>>) -> Result<()> {
        // Create snapshot
        let snapshot = self.create_snapshot(learning_system).await?;
        
        // Try to upload to cloud
        let mut attempts = 0;
        while attempts < self.config.max_retry_attempts {
            match self.upload_snapshot(&snapshot).await {
                Ok(_) => {
                    // Success - update sync state
                    {
                        let mut state = self.sync_state.lock().unwrap();
                        state.last_sync = Some(Utc::now());
                        state.current_generation += 1;
                        state.total_syncs += 1;
                    }
                    
                    // Save local backup
                    self.save_local_backup(&snapshot).await?;
                    
                    // Update last snapshot
                    {
                        let mut last_snapshot = self.last_snapshot.lock().unwrap();
                        *last_snapshot = Some(snapshot);
                    }
                    
                    return Ok(());
                },
                Err(e) => {
                    attempts += 1;
                    if attempts < self.config.max_retry_attempts {
                        eprintln!("🔄 Soul Memory upload attempt {} failed, retrying: {}", attempts, e);
                        sleep(Duration::from_secs(2_u64.pow(attempts))).await; // Exponential backoff
                    } else {
                        return Err(anyhow!("Failed to upload after {} attempts: {}", attempts, e));
                    }
                }
            }
        }

        Ok(())
    }

    async fn create_snapshot(&self, learning_system: &Arc<Mutex<AdvancedLearningSystem>>) -> Result<SoulSnapshot> {
        let learning_data = {
            let system = learning_system.lock().unwrap();
            system.writing_memory.clone()
        };

        let snapshot_id = Uuid::new_v4().to_string();
        let timestamp = Utc::now();
        
        let serialized_data = if self.config.compression_enabled {
            self.compress_data(&serde_json::to_string(&learning_data)?)?
        } else {
            serde_json::to_string(&learning_data)?
        };

        let checksum = self.calculate_checksum(&serialized_data);
        
        let generation = {
            let state = self.sync_state.lock().unwrap();
            state.current_generation + 1
        };

        Ok(SoulSnapshot {
            snapshot_id,
            timestamp,
            learning_data,
            checksum,
            generation,
            compressed: self.config.compression_enabled,
            encrypted: self.config.encryption_key.is_some(),
        })
    }

    async fn upload_snapshot(&self, snapshot: &SoulSnapshot) -> Result<()> {
        let serialized_snapshot = serde_json::to_string(snapshot)?;
        
        // Use JSONBin.io as free cloud storage
        let url = format!("{}/{}", self.config.cloud_endpoint, snapshot.snapshot_id);
        
        let response = self.client
            .put(&url)
            .header("Content-Type", "application/json")
            .header("X-Master-Key", "$2a$10$...") // Would be configured or generated
            .header("X-Bin-Name", format!("pundit-soul-{}", snapshot.generation))
            .body(serialized_snapshot)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("Upload failed with status: {}", response.status()));
        }

        Ok(())
    }

    async fn download_snapshot(&self, snapshot_id: &str) -> Result<SoulSnapshot> {
        let url = format!("{}/{}/latest", self.config.cloud_endpoint, snapshot_id);
        
        let response = self.client
            .get(&url)
            .header("X-Master-Key", "$2a$10$...") // Would be configured
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow!("Download failed with status: {}", response.status()));
        }

        let snapshot_data = response.text().await?;
        let snapshot: SoulSnapshot = serde_json::from_str(&snapshot_data)?;
        
        // Verify checksum
        let serialized_data = serde_json::to_string(&snapshot.learning_data)?;
        let expected_checksum = self.calculate_checksum(&serialized_data);
        
        if snapshot.checksum != expected_checksum {
            return Err(anyhow!("Snapshot checksum mismatch - data may be corrupted"));
        }

        Ok(snapshot)
    }

    async fn save_local_backup(&self, snapshot: &SoulSnapshot) -> Result<()> {
        let backup_file = self.local_backup_path.join(format!("soul_backup_{}.json", snapshot.generation));
        let snapshot_json = serde_json::to_string_pretty(snapshot)?;
        std::fs::write(&backup_file, snapshot_json)?;
        
        // Clean up old backups
        self.cleanup_old_backups().await?;
        
        Ok(())
    }

    async fn cleanup_old_backups(&self) -> Result<()> {
        let mut backup_files = Vec::new();
        
        for entry in std::fs::read_dir(&self.local_backup_path)? {
            let entry = entry?;
            if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(name) = entry.path().file_name().and_then(|s| s.to_str()) {
                    if name.starts_with("soul_backup_") {
                        backup_files.push((entry.path(), entry.metadata()?.modified()?));
                    }
                }
            }
        }
        
        // Sort by modification time (newest first)
        backup_files.sort_by_key(|(_, modified)| std::cmp::Reverse(*modified));
        
        // Remove old backups beyond the configured limit
        if backup_files.len() > self.config.backup_generations as usize {
            for (path, _) in backup_files.iter().skip(self.config.backup_generations as usize) {
                std::fs::remove_file(path)?;
            }
        }
        
        Ok(())
    }

    async fn check_and_recover_corruption(&self) -> Result<()> {
        // Update corruption check timestamp
        {
            let mut state = self.sync_state.lock().unwrap();
            state.last_corruption_check = Some(Utc::now());
        }

        // Check if we have a recent snapshot
        let last_snapshot = {
            let snapshot_lock = self.last_snapshot.lock().unwrap();
            snapshot_lock.clone()
        };

        if let Some(snapshot) = last_snapshot {
            // Verify the snapshot integrity
            let serialized_data = serde_json::to_string(&snapshot.learning_data)?;
            let current_checksum = self.calculate_checksum(&serialized_data);
            
            if snapshot.checksum != current_checksum {
                eprintln!("🚨 Soul Memory corruption detected! Attempting recovery...");
                
                if self.config.auto_recovery {
                    self.attempt_auto_recovery().await?;
                } else {
                    return Err(anyhow!("Data corruption detected but auto-recovery is disabled"));
                }
            }
        }

        Ok(())
    }

    async fn attempt_auto_recovery(&self) -> Result<()> {
        // Try to recover from local backups first
        if let Ok(snapshot) = self.load_latest_local_backup().await {
            println!("🔧 Recovered from local backup (generation {})", snapshot.generation);
            
            {
                let mut last_snapshot = self.last_snapshot.lock().unwrap();
                *last_snapshot = Some(snapshot);
            }
            
            {
                let mut state = self.sync_state.lock().unwrap();
                state.recovery_count += 1;
            }
            
            return Ok(());
        }

        // If local recovery fails, try downloading from cloud
        // This would require maintaining a list of snapshot IDs
        // For now, we'll just clear the corrupted state
        println!("⚠️ Could not recover from corruption, clearing corrupted state");
        
        {
            let mut last_snapshot = self.last_snapshot.lock().unwrap();
            *last_snapshot = None;
        }
        
        Ok(())
    }

    async fn load_latest_local_backup(&self) -> Result<SoulSnapshot> {
        let mut backup_files = Vec::new();
        
        for entry in std::fs::read_dir(&self.local_backup_path)? {
            let entry = entry?;
            if entry.path().extension().and_then(|s| s.to_str()) == Some("json") {
                if let Some(name) = entry.path().file_name().and_then(|s| s.to_str()) {
                    if name.starts_with("soul_backup_") {
                        backup_files.push((entry.path(), entry.metadata()?.modified()?));
                    }
                }
            }
        }
        
        if backup_files.is_empty() {
            return Err(anyhow!("No local backups found"));
        }
        
        // Sort by modification time (newest first)
        backup_files.sort_by_key(|(_, modified)| std::cmp::Reverse(*modified));
        
        let (latest_path, _) = &backup_files[0];
        let backup_data = std::fs::read_to_string(latest_path)?;
        let snapshot: SoulSnapshot = serde_json::from_str(&backup_data)?;
        
        Ok(snapshot)
    }

    async fn load_local_state(&self) -> Result<()> {
        let state_file = self.local_backup_path.join("soul_state.json");
        
        if state_file.exists() {
            let state_data = std::fs::read_to_string(&state_file)?;
            let loaded_state: SoulSyncState = serde_json::from_str(&state_data)?;
            
            {
                let mut state = self.sync_state.lock().unwrap();
                *state = loaded_state;
            }
        }
        
        Ok(())
    }

    async fn save_local_state(&self) -> Result<()> {
        let state_file = self.local_backup_path.join("soul_state.json");
        
        let state = {
            let state_lock = self.sync_state.lock().unwrap();
            state_lock.clone()
        };
        
        let state_json = serde_json::to_string_pretty(&state)?;
        std::fs::write(&state_file, state_json)?;
        
        Ok(())
    }

    pub async fn force_sync(&self, learning_system: &Arc<Mutex<AdvancedLearningSystem>>) -> Result<()> {
        println!("🔄 Forcing Soul Memory sync...");
        self.perform_sync(learning_system).await
    }

    pub async fn get_sync_status(&self) -> SoulSyncState {
        let state = self.sync_state.lock().unwrap();
        state.clone()
    }

    async fn increment_sync_failures(&self) {
        let mut state = self.sync_state.lock().unwrap();
        state.sync_failures += 1;
    }

    async fn reset_sync_failures(&self) {
        let mut state = self.sync_state.lock().unwrap();
        state.sync_failures = 0;
    }

    fn calculate_checksum(&self, data: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }

    fn compress_data(&self, data: &str) -> Result<String> {
        // Simple compression using base64 encoding for now
        // In a real implementation, you'd use actual compression like gzip
        Ok(base64::encode(data))
    }

    fn decompress_data(&self, data: &str) -> Result<String> {
        // Corresponding decompression
        let decoded = base64::decode(data)
            .map_err(|e| anyhow!("Failed to decode compressed data: {}", e))?;
        String::from_utf8(decoded)
            .map_err(|e| anyhow!("Failed to convert decoded data to string: {}", e))
    }
}

impl Clone for SoulMemory {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            sync_state: self.sync_state.clone(),
            client: self.client.clone(),
            is_syncing: self.is_syncing.clone(),
            last_snapshot: self.last_snapshot.clone(),
            local_backup_path: self.local_backup_path.clone(),
        }
    }
}

// Integration with the existing learning system
impl SoulMemory {
    pub async fn integrate_with_learning_system(&self, learning_system: &mut AdvancedLearningSystem) -> Result<()> {
        // Try to restore from the most recent snapshot
        if let Ok(snapshot) = self.load_latest_local_backup().await {
            println!("🧠 Restoring Soul Memory from backup (generation {})", snapshot.generation);
            learning_system.writing_memory = snapshot.learning_data;
        }
        
        Ok(())
    }
}