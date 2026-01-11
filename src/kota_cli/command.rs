use crate::agent::AgentType;
use anyhow::Result;
use colored::*;
use rig::agent::stream_to_stdout;
use rig::streaming::StreamingPrompt;

use super::KotaCli;

impl KotaCli {
    pub async fn handle_command(&self, input: &str) -> Result<bool> {
        match input {
            "/quit" | "/exit" => {
                return Ok(false);
            }
            "/config" => {
                self.show_config();
            }
            "/help" => {
                self.show_help();
            }
            _ if input.starts_with('/') => {
                println!("{} Unknown command: {}", "❌".red(), input);
                println!("{} Type /help for available commands", "💡".bright_blue());
            }
            _ => {
                println!("{}", "🤖 Thinking...".yellow());

                println!("{}", "🤖 kota:".green());
                let response_result = match &self.agent {
                    AgentType::OpenAI(agent) => {
                        let mut stream = agent.stream_prompt(input).multi_turn(20).await;
                        stream_to_stdout(&mut stream).await
                    }
                    AgentType::Anthropic(agent) => {
                        let mut stream = agent.stream_prompt(input).multi_turn(20).await;
                        stream_to_stdout(&mut stream).await
                    }
                    AgentType::Cohere(agent) => {
                        let mut stream = agent.stream_prompt(input).multi_turn(20).await;
                        stream_to_stdout(&mut stream).await
                    }
                    AgentType::DeepSeek(agent) => {
                        let mut stream = agent.stream_prompt(input).multi_turn(20).await;
                        stream_to_stdout(&mut stream).await
                    }
                    AgentType::Ollama(agent) => {
                        let mut stream = agent.stream_prompt(input).multi_turn(20).await;
                        stream_to_stdout(&mut stream).await
                    }
                };
                println!();

                match response_result {
                    Ok(resp) => {
                        println!(
                            "{} Total tokens used: {}",
                            "📊".bright_blue(),
                            resp.usage().total_tokens
                        )
                    }
                    Err(e) => {
                        println!("{} Failed to get AI response: {}", "❌".red(), e);
                        println!(
                            "{} Please check your API key and network connection",
                            "💡".bright_blue()
                        );
                    }
                }
            }
        }
        println!(); // 添加空行分隔
        Ok(true)
    }

    fn show_config(&self) {
        println!("{}", "⚙️  Current Configuration:".bright_cyan());
        println!("  {} {}", "API Base:".bright_white(), self.api_base);
        println!("  {} {}", "Model:".bright_white(), self.model_name);
        println!(
            "  {} {}",
            "API Key:".bright_white(),
            "*".repeat(self.api_key.len().min(8))
        );
        println!();
    }

    fn show_help(&self) {
        println!("{}", "📚 Available Commands:".bright_cyan());
        println!();
        println!("  {} - Exit the application", "/quit".bright_green());
        println!(
            "  {} - Show current model configuration",
            "/config".bright_green()
        );
        println!("  {} - Show this help message", "/help".bright_green());
        println!("  {} - Login to the service", "/login".bright_green());
        println!();
        println!(
            "{}",
            "💡 You can also type any message to chat with the AI!".bright_white()
        );
        println!();
    }
}
