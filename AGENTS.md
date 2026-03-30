# Wispr Flow Clone - AI Agent Instructions

> **Target Agent**: OpenAI Codex GPT-5.4  
> **Project Type**: Rust CLI Application - Voice-to-Text AI Assistant  
> **Status**: Foundation Complete, Moonshine STT Integration Needed  
> **Last Updated**: March 31, 2026

---

## Project Overview

This is a Wispr Flow clone - an AI-powered voice assistant that converts speech to text with automatic enhancement. The project combines:

- **Speech-to-Text (STT)**: Moonshine Tiny ONNX models for transcription
- **LLM Enhancement**: Qwen 3.5 0.8B for text cleanup and formatting
- **Text-to-Speech (TTS)**: Kokoro v1.0 INT8 (planned)

**Current State**: CLI interface complete, audio processing working, mock transcription functional. Real Moonshine ONNX integration is the next critical step.

---

## Your Mission

Implement the complete Moonshine STT integration to replace the mock transcription system. This involves:

1. **Download Moonshine ONNX Models** (if not present)
   - Run: `powershell -ExecutionPolicy Bypass -File scripts/download_moonshine_onnx.ps1`
   - Models: `moonshine-tiny-encoder.onnx` and `moonshine-tiny-decoder.onnx`
   - Location: `models/stt/`

2. **Implement ONNX Runtime Integration**
   - Add `ort` crate back to `Cargo.toml` with proper configuration
   - Fix threading issues (use single-threaded or proper async handling)
   - Implement encoder-decoder inference pipeline
   - Process audio features (mel spectrograms) for encoder input
   - Decode token sequences from decoder output

3. **Audio Preprocessing Pipeline**
   - Convert audio to 16kHz mono (already implemented in `AudioAnalyzer`)
   - Generate mel spectrogram features for Moonshine encoder
   - Normalize audio features to expected input range

4. **Text Post-Processing (Wispr Flow Style)**
   - Remove filler words: "um", "uh", "like", "you know"
   - Add proper punctuation
   - Capitalize sentences
   - Format output cleanly

5. **Testing & Validation**
   - Test with `audio.mp3` (contains: "hello mike testing one two three hello")
   - Verify transcription accuracy
   - Measure performance metrics (speed, RAM usage)
   - Ensure CLI commands work: `--transcribe` and `--wispr`

---

## Technical Constraints

### Must Follow
- **Rust Edition 2024** (already configured)
- **No TUI code** - CLI only, all TUI code is in `trash/`
- **Minimal dependencies** - Only add what's absolutely necessary
- **Windows compatibility** - Primary target platform
- **Release optimization** - LTO, strip symbols, single codegen unit

### Architecture
```
src/
├── main.rs          # CLI interface, command routing
├── llm.rs           # Qwen 3.5 LLM integration (working)
├── stt.rs           # Moonshine STT (needs ONNX implementation)
└── voice.rs         # Voice pipeline utilities

models/
├── llm/             # Qwen GGUF models (working)
├── stt/             # Moonshine ONNX models (download needed)
└── tts/             # Kokoro TTS models (future)

scripts/             # PowerShell download scripts
trash/               # Old TUI code (ignore)
```

### Key Files to Modify
- `src/stt.rs` - Replace `MoonshineSTT::transcribe()` mock with real ONNX inference
- `Cargo.toml` - Add `ort` crate with correct features
- Test with: `cargo run -- --transcribe audio.mp3`

---

## Implementation Guide

### Step 1: Add ONNX Runtime Dependency
```toml
# Add to Cargo.toml [dependencies]
ort = { version = "2.0.0-rc.12", features = ["download-binaries"] }
```

### Step 2: Implement Moonshine Inference
The `MoonshineSTT` struct in `src/stt.rs` needs:
- Load encoder and decoder ONNX models
- Preprocess audio to mel spectrogram features
- Run encoder inference on audio features
- Run decoder inference with encoder outputs
- Convert token IDs to text using tokenizer
- Apply Wispr Flow text enhancements

### Step 3: Handle Threading Issues
ONNX Runtime has threading constraints. Solutions:
- Use `Session::builder()?.with_intra_threads(1)?` for single-threaded
- Or wrap in `tokio::task::spawn_blocking()` for async compatibility
- Ensure `Send + Sync` bounds are satisfied

### Step 4: Audio Feature Extraction
Moonshine expects mel spectrogram features:
- Sample rate: 16kHz (already handled)
- Window size: Typically 25ms (400 samples at 16kHz)
- Hop length: Typically 10ms (160 samples at 16kHz)
- Mel bins: Check model config (likely 80)

### Step 5: Test & Iterate
```bash
# Test transcription only
cargo run -- --transcribe audio.mp3

# Test full Wispr Flow pipeline (STT + LLM)
cargo run -- --wispr audio.mp3

# Expected output for audio.mp3
# "Hello Mike, testing one two three, hello."
```

---

## Success Criteria

✅ **Must Have**:
- Real Moonshine ONNX transcription working (not mock)
- Accurate transcription of `audio.mp3`
- No compilation errors or warnings
- CLI commands functional: `--transcribe`, `--wispr`
- Performance metrics displayed (tokens/sec, RAM usage)

✅ **Nice to Have**:
- Streaming transcription support
- Voice activity detection (VAD)
- Multiple audio format support (MP3, WAV, FLAC)
- Real-time microphone input

---

## Resources

### Documentation
- [Moonshine Model Card](https://huggingface.co/UsefulSensors/moonshine-tiny)
- [ONNX Runtime Rust Docs](https://docs.rs/ort/latest/ort/)
- [Wispr Flow Features](https://wisprflow.ai) - Reference implementation

### Model Files
- Encoder: `models/stt/moonshine-tiny-encoder.onnx` (~13MB)
- Decoder: `models/stt/moonshine-tiny-decoder.onnx` (~14MB)
- Config: `models/stt/moonshine-tiny-config.json`
- Tokenizer: `models/stt/moonshine-tiny-tokenizer.json`

### Test Audio
- File: `audio.mp3` (3.14s, 48kHz mono)
- Content: "hello mike testing one two three hello"
- Expected: Clean transcription with punctuation

---

## Common Pitfalls to Avoid

❌ **Don't**:
- Add back TUI dependencies (ratatui, crossterm, tachyonfx)
- Modify `README.md` (user requested to keep as-is)
- Create unnecessary documentation files
- Use complex async patterns where sync works
- Ignore the existing audio preprocessing in `AudioAnalyzer`

✅ **Do**:
- Use existing `AudioAnalyzer::load_audio()` for audio loading
- Keep error messages clear and actionable
- Add helpful CLI output showing progress
- Test with the provided `audio.mp3` file
- Follow Rust 2024 edition best practices

---

## Quick Start Command

After reading this file, start with:

```bash
# 1. Download models (if needed)
powershell -ExecutionPolicy Bypass -File scripts/download_moonshine_onnx.ps1

# 2. Implement ONNX integration in src/stt.rs
# 3. Test
cargo run -- --transcribe audio.mp3
```

---

## Notes for GPT-5.4 Codex

- This project uses **Rust edition 2024** (latest as of March 2026)
- The LLM integration (Qwen 3.5) is already working perfectly
- Audio loading and preprocessing is complete
- Only the ONNX inference pipeline needs implementation
- Focus on `src/stt.rs` - the `MoonshineSTT::transcribe()` method
- Use the existing `AudioAnalyzer` utilities for audio processing
- The mock transcription shows the expected interface - replace it with real inference

**Priority**: Get Moonshine ONNX working first. Everything else is secondary.

---

**End of Agent Instructions**
