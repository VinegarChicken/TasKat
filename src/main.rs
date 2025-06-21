use clap::{Arg, Command};
use colored::*;
use std::io::{self, Write};
use std::path::Path;

mod context;
mod gemini;
mod registry;
mod runner;

use context::gather_file_context;
use gemini::{generate_python_script, fix_python_script};
use registry::{setup_registry, uninstall_registry};
use runner::{execute_python_script, is_recoverable_error, extract_error_details};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let matches = Command::new("PromptFile")
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
            println!("{}", "✅ PromptFile context menu installed! Right-click anywhere in File Explorer to use it.".green());
        }
        Some(("uninstall", _)) => {
            uninstall_registry()?;
            println!("{}", "✅ PromptFile context menu removed successfully.".green());
        }
        Some(("prompt", sub_matches)) => {
            let folder_path = sub_matches.get_one::<String>("folder").unwrap();
            run_prompt_mode(folder_path).await?;
        }
        _ => {
            println!("{}", "Available commands:".yellow());
            println!("  {} - Install right-click context menu", "promptfile install".cyan());
            println!("  {} - Remove right-click context menu", "promptfile uninstall".cyan());
            println!("  {} - Run directly on a folder", "promptfile prompt <folder>".cyan());
        }
    }

    Ok(())
}

// Updated conversation history management in main.rs
async fn run_prompt_mode(folder_path: &str) -> anyhow::Result<()> {
    let path = Path::new(folder_path);
    if !path.exists() || !path.is_dir() {
        println!("{}", "❌ Invalid folder path".red());
        return Ok(());
    }

    println!("{}", "PromptFile".cyan().bold());
    println!("{}", "==========".cyan());
    println!("{} {}", "Selected Folder:".white(), folder_path.yellow());
    println!();

    let mut conversation_history = Vec::new();
    const MAX_HISTORY_ENTRIES: usize = 10; // Limit history to prevent confusion

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
        match generate_python_script(&context, prompt, &conversation_history).await {
            Ok(script) => {
                let result = execute_script_with_retry(&context, prompt, &conversation_history, script, folder_path).await;
                
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
) -> anyhow::Result<String> {
    const MAX_RETRIES: usize = 3;
    let mut retry_count = 0;

    loop {
        println!("{}", if retry_count == 0 { "Executing..." } else { "Retrying with fixed script..." }.yellow());
        
        // Execute the script
        match execute_python_script(&script, folder_path).await {
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
                        match fix_python_script(context, user_prompt, conversation_history, &script, &error_details).await {
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