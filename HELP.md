# Help Needed - Kokoro TTS Implementation in Pure Rust

**Date:** 2026-04-02

**Problem:** Need to implement Kokoro TTS in pure Rust without Python subprocess calls

---

## Task Description

Implement working Kokoro TTS (text-to-speech) in pure Rust that plays actual synthesized speech.

## Why Pure Rust?

User explicitly requested: "Why are we even using Python at all, bro? Use Rust and its code correctly."

The current Python subprocess approach is a hack and doesn't work anyway (produces beeps).

---

## Current Status

### What's Working ✓
- Moonshine STT (speech-to-text) works correctly in Rust
- Audio playback infrastructure exists (`src/audio/player.rs`)
- Models are downloaded: `models/tts/kokoro-v1.0.int8.onnx` and `models/tts/voices-v1.0.bin`

### What's NOT Working ✗
- **Python approach produces beeps** - tokenization is wrong
- **No pure Rust implementation** - currently using Python subprocess hack
- All Rust ONNX libraries attempted have failed

---

## Rust ONNX Library Attempts

### Attempt 1: tract-onnx
**Status:** FAILED
**Error:** `Unimplemented(DynamicQuantizeLSTM)` - Kokoro's int8 quantized LSTM not supported
**Code tried:**
```rust
use tract_onnx::prelude::*;
let model = tract_onnx::onnx()
    .model_for_path("models/tts/kokoro-v1.0.int8.onnx")?
    .into_optimized()?
    .into_runnable()?;
```

### Attempt 2: ort (ONNX Runtime)
**Status:** FAILED
**Error:** Threading/Send/Sync trait issues, can't convert errors to anyhow
**Code tried:**
```rust
use ort::session::Session;
let session = Session::builder()?
    .with_intra_threads(4)?
    .commit_from_file("models/tts/kokoro-v1.0.int8.onnx")?;
```
**Problem:** `ort::Error<SessionBuilder>` doesn't implement Send/Sync, can't use with `?` operator

### Attempt 3: candle-onnx
**Status:** FAILED
**Error:** Requires protobuf compiler, API unclear, `simple_eval` doesn't exist
**Code tried:**
```rust
use candle_onnx;
let model = candle_onnx::read_file("models/tts/kokoro-v1.0.int8.onnx")?;
let model = candle_onnx::simple_eval::SimpleEval::new(model)?; // simple_eval not found
```

### Attempt 4: Python subprocess (current hack)
**Status:** FAILED - produces beeps
**Problem:** Tokenization is wrong, not a proper solution anyway

---

## Root Cause Analysis

**Problem:** Need pure Rust implementation, not Python subprocess hack

**Why previous attempts failed:**
1. **tract-onnx:** Doesn't support int8 quantized LSTM operators
2. **ort:** Error handling incompatible with anyhow (Send/Sync issues)
3. **candle-onnx:** API unclear, documentation lacking
4. **Python:** Wrong tokenization, not a proper solution

**What's needed:**
1. Working Rust ONNX runtime that supports Kokoro model
2. Proper tokenization (phonemes or IPA, not simple character mapping)
3. Correct voice embedding loading
4. Or: Pure Rust Kokoro implementation without ONNX

---

## Environment Info

- **OS:** Windows (win32)
- **Shell:** bash/PowerShell
- **Rust:** Edition 2024, latest stable
- **Python:** 3.12
- **Key Dependencies:**
  - onnxruntime 1.24.4
  - numpy 2.4.4
  - soundfile 0.13.1
  - rodio 0.22 (Rust audio playback)
  - hound 3.5 (Rust WAV I/O)

---

## Relevant Files

### Python TTS Runner
**File:** `src/models/tts_runner.py`
- Loads `models/tts/kokoro-v1.0.int8.onnx`
- Loads `models/tts/voices-v1.0.bin`
- Tokenizes text (simple character-based)
- Runs ONNX inference
- Saves to WAV at 24kHz

**Key inputs to ONNX model:**
```python
{
    "tokens": input_ids,      # Shape: (1, num_tokens), dtype: int64
    "style": voice_embedding, # Shape: (1, 256), dtype: float32
    "speed": speed            # Shape: (1,), dtype: float32, value: 1.0
}
```

### Rust TTS Wrapper
**File:** `src/models/tts.rs`
- Calls Python script via `std::process::Command`
- Loads generated WAV with hound
- Converts i16 samples to f32: `s as f32 / i16::MAX as f32`
- Calls `AudioPlayer::play(&audio, 24000)`

### Audio Player
**File:** `src/audio/player.rs`
- Uses rodio library
- Creates `OutputStream` and `Sink`
- Appends audio samples
- Calls `sink.sleep_until_end()`

### Audio Loader
**File:** `src/audio/loader.rs`
- Loads MP3/WAV files
- Resamples to 16kHz (this was bypassed for TTS)
- Converts to mono f32 samples

---

## Suggested Solutions (Pure Rust)

### Solution 1: Fix ort (ONNX Runtime) error handling (RECOMMENDED)
The `ort` crate is the most mature Rust ONNX runtime but has error handling issues.

**What to try:**
1. Don't use `?` operator with SessionBuilder, use `.unwrap()` or manual error handling
2. Or wrap errors manually:
```rust
let session = Session::builder()
    .with_intra_threads(4)
    .commit_from_file("models/tts/kokoro-v1.0.int8.onnx")
    .map_err(|e| anyhow::anyhow!("Failed to load model: {:?}", e))?;
```
3. Check ort documentation for correct API usage in 2026
4. The library might have been updated since last attempt

