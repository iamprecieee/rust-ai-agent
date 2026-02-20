use anyhow::Result;
use tracing::info;

use agent::{config::Config, logging::setup_logging};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = setup_logging()?;

    let _config = Config::from_env()?;
    info!("Config loaded");

    Ok(())
}
