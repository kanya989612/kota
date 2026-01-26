use anyhow::Result;
use mlua::prelude::*;
use std::path::Path;

/// Configuration loaded from Lua config file
#[derive(Debug, Clone)]
pub struct KotaConfig {
    pub model: String,
    pub api_key: String,
    pub api_base: String,
    pub temperature: Option<f64>,
    pub enabled_tools: Vec<String>,
    pub disabled_tools: Vec<String>,
}

impl Default for KotaConfig {
    fn default() -> Self {
        Self {
            model: "gpt-4o".to_string(),
            api_key: String::new(),
            api_base: "https://api.openai.com/v1".to_string(),
            temperature: Some(0.7),
            enabled_tools: vec![],
            disabled_tools: vec![],
        }
    }
}

impl KotaConfig {
    /// Load configuration from a Lua file
    ///
    /// # Arguments
    ///
    /// * `config_path` - Path to the Lua configuration file
    ///
    /// # Returns
    ///
    /// Returns a KotaConfig with parsed values
    pub fn from_lua_file<P: AsRef<Path>>(config_path: P) -> Result<Self> {
        let lua = Lua::new();
        let mut config = Self::default();

        // Load and execute the config file to capture the setup call
        let config_content = std::fs::read_to_string(config_path.as_ref())
            .map_err(|e| anyhow::anyhow!("Failed to read config file: {}", e))?;

        // Create a custom os.getenv function that reads from Rust environment
        lua.load(r#"
            _kota_config = nil
            kota = {
                setup = function(args)
                    _kota_config = args
                end
            }
            
            -- Provide os.getenv functionality
            if not os then os = {} end
            os.getenv = function(name)
                return _rust_getenv(name)
            end
        "#).exec()?;
        
        // Register Rust function to get environment variables
        let globals = lua.globals();
        globals.set("_rust_getenv", lua.create_function(|_, name: String| {
            Ok(std::env::var(&name).ok())
        })?)?;

        // Execute the config file
        lua.load(&config_content).exec()
            .map_err(|e| anyhow::anyhow!("Failed to execute Lua config: {}", e))?;
        
        // Parse the configuration
        Self::parse_from_lua(&lua, &mut config)?;

        Ok(config)
    }

    fn parse_from_lua(lua: &Lua, config: &mut KotaConfig) -> Result<()> {
        // Get the captured config
        let captured: LuaTable = lua.globals().get("_kota_config")
            .map_err(|e| anyhow::anyhow!("Config not properly initialized: {}", e))?;

        // Parse model
        if let Ok(model) = captured.get::<_, String>("model") {
            config.model = model;
        }

        // Parse api_key - Lua will have already evaluated os.getenv() expressions
        if let Ok(api_key) = captured.get::<_, String>("api_key") {
            config.api_key = api_key;
        }

        // Parse api_base - Lua will have already evaluated os.getenv() expressions
        if let Ok(api_base) = captured.get::<_, String>("api_base") {
            config.api_base = api_base;
        }

        // Parse temperature
        if let Ok(temp) = captured.get::<_, f64>("temperature") {
            config.temperature = Some(temp);
        }

        // Parse tools configuration
        if let Ok(tools) = captured.get::<_, LuaTable>("tools") {
            if let Ok(enabled) = tools.get::<&str, LuaTable>("enabled") {
                for pair in enabled.pairs::<LuaValue, String>() {
                    if let Ok((_, tool)) = pair {
                        config.enabled_tools.push(tool);
                    }
                }
            }

            if let Ok(disabled) = tools.get::<&str, LuaTable>("disabled") {
                for pair in disabled.pairs::<LuaValue, String>() {
                    if let Ok((_, tool)) = pair {
                        config.disabled_tools.push(tool);
                    }
                }
            }
        }

        Ok(())
    }

    /// Load configuration from .kota/config.lua
    ///
    /// # Returns
    ///
    /// Returns a KotaConfig loaded from .kota/config.lua
    ///
    /// # Errors
    ///
    /// Returns an error if the config file doesn't exist or has syntax errors
    pub fn load() -> Result<Self> {
        let config_path = ".kota/config.lua";
        
        if !Path::new(config_path).exists() {
            return Err(anyhow::anyhow!(
                "Configuration file not found: {}\n\
                Please create a .kota/config.lua file. ",
                config_path
            ));
        }
        
        Self::from_lua_file(config_path)
    }
}
