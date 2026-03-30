# Model Documentation

## Speech-to-Text (STT)

### Moonshine v2
- **Model**: Moonshine Tiny (35M parameters)
- **Location**: `models/stt/moonshine-tiny-*.onnx`
- **License**: MIT
- **Performance**: ~7.2% WER
- **Latency**: < 1s for 3s audio

## Large Language Model (LLM)

### Qwen 3.5
- **Model**: Qwen 3.5 0.8B (Q4_K_M quantized)
- **Location**: `models/llm/Qwen3.5-0.8B-Q4_K_M.gguf`
- **License**: Apache 2.0
- **RAM**: ~1.5GB
- **Use Case**: Text enhancement, filler word removal

## Text-to-Speech (TTS)

### Kokoro v1.0
- **Model**: Kokoro 82M (INT8 quantized)
- **Location**: `models/tts/kokoro-v1.0.int8.onnx`
- **License**: Apache 2.0
- **Quality**: #1 on TTS Arena (44% win rate)
- **Latency**: Very fast, runs on CPU

## Model Upgrades

### Recommended Upgrades by RAM:

**4-8GB RAM:**
- STT: Moonshine Base (100M)
- LLM: Qwen 3.5 4B
- TTS: Kokoro 82M

**8-16GB RAM:**
- STT: Moonshine Large (245M)
- LLM: Qwen 3.5 9B
- TTS: Kokoro + CosyVoice2

**16GB+ RAM:**
- STT: Moonshine Large + Canary Qwen
- LLM: Qwen 3.5 27B
- TTS: Voxtral TTS 4B
