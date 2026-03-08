use anyhow::Result;
use rig::completion::Prompt;
use rig::prelude::*;
use rig::providers::ollama;
use rig::client::Nothing;

use crate::tools::calculator::Calculator;
use crate::tools::file_search::FileSearch;
use crate::tools::weather::WeatherLookup;

pub async fn run(model: &str, question: &str) -> Result<String> {
    let client = ollama::Client::new(Nothing)?;

    let agent = client
        .agent(model)
        .preamble(
            "You are a helpful assistant with access to tools. \
             Use the appropriate tool to answer the user's question. \
             Available tools: calculator (math), get_weather (weather info), \
             search_files (find files). Always use a tool when the question \
             involves math, weather, or file searching.",
        )
        .max_tokens(1024)
        .tool(Calculator)
        .tool(WeatherLookup)
        .tool(FileSearch)
        .build();

    let response = agent.prompt(question).await?;
    Ok(response)
}
