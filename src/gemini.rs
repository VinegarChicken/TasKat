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

pub async fn generate_python_script(context: &FileContext, user_prompt: &str, conversation_history: &[String]) -> Result<String> {
    // Get API key from environment variable
    let api_key = std::env::var("GEMINI_API_KEY")
        .context("GEMINI_API_KEY environment variable not set. Please set it with your Google AI Studio API key.")?;
    
    let client = Client::new();
    
    // Build conversation context
    let conversation_context = if conversation_history.is_empty() {
        String::new()
    } else {
        format!("\nCONVERSATION HISTORY:\n{}\n", conversation_history.join("\n"))
    };

    let system_prompt = format!(
        r#"You are an expert Python programmer specializing in file operations. Generate a complete, safe Python script based on the user's request and the provided file context.

IMPORTANT REQUIREMENTS:
1. Generate ONLY executable Python code - no explanations, no markdown formatting, no ```python blocks
2. The script will be executed directly in the target folder
3. Use os.chdir() at the start to change to the target directory
4. Include proper error handling with try/catch blocks
5. NEVER use input() for user interaction - the script runs non-interactively
6. For operations that might be destructive, add safety checks and warnings instead of prompts
7. Use intelligent defaults and safe assumptions rather than asking for user input
8. Add clear print statements to show what's being done
9. Use pathlib and os modules for cross-platform compatibility
10. Handle edge cases like file permissions, existing files, etc.
11. Consider the conversation history - this request might be related to previous operations

CRITICAL: Do NOT use input() or any interactive prompts. The script must run completely autonomously.

For file creation, use intelligent naming:
- If creating a single file, use a descriptive default name based on the request
- If creating multiple files, use numbered or timestamped names
- Always check if files exist and handle conflicts automatically (e.g., append numbers)

Example of safe file creation without input():
```python
import os
from pathlib import Path
from datetime import datetime

# Create a new text file with timestamp
timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
filename = f"new_file_{{timestamp}}.txt"

# Ensure unique filename
counter = 1
while os.path.exists(filename):
    base_name = f"new_file_{{timestamp}}_{{counter}}"
    filename = f"{{base_name}}.txt"
    counter += 1

with open(filename, 'w') as f:
    f.write("File content here")
print(f"Created: {{filename}}")
```

{conversation_context}

FILE CONTEXT:
{}

USER REQUEST: {}

Generate the Python script now (code only, no other text):"#,
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
            temperature: 0.3,
            top_k: 40,
            top_p: 0.95,
            max_output_tokens: 2048,
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