mod agents;
mod demos;
mod tools;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "try-rig", about = "Explore the Rig crate with Ollama")]
struct Cli {
    /// Ollama model to use
    #[arg(short, long, default_value = "llama3.2")]
    model: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Ask a single question
    Ask {
        /// The question to ask
        question: String,
    },
    /// Interactive chat mode
    Chat,
    /// Ask a question using an agent with tools (calculator, weather, file search)
    Tools {
        /// The question to ask
        question: String,
    },
    /// RAG demo with made-up word definitions
    Rag {
        /// The question to ask
        question: String,
    },
    /// Extract structured data from text
    Extract {
        /// The text to extract data from
        text: String,
    },
    /// Multi-agent orchestration demo
    Multi {
        /// The question to ask
        question: String,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive(tracing::Level::WARN.into()),
        )
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Ask { question } => {
            let response = agents::simple::ask(&cli.model, &question).await?;
            println!("{response}");
        }
        Commands::Chat => {
            agents::simple::chat(&cli.model).await?;
        }
        Commands::Tools { question } => {
            println!("Using tool-equipped agent...\n");
            let response = agents::tool_agent::run(&cli.model, &question).await?;
            println!("{response}");
        }
        Commands::Rag { question } => {
            let response = agents::rag_agent::run(&cli.model, &question).await?;
            println!("{response}");
        }
        Commands::Extract { text } => {
            let response = demos::extraction::run(&cli.model, &text).await?;
            println!("{response}");
        }
        Commands::Multi { question } => {
            println!("Using multi-agent orchestrator...\n");
            let response = agents::multi_agent::run(&cli.model, &question).await?;
            println!("{response}");
        }
    }

    Ok(())
}
