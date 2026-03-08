# Design

## CLI Interface

The CLI uses subcommands to select different demos:

```
try-rig ask "What is the capital of France?"      # Simple Q&A
try-rig chat                                       # Interactive chat
try-rig tools "What is 42 + 17?"                  # Tool-using agent
try-rig rag "What does flurbo mean?"              # RAG demo
try-rig extract "Call John at 555-1234"           # Extraction demo
try-rig multi "Calculate 2+2 then summarize"      # Multi-agent demo
```

## Tool Implementations

### Calculator Tool

```rust
struct Calculator;

impl Tool for Calculator {
    const NAME: &'static str = "calculator";
    type Args = CalcArgs;   // { operation: String, x: f64, y: f64 }
    type Output = f64;
    // Supports: add, subtract, multiply, divide
}
```

### Weather Tool

```rust
struct WeatherLookup;

impl Tool for WeatherLookup {
    const NAME: &'static str = "get_weather";
    type Args = WeatherArgs;  // { city: String }
    type Output = WeatherInfo; // { temp: f64, condition: String, humidity: u8 }
    // Returns simulated weather data from a hardcoded HashMap
}
```

### File Search Tool

```rust
struct FileSearch;

impl Tool for FileSearch {
    const NAME: &'static str = "search_files";
    type Args = SearchArgs;   // { directory: String, pattern: String }
    type Output = Vec<String>; // matching file paths
    // Uses std::fs to walk directory and glob-match
}
```

## RAG Design

1. Define a set of sample documents (word definitions, FAQ entries, etc.)
2. Use `#[derive(Embed)]` on document structs with `#[embed]` on searchable fields
3. Build embeddings with `EmbeddingsBuilder` using Ollama's `nomic-embed-text`
4. Store in `InMemoryVectorStore`
5. Create agent with `.dynamic_context(2, index)` to retrieve top-2 docs per query

## Structured Extraction Design

Use Rig's extractor to parse unstructured text into typed structs:

```rust
#[derive(Deserialize, JsonSchema)]
struct ContactInfo {
    name: String,
    email: Option<String>,
    phone: Option<String>,
}

let extractor = client.extractor::<ContactInfo>("llama3.2").build();
let contact = extractor.extract("Email john@example.com or call 555-1234").await?;
```

## Multi-agent Design

**Agent-as-tool pattern:**
- A `CalculatorAgent` wraps the calculator tools
- A `ResearchAgent` wraps the RAG pipeline
- An `OrchestratorAgent` has both as tools and routes queries

**Router pattern:**
- Classify the query type (math, lookup, general)
- Dispatch to the appropriate specialist agent

## Error Handling

- Use `anyhow::Result` at the application level
- Use `thiserror` for tool-specific errors
- Graceful fallback when Ollama is not running
