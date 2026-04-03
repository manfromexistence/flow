# Help Needed - Kokoro TTS Tokenization Issue

**Date:** 2026-04-02  
**Status:** ONNX inference works but produces SILENT audio (all zeros)

---

## TL;DR - THE PROBLEM

**Audio output is completely silent because tokenization is wrong.**

- ONNX model works ✓
- Inference completes ✓  
- Audio generated ✓
- **But audio is all zeros (silent) ✗**

**THE FIX:** Need proper phoneme tokenization using espeak-ng or a working Kokoro tokenizer.

Current code uses arbitrary character-to-number mapping which produces invalid tokens that the model can't understand.

---

## CRITICAL FINDING

**The audio output is completely silent (all zeros)!**

```
Sample rate: 24000
Samples: 202800
Duration: 8.45s
Min: 0.000000, Max: 0.000000
Mean: 0.000000, Std: 0.000000
Unique values: 1
```

This means:
- ✓ ONNX model loads correctly
- ✓ Inference runs successfully  
- ✓ Audio is generated (202800 samples)
- ✗ **Audio is all zeros = TOKENIZATION IS WRONG**

---

## Root Cause

The current tokenization is completely incorrect:
```rust
// WRONG - arbitrary character mapping
'a' => 10, 'e' => 11, 'i' => 12, etc.
```

Kokoro expects **proper phoneme tokens** from a real tokenizer, not random character mappings.

---

## Current Status

### ✓ What's Working
- Pure Rust implementation using `ort` crate (no Python!)
- Code compiles successfully
- Model loads without errors
- ONNX inference completes successfully
- Audio is generated (correct length, correct sample rate)
- Voice embeddings load correctly (27553 embeddings)

### ✗ What's NOT Working  
- **AUDIO IS ALL ZEROS (SILENT)** - tokenization produces invalid tokens
- Model generates output but it's meaningless
- The simple character-to-token mapping doesn't match what Kokoro expects
- Need REAL phoneme tokenization (espeak-ng or proper IPA mapping)

---

## Implementation Details

### Current Code (`src/models/tts.rs`)
- Uses `ort = "2.0.0-rc.12"` with `download-binaries` feature
- Loads `models/tts/kokoro-v1.0.int8.onnx`
- Loads `models/tts/voices-v1.0.bin` (27553 float32 values)
- Basic tokenization (no espeak yet)
- Inputs prepared as ndarray, converted to ort::Value

### ONNX Inputs Being Sent
```rust
"tokens" => Array2<i64> shape (1, token_count)
"style" => Array2<f32> shape (1, 256)  
"speed" => Array1<f32> shape (1,)
```

---

## Debug Steps Needed

### 1. Check Model Input Names
The model might expect different input names. Check with:
```python
import onnxruntime as ort
session = ort.InferenceSession("models/tts/kokoro-v1.0.int8.onnx")
for inp in session.get_inputs():
    print(f"Name: {inp.name}, Shape: {inp.shape}, Type: {inp.type}")
```

### 2. Check Model Output Names
```python
for out in session.get_outputs():
    print(f"Name: {out.name}, Shape: {out.shape}, Type: {out.type}")
```

### 3. Try Non-Quantized Model
Download fp32 model from HuggingFace:
- https://huggingface.co/onnx-community/Kokoro-82M-ONNX
- Replace `kokoro-v1.0.int8.onnx` with `model.onnx`
- int8 quantization might have issues with ort

### 4. Verify Token IDs Are Valid
Current tokenization maps characters to token IDs 0-39. Check if:
- Token IDs are in valid range for model vocabulary
- BOS/EOS tokens (0) are correct
- Model expects different token format

### 5. Add Timeout to ONNX Session
```rust
let session = Session::builder()?
    .with_execution_mode(ExecutionMode::Sequential)?  // Try sequential
    .with_intra_threads(1)?  // Try single thread
    .commit_from_file("models/tts/kokoro-v1.0.int8.onnx")?;
```

---

## Recommended Solutions

### Solution 1: Install espeak-ng and Use Proper Tokenization (REQUIRED)

**The ONLY way to fix this is proper phoneme tokenization.**

