# TasKat

AI-powered file operations via Windows right-click context menu integration. Generate and execute Python scripts to process files and folders using natural language prompts.

## Features

- **Right-click Integration**: Access TasKat directly from Windows File Explorer
- **AI-Powered Script Generation**: Uses Google's Gemini AI to generate Python scripts
- **Security-First Design**: Built-in security analysis and user approval for potentially dangerous operations
- **Automatic Dependency Management**: Automatically installs required Python packages
- **Error Recovery**: Intelligent retry mechanism with AI-powered script fixing
- **Comprehensive Logging**: Detailed logging for debugging and monitoring

## Security Features

### Script Security Analysis
TasKat analyzes generated scripts for potentially dangerous operations:

- **Critical**: System command execution, dynamic code evaluation, file operations
- **High Risk**: File/directory deletion, recursive operations
- **Medium Risk**: Network requests, external communications

### User Safety Measures
- **Interactive Approval**: User confirmation required for scripts with security warnings
- **Execution Timeout**: Scripts automatically terminated after 5 minutes
- **Detailed Warnings**: Clear explanation of potential risks before execution
- **Comprehensive Logging**: All operations logged for audit trail

## Installation

### Prerequisites

1. **Python**: Install Python 3.7+ and ensure it's in your PATH
2. **Gemini API Key**: Get your free API key from [Google AI Studio](https://makersuite.google.com/app/apikey)

### Setup

1. **Download and Build**:
   ```bash
   git clone <repository-url>
   cd TasKat
   cargo build --release
   ```

2. **Set Environment Variable**:
   ```bash
   # Windows Command Prompt
   setx GEMINI_API_KEY "your-api-key-here"
   
   # PowerShell
   [Environment]::SetEnvironmentVariable("GEMINI_API_KEY", "your-api-key-here", "User")
   ```

3. **Install Context Menu Integration**:
   ```bash
   ./target/release/taskat.exe install
   ```

## Usage

### Via Context Menu
1. Right-click on any folder in Windows File Explorer
2. Select "TasKat Command"
3. Enter your natural language prompt
4. Review and approve any security warnings
5. Watch as the AI generates and executes the appropriate script

### Via Command Line
```bash
# Process a specific folder
taskat prompt "C:\path\to\folder"
taskat install
taskat uninstall

- `RUST_LOG`: Logging level (default: `taskat=info`)

Logs are written to stdout with timestamps. Set `RUST_LOG=taskat=debug` for verbose logging.

set RUST_LOG=taskat=debug
taskat prompt "C:\path\to\folder"
```

## Development

### Building from Source

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test
```

### Dependencies

- **clap**: Command-line argument parsing
- **reqwest**: HTTP client for Gemini API
- **tokio**: Async runtime
- **colored**: Terminal colors
- **log/env_logger**: Logging framework
- **backoff**: Retry logic with exponential backoff
- **winreg**: Windows registry integration

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

[Add your license here]

## Changelog

### v0.2.0 (Security Update)
- Added comprehensive security analysis
- Implemented user approval for dangerous operations
- Added script execution timeout (5 minutes)
- Enhanced error handling with retry logic
- Comprehensive logging system
- Better API error handling with exponential backoff

### v0.1.0
- Initial release
- Basic AI script generation
- Windows context menu integration
- Automatic Python package installation
- **Execution Timeout**: Scripts automatically terminated after 5 minutes
- **Detailed Warnings**: Clear explanation of potential risks before execution
- **Comprehensive Logging**: All operations logged for audit trail

## Installation

### Prerequisites

1. **Python**: Install Python 3.7+ and ensure it's in your PATH
2. **Gemini API Key**: Get your free API key from [Google AI Studio](https://makersuite.google.com/app/apikey)

### Setup

1. **Download and Build**:
   ```bash
   git clone <repository-url>
   cd PromptFile
   cargo build --release
   ```

2. **Set Environment Variable**:
   ```bash
   # Windows Command Prompt
   setx GEMINI_API_KEY "your-api-key-here"
   
   # PowerShell
   [Environment]::SetEnvironmentVariable("GEMINI_API_KEY", "your-api-key-here", "User")
   ```

3. **Install Context Menu Integration**:
   ```bash
   ./target/release/promptfile.exe install
   ```

## Usage

### Via Context Menu
1. Right-click on any folder in Windows File Explorer
2. Select "PromptFile Command"
3. Enter your natural language prompt
4. Review and approve any security warnings
5. Watch as the AI generates and executes the appropriate script

### Via Command Line
```bash
# Process a specific folder
promptfile prompt "C:\path\to\folder"

# Install context menu
promptfile install

# Remove context menu
promptfile uninstall
```

## Example Prompts

- "Create a summary report of all files in this folder"
- "Rename all images to include their creation date"
- "Convert all Word documents to PDF"
- "Organize files by type into subfolders"
- "Generate a CSV inventory of all files with sizes"
- "Create thumbnails for all images"

## Configuration

### Environment Variables

- `GEMINI_API_KEY`: Your Google AI Studio API key (required)
- `RUST_LOG`: Logging level (default: `promptfile=info`)

### Logging

Logs are written to stdout with timestamps. Set `RUST_LOG=promptfile=debug` for verbose logging.

## Security Best Practices

1. **Review Scripts**: Always review security warnings before approving script execution
2. **Backup Important Data**: Create backups before running scripts on important folders
3. **Test First**: Try scripts on test folders before using on production data
4. **Monitor Logs**: Check logs for any unexpected behavior
5. **API Key Security**: Keep your Gemini API key secure and never share it

## Troubleshooting

### Common Issues

**"No Python interpreter found"**
- Install Python and ensure it's in your PATH
- Try running `python --version` in Command Prompt

**"GEMINI_API_KEY environment variable not set"**
- Set the environment variable as shown in setup
- Restart your terminal/File Explorer after setting

**"Failed to install context menu"**
- Run as Administrator
- Ensure you have write permissions to the registry

### Debug Mode

Enable debug logging:
```bash
set RUST_LOG=promptfile=debug
promptfile prompt "C:\path\to\folder"
```

## Development

### Building from Source

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test
```

### Dependencies

- **clap**: Command-line argument parsing
- **reqwest**: HTTP client for Gemini API
- **tokio**: Async runtime
- **colored**: Terminal colors
- **log/env_logger**: Logging framework
- **backoff**: Retry logic with exponential backoff
- **winreg**: Windows registry integration

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

[Add your license here]

## Changelog

### v0.2.0 (Security Update)
- Added comprehensive security analysis
- Implemented user approval for dangerous operations
- Added script execution timeout (5 minutes)
- Enhanced error handling with retry logic
- Comprehensive logging system
- Better API error handling with exponential backoff

### v0.1.0
- Initial release
- Basic AI script generation
- Windows context menu integration
- Automatic Python package installation
- **Execution Timeout**: Scripts automatically terminated after 5 minutes
- **Detailed Warnings**: Clear explanation of potential risks before execution
- **Comprehensive Logging**: All operations logged for audit trail

## Installation

### Prerequisites

1. **Python**: Install Python 3.7+ and ensure it's in your PATH
2. **Gemini API Key**: Get your free API key from [Google AI Studio](https://makersuite.google.com/app/apikey)

### Setup

1. **Download and Build**:
   ```bash
   git clone <repository-url>
   cd PromptFile
   cargo build --release
   ```

2. **Set Environment Variable**:
   ```bash
   # Windows Command Prompt
   setx GEMINI_API_KEY "your-api-key-here"
   
   # PowerShell
   [Environment]::SetEnvironmentVariable("GEMINI_API_KEY", "your-api-key-here", "User")
   ```

3. **Install Context Menu Integration**:
   ```bash
   ./target/release/promptfile.exe install
   ```

## Usage

### Via Context Menu
1. Right-click on any folder in Windows File Explorer
2. Select "PromptFile Command"
3. Enter your natural language prompt
4. Review and approve any security warnings
5. Watch as the AI generates and executes the appropriate script

### Via Command Line
```bash
# Process a specific folder
promptfile prompt "C:\path\to\folder"

# Install context menu
promptfile install

# Remove context menu
promptfile uninstall
```

## Example Prompts

- "Create a summary report of all files in this folder"
- "Rename all images to include their creation date"
- "Convert all Word documents to PDF"
- "Organize files by type into subfolders"
- "Generate a CSV inventory of all files with sizes"
- "Create thumbnails for all images"

## Configuration

### Environment Variables

- `GEMINI_API_KEY`: Your Google AI Studio API key (required)
- `RUST_LOG`: Logging level (default: `promptfile=info`)

### Logging

Logs are written to stdout with timestamps. Set `RUST_LOG=promptfile=debug` for verbose logging.

## Security Best Practices

1. **Review Scripts**: Always review security warnings before approving script execution
2. **Backup Important Data**: Create backups before running scripts on important folders
3. **Test First**: Try scripts on test folders before using on production data
4. **Monitor Logs**: Check logs for any unexpected behavior
5. **API Key Security**: Keep your Gemini API key secure and never share it

## Troubleshooting

### Common Issues

**"No Python interpreter found"**
- Install Python and ensure it's in your PATH
- Try running `python --version` in Command Prompt

**"GEMINI_API_KEY environment variable not set"**
- Set the environment variable as shown in setup
- Restart your terminal/File Explorer after setting

**"Failed to install context menu"**
- Run as Administrator
- Ensure you have write permissions to the registry

### Debug Mode

Enable debug logging:
```bash
set RUST_LOG=promptfile=debug
promptfile prompt "C:\path\to\folder"
```

## Development

### Building from Source

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test
```

### Dependencies

- **clap**: Command-line argument parsing
- **reqwest**: HTTP client for Gemini API
- **tokio**: Async runtime
- **colored**: Terminal colors
- **log/env_logger**: Logging framework
- **backoff**: Retry logic with exponential backoff
- **winreg**: Windows registry integration

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

[Add your license here]

## Changelog

### v0.2.0 (Security Update)
- Added comprehensive security analysis
- Implemented user approval for dangerous operations
- Added script execution timeout (5 minutes)
- Enhanced error handling with retry logic
- Comprehensive logging system
- Better API error handling with exponential backoff

### v0.1.0
- Initial release
- Basic AI script generation
- Windows context menu integration
- Automatic Python package installation
- **Execution Timeout**: Scripts automatically terminated after 5 minutes
- **Detailed Warnings**: Clear explanation of potential risks before execution
- **Comprehensive Logging**: All operations logged for audit trail

## Installation

### Prerequisites

1. **Python**: Install Python 3.7+ and ensure it's in your PATH
2. **Gemini API Key**: Get your free API key from [Google AI Studio](https://makersuite.google.com/app/apikey)

### Setup

1. **Download and Build**:
   ```bash
   git clone <repository-url>
   cd PromptFile
   cargo build --release
   ```

2. **Set Environment Variable**:
   ```bash
   # Windows Command Prompt
   setx GEMINI_API_KEY "your-api-key-here"
   
   # PowerShell
   [Environment]::SetEnvironmentVariable("GEMINI_API_KEY", "your-api-key-here", "User")
   ```

3. **Install Context Menu Integration**:
   ```bash
   ./target/release/promptfile.exe install
   ```

## Usage

### Via Context Menu
1. Right-click on any folder in Windows File Explorer
2. Select "PromptFile Command"
3. Enter your natural language prompt
4. Review and approve any security warnings
5. Watch as the AI generates and executes the appropriate script

### Via Command Line
```bash
# Process a specific folder
promptfile prompt "C:\path\to\folder"

# Install context menu
promptfile install

# Remove context menu
promptfile uninstall
```

## Example Prompts

- "Create a summary report of all files in this folder"
- "Rename all images to include their creation date"
- "Convert all Word documents to PDF"
- "Organize files by type into subfolders"
- "Generate a CSV inventory of all files with sizes"
- "Create thumbnails for all images"

## Configuration

### Environment Variables

- `GEMINI_API_KEY`: Your Google AI Studio API key (required)
- `RUST_LOG`: Logging level (default: `promptfile=info`)

### Logging

Logs are written to stdout with timestamps. Set `RUST_LOG=promptfile=debug` for verbose logging.

## Security Best Practices

1. **Review Scripts**: Always review security warnings before approving script execution
2. **Backup Important Data**: Create backups before running scripts on important folders
3. **Test First**: Try scripts on test folders before using on production data
4. **Monitor Logs**: Check logs for any unexpected behavior
5. **API Key Security**: Keep your Gemini API key secure and never share it

## Troubleshooting

### Common Issues

**"No Python interpreter found"**
- Install Python and ensure it's in your PATH
- Try running `python --version` in Command Prompt

**"GEMINI_API_KEY environment variable not set"**
- Set the environment variable as shown in setup
- Restart your terminal/File Explorer after setting

**"Failed to install context menu"**
- Run as Administrator
- Ensure you have write permissions to the registry

### Debug Mode

Enable debug logging:
```bash
set RUST_LOG=promptfile=debug
promptfile prompt "C:\path\to\folder"
```

## Development

### Building from Source

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test
```

### Dependencies

- **clap**: Command-line argument parsing
- **reqwest**: HTTP client for Gemini API
- **tokio**: Async runtime
- **colored**: Terminal colors
- **log/env_logger**: Logging framework
- **backoff**: Retry logic with exponential backoff
- **winreg**: Windows registry integration

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

[Add your license here]

## Changelog

### v0.2.0 (Security Update)
- Added comprehensive security analysis
- Implemented user approval for dangerous operations
- Added script execution timeout (5 minutes)
- Enhanced error handling with retry logic
- Comprehensive logging system
- Better API error handling with exponential backoff

### v0.1.0
- Initial release
- Basic AI script generation
- Windows context menu integration
- Automatic Python package installation
- **Execution Timeout**: Scripts automatically terminated after 5 minutes
- **Detailed Warnings**: Clear explanation of potential risks before execution
- **Comprehensive Logging**: All operations logged for audit trail

## Installation

### Prerequisites

1. **Python**: Install Python 3.7+ and ensure it's in your PATH
2. **Gemini API Key**: Get your free API key from [Google AI Studio](https://makersuite.google.com/app/apikey)

### Setup

1. **Download and Build**:
   ```bash
   git clone <repository-url>
   cd PromptFile
   cargo build --release
   ```

2. **Set Environment Variable**:
   ```bash
   # Windows Command Prompt
   setx GEMINI_API_KEY "your-api-key-here"
   
   # PowerShell
   [Environment]::SetEnvironmentVariable("GEMINI_API_KEY", "your-api-key-here", "User")
   ```

3. **Install Context Menu Integration**:
   ```bash
   ./target/release/promptfile.exe install
   ```

## Usage

### Via Context Menu
1. Right-click on any folder in Windows File Explorer
2. Select "PromptFile Command"
3. Enter your natural language prompt
4. Review and approve any security warnings
5. Watch as the AI generates and executes the appropriate script

### Via Command Line
```bash
# Process a specific folder
promptfile prompt "C:\path\to\folder"

# Install context menu
promptfile install

# Remove context menu
promptfile uninstall
```

## Example Prompts

- "Create a summary report of all files in this folder"
- "Rename all images to include their creation date"
- "Convert all Word documents to PDF"
- "Organize files by type into subfolders"
- "Generate a CSV inventory of all files with sizes"
- "Create thumbnails for all images"

## Configuration

### Environment Variables

- `GEMINI_API_KEY`: Your Google AI Studio API key (required)
- `RUST_LOG`: Logging level (default: `promptfile=info`)

### Logging

Logs are written to stdout with timestamps. Set `RUST_LOG=promptfile=debug` for verbose logging.

## Security Best Practices

1. **Review Scripts**: Always review security warnings before approving script execution
2. **Backup Important Data**: Create backups before running scripts on important folders
3. **Test First**: Try scripts on test folders before using on production data
4. **Monitor Logs**: Check logs for any unexpected behavior
5. **API Key Security**: Keep your Gemini API key secure and never share it

## Troubleshooting

### Common Issues

**"No Python interpreter found"**
- Install Python and ensure it's in your PATH
- Try running `python --version` in Command Prompt

**"GEMINI_API_KEY environment variable not set"**
- Set the environment variable as shown in setup
- Restart your terminal/File Explorer after setting

**"Failed to install context menu"**
- Run as Administrator
- Ensure you have write permissions to the registry

### Debug Mode

Enable debug logging:
```bash
set RUST_LOG=promptfile=debug
promptfile prompt "C:\path\to\folder"
```

## Development

### Building from Source

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test
```

### Dependencies

- **clap**: Command-line argument parsing
- **reqwest**: HTTP client for Gemini API
- **tokio**: Async runtime
- **colored**: Terminal colors
- **log/env_logger**: Logging framework
- **backoff**: Retry logic with exponential backoff
- **winreg**: Windows registry integration

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

[Add your license here]

## Changelog

### v0.2.0 (Security Update)
- Added comprehensive security analysis
- Implemented user approval for dangerous operations
- Added script execution timeout (5 minutes)
- Enhanced error handling with retry logic
- Comprehensive logging system
- Better API error handling with exponential backoff

### v0.1.0
- Initial release
- Basic AI script generation
- Windows context menu integration
- Automatic Python package installation
- **Execution Timeout**: Scripts automatically terminated after 5 minutes
- **Detailed Warnings**: Clear explanation of potential risks before execution
- **Comprehensive Logging**: All operations logged for audit trail

## Installation

### Prerequisites

1. **Python**: Install Python 3.7+ and ensure it's in your PATH
2. **Gemini API Key**: Get your free API key from [Google AI Studio](https://makersuite.google.com/app/apikey)

### Setup

1. **Download and Build**:
   ```bash
   git clone <repository-url>
   cd PromptFile
   cargo build --release
   ```

2. **Set Environment Variable**:
   ```bash
   # Windows Command Prompt
   setx GEMINI_API_KEY "your-api-key-here"
   
   # PowerShell
   [Environment]::SetEnvironmentVariable("GEMINI_API_KEY", "your-api-key-here", "User")
   ```

3. **Install Context Menu Integration**:
   ```bash
   ./target/release/promptfile.exe install
   ```

## Usage

### Via Context Menu
1. Right-click on any folder in Windows File Explorer
2. Select "PromptFile Command"
3. Enter your natural language prompt
4. Review and approve any security warnings
5. Watch as the AI generates and executes the appropriate script

### Via Command Line
```bash
# Process a specific folder
promptfile prompt "C:\path\to\folder"

# Install context menu
promptfile install

# Remove context menu
promptfile uninstall
```

## Example Prompts

- "Create a summary report of all files in this folder"
- "Rename all images to include their creation date"
- "Convert all Word documents to PDF"
- "Organize files by type into subfolders"
- "Generate a CSV inventory of all files with sizes"
- "Create thumbnails for all images"

## Configuration

### Environment Variables

- `GEMINI_API_KEY`: Your Google AI Studio API key (required)
- `RUST_LOG`: Logging level (default: `promptfile=info`)

### Logging

Logs are written to stdout with timestamps. Set `RUST_LOG=promptfile=debug` for verbose logging.

## Security Best Practices

1. **Review Scripts**: Always review security warnings before approving script execution
2. **Backup Important Data**: Create backups before running scripts on important folders
3. **Test First**: Try scripts on test folders before using on production data
4. **Monitor Logs**: Check logs for any unexpected behavior
5. **API Key Security**: Keep your Gemini API key secure and never share it

## Troubleshooting

### Common Issues

**"No Python interpreter found"**
- Install Python and ensure it's in your PATH
- Try running `python --version` in Command Prompt

**"GEMINI_API_KEY environment variable not set"**
- Set the environment variable as shown in setup
- Restart your terminal/File Explorer after setting

**"Failed to install context menu"**
- Run as Administrator
- Ensure you have write permissions to the registry

### Debug Mode

Enable debug logging:
```bash
set RUST_LOG=promptfile=debug
promptfile prompt "C:\path\to\folder"
```

## Development

### Building from Source

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test
```

### Dependencies

- **clap**: Command-line argument parsing
- **reqwest**: HTTP client for Gemini API
- **tokio**: Async runtime
- **colored**: Terminal colors
- **log/env_logger**: Logging framework
- **backoff**: Retry logic with exponential backoff
- **winreg**: Windows registry integration

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

[Add your license here]

## Changelog

### v0.2.0 (Security Update)
- Added comprehensive security analysis
- Implemented user approval for dangerous operations
- Added script execution timeout (5 minutes)
- Enhanced error handling with retry logic
- Comprehensive logging system
- Better API error handling with exponential backoff

### v0.1.0
- Initial release
- Basic AI script generation
- Windows context menu integration
- Automatic Python package installation
- **Execution Timeout**: Scripts automatically terminated after 5 minutes
- **Detailed Warnings**: Clear explanation of potential risks before execution
- **Comprehensive Logging**: All operations logged for audit trail

## Installation

### Prerequisites

1. **Python**: Install Python 3.7+ and ensure it's in your PATH
2. **Gemini API Key**: Get your free API key from [Google AI Studio](https://makersuite.google.com/app/apikey)

### Setup

1. **Download and Build**:
   ```bash
   git clone <repository-url>
   cd PromptFile
   cargo build --release
   ```

2. **Set Environment Variable**:
   ```bash
   # Windows Command Prompt
   setx GEMINI_API_KEY "your-api-key-here"
   
   # PowerShell
   [Environment]::SetEnvironmentVariable("GEMINI_API_KEY", "your-api-key-here", "User")
   ```

3. **Install Context Menu Integration**:
   ```bash
   ./target/release/promptfile.exe install
   ```

## Usage

### Via Context Menu
1. Right-click on any folder in Windows File Explorer
2. Select "PromptFile Command"
3. Enter your natural language prompt
4. Review and approve any security warnings
5. Watch as the AI generates and executes the appropriate script

### Via Command Line
```bash
# Process a specific folder
promptfile prompt "C:\path\to\folder"

# Install context menu
promptfile install

# Remove context menu
promptfile uninstall
```

## Example Prompts

- "Create a summary report of all files in this folder"
- "Rename all images to include their creation date"
- "Convert all Word documents to PDF"
- "Organize files by type into subfolders"
- "Generate a CSV inventory of all files with sizes"
- "Create thumbnails for all images"

## Configuration

### Environment Variables

- `GEMINI_API_KEY`: Your Google AI Studio API key (required)
- `RUST_LOG`: Logging level (default: `promptfile=info`)

### Logging

Logs are written to stdout with timestamps. Set `RUST_LOG=promptfile=debug` for verbose logging.

## Security Best Practices

1. **Review Scripts**: Always review security warnings before approving script execution
2. **Backup Important Data**: Create backups before running scripts on important folders
3. **Test First**: Try scripts on test folders before using on production data
4. **Monitor Logs**: Check logs for any unexpected behavior
5. **API Key Security**: Keep your Gemini API key secure and never share it

## Troubleshooting

### Common Issues

**"No Python interpreter found"**
- Install Python and ensure it's in your PATH
- Try running `python --version` in Command Prompt

**"GEMINI_API_KEY environment variable not set"**
- Set the environment variable as shown in setup
- Restart your terminal/File Explorer after setting

**"Failed to install context menu"**
- Run as Administrator
- Ensure you have write permissions to the registry

### Debug Mode

Enable debug logging:
```bash
set RUST_LOG=promptfile=debug
promptfile prompt "C:\path\to\folder"
```

## Development

### Building from Source

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test
```

### Dependencies

- **clap**: Command-line argument parsing
- **reqwest**: HTTP client for Gemini API
- **tokio**: Async runtime
- **colored**: Terminal colors
- **log/env_logger**: Logging framework
- **backoff**: Retry logic with exponential backoff
- **winreg**: Windows registry integration

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

[Add your license here]

## Changelog

### v0.2.0 (Security Update)
- Added comprehensive security analysis
- Implemented user approval for dangerous operations
- Added script execution timeout (5 minutes)
- Enhanced error handling with retry logic
- Comprehensive logging system
- Better API error handling with exponential backoff

### v0.1.0
- Initial release
- Basic AI script generation
- Windows context menu integration
- Automatic Python package installation
- **Execution Timeout**: Scripts automatically terminated after 5 minutes
- **Detailed Warnings**: Clear explanation of potential risks before execution
- **Comprehensive Logging**: All operations logged for audit trail

## Installation

### Prerequisites

1. **Python**: Install Python 3.7+ and ensure it's in your PATH
2. **Gemini API Key**: Get your free API key from [Google AI Studio](https://makersuite.google.com/app/apikey)

### Setup

1. **Download and Build**:
   ```bash
   git clone <repository-url>
   cd PromptFile
   cargo build --release
   ```

2. **Set Environment Variable**:
   ```bash
   # Windows Command Prompt
   setx GEMINI_API_KEY "your-api-key-here"
   
   # PowerShell
   [Environment]::SetEnvironmentVariable("GEMINI_API_KEY", "your-api-key-here", "User")
   ```

3. **Install Context Menu Integration**:
   ```bash
   ./target/release/promptfile.exe install
   ```

## Usage

### Via Context Menu
1. Right-click on any folder in Windows File Explorer
2. Select "PromptFile Command"
3. Enter your natural language prompt
4. Review and approve any security warnings
5. Watch as the AI generates and executes the appropriate script

### Via Command Line
```bash
# Process a specific folder
promptfile prompt "C:\path\to\folder"

# Install context menu
promptfile install

# Remove context menu
promptfile uninstall
```

## Example Prompts

- "Create a summary report of all files in this folder"
- "Rename all images to include their creation date"
- "Convert all Word documents to PDF"
- "Organize files by type into subfolders"
- "Generate a CSV inventory of all files with sizes"
- "Create thumbnails for all images"

## Configuration

### Environment Variables

- `GEMINI_API_KEY`: Your Google AI Studio API key (required)
- `RUST_LOG`: Logging level (default: `promptfile=info`)

### Logging

Logs are written to stdout with timestamps. Set `RUST_LOG=promptfile=debug` for verbose logging.

## Security Best Practices

1. **Review Scripts**: Always review security warnings before approving script execution
2. **Backup Important Data**: Create backups before running scripts on important folders
3. **Test First**: Try scripts on test folders before using on production data
4. **Monitor Logs**: Check logs for any unexpected behavior
5. **API Key Security**: Keep your Gemini API key secure and never share it

## Troubleshooting

### Common Issues

**"No Python interpreter found"**
- Install Python and ensure it's in your PATH
- Try running `python --version` in Command Prompt

**"GEMINI_API_KEY environment variable not set"**
- Set the environment variable as shown in setup
- Restart your terminal/File Explorer after setting

**"Failed to install context menu"**
- Run as Administrator
- Ensure you have write permissions to the registry

### Debug Mode

Enable debug logging:
```bash
set RUST_LOG=promptfile=debug
promptfile prompt "C:\path\to\folder"
```

## Development

### Building from Source

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test
```

### Dependencies

- **clap**: Command-line argument parsing
- **reqwest**: HTTP client for Gemini API
- **tokio**: Async runtime
- **colored**: Terminal colors
- **log/env_logger**: Logging framework
- **backoff**: Retry logic with exponential backoff
- **winreg**: Windows registry integration

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

[Add your license here]

## Changelog

### v0.2.0 (Security Update)
- Added comprehensive security analysis
- Implemented user approval for dangerous operations
- Added script execution timeout (5 minutes)
- Enhanced error handling with retry logic
- Comprehensive logging system
- Better API error handling with exponential backoff

### v0.1.0
- Initial release
- Basic AI script generation
- Windows context menu integration
- Automatic Python package installation
- **Execution Timeout**: Scripts automatically terminated after 5 minutes
- **Detailed Warnings**: Clear explanation of potential risks before execution
- **Comprehensive Logging**: All operations logged for audit trail

## Installation

### Prerequisites

1. **Python**: Install Python 3.7+ and ensure it's in your PATH
2. **Gemini API Key**: Get your free API key from [Google AI Studio](https://makersuite.google.com/app/apikey)

### Setup

1. **Download and Build**:
   ```bash
   git clone <repository-url>
   cd PromptFile
   cargo build --release
   ```

2. **Set Environment Variable**:
   ```bash
   # Windows Command Prompt
   setx GEMINI_API_KEY "your-api-key-here"
   
   # PowerShell
   [Environment]::SetEnvironmentVariable("GEMINI_API_KEY", "your-api-key-here", "User")
   ```

3. **Install Context Menu Integration**:
   ```bash
   ./target/release/promptfile.exe install
   ```

## Usage

### Via Context Menu
1. Right-click on any folder in Windows File Explorer
2. Select "PromptFile Command"
3. Enter your natural language prompt
4. Review and approve any security warnings
5. Watch as the AI generates and executes the appropriate script

### Via Command Line
```bash
# Process a specific folder
promptfile prompt "C:\path\to\folder"

# Install context menu
promptfile install

# Remove context menu
promptfile uninstall
```

## Example Prompts

- "Create a summary report of all files in this folder"
- "Rename all images to include their creation date"
- "Convert all Word documents to PDF"
- "Organize files by type into subfolders"
- "Generate a CSV inventory of all files with sizes"
- "Create thumbnails for all images"

## Configuration

### Environment Variables

- `GEMINI_API_KEY`: Your Google AI Studio API key (required)
- `RUST_LOG`: Logging level (default: `promptfile=info`)

### Logging

Logs are written to stdout with timestamps. Set `RUST_LOG=promptfile=debug` for verbose logging.

## Security Best Practices

1. **Review Scripts**: Always review security warnings before approving script execution
2. **Backup Important Data**: Create backups before running scripts on important folders
3. **Test First**: Try scripts on test folders before using on production data
4. **Monitor Logs**: Check logs for any unexpected behavior
5. **API Key Security**: Keep your Gemini API key secure and never share it

## Troubleshooting

### Common Issues

**"No Python interpreter found"**
- Install Python and ensure it's in your PATH
- Try running `python --version` in Command Prompt

**"GEMINI_API_KEY environment variable not set"**
- Set the environment variable as shown in setup
- Restart your terminal/File Explorer after setting

**"Failed to install context menu"**
- Run as Administrator
- Ensure you have write permissions to the registry

### Debug Mode

Enable debug logging:
```bash
set RUST_LOG=promptfile=debug
promptfile prompt "C:\path\to\folder"
```

## Development

### Building from Source

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test
```

### Dependencies

- **clap**: Command-line argument parsing
- **reqwest**: HTTP client for Gemini API
- **tokio**: Async runtime
- **colored**: Terminal colors
- **log/env_logger**: Logging framework
- **backoff**: Retry logic with exponential backoff
- **winreg**: Windows registry integration

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

[Add your license here]

## Changelog

### v0.2.0 (Security Update)
- Added comprehensive security analysis
- Implemented user approval for dangerous operations
- Added script execution timeout (5 minutes)
- Enhanced error handling with retry logic
- Comprehensive logging system
- Better API error handling with exponential backoff

### v0.1.0
- Initial release
- Basic AI script generation
- Windows context menu integration
- Automatic Python package installation
- **Execution Timeout**: Scripts automatically terminated after 5 minutes
- **Detailed Warnings**: Clear explanation of potential risks before execution
- **Comprehensive Logging**: All operations logged for audit trail

## Installation

### Prerequisites

1. **Python**: Install Python 3.7+ and ensure it's in your PATH
2. **Gemini API Key**: Get your free API key from [Google AI Studio](https://makersuite.google.com/app/apikey)

### Setup

1. **Download and Build**:
   ```bash
   git clone <repository-url>
   cd PromptFile
   cargo build --release
   ```

2. **Set Environment Variable**:
   ```bash
   # Windows Command Prompt
   setx GEMINI_API_KEY "your-api-key-here"
   
   # PowerShell
   [Environment]::SetEnvironmentVariable("GEMINI_API_KEY", "your-api-key-here", "User")
   ```

3. **Install Context Menu Integration**:
   ```bash
   ./target/release/promptfile.exe install
   ```

## Usage

### Via Context Menu
1. Right-click on any folder in Windows File Explorer
2. Select "PromptFile Command"
3. Enter your natural language prompt
4. Review and approve any security warnings
5. Watch as the AI generates and executes the appropriate script

### Via Command Line
```bash
# Process a specific folder
promptfile prompt "C:\path\to\folder"

# Install context menu
promptfile install

# Remove context menu
promptfile uninstall
```

## Example Prompts

- "Create a summary report of all files in this folder"
- "Rename all images to include their creation date"
- "Convert all Word documents to PDF"
- "Organize files by type into subfolders"
- "Generate a CSV inventory of all files with sizes"
- "Create thumbnails for all images"

## Configuration

### Environment Variables

- `GEMINI_API_KEY`: Your Google AI Studio API key (required)
- `RUST_LOG`: Logging level (default: `promptfile=info`)

### Logging

Logs are written to stdout with timestamps. Set `RUST_LOG=promptfile=debug` for verbose logging.

## Security Best Practices

1. **Review Scripts**: Always review security warnings before approving script execution
2. **Backup Important Data**: Create backups before running scripts on important folders
3. **Test First**: Try scripts on test folders before using on production data
4. **Monitor Logs**: Check logs for any unexpected behavior
5. **API Key Security**: Keep your Gemini API key secure and never share it

## Troubleshooting

### Common Issues

**"No Python interpreter found"**
- Install Python and ensure it's in your PATH
- Try running `python --version` in Command Prompt

**"GEMINI_API_KEY environment variable not set"**
- Set the environment variable as shown in setup
- Restart your terminal/File Explorer after setting

**"Failed to install context menu"**
- Run as Administrator
- Ensure you have write permissions to the registry

### Debug Mode

Enable debug logging:
```bash
set RUST_LOG=promptfile=debug
promptfile prompt "C:\path\to\folder"
```

## Development

### Building from Source

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test
```

### Dependencies

- **clap**: Command-line argument parsing
- **reqwest**: HTTP client for Gemini API
- **tokio**: Async runtime
- **colored**: Terminal colors
- **log/env_logger**: Logging framework
- **backoff**: Retry logic with exponential backoff
- **winreg**: Windows registry integration

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

[Add your license here]

## Changelog

### v0.2.0 (Security Update)
- Added comprehensive security analysis
- Implemented user approval for dangerous operations
- Added script execution timeout (5 minutes)
- Enhanced error handling with retry logic
- Comprehensive logging system
- Better API error handling with exponential backoff

### v0.1.0
- Initial release
- Basic AI script generation
- Windows context menu integration
- Automatic Python package installation
- **Execution Timeout**: Scripts automatically terminated after 5 minutes
- **Detailed Warnings**: Clear explanation of potential risks before execution
- **Comprehensive Logging**: All operations logged for audit trail

## Installation

### Prerequisites

1. **Python**: Install Python 3.7+ and ensure it's in your PATH
2. **Gemini API Key**: Get your free API key from [Google AI Studio](https://makersuite.google.com/app/apikey)

### Setup

1. **Download and Build**:
   ```bash
   git clone <repository-url>
   cd PromptFile
   cargo build --release
   ```

2. **Set Environment Variable**:
   ```bash
   # Windows Command Prompt
   setx GEMINI_API_KEY "your-api-key-here"
   
   # PowerShell
   [Environment]::SetEnvironmentVariable("GEMINI_API_KEY", "your-api-key-here", "User")
   ```

3. **Install Context Menu Integration**:
   ```bash
   ./target/release/promptfile.exe install
   ```

## Usage

### Via Context Menu
1. Right-click on any folder in Windows File Explorer
2. Select "PromptFile Command"
3. Enter your natural language prompt
4. Review and approve any security warnings
5. Watch as the AI generates and executes the appropriate script

### Via Command Line
```bash
# Process a specific folder
promptfile prompt "C:\path\to\folder"

# Install context menu
promptfile install

# Remove context menu
promptfile uninstall
```

## Example Prompts

- "Create a summary report of all files in this folder"
- "Rename all images to include their creation date"
- "Convert all Word documents to PDF"
- "Organize files by type into subfolders"
- "Generate a CSV inventory of all files with sizes"
- "Create thumbnails for all images"

## Configuration

### Environment Variables

- `GEMINI_API_KEY`: Your Google AI Studio API key (required)
- `RUST_LOG`: Logging level (default: `promptfile=info`)

### Logging

Logs are written to stdout with timestamps. Set `RUST_LOG=promptfile=debug` for verbose logging.

## Security Best Practices

1. **Review Scripts**: Always review security warnings before approving script execution
2. **Backup Important Data**: Create backups before running scripts on important folders
3. **Test First**: Try scripts on test folders before using on production data
4. **Monitor Logs**: Check logs for any unexpected behavior
5. **API Key Security**: Keep your Gemini API key secure and never share it

## Troubleshooting

### Common Issues

**"No Python interpreter found"**
- Install Python and ensure it's in your PATH
- Try running `python --version` in Command Prompt

**"GEMINI_API_KEY environment variable not set"**
- Set the environment variable as shown in setup
- Restart your terminal/File Explorer after setting

**"Failed to install context menu"**
- Run as Administrator
- Ensure you have write permissions to the registry

### Debug Mode

Enable debug logging:
```bash
set RUST_LOG=promptfile=debug
promptfile prompt "C:\path\to\folder"
```

## Development

### Building from Source

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test
```

### Dependencies

- **clap**: Command-line argument parsing
- **reqwest**: HTTP client for Gemini API
- **tokio**: Async runtime
- **colored**: Terminal colors
- **log/env_logger**: Logging framework
- **backoff**: Retry logic with exponential backoff
- **winreg**: Windows registry integration

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

[Add your license here]

## Changelog

### v0.2.0 (Security Update)
- Added comprehensive security analysis
- Implemented user approval for dangerous operations
- Added script execution timeout (5 minutes)
- Enhanced error handling with retry logic
- Comprehensive logging system
- Better API error handling with exponential backoff

### v0.1.0
- Initial release
- Basic AI script generation
- Windows context menu integration
- Automatic Python package installation
- **Execution Timeout**: Scripts automatically terminated after 5 minutes
- **Detailed Warnings**: Clear explanation of potential risks before execution
- **Comprehensive Logging**: All operations logged for audit trail

## Installation

### Prerequisites

1. **Python**: Install Python 3.7+ and ensure it's in your PATH
2. **Gemini API Key**: Get your free API key from [Google AI Studio](https://makersuite.google.com/app/apikey)

### Setup

1. **Download and Build**:
   ```bash
   git clone <repository-url>
   cd PromptFile
   cargo build --release
   ```

2. **Set Environment Variable**:
   ```bash
   # Windows Command Prompt
   setx GEMINI_API_KEY "your-api-key-here"
   
   # PowerShell
   [Environment]::SetEnvironmentVariable("GEMINI_API_KEY", "your-api-key-here", "User")
   ```

3. **Install Context Menu Integration**:
   ```bash
   ./target/release/promptfile.exe install
   ```

## Usage

### Via Context Menu
1. Right-click on any folder in Windows File Explorer
2. Select "PromptFile Command"
3. Enter your natural language prompt
4. Review and approve any security warnings
5. Watch as the AI generates and executes the appropriate script

### Via Command Line
```bash
# Process a specific folder
promptfile prompt "C:\path\to\folder"

# Install context menu
promptfile install

# Remove context menu
promptfile uninstall
```

## Example Prompts

- "Create a summary report of all files in this folder"
- "Rename all images to include their creation date"
- "Convert all Word documents to PDF"
- "Organize files by type into subfolders"
- "Generate a CSV inventory of all files with sizes"
- "Create thumbnails for all images"

## Configuration

### Environment Variables

- `GEMINI_API_KEY`: Your Google AI Studio API key (required)
- `RUST_LOG`: Logging level (default: `promptfile=info`)

### Logging

Logs are written to stdout with timestamps. Set `RUST_LOG=promptfile=debug` for verbose logging.

## Security Best Practices

1. **Review Scripts**: Always review security warnings before approving script execution
2. **Backup Important Data**: Create backups before running scripts on important folders
3. **Test First**: Try scripts on test folders before using on production data
4. **Monitor Logs**: Check logs for any unexpected behavior
5. **API Key Security**: Keep your Gemini API key secure and never share it

## Troubleshooting

### Common Issues

**"No Python interpreter found"**
- Install Python and ensure it's in your PATH
- Try running `python --version` in Command Prompt

**"GEMINI_API_KEY environment variable not set"**
- Set the environment variable as shown in setup
- Restart your terminal/File Explorer after setting

**"Failed to install context menu"**
- Run as Administrator
- Ensure you have write permissions to the registry

### Debug Mode

Enable debug logging:
```bash
set RUST_LOG=promptfile=debug
promptfile prompt "C:\path\to\folder"
```

## Development

### Building from Source

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test
```

### Dependencies

- **clap**: Command-line argument parsing
- **reqwest**: HTTP client for Gemini API
- **tokio**: Async runtime
- **colored**: Terminal colors
- **log/env_logger**: Logging framework
- **backoff**: Retry logic with exponential backoff
- **winreg**: Windows registry integration

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

[Add your license here]

## Changelog

### v0.2.0 (Security Update)
- Added comprehensive security analysis
- Implemented user approval for dangerous operations
- Added script execution timeout (5 minutes)
- Enhanced error handling with retry logic
- Comprehensive logging system
- Better API error handling with exponential backoff

### v0.1.0
- Initial release
- Basic AI script generation
- Windows context menu integration
- Automatic Python package installation
- **Execution Timeout**: Scripts automatically terminated after 5 minutes
- **Detailed Warnings**: Clear explanation of potential risks before execution
- **Comprehensive Logging**: All operations logged for audit trail

## Installation

### Prerequisites

1. **Python**: Install Python 3.7+ and ensure it's in your PATH
2. **Gemini API Key**: Get your free API key from [Google AI Studio](https://makersuite.google.com/app/apikey)

### Setup

1. **Download and Build**:
   ```bash
   git clone <repository-url>
   cd PromptFile
   cargo build --release
   ```

2. **Set Environment Variable**:
   ```bash
   # Windows Command Prompt
   setx GEMINI_API_KEY "your-api-key-here"
   
   # PowerShell
   [Environment]::SetEnvironmentVariable("GEMINI_API_KEY", "your-api-key-here", "User")
   ```

3. **Install Context Menu Integration**:
   ```bash
   ./target/release/promptfile.exe install
   ```

## Usage

### Via Context Menu
1. Right-click on any folder in Windows File Explorer
2. Select "PromptFile Command"
3. Enter your natural language prompt
4. Review and approve any security warnings
5. Watch as the AI generates and executes the appropriate script

### Via Command Line
```bash
# Process a specific folder
promptfile prompt "C:\path\to\folder"

# Install context menu
promptfile install

# Remove context menu
promptfile uninstall
```

## Example Prompts

- "Create a summary report of all files in this folder"
- "Rename all images to include their creation date"
- "Convert all Word documents to PDF"
- "Organize files by type into subfolders"
- "Generate a CSV inventory of all files with sizes"
- "Create thumbnails for all images"

## Configuration

### Environment Variables

- `GEMINI_API_KEY`: Your Google AI Studio API key (required)
- `RUST_LOG`: Logging level (default: `promptfile=info`)

### Logging

Logs are written to stdout with timestamps. Set `RUST_LOG=promptfile=debug` for verbose logging.

## Security Best Practices

1. **Review Scripts**: Always review security warnings before approving script execution
2. **Backup Important Data**: Create backups before running scripts on important folders
3. **Test First**: Try scripts on test folders before using on production data
4. **Monitor Logs**: Check logs for any unexpected behavior
5. **API Key Security**: Keep your Gemini API key secure and never share it

## Troubleshooting

### Common Issues

**"No Python interpreter found"**
- Install Python and ensure it's in your PATH
- Try running `python --version` in Command Prompt

**"GEMINI_API_KEY environment variable not set"**
- Set the environment variable as shown in setup
- Restart your terminal/File Explorer after setting

**"Failed to install context menu"**
- Run as Administrator
- Ensure you have write permissions to the registry

### Debug Mode

Enable debug logging:
```bash
set RUST_LOG=promptfile=debug
promptfile prompt "C:\path\to\folder"
```

## Development

### Building from Source

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test
```

### Dependencies

- **clap**: Command-line argument parsing
- **reqwest**: HTTP client for Gemini API
- **tokio**: Async runtime
- **colored**: Terminal colors
- **log/env_logger**: Logging framework
- **backoff**: Retry logic with exponential backoff
- **winreg**: Windows registry integration

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

[Add your license here]

## Changelog

### v0.2.0 (Security Update)
- Added comprehensive security analysis
- Implemented user approval for dangerous operations
- Added script execution timeout (5 minutes)
- Enhanced error handling with retry logic
- Comprehensive logging system
- Better API error handling with exponential backoff

### v0.1.0
- Initial release
- Basic AI script generation
- Windows context menu integration
- Automatic Python package installation
- **Execution Timeout**: Scripts automatically terminated after 5 minutes
- **Detailed Warnings**: Clear explanation of potential risks before execution
- **Comprehensive Logging**: All operations logged for audit trail

## Installation

### Prerequisites

1. **Python**: Install Python 3.7+ and ensure it's in your PATH
2. **Gemini API Key**: Get your free API key from [Google AI Studio](https://makersuite.google.com/app/apikey)

### Setup

1. **Download and Build**:
   ```bash
   git clone <repository-url>
   cd PromptFile
   cargo build --release
   ```

2. **Set Environment Variable**:
   ```bash
   # Windows Command Prompt
   setx GEMINI_API_KEY "your-api-key-here"
   
   # PowerShell
   [Environment]::SetEnvironmentVariable("GEMINI_API_KEY", "your-api-key-here", "User")
   ```

3. **Install Context Menu Integration**:
   ```bash
   ./target/release/promptfile.exe install
   ```

## Usage

### Via Context Menu
1. Right-click on any folder in Windows File Explorer
2. Select "PromptFile Command"
3. Enter your natural language prompt
4. Review and approve any security warnings
5. Watch as the AI generates and executes the appropriate script

### Via Command Line
```bash
# Process a specific folder
promptfile prompt "C:\path\to\folder"

# Install context menu
promptfile install

# Remove context menu
promptfile uninstall
```

## Example Prompts

- "Create a summary report of all files in this folder"
- "Rename all images to include their creation date"
- "Convert all Word documents to PDF"
- "Organize files by type into subfolders"
- "Generate a CSV inventory of all files with sizes"
- "Create thumbnails for all images"

## Configuration

### Environment Variables

- `GEMINI_API_KEY`: Your Google AI Studio API key (required)
- `RUST_LOG`: Logging level (default: `promptfile=info`)

### Logging

Logs are written to stdout with timestamps. Set `RUST_LOG=promptfile=debug` for verbose logging.

## Security Best Practices

1. **Review Scripts**: Always review security warnings before approving script execution
2. **Backup Important Data**: Create backups before running scripts on important folders
3. **Test First**: Try scripts on test folders before using on production data
4. **Monitor Logs**: Check logs for any unexpected behavior
5. **API Key Security**: Keep your Gemini API key secure and never share it

## Troubleshooting

### Common Issues

**"No Python interpreter found"**
- Install Python and ensure it's in your PATH
- Try running `python --version` in Command Prompt

**"GEMINI_API_KEY environment variable not set"**
- Set the environment variable as shown in setup
- Restart your terminal/File Explorer after setting

**"Failed to install context menu"**
- Run as Administrator
- Ensure you have write permissions to the registry

### Debug Mode

Enable debug logging:
```bash
set RUST_LOG=promptfile=debug
promptfile prompt "C:\path\to\folder"
```

## Development

### Building from Source

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test
```

### Dependencies

- **clap**: Command-line argument parsing
- **reqwest**: HTTP client for Gemini API
- **tokio**: Async runtime
- **colored**: Terminal colors
- **log/env_logger**: Logging framework
- **backoff**: Retry logic with exponential backoff
- **winreg**: Windows registry integration

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

[Add your license here]

## Changelog

### v0.2.0 (Security Update)
- Added comprehensive security analysis
- Implemented user approval for dangerous operations
- Added script execution timeout (5 minutes)
- Enhanced error handling with retry logic
- Comprehensive logging system
- Better API error handling with exponential backoff

### v0.1.0
- Initial release
- Basic AI script generation
- Windows context menu integration
- Automatic Python package installation
- **Execution Timeout**: Scripts automatically terminated after 5 minutes
- **Detailed Warnings**: Clear explanation of potential risks before execution
- **Comprehensive Logging**: All operations logged for audit trail

## Installation

### Prerequisites

1. **Python**: Install Python 3.7+ and ensure it's in your PATH
2. **Gemini API Key**: Get your free API key from [Google AI Studio](https://makersuite.google.com/app/apikey)

### Setup

1. **Download and Build**:
   ```bash
   git clone <repository-url>
   cd PromptFile
   cargo build --release
   ```

2. **Set Environment Variable**:
   ```bash
   # Windows Command Prompt
   setx GEMINI_API_KEY "your-api-key-here"
   
   # PowerShell
   [Environment]::SetEnvironmentVariable("GEMINI_API_KEY", "your-api-key-here", "User")
   ```

3. **Install Context Menu Integration**:
   ```bash
   ./target/release/promptfile.exe install
   ```

## Usage

### Via Context Menu
1. Right-click on any folder in Windows File Explorer
2. Select "PromptFile Command"
3. Enter your natural language prompt
4. Review and approve any security warnings
5. Watch as the AI generates and executes the appropriate script

### Via Command Line
```bash
# Process a specific folder
promptfile prompt "C:\path\to\folder"

# Install context menu
promptfile install

# Remove context menu
promptfile uninstall
```

## Example Prompts

- "Create a summary report of all files in this folder"
- "Rename all images to include their creation date"
- "Convert all Word documents to PDF"
- "Organize files by type into subfolders"
- "Generate a CSV inventory of all files with sizes"
- "Create thumbnails for all images"

## Configuration

### Environment Variables

- `GEMINI_API_KEY`: Your Google AI Studio API key (required)
- `RUST_LOG`: Logging level (default: `promptfile=info`)

### Logging

Logs are written to stdout with timestamps. Set `RUST_LOG=promptfile=debug` for verbose logging.

## Security Best Practices

1. **Review Scripts**: Always review security warnings before approving script execution
2. **Backup Important Data**: Create backups before running scripts on important folders
3. **Test First**: Try scripts on test folders before using on production data
4. **Monitor Logs**: Check logs for any unexpected behavior
5. **API Key Security**: Keep your Gemini API key secure and never share it

## Troubleshooting

### Common Issues

**"No Python interpreter found"**
- Install Python and ensure it's in your PATH
- Try running `python --version` in Command Prompt

**"GEMINI_API_KEY environment variable not set"**
- Set the environment variable as shown in setup
- Restart your terminal/File Explorer after setting

**"Failed to install context menu"**
- Run as Administrator
- Ensure you have write permissions to the registry

### Debug Mode

Enable debug logging:
```bash
set RUST_LOG=promptfile=debug
promptfile prompt "C:\path\to\folder"
```

## Development

### Building from Source

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test
```

### Dependencies

- **clap**: Command-line argument parsing
- **reqwest**: HTTP client for Gemini API
- **tokio**: Async runtime
- **colored**: Terminal colors
- **log/env_logger**: Logging framework
- **backoff**: Retry logic with exponential backoff
- **winreg**: Windows registry integration

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

[Add your license here]

## Changelog

### v0.2.0 (Security Update)
- Added comprehensive security analysis
- Implemented user approval for dangerous operations
- Added script execution timeout (5 minutes)
- Enhanced error handling with retry logic
- Comprehensive logging system
- Better API error handling with exponential backoff

### v0.1.0
- Initial release
- Basic AI script generation
- Windows context menu integration
- Automatic Python package installation
- **Execution Timeout**: Scripts automatically terminated after 5 minutes
- **Detailed Warnings**: Clear explanation of potential risks before execution
- **Comprehensive Logging**: All operations logged for audit trail

## Installation

### Prerequisites

1. **Python**: Install Python 3.7+ and ensure it's in your PATH
2. **Gemini API Key**: Get your free API key from [Google AI Studio](https://makersuite.google.com/app/apikey)

### Setup

1. **Download and Build**:
   ```bash
   git clone <repository-url>
   cd PromptFile
   cargo build --release
   ```

2. **Set Environment Variable**:
   ```bash
   # Windows Command Prompt
   setx GEMINI_API_KEY "your-api-key-here"
   
   # PowerShell
   [Environment]::SetEnvironmentVariable("GEMINI_API_KEY", "your-api-key-here", "User")
   ```

3. **Install Context Menu Integration**:
   ```bash
   ./target/release/promptfile.exe install
   ```

## Usage

### Via Context Menu
1. Right-click on any folder in Windows File Explorer
2. Select "PromptFile Command"
3. Enter your natural language prompt
4. Review and approve any security warnings
5. Watch as the AI generates and executes the appropriate script

### Via Command Line
```bash
# Process a specific folder
promptfile prompt "C:\path\to\folder"

# Install context menu
promptfile install

# Remove context menu
promptfile uninstall
```

## Example Prompts

- "Create a summary report of all files in this folder"
- "Rename all images to include their creation date"
- "Convert all Word documents to PDF"
- "Organize files by type into subfolders"
- "Generate a CSV inventory of all files with sizes"
- "Create thumbnails for all images"

## Configuration

### Environment Variables

- `GEMINI_API_KEY`: Your Google AI Studio API key (required)
- `RUST_LOG`: Logging level (default: `promptfile=info`)

### Logging

Logs are written to stdout with timestamps. Set `RUST_LOG=promptfile=debug` for verbose logging.

## Security Best Practices

1. **Review Scripts**: Always review security warnings before approving script execution
2. **Backup Important Data**: Create backups before running scripts on important folders
3. **Test First**: Try scripts on test folders before using on production data
4. **Monitor Logs**: Check logs for any unexpected behavior
5. **API Key Security**: Keep your Gemini API key secure and never share it

## Troubleshooting

### Common Issues

**"No Python interpreter found"**
- Install Python and ensure it's in your PATH
- Try running `python --version` in Command Prompt

**"GEMINI_API_KEY environment variable not set"**
- Set the environment variable as shown in setup
- Restart your terminal/File Explorer after setting

**"Failed to install context menu"**
- Run as Administrator
- Ensure you have write permissions to the registry

### Debug Mode

Enable debug logging:
```bash
set RUST_LOG=promptfile=debug
promptfile prompt "C:\path\to\folder"
```

## Development

### Building from Source

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test
```

### Dependencies

- **clap**: Command-line argument parsing
- **reqwest**: HTTP client for Gemini API
- **tokio**: Async runtime
- **colored**: Terminal colors
- **log/env_logger**: Logging framework
- **backoff**: Retry logic with exponential backoff
- **winreg**: Windows registry integration

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

[Add your license here]

## Changelog

### v0.2.0 (Security Update)
- Added comprehensive security analysis
- Implemented user approval for dangerous operations
- Added script execution timeout (5 minutes)
- Enhanced error handling with retry logic
- Comprehensive logging system
- Better API error handling with exponential backoff

### v0.1.0
- Initial release
- Basic AI script generation
- Windows context menu integration
- Automatic Python package installation
- **Execution Timeout**: Scripts automatically terminated after 5 minutes
- **Detailed Warnings**: Clear explanation of potential risks before execution
- **Comprehensive Logging**: All operations logged for audit trail

## Installation

### Prerequisites

1. **Python**: Install Python 3.7+ and ensure it's in your PATH
2. **Gemini API Key**: Get your free API key from [Google AI Studio](https://makersuite.google.com/app/apikey)

### Setup

1. **Download and Build**:
   ```bash
   git clone <repository-url>
   cd PromptFile
   cargo build --release
   ```

2. **Set Environment Variable**:
   ```bash
   # Windows Command Prompt
   setx GEMINI_API_KEY "your-api-key-here"
   
   # PowerShell
   [Environment]::SetEnvironmentVariable("GEMINI_API_KEY", "your-api-key-here", "User")
   ```

3. **Install Context Menu Integration**:
   ```bash
   ./target/release/promptfile.exe install
   ```

## Usage

### Via Context Menu
1. Right-click on any folder in Windows File Explorer
2. Select "PromptFile Command"
3. Enter your natural language prompt
4. Review and approve any security warnings
5. Watch as the AI generates and executes the appropriate script

### Via Command Line
```bash
# Process a specific folder
promptfile prompt "C:\path\to\folder"

# Install context menu
promptfile install

# Remove context menu
promptfile uninstall
```

## Example Prompts

- "Create a summary report of all files in this folder"
- "Rename all images to include their creation date"
- "Convert all Word documents to PDF"
- "Organize files by type into subfolders"
- "Generate a CSV inventory of all files with sizes"
- "Create thumbnails for all images"

## Configuration

### Environment Variables

- `GEMINI_API_KEY`: Your Google AI Studio API key (required)
- `RUST_LOG`: Logging level (default: `promptfile=info`)

### Logging

Logs are written to stdout with timestamps. Set `RUST_LOG=promptfile=debug` for verbose logging.

## Security Best Practices

1. **Review Scripts**: Always review security warnings before approving script execution
2. **Backup Important Data**: Create backups before running scripts on important folders
3. **Test First**: Try scripts on test folders before using on production data
4. **Monitor Logs**: Check logs for any unexpected behavior
5. **API Key Security**: Keep your Gemini API key secure and never share it

## Troubleshooting

### Common Issues

**"No Python interpreter found"**
- Install Python and ensure it's in your PATH
- Try running `python --version` in Command Prompt

**"GEMINI_API_KEY environment variable not set"**
- Set the environment variable as shown in setup
- Restart your terminal/File Explorer after setting

**"Failed to install context menu"**
- Run as Administrator
- Ensure you have write permissions to the registry

### Debug Mode

Enable debug logging:
```bash
set RUST_LOG=promptfile=debug
promptfile prompt "C:\path\to\folder"
```

## Development

### Building from Source

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test
```

### Dependencies

- **clap**: Command-line argument parsing
- **reqwest**: HTTP client for Gemini API
- **tokio**: Async runtime
- **colored**: Terminal colors
- **log/env_logger**: Logging framework
- **backoff**: Retry logic with exponential backoff
- **winreg**: Windows registry integration

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

[Add your license here]

## Changelog

### v0.2.0 (Security Update)
- Added comprehensive security analysis
- Implemented user approval for dangerous operations
- Added script execution timeout (5 minutes)
- Enhanced error handling with retry logic
- Comprehensive logging system
- Better API error handling with exponential backoff

### v0.1.0
- Initial release
- Basic AI script generation
- Windows context menu integration
- Automatic Python package installation
- **Execution Timeout**: Scripts automatically terminated after 5 minutes
- **Detailed Warnings**: Clear explanation of potential risks before execution
- **Comprehensive Logging**: All operations logged for audit trail

## Installation

### Prerequisites

1. **Python**: Install Python 3.7+ and ensure it's in your PATH
2. **Gemini API Key**: Get your free API key from [Google AI Studio](https://makersuite.google.com/app/apikey)

### Setup

1. **Download and Build**:
   ```bash
   git clone <repository-url>
   cd PromptFile
   cargo build --release
   ```

2. **Set Environment Variable**:
   ```bash
   # Windows Command Prompt
   setx GEMINI_API_KEY "your-api-key-here"
   
   # PowerShell
   [Environment]::SetEnvironmentVariable("GEMINI_API_KEY", "your-api-key-here", "User")
   ```

3. **Install Context Menu Integration**:
   ```bash
   ./target/release/promptfile.exe install
   ```

## Usage

### Via Context Menu
1. Right-click on any folder in Windows File Explorer
2. Select "PromptFile Command"
3. Enter your natural language prompt
4. Review and approve any security warnings
5. Watch as the AI generates and executes the appropriate script

### Via Command Line
```bash
# Process a specific folder
promptfile prompt "C:\path\to\folder"

# Install context menu
promptfile install

# Remove context menu
promptfile uninstall
```

## Example Prompts

- "Create a summary report of all files in this folder"
- "Rename all images to include their creation date"
- "Convert all Word documents to PDF"
- "Organize files by type into subfolders"
- "Generate a CSV inventory of all files with sizes"
- "Create thumbnails for all images"

## Configuration

### Environment Variables

- `GEMINI_API_KEY`: Your Google AI Studio API key (required)
- `RUST_LOG`: Logging level (default: `promptfile=info`)

### Logging

Logs are written to stdout with timestamps. Set `RUST_LOG=promptfile=debug` for verbose logging.

## Security Best Practices

1. **Review Scripts**: Always review security warnings before approving script execution
2. **Backup Important Data**: Create backups before running scripts on important folders
3. **Test First**: Try scripts on test folders before using on production data
4. **Monitor Logs**: Check logs for any unexpected behavior
5. **API Key Security**: Keep your Gemini API key secure and never share it

## Troubleshooting

### Common Issues

**"No Python interpreter found"**
- Install Python and ensure it's in your PATH
- Try running `python --version` in Command Prompt

**"GEMINI_API_KEY environment variable not set"**
- Set the environment variable as shown in setup
- Restart your terminal/File Explorer after setting

**"Failed to install context menu"**
- Run as Administrator
- Ensure you have write permissions to the registry

### Debug Mode

Enable debug logging:
```bash
set RUST_LOG=promptfile=debug
promptfile prompt "C:\path\to\folder"
```

## Development

### Building from Source

```bash
# Debug build
cargo build

# Release build
cargo build --release

# Run tests
cargo test
```

### Dependencies

- **clap**: Command-line argument parsing
- **reqwest**: HTTP client for Gemini API
- **tokio**: Async runtime
- **colored**: Terminal colors
- **log/env_logger**: Logging framework
- **backoff**: Retry logic with exponential backoff
- **winreg**: Windows registry integration

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests if applicable
5. Submit a pull request

## License

[Add your license here]

## Changelog

### v0.2.0 (Security Update)
- Added comprehensive security analysis
- Implemented user approval for dangerous operations
- Added script execution timeout (5 minutes)
- Enhanced error handling with retry logic
- Comprehensive logging system
- Better API error handling with exponential backoff

### v0.1.0
- Initial release
- Basic AI script generation
- Windows context menu integration
- Automatic Python package installation
- **Execution Timeout**: Scripts automatically terminated after 5 minutes
- **Detailed Warnings**: Clear explanation of potential risks before execution
- **Comprehensive Logging**: All operations logged for audit trail

## Installation

### Prerequisites

1. **Python**: Install Python 3.7+ and ensure it's in your PATH
2. **Gemini API Key**: Get your free API key from [Google AI Studio](https://makersuite.google.com/app/apikey)

### Setup

1. **Download and Build**:
   ```bash
   git clone <repository-url>
   cd PromptFile
   cargo build --release
   ```

2. **Set Environment Variable**:
   ```bash
   # Windows Command Prompt
   setx GEMINI_API_KEY "your-api-key-here"
   
   # PowerShell
   [Environment]::SetEnvironmentVariable("GEMINI_API_KEY", "your-api-key-here", "User")
   ```

3. **Install Context Menu Integration**:
   ```bash
   ./target/release/promptfile.exe install
   ```

## Usage

### Via Context Menu
1. Right-click on any folder in Windows File Explorer
2. Select "PromptFile Command"
3. Enter your natural language prompt
4. Review and approve any security warnings
5. Watch as the AI generates and executes the appropriate script

### Via Command Line
```bash
# Process a specific folder
promptfile prompt "C:\path\to\folder"

# Install context menu
promptfile install

# Remove context menu
promptfile uninstall
```

## Example Prompts

- "Create a summary report of all files in this folder"
- "Rename all images to include their creation date"
- "Convert all Word documents to PDF"
- "Organize files by type into subfolders"
- "Generate a CSV inventory of all files with sizes"
- "Create thumbnails for all images"

## Configuration

### Environment Variables

- `GEMINI_API_KEY`: Your Google AI Studio API key (required)
- `RUST_LOG`: Logging level (default: `promptfile=info`)

### Logging

Logs are written to stdout with timestamps. Set `RUST_LOG=promptfile=debug` for verbose logging.

## Security Best Practices

1. **Review Scripts**: Always review security warnings before approving script execution
2. **Backup Important Data**: Create backups before running scripts on important folders
3. **Test First**: Try scripts on test folders before using on production data
4. **Monitor Logs**: Check logs for any unexpected behavior
5. **API Key Security**: Keep your Gemini API key secure and never share it

## Troubleshooting

### Common Issues

**"No Python interpreter found"**
- Install Python and ensure it's in your PATH
- Try running `python --version` in Command Prompt

**"GEMINI_API_KEY environment variable not set"**
- Set the environment variable as shown in setup
- Restart your terminal
