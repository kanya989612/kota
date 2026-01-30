use anyhow::{Ok, Result};
use colored::Colorize;
use kota::{ContextManager, KotaConfig, SkillManager, CommandRegistry};
use names::Generator;

use kota::kota_cli::KotaCli;

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration from .kota/config.lua
    let config = KotaConfig::load()?;

    println!(
        "{} {}",
        "📝 Model:".bright_cyan(),
        config.model.bright_yellow()
    );

    let session_id = {
        let mut generator = Generator::default();
        generator
            .next()
            .unwrap_or_else(|| "unknown-session".to_string())
    };

    println!(
        "{} {}",
        "🎯 Session ID:".bright_cyan(),
        session_id.bright_yellow()
    );

    // Initialize command registry if commands are defined
    let command_registry = if !config.commands.is_empty() {
        match CommandRegistry::new(&config) {
            std::result::Result::Ok(registry) => {
                println!(
                    "{} {} custom commands loaded",
                    "🔧".bright_cyan(),
                    registry.list_commands().len().to_string().bright_yellow()
                );
                Some(registry)
            }
            Err(e) => {
                println!("{} Failed to initialize command registry: {}", "⚠️".yellow(), e);
                None
            }
        }
    } else {
        None
    };

    let context = ContextManager::new("./.chat_sessions", session_id)?.with_max_messages(100);
    let skill_manager = SkillManager::new();
    let mut cli = KotaCli::new(
        config.api_key,
        config.api_base,
        config.model,
        context,
        skill_manager,
        command_registry,
    )?;
    cli.run().await?;

    Ok(())
}
