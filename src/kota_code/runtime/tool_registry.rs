use anyhow::Result;
use rig::{completion::ToolDefinition, tool::Tool};
use std::collections::HashMap;
use std::sync::Arc;

/// Trait for Kota dynamic tool execution
///
/// This trait allows tools to be stored and called dynamically in the registry.
/// It provides a unified interface for all tools regardless of their specific types.
pub trait KotaTool: Send + Sync {
    /// Get the tool definition for LLM
    fn definition(&self) -> ToolDefinition;

    /// Execute the tool with JSON arguments
    fn call_json(&self, args: serde_json::Value) -> Result<serde_json::Value>;

    /// Get the tool name
    fn name(&self) -> &str;
}

/// Wrapper that implements KotaTool for any rig Tool
pub struct ToolWrapper<T: Tool> {
    tool: T,
    name: String,
}

impl<T: Tool> ToolWrapper<T> {
    pub fn new(tool: T) -> Self {
        Self {
            tool,
            name: T::NAME.to_string(),
        }
    }
}

impl<T: Tool> From<T> for ToolWrapper<T> {
    fn from(tool: T) -> Self {
        Self::new(tool)
    }
}

impl<T> KotaTool for ToolWrapper<T>
where
    T: Tool + Send + Sync,
    T::Args: serde::de::DeserializeOwned,
    T::Output: serde::Serialize,
    T::Error: std::fmt::Display,
{
    fn definition(&self) -> ToolDefinition {
        // Create a new runtime if not in async context
        let definition = if tokio::runtime::Handle::try_current().is_ok() {
            // We're in an async context, spawn blocking
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(self.tool.definition(String::new()))
            })
        } else {
            // Not in async context, create new runtime
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(self.tool.definition(String::new()))
        };
        definition
    }

    fn call_json(&self, args: serde_json::Value) -> Result<serde_json::Value> {
        let typed_args: T::Args = serde_json::from_value(args)?;

        let result = if tokio::runtime::Handle::try_current().is_ok() {
            // We're in an async context, spawn blocking
            tokio::task::block_in_place(|| {
                tokio::runtime::Handle::current().block_on(self.tool.call(typed_args))
            })
        } else {
            // Not in async context, create new runtime
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(self.tool.call(typed_args))
        };

        let output = result.map_err(|e| anyhow::anyhow!("Tool execution failed: {}", e))?;
        Ok(serde_json::to_value(output)?)
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// Tool registry for managing custom tools
///
/// # Example
///
/// ```rust,no_run
/// use kota::kota_code::ToolRegistry;
/// use rig::tool::Tool;
///
/// let mut registry = ToolRegistry::new();
///
/// // Register a custom tool
/// registry.register_rig_tool(MyCustomTool::default());
///
/// // List all tools
/// for name in registry.list_tools() {
///     println!("Tool: {}", name);
/// }
/// ```
pub struct ToolRegistry {
    tools: HashMap<String, Arc<dyn KotaTool>>,
}

impl ToolRegistry {
    /// Create a new empty tool registry
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
        }
    }

    /// Register a tool that directly implements KotaTool
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use std::sync::Arc;
    /// use kota::kota_code::ToolRegistry;
    /// use kota::kota_code::runtime::tool_registry::ToolWrapper;
    ///
    /// let mut registry = ToolRegistry::new();
    /// let tool = Arc::new(ToolWrapper::new(MyCustomTool::default()));
    /// registry.register_tool(tool);
    /// ```
    pub fn register_tool(&mut self, tool: Arc<dyn KotaTool>) -> &mut Self {
        let name = tool.name().to_string();
        self.tools.insert(name, tool);
        self
    }

    /// Register a rig Tool (convenience method)
    ///
    /// This method automatically wraps any rig Tool in a ToolWrapper and Arc,
    /// allowing you to register tools directly without manual wrapping.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// registry.register_rig_tool(MyCustomTool::default());
    /// ```
    pub fn register_rig_tool<T>(&mut self, tool: T) -> &mut Self
    where
        T: Tool + Send + Sync + 'static,
        T::Args: serde::de::DeserializeOwned,
        T::Output: serde::Serialize,
        T::Error: std::fmt::Display,
    {
        let wrapped: ToolWrapper<T> = tool.into();
        self.register_tool(Arc::new(wrapped))
    }

    /// Get a tool by name
    pub fn get_tool(&self, name: &str) -> Option<Arc<dyn KotaTool>> {
        self.tools.get(name).cloned()
    }

    /// List all registered tool names
    pub fn list_tools(&self) -> Vec<String> {
        self.tools.keys().cloned().collect()
    }

    /// Get all tool definitions for LLM
    pub fn get_definitions(&self) -> Vec<ToolDefinition> {
        self.tools.values().map(|t| t.definition()).collect()
    }

    /// Execute a tool by name with JSON arguments
    pub fn execute(&self, name: &str, args: serde_json::Value) -> Result<serde_json::Value> {
        let tool = self
            .get_tool(name)
            .ok_or_else(|| anyhow::anyhow!("Tool not found: {}", name))?;
        tool.call_json(args)
    }

    /// Check if a tool is registered
    pub fn has_tool(&self, name: &str) -> bool {
        self.tools.contains_key(name)
    }

    /// Remove a tool from the registry
    pub fn unregister_tool(&mut self, name: &str) -> Option<Arc<dyn KotaTool>> {
        self.tools.remove(name)
    }

    /// Get the number of registered tools
    pub fn len(&self) -> usize {
        self.tools.len()
    }

    /// Check if the registry is empty
    pub fn is_empty(&self) -> bool {
        self.tools.is_empty()
    }

    /// Clear all tools from the registry
    pub fn clear(&mut self) {
        self.tools.clear();
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}
