use std::env;
use winreg::enums::*;
use winreg::RegKey;
use anyhow::{Result, Context};

pub fn setup_registry() -> Result<()> {
    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
    
    // Get current executable path
    let exe_path = env::current_exe()
        .context("Failed to get current executable path")?;
    let exe_path_str = exe_path.to_string_lossy().to_string();
    
    // Create the registry path for Directory context menu: HKEY_CLASSES_ROOT\Directory\shell\PromptFile
    let directory_key = hkcr.open_subkey_with_flags("Directory", KEY_ALL_ACCESS)
        .context("Failed to open Directory key")?;
    
    let shell_key = directory_key.open_subkey_with_flags("shell", KEY_ALL_ACCESS)
        .context("Failed to open shell key")?;
    
    // Create or open PromptFile key for directories
    let (promptfile_key, _) = shell_key.create_subkey("PromptFile")
        .context("Failed to create PromptFile key")?;
    
    // Set the display name
    promptfile_key.set_value("", &"PromptFile Command")
        .context("Failed to set PromptFile display name")?;
        
    // Set the icon (optional - uses the executable's icon)
    promptfile_key.set_value("Icon", &exe_path_str)
        .context("Failed to set PromptFile icon")?;
    
    // Create the command subkey
    let (command_key, _) = promptfile_key.create_subkey("command")
        .context("Failed to create command key")?;
    
    // Set the command value with the full path to the executable
    let command_value = format!("\"{}\" prompt \"%1\"", exe_path_str);
    command_key.set_value("", &command_value)
        .context("Failed to set command value")?;

    // Also add to Directory\Background (right-click in empty space)
    let directory_background_key = hkcr.open_subkey_with_flags("Directory\\Background", KEY_ALL_ACCESS)
        .context("Failed to open Directory\\Background key")?;
    
    let bg_shell_key = directory_background_key.open_subkey_with_flags("shell", KEY_ALL_ACCESS)
        .context("Failed to open Directory\\Background\\shell key")?;
    
    // Create or open PromptFile key for background
    let (bg_promptfile_key, _) = bg_shell_key.create_subkey("PromptFile")
        .context("Failed to create Directory\\Background PromptFile key")?;
    
    // Set the display name
    bg_promptfile_key.set_value("", &"PromptFile Command")
        .context("Failed to set Directory\\Background PromptFile display name")?;
        
    // Set the icon
    bg_promptfile_key.set_value("Icon", &exe_path_str)
        .context("Failed to set Directory\\Background PromptFile icon")?;
    
    // Create the command subkey
    let (bg_command_key, _) = bg_promptfile_key.create_subkey("command")
        .context("Failed to create Directory\\Background command key")?;
    
    // Set the command value - %V gives the current directory
    let bg_command_value = format!("\"{}\" prompt \"%V\"", exe_path_str);
    bg_command_key.set_value("", &bg_command_value)
        .context("Failed to set Directory\\Background command value")?;
    
    Ok(())
}

pub fn uninstall_registry() -> Result<()> {
    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
    
    // Remove from Directory\shell
    let directory_key = hkcr.open_subkey_with_flags("Directory", KEY_ALL_ACCESS)
        .context("Failed to open Directory key")?;
    
    let shell_key = directory_key.open_subkey_with_flags("shell", KEY_ALL_ACCESS)
        .context("Failed to open shell key")?;
    
    // Delete the PromptFile key and all its subkeys
    if let Err(_) = shell_key.delete_subkey_all("PromptFile") {
        println!("Note: Directory\\shell\\PromptFile key not found or already removed");
    }

    // Remove from Directory\Background\shell
    let directory_background_key = hkcr.open_subkey_with_flags("Directory\\Background", KEY_ALL_ACCESS)
        .context("Failed to open Directory\\Background key")?;
    
    let bg_shell_key = directory_background_key.open_subkey_with_flags("shell", KEY_ALL_ACCESS)
        .context("Failed to open Directory\\Background\\shell key")?;
    
    // Delete the PromptFile key and all its subkeys
    if let Err(_) = bg_shell_key.delete_subkey_all("PromptFile") {
        println!("Note: Directory\\Background\\shell\\PromptFile key not found or already removed");
    }
    
    Ok(())
}