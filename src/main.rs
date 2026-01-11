use anyhow::Result;
use dotenv::dotenv;
use std::env;

mod kota_cli;
use kota_cli::KotaCli;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    
    let api_key = env::var("API_KEY")
        .expect("API_KEY must be set in .env file");
    let api_base = env::var("API_BASE")
        .unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
    let model_name = env::var("MODEL_NAME")
        .unwrap_or_else(|_| "gpt-4o".to_string());
    
    let cli = KotaCli::new(api_key, api_base, model_name);
    cli.run().await?;
    
    Ok(())
}
