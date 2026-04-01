use anyhow::{Result, Context};
use std::path::Path;
use std::fs::File;
use std::io::Read;

/// Real Kokoro TTS using ONNX Runtime
pub struct KokoroTTS {
    session: ort::Session,
    voices: Vec<f32>,
}

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
        
        // Load ONNX model
        let session = ort::Session::builder()?
            .commit_from_file("models/tts/kokoro-v1.0.int8.onnx")
            .context("Failed to load Kokoro ONNX model")?;
        
        // Load voice embeddings
        let mut file = File::open("models/tts/voices-v1.0.bin")?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        
        // Convert bytes to f32 array
        let voices: Vec<f32> = buffer
            .chunks_exact(4)
            .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect();
        
        println!("✓ Kokoro TTS ready");
        println!("  Loaded {} voice embeddings", voices.len() / 256);
        
        Ok(Self { session, voices })
    }
    
    /// Convert text to phonemes using espeak-ng
    fn text_to_phonemes(&self, text: &str) -> Result<String> {
        use espeak_rs::{Speaker, initialize};
        
        // Initialize espeak
        initialize(None).context("Failed to initialize espeak")?;
        
        let speaker = Speaker::new().context("Failed to create speaker")?;
        
        // Get IPA phonemes
        let phonemes = speaker.text_to_phonemes(text, true)
            .context("Failed to convert text to phonemes")?;
        
        Ok(phonemes)
    }
    
    /// Convert phonemes to token IDs (simplified mapping)
    fn phonemes_to_tokens(&self, phonemes: &str) -> Vec<i64> {
        // This is a simplified tokenization
        // Real implementation would use the proper Kokoro tokenizer
        let mut tokens = vec![0i64]; // Start token
        
        for ch in phonemes.chars() {
            // Map characters to token IDs (simplified)
            let token = match ch {
                'a'..='z' => (ch as i64 - 'a' as i64) + 10,
                'A'..='Z' => (ch as i64 - 'A' as i64) + 36,
                ' ' => 16,
                '.' => 4,
                ',' => 5,
                '!' => 6,
                '?' => 7,
                _ => 16, // Space for unknown
            };
            tokens.push(token);
        }
        
        tokens.push(0); // End token
        tokens
    }
    
    /// Synthesize speech from text
    pub fn synthesize(&self, text: &str) -> Result<Vec<f32>> {
        println!("\n→ Synthesizing speech with Kokoro...");
        println!("  Text: \"{}\"", text);
        
        // Convert text to phonemes
        let phonemes = self.text_to_phonemes(text)?;
        println!("  Phonemes: {}", phonemes);
        
        // Convert phonemes to tokens
        let tokens = self.phonemes_to_tokens(&phonemes);
        let token_count = tokens.len();
        println!("  Tokens: {} tokens", token_count);
        
        // Ensure tokens fit in context (max 512)
        if token_count > 510 {
            return Err(anyhow::anyhow!("Text too long: {} tokens (max 510)", token_count));
        }
        
        // Get voice embedding (use default voice af)
        // Voice embeddings are stored as [num_lengths, 1, 256]
        // We select based on token length
        let voice_idx = token_count.min(self.voices.len() / 256 - 1);
        let voice_start = voice_idx * 256;
        let voice_embedding: Vec<f32> = self.voices[voice_start..voice_start + 256].to_vec();
        
        // Prepare inputs for ONNX
        use ndarray::{Array2, Array3};
        
        let input_ids = Array2::from_shape_vec(
            (1, token_count),
            tokens
        )?;
        
        let style = Array3::from_shape_vec(
            (1, 1, 256),
            voice_embedding
        )?;
        
        let speed = Array2::from_shape_vec(
            (1, 1),
            vec![1.0f32]
        )?;
        
        // Run ONNX inference
        println!("  Running ONNX inference...");
        let outputs = self.session.run(ort::inputs![
            "input_ids" => input_ids.view(),
            "style" => style.view(),
            "speed" => speed.view(),
        ]?)?;
        
        // Extract audio
        let audio_tensor = outputs["audio"].try_extract_tensor::<f32>()?;
        let audio: Vec<f32> = audio_tensor.view().iter().copied().collect();
        
        println!("✓ Generated {} samples ({:.2}s at 24kHz)", 
            audio.len(), audio.len() as f64 / 24000.0);
        
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
