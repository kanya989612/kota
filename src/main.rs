use anyhow::Result;
use dotenv::dotenv;
use rig::{
    agent::Agent,
    client::CompletionClient,
    providers::{
        anthropic, cohere,
        deepseek::{self, DEEPSEEK_CHAT},
        openai,
    },
};
use std::env;

mod kota_cli;
use kota_cli::KotaCli;

#[derive(Debug, Clone)]
pub enum Provider {
    OpenAI,
    Anthropic,
    Cohere,
    DeepSeek,
}

// Agent enum to handle different provider types
pub enum AgentType {
    OpenAI(Agent<openai::responses_api::ResponsesCompletionModel>),
    Anthropic(Agent<anthropic::completion::CompletionModel>),
    Cohere(Agent<cohere::CompletionModel>),
    DeepSeek(Agent<deepseek::CompletionModel>),
}

// 根据模型名称确定 Provider
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

        _ => {
            // 默认根据常见模型名称判断
            match model_name {
                "gpt-4o" | "gpt-4" | "gpt-3.5-turbo" | "o1-preview" | "o1-mini" => {
                    Ok(Provider::OpenAI)
                }
                _ => Err(anyhow::anyhow!(
                    "Unknown model: {}. Please specify a supported model.",
                    model_name
                )),
            }
        }
    }
}

// 创建对应的 agent
fn create_agent(provider: Provider, api_key: &str, model_name: &str) -> Result<AgentType> {
    match provider {
        Provider::OpenAI => {
            let client = openai::Client::new(api_key)?;
            let agent = client
                .agent(model_name)
                .preamble(
                    "You are a helpful AI assistant. Please provide clear and concise responses.",
                )
                .build();
            Ok(AgentType::OpenAI(agent))
        }
        Provider::Anthropic => {
            let client = anthropic::Client::new(api_key)?;
            let agent = client
                .agent(model_name)
                .preamble(
                    "You are a helpful AI assistant. Please provide clear and concise responses.",
                )
                .build();
            Ok(AgentType::Anthropic(agent))
        }
        Provider::Cohere => {
            let client = cohere::Client::new(api_key)?;
            let agent = client
                .agent(model_name)
                .preamble(
                    "You are a helpful AI assistant. Please provide clear and concise responses.",
                )
                .build();
            Ok(AgentType::Cohere(agent))
        }
        Provider::DeepSeek => {
            let client = deepseek::Client::new(api_key)?;
            let agent = client
                .agent(DEEPSEEK_CHAT)
                .preamble(
                    "You are a helpful AI assistant. Please provide clear and concise responses.",
                )
                .build();
            Ok(AgentType::DeepSeek(agent))
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    let api_key = env::var("API_KEY").expect("API_KEY must be set in .env file");
    let api_base = env::var("API_BASE").unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
    let model_name = env::var("MODEL_NAME").unwrap_or_else(|_| "gpt-4o".to_string());

    // 根据模型名称确定 Provider
    let provider = get_provider_from_model(&model_name)?;

    // 创建对应的 agent
    let agent = create_agent(provider.clone(), &api_key, &model_name)?;

    let cli = KotaCli::new(api_key, api_base, model_name, agent);
    cli.run().await?;

    Ok(())
}
