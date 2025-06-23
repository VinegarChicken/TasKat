use clap::{Arg, Command};
use colored::*;
use std::io::{self, Write};
use std::path::Path;
use std::env;

mod context;
mod gemini;
mod registry;
mod runner;
mod commands; // Add the new module

use context::gather_file_context;
use gemini::{generate_python_script, fix_python_script};
use registry::{setup_registry, uninstall_registry, is_installed};
use runner::{execute_python_script, is_recoverable_error, extract_error_details};
use commands::{CommandRegistry, get_config_path, is_command, extract_command_name, display_help};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let matches = Command::new("TasKat")
        .version("1.0")
        .about("AI-powered file operations via right-click context menu")
        .subcommand(
            Command::new("prompt")
                .about("Process files in a folder with AI")
                .arg(
                    Arg::new("folder")
                        .help("Target folder path")
                        .required(true)
                        .index(1)
                )
        )
        .subcommand(
            Command::new("install")
                .about("Install right-click context menu integration")
        )
        .subcommand(
            Command::new("uninstall")
                .about("Remove right-click context menu integration")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("install", _)) => {
            setup_registry()?;
            println!("{}", "✅ TasKat context menu installed! Right-click anywhere in File Explorer to use it.".green());
        }
        Some(("uninstall", _)) => {
            uninstall_registry()?;
            println!("{}", "✅ TasKat context menu removed successfully.".green());
            println!("{}", "If you ran into any issues or problems, be sure to contact the developer!".yellow());
            println!("{}", "Thank you for using TasKat 🫡".blue());
        }
        Some(("prompt", sub_matches)) => {
            let folder_path = sub_matches.get_one::<String>("folder").unwrap();
            run_prompt_mode(folder_path).await?;
        }
        _ => {
            // Handle no arguments - check installation status and prompt user
            handle_no_arguments().await?;
        }
    }

    Ok(())
}

