use anyhow::Result;
use rig::completion::Prompt;
use rig::prelude::*;
use rig::providers::ollama;
use rig::client::Nothing;

use crate::tools::calculator::Calculator;
use crate::tools::weather::WeatherLookup;

pub async fn run(model: &str, question: &str) -> Result<String> {
    let client = ollama::Client::new(Nothing)?;

    // Specialist: Calculator agent
    let calc_agent = client
        .agent(model)
        .preamble(
            "You are a math specialist. Use the calculator tool to solve math problems. \
             Always show your work.",
        )
        .max_tokens(1024)
        .tool(Calculator)
        .build();

    // Specialist: Weather agent
    let weather_agent = client
        .agent(model)
        .preamble(
            "You are a weather specialist. Use the get_weather tool to look up weather \
             information. Provide a brief, friendly weather report.",
        )
        .max_tokens(1024)
        .tool(WeatherLookup)
        .build();

    // Orchestrator: uses specialist agents as tools
    let orchestrator = client
        .agent(model)
        .preamble(
            "You are an orchestrator assistant. You have access to specialist agents: \
             a math agent for calculations, and a weather agent for weather lookups. \
             Route the user's question to the appropriate specialist. \
             If the question involves multiple topics, call multiple specialists.",
        )
        .max_tokens(2048)
        .tool(calc_agent)
        .tool(weather_agent)
        .build();

    let response = orchestrator.prompt(question).await?;
    Ok(response)
}
