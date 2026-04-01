/// Complete Wispr Flow Pipeline Example
/// 
/// This demonstrates the full voice assistant pipeline:
/// 1. STT: Transcribe audio using Moonshine v2
/// 2. LLM: Enhance text using Qwen 3.5 (Wispr Flow style)
/// 3. TTS: Synthesize enhanced text using Kokoro
///
/// Usage:
///   cargo run --example wispr_flow_complete

use anyhow::Result;
use flow::models::{MoonshineSTT, QwenLLM, KokoroTTS};

fn main() -> Result<()> {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║         FLOW - Complete Wispr Flow Pipeline              ║");
    println!("║  STT (Moonshine) → LLM (Qwen) → TTS (Kokoro)            ║");
    println!("╚═══════════════════════════════════════════════════════════╝\n");
    
    let audio_path = "tests/fixtures/audio.mp3";
    
    // Step 1: Speech-to-Text (Moonshine v2 Tiny)
    println!("═══════════════════════════════════════════════════════════");
    println!("  STEP 1: SPEECH-TO-TEXT (Moonshine v2 Tiny)");
    println!("═══════════════════════════════════════════════════════════");
    
    let raw_transcript = if MoonshineSTT::is_available() {
        let stt = MoonshineSTT::new()?;
        stt.transcribe(audio_path)?
    } else {
        println!("⚠ Moonshine not available, using mock transcript");
        "hello mike testing one two three hello".to_string()
    };
    
    println!("\n📝 Raw Transcript: \"{}\"", raw_transcript);
    
    // Step 2: LLM Enhancement (Qwen 3.5 0.8B)
    println!("\n═══════════════════════════════════════════════════════════");
    println!("  STEP 2: TEXT ENHANCEMENT (Qwen 3.5 0.8B)");
    println!("═══════════════════════════════════════════════════════════");
    
    let enhanced_text = if QwenLLM::is_available() {
        println!("🧠 Loading Qwen 3.5 0.8B...");
        let llm = QwenLLM::new()?;
        
        let prompt = format!(
            "You are a text enhancement AI like Wispr Flow. \
            Your job is to take raw speech transcripts and make them polished and professional.\n\n\
            Rules:\n\
            1. Remove filler words (um, uh, like, you know)\n\
            2. Add proper punctuation and capitalization\n\
            3. Fix grammar and make it sound natural\n\
            4. Keep the meaning exactly the same\n\
            5. Output ONLY the enhanced text, nothing else\n\n\
            Raw transcript: \"{}\"\n\n\
            Enhanced text:",
            raw_transcript
        );
        
        println!("💭 Enhancing with LLM...");
        let response = llm.generate(&prompt)?;
        
        // Extract just the enhanced text (remove any extra commentary)
        let enhanced = response.lines()
            .find(|line| !line.trim().is_empty())
            .unwrap_or(&response)
            .trim()
            .to_string();
        
        enhanced
    } else {
        println!("⚠ Qwen not available, using basic enhancement");
        enhance_text_basic(&raw_transcript)
    };
    
    println!("\n✨ Enhanced Text: \"{}\"", enhanced_text);
    
    // Step 3: Text-to-Speech (Kokoro 82M)
    println!("\n═══════════════════════════════════════════════════════════");
    println!("  STEP 3: TEXT-TO-SPEECH (Kokoro 82M)");
    println!("═══════════════════════════════════════════════════════════");
    
    if KokoroTTS::is_available() {
        let tts = KokoroTTS::new()?;
        let audio = tts.synthesize(&enhanced_text)?;
        
        let output_path = "output_wispr_flow.wav";
        tts.save_wav(&audio, output_path)?;
        
        println!("\n🎵 Audio saved to: {}", output_path);
    } else {
        println!("⚠ Kokoro TTS not available");
        println!("   Run: ./scripts/download_ultralight_models.ps1");
    }
    
    // Summary
    println!("\n╔═══════════════════════════════════════════════════════════╗");
    println!("║                    PIPELINE COMPLETE                      ║");
    println!("╚═══════════════════════════════════════════════════════════╝");
    println!("\n📊 Summary:");
    println!("   Input:    {} (audio)", audio_path);
    println!("   STT:      \"{}\"", raw_transcript);
    println!("   Enhanced: \"{}\"", enhanced_text);
    println!("   Output:   output_wispr_flow.wav");
    
    println!("\n✅ Wispr Flow pipeline completed successfully!");
    
    Ok(())
}

/// Basic text enhancement (fallback when LLM not available)
fn enhance_text_basic(text: &str) -> String {
    // Remove filler words
    let fillers = ["um", "uh", "like", "you know", "sort of", "kind of"];
    let mut enhanced = text.to_lowercase();
    
    for filler in &fillers {
        enhanced = enhanced.replace(&format!(" {} ", filler), " ");
    }
    
    // Clean up spaces
    enhanced = enhanced.split_whitespace().collect::<Vec<_>>().join(" ");
    
    // Add punctuation
    if !enhanced.is_empty() && !enhanced.ends_with('.') {
        enhanced.push('.');
    }
    
    // Capitalize first letter
    if let Some(first_char) = enhanced.chars().next() {
        enhanced = first_char.to_uppercase().collect::<String>() + &enhanced[1..];
    }
    
    enhanced
}
