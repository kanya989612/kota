use mlua::prelude::*;
use rig::{completion::ToolDefinition, tool::Tool};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DynToolError {
    #[error("Input is invalid: {0}")]
    InvalidInput(String),
}

/// Lua tool that wraps a Lua function and makes it callable as a rig Tool
#[derive(Clone)]
pub struct LuaDynTool {
    name: String,
    description: String,
    parameters: JsonValue,
    lua_code: Arc<Vec<u8>>, // Lua function bytecode
}

impl LuaDynTool {
    pub fn new(
        name: String,
        description: String,
        parameters: JsonValue,
        lua_code: Vec<u8>,
    ) -> Self {
        Self {
            name,
            description,
            parameters,
            lua_code: Arc::new(lua_code),
        }
    }

    pub fn tool_name(&self) -> &str {
        &self.name
    }
}

#[derive(Deserialize)]
pub struct LuaToolArgs {
    #[serde(flatten)]
    pub args: JsonValue,
}

#[derive(Serialize)]
pub struct LuaToolOutput {
    pub result: JsonValue,
}

impl Tool for LuaDynTool {
    const NAME: &'static str = "lua_dyn_tool";

    type Error = DynToolError;
    type Args = LuaToolArgs;
    type Output = LuaToolOutput;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: self.name.clone(),
            description: self.description.clone(),
            parameters: self.parameters.clone(),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        let lua = Lua::new();
        
        // Load the Lua function from bytecode
        let func: LuaFunction = lua
            .load(&*self.lua_code)
            .call(())
            .map_err(|e| {
                DynToolError::InvalidInput(format!("Failed to load Lua function: {}", e))
            })?;

        // Convert JSON args to Lua value
        let lua_args = json_to_lua(&lua, &args.args).map_err(|e| {
            DynToolError::InvalidInput(format!("Failed to convert args to Lua: {}", e))
        })?;

        // Call the Lua function
        let result: LuaValue = func.call(lua_args).map_err(|e| {
            DynToolError::InvalidInput(format!("Lua function call failed: {}", e))
        })?;

        // Convert Lua result back to JSON
        let json_result = lua_to_json(&result).map_err(|e| {
            DynToolError::InvalidInput(format!("Failed to convert Lua result to JSON: {}", e))
        })?;

        Ok(LuaToolOutput {
            result: json_result,
        })
    }
}

// Helper: Convert JSON to Lua value
fn json_to_lua<'lua>(lua: &'lua Lua, json: &JsonValue) -> LuaResult<LuaValue<'lua>> {
    match json {
        JsonValue::Null => Ok(LuaValue::Nil),
        JsonValue::Bool(b) => Ok(LuaValue::Boolean(*b)),
        JsonValue::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(LuaValue::Integer(i))
            } else if let Some(f) = n.as_f64() {
                Ok(LuaValue::Number(f))
            } else {
                Ok(LuaValue::Nil)
            }
        }
        JsonValue::String(s) => Ok(LuaValue::String(lua.create_string(s)?)),
        JsonValue::Array(arr) => {
            let table = lua.create_table()?;
            for (i, val) in arr.iter().enumerate() {
                table.set(i + 1, json_to_lua(lua, val)?)?;
            }
            Ok(LuaValue::Table(table))
        }
        JsonValue::Object(obj) => {
            let table = lua.create_table()?;
            for (key, val) in obj.iter() {
                table.set(key.as_str(), json_to_lua(lua, val)?)?;
            }
            Ok(LuaValue::Table(table))
        }
    }
}

// Helper: Convert Lua value to JSON
fn lua_to_json(value: &LuaValue) -> LuaResult<JsonValue> {
    match value {
        LuaValue::Nil => Ok(JsonValue::Null),
        LuaValue::Boolean(b) => Ok(JsonValue::Bool(*b)),
        LuaValue::Integer(i) => Ok(JsonValue::Number((*i).into())),
        LuaValue::Number(n) => {
            if let Some(num) = serde_json::Number::from_f64(*n) {
                Ok(JsonValue::Number(num))
            } else {
                Ok(JsonValue::Null)
            }
        }
        LuaValue::String(s) => Ok(JsonValue::String(s.to_str()?.to_string())),
        LuaValue::Table(table) => {
            // Check if it's an array or object
            let mut is_array = true;
            let mut max_index = 0;
            
            for pair in table.clone().pairs::<LuaValue, LuaValue>() {
                let (key, _) = pair?;
                if let LuaValue::Integer(i) = key {
                    if i > 0 {
                        max_index = max_index.max(i);
                    } else {
                        is_array = false;
                        break;
                    }
                } else {
                    is_array = false;
                    break;
                }
            }

            if is_array && max_index > 0 {
                let mut arr = Vec::new();
                for i in 1..=max_index {
                    let val: LuaValue = table.get(i)?;
                    arr.push(lua_to_json(&val)?);
                }
                Ok(JsonValue::Array(arr))
            } else {
                let mut obj = serde_json::Map::new();
                for pair in table.clone().pairs::<LuaValue, LuaValue>() {
                    let (key, val) = pair?;
                    let key_str = match key {
                        LuaValue::String(s) => s.to_str()?.to_string(),
                        LuaValue::Integer(i) => i.to_string(),
                        _ => continue,
                    };
                    obj.insert(key_str, lua_to_json(&val)?);
                }
                Ok(JsonValue::Object(obj))
            }
        }
        _ => Ok(JsonValue::Null),
    }
}
