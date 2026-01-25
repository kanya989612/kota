<a href="https://docs.rs/kota/latest/kota"><img src="https://img.shields.io/badge/docs-API Reference-dca282.svg" /></a> 
<a href="https://crates.io/crates/kota"><img src="https://img.shields.io/crates/v/kota.svg?color=dca282" /></a>  &nbsp;
# Kota
A lightweight AI code agent in Rust:  

![kota_screenshot](assert/screenshort.jpg)

## Setup

### Environment Configuration

Before running Kota, you need to create a `.env` file with your API configuration:

1. Copy the example environment file:
   ```bash
   cp .env.example .env
   ```

2. Edit the `.env` file and configure your API settings:
   ```env
   API_KEY=your-api-key-here
   MODEL_NAME=deepseek-chat
   ```

   - `API_KEY`: Your LLM provider API key
   - `MODEL_NAME`: The model to use (see supported models below)

## Installation

### As a CLI Tool
```bash
cargo install kota
```
Then start it:
```bash
kota
```

### As a Library
Add Kota to your `Cargo.toml`:
```toml
[dependencies]
kota = "0.1.3"
tokio = { version = "1.0", features = ["full"] }
anyhow = "1.0"
```

## Usage as a Library

Kota can be used as a library to build your own AI code agents. Here's a quick example:

```rust
use kota::kota_code::{AgentBuilder, ContextManager};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Create an agent
    let agent = AgentBuilder::new(
        "your-api-key".to_string(),
        "gpt-4".to_string()
    )?.build()?;

    // Create context manager for conversation history
    let mut context = ContextManager::new(".chat_sessions", "my-session".to_string())?;

    // Use the agent...
    Ok(())
}
```

### Library Features

- **Agent Builder**: Create customized AI agents with different LLM providers
- **Context Management**: Persistent conversation history with session support
- **Plan Management**: Structured task execution with dependencies
- **Skills System**: Specialized agent behaviors for different tasks
- **Built-in Tools**: File operations, code scanning, grep search, and more

## Supported Models

Kota supports various LLM providers:

- **OpenAI Compatible** - (e.g., gpt-4o, gpt-4o-mini, gpt-4-turbo, gpt-3.5-turbo...)
- **DeepSeek** - (e.g., deepseek-chat, deepseek-coder...)
- **Anthropic Claude** (e.g., claude-3-5-sonnet, claude-4-opus...)

### Other Providers
- Any OpenAI-compatible API endpoint can be configured
- Local models via Ollama or similar services

## CLI Features

### Interactive Commands

Kota provides an interactive CLI with the following commands:

- `/quit` or `/exit` - Exit the application
- `/config` - Show current model configuration
- `/help` - Show available commands
- `/history` - Show conversation history
- `/skills` - List all available skills
- `/skill <name>` - Activate a specific skill
- `/skill-off` - Deactivate current skill
- `/load <session_id>` - Load specific session
- `/sessions` - List all sessions
- `/delete <session_id>` - Delete a specific session

### Tab Completion

Kota supports intelligent tab completion for commands:

- Type partial commands (e.g., `/h`) and press **Tab** to auto-complete
- Commands are highlighted in green when recognized
- Use **Ctrl+C** to exit or **Ctrl+D** for EOF

**Example usage:**
```
❯ /h<Tab>        # Completes to /help
❯ /hi<Tab>       # Completes to /history
```

## Current architecture of Kota
![architecture](assert/architecture.png)

## Available Tools

Kota comes with a comprehensive set of file system and development tools:

| Category | Tool | Description |
|----------|------|-------------|
| **File Operations** | `read_file` | Read the contents of a file from the filesystem |
| | `write_file` | Write content to a file, creating it if it doesn't exist or overwriting completely |
| | `edit_file` | Apply unified diff patches to files for targeted changes |
| | `delete_file` | Delete a file from the filesystem |
| **Directory Operations** | `make_dir` | Create directories and all necessary parent directories |
| | `scan_codebase` | Scan and display the structure of a codebase directory tree |
| **Search Operations** | `grep_find` | Search for text patterns in files using regular expressions with recursive directory traversal |
| **System Operations** | `exec_cmd` | Execute bash/cmd commands and return output (use with caution) |
| **Plan Mode** | `update_plan` | Manage structured execution plans with tasks, dependencies, and status tracking (similar to Claude Code) |

Each tool provides detailed feedback during execution and handles common error cases like permission issues and missing files.

## Skills System

Kota now includes a powerful Skills system similar to Claude's skills, allowing you to specialize the AI assistant for specific tasks.

### Built-in Skills

| Skill | Description | Available Tools |
|-------|-------------|-----------------|
| **code_review** | Code quality analysis and review | read_file, scan_codebase, grep_search |
| **refactor** | Code refactoring and optimization | read_file, edit_file, write_file |
| **debug** | Problem diagnosis and debugging | read_file, execute_bash, grep_search |
| **documentation** | Documentation writing and improvement | read_file, write_file, scan_codebase |

### Using Skills

```bash
# List all available skills
❯ /skills

# Activate a skill
❯ /skill code_review

# Use the skill
❯ Please review src/main.rs for code quality issues

# Deactivate skill
❯ /skill-off
```

## Roadmap & TODO

### Upcoming Features

1. **MCP (Model Context Protocol) Support**
   - Integration with MCP servers for extended functionality
   - Support for custom MCP tools and resources
   - Enhanced context management through MCP

2. **Claude Skills Integration**
   - Implement Claude-like skills system for specialized capabilities
   - Modular skill architecture for extensibility
   - Built-in skills for common development tasks

### Future Enhancements
- Plugin system for custom tools
- Enhanced session management
- Multi-project workspace support
- Integration with popular IDEs and editors

## API Documentation

Full API documentation is available at [docs.rs/kota](https://docs.rs/kota).

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

