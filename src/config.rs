use std::env;

use anyhow::{Ok, Result};

#[derive(Debug, Clone)]
pub struct Config {
    pub agent_model: String,
    pub gemini_api_key: String,
    pub provider: String,
    pub ollama_api_base_url: String,
    pub log_level: String,
    pub temperature: f64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            agent_model: "gemini-2.0-flash".to_string(),
            gemini_api_key: "".to_string(),
            provider: "gemini".to_string(),
            ollama_api_base_url: "".to_string(),
            log_level: "info".to_string(),
            temperature: 0.3,
        }
    }
}

impl Config {
    pub fn from_env() -> Result<Self> {
        if let Err(e) = dotenvy::dotenv() {
            anyhow::bail!("Failed to load env variables: {}", e);
        }

        let config = Self {
            agent_model: env::var("AGENT_MODEL").unwrap_or(Self::default().agent_model),
            gemini_api_key: env::var("GEMINI_API_KEY").unwrap_or(Self::default().gemini_api_key),
            provider: env::var("PROVIDER").unwrap_or(Self::default().provider),
            ollama_api_base_url: env::var("OLLAMA_API_BASE_URL")
                .unwrap_or(Self::default().ollama_api_base_url),
            log_level: env::var("LOG_LEVEL").unwrap_or(Self::default().log_level),
            temperature: env::var("TEMPERATURE")
                .unwrap_or(Self::default().temperature.to_string())
                .parse::<f64>()
                .unwrap_or(Self::default().temperature),
        };

        Self::validate_provider(&config.provider)?;
        Self::validate_gemini_api_key(&config.gemini_api_key, &config.provider)?;
        Self::validate_ollama_base_url(&config.ollama_api_base_url, &config.provider)?;

        Ok(config)
    }

    fn validate_provider(provider: &str) -> Result<()> {
        match provider {
            "gemini" | "ollama" => Ok(()),
            _ => anyhow::bail!("Invalid provider: {}", provider),
        }
    }

    fn validate_gemini_api_key(gemini_api_key: &str, provider: &str) -> Result<()> {
        if provider == "gemini" && gemini_api_key.is_empty() {
            anyhow::bail!("GEMINI_API_KEY is not set");
        }
        Ok(())
    }

    fn validate_ollama_base_url(ollama_base_url: &str, provider: &str) -> Result<()> {
        if provider == "ollama" && ollama_base_url.is_empty() {
            anyhow::bail!("OLLAMA_API_BASE_URL is not set");
        }
        Ok(())
    }
}
