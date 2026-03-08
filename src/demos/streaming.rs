use anyhow::Result;
use futures::StreamExt;
use rig::agent::MultiTurnStreamItem;
use rig::completion::Message;
use rig::message::{AssistantContent, UserContent};
use rig::prelude::*;
use rig::providers::ollama;
use rig::client::Nothing;
use rig::streaming::{StreamedAssistantContent, StreamingChat, StreamingPrompt};
use rig::OneOrMany;
use std::io::{self, Write};

pub async fn ask(model: &str, question: &str) -> Result<()> {
    let client = ollama::Client::new(Nothing)?;

    let agent = client
        .agent(model)
        .preamble("You are a helpful assistant. Be concise and direct.")
        .build();

    let mut stream = agent.stream_prompt(question).await;

    while let Some(chunk) = stream.next().await {
        match chunk {
            Ok(MultiTurnStreamItem::StreamAssistantItem(
                StreamedAssistantContent::Text(text),
            )) => {
                print!("{}", text.text);
                io::stdout().flush()?;
            }
            Err(e) => {
                eprintln!("\nStream error: {e}");
                break;
            }
            _ => {}
        }
    }
    println!();

    Ok(())
}

pub async fn chat(model: &str) -> Result<()> {
    let client = ollama::Client::new(Nothing)?;

    let agent = client
        .agent(model)
        .preamble("You are a helpful assistant. Be concise and direct.")
        .build();

    let mut history: Vec<Message> = Vec::new();
    println!("Streaming chat mode (type 'quit' to exit)");

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

        let mut stream = agent.stream_chat(input, history.clone()).await;
        let mut response_text = String::new();

        print!("\n");
        while let Some(chunk) = stream.next().await {
            match chunk {
                Ok(MultiTurnStreamItem::StreamAssistantItem(
                    StreamedAssistantContent::Text(text),
                )) => {
                    print!("{}", text.text);
                    io::stdout().flush()?;
                    response_text.push_str(&text.text);
                }
                Err(e) => {
                    eprintln!("\nStream error: {e}");
                    break;
                }
                _ => {}
            }
        }
        println!("\n");

        history.push(Message::User {
            content: OneOrMany::one(UserContent::Text(input.into())),
        });
        history.push(Message::Assistant {
            id: None,
            content: OneOrMany::one(AssistantContent::Text(response_text.into())),
        });
    }

    Ok(())
}