// Updated conversation history management in main.rs
// Updated run_prompt_mode function to ask for permission preference
async fn run_prompt_mode(folder_path: &str) -> anyhow::Result<()> {
    let path = Path::new(folder_path);
    if !path.exists() || !path.is_dir() {
        println!("{}", "❌ Invalid folder path".red());
        return Ok(());
    }

    println!("{}", "TasKat".cyan().bold());
    println!("{}", "==========".cyan());
    println!("{} {}", "Selected Folder:".white(), folder_path.yellow());
    println!();

    // Ask for permission preference
    println!("{}", "Would you like to be asked for permission before deleting or overwriting files?".yellow());
    print!("{}", "Enter 'yes' or 'no' (default: yes): ".green());
    io::stdout().flush()?;

    let mut pref_input = String::new();
    io::stdin().read_line(&mut pref_input)?;
    let ask_permission = !pref_input.trim().eq_ignore_ascii_case("no");

    if ask_permission {
        println!("{}", "✅ You will be asked for permission before deleting or overwriting files.".green());
    } else {
        println!("{}", "ℹ️ Files may be deleted or overwritten without asking for permission.".blue());
    }
    println!();

    // Load command registry
    let config_path = get_config_path();
    let mut command_registry = CommandRegistry::load_from_file(&config_path)?;
    
    let mut conversation_history = Vec::new();
    const MAX_HISTORY_ENTRIES: usize = 10; // Limit history to prevent confusion
    
    // Store the last successful script for #save command
    let mut last_script = String::new();
    let mut last_prompt = String::new();

    loop {
        print!("{}", "Enter prompt here: > ".green());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let prompt = input.trim();

        if prompt.is_empty() {
            continue;
        }

        if prompt.eq_ignore_ascii_case("exit") || prompt.eq_ignore_ascii_case("quit") {
            break;
        }
        
        // Handle commands
        if is_command(prompt) {
            let command_name = extract_command_name(prompt);
            
            // Handle built-in commands
            if command_name.eq_ignore_ascii_case("help") {
                println!("{}", display_help());
                continue;
            } else if command_name.eq_ignore_ascii_case("save") {
                if last_script.is_empty() {
                    println!("{}", "❌ No script to save. Run a prompt first.".red());
                    continue;
                }
                
                // Prompt for command name
                print!("{}", "Enter command name (without #): ".green());
                io::stdout().flush()?;
                let mut name_input = String::new();
                io::stdin().read_line(&mut name_input)?;
                let command_name = name_input.trim();
                
                if command_name.is_empty() || command_name.contains(" ") {
                    println!("{}", "❌ Invalid command name. Names cannot be empty or contain spaces.".red());
                    continue;
                }
                
                // Prompt for description (optional)
                print!("{}", "Enter description (optional, press Enter to auto-generate): ".green());
                io::stdout().flush()?;
                let mut desc_input = String::new();
                io::stdin().read_line(&mut desc_input)?;
                let description = desc_input.trim();
                
                let final_description = if description.is_empty() {
                    // Generate description based on the prompt or script
                    if !last_prompt.is_empty() {
                        format!("Command for: {}", last_prompt)
                    } else {
                        command_registry.generate_description_for_script(&last_script)
                    }
                } else {
                    description.to_string()
                };
                
                // Save the command
                command_registry.add_command(command_name.to_string(), final_description, last_script.clone())?;
                command_registry.save_to_file(&config_path)?;
                
                println!("{} {}", "✅ Command saved:".green(), format!("#{}", command_name).cyan());
                continue;
            } else {
                // Check if it's a user-defined command
                if let Some(command) = command_registry.get_command(command_name) {
                    println!("{} {}", "🔄 Executing command:".blue(), format!("#{}", command_name).cyan());
                    println!("{} {}", "Description:".blue(), command.description);
                    
                    // Execute the saved script
                    let result = execute_script_with_retry(
                        &gather_file_context(folder_path)?,
                        &format!("Execute saved command: {}", command_name),
                        &conversation_history,
                        command.script.clone(),
                        folder_path,
                        ask_permission
                    ).await;
                    
                    match result {
                        Ok(success_msg) => {
                            println!("{}", "Complete!".green().bold());
                            conversation_history.push(format!("EXECUTED COMMAND: #{}", command_name));
                        },
                        Err(e) => {
                            println!("{} {}", "❌ Command execution failed:".red(), e);
                            conversation_history.push(format!("COMMAND_FAILED: #{} - {}", command_name, e));
                        }
                    }
                    
                    println!();
                    continue;
                } else {
                    println!("{} {}", "❌ Unknown command:".red(), prompt);
                    println!("Type {} for available commands", "#help".cyan());
                    continue;
                }
            }
        }

        // Regular prompt processing
        last_prompt = prompt.to_string(); // Save the prompt for potential #save command
        
        // Add current prompt to conversation history with clear formatting
        conversation_history.push(format!("USER REQUEST: {}", prompt));

        // Keep conversation history manageable
        if conversation_history.len() > MAX_HISTORY_ENTRIES {
            conversation_history.drain(0..2); // Remove oldest entries (request + response pair)
        }

        // Gather file context fresh each time
        println!("{}", "📁 Analyzing folder structure...".blue());
        let context = gather_file_context(folder_path)?;
        
        // Generate Python script using Gemini with limited conversation history
        println!("{}", "🤖 Generating Python script...".blue());
        // Update the call to generate_python_script to include the ask_permission parameter
        match generate_python_script(&context, prompt, &conversation_history, ask_permission).await {
            Ok(script) => {
                // Save the script for potential #save command
                last_script = script.clone();
                
                let result = execute_script_with_retry(&context, prompt, &conversation_history, script, folder_path, ask_permission).await;
                
                match result {
                    Ok(success_msg) => {
                        println!("{}", "Complete!".green().bold());
                        // Add a concise summary to history, not the full technical details
                        conversation_history.push(format!("COMPLETED: {}", prompt));
                    },
                    Err(e) => {
                        println!("{} {}", "❌ Final execution failed:".red(), e);
                        conversation_history.push(format!("FAILED: {} - {}", prompt, e));
                    }
                }
            }
            Err(e) => {
                println!("{} {}", "❌ Failed to generate script:".red(), e);
                conversation_history.push(format!("GENERATION_FAILED: {} - {}", prompt, e));
            }
        }
        
        println!();
    }

    Ok(())
}

