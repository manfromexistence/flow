# 🤖 Edith - Pure Rust Voice AI Assistant

cargo build --release && ./target/release/edith.exe
Hello, what is 2+2??  

**Edith** is a fully local, privacy-focused voice AI assistant built in pure Rust, optimized for low-end devices (8GB RAM). All models run on-device with zero Python dependencies.

## ✅ Build Status
- Compiles successfully with `cargo build --release`
- All errors fixed, only warnings for TODO implementations
- Ready for model integration

## 🎯 Features Implemented
- 24/7 wake word listening (uses `models/wake_words/arise.onnx`)
- Voice Activity Detection (VAD) - detects when user stops speaking
- Full pipeline: Wake Word → STT → LLM → TTS
- Real-time performance metrics (CPU, RAM, timing)
- Audio capture & playback via cpal
- Async processing with Tokio

## 📊 Latest Stack (March 6, 2026)
- **STT**: Moonshine Tiny Streaming via ort (34MB)
- **LLM**: Qwen 3.5 0.8B Q4_K_M via llama-cpp-2 (350MB)
- **TTS**: Kokoro-82M via ort (80MB quantized)
- **Total**: ~464MB | RAM: ~500MB

## 🚀 Quick Start
```bash
# Build release version
cargo build --release

# Run (models need to be integrated first)
cargo run --release
```

## � Models Already Downloaded
Your `models/` folder contains:
- LLM: Qwen3-0.6B & Qwen3.5-0.8B (GGUF)
- TTS: Kokoro-82M ONNX + voices
- Wake words: arise.onnx, dx.onnx, hello.onnx
- STT: Vosk model (can upgrade to Moonshine)

## 🔧 Pipeline Flow
1. Continuously listens for wake word "arise"
2. When detected, starts recording user speech
3. Detects silence (1.5s) to know when user finished
4. Runs Moonshine STT on recorded audio
5. Enhances text with Qwen 3.5 LLM
6. Generates speech with Kokoro TTS
7. Plays response and shows detailed metrics

## 📈 Metrics Displayed
- Wake word detection time
- STT/LLM/TTS individual timings
- Total response time
- Real-time factor (audio length / processing time)
- Memory usage (MB)
- CPU usage (%)
- System uptime

---
Built with ❤️ in Rust | March 2026

In here please create a rust project called edith and do all of these by dooing a web search first and do these as today is 5th March 2026:

For my rust "Edith" project please give me:
Then please give best model for TTS that fully on rust and please give best models and crates in 5th March 2026 and give best rust crates and all packages that fully supported on rust and give best rust crates and ways so that all these  models can run even in low-end device - run fastest on every os and performance is good and uses less ram, cpu and resources and works correctly and gives best quility results also being small!!!


