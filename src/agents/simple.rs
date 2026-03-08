use anyhow::Result;
use rig::completion::{Chat, Message, Prompt};
use rig::message::{AssistantContent, UserContent};
use rig::prelude::*;
use rig::providers::ollama;
use rig::client::Nothing;
use rig::OneOrMany;

pub async fn ask(model: &str, question: &str) -> Result<String> {
    let client = ollama::Client::new(Nothing)?;

    let agent = client
        .agent(model)
        .preamble("You are a helpful assistant. Be concise and direct in your responses.")
        .build();

    let response = agent.prompt(question).await?;
    Ok(response)
}

pub async fn chat(model: &str) -> Result<()> {
    use std::io::{self, Write};

    let client = ollama::Client::new(Nothing)?;

    let agent = client
        .agent(model)
        .preamble("You are a helpful assistant. Be concise and direct in your responses.")
        .build();

    let mut history: Vec<Message> = Vec::new();
    println!("Chat mode (type 'quit' to exit)");

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.eq_ignore_ascii_case("quit") {
            break;
        }

        if input.is_empty() {
            continue;
        }

        let response = agent.chat(input, history.clone()).await?;
        println!("\n{response}\n");

        history.push(Message::User {
            content: OneOrMany::one(UserContent::Text(input.into())),
        });
        history.push(Message::Assistant {
            id: None,
            content: OneOrMany::one(AssistantContent::Text(response.into())),
        });
    }

    Ok(())
}
