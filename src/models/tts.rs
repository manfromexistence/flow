use anyhow::Result;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use ort::session::Session;
use ort::value::Value;
use ndarray::{Array1, Array2};
use voice_g2p::english_to_phonemes;

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
        // Use voice-g2p for proper Kokoro-compatible phonemization
        println!("  Converting text to phonemes...");
        let phonemes = english_to_phonemes(text)
            .map_err(|e| anyhow::anyhow!("G2P conversion failed: {:?}", e))?;
        println!("  Phonemes generated: {}", &phonemes[..phonemes.len().min(100)]);
        Ok(phonemes)
    }
    
    fn phonemes_to_tokens(&self, phonemes: &str) -> Vec<i64> {
        // Kokoro phoneme-to-token mapping (Misaki notation)
        // Based on the Kokoro model's expected input format
        let mut tokens = vec![0i64]; // BOS token
        
        let mut i = 0;
        let chars: Vec<char> = phonemes.chars().collect();
        
        while i < chars.len() {
            // Try to match multi-character phonemes first (diphthongs)
            let remaining: String = chars[i..].iter().collect();
            
            let (token, advance) = if remaining.starts_with("aɪ") {
                (21, 2) // /aɪ/ as in "my"
            } else if remaining.starts_with("aʊ") {
                (22, 2) // /aʊ/ as in "now"
            } else if remaining.starts_with("eɪ") {
                (23, 2) // /eɪ/ as in "day"
            } else if remaining.starts_with("oʊ") {
                (24, 2) // /oʊ/ as in "go"
            } else if remaining.starts_with("ɔɪ") {
                (25, 2) // /ɔɪ/ as in "boy"
            } else {
                // Single character phonemes
                let ch = chars[i];
                let token = match ch {
                    // Vowels (IPA/Misaki notation)
                    'a' => 7,   // /a/ as in "father"
                    'ɑ' => 8,   // /ɑ/ as in "lot"
                    'æ' => 9,   // /æ/ as in "cat"
                    'e' => 10,  // /e/ as in "day"
                    'ɛ' => 11,  // /ɛ/ as in "bed"
                    'ə' => 12,  // /ə/ schwa as in "about"
                    'ᵊ' => 12,  // alternate schwa
                    'i' => 13,  // /i/ as in "see"
                    'ɪ' => 14,  // /ɪ/ as in "sit"
                    'I' => 14,  // alternate /ɪ/
                    'o' => 15,  // /o/ as in "go"
                    'ɔ' => 16,  // /ɔ/ as in "thought"
                    'O' => 16,  // alternate /ɔ/
                    'u' => 17,  // /u/ as in "food"
                    'ʊ' => 18,  // /ʊ/ as in "foot"
                    'ʌ' => 19,  // /ʌ/ as in "cup"
                    'ɜ' => 20,  // /ɜ/ as in "bird"
                    
                    // Consonants
                    'b' => 30,  // /b/ as in "bat"
                    'p' => 31,  // /p/ as in "pat"
                    'd' => 32,  // /d/ as in "dog"
                    't' => 33,  // /t/ as in "top"
                    'g' => 34,  // /g/ as in "go"
                    'k' => 35,  // /k/ as in "cat"
                    'f' => 36,  // /f/ as in "fat"
                    'v' => 37,  // /v/ as in "vat"
                    's' => 38,  // /s/ as in "sat"
                    'z' => 39,  // /z/ as in "zoo"
                    'ʃ' => 40,  // /ʃ/ as in "she"
                    'ʒ' => 41,  // /ʒ/ as in "measure"
                    'θ' => 42,  // /θ/ as in "think"
                    'ð' => 43,  // /ð/ as in "this"
                    'h' => 44,  // /h/ as in "hat"
                    'm' => 45,  // /m/ as in "mat"
                    'n' => 46,  // /n/ as in "nat"
                    'ŋ' => 47,  // /ŋ/ as in "sing"
                    'l' => 48,  // /l/ as in "lat"
                    'r' => 49,  // /r/ as in "rat"
                    'ɹ' => 49,  // alternate /r/
                    'w' => 50,  // /w/ as in "wat"
                    'j' => 51,  // /j/ as in "yes"
                    'ʤ' => 52,  // /ʤ/ as in "judge"
                    'ʧ' => 53,  // /ʧ/ as in "church"
                    
                    // Stress markers
                    'ˈ' => 2,   // primary stress
                    'ˌ' => 3,   // secondary stress
                    
                    // Punctuation and special
                    ' ' => 1,   // word boundary
                    '.' => 4,   // period
                    ',' => 5,   // comma
                    '!' => 6,   // exclamation
                    '?' => 6,   // question
                    
                    // Default: treat as word boundary
                    _ => 1,
                };
                (token, 1)
            };
            
            tokens.push(token);
            i += advance;
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
        println!("  Token IDs: {:?}", &tokens[..tokens.len().min(20)]);
        
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
