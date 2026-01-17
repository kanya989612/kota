# Kota
A lightweight AI code agent in Rust.

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
   - `MODEL_NAME`: The model to use (e.g., deepseek-chat, gpt-4o)

### Running Kota

```bash
cargo run -r
```

## CLI Features

### Interactive Commands

Kota provides an interactive CLI with the following commands:

- `/quit` or `/exit` - Exit the application
- `/config` - Show current model configuration
- `/help` - Show available commands
- `/history` - Show conversation history
- `/load [session_id]` - List all sessions or load specific session
- `/delete <session_id>` - Delete a specific session

### Tab Completion

Kota supports intelligent tab completion for commands:

- Type `/` and press **Tab** to see all available commands
- Type partial commands (e.g., `/h`) and press **Tab** to auto-complete
- Commands are highlighted in green when recognized
- Use **Ctrl+C** to exit or **Ctrl+D** for EOF

**Example usage:**
```
❯ /h<Tab>        # Completes to /help
❯ /lo<Tab>       # Completes to /load
❯ /<Tab>         # Shows all available commands
```

## Available Tools

Kota comes with a comprehensive set of file system and development tools:

### File Operations
- **`read_file`** - Read the contents of a file from the filesystem
- **`write_file`** - Write content to a file, creating it if it doesn't exist or overwriting completely
- **`edit_file`** - Apply unified diff patches to files for targeted changes
- **`delete_file`** - Delete a file from the filesystem

### Directory Operations
- **`create_directory`** - Create directories and all necessary parent directories
- **`scan_codebase`** - Scan and display the structure of a codebase directory tree

### Search Operations
- **`grep_search`** - Search for text patterns in files using regular expressions with recursive directory traversal

### System Operations
- **`execute_bash_command`** - Execute bash/cmd commands and return output (use with caution)

Each tool provides detailed feedback during execution and handles common error cases like permission issues and missing files.
