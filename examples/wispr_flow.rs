// Example: Full Wispr Flow pipeline (STT + LLM enhancement)

use anyhow::Result;
use flow::{MoonshineSTT, LocalLlm};

#[tokio::main]
async fn main() -> Result<()> {
    println!("✨ Flow - Wispr Flow Pipeline Example");
    
    // Step 1: Transcribe audio
    println!("🎤 Transcribing audio...");
    let raw_text = MoonshineSTT::transcribe("tests/fixtures/audio.mp3")?;
    println!("📝 Raw: {}", raw_text);
    
    // Step 2: Enhance with LLM
    println!("🧠 Enhancing with LLM...");
    let llm = LocalLlm::new();
    llm.initialize().await?;
    
    let prompt = format!(
        "Clean up this transcription. Remove filler words, add punctuation:\n\n{}",
        raw_text
    );
    let (enhanced, _) = llm.generate_with_metrics(&prompt).await?;
    println!("✨ Enhanced: {}", enhanced);
    
    Ok(())
}
