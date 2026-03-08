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
struct KnowledgeEntry {
    id: String,
    topic: String,
    #[embed]
    content: Vec<String>,
}

pub async fn run(model: &str, question: &str) -> Result<String> {
    let client = ollama::Client::new(Nothing)?;
    let embedding_model = client.embedding_model_with_ndims("nomic-embed-text", 768);

    let documents = vec![
        KnowledgeEntry {
            id: "rust_ownership".into(),
            topic: "Rust Ownership".into(),
            content: vec![
                "Rust uses an ownership system to manage memory without a garbage collector. \
                 Each value has exactly one owner, and when the owner goes out of scope, \
                 the value is dropped. Ownership can be transferred via moves or shared \
                 via borrowing with references.".into(),
                "The borrow checker enforces rules at compile time: you can have either \
                 one mutable reference or any number of immutable references, but not both. \
                 This prevents data races and use-after-free bugs.".into(),
            ],
        },
        KnowledgeEntry {
            id: "photosynthesis".into(),
            topic: "Photosynthesis".into(),
            content: vec![
                "Photosynthesis is the process by which green plants and certain organisms \
                 convert light energy into chemical energy. Using sunlight, water, and carbon \
                 dioxide, plants produce glucose and release oxygen as a byproduct.".into(),
                "The process occurs in chloroplasts, specifically using chlorophyll pigments \
                 that absorb red and blue light while reflecting green light. The light \
                 reactions happen in thylakoid membranes and the Calvin cycle in the stroma.".into(),
            ],
        },
        KnowledgeEntry {
            id: "kubernetes".into(),
            topic: "Kubernetes".into(),
            content: vec![
                "Kubernetes (K8s) is an open-source container orchestration platform that \
                 automates deployment, scaling, and management of containerized applications. \
                 It groups containers into pods, the smallest deployable units.".into(),
                "Key components include the API server, etcd for state storage, the scheduler \
                 for pod placement, and kubelet agents on each node. Services provide stable \
                 networking, and deployments manage rolling updates.".into(),
            ],
        },
        KnowledgeEntry {
            id: "sourdough".into(),
            topic: "Sourdough Bread".into(),
            content: vec![
                "Sourdough bread is made using a naturally fermented starter culture of wild \
                 yeast and lactic acid bacteria instead of commercial yeast. The starter is \
                 maintained by regularly feeding it with flour and water.".into(),
                "The long fermentation process (12-24 hours) develops complex flavors and \
                 a tangy taste from lactic and acetic acids. Sourdough has a lower glycemic \
                 index than regular bread and the fermentation breaks down gluten partially, \
                 making it easier to digest for some people.".into(),
            ],
        },
        KnowledgeEntry {
            id: "black_holes".into(),
            topic: "Black Holes".into(),
            content: vec![
                "A black hole is a region of spacetime where gravity is so strong that \
                 nothing, not even light or electromagnetic radiation, can escape. They form \
                 when massive stars collapse at the end of their life cycle in a supernova.".into(),
                "The boundary of a black hole is called the event horizon. Beyond it lies the \
                 singularity, a point of theoretically infinite density. Hawking radiation \
                 suggests black holes slowly evaporate over astronomical timescales.".into(),
            ],
        },
        KnowledgeEntry {
            id: "tcp_ip".into(),
            topic: "TCP/IP Networking".into(),
            content: vec![
                "TCP/IP is the foundational protocol suite for the internet. TCP (Transmission \
                 Control Protocol) provides reliable, ordered delivery of data through \
                 three-way handshakes, sequence numbers, and acknowledgments.".into(),
                "IP (Internet Protocol) handles addressing and routing packets across networks. \
                 IPv4 uses 32-bit addresses while IPv6 uses 128-bit addresses. Together TCP/IP \
                 enables applications like HTTP, email, and file transfer to communicate \
                 across diverse networks.".into(),
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
            "You are a knowledgeable assistant. Use the provided context to answer \
             questions accurately. Base your answers on the context given. \
             If the context doesn't contain relevant information, say so.",
        )
        .dynamic_context(2, index)
        .build();

    let response = rag_agent.prompt(question).await?;
    Ok(response)
}
