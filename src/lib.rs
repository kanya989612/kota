//! # Kota - A lightweight ai code agent in Rust.
//!
//! ## Modules
//!
//! - `kota_code`: Core AI code agent functionality with LLM integration, file operations,
//!   code analysis, and task management.
//! - `kota_cli`: Command-line interface components
//!

pub mod kota_cli;
pub mod kota_code;

// Re-export commonly used types for convenience
pub use kota_code::{
    create_agent, AgentBuilder, AgentInstance, AgentType, CommandDef, ContextManager, KotaConfig,
    McpClient, McpManager, Plan, PlanManager, Provider, SerializableMessage, SessionIdHook,
    SessionMetadata, Skill, SkillManager, Task, TaskStatus, ToolRegistry,
};

// Re-export CLI components for testing
pub use kota_cli::{parse_command_input, CommandRegistry};

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::kota_code::prelude::*;
}
