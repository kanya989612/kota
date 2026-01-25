use anyhow::Result;
use rig::{
    agent::Agent,
    client::CompletionClient,
    providers::{
        anthropic, cohere,
        deepseek::{self, DEEPSEEK_CHAT},
        ollama, openai,
    },
};

use super::plan::PlanManager;
use super::tools::{
    WrappedCreateDirectoryTool, WrappedDeleteFileTool, WrappedEditFileTool,
    WrappedExecuteBashCommandTool, WrappedGrepSearchTool, WrappedReadFileTool,
    WrappedScanCodebaseTool, WrappedUpdatePlanTool, WrappedWriteFileTool,
};

macro_rules! build_agent {
    ($client_expr:expr, $model_name:expr, $preamble:expr, $tools:expr, $variant:ident) => {{
        let client = $client_expr?;
        let agent = client
            .agent($model_name)
            .preamble($preamble)
            .max_tokens(4096)
            .tool($tools.read_file)
            .tool($tools.write_file)
            .tool($tools.edit_file)
            .tool($tools.delete_file)
            .tool($tools.execute_bash)
            .tool($tools.scan_codebase)
            .tool($tools.make_dir)
            .tool($tools.grep_find)
            .tool($tools.update_plan)
            .build();
        Ok(AgentType::$variant(agent))
    }};
}

/// Supported LLM providers
#[derive(Debug, Clone)]
pub enum Provider {
    /// OpenAI (GPT-4, GPT-3.5, etc.)
    OpenAI,
    /// Anthropic Claude models
    Anthropic,
    /// Cohere models
    Cohere,
    /// DeepSeek models
    DeepSeek,
    /// Local Ollama models
    Ollama,
}

/// Agent enum to handle different provider types
/// 
/// This enum wraps agents from different LLM providers, allowing you to work
/// with them through a unified interface.
pub enum AgentType {
    /// OpenAI agent
    OpenAI(Agent<openai::responses_api::ResponsesCompletionModel>),
    /// Anthropic Claude agent
    Anthropic(Agent<anthropic::completion::CompletionModel>),
    /// Cohere agent
    Cohere(Agent<cohere::CompletionModel>),
    /// DeepSeek agent
    DeepSeek(Agent<deepseek::CompletionModel>),
    /// Ollama local agent
    Ollama(Agent<ollama::CompletionModel>),
}

/// Builder for creating AI agents with custom configuration
/// 
/// # Example
/// 
/// ```rust,no_run
/// use kota::{AgentBuilder, PlanManager};
/// use anyhow::Result;
/// 
/// fn main() -> Result<()> {
///     let plan_manager = PlanManager::new();
///     let agent = AgentBuilder::new("api-key".to_string(), "gpt-4".to_string())?
///         .with_plan_manager(plan_manager)
///         .build()?;
///     Ok(())
/// }
/// ```
pub struct AgentBuilder {
    provider: Provider,
    api_key: String,
    model_name: String,
    plan_manager: PlanManager,
}

impl AgentBuilder {
    /// Create a new agent builder
    /// 
    /// # Arguments
    /// 
    /// * `api_key` - API key for the LLM provider
    /// * `model_name` - Model name (e.g., "gpt-4", "claude-3-5-sonnet", "deepseek-chat")
    /// 
    /// # Returns
    /// 
    /// Returns a builder that can be configured and built into an agent
    pub fn new(api_key: String, model_name: String) -> Result<Self> {
        let provider = Self::get_provider_from_model(&model_name)?;
        Ok(Self {
            provider,
            api_key,
            model_name,
            plan_manager: PlanManager::new(),
        })
    }

    /// Set a custom plan manager for task management
    /// 
    /// # Arguments
    /// 
    /// * `manager` - A PlanManager instance for managing tasks and plans
    pub fn with_plan_manager(mut self, manager: PlanManager) -> Self {
        self.plan_manager = manager;
        self
    }

