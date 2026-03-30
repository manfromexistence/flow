use anyhow::Result;
use crate::models::{LocalLlm, MoonshineSTT, KokoroTTS};
use crate::cli::Command;

/// Execute CLI command
pub async fn execute(command: Command) -> Result<()> {
    match command {
        Command::Transcribe { file } => {
            println!("🎤 Transcribing: {}", file);
            let text = MoonshineSTT::transcribe(&file)?;
            println!("📝 Result: {}", text);
        }
        
        Command::Wispr { file } => {
            println!("🎤 Wispr Flow: {}", file);
            
            // STT
            let raw_text = MoonshineSTT::transcribe(&file)?;
            println!("📝 Raw: {}", raw_text);
            
            // LLM Enhancement
            let llm = LocalLlm::new();
            llm.initialize().await?;
            
            let prompt = format!(
                "Clean up this transcription. Remove filler words, add punctuation, \
                 and capitalize properly. Only output the cleaned text:\n\n{}",
                raw_text
            );
            let (enhanced, _) = llm.generate_with_metrics(&prompt).await?;
            println!("✨ Enhanced: {}", enhanced);
        }
        
        Command::Speak { text } => {
            println!("🔊 Speaking: {}", text);
            let tts = KokoroTTS::new()?;
            let audio = tts.synthesize(&text)?;
            println!("✅ Generated {} samples", audio.len());
        }
        
        Command::Interactive => {
            println!("🎙️ Flow - Interactive Voice Assistant");
            println!("Commands:");
            println!("  --transcribe <file>  : Transcribe audio file");
            println!("  --wispr <file>       : Full pipeline (STT + LLM)");
            println!("  --speak <text>       : Text-to-speech");
        }
    }
    
    Ok(())
}
