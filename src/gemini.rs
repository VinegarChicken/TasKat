use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::{Result, Context};
use crate::context::FileContext;

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
    #[serde(rename = "generationConfig")]
    generation_config: GenerationConfig,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Serialize)]
struct GenerationConfig {
    temperature: f32,
    #[serde(rename = "topK")]
    top_k: u32,
    #[serde(rename = "topP")]
    top_p: f32,
    #[serde(rename = "maxOutputTokens")]
    max_output_tokens: u32,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: ResponseContent,
}

#[derive(Deserialize)]
struct ResponseContent {
    parts: Vec<ResponsePart>,
}

#[derive(Deserialize)]
struct ResponsePart {
    text: String,
}

pub async fn generate_python_script(context: &FileContext, user_prompt: &str, conversation_history: &[String], ask_permission: bool) -> Result<String> {
    generate_python_script_internal(context, user_prompt, conversation_history, None, ask_permission).await
}

pub async fn fix_python_script(context: &FileContext, user_prompt: &str, conversation_history: &[String], original_script: &str, error_details: &str, ask_permission: bool) -> Result<String> {
    generate_python_script_internal(context, user_prompt, conversation_history, Some((original_script, error_details)), ask_permission).await
}

// Updated system prompt for better instruction following
async fn generate_python_script_internal(context: &FileContext, user_prompt: &str, conversation_history: &[String], error_context: Option<(&str, &str)>, ask_permission: bool) -> Result<String> {
    // Get API key from environment variable
    let api_key = std::env::var("GEMINI_API_KEY")
        .context("GEMINI_API_KEY environment variable not set. Please set it with your Google AI Studio API key.")?;
    
    let client = Client::new();
    
    // Build conversation context - only include the last 3 exchanges to avoid confusion
    let conversation_context = if conversation_history.is_empty() {
        String::new()
    } else {
        let recent_history: Vec<&String> = conversation_history.iter().rev().take(6).collect();
        let recent_history: Vec<&String> = recent_history.into_iter().rev().collect();
        format!("\nRECENT CONVERSATION CONTEXT (for reference only):\n{}\n", recent_history.iter().map(|s| s.as_str()).collect::<Vec<&str>>().join("\n"))
    };

    let error_context_str = if let Some((original_script, error_details)) = error_context {
        format!(
            "\nERROR CONTEXT - Fix this script:\n{}\n\nERROR DETAILS:\n{}\n",
            original_script, error_details
        )
    } else {
        String::new()
    };

    let system_prompt = format!(
        r#"You are an expert Python programmer specializing in file operations. Generate a complete, safe Python script based ONLY on the current user request.

CRITICAL INSTRUCTION FOLLOWING:
- Focus ONLY on the current user request - ignore previous requests unless explicitly referenced
- If user specifies a file type (PDF, DOCX, etc.), create ONLY that file type
- If NO file type is specified, default to creating a TEXT file (.txt)
- If user specifies length (e.g., "2 pages"), ensure the content meets that requirement
- Don't mix up current request with previous requests or existing files
- The conversation history is for context only - the current request takes priority

IMPORTANT REQUIREMENTS:
1. Generate ONLY executable Python code - no explanations, no markdown formatting, no ```python blocks
2. The script will be executed directly in the target folder
3. Use os.chdir() at the start to change to the target directory
4. Include proper error handling with try/catch blocks
5. NEVER use input() for user interaction - the script runs non-interactively

FILE OPERATION SAFETY:
- For operations that might DELETE or OVERWRITE files: {}
- ⚠️ CRITICAL WARNING: ENSURE ALL IMPORTANT FILES ARE BACKED UP BEFORE RUNNING THIS SCRIPT!
- ⚠️ THIS SCRIPT WILL PERMANENTLY MODIFY OR DELETE FILES WITHOUT CREATING BACKUPS!
- ⚠️ MAKE SURE YOU HAVE COPIES OF ANY IMPORTANT DATA BEFORE PROCEEDING!
- Use descriptive variable names and add comments for clarity

CONTENT GENERATION REQUIREMENTS:
- If user asks for specific length (e.g., "2 pages"), generate sufficient content to meet that requirement
- For essays: aim for 300-400 words per page minimum
- For reports: include proper structure with headings, paragraphs, and conclusions
- Don't generate placeholder text - create actual meaningful content
- Use the topic specified by the user, not topics from previous requests

CRITICAL PYTHON 3 COMPATIBILITY:
- NEVER import 'exceptions' - all standard exceptions are built-in in Python 3
- Use built-in exceptions directly: Exception, ValueError, TypeError, etc.
- For Word document operations, use: 'from docx import Document' (the python-docx package)
- For PDF operations, prefer 'fpdf' or 'reportlab' for direct PDF creation
- For image operations, use: 'from PIL import Image' (the Pillow package)
- Always use the correct import statements for modern Python packages

CRITICAL UNICODE HANDLING:
- Always handle Unicode characters in filenames properly
- For file operations that modify or move files, use repr() to show the full path safely
- For simple file listing operations, use .name attribute to show just the filename
- When listing directory contents, prefer: print(path.name) over print(repr(path))
- Example for file listing:
  ```python
  for item in Path('.').iterdir():
      print(item.name)  # Shows just the filename
  ```
- Use repr() or ascii() when printing filenames to avoid encoding issues
- Instead of: print(f"Moved '{{{{filename}}}}' to '{{{{destination}}}}'")
- Use: print(f"Moved {{{{repr(filename)}}}} to {{{{repr(destination)}}}}")
- For file operations, pathlib handles Unicode correctly, so prefer Path objects

FILE FORMAT REQUIREMENTS:
- If no file type is specified, default to creating a TEXT file (.txt)
- Use appropriate libraries for specific file types when requested
- If creating documents with significant content, ensure proper formatting and length

CRITICAL: Do NOT use input() or any interactive prompts. The script must run completely autonomously.

For file creation, use intelligent naming:
- If user specifies a filename, use exactly that name
- Only add timestamps if: (1) no filename is given by user, OR (2) the requested filename already exists
- When adding timestamps for conflicts: filename_YYYYMMDD_HHMMSS.ext
- Always check if files exist and handle conflicts automatically

Example for text file creation:
```python
from datetime import datetime
import os
from pathlib import Path

# Change to target directory
os.chdir(target_directory)

# Use user-specified filename or create descriptive name
requested_filename = "user_specified_name.txt"  # Use actual user request
filename = requested_filename

# Only add timestamp if file exists
if Path(filename).exists():
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    name_part = Path(filename).stem
    ext_part = Path(filename).suffix
    filename = f"{{name_part}}_{{timestamp}}{{ext_part}}"
    print(f"File {{requested_filename}} already exists. Creating {{filename}} instead.")

# Write content
with open(filename, 'w', encoding='utf-8') as f:
    f.write("Document Title\n\n")
    f.write("This is the content of the document.\n")
    # Add more substantial content here
    
print(f"Created: {{filename}}")
```

{conversation_context}{error_context_str}

CURRENT FILE CONTEXT (for reference):
{}

CURRENT USER REQUEST (this takes priority): {}

Generate a Python script that fulfills ONLY the current user request above:"#,
    if ask_permission {
        "proceed with file operations (permission will be handled by the system)"
    } else {
        "proceed without asking for permission"
    },
    context.to_string(),
    user_prompt
);

    let request = GeminiRequest {
        contents: vec![Content {
            parts: vec![Part {
                text: system_prompt,
            }],
        }],
        generation_config: GenerationConfig {
            temperature: 0.2,  // Lower temperature for more focused responses
            top_k: 40,
            top_p: 0.95,
            max_output_tokens: 3072,  // Increased for longer content generation
        },
    };

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
        api_key
    );

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .context("Failed to send request to Gemini API")?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(anyhow::anyhow!("Gemini API error: {}", error_text));
    }

    let gemini_response: GeminiResponse = response
        .json()
        .await
        .context("Failed to parse Gemini API response")?;

    let script = gemini_response
        .candidates
        .first()
        .and_then(|c| c.content.parts.first())
        .map(|p| p.text.clone())
        .context("No response from Gemini API")?;

    // Clean up the script - remove any markdown formatting if present
    let cleaned_script = script
        .lines()
        .filter(|line| !line.trim().starts_with("```"))
        .collect::<Vec<_>>()
        .join("\n");

    Ok(cleaned_script.trim().to_string())
}


pub async fn validate_script_safety(script: &str) -> Result<(bool, String)> {
    let system_prompt = format!(
        r#"You are a helpful assistant that explains what a Python script will do before it runs.

Please:
You are a helpful assistant that explains what a Python script will do before it runs.

Your job is to describe the script’s behavior in a friendly, simple way so the user can decide if they want to run it.

Please:
- Use short, clear bullet points
- Avoid technical terms (like "working directory", "console", "variables", etc.)
- Keep the language easy for anyone to understand, even without coding experience
- If the script deletes any files or folders, gently remind the user to make sure they have backups first
- If the script does anything unusual (like installing programs or changing system settings), explain it calmly and clearly so the user can decide
- Only use strong language like “dangerous” or “security warning” if something truly harmful is happening (like wiping system files)

At the end, include one of the following messages:

**"✅ All clear. Proceed?"**  
or  
**"⚠️ This script includes sensitive system operations. Are you sure you want to continue?"**

### Here's the script to review:

```python
{}
```"#,
        script
    );

    let request = GeminiRequest {
        contents: vec![Content {
            parts: vec![Part {
                text: system_prompt,
            }],
        }],
        generation_config: GenerationConfig {
            temperature: 0.1,  // Very low temperature for consistent safety analysis
            top_k: 40,
            top_p: 0.95,
            max_output_tokens: 1024,
        },
    };

    // Get API key from environment
    let api_key = std::env::var("GEMINI_API_KEY")
        .context("GEMINI_API_KEY environment variable not set")?;

    let client = Client::new();
    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
        api_key
    );

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .json(&request)
        .send()
        .await
        .context("Failed to send request to Gemini API")?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_else(|_| "Unknown error".to_string());
        return Err(anyhow::anyhow!("Gemini API error: {}", error_text));
    }

    let gemini_response: GeminiResponse = response
        .json()
        .await
        .context("Failed to parse Gemini API response")?;

    let validation_text = gemini_response
        .candidates
        .first()
        .and_then(|c| c.content.parts.first())
        .map(|p| p.text.clone())
        .context("No response from Gemini API")?
        .trim()
        .to_string();
    
    // Check if Gemini marked it as safe or requiring caution
    let is_safe = validation_text.contains("✅ All clear. Proceed?");
    
    Ok((is_safe, validation_text))
}