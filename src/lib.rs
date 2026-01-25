//! # Kota - A lightweight ai code agent in Rust.
//!
//! ## Modules
//!
//! - `kota_code`: Core AI code agent functionality with LLM integration, file operations,
//!   code analysis, and task management.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use kota::kota_code::{AgentBuilder, ContextManager};
//! use anyhow::Result;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let agent = AgentBuilder::new(
//!         "your-api-key".to_string(),
//!         "gpt-4".to_string()
//!     )?.build()?;
//!
//!     let mut context = ContextManager::new(".chat_sessions", "my-session".to_string())?;
//!     
//!     // Use the agent...
//!     Ok(())
//! }
//! ```

pub mod kota_code;

// Re-export commonly used types for convenience
pub use kota_code::{
    create_agent, AgentBuilder, AgentType, ContextManager, Plan, PlanManager, Provider,
    SerializableMessage, SessionIdHook, SessionMetadata, Skill, SkillManager, Task, TaskStatus,
};

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::kota_code::prelude::*;
}