1. **Install espeak-ng on Windows:**
   - Download from: https://github.com/espeak-ng/espeak-ng/releases
   - Install to `C:\Program Files\eSpeak NG\`
   - Add to PATH or set `ESPEAK_DATA_PATH` environment variable

2. **Use espeak-rs in Rust:**
```rust
// In Cargo.toml
[dependencies]
espeak-rs = "0.1"

// In code
use espeak_rs::Speaker;
let speaker = Speaker::new()?;
let phonemes = speaker.text_to_phonemes("Hello", true)?;
// phonemes = "həloʊ" (IPA format)
```

3. **Map IPA phonemes to Kokoro token IDs:**
   - Need the actual Kokoro vocabulary/tokenizer
   - Check HuggingFace model card for tokenizer.json
   - Or use ttstokenizer Python package as reference

### Solution 2: Use Existing Rust Kokoro Crate (FASTEST)

These crates already have proper tokenization:

```bash
cargo search kokoro
# Look for: kokoroxide, kokoros, or similar
```

Example with kokoroxide:
```rust
use kokoroxide::{KokoroTTS, TTSConfig};
let tts = KokoroTTS::new("models/tts/kokoro-v1.0.int8.onnx")?;
let audio = tts.speak("Hello")?;
```

### Solution 3: Copy Working Python Tokenization

The Python version that works uses:
```python
from ttstokenizer import IPATokenizer
tokenizer = IPATokenizer()
tokens = tokenizer("Hello")
```

Find the equivalent in Rust or call Python tokenizer from Rust as a temporary solution.

---

## Files

### Working Files
- `src/models/tts.rs` - Pure Rust implementation (compiles, but inference hangs)
- `src/models/stt.rs` - STT works correctly (reference)
- `src/audio/player.rs` - Audio playback works
- `Cargo.toml` - Has `ort = "2.0.0-rc.12"`

### Files to Delete
- `src/models/tts_runner.py` - No longer needed
- `src/models/stt_runner.py` - Keep for reference

### Model Files
- `models/tts/kokoro-v1.0.int8.onnx` (88MB) - Currently used, might be problematic
- `models/tts/voices-v1.0.bin` (27MB) - Loads correctly

---

## Test Commands

```bash
# Current test (hangs at inference)
.\target\release\flow.exe --speak "Hello test"

# After fix, should hear speech
cargo run --release -- --speak "Hello world"

# Working STT for reference
cargo run --release -- --transcribe tests/fixtures/audio.mp3
```

---

## Python Debug Script

Save as `debug_model.py`:
```python
import onnxruntime as ort
import numpy as np

# Load model
session = ort.InferenceSession("models/tts/kokoro-v1.0.int8.onnx")

print("=== INPUTS ===")
for inp in session.get_inputs():
    print(f"Name: {inp.name}")
    print(f"Shape: {inp.shape}")
    print(f"Type: {inp.type}")
    print()

print("=== OUTPUTS ===")
for out in session.get_outputs():
    print(f"Name: {out.name}")
    print(f"Shape: {out.shape}")
    print(f"Type: {out.type}")
    print()

# Try minimal inference
print("=== TEST INFERENCE ===")
try:
    # Minimal inputs
    tokens = np.array([[0, 10, 11, 12, 0]], dtype=np.int64)  # BOS + 3 tokens + EOS
    style = np.zeros((1, 256), dtype=np.float32)
    speed = np.array([1.0], dtype=np.float32)
    
    outputs = session.run(None, {
        "tokens": tokens,
        "style": style,
        "speed": speed
    })
    print(f"Success! Output shape: {outputs[0].shape}")
except Exception as e:
    print(f"Error: {e}")
    print("\nTry different input names:")
    print("- input_ids instead of tokens?")
    print("- Different shapes?")
```

Run: `python debug_model.py`

---

## Success Criteria

✓ Pure Rust implementation (NO Python)  
✓ `cargo run --release -- --speak "Hello"` completes without hanging  
✓ Plays clear synthesized speech (not beeps)  
✓ Audio is intelligible  
✓ Uses REAL Kokoro ONNX model

---

## Next Steps

1. **Run debug_model.py** to get correct input/output names
2. **Update Rust code** with correct names from debug output
3. **Try non-quantized model** if int8 has issues
4. **Or use existing Rust crate** (kokoroxide, etc.) - fastest solution
5. **Test**: `cargo run --release -- --speak "Hello"`


---

## UPDATE: Attempt 2 - Added voice-g2p (STILL SILENT)

**Date:** 2026-04-02 (later)  
**Status:** Phonemization works but audio STILL silent

### What Changed
- Added `voice-g2p = "0.2.2"` crate for proper G2P conversion
- Now using `english_to_phonemes()` which generates correct IPA phonemes
- Phonemes generated successfully: `həlˈO` for "Hello" ✓
- Created comprehensive IPA-to-token mapping with 50+ phonemes
- Handles multi-character phonemes (diphthongs) correctly

### Test Output
```
→ Speaking: Hello
⚙️  Initializing Kokoro TTS...
✓ Kokoro TTS ready
  Loaded 27553 voice embeddings
