use std::env;
use winreg::enums::*;
use winreg::RegKey;
use anyhow::{Result, Context};
use std::path::Path;

pub fn setup_registry() -> Result<()> {
    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
    
    // Get current executable path
    let exe_path = env::current_exe()
        .context("Failed to get current executable path")?;
    let exe_path_str = exe_path.to_string_lossy().to_string();
    
    // Get the icon path (TasKat.ico in the same directory as the executable)
    let exe_dir = exe_path.parent()
        .context("Failed to get executable directory")?;
    let icon_path = exe_dir.join("TasKat.ico");
    let icon_path_str = icon_path.to_string_lossy().to_string();
    
    // Create the registry path for Directory context menu: HKEY_CLASSES_ROOT\Directory\shell\TasKat
    let directory_key = hkcr.open_subkey_with_flags("Directory", KEY_ALL_ACCESS)
        .context("Failed to open Directory key")?;
    
    let shell_key = directory_key.open_subkey_with_flags("shell", KEY_ALL_ACCESS)
        .context("Failed to open shell key")?;
    
    // Create or open TasKat key for directories
    let (TasKat_key, _) = shell_key.create_subkey("TasKat")
        .context("Failed to create TasKat key")?;
    
    // Set the display name
    TasKat_key.set_value("", &"TasKat")
        .context("Failed to set TasKat display name")?;
        
    // Set the icon to use TasKat.ico if it exists, otherwise fall back to executable icon
    let icon_to_use = if Path::new(&icon_path_str).exists() {
        icon_path_str.clone()
    } else {
        exe_path_str.clone()
    };
    
    TasKat_key.set_value("Icon", &icon_to_use)
        .context("Failed to set TasKat icon")?;
    
    // Create the command subkey
    let (command_key, _) = TasKat_key.create_subkey("command")
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
    
    // Create or open TasKat key for background
    let (bg_TasKat_key, _) = bg_shell_key.create_subkey("TasKat")
        .context("Failed to create Directory\\Background TasKat key")?;
    
    // Set the display name
    bg_TasKat_key.set_value("", &"TasKat Command")
        .context("Failed to set Directory\\Background TasKat display name")?;
        
    // Set the icon to use TasKat.ico if it exists, otherwise fall back to executable icon
    bg_TasKat_key.set_value("Icon", &icon_to_use)
        .context("Failed to set Directory\\Background TasKat icon")?;
    
    // Create the command subkey
    let (bg_command_key, _) = bg_TasKat_key.create_subkey("command")
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
    
    // Delete the TasKat key and all its subkeys
    if let Err(_) = shell_key.delete_subkey_all("TasKat") {
        println!("Note: Directory\\shell\\TasKat key not found or already removed");
    }

    // Remove from Directory\Background\shell
    let directory_background_key = hkcr.open_subkey_with_flags("Directory\\Background", KEY_ALL_ACCESS)
        .context("Failed to open Directory\\Background key")?;
    
    let bg_shell_key = directory_background_key.open_subkey_with_flags("shell", KEY_ALL_ACCESS)
        .context("Failed to open Directory\\Background\\shell key")?;
    
    // Delete the TasKat key and all its subkeys
    if let Err(_) = bg_shell_key.delete_subkey_all("TasKat") {
        println!("Note: Directory\\Background\\shell\\TasKat key not found or already removed");
    }
    
    Ok(())
}

pub fn is_installed() -> bool {
    let hkcr = RegKey::predef(HKEY_CLASSES_ROOT);
    
    // Check if the TasKat key exists in Directory\shell
    if let Ok(directory_key) = hkcr.open_subkey_with_flags("Directory", KEY_READ) {
        if let Ok(shell_key) = directory_key.open_subkey_with_flags("shell", KEY_READ) {
            if shell_key.open_subkey("TasKat").is_ok() {
                return true;
            }
        }
    }
    
    false
}