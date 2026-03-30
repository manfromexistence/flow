# Wispr Flow Clone - Complete Implementation Guide

> **Target Agent**: OpenAI Codex GPT-5.4  
> **Project**: Rust CLI Voice Assistant (Wispr Flow Killer)  
> **Status**: Foundation Complete, STT/TTS Integration Needed  
> **Date**: March 31, 2026

---

## 🎯 Mission: Build the Ultimate Open-Source Voice Assistant

Create a production-ready Wispr Flow alternative using the latest March 2026 models:
- **STT**: Moonshine v2 (beats Whisper Large v3 with 6x fewer params)
- **LLM**: Qwen 3.5 Small Series (beats models 3x its size)
- **TTS**: Kokoro v1.0 (#1 on TTS Arena) + Voxtral TTS (beats ElevenLabs)

**Current State**: CLI working, LLM integrated (Qwen 3.5 0.8B), audio processing complete, Moonshine ONNX models downloaded. Need: Real ONNX inference implementation.

---

## 📋 Implementation Checklist

### Phase 1: Moonshine STT Integration (PRIORITY)
- [ ] Add `ort` crate to Cargo.toml with proper threading config
- [ ] Implement mel spectrogram feature extraction (80 mel bins, 16kHz)
- [ ] Load encoder/decoder ONNX models in `src/stt.rs`
- [ ] Implement encoder inference (audio features → hidden states)
- [ ] Implement decoder inference (hidden states → token IDs)
- [ ] Parse tokenizer.json and implement token-to-text decoding
- [ ] Test with audio.mp3: "hello mike testing one two three hello"
- [ ] Verify WER < 10% on test audio

### Phase 2: Text Enhancement (Wispr Flow Style)
- [ ] Remove filler words: "um", "uh", "like", "you know", "sort of", "kind of"
- [ ] Add proper punctuation using simple rules or LLM
- [ ] Capitalize sentences correctly
- [ ] Format output cleanly (trim whitespace, fix spacing)
- [ ] Test `--wispr` command (STT + LLM enhancement pipeline)

### Phase 3: TTS Integration (Optional but Recommended)
- [ ] Implement Kokoro v1.0 INT8 inference (already downloaded)
- [ ] Add `--speak` command to read back enhanced text
- [ ] Test voice output quality
- [ ] Add Voxtral TTS as premium option (if 16GB+ RAM available)

### Phase 4: Production Features
- [ ] Add real-time microphone input support
- [ ] Implement streaming transcription (Moonshine supports this!)
- [ ] Add voice activity detection (VAD)
- [ ] Support multiple audio formats (MP3, WAV, FLAC, OGG)
- [ ] Add hotkey activation for system-wide use
- [ ] Implement clipboard injection for transcribed text

---

## 🏗️ Technical Architecture

### Current Project Structure
```
wispr-flow/
├── AGENTS.md              ← This file (your guide)
├── README.md              ← User documentation (don't modify)
├── Cargo.toml             ← Rust dependencies
├── audio.mp3              ← Test file (3.14s, "hello mike...")
│
├── src/
│   ├── main.rs           ← CLI interface (working)
│   ├── llm.rs            ← Qwen 3.5 LLM (working)
│   ├── stt.rs            ← Moonshine STT (needs ONNX impl)
│   └── voice.rs          ← Voice pipeline utilities
│
├── models/
│   ├── llm/              ← Qwen GGUF models (working)
│   │   ├── Qwen3.5-0.8B-Q4_K_M.gguf
│   │   ├── Qwen3.5-2B-Q4_K_M.gguf
│   │   └── Qwen3.5-4B-Q4_K_M.gguf (not downloaded yet)
│   ├── stt/              ← Moonshine ONNX (downloaded ✓)
│   │   ├── moonshine-tiny-encoder.onnx
│   │   ├── moonshine-tiny-decoder.onnx
│   │   ├── moonshine-tiny-config.json
│   │   └── moonshine-tiny-tokenizer.json
│   └── tts/              ← Kokoro TTS (downloaded ✓)
│       ├── kokoro-v1.0.int8.onnx
│       └── voices-v1.0.bin
│
└── scripts/              ← Download scripts
    ├── download_moonshine_onnx.ps1
    └── download_whisper.ps1
```

---

## 🔬 Latest Model Research (March 2026)

### STT: Moonshine v2 (Released Feb 13, 2026)
**Why Moonshine beats Whisper**:
- Moonshine Large (245M params): 6.65% WER
- Whisper Large v3 (1.55B params): 7.44% WER
- **6x smaller, MORE accurate!**

**Architecture advantages**:
- Variable-length input (no 30s padding waste)
- Streaming support with state caching
- 5x faster than Whisper in real-time
- C++ core library (like llama.cpp) - perfect for Rust FFI
- MIT License ✅

**Model sizes available**:
| Model | Params | Size | WER | Use Case |
|-------|--------|------|-----|----------|
| Nano  | 26M    | 26MB | 8.5% | IoT/Embedded |
| Tiny  | 35M    | 35MB | 7.2% | Mobile/Edge |
| Base  | 100M   | 100MB| 6.9% | Laptops (4-8GB) |
| Large | 245M   | 245MB| 6.65%| Workstations (8GB+) |

**Current implementation**: Using Tiny (35M) - downloaded and ready.

### LLM: Qwen 3.5 Small Series (Released March 2, 2026)
**Why Qwen 3.5 is perfect**:
- Hybrid architecture: Gated Delta Networks + sparse MoE
- 201 languages supported
- Apache 2.0 license ✅
- **Ollama can't run Qwen 3.5 GGUF** (vision file issues) - YOUR ADVANTAGE!

**Model comparison**:
| Model | Params | RAM | Performance | Use Case |
|-------|--------|-----|-------------|----------|
| 0.8B  | 0.8B   | ~1.5GB | Basic | Current (working) |
| 2B    | 2B     | ~2.5GB | Good | Phones/Budget |
| 4B    | 4B     | ~4.5GB | **Coding sweet spot** | Recommended |
| 9B    | 9B     | ~12GB | Beats Qwen3-30B | High-end |

**Current implementation**: Using 0.8B - works perfectly. Upgrade to 4B recommended.

### TTS: Kokoro v1.0 + Voxtral TTS
**Kokoro v1.0** (Current choice):
- 82M params, ~80MB INT8
- #1 on TTS Arena (44% win rate)
- Apache 2.0 license ✅
- Runs on Raspberry Pi
- Already downloaded ✓

**Voxtral TTS** (Released March 26, 2026 - 5 days ago!):
- 4B params, ~3GB quantized
- **Beats ElevenLabs in blind tests** (68.4% win rate)
- 70ms time-to-first-audio
- 9.7x real-time factor
- CC BY-NC 4.0 (non-commercial) - OK for free CLI
- Requires 16GB+ RAM

---

## 🛠️ Implementation Details

### Step 1: Add Dependencies to Cargo.toml

```toml
[dependencies]
# Existing (keep these)
tokio = { version = "1.42", features = ["rt-multi-thread", "macros", "sync"] }
llama-cpp-2 = "0.1"
anyhow = "1.0"
sysinfo = "0.32"
hound = "3.5.1"
rodio = "0.22.2"
cpal = "0.17.3"
tiktoken-rs = "0.9"
atty = "0.2.14"

# NEW: Add these for ONNX + audio processing
ort = { version = "2.0.0-rc.12", features = ["download-binaries"] }
ndarray = "0.17"  # For tensor operations
rustfft = "6.2"   # For FFT (mel spectrogram)
```

### Step 2: Implement Mel Spectrogram Feature Extraction

Create `src/audio_features.rs`:

```rust
use rustfft::{FftPlanner, num_complex::Complex};
use ndarray::{Array1, Array2};

pub struct MelSpectrogramConfig {
    pub sample_rate: usize,
    pub n_fft: usize,
    pub hop_length: usize,
    pub n_mels: usize,
}

impl Default for MelSpectrogramConfig {
    fn default() -> Self {
        Self {
            sample_rate: 16000,
            n_fft: 400,      // 25ms window at 16kHz
            hop_length: 160, // 10ms hop
            n_mels: 80,      // Standard for speech
        }
    }
}

pub fn compute_mel_spectrogram(
    audio: &[f32],
    config: &MelSpectrogramConfig,
) -> Array2<f32> {
    // 1. Apply Hann window
    // 2. Compute STFT using rustfft
    // 3. Convert to mel scale
    // 4. Apply log scaling
    // 5. Normalize to [-1, 1] range
    
    // TODO: Implement this
    // Return shape: [n_mels, time_steps]
}
```

### Step 3: Implement ONNX Inference in src/stt.rs

Replace the mock `MoonshineSTT` implementation:

```rust
use ort::{Session, Value, inputs};
use ndarray::{Array2, Array3};

pub struct MoonshineSTT {
    encoder: Session,
    decoder: Session,
    tokenizer: Tokenizer, // Parse from moonshine-tiny-tokenizer.json
}

impl MoonshineSTT {
    pub fn new() -> Result<Self> {
        // Load models
        let encoder = Session::builder()?
            .with_intra_threads(1)? // Single-threaded to avoid issues
            .commit_from_file("models/stt/moonshine-tiny-encoder.onnx")?;
        
        let decoder = Session::builder()?
            .with_intra_threads(1)?
            .commit_from_file("models/stt/moonshine-tiny-decoder.onnx")?;
        
        let tokenizer = Tokenizer::from_file(
            "models/stt/moonshine-tiny-tokenizer.json"
        )?;
        
        Ok(Self { encoder, decoder, tokenizer })
    }
    
    pub fn transcribe(&self, audio_path: &str) -> Result<String> {
        // 1. Load audio (already implemented in AudioAnalyzer)
        let audio = AudioAnalyzer::load_audio(audio_path)?;
        
        // 2. Compute mel spectrogram features
        let features = compute_mel_spectrogram(&audio, &Default::default());
        
        // 3. Run encoder
        let encoder_input = Array3::from_shape_vec(
            (1, features.nrows(), features.ncols()),
            features.into_raw_vec()
        )?;
        
        let encoder_outputs = self.encoder.run(inputs![encoder_input]?)?;
        let hidden_states = encoder_outputs[0].try_extract_tensor::<f32>()?;
        
        // 4. Run decoder (autoregressive)
        let mut tokens = vec![1]; // BOS token
        let max_length = 448; // From config
        
        for _ in 0..max_length {
            let decoder_input = Array2::from_shape_vec(
                (1, tokens.len()),
                tokens.iter().map(|&t| t as i64).collect()
            )?;
            
            let decoder_outputs = self.decoder.run(inputs![
                decoder_input,
                hidden_states.view()
            ]?)?;
            
            let logits = decoder_outputs[0].try_extract_tensor::<f32>()?;
            let next_token = argmax(&logits);
            
            if next_token == 2 { break; } // EOS token
            tokens.push(next_token);
        }
        
        // 5. Decode tokens to text
        let text = self.tokenizer.decode(&tokens[1..], true)?;
        
        // 6. Apply Wispr Flow enhancements
        Ok(Self::enhance_transcript(&text))
    }
    
    fn enhance_transcript(text: &str) -> String {
        // Remove filler words
        let fillers = ["um", "uh", "like", "you know", "sort of", "kind of"];
        let mut enhanced = text.to_lowercase();
        
        for filler in &fillers {
            enhanced = enhanced.replace(filler, "");
        }
        
        // Clean up spacing
        enhanced = enhanced.split_whitespace().collect::<Vec<_>>().join(" ");
        
        // Add punctuation (basic rules)
        if !enhanced.is_empty() && !enhanced.ends_with('.') {
            enhanced.push('.');
        }
        
        // Capitalize first letter
        if let Some(first) = enhanced.chars().next() {
            enhanced = first.to_uppercase().collect::<String>() + &enhanced[1..];
        }
        
        enhanced
    }
}
```

### Step 4: Parse Tokenizer JSON

Create `src/tokenizer.rs`:

```rust
use serde_json::Value;
use std::collections::HashMap;

pub struct Tokenizer {
    vocab: HashMap<u32, String>,
}

impl Tokenizer {
    pub fn from_file(path: &str) -> Result<Self> {
        let json: Value = serde_json::from_str(&std::fs::read_to_string(path)?)?;
        
        // Parse vocab from tokenizer.json
        let vocab = json["model"]["vocab"]
            .as_object()
            .unwrap()
            .iter()
            .map(|(k, v)| (v.as_u64().unwrap() as u32, k.clone()))
            .collect();
        
        Ok(Self { vocab })
    }
    
    pub fn decode(&self, tokens: &[u32], skip_special: bool) -> Result<String> {
        let text = tokens
            .iter()
            .filter_map(|&t| {
                if skip_special && (t == 0 || t == 1 || t == 2) {
                    None
                } else {
                    self.vocab.get(&t).cloned()
                }
            })
            .collect::<Vec<_>>()
            .join("");
        
        Ok(text)
    }
}
```

### Step 5: Test Commands

```bash
# Test STT only
cargo run -- --transcribe audio.mp3

# Expected output:
# "Hello mike testing one two three hello."

# Test full Wispr Flow pipeline (STT + LLM)
cargo run -- --wispr audio.mp3

# Expected output:
# Original: "hello mike testing one two three hello"
# Enhanced: "Hello Mike, testing: one, two, three. Hello."
```

---

## 🎯 Success Criteria

### Must Have (Phase 1)
- ✅ Real Moonshine ONNX transcription (not mock)
- ✅ Accurate transcription of audio.mp3 (WER < 10%)
- ✅ No compilation errors or warnings
- ✅ `--transcribe` command works
- ✅ `--wispr` command works (STT + LLM)
- ✅ Performance metrics displayed

### Should Have (Phase 2)
- ✅ Filler word removal working
- ✅ Proper punctuation added
- ✅ Clean formatting
- ✅ Fast inference (< 1s for 3s audio)

### Nice to Have (Phase 3+)
- ⭐ Kokoro TTS integration
- ⭐ Real-time microphone input
- ⭐ Streaming transcription
- ⭐ Voice activity detection
- ⭐ Hotkey activation
- ⭐ System-wide clipboard injection

---

## 🚨 Common Pitfalls & Solutions

### Issue 1: ONNX Threading Errors
**Problem**: `NonNull<OrtSessionOptions>: Send` not satisfied  
**Solution**: Use `.with_intra_threads(1)?` for single-threaded execution

### Issue 2: Tensor Shape Mismatches
**Problem**: Input shape doesn't match model expectations  
**Solution**: Check `moonshine-tiny-config.json` for exact input shapes

### Issue 3: Tokenizer Decoding Issues
**Problem**: Garbled output or missing words  
**Solution**: Ensure proper BPE decoding, handle special tokens correctly

### Issue 4: Audio Feature Extraction
**Problem**: Mel spectrogram doesn't match expected format  
**Solution**: Use 80 mel bins, 16kHz sample rate, normalize to [-1, 1]

### Issue 5: Memory Usage
**Problem**: High RAM usage during inference  
**Solution**: Process audio in chunks, clear intermediate tensors

---

## 📚 Resources & References

### Official Documentation
- [Moonshine v2 Model Card](https://huggingface.co/UsefulSensors/moonshine-tiny)
- [Moonshine GitHub](https://github.com/usefulsensors/moonshine)
- [ONNX Runtime Rust](https://docs.rs/ort/latest/ort/)
- [Qwen 3.5 Release](https://qwenlm.github.io/blog/qwen3.5/)
- [Kokoro TTS](https://huggingface.co/hexgrad/Kokoro-82M)
- [Voxtral TTS](https://mistral.ai/news/voxtral/)

### Model Files (Already Downloaded)
```
models/stt/
├── moonshine-tiny-encoder.onnx    (~13MB)
├── moonshine-tiny-decoder.onnx    (~14MB)
├── moonshine-tiny-config.json     (model config)
└── moonshine-tiny-tokenizer.json  (BPE tokenizer)

models/llm/
├── Qwen3.5-0.8B-Q4_K_M.gguf      (~500MB) ✓ Working
├── Qwen3.5-2B-Q4_K_M.gguf        (~1.2GB) ✓ Available
└── Qwen3.5-4B-Q4_K_M.gguf        (~2.5GB) - Recommended upgrade

models/tts/
├── kokoro-v1.0.int8.onnx         (~80MB) ✓ Ready
└── voices-v1.0.bin               (voice data)
```

### Test Audio
- **File**: `audio.mp3`
- **Duration**: 3.14 seconds
- **Format**: 48kHz mono MP3
- **Content**: "hello mike testing one two three hello"
- **Expected WER**: < 10% (should get most words correct)

---

## 🎓 Implementation Strategy

### Recommended Order
1. **Start simple**: Get basic ONNX inference working (even if output is wrong)
2. **Fix audio features**: Implement proper mel spectrogram extraction
3. **Debug tokenizer**: Ensure token-to-text decoding works
4. **Optimize**: Add batching, caching, streaming
5. **Polish**: Add TTS, real-time input, hotkeys

### Testing Approach
1. Test encoder output shape matches decoder input
2. Test decoder with dummy inputs first
3. Test tokenizer decoding with known token sequences
4. Test full pipeline with audio.mp3
5. Test with various audio files (different lengths, quality)

### Performance Targets
- **Latency**: < 1 second for 3-second audio
- **Memory**: < 500MB RAM for Tiny model
- **Accuracy**: WER < 10% on clean speech
- **Real-time factor**: > 3x (process 3s audio in < 1s)

---

## 🔥 Competitive Advantages

### vs Wispr Flow (Commercial)
- ✅ **Free & unlimited** (no 2000 word/week limit)
- ✅ **Fully offline** (no internet required)
- ✅ **Open source** (Apache 2.0 / MIT)
- ✅ **Better models** (Moonshine v2 > Whisper, Qwen 3.5 > GPT-3.5)
- ✅ **Lower latency** (no API calls)
- ✅ **Privacy** (data never leaves device)

### vs Other Open-Source Solutions
- ✅ **Latest models** (March 2026 SOTA)
- ✅ **Rust performance** (faster than Python)
- ✅ **Minimal dependencies** (no PyTorch, TensorFlow)
- ✅ **Cross-platform** (Windows, Mac, Linux)
- ✅ **Low resource** (runs on 2GB RAM)

---

## 🚀 Quick Start for Codex

```bash
# 1. Verify models are downloaded
ls models/stt/moonshine-tiny-*.onnx

# 2. Add dependencies to Cargo.toml
# (ort, ndarray, rustfft, serde_json)

# 3. Implement in this order:
# - src/audio_features.rs (mel spectrogram)
# - src/tokenizer.rs (BPE decoder)
# - src/stt.rs (ONNX inference)

# 4. Test
cargo run -- --transcribe audio.mp3

# 5. Verify output
# Should see: "Hello mike testing one two three hello."
# (or similar with < 10% WER)
```

---

## 📝 Notes for GPT-5.4 Codex

- **Rust Edition 2024** (latest, use modern syntax)
- **No TUI code** (all in trash/, ignore it)
- **Don't modify README.md** (user request)
- **Focus on src/stt.rs** (main implementation file)
- **Use existing AudioAnalyzer** (audio loading already works)
- **Test incrementally** (don't implement everything at once)
- **Ask for help if stuck** (better to ask than guess wrong)

**Priority**: Get Moonshine ONNX working. Everything else is secondary.

---

**End of Agent Instructions**

*Last updated: March 31, 2026*
*Models: Moonshine v2, Qwen 3.5, Kokoro v1.0, Voxtral TTS*