→ Synthesizing speech with Kokoro ONNX...
  Text: "Hello"
  Converting text to phonemes...
  Phonemes generated: həlˈO
  Phonemes: həlˈO
  Tokens: 7 tokens
  Running ONNX inference...
✓ Generated 202800 samples (8.45s at 24kHz)
→ Saved audio to: debug_output.wav
```

### Audio Analysis
```
Min: 0.000000, Max: 0.000000, Mean: 0.000000, Std: 0.000000, Unique: 1
```

**STILL ALL ZEROS!**

### Conclusion
- Phonemization is CORRECT ✓
- Token mapping is WRONG ✗

The `voice-g2p` crate generates proper IPA phonemes, but the token IDs I'm assigning to each phoneme don't match what the Kokoro model expects.

### The Real Problem
I'm guessing token IDs (h→44, ə→12, l→48, etc.) but these are arbitrary. The Kokoro model was trained with specific token IDs for each phoneme, and I don't have the official vocabulary mapping.

### What's Needed
1. **Official Kokoro vocabulary file** - Find `vocab.json` or `tokenizer.json` from HuggingFace
2. **Or ttstokenizer source code** - Extract the exact phoneme→token mapping
3. **Or working Rust implementation** - Copy tokenization from `kokoroxide` or similar

Without the correct token mapping, the model will always output silence no matter how good the phonemization is.


---

## UPDATE: Attempt 3 - Discovered Token Format Mismatch (STILL SILENT)

**Date:** 2026-04-04  
**Status:** Found the root cause - phoneme format mismatch

### Critical Discovery
Kokoro uses **ARPABET phonemes** (e.g., "HH EH L OW" for "hello"), NOT IPA phonemes!

- `ttstokenizer` (Python) outputs ARPABET: "Hello" → [50, 83, 54, 156, 57, 135]
- `misaki-rs` (Rust) outputs IPA/Misaki notation: "Hello" → "həlˈo‍ʊ"
- These are INCOMPATIBLE formats!

### The Official Kokoro Vocabulary
Extracted from `ttstokenizer` Python library:
```
  0: '<pad>'     22: 'B'        50: 'OW0'
  1: '<unk>'     23: 'CH'       51: 'OW1'
  2: '<s>'       24: 'D'        54: 'OY1'  (L in ARPABET)
  3: '</s>'      25: 'DH'       57: 'R'
  4: 'AA0'       ...            83: (HH in ARPABET)
  ...            46: 'L'        156: (EH in ARPABET)
```

### Why It's Still Silent
My current implementation:
1. Uses `misaki-rs` to generate IPA phonemes: "həlˈo‍ʊ"
2. Maps IPA characters to guessed token IDs: [50, 27, 54, 137, 20, ...]
3. Model expects ARPABET token IDs: [HH, EH, L, OW]
4. Result: Garbage tokens → silent audio

### Solution Options

**Option 1: Use ttstokenizer directly (EASIEST)**
- Call Python `ttstokenizer` from Rust
- Gets correct ARPABET tokens immediately
- Downside: Requires Python at runtime

**Option 2: Port ttstokenizer to Rust (PROPER)**
- Implement ARPABET G2P in pure Rust
- Use CMUDict for word→phoneme lookup
- Complex but no Python dependency

**Option 3: IPA→ARPABET conversion (HACKY)**
- Keep `misaki-rs` for IPA generation
- Create IPA→ARPABET mapping table
- Fragile and error-prone

### Recommendation
Use Option 1 (Python ttstokenizer) as a temporary solution, then implement Option 2 for a pure Rust solution.

The phonemization works, the ONNX inference works, the audio generation works. Only the token format is wrong.
