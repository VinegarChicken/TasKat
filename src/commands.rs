use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use chrono::Local;
use anyhow::{Result, Context};
use colored::Colorize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    pub name: String,
    pub description: String,
    pub script: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CommandRegistry {
    commands: HashMap<String, Command>,
}

impl CommandRegistry {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn load_from_file(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::new());
        }

        let mut file = File::open(path)
            .context(format!("Failed to open command registry at {:?}", path))?;
        
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .context("Failed to read command registry file")?;
        
        let registry: CommandRegistry = serde_json::from_str(&contents)
            .context("Failed to parse command registry JSON")?;
        
        Ok(registry)
    }

    pub fn save_to_file(&self, path: &Path) -> Result<()> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .context(format!("Failed to create directory at {:?}", parent))?;
        }

        let json = serde_json::to_string_pretty(self)
            .context("Failed to serialize command registry")?;
        
        let mut file = File::create(path)
            .context(format!("Failed to create command registry file at {:?}", path))?;
        
        file.write_all(json.as_bytes())
            .context("Failed to write command registry to file")?;
        
        Ok(())
    }

    pub fn add_command(&mut self, name: String, description: String, script: String) -> Result<()> {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        
        let command = Command {
            name: name.clone(),
            description,
            script,
            created_at: timestamp,
        };
        
        self.commands.insert(name, command);
        Ok(())
    }

    pub fn get_command(&self, name: &str) -> Option<&Command> {
        self.commands.get(name)
    }

    pub fn list_commands(&self) -> Vec<&Command> {
        self.commands.values().collect()
    }

    pub fn generate_description_for_script(&self, script: &str) -> String {
        // Simple description generation based on script content
        // In a real implementation, you might want to use AI to generate this
        let first_line = script.lines().next().unwrap_or("");
        if first_line.starts_with("#") {
            // Use comment as description
            return first_line.trim_start_matches("#").trim().to_string();
        }
        
        "A custom command created with PromptFile".to_string()
    }
}

pub fn get_config_path() -> PathBuf {
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let desktop_dir = home_dir.join("Desktop");
    let config_dir = desktop_dir.join("PromptFile");
    config_dir.join("config.json")
}

pub fn is_command(input: &str) -> bool {
    input.starts_with("#") && !input.contains(" ")
}

pub fn extract_command_name(input: &str) -> &str {
    input.trim_start_matches("#")
}

pub fn display_help() -> String {
    format!(
        r#"{}
{}

{}
  {} - Show this help message
  {} - Save the last generated script as a reusable command
  {} - Undo the last operation (rollback to most recent snapshot)
  {} - List all available commands

{}
  Example: {} "Create a backup of all .txt files"
  Example: {} "Organize photos by date"

{}
1. Type your prompt and get a response
2. Type {} to save the last prompt as a command
3. You'll be asked for a command name and optional description

{}
- Type {} to execute a saved command
- Commands are saved in Desktop/PromptFile/config.json

{}
- "Create a file that lists all files in the current directory" (regular prompt)
- "{}" (save the previous prompt as a command)
- "{}" (execute the saved command)
"#,
        "📋 TasCat Help".cyan().bold(),
        "==============".cyan(),
        "Built-in Commands:".green().bold(),
        "#help".cyan(),
        "#save".cyan(),
        "#undo".cyan(),
        "#list".cyan(),
        "Usage:".green().bold(),
        "tascat".yellow(),
        "tascat \"prompt\"".yellow(),
        "Creating Commands:".green().bold(),
        "#save".cyan(),
        "Using Commands:".green().bold(),
        "#commandname".cyan(),
        "Examples:".green().bold(),
        "#save".cyan(),
        "#listfiles".cyan()
    )
}
