# Trial Runs

Date: 2026-03-08
Model: `llama3.2:latest` (3B parameters)
Ollama: running locally on port 11434

## 1. Simple Ask

```
$ cargo run -- ask "What is Rust?"
```

**Output:**
```
Rust is a systems programming language that prioritizes safety and performance. It was designed
to provide memory safety guarantees without sacrificing speed. Key features include:

- Ownership and borrowing
- Type system with zero-cost abstractions
- Concurrency model
- Error handling
- Support for systems programming and building systems

Rust aims to provide a balance between modern language features and the reliability and
efficiency of languages like C.
```

**Result:** Works well. Clean, concise response.

## 2. Tool-using Agent

```
$ cargo run -- tools "What is 42 * 17?"
```

**Output:**
```
Using tool-equipped agent...

{"name": "Perform arithmetic operations", "parameters": {"operation": "multiply", "x": "42", "y": "17"}}
```

**Result:** Partial success. The model recognized it should use the calculator tool and
generated correct parameters, but returned raw JSON instead of a natural language answer.
The model sent numbers as strings (fixed with custom deserializer). The small 3B model
struggles with the full tool-calling round-trip.

**Note:** First run (before fix) produced a `ToolCallError: JsonError: invalid type: string "42",
expected f64` because `llama3.2` sends numeric arguments as strings. Added a custom
`deserialize_f64_from_any` deserializer to handle both.

## 3. RAG Demo

```
$ cargo run -- rag "What does flurbo mean?"
```

**Output:**
```
Building embeddings...
I couldn't find any information about the word "flurbo" in the provided context. The context
only contains information about "quibblex" and "znorp", which are not related to the word
"flurbo".
```

**Result:** RAG pipeline works (embeddings built, vector search retrieved documents, agent
responded with context), but the vector search returned the wrong documents (quibblex and
znorp instead of flurbo). This is likely due to the small embedding model's limited semantic
discrimination on short made-up words. The agent correctly reported it couldn't find flurbo
in the retrieved context.

## 4. Structured Extraction

```
$ cargo run -- extract "Please contact John Smith at john@example.com or call 555-867-5309"
```

**Output:**
```
{"name": "John Smith", "email": "john@example.com", "phone": "555-867-5309"}
```

**Result:** Works perfectly. Clean JSON output with all three fields correctly extracted.

## 5. Multi-agent Orchestration

```
$ cargo run -- multi "What is the weather in Tokyo?"
```

**Output:**
```
Using multi-agent orchestrator...

{"name": "get_weather", "parameters": {"prompt": "Tokyo"}}
```

**Result:** Partial success. The orchestrator recognized the query should go to the weather
specialist but returned raw JSON for the tool call instead of completing the round-trip.
The `llama3.2:3b` model has difficulty with nested agent-as-tool invocation. A larger model
(e.g. `llama3.1:8b` or `qwen2.5:7b-instruct`) would likely handle this better.

## Summary

| Demo | Status | Notes |
|---|---|---|
| ask | Works | Clean responses |
| tools | Partial | Correct tool selection, but raw JSON output instead of NL answer |
| rag | Partial | Pipeline works, but vector search returned wrong docs for made-up words |
| extract | Works | Perfect JSON extraction |
| multi | Partial | Orchestrator routes correctly but doesn't complete tool round-trip |

## Observations

1. **Model size matters for tool calling.** The 3B `llama3.2` model can identify which tool to
   use but struggles with the full tool-call protocol (generating proper function calls,
   processing results, and forming natural language answers).

2. **String vs number types.** Local models often serialize numbers as strings in JSON tool
   calls. The custom deserializer fix is essential for robustness with Ollama models.

3. **RAG with made-up words.** Embedding models have trouble distinguishing between short,
   novel words with no real-world semantic content. A larger corpus or more distinctive
   documents would improve retrieval accuracy.

4. **Extraction works great.** Even the small model handles prompt-based structured extraction
   reliably when given clear formatting instructions.

