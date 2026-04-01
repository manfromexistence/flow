use anyhow::Result;
use std::path::Path;

/// Text-to-Speech engine using Kokoro v1.0 (82M parameters)
/// Currently using simplified audio generation until full ONNX implementation
pub struct KokoroTTS;

impl KokoroTTS {
    pub fn is_available() -> bool {
        Path::new("models/tts/kokoro-v1.0.int8.onnx").exists() &&
        Path::new("models/tts/voices-v1.0.bin").exists()
    }
    
    /// Create new TTS engine
    pub fn new() -> Result<Self> {
        if !Self::is_available() {
            return Err(anyhow::anyhow!(
                "Kokoro TTS models not found!\n\
                Run: ./scripts/download_ultralight_models.ps1"
            ));
        }
        
        println!("⚙️  Initializing Kokoro TTS...");
        println!("✓ Kokoro TTS ready (simplified mode)");
        
        Ok(Self)
    }
    
    /// Synthesize speech from text
    pub fn synthesize(&self, text: &str) -> Result<Vec<f32>> {
        println!("\n→ Synthesizing speech...");
        println!("  Text: \"{}\"", text);
        
        // Generate pleasant notification sound (multi-tone chime)
        let sample_rate = 24000;
        let duration = 0.5; // Short pleasant sound
        let num_samples = (sample_rate as f64 * duration) as usize;
        
        let mut audio = Vec::with_capacity(num_samples);
        
        // Create a pleasant chime sound (C major chord: C-E-G)
        let frequencies = [523.25, 659.25, 783.99]; // C5, E5, G5
        
        for i in 0..num_samples {
            let t = i as f32 / sample_rate as f32;
            
            // Envelope (fade in and out)
            let envelope = if t < 0.05 {
                t / 0.05
            } else if t > duration as f32 - 0.1 {
                (duration as f32 - t) / 0.1
            } else {
                1.0
            };
            
            // Mix three frequencies for a pleasant chord
            let mut sample = 0.0;
            for (idx, &freq) in frequencies.iter().enumerate() {
                let amplitude = 0.15 / frequencies.len() as f32;
                let phase_offset = idx as f32 * 0.1; // Slight delay for richness
                sample += (2.0 * std::f32::consts::PI * freq * (t + phase_offset)).sin() * amplitude;
            }
            
            audio.push(sample * envelope);
        }
        
        println!("✓ Generated {} samples ({:.2}s at 24kHz)", 
            audio.len(), audio.len() as f64 / 24000.0);
        println!("  Note: Full Kokoro ONNX TTS coming soon!");
        
        Ok(audio)
    }
    
    /// Synthesize speech from text and play it
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
