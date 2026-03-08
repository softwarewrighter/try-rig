Output docs/demo.gif
Set Theme "Catppuccin Mocha"
Set FontSize 16
Set Width 900
Set Height 600
Set Padding 20

Type "# try-rig: Exploring Rig with Ollama"
Enter
Sleep 1s

Type "# 1. Simple ask"
Enter
Sleep 500ms
Type `cargo run -- -m qwen2.5:7b-instruct ask "What is Rust in one sentence?"`
Enter
Sleep 8s

Type "# 2. Tool-calling agent (calculator)"
Enter
Sleep 500ms
Type `cargo run -- -m qwen2.5:7b-instruct tools "What is 42 * 17?"`
Enter
Sleep 10s

Type "# 3. Tool-calling agent (weather)"
Enter
Sleep 500ms
Type `cargo run -- -m qwen2.5:7b-instruct tools "What is the weather in Tokyo?"`
Enter
Sleep 10s

Type "# 4. RAG with knowledge base"
Enter
Sleep 500ms
Type `cargo run -- -m qwen2.5:7b-instruct rag "How does Rust manage memory?"`
Enter
Sleep 12s

Type "# 5. Structured extraction"
Enter
Sleep 500ms
Type `cargo run -- -m qwen2.5:7b-instruct extract "Email jane@co.org or call 415-555-0199"`
Enter
Sleep 10s

Type "# 6. Multi-agent orchestration"
Enter
Sleep 500ms
Type `cargo run -- -m qwen2.5:7b-instruct multi "What is 100 / 7?"`
Enter
Sleep 12s

Sleep 3s