### Solution 2: Use wonnx (WebGPU ONNX runtime)
**Crate:** `wonnx`
**Status:** Archived in 2025, but might still work
**What to try:**
```rust
use wonnx::utils::OutputTensor;
let session = wonnx::Session::from_path("models/tts/kokoro-v1.0.int8.onnx")?;
```

### Solution 3: Use tract-onnx with non-quantized model
**Problem:** Current model uses int8 quantization which tract doesn't support
**What to try:**
1. Download non-quantized Kokoro model (fp32 or fp16)
2. From: https://huggingface.co/onnx-community/Kokoro-82M-ONNX
3. Use `model.onnx` instead of `kokoro-v1.0.int8.onnx`
4. tract-onnx should work with standard operators

### Solution 4: Implement Kokoro in pure Rust without ONNX
**Most work but most reliable**
**What to try:**
1. Find Kokoro model architecture (Transformer-based TTS)
2. Implement using `candle` or `burn` ML frameworks
3. Load weights from ONNX or PyTorch checkpoint
4. This is how production systems do it

### Solution 5: Use burn-onnx
**Crate:** `burn` with ONNX support
**What to try:**
```rust
use burn::backend::NdArray;
use burn_import::onnx::ModelGen;
// Load and run ONNX model with burn
```

---

## Testing Commands (Pure Rust)

```bash
# After implementing in Rust, test with:
cargo run --release -- --speak "Hello test"

# Should hear clear speech, not beeps

# For debugging:
cargo run --release -- --speak "Hello" 2>&1 | tee debug.log

# Compare with working STT:
cargo run --release -- --transcribe tests/fixtures/audio.mp3
```

## Reference Implementations

### Working Rust ONNX Examples
- Search GitHub: "rust onnx inference 2026"
- Check ort crate examples: https://github.com/pykeio/ort
- Look for Rust TTS implementations

### Kokoro Model Info
- HuggingFace: https://huggingface.co/onnx-community/Kokoro-82M-ONNX
- NeuML version: https://huggingface.co/NeuML/kokoro-base-onnx
- Model architecture: Transformer-based TTS (82M parameters)
- Input: Text tokens (phonemes or IPA)
- Output: Audio waveform at 24kHz

### Tokenization References
- espeak-rs crate for phoneme conversion
- IPA (International Phonetic Alphabet) mapping
- Check if Kokoro has its own tokenizer format

---

## Expected Behavior

When running:
```bash
.\target\release\flow.exe --speak "Hello, this is a test"
```

Should hear: Clear synthesized speech saying "Hello, this is a test"
Actually hears: Beep sound

---

## Additional Context

- User explicitly requested NO dummy/mock implementations
- Must use REAL ONNX models (kokoro-v1.0.int8.onnx)
- Models are downloaded and present in `models/tts/` directory
- Python script successfully runs ONNX inference (confirmed by debug output)
- Audio file generation completes successfully
- File sizes are reasonable (e.g., 698KB for 14.55s audio)

---

## Next Steps for AI Assistant

**PRIORITY: Implement in pure Rust, no Python**

### Option A: Fix ort crate (fastest)
1. Add ort with correct version: `cargo add ort --features download-binaries`
2. Fix error handling - don't use `?` with SessionBuilder
3. Implement proper tokenization in Rust (use espeak-rs or phoneme mapping)
4. Load voice embeddings correctly
5. Run ONNX inference
6. Test: `cargo run --release -- --speak "Hello"`

### Option B: Use non-quantized model with tract-onnx
1. Download fp32 Kokoro model from HuggingFace
2. Replace `kokoro-v1.0.int8.onnx` with `model.onnx`
3. Use tract-onnx (should work with standard operators)
4. Implement tokenization
5. Test

### Option C: Pure Rust implementation (most work)
1. Research Kokoro architecture
2. Implement with `candle` or `burn`
3. Load weights from checkpoint
4. This is the "proper" way but takes longest

### Tokenization in Rust
Regardless of ONNX runtime choice, need proper tokenization:
```rust
// Option 1: Use espeak-rs for phonemes
use espeak_rs::Speaker;
let speaker = Speaker::new()?;
let phonemes = speaker.text_to_phonemes(text, true)?;

// Option 2: Use phoneme mapping table
// Map text -> IPA phonemes -> token IDs

// Option 3: Use pre-trained tokenizer
// Load tokenizer.json and implement BPE/WordPiece
```

---

## Files to Review/Modify

### Files to DELETE (Python hack):
- `src/models/tts_runner.py` - Delete this
- `src/models/stt_runner.py` - Keep for reference but STT works

### Files to IMPLEMENT (Pure Rust):
- `src/models/tts.rs` - Rewrite with pure Rust ONNX inference
- `Cargo.toml` - Add correct ONNX runtime crate

### Files that work (reference):
- `src/models/stt.rs` - Working STT implementation (if it uses Rust)
- `src/audio/player.rs` - Audio playback (works)
- `src/audio/loader.rs` - Audio loading (works)

### Model files:
- `models/tts/kokoro-v1.0.int8.onnx` - ONNX model (88MB)
- `models/tts/voices-v1.0.bin` - Voice embeddings (27MB)
- May need to download non-quantized version if using tract-onnx

---

## Success Criteria

✓ Pure Rust implementation (NO Python subprocess)
✓ Running `cargo run --release -- --speak "Hello"` plays clear synthesized speech
✓ No beep sounds
✓ Audio is intelligible and sounds like human speech
✓ Uses REAL Kokoro ONNX model (no mocks/dummies)
✓ All code is in Rust, no external scripts
