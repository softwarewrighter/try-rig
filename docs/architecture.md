# Architecture

## Rig Core Architecture

Rig is layered into these core abstractions:

```
┌─────────────────────────────────────────────┐
│                  Application                │
│         (Agent, Extractor, Pipeline)        │
├─────────────────────────────────────────────┤
│              Capabilities                   │
│    (Tools, RAG, Dynamic Context/Tools)      │
├─────────────────────────────────────────────┤
│            Model Interfaces                 │
│   (CompletionModel, EmbeddingModel traits)  │
├─────────────────────────────────────────────┤
│               Providers                     │
│   (Ollama, OpenAI, Anthropic, 20+ more)     │
└─────────────────────────────────────────────┘
```

### Key Traits

| Trait | Purpose |
|---|---|
| `CompletionModel` | LLM text completion |
| `EmbeddingModel` | Vector embedding generation |
| `Prompt` | Single-shot `.prompt()` on agents |
| `Chat` | `.chat()` with message history |
| `Tool` | Define callable functions for agents |
| `ToolDyn` | Dynamic dispatch variant of Tool |
| `VectorStoreIndex` | Semantic search over embeddings |
| `Op` / `TryOp` | Pipeline operation composition |

### Agent Architecture

```
AgentBuilder
  ├── .preamble("system prompt")
  ├── .context("static context")
  ├── .dynamic_context(n, vector_index)   // RAG
  ├── .tool(MyTool)                        // static tools
  ├── .dynamic_tools(n, index, toolset)    // RAG-selected tools
  └── .build() → Agent
```

An `Agent` combines:
1. A model (any `CompletionModel`)
2. A system prompt (preamble)
3. Optional static context documents
4. Optional dynamic context via vector search (RAG)
5. Optional tools (static or dynamically selected)

### Tool Trait

```rust
trait Tool {
    type Error: Error;
    type Args: Deserialize;
    type Output: Serialize;
    const NAME: &'static str;

    async fn definition(&self, prompt: String) -> ToolDefinition;
    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error>;
}
```

Tools return a `ToolDefinition` with a JSON Schema describing parameters. The LLM sees this schema and generates structured arguments to call the tool.

## Project Architecture

```
try-rig/
├── src/
│   ├── main.rs          # CLI entry point, argument parsing
│   ├── tools/
│   │   ├── mod.rs
│   │   ├── calculator.rs    # Math operations tool
│   │   ├── weather.rs       # Simulated weather tool
│   │   └── file_search.rs   # File search tool
│   ├── agents/
│   │   ├── mod.rs
│   │   ├── simple.rs        # Basic Q&A agent
│   │   ├── tool_agent.rs    # Agent with tools
│   │   ├── rag_agent.rs     # RAG demo agent
│   │   └── multi_agent.rs   # Multi-agent orchestration
│   └── demos/
│       ├── mod.rs
│       ├── extraction.rs    # Structured extraction demo
│       └── rag.rs           # RAG demo with sample docs
├── docs/
└── Cargo.toml
```

### Ollama Provider

Rig's Ollama client connects to `http://localhost:11434` by default. No API key is needed -- the client accepts `Nothing` as credentials.

```rust
let client = ollama::Client::new(Nothing).unwrap();
let agent = client.agent("llama3.2").preamble("...").build();
```

Predefined model constants: `ollama::LLAMA3_2`, `ollama::MISTRAL`, `ollama::ALL_MINILM`, `ollama::NOMIC_EMBED_TEXT`.
