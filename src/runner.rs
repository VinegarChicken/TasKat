use std::process::{Command, Stdio};
use std::fs;
use tempfile::NamedTempFile;
use anyhow::{Result, Context};
use std::io::{self, Write};
use regex::Regex;
use std::collections::HashSet;
use crate::gemini::validate_script_safety;
use colored::*;  // Add this import at the top

pub struct ExecutionResult {
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
}

// In the execute_python_script function:
pub async fn execute_python_script(script: &str, folder_path: &str, ask_permission: bool) -> Result<ExecutionResult> {
    // Step 1: Safety validation with Gemini
    println!("{}", "🔍 Validating script safety...".blue());
    match validate_script_safety(script).await {
        Ok((is_safe, validation_message)) => {
            // Format the validation message with colors
            let formatted_message = validation_message
                .replace("create", &"create".green().to_string())
                .replace("delete", &"delete".red().to_string())
                .replace("modify", &"modify".yellow().to_string())
                .replace("run command", &"run command".magenta().to_string())
                .replace("SAFE", &"SAFE".green().bold().to_string())
                .replace("CAUTION", &"CAUTION".red().bold().to_string());
            
            println!("{}", formatted_message);
            
            let prompt_color = if is_safe { "green" } else { "yellow" };
            print!("{}", format!("Continue? (yes/no): ").color(prompt_color));
            io::stdout().flush().unwrap();
            
            let mut response = String::new();
            io::stdin().read_line(&mut response).unwrap();
            
            if !response.trim().eq_ignore_ascii_case("yes") {
                return Ok(ExecutionResult {
                    success: true,
                    output: "Script execution cancelled by user after safety review.".to_string(),
                    error: None,
                });
            }
        },
        Err(e) => {
            println!("{}", format!("⚠️ Could not validate script safety: {}", e).yellow());
            print!("{}", "Continue anyway? (yes/no): ".yellow());
            io::stdout().flush().unwrap();
            
            let mut response = String::new();
            io::stdin().read_line(&mut response).unwrap();
            
            if !response.trim().eq_ignore_ascii_case("yes") {
                return Ok(ExecutionResult {
                    success: true,
                    output: "Script execution cancelled due to safety validation failure.".to_string(),
                    error: None,
                });
            }
        }
    }

    // Step 2: Check for file deletion operations (existing permission logic)
    if ask_permission && (script.contains("os.remove") || script.contains("os.unlink") || 
                         script.contains("shutil.rmtree") || script.contains("pathlib") && script.contains(".unlink") ||
                         script.contains("Path(") && script.contains(".unlink")) {
        print!("This script will delete or modify files. Do you want to continue? (yes/y/no/n): ");
        io::stdout().flush().unwrap();
        
        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap();
        
        print!("{}", "Continue with execution? (yes/y/no/n): ".yellow());
        io::stdout().flush().unwrap();
        
        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap();
        
        if !(response.trim().eq_ignore_ascii_case("yes") || response.trim().eq_ignore_ascii_case("y")) {
            return Ok(ExecutionResult {
                success: true,
                output: "Operation cancelled by user.".to_string(),
                error: None,
            });
        }
    }

    // First, check and install required modules
    let required_modules = extract_import_modules(script);
    if !required_modules.is_empty() {
        println!("🔍 Checking required Python modules: {:?}", required_modules);
        install_missing_modules(&required_modules).await?;
    }

    // Create a temporary Python file
    let mut temp_file = NamedTempFile::new()
        .context("Failed to create temporary file")?;
    
    // Write the script with folder path change at the beginning and Unicode handling
    let full_script = format!(
        r#"import os
import sys
from pathlib import Path

# Fix Unicode encoding issues on Windows
if os.name == 'nt':  # Windows
    import codecs
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')
    sys.stderr.reconfigure(encoding='utf-8', errors='replace')

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
        let mut cmd = Command::new(python_cmd);
        cmd.arg(temp_path)
            .current_dir(folder_path)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());
        
        // Set UTF-8 environment variables for better Unicode handling
        cmd.env("PYTHONIOENCODING", "utf-8");
        cmd.env("PYTHONUTF8", "1");
        
        let output = cmd.output();
            
        match output {
            Ok(output) => {
                let stdout_str = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr_str = String::from_utf8_lossy(&output.stderr).to_string();
                
                if output.status.success() {
                    // Success case
                    if !stdout_str.is_empty() {
                        println!("{}", stdout_str);
                    }
                    if !stderr_str.is_empty() {
                        println!("Python info: {}", stderr_str);
                    }
                    
                    return Ok(ExecutionResult {
                        success: true,
                        output: format!("{}{}", stdout_str, if stderr_str.is_empty() { String::new() } else { format!("\n{}", stderr_str) }),
                        error: None,
                    });
                } else {
                    // Execution failed - check if it's a recoverable error
                    let error_message = if !stderr_str.is_empty() {
                        stderr_str.clone()
                    } else {
                        stdout_str.clone()
                    };
                    
                    println!("❌ Python execution failed:");
                    println!("{}", error_message);
                    
                    return Ok(ExecutionResult {
                        success: false,
                        output: stdout_str,
                        error: Some(error_message),
                    });
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

fn extract_import_modules(script: &str) -> Vec<String> {
    let mut modules = HashSet::new();
    
    // Regex patterns for different import styles
    let import_patterns = vec![
        Regex::new(r"^import\s+([a-zA-Z_][a-zA-Z0-9_]*(?:\.[a-zA-Z_][a-zA-Z0-9_]*)*)").unwrap(),
        Regex::new(r"^from\s+([a-zA-Z_][a-zA-Z0-9_]*(?:\.[a-zA-Z_][a-zA-Z0-9_]*)*)\s+import").unwrap(),
    ];
    
    for line in script.lines() {
        let line = line.trim();
        for pattern in &import_patterns {
            if let Some(captures) = pattern.captures(line) {
                if let Some(module_match) = captures.get(1) {
                    let module_name = module_match.as_str();
                    // Get the root module name (before any dots)
                    let root_module = module_name.split('.').next().unwrap_or(module_name);
                    
                    // Skip built-in modules
                    if !is_builtin_module(root_module) {
                        modules.insert(root_module.to_string());
                    }
                }
            }
        }
    }
    
    modules.into_iter().collect()
}

fn is_builtin_module(module: &str) -> bool {
    // Common built-in Python modules that don't need installation
    let builtins = vec![
        "os", "sys", "re", "json", "datetime", "time", "random", "math", 
        "collections", "itertools", "functools", "pathlib", "glob", "shutil",
        "subprocess", "threading", "multiprocessing", "asyncio", "logging",
        "urllib", "http", "email", "csv", "xml", "html", "base64", "hashlib",
        "hmac", "secrets", "uuid", "pickle", "sqlite3", "tkinter", "argparse",
        "configparser", "io", "tempfile", "gzip", "zipfile", "tarfile"
    ];
    
    builtins.contains(&module)
}

// Add this function to runner.rs to map import names to correct package names
fn get_correct_package_name(module: &str) -> &str {
    match module {
        "docx" => "python-docx",  // Map docx import to python-docx package
        "PIL" => "Pillow",        // Map PIL import to Pillow package
        "cv2" => "opencv-python", // Map cv2 import to opencv-python package
        "skimage" => "scikit-image", // Map skimage import to scikit-image package
        "sklearn" => "scikit-learn", // Map sklearn import to scikit-learn package
        "yaml" => "PyYAML",       // Map yaml import to PyYAML package
        _ => module,              // Return original name for other modules
    }
}

// Update the install_missing_modules function to use the mapping
async fn install_missing_modules(modules: &[String]) -> Result<()> {
    let python_commands = ["python", "python3", "py"];
    let mut python_cmd = None;
    
    // Find available Python command
    for cmd in &python_commands {
        if Command::new(cmd).arg("--version").output().is_ok() {
            python_cmd = Some(*cmd);
            break;
        }
    }
    
    let python_cmd = python_cmd.ok_or_else(|| anyhow::anyhow!("No Python interpreter found"))?;
    
    for module in modules {
        // Get the correct package name for installation
        let package_name = get_correct_package_name(module);
        
        // Check if module is already installed (check the import name, not package name)
        let check_output = Command::new(python_cmd)
            .arg("-c")
            .arg(&format!("import {}", module))
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
            
        match check_output {
            Ok(status) if status.success() => {
                println!("✅ Module '{}' is already installed", module);
                continue;
            },
            _ => {
                // If the package name is different from module name, show both
                if package_name != module {
                    println!("📦 Installing '{}' package for '{}' module", package_name, module);
                } else {
                    println!("📦 Installing missing module: {}", module);
                }
                
                // Try to install the correct package
                let install_output = Command::new(python_cmd)
                    .args(&["-m", "pip", "install", package_name])
                    .stdin(Stdio::null())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .output();
                    
                match install_output {
                    Ok(output) if output.status.success() => {
                        println!("✅ Successfully installed '{}' for module '{}'", package_name, module);
                    },
                    Ok(output) => {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        println!("⚠️  Warning: Failed to install '{}': {}", package_name, stderr);
                        println!("   The script may still work if the module is available through other means.");
                    },
                    Err(e) => {
                        println!("⚠️  Warning: Could not install '{}': {}", package_name, e);
                        println!("   Please install it manually if the script fails.");
                    }
                }
            }
        }
    }
    
    Ok(())
}

pub fn is_recoverable_error(error: &str) -> bool {
    let recoverable_patterns = vec![
        "NameError:",
        "TypeError:",
        "AttributeError:",
        "ValueError:",
        "KeyError:",
        "IndexError:",
        "SyntaxError:",
        "IndentationError:",
        "UnboundLocalError:",
        "FileNotFoundError:",
        "PermissionError:",
        "ImportError:",
        "ModuleNotFoundError:",
        "UnicodeEncodeError:",  // Added this as recoverable
        "UnicodeDecodeError:",  // Added this as recoverable
    ];
    
    recoverable_patterns.iter().any(|pattern| error.contains(pattern))
}

pub fn extract_error_details(error: &str) -> String {
    // Extract the most relevant error information
    let lines: Vec<&str> = error.lines().collect();
    
    // Find the actual error line (usually the last line with "Error:")
    for line in lines.iter().rev() {
        if line.contains("Error:") {
            return line.trim().to_string();
        }
    }
    
    // If no specific error line found, return the last few lines
    if lines.len() > 3 {
        lines[lines.len()-3..].join("\n")
    } else {
        error.to_string()
    }
}