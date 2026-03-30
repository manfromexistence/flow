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
    
    print!("Initializing GGUF model...");
    io::stdout().flush()?;
    llm.initialize().await?;
    println!(" ✓");

    // If prompt provided as argument, use it
    if !args.is_empty() {
        let prompt = args.join(" ");
        println!("\n> {}\n", prompt);
        
        let response = llm.generate(&prompt).await?;
        let cleaned = clean_response(&response);
        println!("{}\n", cleaned);
        return Ok(());
    }

    // Check if input is piped
    if atty::isnt(atty::Stream::Stdin) {
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let prompt = input.trim();
        
        if !prompt.is_empty() {
            println!("\n> {}\n", prompt);
            let response = llm.generate(prompt).await?;
            let cleaned = clean_response(&response);
            println!("{}\n", cleaned);
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
        let response = llm.generate(prompt).await?;
        let cleaned = clean_response(&response);
        println!("{}\n", cleaned);
    }

    Ok(())
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
