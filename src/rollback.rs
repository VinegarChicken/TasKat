use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::fs;
use anyhow::Result;
use sha2::{Sha256, Digest};
use uuid::Uuid;
use walkdir::WalkDir;

#[cfg(windows)]
use winapi::um::winbase::CreateHardLinkW;

pub struct RollbackManager {
    snapshots: HashMap<String, SnapshotInfo>,
    snapshot_dir: PathBuf,
}

#[derive(Debug)]
pub struct SnapshotInfo {
    pub id: String,
    pub timestamp: chrono::DateTime<chrono::Local>,
    pub original_path: PathBuf,
    pub snapshot_path: PathBuf,
    pub file_hashes: HashMap<PathBuf, String>,
}

impl RollbackManager {
    pub fn new() -> Result<Self> {
        let snapshot_dir = dirs::cache_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("TasCat")
            .join("snapshots");
        
        fs::create_dir_all(&snapshot_dir)?;
        
        Ok(RollbackManager {
            snapshots: HashMap::new(),
            snapshot_dir,
        })
    }
    
    pub fn create_snapshot(&mut self, folder_path: &str) -> Result<String> {
        let snapshot_id = generate_snapshot_id();
        let snapshot_path = self.snapshot_dir.join(&snapshot_id);
        
        #[cfg(windows)]
        {
            // Use Volume Shadow Copy Service (VSS) for Windows
            self.create_vss_snapshot(folder_path, &snapshot_path)?;
        }
        
        #[cfg(unix)]
        {
            // Use hard links for POSIX systems
            self.create_hardlink_snapshot(folder_path, &snapshot_path)?;
        }
        
        let snapshot_info = SnapshotInfo {
            id: snapshot_id.clone(),
            timestamp: chrono::Local::now(),
            original_path: PathBuf::from(folder_path),
            snapshot_path,
            file_hashes: self.calculate_file_hashes(folder_path)?,
        };
        
        self.snapshots.insert(snapshot_id.clone(), snapshot_info);
        Ok(snapshot_id)
    }
    
    pub fn rollback(&self, snapshot_id: &str) -> Result<()> {
        let snapshot = self.snapshots.get(snapshot_id)
            .ok_or_else(|| anyhow::anyhow!("Snapshot not found: {}", snapshot_id))?;
        
        // Restore files from snapshot
        self.restore_from_snapshot(snapshot)?;
        
        println!("✅ Successfully rolled back to snapshot: {}", snapshot_id);
        Ok(())
    }
    
    pub fn rollback_to_snapshot(&self, snapshot_id: &str) -> Result<()> {
        let snapshot = self.snapshots.get(snapshot_id)
            .ok_or_else(|| anyhow::anyhow!("Snapshot not found: {}", snapshot_id))?;
        
        self.restore_from_snapshot(snapshot)?;
        println!("✅ Successfully rolled back to snapshot from {}", 
                 snapshot.timestamp.format("%Y-%m-%d %H:%M:%S"));
        Ok(())
    }
    
    // Add missing method implementations
    pub fn calculate_file_hashes(&self, folder_path: &str) -> Result<HashMap<PathBuf, String>> {
        let mut hashes = HashMap::new();
        
        for entry in WalkDir::new(folder_path) {
            let entry = entry?;
            if entry.file_type().is_file() {
                let file_path = entry.path();
                let relative_path = file_path.strip_prefix(folder_path)?;
                
                let content = fs::read(file_path)?;
                let mut hasher = Sha256::new();
                hasher.update(&content);
                let hash = format!("{:x}", hasher.finalize());
                
                hashes.insert(relative_path.to_path_buf(), hash);
            }
        }
        
        Ok(hashes)
    }
    
    pub fn restore_from_snapshot(&self, snapshot: &SnapshotInfo) -> Result<()> {
        // Remove current directory contents
        if snapshot.original_path.exists() {
            for entry in fs::read_dir(&snapshot.original_path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    fs::remove_dir_all(&path)?;
                } else {
                    fs::remove_file(&path)?;
                }
            }
        }
        
        // Copy snapshot contents back to original location
        crate::sandbox::copy_directory_structure(
            snapshot.snapshot_path.to_str().unwrap(),
            snapshot.original_path.to_str().unwrap()
        )?;
        
        Ok(())
    }
    
    #[cfg(windows)]
    fn create_vss_snapshot(&self, source: &str, target: &Path) -> Result<()> {
        // For now, use regular copy - VSS implementation would require more complex Windows APIs
        crate::sandbox::copy_directory_structure(source, target.to_str().unwrap())
    }
    
    #[cfg(unix)]
    fn create_hardlink_snapshot(&self, source: &str, target: &Path) -> Result<()> {
        fs::create_dir_all(target)?;
        
        for entry in walkdir::WalkDir::new(source) {
            let entry = entry?;
            let relative_path = entry.path().strip_prefix(source)?;
            let target_path = target.join(relative_path);
            
            if entry.path().is_file() {
                if let Some(parent) = target_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                fs::hard_link(entry.path(), target_path)?;
            }
        }
        Ok(())
    }
    
    pub fn get_most_recent_snapshot(&self, folder_path: &str) -> Option<&SnapshotInfo> {
        self.snapshots
            .values()
            .filter(|snapshot| snapshot.original_path == PathBuf::from(folder_path))
            .max_by_key(|snapshot| snapshot.timestamp)
    }
    
    pub fn list_snapshots(&self, folder_path: &str) -> Vec<&SnapshotInfo> {
        let mut snapshots: Vec<&SnapshotInfo> = self.snapshots
            .values()
            .filter(|snapshot| snapshot.original_path == PathBuf::from(folder_path))
            .collect();
        
        // Sort by timestamp, most recent first
        snapshots.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        snapshots
    }
    
    pub fn rollback_to_most_recent(&self, folder_path: &str) -> Result<()> {
        if let Some(snapshot) = self.get_most_recent_snapshot(folder_path) {
            self.restore_from_snapshot(snapshot)?;
            println!("✅ Successfully rolled back to most recent snapshot from {}", 
                     snapshot.timestamp.format("%Y-%m-%d %H:%M:%S"));
            Ok(())
        } else {
            Err(anyhow::anyhow!("No snapshots found for this folder"))
        }
    }
}

// Add missing function
pub fn generate_snapshot_id() -> String {
    Uuid::new_v4().to_string()
}

// Add execute_with_rollback function
pub async fn execute_with_rollback(
    script: &str,
    folder_path: &str,
    snapshot_id: &str,
) -> Result<crate::runner::ExecutionResult> {
    // Execute the script normally
    crate::runner::execute_python_script(script, folder_path, false).await
}