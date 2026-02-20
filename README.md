## Simple Rust AI Agent

A Rust-based agent that uses Qdrant as a vector store for knowledge retrieval, with a small set of tools for interacting with the environment. The agent follows a RAG pattern: on each task, it queries the vector store for relevant context, selects one or more 
tools, executes them, and synthesizes a response.

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (2024 edition)
- An API key for Gemini OR a local [Ollama](https://ollama.com/) instance.

### Configuration

Create a `.env` file in the root directory:

```env
PROVIDER=gemini # or "ollama"
AGENT_MODEL=gemini-2.0-flash
TEMPERATURE=0.3
GEMINI_API_KEY=your_key_here
OLLAMA_API_BASE_URL=http://localhost:11434 # only needed for Ollama
```

### Running the Agent

```bash
cargo run
```

Type `exit` or `quit` to stop the session.

### Output Example
```text
Agent initialized. Type 'exit' or 'quit' to stop.

You: Hello
Agent: Hello! 😊 How can I assist you today?
---
You: What are you?
Agent: Hi, I'm **Qwen**, a large language model developed by Tongyi Lab. I'm here to help with a wide range of tasks, including answering questions, writing stories, emails, scripts, performing logical reasoning, coding, and more. I can also express opinions, play games, and even translate languages. Whether you need assistance with learning, work, or just want to chat, I'm happy to help! 😊  
What would you like to do today?
---
You: What is the capital of France?
Agent (thinking...)
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.