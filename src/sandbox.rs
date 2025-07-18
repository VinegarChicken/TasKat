use std::fs;
use std::path::{Path, PathBuf};
use tempfile::TempDir;
use anyhow::Result;
use crate::runner::ExecutionResult;

// Add missing types
#[derive(Debug)]
pub struct ChangePreview {
    pub created_files: Vec<PathBuf>,
    pub modified_files: Vec<PathBuf>,
    pub deleted_files: Vec<PathBuf>,
}

#[derive(Debug)]
pub struct DryRunResult {
    pub diff: String,
    pub would_create: Vec<PathBuf>,
    pub would_modify: Vec<PathBuf>,
    pub would_delete: Vec<PathBuf>,
}

impl DryRunResult {
    pub fn format_preview(&self) -> String {
        let mut preview = String::new();
        preview.push_str("\n📋 Dry Run Preview:\n");
        preview.push_str("==================\n");
        
        if !self.would_create.is_empty() {
            preview.push_str("\n✅ Files to be created:\n");
            for file in &self.would_create {
                preview.push_str(&format!("  + {}\n", file.display()));
            }
        }
        
        if !self.would_modify.is_empty() {
            preview.push_str("\n📝 Files to be modified:\n");
            for file in &self.would_modify {
                preview.push_str(&format!("  ~ {}\n", file.display()));
            }
        }
        
        if !self.would_delete.is_empty() {
            preview.push_str("\n❌ Files to be deleted:\n");
            for file in &self.would_delete {
                preview.push_str(&format!("  - {}\n", file.display()));
            }
        }
        
        if !self.diff.is_empty() {
            preview.push_str("\n📄 Detailed Changes:\n");
            preview.push_str(&self.diff);
        }
        
        preview
    }
}

pub struct SandboxEnvironment {
    pub temp_dir: TempDir,
    pub sandbox_path: PathBuf,
    pub original_path: PathBuf,
}

impl SandboxEnvironment {
    pub fn new(original_path: &str) -> Result<Self> {
        let temp_dir = tempfile::tempdir()?;
        let sandbox_path = temp_dir.path().to_path_buf();
        
        // Copy entire directory structure
        copy_dir_recursive(Path::new(original_path), &sandbox_path)?;
        
        Ok(SandboxEnvironment {
            temp_dir,
            sandbox_path,
            original_path: PathBuf::from(original_path),
        })
    }
    
    pub async fn execute_script(&self, script: &str) -> Result<ExecutionResult> {
        // Execute script in sandbox environment
        crate::runner::execute_python_script(script, self.sandbox_path.to_str().unwrap(), false).await
    }
    
    pub fn preview_changes(&self) -> Result<ChangePreview> {
        // Compare sandbox with original to show what would change
        compare_directories(&self.original_path, &self.sandbox_path)
    }
    
    pub fn apply_changes(&self) -> Result<()> {
        // Copy changes from sandbox back to original
        sync_directories(&self.sandbox_path, &self.original_path)
    }
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

// Add missing functions
pub fn copy_directory_structure(src: &str, dst: &str) -> Result<()> {
    copy_dir_recursive(Path::new(src), Path::new(dst))
}

pub async fn execute_python_script_in_sandbox(script: &str, sandbox_path: &str) -> Result<ExecutionResult> {
    crate::runner::execute_python_script(script, sandbox_path, false).await
}

pub fn compare_directories(original: &Path, modified: &Path) -> Result<ChangePreview> {
    let mut created_files = Vec::new();
    let mut modified_files = Vec::new();
    let mut deleted_files = Vec::new();
    
    // Walk through modified directory to find new and changed files
    if modified.exists() {
        for entry in walkdir::WalkDir::new(modified) {
            let entry = entry?;
            if entry.file_type().is_file() {
                let relative_path = entry.path().strip_prefix(modified)?;
                let original_file = original.join(relative_path);
                
                if !original_file.exists() {
                    created_files.push(relative_path.to_path_buf());
                } else {
                    // Check if file was modified (simple size/time check)
                    let original_meta = fs::metadata(&original_file)?;
                    let modified_meta = fs::metadata(entry.path())?;
                    
                    if original_meta.len() != modified_meta.len() ||
                       original_meta.modified()? != modified_meta.modified()? {
                        modified_files.push(relative_path.to_path_buf());
                    }
                }
            }
        }
    }
    
    // Walk through original directory to find deleted files
    if original.exists() {
        for entry in walkdir::WalkDir::new(original) {
            let entry = entry?;
            if entry.file_type().is_file() {
                let relative_path = entry.path().strip_prefix(original)?;
                let modified_file = modified.join(relative_path);
                
                if !modified_file.exists() {
                    deleted_files.push(relative_path.to_path_buf());
                }
            }
        }
    }
    
    Ok(ChangePreview {
        created_files,
        modified_files,
        deleted_files,
    })
}

pub fn sync_directories(src: &Path, dst: &Path) -> Result<()> {
    // Remove destination if it exists
    if dst.exists() {
        fs::remove_dir_all(dst)?;
    }
    
    // Copy source to destination
    copy_dir_recursive(src, dst)
}

pub fn generate_git_style_diff(original: &str, modified: &str) -> Result<String> {
    let mut diff = String::new();
    
    // Simple diff implementation - in production you'd want to use a proper diff library
    diff.push_str(&format!("diff --git a/{} b/{}\n", original, modified));
    diff.push_str(&format!("--- a/{}\n", original));
    diff.push_str(&format!("+++ b/{}\n", modified));
    
    // For now, just show file-level changes
    let changes = compare_directories(Path::new(original), Path::new(modified))?;
    
    for file in &changes.created_files {
        diff.push_str(&format!("+ Created: {}\n", file.display()));
    }
    
    for file in &changes.modified_files {
        diff.push_str(&format!("~ Modified: {}\n", file.display()));
    }
    
    for file in &changes.deleted_files {
        diff.push_str(&format!("- Deleted: {}\n", file.display()));
    }
    
    Ok(diff)
}

// Add dry run execution function
pub async fn execute_dry_run(script: &str, folder_path: &str) -> Result<DryRunResult> {
    // Create temporary sandbox directory
    let temp_dir = tempfile::tempdir()?;
    let sandbox_path = temp_dir.path();
    
    // Copy folder structure to sandbox
    copy_directory_structure(folder_path, sandbox_path.to_str().unwrap())?;
    
    // Execute script in sandbox
    let _result = execute_python_script_in_sandbox(script, sandbox_path.to_str().unwrap()).await?;
    
    // Generate diff between original and sandbox
    let diff = generate_git_style_diff(folder_path, sandbox_path.to_str().unwrap())?;
    let changes = compare_directories(Path::new(folder_path), sandbox_path)?;
    
    Ok(DryRunResult {
        diff,
        would_create: changes.created_files,
        would_modify: changes.modified_files,
        would_delete: changes.deleted_files,
    })
}