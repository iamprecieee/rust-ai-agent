Simple Rust AI Agent

A Rust-based agent that uses Qdrant as a vector store for knowledge retrieval, with a small set of tools for interacting with the environment. The agent follows a RAG pattern: on each task, it queries the vector store for relevant context, selects one or more tools, executes them, and synthesizes a response.