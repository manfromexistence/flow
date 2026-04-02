use anyhow::Result;
use std::path::Path;
use std::process::Command;

/// REAL Moonshine STT - runs actual ONNX model via Python
pub struct MoonshineSTT {}

impl MoonshineSTT {
    pub fn is_available() -> bool {
        Path::new("models/stt/moonshine-tiny-encoder.onnx").exists() &&
        Path::new("models/stt/moonshine-tiny-decoder.onnx").exists() &&
        Path::new("models/stt/moonshine-tiny-tokenizer.json").exists() &&
        Path::new("src/models/stt_runner.py").exists()
    }
    
    pub fn new() -> Result<Self> {
        if !Self::is_available() {
            return Err(anyhow::anyhow!(
                "Moonshine ONNX models not found!\n\
                Run: ./scripts/download_ultralight_models.ps1"
            ));
        }
        
        println!("⚙️  Initializing Moonshine STT...");
        println!("✓ Moonshine STT ready");
        
        Ok(Self {})
    }
    
    pub fn transcribe(&self, audio_path: &str) -> Result<String> {
        println!("\n→ Transcribing audio with Moonshine ONNX...");
        
        // Run REAL Python ONNX runner
        let output = Command::new("python")
            .arg("src/models/stt_runner.py")
            .arg(audio_path)
            .output()?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("STT failed: {}", stderr));
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let result: serde_json::Value = serde_json::from_str(&stdout)?;
        
        if let Some(error) = result.get("error") {
            return Err(anyhow::anyhow!("STT error: {}", error));
        }
        
        let text = result["text"].as_str().unwrap_or("");
        let tokens = result["tokens"].as_u64().unwrap_or(0);
        
        println!("  Generated {} tokens", tokens);
        println!("✓ Enhanced: \"{}\"", text);
        
        Ok(text.to_string())
    }
}
