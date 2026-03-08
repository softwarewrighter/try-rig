# try-rig

Exploring the [Rig](https://rig.rs/) Rust crate for building LLM-powered applications. This project builds a CLI tool that uses Ollama locally, demonstrates AI agents with custom tools, and explores Rig's broader capabilities.

## Demo

![try-rig demo](docs/demo.gif)

## Overview

Rig is an open-source Rust library for building LLM-powered applications with a focus on ergonomics, modularity, and type safety. It supports 20+ LLM providers, tool calling, RAG, structured extraction, pipelines, and multi-agent patterns.

This project demonstrates:

- **CLI Agent** -- A command-line tool that talks to Ollama locally
- **Tool-using Agent** -- 5 custom tools (calculator, weather, file search, datetime, string ops)
- **RAG** -- Retrieval-augmented generation with in-memory vector store
- **Structured Extraction** -- Typed extraction with `schemars::JsonSchema`
- **Multi-agent Patterns** -- Agent-as-tool composition with orchestrator routing
- **Streaming** -- Token-by-token streaming responses

## Prerequisites

- Rust (edition 2024)
- [Ollama](https://ollama.ai/) running locally (`ollama serve`)
- Models pulled: `qwen2.5:7b-instruct` (recommended), `nomic-embed-text`

```bash
ollama pull qwen2.5:7b-instruct
ollama pull nomic-embed-text
```

## Usage

```bash
# Simple question
cargo run -- -m qwen2.5:7b-instruct ask "What is Rust?"

# Interactive chat
cargo run -- -m qwen2.5:7b-instruct chat

# Tool-calling agent (calculator, weather, datetime, string ops, file search)
cargo run -- -m qwen2.5:7b-instruct tools "What is 42 * 17?"
cargo run -- -m qwen2.5:7b-instruct tools "What time is it?"
cargo run -- -m qwen2.5:7b-instruct tools "What is the weather in Tokyo?"

# RAG with knowledge base
cargo run -- -m qwen2.5:7b-instruct rag "How does Rust manage memory?"

# Structured extraction
cargo run -- -m qwen2.5:7b-instruct extract "Contact Jane at jane@co.org or 555-0199"

# Multi-agent orchestration
cargo run -- -m qwen2.5:7b-instruct multi "What is 100 divided by 7?"

# Streaming responses
cargo run -- -m qwen2.5:7b-instruct stream "Explain TCP in two sentences"
cargo run -- -m qwen2.5:7b-instruct stream-chat

# Verbose mode (shows tool calls and args)
cargo run -- -v -m qwen2.5:7b-instruct tools "What is 5 + 3?"
```

## Documentation

- [Product Requirements](docs/prd.md)
- [Architecture](docs/architecture.md)
- [Design](docs/design.md)
- [Implementation Plan](docs/plan.md)
- [Status](docs/status.md)
- [Trial Results](docs/trials.md)

## Links

- [Rig website](https://rig.rs/)
- [Rig docs (docs.rs)](https://docs.rs/rig-core/latest/rig/)
- [Rig GitHub](https://github.com/0xPlaygrounds/rig)
- [Ollama](https://ollama.ai/)
