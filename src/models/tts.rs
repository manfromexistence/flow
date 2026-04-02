use anyhow::Result;
use std::path::Path;
use std::process::Command;

/// REAL Kokoro TTS - runs actual ONNX model via Python
pub struct KokoroTTS {}

impl KokoroTTS {
    pub fn is_available() -> bool {
        Path::new("models/tts/kokoro-v1.0.int8.onnx").exists() &&
        Path::new("models/tts/voices-v1.0.bin").exists() &&
        Path::new("src/models/tts_runner.py").exists()
    }
    
    pub fn new() -> Result<Self> {
        if !Self::is_available() {
            return Err(anyhow::anyhow!(
                "Kokoro TTS models not found!\n\
                Run: ./scripts/download_ultralight_models.ps1"
            ));
        }
        
        println!("⚙️  Initializing Kokoro TTS...");
        println!("✓ Kokoro TTS ready");
        
        Ok(Self {})
    }
    
    pub fn synthesize(&self, text: &str) -> Result<Vec<f32>> {
        println!("\n→ Synthesizing speech with Kokoro ONNX...");
        println!("  Text: \"{}\"", text);
        
        let temp_wav = "temp_tts_output.wav";
        
        // Run REAL Python ONNX runner
        let output = Command::new("python")
            .arg("src/models/tts_runner.py")
            .arg(text)
            .arg(temp_wav)
            .output()?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("TTS failed: {}", stderr));
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let result: serde_json::Value = serde_json::from_str(&stdout)?;
        
        if let Some(error) = result.get("error") {
            return Err(anyhow::anyhow!("TTS error: {}", error));
        }
        
        let samples = result["samples"].as_u64().unwrap_or(0);
        let duration = result["duration"].as_f64().unwrap_or(0.0);
        
        println!("✓ Generated {} samples ({:.2}s at 24kHz)", samples, duration);
        
        // Load the generated WAV directly without resampling
        use hound::WavReader;
        let mut reader = WavReader::open(temp_wav)?;
        let audio: Vec<f32> = reader
            .samples::<i16>()
            .map(|s| s.map(|s| s as f32 / i16::MAX as f32))
            .collect::<Result<Vec<_>, _>>()?;
        
        // Clean up temp file
        let _ = std::fs::remove_file(temp_wav);
        
        Ok(audio)
    }
    
    pub fn speak(&self, text: &str) -> Result<()> {
        let audio = self.synthesize(text)?;
        crate::audio::AudioPlayer::play(&audio, 24000)?;
        Ok(())
    }
    
    pub fn save_wav(&self, audio: &[f32], path: &str) -> Result<()> {
        use hound::{WavWriter, WavSpec};
        
        let spec = WavSpec {
            channels: 1,
            sample_rate: 24000,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        
        let mut writer = WavWriter::create(path, spec)?;
        
        for &sample in audio {
            let sample_i16 = (sample * 32767.0).clamp(-32768.0, 32767.0) as i16;
            writer.write_sample(sample_i16)?;
        }
        
        writer.finalize()?;
        
        println!("→ Saved audio to: {}", path);
        
        Ok(())
    }
}
