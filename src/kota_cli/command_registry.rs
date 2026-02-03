use anyhow::{anyhow, Result};
use mlua::prelude::*;
use std::collections::HashMap;

use crate::kota_code::runtime::config::{CommandDef, KotaConfig};

/// Command registry that manages custom commands from Lua config
pub struct CommandRegistry {
    lua: Lua,
    commands: HashMap<String, CommandDef>,
}

impl CommandRegistry {
    /// Create a new command registry from config
    pub fn new(config: &KotaConfig) -> Result<Self> {
        let lua = Lua::new();

        // Setup basic Lua environment
        lua.load(
            r#"
            -- Helper function to convert args table to string
            function _format_args(args)
                if type(args) ~= "table" then
                    return tostring(args or "")
                end
                
                local result = {}
                for k, v in pairs(args) do
                    result[k] = tostring(v)
                end
                return result
            end
        "#,
        )
        .exec()?;

        Ok(Self {
            lua,
            commands: config.commands.clone(),
        })
    }

    /// Execute a command with given arguments
    ///
    /// # Arguments
    ///
    /// * `name` - Command name
    /// * `args` - Arguments as key-value pairs
    ///
    /// # Returns
    ///
    /// Returns the formatted prompt string
    pub fn execute(&self, name: &str, args: HashMap<String, String>) -> Result<String> {
        let cmd_def = self
            .commands
            .get(name)
            .ok_or_else(|| anyhow!("Command '{}' not found", name))?;

        match cmd_def {
            CommandDef::String(template) => {
                // Simple string template - just return as is
                Ok(template.clone())
            }
            CommandDef::Function(bytecode) => {
                // Load function from bytecode
                let func: LuaFunction = self.lua.load(bytecode).into_function()?;

                // Convert args to Lua table
                let lua_args = self.lua.create_table()?;
                for (key, value) in args {
                    lua_args.set(key, value)?;
                }

                // Call function with args
                let result: LuaValue = func.call(lua_args)?;

                // Convert result to string
                match result {
                    LuaValue::String(s) => Ok(s.to_str()?.to_string()),
                    LuaValue::Nil => Ok(String::new()),
                    _ => Ok(format!("{:?}", result)),
                }
            }
        }
    }

    /// List all available commands
    pub fn list_commands(&self) -> Vec<String> {
        self.commands.keys().cloned().collect()
    }

    /// Check if a command exists
    pub fn has_command(&self, name: &str) -> bool {
        self.commands.contains_key(name)
    }

    /// Get command type (string or function)
    pub fn command_type(&self, name: &str) -> Option<&str> {
        self.commands.get(name).map(|def| match def {
            CommandDef::String(_) => "string",
            CommandDef::Function(_) => "function",
        })
    }
}

/// Parse command line input into command name and arguments
///
/// Supports formats:
/// - `command` - no args
/// - `command arg1 arg2` - positional args (stored as "1", "2", etc.)
/// - `command key=value key2=value2` - named args
///
/// # Arguments
///
/// * `input` - Command input string
///
/// # Returns
///
/// Returns (command_name, args_map)
pub fn parse_command_input(input: &str) -> Result<(String, HashMap<String, String>)> {
    let parts: Vec<&str> = input.split_whitespace().collect();

    if parts.is_empty() {
        return Err(anyhow!("Empty command"));
    }

    let command_name = parts[0].to_string();
    let mut args = HashMap::new();
    let mut positional_index = 1;

    for part in parts.iter().skip(1) {
        if let Some((key, value)) = part.split_once('=') {
            // Named argument
            args.insert(key.to_string(), value.to_string());
        } else {
            // Positional argument
            args.insert(positional_index.to_string(), part.to_string());
            positional_index += 1;
        }
    }

    Ok((command_name, args))
}