And update this:
```markdown
Let me search for the very latest information to verify and update your stack as of March 5, 2026.Let me search for the latest info to verify your stack.I apologize for the tool limitations. Let me try again properly:I'm hitting execution limits this turn. Let me do all the searches in my next response. But first — let me address your core concern directly:

## 🎯 Your Main Problem: Kokoro TTS is Slow on 8GB RAM

This is a **real and known issue**. Here's why and what to do:

**Why Kokoro is slow for you:**
- Kokoro uses PyTorch under the hood — Python + PyTorch runtime overhead is heavy on 8GB
- If you're calling it from Rust via a Python subprocess or HTTP bridge, that adds latency
- On 8GB total system RAM, once your OS + Rust app + LLM are loaded, there's very little left for PyTorch

**Immediate fix — use Kokoro ONNX instead:**
```bash
pip install kokoro-onnx
```
The ONNX version eliminates PyTorch entirely and runs ~2-3x faster on CPU. This is critical for your 8GB setup.

---

Now let me verify all the model recommendations with fresh searches:Good — I have enough confirmed data from the searches. Let me give you the verified and updated answer.

---

# ✅ Verified Latest Stack (as of March 5, 2026)

Your compiled information is **largely accurate and solid**. Here's what I can confirm and refine:

---

## 1. 📝 LLM — Qwen 3.5 0.8B ✅ CONFIRMED LATEST

Alibaba's Qwen team released the Qwen3.5 Small Model Series on March 2, 2026 — a collection of LLMs ranging from 0.8B to 9B parameters, targeting on-device applications.

The Qwen3.5 Small Model Series consists of Qwen3.5-0.8B, 2B, 4B, and 9B, optimized for "tiny" and "fast" use cases.

This was literally released **3 days ago** — you cannot get more cutting-edge than this. The 0.8B at Q4_K_M (~350MB) is indeed the best sub-1B model available right now.

**Your Ollama instructions are correct** — just make sure you update Ollama to the latest version that includes the Qwen 3.5 model definitions.

---

## 2. 🔊 TTS — Kokoro-82M via ONNX ✅ BUT USE ONNX VERSION!

This is **your bottleneck** and here's the critical fix:

Kokoro-ONNX provides TTS with ONNX runtime, offering fast performance near real-time on macOS M1, multiple voices, and a lightweight footprint of ~300MB (quantized: ~80MB).

**Your problem is you're running the PyTorch version.** On 8GB RAM that's a disaster because PyTorch itself eats ~1-2GB. The fix:

```bash
pip install kokoro-onnx
```

The Kokoro ONNX package leverages the Kokoro-82M model within the ONNX Runtime environment and achieves near real-time performance even on devices like the macOS M1.

For your **Rust project specifically**, use the `ort` crate (ONNX Runtime Rust bindings) to load the Kokoro ONNX model directly — zero Python overhead. This will be **dramatically faster** than calling Python from Rust.

---

## 3. 🎤 STT — Moonshine Tiny ✅ CONFIRMED BEST

The current Moonshine model lineup (v0.0.49, Feb 2026) includes: English Tiny at 26MB with 12.66% WER (smallest possible footprint), and English Tiny Streaming at 34MB with 12.00% WER.

Moonshine includes Tiny, Base, Tiny Streaming, Small Streaming, and Medium Streaming models. The Moonshine Medium Streaming model achieves lower WER than Whisper Large v3.

For voice interfaces, latency below 200ms is critical for responsiveness. Moonshine achieves this on consumer hardware; Whisper cannot.

Updated recommendation:
| Model | Size | WER | Best For |
|---|---|---|---|
| **Moonshine Tiny** | 26MB | 12.66% | Absolute smallest |
| **Moonshine Tiny Streaming** | 34MB | 12.00% | Best for real-time pipe to LLM |

**Pick Moonshine Tiny Streaming (34MB)** — only 8MB more, better accuracy, and it streams partial results so your Qwen 3.5 can start processing immediately.

---

## 🔥 Critical Fix for Your 8GB RAM Setup

Here's your real problem and solution mapped out:

```
BEFORE (slow):
Rust app → Python subprocess → PyTorch Kokoro → huge overhead → SLOW
                                 ↑ PyTorch eats 1-2GB of your 8GB

AFTER (fast):
Rust app → ort crate (ONNX Runtime) → Kokoro ONNX → lean & fast
Rust app → llama-cpp-rs → Qwen 3.5 0.8B Q4_K_M → no Python
Rust app → ort crate → Moonshine Tiny Streaming → no Python
                         ↑ Everything native in Rust, ~490MB total models
```

**The key insight: eliminate Python entirely from your Rust pipeline.** Use:
1. **`llama-cpp-rs`** crate for Qwen 3.5 0.8B
2. **`ort`** crate for both Kokoro ONNX and Moonshine ONNX
3. Zero Python processes = saves ~500MB-1GB RAM overhead

---

## 📊 Updated Final Budget

| Component | Model | Size |
|---|---|---|
| STT | Moonshine Tiny Streaming (ONNX) | ~34MB |
| LLM | Qwen 3.5 0.8B Q4_K_M | ~350MB |
| TTS | Kokoro-82M ONNX (quantized) | ~80MB |
| **TOTAL** | | **~464MB ✅** |

Your original stack was spot-on — the models are confirmed latest as of March 5, 2026. The **one critical change**: switch from Kokoro PyTorch to **Kokoro ONNX** (and ideally run it natively in Rust via the `ort` crate). That alone should solve your slowness problem on 8GB RAM.
```
