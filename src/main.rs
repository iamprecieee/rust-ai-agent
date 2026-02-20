use std::io::Write;

use anyhow::{Context, Result};
use colorized::{Color, Colors};
use futures::StreamExt;

use agent::{agent::Agent, config::Config, logging::setup_logging};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _guard = setup_logging()?;

    let config = Config::from_env()?;

    let agent = Agent::new(config).await.context("Failed to create agent")?;

    println!(
        "{} {} {} {} {}",
        "Agent initialized. Type".color(Colors::BrightYellowFg),
        "'exit'".color(Colors::BrightRedFg),
        "or".color(Colors::BrightYellowFg),
        "'quit'".color(Colors::BrightRedFg),
        "to stop.".color(Colors::BrightYellowFg)
    );

    loop {
        print!("{}", "You: ".color(Colors::BrightBlueFg));
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;
        input = input.trim().to_string();

        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            break;
        }

        if input.is_empty() {
            continue;
        }

        let mut stream = agent.chat(&input).await;

        print!("{}", "Agent: ".color(Colors::BrightMagentaFg));
        std::io::stdout().flush()?;

        let mut is_agent_thinking = false;

        while let Some(chunk) = stream.next().await {
            if chunk.contains("StreamedAssistantContent::Reasoning") {
                if !is_agent_thinking {
                    print!("\r\x1B[K");
                    std::io::stdout().flush()?;
                    print!("{}", "Agent (thinking...)".color(Colors::BrightMagentaFg));
                    std::io::stdout().flush()?;
                    is_agent_thinking = true;
                }
            } else {
                if is_agent_thinking {
                    print!("\r\x1B[K");
                    std::io::stdout().flush()?;
                    print!("{}", "Agent: ".color(Colors::BrightMagentaFg));
                    std::io::stdout().flush()?;
                    is_agent_thinking = false;
                }
                print!("{}", chunk);
            }
            std::io::stdout().flush()?;
        }
        println!("{}", "\n---".color(Colors::BrightGreenFg));
    }

    Ok(())
}
