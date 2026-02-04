pub mod dyn_tool;

use anyhow::Result;
use mlua::prelude::*;
use std::path::Path;

use dyn_tool::LuaDynTool;

/// Load Lua tools from .kota/tools directory
pub struct LuaToolLoader;

impl LuaToolLoader {
    /// Load all Lua tools from .kota/tools/mod.lua
    pub fn load_tools() -> Result<Vec<LuaDynTool>> {
        let tools_path = ".kota/tools/init.lua";

        if !Path::new(tools_path).exists() {
            // No tools file, return empty vec
            return Ok(Vec::new());
        }

        let lua = Lua::new();
        let mut tools = Vec::new();

        // Setup the tools table
        lua.load(
            r#"
            _kota_tools = {}
            
            kota = kota or {}
            kota.register_tool = function(tool_def)
                table.insert(_kota_tools, tool_def)
            end
        "#,
        )
        .exec()?;

        // Load the tools file
        let tools_content = std::fs::read_to_string(tools_path)?;
        lua.load(&tools_content).exec()?;

        // Extract registered tools
        let tools_table: LuaTable = lua.globals().get("_kota_tools")?;

        for pair in tools_table.pairs::<LuaValue, LuaTable>() {
            let (_, tool_def) = pair?;

            // Extract tool definition fields
            let name: String = tool_def.get("name")?;
            let description: String = tool_def.get("description")?;
            let parameters: LuaTable = tool_def.get("parameters")?;
            let handler: LuaFunction = tool_def.get("entry")?;

            // Convert parameters table to JSON
            let params_json = Self::lua_table_to_json(&parameters)?;

            // Dump handler function to bytecode
            let bytecode = handler.dump(false);

            // Create LuaDynTool
            let tool = LuaDynTool::new(name, description, params_json, bytecode);
            tools.push(tool);
        }

        Ok(tools)
    }

    /// Convert Lua table to serde_json::Value
    fn lua_table_to_json(table: &LuaTable) -> Result<serde_json::Value> {
        let mut map = serde_json::Map::new();

        for pair in table.clone().pairs::<LuaValue, LuaValue>() {
            let (key, value) = pair?;

            let key_str = match key {
                LuaValue::String(s) => s.to_str()?.to_string(),
                LuaValue::Integer(i) => i.to_string(),
                _ => continue,
            };

            let json_value = Self::lua_value_to_json(&value)?;
            map.insert(key_str, json_value);
        }

        Ok(serde_json::Value::Object(map))
    }

    /// Convert Lua value to serde_json::Value
    fn lua_value_to_json(value: &LuaValue) -> Result<serde_json::Value> {
        match value {
            LuaValue::Nil => Ok(serde_json::Value::Null),
            LuaValue::Boolean(b) => Ok(serde_json::Value::Bool(*b)),
            LuaValue::Integer(i) => Ok(serde_json::Value::Number((*i).into())),
            LuaValue::Number(n) => {
                if let Some(num) = serde_json::Number::from_f64(*n) {
                    Ok(serde_json::Value::Number(num))
                } else {
                    Ok(serde_json::Value::Null)
                }
            }
            LuaValue::String(s) => Ok(serde_json::Value::String(s.to_str()?.to_string())),
            LuaValue::Table(table) => {
                // Check if it's an array
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
                        arr.push(Self::lua_value_to_json(&val)?);
                    }
                    Ok(serde_json::Value::Array(arr))
                } else {
                    Self::lua_table_to_json(table)
                }
            }
            _ => Ok(serde_json::Value::Null),
        }
    }
}
