# Lua Configuration

Kota uses Lua-based configuration, inspired by Neovim's configuration system. This provides a flexible and programmable way to configure your AI code agent.

## Configuration File

Create a `.kota/config.lua` file in your project root:

```lua
kota.setup({
  model = "deepseek-chat",
  api_key = os.getenv("API_KEY"),
  api_base = "https://api.deepseek.com/v1",
  temperature = 0.7,
  
  tools = {
    enabled = { "grep_find", "write_file", "read_file" },
    disabled = { "delete_file" },
  },
  
  commands = {
    ["fix"] = "analyze and fix the current file",
    ["test"] = "run tests for current file",
  },
  
  hooks = {
    before_execute = function(tool, args)
      -- Custom logic before tool execution
    end,
    after_execute = function(result)
      -- Custom logic after tool execution
    end,
  },
})
```

## Configuration Options

### Basic Settings

- **model** (string): The LLM model to use
  - Examples: `"gpt-4o"`, `"claude-3-5-sonnet"`, `"deepseek-chat"`
  
- **api_key** (string): Your API key for the LLM provider
  - Can use `os.getenv("API_KEY")` to read from environment variables
  
- **temperature** (number, optional): Temperature for LLM responses
  - Default: `0.7`
  - Range: `0.0` to `2.0`

### Tools Configuration

Control which tools are available to the agent:

```lua
tools = {
  enabled = { "grep_find", "write_file", "read_file" },
  disabled = { "delete_file" },
}
```

Available tools:
- `read_file` - Read file contents
- `write_file` - Write to files
- `edit_file` - Apply patches to files
- `delete_file` - Delete files
- `make_dir` - Create directories
- `scan_codebase` - Scan directory structure
- `grep_find` - Search for patterns
- `exec_cmd` - Execute shell commands
- `update_plan` - Manage task plans

### Commands (Coming Soon)

Define custom command shortcuts:

```lua
commands = {
  ["fix"] = "analyze and fix the current file",
  ["test"] = "run tests for current file",
  ["review"] = "perform code review on current file",
}
```

### Hooks (Coming Soon)

Add custom logic before and after tool execution:

```lua
hooks = {
  before_execute = function(tool, args)
    print("Executing tool: " .. tool)
  end,
  after_execute = function(result)
    print("Tool execution completed")
  end,
}
```

## Environment Variables

The Lua configuration can read environment variables using `os.getenv()`:

```lua
kota.setup({
  api_key = os.getenv("API_KEY"),
})
```

Set your environment variables:

```bash
# Linux/Mac
export API_KEY="your-api-key-here"

# Windows PowerShell
$env:API_KEY="your-api-key-here"
```

## Benefits of Lua Configuration

1. **Programmable**: Use Lua logic for dynamic configuration
2. **Type-safe**: Lua provides better structure than plain text
3. **Extensible**: Easy to add custom tools and hooks
4. **Familiar**: Similar to Neovim's configuration system
5. **Flexible**: Supports both static values and environment variables

## Future Enhancements

- Custom tool definitions in Lua
- Plugin system for extending functionality
- Configuration hot-reloading
- Project-specific vs global configuration
- Configuration validation and error reporting
