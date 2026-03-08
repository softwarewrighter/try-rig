# Product Requirements Document

## Goal

Build a Rust CLI tool using the Rig crate that demonstrates LLM-powered agent capabilities with Ollama as the local provider. The tool should serve as both a learning exercise and a reference implementation for Rig's features.

## Requirements (Completed)

### R1: CLI Agent with Ollama

- Connect to a locally running Ollama instance (no API keys needed)
- Accept user questions from the command line
- Return LLM-generated responses
- Support interactive chat mode with conversation history

### R2: Tool-using Agent

- 5 custom tools: Calculator, WeatherLookup, FileSearch, DateTime, StringTool
- Agent autonomously decides which tool to call based on the user's question
- Display tool invocations and results transparently (`--verbose` flag)

### R3: RAG Demo

- 6 semantically distinct knowledge entries in an in-memory vector store
- `nomic-embed-text` via Ollama for embeddings
- Retrieval-augmented generation with `.dynamic_context()`

### R4: Structured Extraction

- Typed extraction using Rig's `Extractor` with `schemars::JsonSchema`
- `ContactInfo` struct with name, email, phone fields

### R5: Multi-agent Demo

- Agent-as-tool pattern with named/described sub-agents
- Orchestrator routes to math agent or weather agent

### R6: Streaming

- Token-by-token streaming output for single prompts and interactive chat
- Uses `StreamingPrompt` / `StreamingChat` traits

## Future Requirements

### R7: Dynamic Tool Selection (RAG'd Tools)

- Use `ToolEmbedding` trait to make tools discoverable via vector search
- Use `.dynamic_tools(n, index, toolset)` instead of loading all tools statically
- Agent selects relevant tools per query from a larger toolset
- Reduces prompt size and improves accuracy when many tools are available

### R8: Streaming with Tools

- Stream responses while the agent calls tools mid-stream
- Follow rig's `ollama_streaming_with_tools` pattern
- Show tool call events inline during streaming output

### R9: Chat History Persistence

- Save/load conversation history to disk (JSON or MessagePack)
- Resume previous conversations across sessions
- Support named conversation threads

### R10: Pipelines

- Use rig's `pipeline::new().map().chain()` DAG-based workflow composition
- Chain multiple AI and non-AI operations (e.g., extract → validate → summarize)
- Explore `parallel!` macro for concurrent pipeline steps

### R11: Image/Vision

- Use `llava` or `qwen2.5vl:7b` (already downloaded) for image understanding
- Accept image file paths as input
- Demonstrate image description, OCR, and visual Q&A

### R12: File Loaders

- Load PDFs, text files, or other documents into the RAG pipeline
- Replace hardcoded knowledge entries with real file content
- Support incremental document ingestion

### R13: Real Tools

- Replace simulated weather with a real weather API
- Add web search tool (e.g., via a search API)
- Add database query tool (SQLite)
- Demonstrate real-world tool integration patterns

### R14: Error Recovery

- Graceful handling when Ollama is down (detect, report, suggest fixes)
- Model-not-found detection with available model suggestions
- Tool call retry with exponential backoff
- Timeout handling for slow model responses

### R15: Model Benchmarking

- Compare tool-calling accuracy across downloaded models
- Test range: smollm:360m, qwen2.5:0.5b, qwen2.5-coder:1.5b, llama3.2:3b,
  qwen2.5:7b-instruct, llama3.1:8b, qwen2.5-coder:14b
- Measure: tool selection accuracy, JSON validity, response quality, latency
- Output results as a comparison table
