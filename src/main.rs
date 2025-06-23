use clap::{Arg, Command};
use colored::*;
use std::io::{self, Write};
use std::path::Path;
use std::env;
use serde::{Deserialize, Serialize};

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
            println!("{}", "✅ TasKat context menu installed! Right-click anywhere in File Explorer to use it. 
            Remember to install python 3 if you havent already !".green());
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
    print!("{}", "Enter 'yes'/'y' or 'no'/'n' (default: yes): ".green());
    io::stdout().flush()?;

    let mut pref_input = String::new();
    io::stdin().read_line(&mut pref_input)?;
    let ask_permission = !(pref_input.trim().eq_ignore_ascii_case("no") || pref_input.trim().eq_ignore_ascii_case("n"));

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
        print!("Would you like to uninstall? (yes/y/no/n): ");
        io::stdout().flush()?;
        
        let mut response = String::new();
        io::stdin().read_line(&mut response)?;
        
        if response.trim().eq_ignore_ascii_case("yes") || response.trim().eq_ignore_ascii_case("y") {
            uninstall_registry()?;
            println!("TasKat context menu removed successfully.");
            
            // Ask about license removal
            println!();
            print!("Would you like to remove your saved license key from this computer as well? (yes/y/no/n): ");
            io::stdout().flush()?;
            
            let mut license_response = String::new();
            io::stdin().read_line(&mut license_response)?;
            
            if license_response.trim().eq_ignore_ascii_case("yes") || license_response.trim().eq_ignore_ascii_case("y") {
                remove_license_key()?;
            } else {
                println!("License key kept for future installations.");
            }
            
            // Ask about Gemini API key removal
            println!();
            print!("Would you like to remove your saved Gemini API key from this computer as well? (yes/y/no/n): ");
            io::stdout().flush()?;
            
            let mut gemini_response = String::new();
            io::stdin().read_line(&mut gemini_response)?;
            
            if gemini_response.trim().eq_ignore_ascii_case("yes") || gemini_response.trim().eq_ignore_ascii_case("y") {
                remove_gemini_api_key()?;
            } else {
                println!("Gemini API key kept for future use.");
            }
            
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
        
        loop {
            print!("Would you like to install? (yes/y/no/n): ");
            io::stdout().flush()?;
            
            let mut response = String::new();
            io::stdin().read_line(&mut response)?;
            
            if response.trim().eq_ignore_ascii_case("yes") || response.trim().eq_ignore_ascii_case("y") {
                // Verify license before installation
                if !verify_gumroad_license().await? {
                    println!();
                    println!("❌ License verification failed. TasKat requires a valid license to install.");
                    println!("Please purchase a license at: https://gumroad.com/l/taskat");
                    println!();
                    println!("Press Enter to try again or type 'exit' to quit...");
                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    
                    if input.trim().eq_ignore_ascii_case("exit") {
                        return Ok(());
                    }
                    continue; // Go back to the beginning of the loop
                }
                
                setup_registry()?;
                println!("✅ TasKat context menu installed! Right-click anywhere in File Explorer to use it. Remember to install python 3 if you havent already !");
                println!();
                println!("You may now close this window.");
                println!("Press Enter to exit...");
                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                break; // Exit the loop after successful installation
            } else if response.trim().eq_ignore_ascii_case("no") || response.trim().eq_ignore_ascii_case("n") {
                println!("Installation skipped.");
                break; // Exit the loop if user chooses not to install
            } else {
                println!("Please enter 'yes'/'y' or 'no'/'n'.");
                continue; // Ask again for valid input
            }
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

#[derive(Deserialize)]
struct GumroadLicenseResponse {
    success: bool,
    uses: Option<i32>,
    purchase: Option<GumroadPurchase>,
}

#[derive(Deserialize)]
struct GumroadPurchase {
    seller_id: String,
    product_id: String,
    product_name: String,
    permalink: String,
    product_permalink: String,
    email: String,
    price: i32,
    gumroad_fee: i32,
    currency: String,
    quantity: i32,
    discover_fee_charged: bool,
    can_contact: bool,
    referrer: String,
    card: Option<serde_json::Value>,
    order_number: i64,
    sale_id: String,
    sale_timestamp: String,
    purchaser_id: String,
    subscription_id: Option<String>,
    variants: String,
    license_key: String,
    is_multiseat_license: Option<bool>,  // Already fixed
    ip_country: String,
    recurrence: Option<String>,  // Changed from String to Option<String>
    is_gift_receiver_purchase: bool,
    refunded: bool,
    disputed: bool,
    dispute_won: bool,
    id: String,
    created_at: String,
    custom_fields: Vec<serde_json::Value>,
    chargebacked: Option<bool>,
    subscription_ended_at: Option<String>,
    subscription_cancelled_at: Option<String>,
    subscription_failed_at: Option<String>,
}

async fn verify_gumroad_license() -> anyhow::Result<bool> {
    let product_id = "vjl65ppDUoOkJ5l9xcaT7g==".to_string();
    
    // Check for existing license key
    let license_key = if let Some(saved_key) = get_saved_license_key() {
        println!("📋 Previous license detected.");
        print!("Is the current license information correct? If not, enter a new license key (or press Enter to use saved): ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input.is_empty() {
            saved_key
        } else {
            input.to_string()
        }
    } else {
        // Prompt user for license key
        print!("Please enter your TasKat license key: ");
        io::stdout().flush()?;
        
        let mut license_key = String::new();
        io::stdin().read_line(&mut license_key)?;
        let license_key = license_key.trim();
        
        if license_key.is_empty() {
            println!("❌ No license key provided.");
            return Ok(false);
        }
        
        license_key.to_string()
    };

    println!("🔍 Verifying license...");
    
    // Make POST request to Gumroad API with form data
    let client = reqwest::Client::new();
    let params = [
        ("product_id", product_id.as_str()),
        ("license_key", &license_key),
        ("increment_uses_count", "false"), // Don't increment usage count for verification
    ];
    
    match client.post("https://api.gumroad.com/v2/licenses/verify")
        .form(&params)
        .send()
        .await 
    {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<GumroadLicenseResponse>().await {
                    Ok(license_response) => {
                        if license_response.success {
                            if let Some(purchase) = license_response.purchase {
                                println!("✅ License verified successfully!");
                                println!("   Product: {}", purchase.product_name);
                                println!("   Purchaser: {}", purchase.email);
                                println!("   Purchase Date: {}", purchase.created_at);
                                
                                // Check for refunded, disputed, or chargebacked licenses
                                if purchase.refunded {
                                    println!("❌ This license has been refunded and is no longer valid.");
                                    return Ok(false);
                                }
                                
                                if purchase.disputed {
                                    println!("❌ This license is under dispute and cannot be used.");
                                    return Ok(false);
                                }
                                
                                if let Some(chargebacked) = purchase.chargebacked {
                                    if chargebacked {
                                        println!("❌ This license has been chargebacked and is no longer valid.");
                                        return Ok(false);
                                    }
                                }
                                
                                // Check if it's a multiseat license
                                if let Some(is_multiseat) = purchase.is_multiseat_license {
                                    if is_multiseat {
                                        println!("ℹ️  This is a multiseat license.");
                                    }
                                }
                                
                                // Save the license key after successful verification
                                if let Err(e) = save_license_key(&license_key) {
                                    println!("⚠️ Warning: Could not save license key: {}", e);
                                }
                                
                                // After successful license verification, validate and set Gemini API key
                                loop {
                                    if !validate_and_set_gemini_api_key().await? {
                                        println!();
                                        println!("❌ Gemini API key validation failed. TasKat requires a valid Gemini API key to function.");
                                        println!("Please get your API key from: https://aistudio.google.com/app/apikey, or follow this youtube video for a quick step by step guide: https://youtu.be/o8iyrtQyrZM");
                                        println!();
                                        print!("Press Enter to try again or type 'exit' to cancel: ");
                                        io::stdout().flush()?;
                                        let mut input = String::new();
                                        io::stdin().read_line(&mut input)?;
                                        if input.trim().eq_ignore_ascii_case("exit") {
                                            return Ok(false);
                                        }
                                        continue; // Go back to the beginning of the loop
                                    }
                                    
                                    // If we get here, API key validation succeeded
                                    break;
                                }
                                return Ok(true);
                            } else {
                                println!("❌ Invalid license: No purchase information found.");
                                return Ok(false);
                            }
                        } else {
                            println!("❌ License verification failed: Invalid or expired license key.");
                            return Ok(false);
                        }
                    }
                    Err(e) => {
                        println!("❌ Failed to parse license response: {}", e);
                        return Ok(false);
                    }
                }
            } else {
                println!("❌ License verification failed: HTTP {}", response.status());
                if let Ok(error_text) = response.text().await {
                    println!("   Error details: {}", error_text);
                }
                return Ok(false);
            }
        }
        Err(e) => {
            println!("❌ Failed to connect to license server: {}", e);
            println!("Please check your internet connection and try again.");
            return Ok(false);
        }
    }
}

// Add this function after verify_gumroad_license
async fn validate_and_set_gemini_api_key() -> anyhow::Result<bool> {
    // Check for existing Gemini API key
    if let Ok(existing_key) = std::env::var("GEMINI_API_KEY") {
        if !existing_key.is_empty() {
            println!("🔑 Existing Gemini API key detected.");
            print!("Use existing API key? (yes/y/no/n): ");
            io::stdout().flush()?;
            
            let mut response = String::new();
            io::stdin().read_line(&mut response)?;
            
            if response.trim().eq_ignore_ascii_case("yes") || response.trim().eq_ignore_ascii_case("y") {
                println!("✅ Using existing Gemini API key.");
                return Ok(true);
            }
        }
    }
    
    println!();
    println!("🔑 Setting up Gemini API key...");
    print!("Please enter your Gemini API key (from Google AI Studio): ");
    io::stdout().flush()?;
    
    let mut api_key = String::new();
    io::stdin().read_line(&mut api_key)?;
    let api_key = api_key.trim();
    
    if api_key.is_empty() {
        println!("❌ No API key provided.");
        return Ok(false);
    }
    
    println!("🔍 Validating Gemini API key...");
    
    // Test the API key with a simple request
    let client = reqwest::Client::new();
    let test_request = serde_json::json!({
        "contents": [{
            "parts": [{
                "text": "Hello, respond with just 'OK' if you can see this message."
            }]
        }]
    });
    
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
        api_key
    );
    
    match client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&test_request)
        .send()
        .await
    {
        Ok(response) => {
            if response.status().is_success() {
                println!("✅ Gemini API key validated successfully!");
                
                // Set the environment variable for the current process
                env::set_var("GEMINI_API_KEY", api_key);
                
                // Also try to set it persistently for the user (Windows)
                let _result = std::process::Command::new("setx")
                    .args(["GEMINI_API_KEY", api_key])
                    .output();
                
                println!("🔧 Gemini API key has been set for this session and saved to your environment variables.");
                println!("   Note: You may need to restart your terminal for the persistent setting to take effect.");
                
                return Ok(true);
            } else {
                println!("❌ Invalid Gemini API key: HTTP {}", response.status());
                if let Ok(error_text) = response.text().await {
                    println!("   Error details: {}", error_text);
                }
                return Ok(false);
            }
        }
        Err(e) => {
            println!("❌ Failed to validate Gemini API key: {}", e);
            println!("Please check your internet connection and API key.");
            return Ok(false);
        }
    }
}

