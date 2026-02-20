use std::env;

use anyhow::{Ok, Result};

#[derive(Debug, Clone)]
pub struct Config {
    pub log_level: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            log_level: "info".to_string(),
        }
    }
}

impl Config {
    pub fn from_env() -> Result<Self> {
        if let Err(e) = dotenvy::dotenv() {
            anyhow::bail!("Failed to load env variables: {}", e);
        }

        let config = Self {
            log_level: env::var("LOG_LEVEL").unwrap_or_default(),
        };

        Ok(config)
    }
}
