# Wispr Flow Clone - Implementation Plan

## Goal
Build an open-source Wispr Flow alternative using:
- Moonshine Tiny/Base for STT (MIT license, beats Whisper)
- Qwen 3.5 for text cleanup/formatting
- Kokoro for TTS (optional)
- All running locally, no cloud

## Architecture

```
Audio Input → Moonshine STT → Qwen 3.5 (cleanup) → Output Text
                                    ↓
                            (optional: Kokoro TTS)
```

## Implementation Tasks

### ✅ Completed
- [x] CLI interface with metrics
- [x] LLM integration (Qwen 3.5 0.8B)
- [x] Audio file loading (MP3/WAV)
- [x] System info display
- [x] Performance metrics

### 🔄 In Progress
- [ ] Download Moonshine Tiny ONNX model
- [ ] Implement Moonshine STT with ort crate
- [ ] Test with audio.mp3

### 📋 Pending
- [ ] Real-time audio capture
- [ ] Filler word removal (using Qwen)
- [ ] Context-aware formatting
- [ ] Voice commands ("make this a list", etc.)
- [ ] Hotkey activation
- [ ] System-wide text injection
- [ ] TTS with Kokoro (optional)

## Current Status
Setting up Moonshine ONNX model for STT transcription.
