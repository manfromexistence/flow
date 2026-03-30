# CLI Mode for AI Agents

This project is designed for AI agents to test local GGUF models. CLI mode is the default.

## Quick Start for AI Agents

```bash
# Build the project
cargo build

# Single query (recommended for testing)
cargo run "What is Rust?"

# Interactive mode
cargo run

# Pipe input
echo "Explain async programming" | cargo run

# Show help
cargo run -- --help
```

## Usage Modes

### 1. Single Query Mode (Default for AI Agents)
```bash
cargo run "Your question here"
```
The model initializes, processes your query, outputs clean response, and exits.

### 2. Interactive Mode
```bash
cargo run
```
Starts an interactive session. Type `exit` or `quit` to end.

### 3. Piped Input Mode
```bash
echo "Your question" | cargo run
```
Reads from stdin and processes the input.

### 4. TUI Mode (Human Use Only)
```bash
cargo run -- --tui
```
Launches the full terminal UI interface.

## Model Configuration

- Model: `Qwen3.5-0.8B-Q4_K_M.gguf`
- Location: `models/llm/`
- Context: 32K tokens
- Temperature: 0.7
- RAM Required: ~3.4GB

## AI Agent Testing Examples

```bash
# Test basic functionality
cargo run "Hello, respond in one sentence"

# Test code generation
cargo run "Write a Rust function to calculate fibonacci"

# Test reasoning
cargo run "Explain the difference between async and sync"

# Test with timeout (for CI/CD)
timeout 30 cargo run "Quick test response"
```

## Output Format

Responses are cleaned automatically:
- `<think>` tags removed
- Extra whitespace trimmed
- Clean, parseable output

## Exit Codes

- `0`: Success
- `1`: Initialization failure or error

## Performance

- Model load: ~3-5 seconds
- Inference: ~20-40 tokens/second (CPU)
- Memory: ~3.4GB RAM usage
