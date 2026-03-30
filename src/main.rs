mod llm;
mod stt;
mod voice;

use anyhow::Result;
use llm::LocalLlm;
use std::env;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // Check for arguments
    if args.len() > 1 {
        match args[1].as_str() {
            "--models" | "-m" => {
                voice::VoicePipeline::list_available_models()?;
                return Ok(());
            }
            "--transcribe" => {
                let audio_file = if args.len() > 2 {
                    args[2].as_str()
                } else {
                    "audio.mp3"
                };
                
                println!("Wispr Flow - Speech-to-Text Transcription");
                println!("Audio file: {}", audio_file);
                println!();
                
                if !std::path::Path::new(audio_file).exists() {
                    eprintln!("Error: Audio file not found: {}", audio_file);
                    std::process::exit(1);
                }
                
                let start = std::time::Instant::now();
                let transcript = stt::AudioAnalyzer::analyze_and_transcribe(audio_file)?;
                let elapsed = start.elapsed();
                
                println!("\n═══════════════════════════════════════════════════════════");
                println!("  FINAL TRANSCRIPT");
                println!("═══════════════════════════════════════════════════════════");
                println!("{}", transcript);
                println!("\nProcessing time: {:.2}s", elapsed.as_secs_f64());
                println!("═══════════════════════════════════════════════════════════");
                
                return Ok(());
            }
            "--wispr" => {
                let audio_file = if args.len() > 2 {
                    args[2].as_str()
                } else {
                    "audio.mp3"
                };
                
                println!("Wispr Flow - Full Pipeline");
                println!("Audio file: {}", audio_file);
                println!();
                
                if !std::path::Path::new(audio_file).exists() {
                    eprintln!("Error: Audio file not found: {}", audio_file);
                    std::process::exit(1);
                }
                
                // Step 1: Transcribe
                println!("Step 1: Speech-to-Text");
                let transcript = stt::AudioAnalyzer::analyze_and_transcribe(audio_file)?;
                
                // Step 2: Enhance with LLM
                println!("\nStep 2: LLM Enhancement");
                let llm = LocalLlm::new();
                llm.initialize().await?;
                
                let enhancement_prompt = format!(
                    "Clean up this transcription by:\n\
                    1. Removing filler words (um, uh, like)\n\
                    2. Adding proper punctuation\n\
                    3. Fixing grammar\n\
                    4. Formatting appropriately\n\n\
                    Transcription: {}\n\n\
                    Output only the cleaned text, nothing else.",
                    transcript
                );
                
                let (enhanced, _) = llm.generate_with_metrics(&enhancement_prompt).await?;
                let cleaned = clean_response(&enhanced);
                
                println!("\n═══════════════════════════════════════════════════════════");
                println!("  WISPR FLOW OUTPUT");
                println!("═══════════════════════════════════════════════════════════");
                println!("Original: {}", transcript);
                println!("\nEnhanced: {}", cleaned);
                println!("═══════════════════════════════════════════════════════════");
                
                return Ok(());
            }
            "--help" | "-h" => {
                print_help();
                return Ok(());
            }
            _ => {
                // Treat any other argument as a prompt
                run_cli_mode(&args[1..]).await?;
                return Ok(());
            }
        }
    }

    // Default: CLI mode for AI agents
    run_cli_mode(&[]).await
}

fn print_help() {
    println!("Wispr Flow Clone - AI Voice Assistant");
    println!();
    println!("USAGE:");
    println!("  cargo run                              Interactive CLI (default)");
    println!("  cargo run \"prompt\"                     Single LLM query");
    println!("  cargo run -- --transcribe [file]       Transcribe audio (STT only)");
    println!("  cargo run -- --wispr [file]            Full Wispr Flow (STT + LLM)");
    println!("  cargo run -- --models                  List available models");
    println!("  cargo run -- --help                    Show this help");
    println!();
    println!("WISPR FLOW EXAMPLES:");
    println!("  cargo run -- --transcribe audio.mp3   # Basic transcription");
    println!("  cargo run -- --wispr audio.mp3        # Full enhancement pipeline");
    println!();
    println!("LLM EXAMPLES:");
    println!("  cargo run \"What is Rust?\"              # Single query");
    println!("  cargo run                              # Interactive mode");
    println!();
    println!("MODEL INFO:");
    println!("  LLM: Qwen 3.5 0.8B Q4_K_M (~3.4GB RAM)");
    println!("  STT: Moonshine Tiny ONNX (~27M params)");
    println!("  TTS: Kokoro v1.0 INT8 (planned)");
    println!("  Context: 32K tokens");
    println!();
    println!("WISPR FLOW FEATURES:");
    println!("  ✓ Speech-to-text transcription");
    println!("  ✓ Automatic filler word removal");
    println!("  ✓ Punctuation and formatting");
    println!("  ✓ LLM-powered text enhancement");
    println!("  ⧗ Voice commands (coming soon)");
    println!("  ⧗ Real-time streaming (coming soon)");
}

