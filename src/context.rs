use std::collections::HashMap;
use std::path::Path;
use walkdir::WalkDir;
use anyhow::Result;

#[derive(Debug)]
pub struct FileContext {
    pub folder_path: String,
    pub total_files: usize,
    pub total_directories: usize,
    pub file_types: HashMap<String, usize>,
    pub sample_filenames: Vec<String>,
    pub directory_structure: Vec<String>,
    pub largest_files: Vec<(String, u64)>,
}

impl FileContext {
    pub fn to_string(&self) -> String {
        let mut context = String::new();
        
        context.push_str(&format!("FOLDER ANALYSIS: {}\n", self.folder_path));
        context.push_str(&format!("==================================================\n"));
        context.push_str(&format!("Total Files: {}\n", self.total_files));
        context.push_str(&format!("Total Directories: {}\n", self.total_directories));
        context.push_str("\n");
        
        // File types breakdown
        context.push_str("FILE TYPES:\n");
        let mut sorted_types: Vec<_> = self.file_types.iter().collect();
        sorted_types.sort_by(|a, b| b.1.cmp(a.1));
        for (ext, count) in sorted_types.iter().take(10) {
            context.push_str(&format!("  {}: {} files\n", ext, count));
        }
        context.push_str("\n");
        
        // Sample filenames
        if !self.sample_filenames.is_empty() {
            context.push_str("SAMPLE FILENAMES:\n");
            for filename in self.sample_filenames.iter().take(15) {
                context.push_str(&format!("  {}\n", filename));
            }
            context.push_str("\n");
        }
        
        // Directory structure
        if !self.directory_structure.is_empty() {
            context.push_str("DIRECTORY STRUCTURE:\n");
            for dir in self.directory_structure.iter().take(10) {
                context.push_str(&format!("  {}\n", dir));
            }
            context.push_str("\n");
        }
        
        // Largest files
        if !self.largest_files.is_empty() {
            context.push_str("LARGEST FILES:\n");
            for (filename, size) in self.largest_files.iter().take(5) {
                context.push_str(&format!("  {} ({} bytes)\n", filename, size));
            }
        }
        
        context
    }
}

pub fn gather_file_context(folder_path: &str) -> Result<FileContext> {
    let path = Path::new(folder_path);
    let mut total_files = 0;
    let mut total_directories = 0;
    let mut file_types: HashMap<String, usize> = HashMap::new();
    let mut sample_filenames = Vec::new();
    let mut directory_structure = Vec::new();
    let mut files_with_sizes = Vec::new();
    
    for entry in WalkDir::new(path).max_depth(3) {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            total_files += 1;
            
            // Track file extension
            let extension = path.extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("(no extension)")
                .to_lowercase();
            *file_types.entry(extension).or_insert(0) += 1;
            
            // Sample filenames (first 20)
            if sample_filenames.len() < 20 {
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    sample_filenames.push(filename.to_string());
                }
            }
            
            // Track file sizes for largest files
            if let Ok(metadata) = path.metadata() {
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    files_with_sizes.push((filename.to_string(), metadata.len()));
                }
            }
            
        } else if path.is_dir() && path != Path::new(folder_path) {
            total_directories += 1;
            
            // Track directory structure (relative paths)
            if let Ok(relative_path) = path.strip_prefix(folder_path) {
                if directory_structure.len() < 15 {
                    directory_structure.push(relative_path.to_string_lossy().to_string());
                }
            }
        }
    }
    
    // Sort files by size and keep top 5
    files_with_sizes.sort_by(|a, b| b.1.cmp(&a.1));
    let largest_files = files_with_sizes.into_iter().take(5).collect();
    
    Ok(FileContext {
        folder_path: folder_path.to_string(),
        total_files,
        total_directories,
        file_types,
        sample_filenames,
        directory_structure,
        largest_files,
    })
}