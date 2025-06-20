use std::process::{Command, Stdio};
use std::fs;
use tempfile::NamedTempFile;
use anyhow::{Result, Context};
use std::io::Write;

pub async fn execute_python_script(script: &str, folder_path: &str) -> Result<String> {
    // Create a temporary Python file
    let mut temp_file = NamedTempFile::new()
        .context("Failed to create temporary file")?;
    
    // Write the script with folder path change at the beginning
    let full_script = format!(
        r#"import os
import sys
from pathlib import Path

# Change to target directory
target_dir = r"{}"
try:
    os.chdir(target_dir)
    print(f"Working in: {{os.getcwd()}}")
except Exception as e:
    print(f"Error changing to directory: {{e}}")
    sys.exit(1)

# User's generated script
{}
"#,
        folder_path, script
    );
    
    temp_file.write_all(full_script.as_bytes())
        .context("Failed to write script to temporary file")?;
    
    let temp_path = temp_file.path();
    
    // Try to execute with different Python commands
    let python_commands = ["python", "python3", "py"];
    let mut last_error = None;
    
    for python_cmd in &python_commands {
        let output = Command::new(python_cmd)
            .arg(temp_path)
            .current_dir(folder_path)
            .stdin(Stdio::null())  // Explicitly set stdin to null
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();
            
        match output {
            Ok(output) => {
                // Collect and return both stdout and stderr
                let mut result = String::new();
                
                if !output.stdout.is_empty() {
                    let stdout_str = String::from_utf8_lossy(&output.stdout);
                    println!("{}", stdout_str);
                    result.push_str(&stdout_str);
                }
                
                if !output.stderr.is_empty() {
                    let stderr_str = String::from_utf8_lossy(&output.stderr);
                    if !output.status.success() {
                        eprintln!("Python stderr: {}", stderr_str);
                        return Err(anyhow::anyhow!("Python script failed: {}", stderr_str));
                    } else {
                        // Sometimes stderr contains warnings or info, not errors
                        println!("Python info: {}", stderr_str);
                        if !result.is_empty() {
                            result.push_str("\n");
                        }
                        result.push_str(&stderr_str);
                    }
                }
                
                if output.status.success() {
                    return Ok(result);
                } else {
                    last_error = Some(anyhow::anyhow!("Python script execution failed with exit code: {}", 
                        output.status.code().unwrap_or(-1)));
                }
            }
            Err(e) => {
                last_error = Some(anyhow::anyhow!("Failed to run {}: {}", python_cmd, e));
                continue;
            }
        }
    }
    
    // If we get here, all Python commands failed
    Err(last_error.unwrap_or_else(|| anyhow::anyhow!("No Python interpreter found. Please install Python and ensure it's in your PATH.")))
}