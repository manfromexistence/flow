# Help Needed - Kokoro TTS Pure Rust Implementation

**Date:** 2026-04-02  
**Status:** Pure Rust implementation compiles but ONNX inference hangs

---

## Current Status

### ✓ What's Working
- Pure Rust implementation using `ort` crate (no Python!)
- Code compiles successfully
- Model loads without errors
- Tokenization runs (basic fallback without espeak)
- Voice embeddings load correctly (27553 embeddings)

### ✗ What's NOT Working
- **ONNX inference hangs** - `session.run()` never returns
- Likely causes:
  1. Input names are wrong ("tokens", "style", "speed")
  2. Input shapes are incorrect
  3. int8 quantized model has compatibility issues with ort
  4. Tokenization produces invalid token IDs

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

### Solution 1: Use Working Rust Crate (FASTEST)
Install one of the existing Kokoro Rust crates:

**Option A: kokoroxide**
```toml
[dependencies]
kokoroxide = "0.1"  # Check crates.io for latest
```

**Option B: Search crates.io**
```bash
cargo search kokoro
```

These crates already solve tokenization and ONNX inference.

### Solution 2: Fix Current Implementation
1. Run Python debug script to get correct input/output names
2. Update Rust code with correct names
3. Try non-quantized model if int8 has issues
4. Implement proper tokenization with espeak-rs

### Solution 3: Use tract-onnx with FP32 Model
```bash
# Download non-quantized model
# models/tts/model.onnx (fp32)

cargo remove ort
cargo add tract-onnx
```

tract-onnx works with standard operators, just not int8 quantization.

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
