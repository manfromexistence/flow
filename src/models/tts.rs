use anyhow::Result;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use ort::session::Session;
use ort::value::Value;
use ndarray::{Array1, Array2};

/// REAL Kokoro TTS using ONNX Runtime (ort crate)
pub struct KokoroTTS {
    session: Session,
    voices: Vec<f32>,
}

impl KokoroTTS {
    pub fn is_available() -> bool {
        Path::new("models/tts/kokoro-v1.0.int8.onnx").exists() &&
        Path::new("models/tts/voices-v1.0.bin").exists()
    }
    
    pub fn new() -> Result<Self> {
        if !Self::is_available() {
            return Err(anyhow::anyhow!(
                "Kokoro TTS models not found!\n\
                Run: ./scripts/download_ultralight_models.ps1"
            ));
        }
        
        println!("⚙️  Initializing Kokoro TTS...");
        
        // Initialize ONNX Runtime session with proper error handling
        let session = Session::builder()
            .map_err(|e| anyhow::anyhow!("Failed to create session builder: {:?}", e))?
            .with_intra_threads(4)
            .map_err(|e| anyhow::anyhow!("Failed to set threads: {:?}", e))?
            .commit_from_file("models/tts/kokoro-v1.0.int8.onnx")
            .map_err(|e| anyhow::anyhow!("Failed to load model: {:?}", e))?;
        
        // Load voice embeddings
        let mut file = File::open("models/tts/voices-v1.0.bin")?;
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)?;
        
        // Ensure buffer size is multiple of 4 (float32)
        let voice_size = buffer.len() / 4 * 4;
        let voices: Vec<f32> = buffer[..voice_size]
            .chunks_exact(4)
            .map(|chunk| f32::from_le_bytes([chunk[0], chunk[1], chunk[2], chunk[3]]))
            .collect();
        
        println!("✓ Kokoro TTS ready");
        println!("  Loaded {} voice embeddings", voices.len() / 256);
        
        Ok(Self { session, voices })
    }
    
    fn text_to_phonemes(&self, text: &str) -> Result<String> {
        // Try to use espeak-rs for proper phonemization
        #[cfg(feature = "espeak")]
        {
            use espeak_rs::Speaker;
            let speaker = Speaker::new().map_err(|e| anyhow::anyhow!("espeak init failed: {:?}", e))?;
            let phonemes = speaker.text_to_phonemes(text, true)
                .map_err(|e| anyhow::anyhow!("phoneme conversion failed: {:?}", e))?;
            return Ok(phonemes);
        }
        
        // Fallback: Simple text normalization (not ideal but works for basic cases)
        #[cfg(not(feature = "espeak"))]
        {
            // Basic normalization - expand common abbreviations
            let normalized = text
                .to_lowercase()
                .replace("mr.", "mister")
                .replace("mrs.", "misses")
                .replace("dr.", "doctor")
                .replace("'", " ");
            Ok(normalized)
        }
    }
    
    fn phonemes_to_tokens(&self, phonemes: &str) -> Vec<i64> {
        // Kokoro token mapping (simplified - proper version would use full IPA mapping)
        let mut tokens = vec![0i64]; // BOS token
        
        for ch in phonemes.chars() {
            let token = match ch {
                // Vowels
                'a' | 'æ' => 10,
                'e' | 'ɛ' => 11,
                'i' | 'ɪ' => 12,
                'o' | 'ɔ' => 13,
                'u' | 'ʊ' => 14,
                
                // Consonants
                'b' => 20, 'p' => 21,
                'd' => 22, 't' => 23,
                'g' => 24, 'k' => 25,
                'f' => 26, 'v' => 27,
                's' => 28, 'z' => 29,
                'ʃ' => 30, 'ʒ' => 31,
                'h' => 32,
                'm' => 33, 'n' => 34, 'ŋ' => 35,
                'l' => 36, 'r' => 37,
                'w' => 38, 'j' => 39,
                
                // Special
                ' ' => 1,
                '.' => 2,
                ',' => 3,
                '!' => 4,
                '?' => 5,
                
                // Default: map to space
                _ => 1,
            };
            tokens.push(token);
        }
        
        tokens.push(0); // EOS token
        tokens
    }
    
    pub fn synthesize(&mut self, text: &str) -> Result<Vec<f32>> {
        println!("\n→ Synthesizing speech with Kokoro ONNX...");
        println!("  Text: \"{}\"", text);
        
        // Convert text to phonemes
        let phonemes = self.text_to_phonemes(text)?;
        println!("  Phonemes: {}", &phonemes[..phonemes.len().min(50)]);
        
        // Convert phonemes to tokens
        let tokens = self.phonemes_to_tokens(&phonemes);
        let token_count = tokens.len();
        println!("  Tokens: {} tokens", token_count);
        
        if token_count > 510 {
            return Err(anyhow::anyhow!("Text too long: {} tokens (max 510)", token_count));
        }
        
        // Get voice embedding - use appropriate style for token length
        let style_idx = (token_count / 256).min((self.voices.len() / 256) - 1);
        let voice_start = style_idx * 256;
        let voice_embedding: Vec<f32> = self.voices[voice_start..voice_start + 256].to_vec();
        
        // Prepare inputs as ndarray
        let input_ids = Array2::from_shape_vec((1, token_count), tokens)?;
        let style = Array2::from_shape_vec((1, 256), voice_embedding)?;
        let speed = Array1::from_vec(vec![1.0f32]);
        
        println!("  Running ONNX inference...");
        
        // Convert to ort Values
        let tokens_value = Value::from_array(input_ids)?;
        let style_value = Value::from_array(style)?;
        let speed_value = Value::from_array(speed)?;
        
        // Run inference
        let outputs = self.session
            .run(ort::inputs!["tokens" => tokens_value, "style" => style_value, "speed" => speed_value])
            .map_err(|e| anyhow::anyhow!("ONNX inference failed: {:?}", e))?;
        
        // Extract audio from output (output name is "audio")
        let audio_tensor = &outputs["audio"];
        let (_shape, data) = audio_tensor.try_extract_tensor::<f32>()
            .map_err(|e| anyhow::anyhow!("Failed to extract audio: {:?}", e))?;
        
        let audio: Vec<f32> = data.to_vec();
        
        println!("✓ Generated {} samples ({:.2}s at 24kHz)", 
            audio.len(), audio.len() as f64 / 24000.0);
        
        Ok(audio)
    }
    
    pub fn speak(&mut self, text: &str) -> Result<()> {
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
