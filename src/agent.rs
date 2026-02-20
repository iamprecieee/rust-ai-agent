use anyhow::Result;
use futures::{Stream, StreamExt, stream::BoxStream};
use rig::{
    agent::{Agent as RigAgent, MultiTurnStreamItem, StreamingError},
    client::{CompletionClient, ProviderClient},
    providers::{gemini, ollama},
    streaming::{StreamedAssistantContent, StreamingPrompt},
};

use crate::config::Config;

pub enum Agent {
    Gemini(RigAgent<gemini::CompletionModel>),
    Ollama(RigAgent<ollama::CompletionModel>),
}

impl Agent {
    pub async fn new(config: Config) -> Result<Self> {
        Ok(match config.provider.as_str() {
            "gemini" => {
                let client = gemini::Client::from_env();
                let agent = Self::build_agent(client, &config);
                Self::Gemini(agent)
            }
            "ollama" => {
                let client = ollama::Client::from_env();
                let agent = Self::build_agent(client, &config);
                Self::Ollama(agent)
            }
            _ => anyhow::bail!("Invalid provider: {}", config.provider),
        })
    }

    fn build_agent<T: CompletionClient>(
        client: T,
        config: &Config,
    ) -> RigAgent<T::CompletionModel> {
        let preamble = "";
        client
            .agent(&config.agent_model)
            .preamble(preamble)
            .temperature(config.temperature)
            .build()
    }

    pub async fn chat(&self, prompt: &str) -> BoxStream<'static, String> {
        match self {
            Agent::Gemini(agent) => {
                let streaming_response = agent.stream_prompt(prompt).await;
                Box::pin(Self::unify_stream(streaming_response))
            }
            Agent::Ollama(agent) => {
                let streaming_response = agent.stream_prompt(prompt).await;
                Box::pin(Self::unify_stream(streaming_response))
            }
        }
    }

    fn unify_stream<R>(
        stream: impl Stream<Item = Result<MultiTurnStreamItem<R>, StreamingError>> + Send + 'static,
    ) -> impl Stream<Item = String> {
        stream.filter_map(|item| async {
            match item {
                Ok(MultiTurnStreamItem::StreamAssistantItem(StreamedAssistantContent::Text(
                    val,
                ))) => Some(val.text),
                Ok(MultiTurnStreamItem::StreamAssistantItem(
                    StreamedAssistantContent::Reasoning(_)
                    | StreamedAssistantContent::ReasoningDelta { .. },
                )) => Some("StreamedAssistantContent::Reasoning...".to_string()),
                _ => None,
            }
        })
    }
}
