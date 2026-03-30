# CLI Mode for AI Agents

This project is designed for AI agents to test local GGUF models with detailed performance metrics.

## Quick Start for AI Agents

```bash
# Build the project
cargo build

# Single query with full metrics
cargo run "What is Rust?"

# Interactive mode
cargo run

# Show help
cargo run -- --help
```

## Output Format

Each query displays:

1. **System Information** (on startup)
   - CPU model and core count
   - Total/Used/Available RAM
   
2. **Model Initialization**
   - Load time in seconds

3. **Response** (cleaned, no `<think>` tags)

4. **Performance Metrics**
   - Prompt tokens
   - Generated tokens
   - Total tokens
   - Prompt evaluation time
   - Generation time
   - Total time
   - Generation speed (tokens/sec)
   - Estimated RAM usage

## Example Output

```
═══════════════════════════════════════════════════════════
  SYSTEM INFORMATION
═══════════════════════════════════════════════════════════
CPU: AMD Ryzen 5 5600G with Radeon Graphics
Physical Cores: 6
Logical Cores: 12
Total RAM: 7.30 GB
Used RAM: 5.94 GB (81.4%)
Available RAM: 1.36 GB
═══════════════════════════════════════════════════════════

Initializing GGUF model... ✓ (4.69s)

> Hi

What?

───────────────────────────────────────────────────────────
  PERFORMANCE METRICS
───────────────────────────────────────────────────────────
Prompt Tokens: 207
Generated Tokens: 6
Total Tokens: 213

Prompt Eval Time: 2.17s
Generation Time: 0.71s
Total Time: 4.33s

Generation Speed: 8.44 tokens/sec
Estimated RAM Usage: 3.40 GB
───────────────────────────────────────────────────────────
```

## AI Agent Testing Examples

```bash
# Test basic functionality
cargo run "Hello"

# Test code generation
cargo run "Write a Rust function to calculate fibonacci"

# Test reasoning
cargo run "Explain async in one sentence"

# Benchmark performance
cargo run "Count to 10"
```

## Model Configuration

- Model: `Qwen3.5-0.8B-Q4_K_M.gguf`
- Location: `models/llm/`
- Context: 32K tokens
- Temperature: 0.7
- Base RAM: ~3.4GB

## Metrics Explained

- **Prompt Tokens**: Input size (includes system prompt)
- **Generated Tokens**: Output size
- **Prompt Eval Time**: Time to process input
- **Generation Time**: Time to generate output
- **Tokens/sec**: Generation speed (higher is better)
- **Estimated RAM**: Model + context memory usage

## Exit Codes

- `0`: Success
- `1`: Initialization failure or error