5. **Recommended next steps:** Try with `llama3.1:8b` or `qwen2.5:7b-instruct` for better
   tool-calling behavior. Consider adding `--model` flag usage examples.

---

## Trial 2: qwen2.5:7b-instruct

Date: 2026-03-08
Model: `qwen2.5:7b-instruct` (7B parameters)

### 1. Simple Ask

```
$ cargo run -- -m qwen2.5:7b-instruct ask "What is Rust?"
```

**Output:**
```
Rust is a systems programming language that focuses on safety, concurrency, and speed. It was
developed by Mozilla and is known for its zero-cost abstractions and unique ownership model,
which helps prevent bugs at compile time.
```

**Result:** Works well. Concise and accurate.

### 2. Tool-using Agent

```
$ cargo run -- -m qwen2.5:7b-instruct tools "What is 42 * 17?"
```

**Output:**
```
Using tool-equipped agent...

The result of 42 * 17 is 714.
```

**Result:** Works perfectly! The model correctly called the calculator tool, processed the
result, and returned a natural language answer. Major improvement over llama3.2:3b.

### 3. RAG Demo

```
$ cargo run -- -m qwen2.5:7b-instruct rag "What does flurbo mean?"
```

**Output:**
```
Building embeddings...
The word "flurbo" is not defined in the provided contexts. Neither "doc0" nor "doc1" contain
any information about a word "flurbo".
```

**Result:** Same issue as llama3.2 -- the vector search retrieves the wrong documents.
This is an embedding model issue (nomic-embed-text), not a completion model issue.
The agent correctly reports it can't find flurbo in the retrieved context.

### 4. Structured Extraction

```
$ cargo run -- -m qwen2.5:7b-instruct extract "Please contact John Smith at john@example.com or call 555-867-5309"
```

**Output:**
```
{"name": "John Smith", "email": "john@example.com", "phone": "555-867-5309"}
```

**Result:** Works perfectly.

### 5. Multi-agent Orchestration

```
$ cargo run -- -m qwen2.5:7b-instruct multi "What is the weather in Tokyo?"
```

**Output:**
```
Using multi-agent orchestrator...

Error: CompletionError: ResponseError: No content provided
```

**Result:** Failed. The orchestrator attempted to call a sub-agent as a tool but the
nested agent-as-tool invocation produced no content. This may be a limitation of how
Ollama handles nested tool calls or how the agent-as-tool pattern works with models
that have different tool-calling conventions.

### Summary (qwen2.5:7b-instruct)

| Demo | Status | Notes |
|---|---|---|
| ask | Works | Clean, concise response |
| tools | Works | Full tool-call round-trip with natural language answer |
| rag | Partial | Embedding retrieval returns wrong docs (embedding model issue) |
| extract | Works | Perfect JSON extraction |
| multi | Failed | Nested agent-as-tool produces no content error |

### Comparison: llama3.2:3b vs qwen2.5:7b-instruct

| Demo | llama3.2:3b | qwen2.5:7b-instruct |
|---|---|---|
| ask | Works | Works |
| tools | Partial (raw JSON) | **Works** (natural language) |
| rag | Partial | Partial (same embedding issue) |
| extract | Works | Works |
| multi | Partial (raw JSON) | Failed (error) |

### Key Takeaways

1. **Tool calling is dramatically better** with the 7B instruction-tuned model. It completes
   the full round-trip: recognizes the tool, generates correct args, processes the result,
   and responds in natural language.

2. **RAG retrieval is model-independent.** The wrong-document retrieval is an embedding model
   issue (`nomic-embed-text`), not related to the completion model.

3. **Multi-agent is the hardest pattern.** Neither model handled the nested agent-as-tool
   pattern successfully. This likely needs either a larger model or adjustments to the
   agent-as-tool configuration.

---

## Trial 3: Improved RAG corpus (qwen2.5:7b-instruct)

