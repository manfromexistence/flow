use anyhow::Result;
use ort::session::Session;
use ort::value::Value;
use ndarray::{Array1, Array2, ArrayBase, IxDyn, OwnedRepr};
use ndarray_npy::NpzReader;
use std::collections::HashMap;
use std::fs::File;
use std::path::Path;

/// Kokoro TTS using local ONNX models (pure Rust, no Python!)
pub struct KokoroTTS {
    session: Session,
    vocab: HashMap<String, i64>,
    voices: HashMap<String, Vec<f32>>,
    default_voice: String,
}

impl KokoroTTS {
    pub fn is_available() -> bool {
        Path::new("models/tts/kokoro-v1.0.int8.onnx").exists() &&
        Path::new("models/tts/voices-v1.0.bin").exists() &&
        Path::new("models/tts/config.json").exists()
    }
    
    pub async fn new_async() -> Result<Self> {
        println!("⚙️  Initializing Kokoro TTS...");
        
        // Load ONNX model
        let session = Session::builder()?
            .commit_from_file("models/tts/kokoro-v1.0.int8.onnx")?;
        
        // Load vocabulary from config.json
        let config_str = std::fs::read_to_string("models/tts/config.json")?;
        let config: serde_json::Value = serde_json::from_str(&config_str)?;
        
        let mut vocab = HashMap::new();
        if let Some(vocab_obj) = config["vocab"].as_object() {
            for (k, v) in vocab_obj {
                if let Some(id) = v.as_i64() {
                    vocab.insert(k.clone(), id);
                }
            }
        }
        
        println!("  Loaded {} vocabulary tokens", vocab.len());
        
        // Load voice embeddings from NPZ file
        let voices = Self::load_voices("models/tts/voices-v1.0.bin")?;
        
        println!("  Loaded {} voices", voices.len());
        
        // Use af_sky as default voice
        let default_voice = "af_sky".to_string();
        if !voices.contains_key(&default_voice) {
            return Err(anyhow::anyhow!("Default voice 'af_sky' not found in voices file"));
        }
        
        println!("✓ Kokoro TTS ready");
        
        Ok(Self { session, vocab, voices, default_voice })
    }
    
    fn load_voices(path: &str) -> Result<HashMap<String, Vec<f32>>> {
        let file = File::open(path)?;
        let mut npz = NpzReader::new(file)?;
        let mut voices = HashMap::new();

        for name in npz.names()? {
            // Read the voice array
            let arr: ArrayBase<OwnedRepr<f32>, IxDyn> = npz.by_name(&name)?;
            
            // Voice arrays are shaped (512, 1, 256) - 512 length variants, 1 batch, 256 features
            // We need to extract the first variant (index 0) which gives us (1, 256)
            let shape = arr.shape();
            if shape.len() == 3 && shape[1] == 1 && shape[2] == 256 {
                // Extract the first length variant (index 0)
                let start = 0;
                let end = 256;
                let data = arr.as_slice()
                    .ok_or_else(|| anyhow::anyhow!("Failed to get slice for voice {}", name))?
                    [start..end]
                    .to_vec();
                
                // Remove .npy extension from name
                let voice_name = name.trim_end_matches(".npy").to_string();
                voices.insert(voice_name, data);
            }
        }

        Ok(voices)
    }
    
    pub fn new() -> Result<Self> {
        Err(anyhow::anyhow!("Use new_async() instead - TTS requires async initialization"))
    }
    
    pub fn synthesize(&mut self, text: &str) -> Result<Vec<f32>> {
        println!("\n→ Synthesizing speech with Kokoro...");
        println!("  Text: \"{}\"", text);
        
        // Convert text to phonemes using voice-g2p
        let phonemes = voice_g2p::english_to_phonemes(text)
            .map_err(|e| anyhow::anyhow!("Phonemization failed: {:?}", e))?;
        
        println!("  Phonemes: {}", phonemes);
        
        // Convert phonemes to token IDs
        let mut token_ids = vec![0i64]; // Start with pad token
        
        for c in phonemes.chars() {
            let char_str = c.to_string();
            if let Some(&id) = self.vocab.get(&char_str) {
                token_ids.push(id);
            } else {
                println!("  Warning: Unknown phoneme '{}' (skipping)", c);
            }
        }
        
        token_ids.push(0); // End with pad token
        
        println!("  Token IDs: {:?}", token_ids);
        println!("  Token count: {}", token_ids.len());
        
        // Get the voice embedding for the default voice
        let style = self.voices.get(&self.default_voice)
            .ok_or_else(|| anyhow::anyhow!("Voice '{}' not found", self.default_voice))?
            .clone();
        
        println!("  Using voice: {}", self.default_voice);
        
        // Create style tensor (1, 256)
        let style_array = Array2::from_shape_vec((1, 256), style)?;
        
        // Create input tensors
        let tokens_len = token_ids.len();
        let tokens_array = Array2::from_shape_vec((1, tokens_len), token_ids)?;
        let speed_array = Array1::from_vec(vec![1.0f32]);
        
        // Run inference (using names from the Python example)
        let outputs = self.session.run(ort::inputs![
            "tokens" => Value::from_array(tokens_array)?,
            "style" => Value::from_array(style_array)?,
            "speed" => Value::from_array(speed_array)?
        ])?;
        
        // Extract audio
        let audio_tensor = &outputs[0];
        let (_shape, audio_data) = audio_tensor.try_extract_tensor::<f32>()?;
        let audio: Vec<f32> = audio_data.to_vec();
        
        println!("✓ Generated {} samples ({:.2}s at 24kHz)", 
            audio.len(), audio.len() as f64 / 24000.0);
        
        Ok(audio)
    }
    
    pub fn speak(&mut self, text: &str) -> Result<()> {
        let audio = self.synthesize(text)?;
        
        // Check if audio is silent (all zeros or near-zero)
        let max_amplitude = audio.iter().map(|&x| x.abs()).fold(0.0f32, f32::max);
        let non_zero_count = audio.iter().filter(|&&x| x.abs() > 0.001).count();
        
        println!("  Audio stats:");
        println!("    Max amplitude: {:.4}", max_amplitude);
        println!("    Non-zero samples: {} / {} ({:.1}%)", 
            non_zero_count, audio.len(), 
            (non_zero_count as f64 / audio.len() as f64) * 100.0);
        
        if max_amplitude < 0.001 {
            println!("  ⚠️  WARNING: Audio appears to be silent!");
        }
        
        // Save to output.wav in root
        self.save_wav(&audio, "output.wav")?;
        
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
            let sample_i16 = (sample.clamp(-1.0, 1.0) * 32767.0) as i16;
            writer.write_sample(sample_i16)?;
        }
        
        writer.finalize()?;
        println!("✓ Saved audio to: {}", path);
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_tts_availability() {
        assert!(KokoroTTS::is_available());
    }
}
