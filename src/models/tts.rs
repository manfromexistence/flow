use anyhow::Result;

/// Kokoro TTS using kokoro-tiny crate (proper tokenization)
pub struct KokoroTTS {
    engine: Option<kokoro_tiny::TtsEngine>,
}

impl KokoroTTS {
    pub fn is_available() -> bool {
        // kokoro-tiny auto-downloads models, so always available if espeak-ng is installed
        true
    }
    
    pub async fn new_async() -> Result<Self> {
        println!("⚙️  Initializing Kokoro TTS...");
        println!("  Note: First run will download models (~337MB) to ~/.cache/k/");
        
        let engine = kokoro_tiny::TtsEngine::new().await
            .map_err(|e| anyhow::anyhow!("Failed to initialize Kokoro: {}", e))?;
        
        println!("✓ Kokoro TTS ready");
        
        Ok(Self { engine: Some(engine) })
    }
    
    pub fn new() -> Result<Self> {
        // Synchronous wrapper - only use if not in async context
        Err(anyhow::anyhow!("Use new_async() instead - TTS requires async initialization"))
    }
    
    pub fn synthesize(&mut self, text: &str) -> Result<Vec<f32>> {
        println!("\n→ Synthesizing speech with Kokoro...");
        println!("  Text: \"{}\"", text);
        
        let engine = self.engine.as_mut()
            .ok_or_else(|| anyhow::anyhow!("TTS engine not initialized"))?;
        
        // Use default voice (af_sky) with normal speed
        let audio = engine.synthesize(text, None)
            .map_err(|e| anyhow::anyhow!("Synthesis failed: {}", e))?;
        
        println!("✓ Generated {} samples ({:.2}s at 24kHz)", 
            audio.len(), audio.len() as f64 / 24000.0);
        
        Ok(audio)
    }
    
    pub fn speak(&mut self, text: &str) -> Result<()> {
        let audio = self.synthesize(text)?;
        
        // Save to file for debugging
        self.save_wav(&audio, "debug_output.wav")?;
        println!("  Saved to debug_output.wav for inspection");
        
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tts_availability() {
        if KokoroTTS::is_available() {
            let tts = KokoroTTS::new();
            assert!(tts.is_ok());
        }
    }
}
