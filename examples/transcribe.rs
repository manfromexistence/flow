// Example: Basic audio transcription

use anyhow::Result;
use flow::MoonshineSTT;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🎤 Flow - Audio Transcription Example");
    
    let text = MoonshineSTT::transcribe("tests/fixtures/audio.mp3")?;
    
    println!("📝 Transcription: {}", text);
    
    Ok(())
}
