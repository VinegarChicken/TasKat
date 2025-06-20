use clap::{Arg, Command};
use colored::*;
use std::io::{self, Write};
use std::path::Path;

mod context;
mod gemini;
mod registry;
mod runner;

use context::gather_file_context;
use gemini::generate_python_script;
use registry::{setup_registry, uninstall_registry};
use runner::execute_python_script;

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
            Command::new("setup")
                .about("Install right-click context menu integration")
        )
        .subcommand(
            Command::new("uninstall")
                .about("Remove right-click context menu integration")
        )
        .get_matches();

    match matches.subcommand() {
        Some(("setup", _)) => {
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
            println!("  {} - Install right-click context menu", "promptfile setup".cyan());
            println!("  {} - Remove right-click context menu", "promptfile uninstall".cyan());
            println!("  {} - Run directly on a folder", "promptfile prompt <folder>".cyan());
        }
    }

    Ok(())
}

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

        // Add current prompt to conversation history
        conversation_history.push(format!("User: {}", prompt));

        // Gather file context
        println!("{}", "📁 Analyzing folder structure...".blue());
        let context = gather_file_context(folder_path)?;
        
        // Generate Python script using Gemini with conversation history
        println!("{}", "🤖 Generating Python script...".blue());
        match generate_python_script(&context, prompt, &conversation_history).await {
            Ok(script) => {
                println!("{}", "Executing...".yellow());
                
                // Execute the script
                match execute_python_script(&script, folder_path).await {
                    Ok(output) => {
                        println!("{}", "Complete!".green().bold());
                        // Add AI response to conversation history
                        conversation_history.push(format!("Assistant: Executed script successfully. {}", output));
                    },
                    Err(e) => {
                        println!("{} {}", "❌ Execution failed:".red(), e);
                        conversation_history.push(format!("Assistant: Execution failed: {}", e));
                    }
                }
            }
            Err(e) => {
                println!("{} {}", "❌ Failed to generate script:".red(), e);
                conversation_history.push(format!("Assistant: Failed to generate script: {}", e));
            }
        }
        
        println!();
    }

    Ok(())
}