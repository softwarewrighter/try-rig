# Product Requirements Document

## Goal

Build a Rust CLI tool using the Rig crate that demonstrates LLM-powered agent capabilities with Ollama as the local provider. The tool should serve as both a learning exercise and a reference implementation for Rig's features.

## Requirements

### R1: CLI Agent with Ollama

- Connect to a locally running Ollama instance (no API keys needed)
- Accept user questions from the command line
- Return LLM-generated responses
- Support interactive chat mode with conversation history

### R2: Tool-using Agent

- Implement at least 3 custom tools:
  - **Calculator** -- arithmetic operations (add, subtract, multiply, divide)
  - **Weather Lookup** -- simulated weather data for a given city
  - **File Search** -- search for files matching a pattern in a directory
- Agent autonomously decides which tool to call based on the user's question
- Display tool invocations and results transparently

### R3: RAG Demo

- Load a small corpus of documents into an in-memory vector store
- Use `nomic-embed-text` via Ollama for embeddings
- Demonstrate retrieval-augmented generation where the agent answers questions using retrieved context

### R4: Structured Extraction

- Extract typed Rust structs from unstructured text using Rig's extractor
- Demo: extract contact info (name, email, phone) from a paragraph

### R5: Multi-agent Demo

- Demonstrate agent-as-tool pattern (one agent delegates to another)
- Show how a router agent selects the right specialist agent

### Non-goals

- Production deployment, authentication, persistence
- External vector store integrations (MongoDB, Qdrant, etc.)
- Streaming responses (can be added later)
