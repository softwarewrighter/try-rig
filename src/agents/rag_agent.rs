use anyhow::Result;
use rig::completion::Prompt;
use rig::embeddings::EmbeddingsBuilder;
use rig::prelude::*;
use rig::providers::ollama;
use rig::client::Nothing;
use rig::vector_store::in_memory_store::InMemoryVectorStore;
use rig::Embed;
use serde::Serialize;

#[derive(Embed, Serialize, Clone, Debug, Eq, PartialEq, Default)]
struct Definition {
    id: String,
    word: String,
    #[embed]
    definitions: Vec<String>,
}

pub async fn run(model: &str, question: &str) -> Result<String> {
    let client = ollama::Client::new(Nothing)?;
    let embedding_model = client.embedding_model_with_ndims("nomic-embed-text", 768);

    let documents = vec![
        Definition {
            id: "doc0".into(),
            word: "flurbo".into(),
            definitions: vec![
                "A green alien currency used on planet Zorblatt.".into(),
                "Worth approximately 3.7 Earth dollars.".into(),
            ],
        },
        Definition {
            id: "doc1".into(),
            word: "glarb".into(),
            definitions: vec![
                "A type of intergalactic sandwich made with quantum bread.".into(),
                "Popular among space travelers for its zero-gravity stability.".into(),
            ],
        },
        Definition {
            id: "doc2".into(),
            word: "znorp".into(),
            definitions: vec![
                "The sound made by a Plutonian trumpet.".into(),
                "Often described as a mix between a honk and a cosmic hum.".into(),
            ],
        },
        Definition {
            id: "doc3".into(),
            word: "quibblex".into(),
            definitions: vec![
                "A philosophical debate format used by robots.".into(),
                "Arguments must be expressed in prime numbers of words.".into(),
            ],
        },
    ];

    println!("Building embeddings...");
    let embeddings = EmbeddingsBuilder::new(embedding_model.clone())
        .documents(documents)?
        .build()
        .await?;

    let vector_store = InMemoryVectorStore::from_documents(embeddings);
    let index = vector_store.index(embedding_model);

    let rag_agent = client
        .agent(model)
        .preamble(
            "You are a dictionary assistant for made-up words. \
             Use the provided context to answer questions about word definitions. \
             If the context doesn't contain relevant information, say so.",
        )
        .dynamic_context(2, index)
        .build();

    let response = rag_agent.prompt(question).await?;
    Ok(response)
}
