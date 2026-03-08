# try-rig

Exploring the [Rig](https://rig.rs/) Rust crate for building LLM-powered applications. This project builds a CLI tool that uses Ollama locally, demonstrates AI agents with custom tools, and explores Rig's broader capabilities.

## Overview

Rig is an open-source Rust library for building LLM-powered applications with a focus on ergonomics, modularity, and type safety. It supports 20+ LLM providers, tool calling, RAG, structured extraction, pipelines, and multi-agent patterns.

This project demonstrates:

- **CLI Agent** -- A command-line tool that talks to Ollama locally
- **Tool-using Agent** -- Custom tools (calculator, weather lookup, file search) that an agent can call autonomously
- **RAG** -- Retrieval-augmented generation with in-memory vector store
- **Structured Extraction** -- Pulling typed data from unstructured text
- **Multi-agent Patterns** -- Agent-as-tool composition, routing, and orchestration

## Prerequisites

- Rust (edition 2024)
- [Ollama](https://ollama.ai/) running locally (`ollama serve`)
- Models pulled: `llama3.2`, `nomic-embed-text`

## Usage

```bash
cargo run
```

## Documentation

- [Product Requirements](docs/prd.md)
- [Architecture](docs/architecture.md)
- [Design](docs/design.md)
- [Implementation Plan](docs/plan.md)
- [Status](docs/status.md)

## Links

- [Rig website](https://rig.rs/)
- [Rig docs (docs.rs)](https://docs.rs/rig-core/latest/rig/)
- [Rig GitHub](https://github.com/0xPlaygrounds/rig)
- [Ollama](https://ollama.ai/)
