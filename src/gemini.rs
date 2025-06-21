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
    generate_python_script_internal(context, user_prompt, conversation_history, None).await
}

pub async fn fix_python_script(context: &FileContext, user_prompt: &str, conversation_history: &[String], original_script: &str, error_details: &str) -> Result<String> {
    generate_python_script_internal(context, user_prompt, conversation_history, Some((original_script, error_details))).await
}

// Updated system prompt for better instruction following
async fn generate_python_script_internal(context: &FileContext, user_prompt: &str, conversation_history: &[String], error_context: Option<(&str, &str)>) -> Result<String> {
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
- If user asks for "a PDF", create ONLY a PDF file, not multiple formats
- If user specifies length (e.g., "2 pages"), ensure the content meets that requirement
- Don't mix up current request with previous requests or existing files
- The conversation history is for context only - the current request takes priority

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
- Use repr() or ascii() when printing filenames to avoid encoding issues
- Instead of: print(f"Moved '{{filename}}' to '{{destination}}'")
- Use: print(f"Moved {{repr(filename)}} to {{repr(destination)}}")
- For file operations, pathlib handles Unicode correctly, so prefer Path objects

FILE FORMAT REQUIREMENTS:
- If user asks for PDF only, create ONLY PDF - don't create intermediate DOCX files
- Use appropriate libraries: fpdf for simple PDFs, reportlab for complex layouts
- If creating documents with significant content, ensure proper formatting and length

CRITICAL: Do NOT use input() or any interactive prompts. The script must run completely autonomously.

For file creation, use intelligent naming:
- Use descriptive names based on the current request topic
- Include timestamps to avoid conflicts: filename_YYYYMMDD_HHMMSS.ext
- Always check if files exist and handle conflicts automatically

Example for PDF creation:
```python
from fpdf import FPDF
from datetime import datetime
import os

# Change to target directory
os.chdir(target_directory)

# Create PDF with substantial content
pdf = FPDF()
pdf.add_page()
pdf.set_font('Arial', 'B', 16)
pdf.cell(0, 10, 'Document Title', ln=True, align='C')

# Add enough content for the requested length
pdf.set_font('Arial', '', 12)
for page_num in range(num_pages):
    if page_num > 0:
        pdf.add_page()
    # Add substantial content here - not just placeholder text
    
timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
filename = f"document_{{timestamp}}.pdf"
pdf.output(filename)
print(f"Created: {{repr(filename)}}")
```

{conversation_context}{error_context_str}

CURRENT FILE CONTEXT (for reference):
{}

CURRENT USER REQUEST (this takes priority): {}

Generate a Python script that fulfills ONLY the current user request above:"#,
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