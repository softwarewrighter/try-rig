use anyhow::Result;
use rig::completion::Prompt;
use rig::prelude::*;
use rig::providers::ollama;
use rig::client::Nothing;

use crate::tools::calculator::Calculator;
use crate::tools::weather::WeatherLookup;

pub async fn run(model: &str, question: &str) -> Result<String> {
    let client = ollama::Client::new(Nothing)?;

    // Specialist: Calculator agent with a distinct name and description
    let calc_agent = client
        .agent(model)
        .preamble(
            "You are a math specialist. Use the calculator tool to solve math problems. \
             Return just the numeric result with a brief explanation.",
        )
        .name("math_agent")
        .description("A math specialist agent that can perform arithmetic: add, subtract, multiply, divide. Send it a math question as the prompt.")
        .max_tokens(1024)
        .tool(Calculator)
        .build();

    // Specialist: Weather agent with a distinct name and description
    let weather_agent = client
        .agent(model)
        .preamble(
            "You are a weather specialist. Use the get_weather tool to look up weather \
             information. Provide a brief, friendly weather report.",
        )
        .name("weather_agent")
        .description("A weather specialist agent that looks up current weather for cities. Send it a city name or weather question as the prompt.")
        .max_tokens(1024)
        .tool(WeatherLookup)
        .build();

    // Orchestrator: uses specialist agents as tools
    let orchestrator = client
        .agent(model)
        .preamble(
            "You are an orchestrator assistant with access to two specialist tools:\n\
             - math_agent: for arithmetic calculations (send it math questions)\n\
             - weather_agent: for weather lookups (send it a city name)\n\n\
             For each user question, call the appropriate tool with a clear prompt. \
             Then use the tool's response to answer the user.",
        )
        .max_tokens(2048)
        .tool(calc_agent)
        .tool(weather_agent)
        .build();

    let response = orchestrator.prompt(question).await?;
    Ok(response)
}
