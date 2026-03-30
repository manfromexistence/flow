mod app;
mod autocomplete;
mod components;
mod effects;
mod font;
mod gruvbox;
mod input;
mod llm;
mod modal;
mod perf;
mod render; // Now a module directory with submodules
// mod screens; // Unused - only for standalone screen demos
mod splash;
mod tachyonfx;
mod theme;

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
            "--tui" => {
                // TUI mode (optional, for human use)
                let mut app = app::ChatApp::new();
                app.run().await?;
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
    println!("Local GGUF Model CLI - AI Agent Testing Interface");
    println!();
    println!("USAGE:");
    println!("  cargo run                              Interactive CLI (default)");
    println!("  cargo run \"prompt\"                     Single query");
    println!("  cargo run -- --tui                     Launch TUI (human mode)");
    println!("  cargo run -- --help                    Show this help");
    println!();
    println!("EXAMPLES:");
    println!("  cargo run \"What is Rust?\"              # Single query");
    println!("  cargo run                              # Interactive mode");
    println!("  echo \"Explain async\" | cargo run      # Pipe input");
    println!();
    println!("MODEL INFO:");
    println!("  Path: models/llm/Qwen3.5-0.8B-Q4_K_M.gguf");
    println!("  Context: 32K tokens");
    println!("  RAM: ~3.4GB required");
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
