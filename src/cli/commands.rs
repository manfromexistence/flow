use anyhow::Result;
use crate::models::{LocalLlm, MoonshineSTT, KokoroTTS};
use crate::cli::Command;

/// Execute CLI command
pub async fn execute(command: Command) -> Result<()> {
    match command {
        Command::Transcribe { file } => {
            println!("→ Transcribing: {}", file);
            let stt = MoonshineSTT::new()?;
            let text = stt.transcribe(&file)?;
            println!("✓ Result: {}", text);
        }
        
        Command::Wispr { file } => {
            println!("→ Wispr Flow: {}", file);
            
            // STT
            let stt = MoonshineSTT::new()?;
            let raw_text = stt.transcribe(&file)?;
            println!("  Raw: {}", raw_text);
            
            // LLM Enhancement
            let llm = LocalLlm::new();
            llm.initialize().await?;
            
            let prompt = format!(
                "Clean up this transcription. Remove filler words, add punctuation, \
                 and capitalize properly. Only output the cleaned text:\n\n{}",
                raw_text
            );
            let (enhanced, _) = llm.generate_with_metrics(&prompt).await?;
            println!("✓ Enhanced: {}", enhanced);
        }
        
        Command::Speak { text } => {
            println!("→ Speaking: {}", text);
            let mut tts = KokoroTTS::new_async().await?;
            tts.speak(&text)?;
        }
        
        Command::Live => {
            println!("╔═══════════════════════════════════════════════════════════╗");
            println!("║         FLOW - Live Voice Assistant Mode                 ║");
            println!("║  Microphone → STT → Enhance → TTS → Speaker              ║");
            println!("╚═══════════════════════════════════════════════════════════╝");
            println!();
            
            // Initialize models
            println!("⚙️  Initializing models...");
            let stt = MoonshineSTT::new()?;
            let mut tts = KokoroTTS::new_async().await?;
            println!("✓ Models ready!");
            println!();
            
            loop {
                // Record from microphone
                let recorder = crate::audio::MicRecorder::new();
                let audio_samples = match recorder.record_until_silence() {
                    Ok(samples) => samples,
                    Err(e) => {
                        eprintln!("✗ Recording error: {}", e);
                        continue;
                    }
                };
                
                // Save recorded audio for debugging
                let temp_file = "temp_recording.wav";
                {
                    use hound::{WavWriter, WavSpec};
                    let spec = WavSpec {
                        channels: 1,
                        sample_rate: 16000,
                        bits_per_sample: 16,
                        sample_format: hound::SampleFormat::Int,
                    };
                    let mut writer = WavWriter::create(temp_file, spec)?;
                    for &sample in &audio_samples {
                        let sample_i16 = (sample * 32767.0).clamp(-32768.0, 32767.0) as i16;
                        writer.write_sample(sample_i16)?;
                    }
                    writer.finalize()?;
                }
                
                // STT: Transcribe
                let transcript = stt.transcribe(temp_file)?;
                
                // TTS: Speak the enhanced text
                tts.speak(&transcript)?;
                
                println!("\n✓ Complete! Listening for next input...");
                println!("  (Press Ctrl+C to exit)");
                println!();
            }
        }
        
        Command::Interactive => {
            println!("→ Flow - Interactive Voice Assistant");
            println!();
            println!("Commands:");
            println!("  --transcribe <file>  : Transcribe audio file");
            println!("  --wispr <file>       : Full pipeline (STT + LLM)");
            println!("  --speak <text>       : Text-to-speech");
            println!("  --live               : Live microphone mode (VAD + STT + TTS)");
            println!();
            println!("Example:");
            println!("  cargo run -- --live");
        }
    }
    
    Ok(())
}