async fn run_cli_mode(args: &[String]) -> Result<()> {
    let llm = LocalLlm::new();
    
    // Show system info
    print_system_info();
    
    print!("\nInitializing GGUF model...");
    io::stdout().flush()?;
    let init_start = std::time::Instant::now();
    llm.initialize().await?;
    let init_time = init_start.elapsed();
    println!(" ✓ ({:.2}s)", init_time.as_secs_f64());

    // If prompt provided as argument, use it
    if !args.is_empty() {
        let prompt = args.join(" ");
        println!("\n> {}\n", prompt);
        
        let (response, metrics) = llm.generate_with_metrics(&prompt).await?;
        let cleaned = clean_response(&response);
        println!("{}\n", cleaned);
        print_metrics(&metrics);
        return Ok(());
    }

    // Check if input is piped
    if atty::isnt(atty::Stream::Stdin) {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let prompt = input.trim();
        
        if !prompt.is_empty() {
            println!("\n> {}\n", prompt);
            let (response, metrics) = llm.generate_with_metrics(prompt).await?;
            let cleaned = clean_response(&response);
            println!("{}\n", cleaned);
            print_metrics(&metrics);
        }
        return Ok(());
    }

    // Interactive mode
    println!("\nInteractive mode. Type 'exit' or 'quit' to end.\n");
    
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let prompt = input.trim();

        if prompt.is_empty() {
            continue;
        }

        if prompt == "exit" || prompt == "quit" {
            println!("Goodbye!");
            break;
        }

        println!();
        let (response, metrics) = llm.generate_with_metrics(prompt).await?;
        let cleaned = clean_response(&response);
        println!("{}\n", cleaned);
        print_metrics(&metrics);
    }

    Ok(())
}

fn print_system_info() {
    use sysinfo::System;
    
    let mut sys = System::new_all();
    sys.refresh_all();
    
    println!("═══════════════════════════════════════════════════════════");
    println!("  SYSTEM INFORMATION");
    println!("═══════════════════════════════════════════════════════════");
    
    // CPU info
    if let Some(cpu) = sys.cpus().first() {
        println!("CPU: {}", cpu.brand());
    }
    println!("Physical Cores: {}", sys.physical_core_count().unwrap_or(0));
    println!("Logical Cores: {}", sys.cpus().len());
    
    // Memory info
    let total_mem = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let used_mem = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let available_mem = sys.available_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    
    println!("Total RAM: {:.2} GB", total_mem);
    println!("Used RAM: {:.2} GB ({:.1}%)", used_mem, (used_mem / total_mem) * 100.0);
    println!("Available RAM: {:.2} GB", available_mem);
    
    println!("═══════════════════════════════════════════════════════════");
}

fn print_metrics(metrics: &llm::GenerationMetrics) {
    println!("───────────────────────────────────────────────────────────");
    println!("  PERFORMANCE METRICS");
    println!("───────────────────────────────────────────────────────────");
    println!("Prompt Tokens: {}", metrics.prompt_tokens);
    println!("Generated Tokens: {}", metrics.generated_tokens);
    println!("Total Tokens: {}", metrics.prompt_tokens + metrics.generated_tokens);
    println!();
    println!("Prompt Eval Time: {:.2}s", metrics.prompt_eval_time_ms as f64 / 1000.0);
    println!("Generation Time: {:.2}s", metrics.generation_time_ms as f64 / 1000.0);
    println!("Total Time: {:.2}s", metrics.total_time_ms as f64 / 1000.0);
    println!();
    println!("Generation Speed: {:.2} tokens/sec", metrics.tokens_per_second);
    
    // Calculate RAM usage estimate
    let base_model_ram = 3.4; // GB for 0.8B Q4_K_M
    let context_ram = (metrics.prompt_tokens + metrics.generated_tokens) as f64 * 0.000002; // Rough estimate
    let estimated_ram = base_model_ram + context_ram;
    
    println!("Estimated RAM Usage: {:.2} GB", estimated_ram);
    println!("───────────────────────────────────────────────────────────");
}

fn clean_response(response: &str) -> String {
    // Remove <think> tags and their content
    let mut cleaned = response.to_string();
    
    // Remove <think>...</think> blocks
    while let Some(start) = cleaned.find("<think>") {
        if let Some(end) = cleaned.find("</think>") {
            cleaned.replace_range(start..=end + 7, "");
        } else {
            break;
        }
    }
    
    // Trim extra whitespace
    cleaned.trim().to_string()
}
