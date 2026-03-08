use anyhow::Result;
use rig::completion::Prompt;
use rig::prelude::*;
use rig::providers::ollama;
use rig::client::Nothing;

pub async fn run(model: &str, text: &str) -> Result<String> {
    let client = ollama::Client::new(Nothing)?;

    let agent = client
        .agent(model)
        .preamble(
            "You are a data extraction assistant. Extract contact information from the text. \
             Return a JSON object with these fields: \
             {\"name\": string or null, \"email\": string or null, \"phone\": string or null}. \
             Only return the JSON, no other text.",
        )
        .build();

    let response = agent.prompt(text).await?;
    Ok(response)
}
