use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginTemplate {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub tags: Vec<String>,
    pub script_template: String,
    pub parameters: Vec<PluginParameter>,
    pub examples: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PluginParameter {
    pub name: String,
    pub description: String,
    pub param_type: String, // "string", "number", "boolean", "file", "directory"
    pub required: bool,
    pub default_value: Option<String>,
}

pub struct PluginManager {
    plugins: HashMap<String, PluginTemplate>,
    plugin_dir: PathBuf,
}

impl PluginManager {
    pub fn new() -> Result<Self> {
        let plugin_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("TasCat")
            .join("plugins");
        
        fs::create_dir_all(&plugin_dir)?;
        
        let mut manager = PluginManager {
            plugins: HashMap::new(),
            plugin_dir,
        };
        
        manager.load_plugins()?;
        Ok(manager)
    }
    
    pub fn load_plugins(&mut self) -> Result<()> {
        for entry in fs::read_dir(&self.plugin_dir)? {
            let entry = entry?;
            if entry.path().extension().and_then(|s| s.to_str()) == Some("toml") {
                let plugin = self.load_plugin_from_file(&entry.path())?;
                self.plugins.insert(plugin.name.clone(), plugin);
            }
        }
        Ok(())
    }
    
    fn load_plugin_from_file(&self, path: &Path) -> Result<PluginTemplate> {
        let content = fs::read_to_string(path)?;
        let plugin: PluginTemplate = toml::from_str(&content)?;
        Ok(plugin)
    }
    
    pub fn execute_plugin(&self, plugin_name: &str, params: &HashMap<String, String>) -> Result<String> {
        let plugin = self.plugins.get(plugin_name)
            .ok_or_else(|| anyhow::anyhow!("Plugin not found: {}", plugin_name))?;
        
        // Replace template variables with actual parameters
        let script = self.render_template(&plugin.script_template, params)?;
        Ok(script)
    }
    
    fn render_template(&self, template: &str, params: &HashMap<String, String>) -> Result<String> {
        let mut result = template.to_string();
        
        for (key, value) in params {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }
        
        Ok(result)
    }
    
    // Add method to handle commands
    pub fn handle_command(&self, command: &str) -> Result<Option<String>> {
        if !command.starts_with('#') {
            return Ok(None);
        }
        
        let command_name = command.trim_start_matches('#');
        let params = HashMap::new(); // For now, simple implementation
        
        if self.plugins.contains_key(command_name) {
            Ok(Some(self.execute_plugin(command_name, &params)?))
        } else {
            Ok(None)
        }
    }
}