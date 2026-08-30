# TasCat (TasKat)

**AI-powered file operations from your File Explorer right-click menu.**

TasCat adds a right-click context menu entry to Windows File Explorer. Point it at any folder, describe what you want done in plain English, and it uses Google's Gemini API to write and run a Python script that does it — with previews, permission prompts, and one-command rollback so you're never left guessing what changed.

> ⚠️ **Heads up:** TasCat generates and executes Python code against your files based on AI output. Review the dry-run preview before approving any operation, and keep backups of anything important. AI-generated scripts can make mistakes.

---

## Features

- **Right-click integration** — Adds a "TasCat" entry to the folder and folder-background context menus in Windows Explorer.
- **Natural-language prompts** — Describe a task ("rename all these to lowercase", "convert the PDFs to text", "delete anything older than 30 days") and TasCat generates a Python script to do it.
- **Permission prompts & dry runs** — Optionally require confirmation before any file is deleted or overwritten, with a preview of what will be created, modified, or deleted.
- **Snapshots & rollback** — Take a snapshot before risky operations and undo back to it later with the `#undo` command.
- **Saved commands** — Turn a successful script into a reusable slash-style command with `#save`, then re-run it any time with `#<name>`.
- **Plugin system** — Drop `.toml` plugin templates (like the included `resize` plugin) into your plugin folder to expose parameterized, reusable operations as `#commands`.
- **Automatic error recovery** — If a generated script fails, TasCat feeds the error back to Gemini and retries with a fixed version (up to 3 attempts).
- **Multi-provider config** — `config/models.toml` defines API routing across Gemini, Claude, OpenAI, and a local Ollama fallback (Gemini is the only provider wired up by default).

## How it works

1. Right-click a folder (or right-click inside a folder) and choose **TasCat**.
2. TasCat scans the folder and builds a summary of its structure and contents.
3. You type a prompt describing what you want done.
4. Gemini generates a Python script based on your prompt and the folder context.
5. TasCat shows a preview of the changes (if permission prompts are enabled) and asks for confirmation.
6. The script runs. If it fails, TasCat asks Gemini to fix it and retries automatically.
7. You can undo the operation later via `#undo`, restoring from a snapshot.

## Requirements

- Windows (the context-menu integration relies on the Windows Registry and `winreg`/`winapi`)
- [Rust](https://rustup.rs/) toolchain (to build from source)
- Python 3 on your `PATH` (used to run the generated scripts)
- A [Gemini API key](https://aistudio.google.com/app/apikey)
- A valid TasCat license (verified against Gumroad on first install)

## Installation

### Build from source

```bash
git clone https://github.com/VinegarChicken/TasKat.git
cd TasKat
cargo build --release
```

Or run the provided Windows build script:

```bat
build.bat
```

The compiled executable will be at `target\release\TasCat.exe`.

### Set up

```bat
:: Set your Gemini API key
setx GEMINI_API_KEY "your-api-key-here"

:: Install the right-click context menu (requires admin privileges)
target\release\TasCat.exe install
```

Running the executable with no arguments also walks you through installation, license verification, and Gemini API key setup interactively.

## Usage

```bat
TasCat install                  :: Install the right-click context menu
TasCat uninstall                :: Remove the right-click context menu
TasCat prompt "C:\some\folder"  :: Run TasCat directly against a folder
```

Once inside a prompt session, you can also use these built-in commands:

| Command      | Description                                          |
| ------------ | ----------------------------------------------------- |
| `#help`      | Show available commands                                |
| `#undo`      | List and restore a previous snapshot                   |
| `#save`      | Save the last successful script as a reusable command  |
| `#<name>`    | Run a saved command or an installed plugin              |
| `exit`/`quit`| Leave the prompt session                                |

## Plugins

Plugins are TOML templates that define a reusable, parameterized operation exposed as a `#command`. TasCat loads them from your user config directory's `TasCat/plugins` folder. The repo ships an example, `plugins/resize.toml`, which resizes images in the current folder:

```
#resize width=800 height=600
#resize width=1920 format=jpg
```

Each plugin defines a name, description, parameters, and a Python `script_template` with `{{parameter}}` placeholders that get substituted at run time.

## Configuration

`config/models.toml` controls which AI provider TasCat talks to and the fallback order if a request fails:

```toml
[providers]
gemini = { api_key_env = "GEMINI_API_KEY", enabled = true }
claude = { api_key_env = "ANTHROPIC_API_KEY", enabled = false }
openai = { api_key_env = "OPENAI_API_KEY", enabled = false }
ollama = { base_url = "http://localhost:11434/api/generate", model = "codellama", enabled = true }

[routing]
default_provider = "gemini"
fallback_order = ["claude", "openai", "ollama"]
```

Only Gemini is enabled and wired up out of the box; the other providers are placeholders for future support.

## Project structure

```
TasKat/
├── src/
│   ├── main.rs        # CLI entry point, install flow, license/API key setup
│   ├── context.rs     # Folder scanning and context summarization
│   ├── gemini.rs       # Gemini API integration (script generation, fixing, safety review)
│   ├── registry.rs     # Windows Registry context-menu install/uninstall
│   ├── runner.rs       # Executes generated Python scripts
│   ├── sandbox.rs      # Dry-run / change-preview logic
│   ├── rollback.rs     # Snapshot creation and rollback
│   ├── plugins.rs      # Plugin loading and template rendering
│   └── commands.rs     # Saved-command registry (#save / #<name>)
├── plugins/            # Example plugin templates (.toml)
├── config/             # AI provider configuration
├── build.rs            # Embeds the app icon into the Windows executable
└── build.bat           # Convenience build script
```

## License

TasCat requires a paid license, verified against Gumroad on install. See the in-app installer for the purchase link.
