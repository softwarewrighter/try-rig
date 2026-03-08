# Implementation Plan

## Phase 1: Foundation (Done)

- [x] Initialize Rust project
- [x] Add dependencies to `Cargo.toml`
- [x] Set up module structure (`tools/`, `agents/`, `demos/`)
- [x] Verify Ollama connectivity with a basic prompt

## Phase 2: CLI & Simple Agent (Done)

- [x] Add `clap` for CLI argument parsing with subcommands
- [x] Implement `ask` subcommand (single prompt → response)
- [x] Implement `chat` subcommand (interactive loop with history)
- [x] Add tracing/logging setup with `--verbose` flag

## Phase 3: Custom Tools (Done)

- [x] Implement `Calculator` tool (add, subtract, multiply, divide)
- [x] Implement `WeatherLookup` tool (simulated data for 8 cities)
- [x] Implement `FileSearch` tool (directory walk + glob pattern match)
- [x] Implement `DateTime` tool (current date/time with timezone)
- [x] Implement `StringTool` (uppercase, lowercase, reverse, count_words, replace, trim)
- [x] Create tool-using agent with all 5 tools
- [x] Add custom deserializer for string-encoded numbers from Ollama
- [x] Implement `tools` subcommand

## Phase 4: RAG Demo (Done)

- [x] Define 6-topic knowledge corpus (Rust, photosynthesis, K8s, sourdough, black holes, TCP/IP)
- [x] Implement embedding pipeline with `nomic-embed-text`
- [x] Build in-memory vector store with `InMemoryVectorStore`
- [x] Create RAG-augmented agent with `.dynamic_context(2, index)`
- [x] Implement `rag` subcommand

## Phase 5: Structured Extraction (Done)

- [x] Define `ContactInfo` struct with `schemars::JsonSchema`
- [x] Create typed extractor using Rig's `Extractor` API
- [x] Implement `extract` subcommand with pretty-printed JSON output

## Phase 6: Multi-agent Patterns (Done)

- [x] Create specialist agents with `.name()` and `.description()`
- [x] Implement agent-as-tool composition (calculator agent + weather agent)
- [x] Create orchestrator agent that routes to specialists
- [x] Implement `multi` subcommand

## Phase 7: Polish (Done)

- [x] Add `--model` flag to override the default model
- [x] Add `--verbose` flag for detailed tool call logging via tracing
- [x] Add streaming responses (`stream`, `stream-chat` subcommands)
- [x] Update README, status, and trials documentation
- [x] Test with llama3.2:3b and qwen2.5:7b-instruct

---

## Future Phases

### Phase 8: Dynamic Tool Selection

- [ ] Implement `ToolEmbedding` trait on existing tools (add `embedding_docs()`, `context()`)
- [ ] Build a tool index using embeddings
- [ ] Replace static `.tool()` calls with `.dynamic_tools(n, index, toolset)`
- [ ] Add a `dynamic-tools` subcommand to demo the difference
- [ ] Benchmark: compare accuracy with static vs dynamic tool selection

### Phase 9: Streaming with Tools

- [ ] Implement streaming tool-call display (show tool events inline)
- [ ] Handle `StreamedAssistantContent::ToolCall` and `ToolCallDelta` variants
- [ ] Show progress: "Calling calculator(add, 5, 3)... result: 8"
- [ ] Add `stream-tools` subcommand

### Phase 10: Chat History Persistence

- [ ] Define serializable conversation format (JSON)
- [ ] Save history to `~/.try-rig/conversations/`
- [ ] Add `--session <name>` flag to chat/stream-chat commands
- [ ] List and resume previous sessions

### Phase 11: Pipelines

- [ ] Create a multi-step pipeline: extract → validate → summarize
- [ ] Use rig's `Op` / `TryOp` traits and `pipeline::new()`
- [ ] Demo `parallel!` macro for concurrent steps
- [ ] Add `pipeline` subcommand

### Phase 12: Image/Vision

- [ ] Add image input support (accept file path argument)
- [ ] Use `llava` or `qwen2.5vl:7b` for image understanding
- [ ] Implement `vision` subcommand with describe/ocr/qa modes
- [ ] Handle base64 encoding of image data

### Phase 13: File Loaders

- [ ] Add text file loader for RAG corpus
- [ ] Add PDF loader (if rig supports it)
- [ ] Replace hardcoded knowledge entries with file-based ingestion
- [ ] Add `--docs <directory>` flag to `rag` subcommand

### Phase 14: Real Tools

- [ ] Replace simulated weather with a real API (e.g., Open-Meteo, free, no key)
- [ ] Add web search tool
- [ ] Add SQLite query tool
- [ ] Add `real-tools` subcommand or integrate into existing `tools`

### Phase 15: Error Recovery

- [ ] Detect Ollama not running and print helpful message
- [ ] Detect model not found and suggest available models
- [ ] Add retry logic for transient tool call failures
- [ ] Add `--timeout` flag for slow model responses

### Phase 16: Model Benchmarking

- [ ] Create benchmark harness with fixed test prompts
- [ ] Test tool calling across model sizes (360M to 14B)
- [ ] Measure: tool selection accuracy, JSON validity, response quality, latency
- [ ] Output results as markdown table
- [ ] Add `bench` subcommand
