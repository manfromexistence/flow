# Flow API Documentation

## Library Usage

Flow can be used as both a CLI tool and a library.

### As a Library

Add to your `Cargo.toml`:

```toml
[dependencies]
flow = "0.1.0"
```

### Basic Usage

```rust
use flow::{MoonshineSTT, LocalLlm, KokoroTTS};
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Speech-to-Text
    let stt = MoonshineSTT::new()?;
    let text = stt.transcribe("audio.mp3")?;
    
    // Text Enhancement
    let llm = LocalLlm::new(None)?;
    let enhanced = llm.generate(&format!("Clean: {}", text)).await?;
    
    // Text-to-Speech
    let tts = KokoroTTS::new()?;
    let audio = tts.synthesize(&enhanced)?;
    
    Ok(())
}
```

## Modules

### `flow::audio`
Audio processing utilities.

- `AudioLoader::load(path)` - Load audio file
- `compute_mel_spectrogram(audio, config)` - Compute features

### `flow::models`
ML model inference.

- `MoonshineSTT::new()` - Create STT engine
- `LocalLlm::new(model_path)` - Create LLM engine
- `KokoroTTS::new()` - Create TTS engine

### `flow::pipeline`
Processing pipelines.

- `VoicePipeline::new()` - Create voice pipeline

### `flow::utils`
Utility functions.

- `get_memory_info()` - Get system memory
- `check_memory_requirements(mb)` - Check if enough RAM

## CLI Usage

```bash
# Transcribe audio
flow --transcribe audio.mp3

# Full pipeline
flow --wispr audio.mp3

# Text-to-speech
flow --speak "Hello world"
```

## Configuration

Models are loaded from:
- `models/llm/` - LLM models (GGUF)
- `models/stt/` - STT models (ONNX)
- `models/tts/` - TTS models (ONNX)

## Error Handling

All functions return `Result<T, anyhow::Error>` for comprehensive error handling.

```rust
match stt.transcribe("audio.mp3") {
    Ok(text) => println!("Success: {}", text),
    Err(e) => eprintln!("Error: {}", e),
}
```
