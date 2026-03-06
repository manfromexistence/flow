use anyhow::Result;
use std::io::{self, Write};
use std::time::Instant;
use sysinfo::System;
use tracing::{info, Level};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🤖 Edith AI Chat CLI");
    info!("💬 Simple text-based chat interface\n");
    info!("⚠️  Note: LLM integration pending - using echo mode for now\n");

    // Initialize system monitor
    let mut sys = System::new_all();
    sys.refresh_all();

    // Chat loop
    loop {
        // Get user input
        print!("\n💬 You: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input.is_empty() {
            continue;
        }
        
        if input == "exit" || input == "quit" {
            info!("� Goodbye!");
            break;
        }

        // Start timing
        let start_time = Instant::now();
        sys.refresh_all();
        let start_memory = sys.used_memory();
        
        // Simulate processing
        print!("🤖 Edith: ");
        io::stdout().flush()?;
        
        // Echo response (placeholder for actual LLM)
        let response = format!("I heard you say: '{}'. (LLM integration coming soon!)", input);
        
        // Simulate streaming output
        for char in response.chars() {
            print!("{}", char);
            io::stdout().flush()?;
            std::thread::sleep(std::time::Duration::from_millis(10));
        }
        
        println!(); // New line after response
        
        // Calculate metrics
        let elapsed = start_time.elapsed();
        let elapsed_secs = elapsed.as_secs_f64();
        let chars_per_sec = response.len() as f64 / elapsed_secs;
        
        sys.refresh_all();
        let end_memory = sys.used_memory();
        let memory_delta = (end_memory.saturating_sub(start_memory)) / 1024 / 1024; // MB
        
        // Display metrics
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║                  PERFORMANCE METRICS                   ║");
        println!("╠════════════════════════════════════════════════════════╣");
        println!("║ Characters Generated:  {:>6}                          ║", response.len());
        println!("║ Time Taken:            {:>6.2} seconds                 ║", elapsed_secs);
        println!("║ Chars/Second:          {:>6.2} ch/s                    ║", chars_per_sec);
        println!("║ Memory Delta:          {:>6} MB                        ║", memory_delta);
        println!("║ Total Memory:          {:>6} MB                        ║", sys.used_memory() / 1024 / 1024);
        println!("║ CPU Usage:             {:>6.1}%                         ║", sys.global_cpu_usage());
        println!("╚════════════════════════════════════════════════════════╝");
    }

    Ok(())
}