Date: 2026-03-08
Model: `qwen2.5:7b-instruct` (7B), Embeddings: `nomic-embed-text`

Replaced made-up words with 6 semantically distinct real-world topics:
Rust ownership, photosynthesis, Kubernetes, sourdough bread, black holes, TCP/IP networking.

### 3a. Rust ownership query

```
$ cargo run -- -m qwen2.5:7b-instruct rag "How does Rust manage memory?"
```

**Output:**
```
Building embeddings...
Rust manages memory using an ownership system. Each value in Rust has exactly one owner, and
ownership can be transferred via moves or shared via borrowing with references. The borrow
checker enforces rules at compile time to prevent data races and use-after-free bugs, ensuring
that either one mutable reference or any number of immutable references exist at any given
time, but not both.
```

**Result:** Correct retrieval. Answer grounded in the provided context.

### 3b. Photosynthesis query

```
$ cargo run -- -m qwen2.5:7b-instruct rag "How do plants make food from sunlight?"
```

**Output:**
```
Building embeddings...
Plants make food from sunlight through the process of photosynthesis. During photosynthesis,
green plants and certain organisms convert light energy into chemical energy. They use sunlight,
water, and carbon dioxide to produce glucose and release oxygen as a byproduct. This process
takes place in chloroplasts, specifically utilizing chlorophyll pigments that absorb red and
blue light while reflecting green light. The light reactions occur in the thylakoid membranes,
while the Calvin cycle, which fixes carbon dioxide into organic molecules, takes place in the
stroma.
```

**Result:** Correct retrieval. Comprehensive answer from context.

### 3c. Kubernetes query

```
$ cargo run -- -m qwen2.5:7b-instruct rag "What is a pod in container orchestration?"
```

**Output:**
```
Building embeddings...
In container orchestration, particularly within Kubernetes, a pod is the smallest deployable
unit that can be created, managed, and scheduled by Kubernetes. Pods group containers for a
single application into the same address space, sharing the same storage resources and
networking.
```

**Result:** Correct retrieval. Accurate answer.

### 3d. Sourdough query

```
$ cargo run -- -m qwen2.5:7b-instruct rag "How do you make sourdough bread?"
```

**Output:**
```
Building embeddings...
Sourdough bread is made using a naturally fermented starter culture of wild yeast and lactic
acid bacteria instead of commercial yeast. The process involves maintaining a starter by
regularly feeding it with flour and water. The bread undergoes a long fermentation process,
typically 12-24 hours, which develops complex flavors and a tangy taste from lactic and acetic
acids. This method results in a lower glycemic index compared to regular bread and partially
breaks down the gluten, making it easier to digest for some people.
```

**Result:** Correct retrieval. Detailed answer.

### 3e. Black holes query

```
$ cargo run -- -m qwen2.5:7b-instruct rag "What happens at the event horizon?"
```

**Output:**
```
Building embeddings...
At the event horizon of a black hole, you would find the boundary of the region from which
nothing, not even light, can escape. Beyond this boundary lies the singularity, a point of
theoretically infinite density.
```

**Result:** Correct retrieval. Concise, accurate answer.

### Summary (improved RAG)

| Query | Retrieved correct docs? | Answer quality |
|---|---|---|
| Rust memory management | Yes | Excellent |
| Photosynthesis | Yes | Excellent |
| Kubernetes pods | Yes | Excellent |
| Sourdough bread | Yes | Excellent |
| Event horizon | Yes | Excellent |

**5/5 correct retrievals.** The original made-up-words corpus failed because short nonsense
words with similar sci-fi definitions all clustered together in embedding space. Switching to
semantically distinct real-world topics with longer, descriptive content gives `nomic-embed-text`
enough signal to differentiate and retrieve correctly.

---

## Trial 4: New features (qwen2.5:7b-instruct)

Date: 2026-03-08
Model: `qwen2.5:7b-instruct` (7B)

Added: datetime tool, string tool, typed extractor (schemars), streaming, multi-agent fix,
`--verbose` flag.

