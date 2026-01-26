use anyhow::{Ok, Result};
use colored::Colorize;
use kota::{ContextManager, KotaConfig, SkillManager};
use names::Generator;

mod kota_cli;

use kota_cli::KotaCli;

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

    let context = ContextManager::new("./.chat_sessions", session_id)?.with_max_messages(100);
    let skill_manager = SkillManager::new();
    let mut cli = KotaCli::new(
        config.api_key,
        config.api_base,
        config.model,
        context,
        skill_manager,
    )?;
    cli.run().await?;

    Ok(())
}
