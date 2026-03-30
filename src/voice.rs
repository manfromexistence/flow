//! Voice pipeline: STT → LLM → TTS

use anyhow::Result;
use std::path::Path;

pub struct VoicePipeline {
    // Placeholder for future voice pipeline
}

impl VoicePipeline {
    pub fn new() -> Self {
        Self {}
    }
    
    pub fn list_available_models() -> Result<()> {
        println!("═══════════════════════════════════════════════════════════");
        println!("  AVAILABLE MODELS");
        println!("═══════════════════════════════════════════════════════════");
        
        // Check LLM models
        println!("\n📝 LLM Models (models/llm/):");
        if Path::new("models/llm/Qwen3.5-0.8B-Q4_K_M.gguf").exists() {
            println!("  ✓ Qwen3.5-0.8B-Q4_K_M.gguf (~500MB, 0.8B params)");
        }
        if Path::new("models/llm/Qwen3.5-2B-Q4_K_M.gguf").exists() {
            println!("  ✓ Qwen3.5-2B-Q4_K_M.gguf (~1.2GB, 2B params)");
        }
        if Path::new("models/llm/Qwen3-0.6B-Q4_K_M.gguf").exists() {
            println!("  ✓ Qwen3-0.6B-Q4_K_M.gguf (~400MB, 0.6B params)");
        }
        
        // Check STT models
        println!("\n🎤 STT Models:");
        if Path::new("models/vosk/vosk-model-small-en-us-0.15").exists() {
            println!("  ✓ Vosk Small EN-US (offline, lightweight)");
        }
        
        // Check TTS models
        println!("\n🔊 TTS Models (models/tts/):");
        if Path::new("models/tts/kokoro-v1.0.int8.onnx").exists() {
            println!("  ✓ Kokoro v1.0 INT8 (~80MB, #1 TTS Arena)");
        }
        if Path::new("models/tts/voices-v1.0.bin").exists() {
            println!("  ✓ Voice embeddings available");
        }
        
        // Check wake word models
        println!("\n⚡ Wake Word Models (models/wake_words/):");
        if Path::new("models/wake_words/dx.onnx").exists() {
            println!("  ✓ 'dx' wake word");
        }
        if Path::new("models/wake_words/hello.onnx").exists() {
            println!("  ✓ 'hello' wake word");
        }
        if Path::new("models/wake_words/arise.onnx").exists() {
            println!("  ✓ 'arise' wake word");
        }
        
        println!("\n═══════════════════════════════════════════════════════════");
        
        Ok(())
    }
}