### 4a. DateTime tool

```
$ cargo run -- -m qwen2.5:7b-instruct tools "What time is it?"
```

**Output:**
```
Using tool-equipped agent...

The current local time is 13:01:32 on March 8, 2026.
```

**Result:** Works. Agent called `get_datetime` and formatted the response naturally.

### 4b. String tool

```
$ cargo run -- -m qwen2.5:7b-instruct tools "Convert 'hello world' to uppercase"
```

**Output:**
```
Using tool-equipped agent...

The text 'hello world' converted to uppercase is 'HELLO WORLD'.
```

**Result:** Works. Agent called `string_tool` with `uppercase` operation.

### 4c. Typed extractor (schemars)

```
$ cargo run -- -m qwen2.5:7b-instruct extract "Hi, I'm Jane Doe. Reach me at jane.doe@company.org or 415-555-0199"
```

**Output:**
```
{
  "name": "Jane Doe",
  "email": "jane.doe@company.org",
  "phone": "415-555-0199"
}
```

**Result:** Works. Now uses Rig's `Extractor` with `schemars::JsonSchema` for typed
extraction instead of prompt-based JSON. Returns pretty-printed `ContactInfo` struct.
Note: Ollama logs a warning about `tool_choice` not being supported (harmless).

### 4d. Multi-agent orchestration (fixed)

```
$ cargo run -- -m qwen2.5:7b-instruct multi "What is 100 divided by 7?"
```

**Output:**
```
Using multi-agent orchestrator...

The result of dividing 100 by 7 is approximately 14.285714285714286.
```

```
$ cargo run -- -m qwen2.5:7b-instruct multi "What is the weather in Seattle?"
```

**Output:**
```
Using multi-agent orchestrator...

The current weather in Seattle is rainy with a temperature of 48°F and humidity at 85%.
Stay dry out there!
```

**Result:** Both work! Fixed by adding `.name()` and `.description()` to sub-agents so the
orchestrator has clear tool definitions to work with.

### 4e. Streaming response

```
$ cargo run -- -m qwen2.5:7b-instruct stream "Explain TCP in two sentences"
```

**Output:**
```
TCP stands for Transmission Control Protocol and ensures reliable data transmission over
a network by providing error-checking and flow control.
```

**Result:** Works. Text streams token-by-token to stdout. Also supports `stream-chat`
for interactive streaming chat with history.

### 4f. Verbose mode

```
$ cargo run -- -v -m qwen2.5:7b-instruct tools "What is 5 + 3?"
```

**Output (key lines):**
```
execute_tool{gen_ai.tool.name="calculator" gen_ai.tool.call.arguments="{\"operation\":\"add\",\"x\":5,\"y\":3}"}:
  rig: Calling tool calculator with args: "{\"operation\":\"add\",\"x\":5,\"y\":3}"
  rig::agent::prompt_request: executed tool calculator with args {"operation":"add","x":5,"y":3}. result: 8.0
The result of 5 + 3 is 8.
```

**Result:** Works. The `-v` flag enables DEBUG-level tracing, showing tool names, arguments,
and results. Rig's built-in OpenTelemetry-style spans provide the detail automatically.

### Summary (Trial 4)

| Feature | Status | Notes |
|---|---|---|
| DateTime tool | Works | Returns current date/time |
| String tool | Works | uppercase, lowercase, reverse, count_words, replace, trim |
| Typed extractor | Works | schemars-based `ContactInfo` extraction |
| Multi-agent (math) | Works | Fixed with `.name()` and `.description()` on sub-agents |
| Multi-agent (weather) | Works | Full nested tool-call round-trip |
| Streaming | Works | Token-by-token output + streaming chat |
| Verbose mode | Works | Shows tool calls, args, and results via tracing |

**All 7 features working.** The multi-agent fix was the key breakthrough -- sub-agents need
explicit `.name()` and `.description()` so the orchestrator model knows what each tool does
and how to call it.