    /// Build the agent with the configured settings
    /// 
    /// # Returns
    /// 
    /// Returns an AgentType that can be used to interact with the LLM
    pub fn build(self) -> Result<AgentType> {
        let tools = self.create_tools();
        let preamble = self.get_preamble();

        match self.provider {
            Provider::OpenAI => {
                build_agent!(
                    openai::Client::new(&self.api_key),
                    &self.model_name,
                    preamble,
                    tools,
                    OpenAI
                )
            }
            Provider::Anthropic => {
                build_agent!(
                    anthropic::Client::new(&self.api_key),
                    &self.model_name,
                    preamble,
                    tools,
                    Anthropic
                )
            }
            Provider::Cohere => {
                build_agent!(
                    cohere::Client::new(&self.api_key),
                    &self.model_name,
                    preamble,
                    tools,
                    Cohere
                )
            }
            Provider::DeepSeek => {
                build_agent!(
                    deepseek::Client::new(&self.api_key),
                    DEEPSEEK_CHAT,
                    preamble,
                    tools,
                    DeepSeek
                )
            }
            Provider::Ollama => {
                build_agent!(
                    ollama::Client::new(rig::client::Nothing),
                    &self.model_name,
                    preamble,
                    tools,
                    Ollama
                )
            }
        }
    }

    fn get_provider_from_model(model_name: &str) -> Result<Provider> {
        match model_name.to_lowercase().as_str() {
            // OpenAI models
            name if name.starts_with("gpt-") || name.starts_with("o1-") => Ok(Provider::OpenAI),

            // Anthropic models
            name if name.starts_with("claude-") => Ok(Provider::Anthropic),

            // Cohere models
            name if name.starts_with("command-") => Ok(Provider::Cohere),

            // DeepSeek models
            name if name.starts_with("deepseek-") => Ok(Provider::DeepSeek),
            "ollama" | "local" => Ok(Provider::Ollama),

            _ => Err(anyhow::anyhow!(
                "Unknown model: {}. Please specify a supported model.",
                model_name
            )),
        }
    }

    fn create_tools(&self) -> AgentTools {
        AgentTools {
            read_file: WrappedReadFileTool::new(),
            write_file: WrappedWriteFileTool::new(),
            edit_file: WrappedEditFileTool::new(),
            delete_file: WrappedDeleteFileTool::new(),
            execute_bash: WrappedExecuteBashCommandTool::new(),
            scan_codebase: WrappedScanCodebaseTool::new(),
            make_dir: WrappedCreateDirectoryTool::new(),
            grep_find: WrappedGrepSearchTool::new(),
            update_plan: WrappedUpdatePlanTool::new(self.plan_manager.clone()),
        }
    }

    fn get_preamble(&self) -> &str {
        r#"
        Your name is Kato. You are a helpful AI code assistant with comprehensive file system and command execution access. 
        You can read, write, edit (with patches), and delete files, execute bash commands, scan codebase structures, search text in the codebase and create directories. 
        Use the edit_file tool for making small, targeted changes to existing files - it's more efficient than rewriting entire files.
        
        You also have access to Plan Mode via the update_plan tool. Use it to:
        - Create structured execution plans for complex tasks
        - Break down work into manageable tasks with dependencies
        - Track progress and update task status (pending, in_progress, completed, blocked)
        - Show current plan and identify next available tasks
        
        Please provide clear and concise responses and be careful when modifying files or executing commands."#
    }
}

struct AgentTools {
    read_file: WrappedReadFileTool,
    write_file: WrappedWriteFileTool,
    edit_file: WrappedEditFileTool,
    delete_file: WrappedDeleteFileTool,
    execute_bash: WrappedExecuteBashCommandTool,
    scan_codebase: WrappedScanCodebaseTool,
    make_dir: WrappedCreateDirectoryTool,
    grep_find: WrappedGrepSearchTool,
    update_plan: WrappedUpdatePlanTool,
}

/// Convenience function for creating an agent with default settings
/// 
/// # Arguments
/// 
/// * `api_key` - API key for the LLM provider
/// * `model_name` - Model name (e.g., "gpt-4", "claude-3-5-sonnet", "deepseek-chat")
/// 
/// # Example
/// 
/// ```rust,no_run
/// use kota::create_agent;
/// use anyhow::Result;
/// 
/// fn main() -> Result<()> {
///     let agent = create_agent("api-key".to_string(), "gpt-4".to_string())?;
///     Ok(())
/// }
/// ```
pub fn create_agent(api_key: String, model_name: String) -> Result<AgentType> {
    AgentBuilder::new(api_key, model_name)?.build()
}