// Add these license management functions
fn save_license_key(license_key: &str) -> anyhow::Result<()> {
    // Set environment variable for current session
    std::env::set_var("TASKAT_LICENSE_KEY", license_key);
    
    // Set persistent environment variable on Windows
    let output = std::process::Command::new("setx")
        .args(["TASKAT_LICENSE_KEY", license_key])
        .output();
    
    match output {
        Ok(_) => {
            println!("✅ License key saved successfully.");
            Ok(())
        }
        Err(e) => {
            println!("⚠️ Warning: Could not save license key persistently: {}", e);
            println!("   License key is set for this session only.");
            Ok(())
        }
    }
}

fn get_saved_license_key() -> Option<String> {
    std::env::var("TASKAT_LICENSE_KEY").ok()
}

fn remove_license_key() -> anyhow::Result<()> {
    // Remove from current session
    std::env::remove_var("TASKAT_LICENSE_KEY");
    
    // Remove persistent environment variable on Windows
    let output = std::process::Command::new("setx")
        .args(["TASKAT_LICENSE_KEY", ""])
        .output();
    
    match output {
        Ok(_) => {
            println!("✅ License key removed successfully.");
            Ok(())
        }
        Err(e) => {
            println!("⚠️ Warning: Could not remove license key: {}", e);
            Ok(())
        }
    }
}

fn get_saved_gemini_api_key() -> Option<String> {
    std::env::var("GEMINI_API_KEY").ok()
}

fn remove_gemini_api_key() -> anyhow::Result<()> {
    // Remove from current session
    std::env::remove_var("GEMINI_API_KEY");
    
    // Remove persistent environment variable on Windows
    let output = std::process::Command::new("setx")
        .args(["GEMINI_API_KEY", ""])
        .output();
    
    match output {
        Ok(_) => {
            println!("✅ Gemini API key removed successfully.");
            Ok(())
        }
        Err(e) => {
            println!("⚠️ Warning: Could not remove Gemini API key: {}", e);
            Ok(())
        }
    }
}