async fn execute_script_with_retry(
    context: &context::FileContext,
    user_prompt: &str,
    conversation_history: &[String],
    mut script: String,
    folder_path: &str,
    ask_permission: bool,
) -> anyhow::Result<String> {
    const MAX_RETRIES: usize = 3;
    let mut retry_count = 0;

    loop {
        println!("{}", if retry_count == 0 { "Executing..." } else { "Retrying with fixed script..." }.yellow());
        
        // Execute the script
        match execute_python_script(&script, folder_path, ask_permission).await {
            Ok(execution_result) => {
                if execution_result.success {
                    return Ok(format!("Executed script successfully. {}", execution_result.output));
                } else if let Some(error) = execution_result.error {
                    // Script failed, check if we can recover
                    if retry_count < MAX_RETRIES && is_recoverable_error(&error) {
                        println!("{}", "🔧 Attempting to fix the script...".yellow());
                        
                        let error_details = extract_error_details(&error);
                        println!("Error details: {}", error_details.red());
                        
                        // Ask Gemini to fix the script
                        match fix_python_script(context, user_prompt, conversation_history, &script, &error_details, ask_permission).await {
                            Ok(fixed_script) => {
                                println!("{}", "🤖 Generated fixed script, retrying...".blue());
                                script = fixed_script;
                                retry_count += 1;
                                continue;
                            },
                            Err(fix_error) => {
                                return Err(anyhow::anyhow!("Failed to generate fix: {} (Original error: {})", fix_error, error));
                            }
                        }
                    } else {
                        // Either max retries reached or non-recoverable error
                        if retry_count >= MAX_RETRIES {
                            return Err(anyhow::anyhow!("Max retries ({}) reached. Last error: {}", MAX_RETRIES, error));
                        } else {
                            return Err(anyhow::anyhow!("Non-recoverable error: {}", error));
                        }
                    }
                } else {
                    return Err(anyhow::anyhow!("Script execution failed without error details"));
                }
            },
            Err(e) => {
                return Err(anyhow::anyhow!("Execution system error: {}", e));
            }
        }
    }
}

// Add this new function to handle no arguments
async fn handle_no_arguments() -> anyhow::Result<()> {
    // Check if running as administrator, if not, elevate
    if !is_elevated() {
        println!("Administrator privileges required for registry operations.");
        println!("Restarting with elevated privileges...");
        
        let exe_path = env::current_exe()?;
        let status = std::process::Command::new("powershell")
            .args([
                "-Command",
                &format!("Start-Process '{}' -Verb RunAs", exe_path.display())
            ])
            .status();
            
        match status {
            Ok(_) => {
                println!("Please check the new elevated window.");
                return Ok(());
            }
            Err(e) => {
                println!("Failed to elevate privileges: {}", e);
                println!("Please run as administrator manually.");
            }
        }
    }
    
    println!("TasKat");
    println!("==========");
    println!();
    
    // Reminder about right-click usage
    println!("Reminder: Right-click in any folder in File Explorer to use TasKat!");
    println!();
    
    // Check installation status
    let installed = is_installed();
    
    if installed {
        println!("TasKat is already installed.");
        print!("Would you like to uninstall? (yes/no): ");
        io::stdout().flush()?;
        
        let mut response = String::new();
        io::stdin().read_line(&mut response)?;
        
        if response.trim().eq_ignore_ascii_case("yes") {
            uninstall_registry()?;
            println!("TasKat context menu removed successfully.");
            println!("If you ran into any issues or problems, be sure to contact the developer!");
            println!("Thank you for using TasKat !");
            println!();
            println!("You may now close this window.");
            println!("Press Enter to exit...");
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
        } else {
            println!("Installation unchanged.");
        }
    } else {
        println!("TasKat currently isn't installed.");
        print!("Would you like to install? (yes/no): ");
        io::stdout().flush()?;
        
        let mut response = String::new();
        io::stdin().read_line(&mut response)?;
        
        if response.trim().eq_ignore_ascii_case("yes") {
            setup_registry()?;
            println!("✅ TasKat context menu installed! Right-click anywhere in File Explorer to use it.");
            println!();
            println!("You may now close this window.");
            println!("Press Enter to exit...");
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
        } else {
            println!("Installation skipped.");
        }
    }
    
    println!();
    println!("Available commands:");
    println!("  taskat install - Install right-click context menu");
    println!("  taskat uninstall - Remove right-click context menu");
    println!("  taskat prompt <folder> - Run directly on a folder");
    
    Ok(())
}

// Add this helper function to check if running as administrator
fn is_elevated() -> bool {
    let output = std::process::Command::new("net")
        .args(["session"])
        .output();
        
    match output {
        Ok(result) => result.status.success(),
        Err(_) => false,
    }
}