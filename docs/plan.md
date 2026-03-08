# Implementation Plan

## Phase 1: Foundation

- [x] Initialize Rust project
- [ ] Add dependencies to `Cargo.toml`
- [ ] Set up module structure (`tools/`, `agents/`, `demos/`)
- [ ] Verify Ollama connectivity with a basic prompt

**Deliverable:** `cargo run` sends a prompt to Ollama and prints the response.

## Phase 2: CLI & Simple Agent

- [ ] Add `clap` for CLI argument parsing with subcommands
- [ ] Implement `ask` subcommand (single prompt → response)
- [ ] Implement `chat` subcommand (interactive loop with history)
- [ ] Add tracing/logging setup

**Deliverable:** `try-rig ask "question"` and `try-rig chat` work end-to-end.

## Phase 3: Custom Tools

- [ ] Implement `Calculator` tool (add, subtract, multiply, divide)
- [ ] Implement `WeatherLookup` tool (simulated data)
- [ ] Implement `FileSearch` tool (directory walk + pattern match)
- [ ] Create tool-using agent with all three tools
- [ ] Implement `tools` subcommand
- [ ] Display tool call details (which tool, args, result) in output

**Deliverable:** `try-rig tools "What's 42 * 17?"` shows the agent calling the calculator.

## Phase 4: RAG Demo

- [ ] Define sample document corpus (word definitions or FAQ)
- [ ] Implement embedding pipeline with `nomic-embed-text`
- [ ] Build in-memory vector store
- [ ] Create RAG-augmented agent
- [ ] Implement `rag` subcommand

**Deliverable:** `try-rig rag "What does flurbo mean?"` retrieves context and answers.

## Phase 5: Structured Extraction

- [ ] Define `ContactInfo` extraction struct with JSON schema
- [ ] Create extractor using Ollama model
- [ ] Implement `extract` subcommand
- [ ] Pretty-print extracted data

**Deliverable:** `try-rig extract "..."` extracts structured data from text.

## Phase 6: Multi-agent Patterns

- [ ] Create specialist agents (calculator agent, research agent)
- [ ] Implement agent-as-tool composition
- [ ] Create orchestrator agent
- [ ] Implement `multi` subcommand

**Deliverable:** `try-rig multi "..."` shows multi-agent delegation.

## Phase 7: Polish

- [ ] Improve error messages (especially Ollama connection failures)
- [ ] Add `--model` flag to override the default model
- [ ] Add `--verbose` flag for detailed tool call logging
- [ ] Update README with usage examples
- [ ] Update status doc

## Dependencies

```toml
[dependencies]
rig-core = { version = "0.32", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
anyhow = "1"
thiserror = "2"
clap = { version = "4", features = ["derive"] }
tracing = "0.1"
tracing-subscriber = "0.3"
glob = "0.3"
```
