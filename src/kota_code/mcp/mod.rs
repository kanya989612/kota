//! MCP (Model Context Protocol) Manager
//!
//! This module provides functionality to manage MCP clients and interact with MCP servers.
//! It supports connecting to MCP servers via child processes and calling tools.

pub mod client;

use anyhow::{Context, Result};
use rmcp::
    model::{InitializeResult, Tool}
;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::kota_code::mcp::client::McpClient;

/// MCP Manager for managing multiple MCP client connections
pub struct McpManager {
    /// Map of server name to MCP client
    clients: Arc<RwLock<HashMap<String, McpClient>>>,
}

impl McpManager {
    /// Create a new MCP manager
    pub fn new() -> Self {
        Self {
            clients: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add a new MCP server connection
    ///
    /// # Arguments
    ///
    /// * `name` - Unique name for this server connection
    /// * `command` - The command to execute (e.g., "uvx", "npx")
    /// * `args` - Arguments for the command
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// manager.add_server("git", "uvx", vec!["mcp-server-git".to_string()]).await?;
    /// ```
    pub async fn add_server(&self, name: &str, command: &str, args: Vec<String>) -> Result<()> {
        let client = McpClient::new(command, args)
            .await
            .context(format!("Failed to add MCP server: {}", name))?;

        let mut clients = self.clients.write().await;
        clients.insert(name.to_string(), client);

        Ok(())
    }

    /// Remove an MCP server connection
    pub async fn remove_server(&self, name: &str) -> Result<()> {
        let mut clients = self.clients.write().await;

        if let Some(client) = clients.remove(name) {
            client.close().await?;
        }

        Ok(())
    }

    /// Get information about a specific server
    pub async fn get_server_info(&self, name: &str) -> Result<InitializeResult> {
        let clients = self.clients.read().await;

        let client = clients
            .get(name)
            .context(format!("Server not found: {}", name))?;

        Ok(client.server_info().clone())
    }

    /// List all available tools from a specific server
    pub async fn list_tools(&self, server_name: &str) -> Result<Vec<Tool>> {
        let clients = self.clients.read().await;

        let client = clients
            .get(server_name)
            .context(format!("Server not found: {}", server_name))?;

        Ok(client.tools().to_vec())
    }

    /// List all servers and their tools
    pub async fn list_all_tools(&self) -> Result<HashMap<String, Vec<Tool>>> {
        let clients = self.clients.read().await;

        let mut all_tools = HashMap::new();
        for (name, client) in clients.iter() {
            all_tools.insert(name.clone(), client.tools().to_vec());
        }

        Ok(all_tools)
    }

    /// Call a tool on a specific server
    ///
    /// # Arguments
    ///
    /// * `server_name` - Name of the server
    /// * `tool_name` - Name of the tool to call
    /// * `arguments` - JSON arguments for the tool
    pub async fn call_tool(
        &self,
        server_name: &str,
        tool_name: &str,
        arguments: Option<Value>,
    ) -> Result<Value> {
        let clients = self.clients.read().await;

        let client = clients
            .get(server_name)
            .context(format!("Server not found: {}", server_name))?;

        client.call_tool(tool_name, arguments).await
    }

    /// Refresh tools list for a specific server
    pub async fn refresh_server_tools(&self, server_name: &str) -> Result<()> {
        let mut clients = self.clients.write().await;

        let client = clients
            .get_mut(server_name)
            .context(format!("Server not found: {}", server_name))?;

        client.refresh_tools().await
    }

    /// Get list of all connected servers
    pub async fn list_servers(&self) -> Vec<String> {
        let clients = self.clients.read().await;
        clients.keys().cloned().collect()
    }

    /// Check if a server is connected
    pub async fn has_server(&self, name: &str) -> bool {
        let clients = self.clients.read().await;
        clients.contains_key(name)
    }

    /// Close all connections and cleanup
    pub async fn close_all(&self) -> Result<()> {
        let mut clients = self.clients.write().await;

        for (_, client) in clients.drain() {
            let _ = client.close().await; // Ignore errors during cleanup
        }

        Ok(())
    }
}

impl Default for McpManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mcp_manager_creation() {
        let manager = McpManager::new();
        assert_eq!(manager.list_servers().await.len(), 0);
    }

    #[tokio::test]
    async fn test_server_check() {
        let manager = McpManager::new();
        assert!(!manager.has_server("test").await);
    }
}
