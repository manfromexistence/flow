use anyhow::Result;
use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::{AddBos, LlamaModel};
use std::io::{self, Write};
use std::time::Instant;
use sysinfo::System;
use tracing::{info, error, Level};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .init();

    info!("🤖 Edith AI Chat CLI - Qwen 3.5 0.8B");
    info!("💬 Streaming chat interface with real LLM\n");

    // Initialize system monitor
    let mut sys = System::new_all();
    sys.refresh_all();

    // Initialize llama backend
    info!("🔧 Initializing llama.cpp backend...");
    let mut backend = LlamaBackend::init()?;
    backend.void_logs();
    
    // Load model
    let model_path = "models/llm/Qwen3.5-0.8B-Q4_K_M.gguf";
    let model_params = LlamaModelParams::default();
    let model = LlamaModel::load_from_file(&backend, model_path, &model_params)?;
    
    let n_ctx_train = model.n_ctx_train();
    let n_vocab = model.n_vocab();
    
    info!("✅ Model loaded successfully");
    info!("   Model: Qwen 3.5 0.8B Q4_K_M");
    info!("   Training context: {} tokens", n_ctx_train);
    info!("   Vocabulary size: {} tokens", n_vocab);
    
    // Create context
    let ctx_params = LlamaContextParams::default();
    let mut ctx = model.new_context(&backend, ctx_params)?;
    info!("✅ Context created\n");

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
            info!("👋 Goodbye!");
            break;
        }

        // Start timing
        let start_time = Instant::now();
        sys.refresh_all();
        let start_memory = sys.used_memory();

        // This CLI currently handles each prompt independently, so reset KV cache
        // to avoid position/cache conflicts across turns.
        ctx.clear_kv_cache();
        
        // Tokenize input with BOS token
        let prompt = format!("<|im_start|>user\n{}<|im_end|>\n<|im_start|>assistant\n", input);
        let tokens = match model.str_to_token(&prompt, AddBos::Always) {
            Ok(t) => t,
            Err(e) => {
                error!("Failed to tokenize: {}", e);
                continue;
            }
        };
        
        info!("🔢 Input tokens: {}", tokens.len());
        print!("🤖 Qwen: ");
        io::stdout().flush()?;
        if tokens.is_empty() {
            error!("Tokenization produced zero tokens");
            continue;
        }

        // Decode prompt as sequence 0, with logits on the last token.
        let mut prompt_batch = LlamaBatch::new(tokens.len(), 1);
        if let Err(e) = prompt_batch.add_sequence(&tokens, 0, false) {
            error!("Failed to add prompt tokens to batch: {}", e);
            continue;
        }
        if let Err(e) = ctx.decode(&mut prompt_batch) {
            error!("Failed to decode: {}", e);
            continue;
        }
        
        // Generate response
        let mut generated_tokens = 0;
        let max_tokens = 256;
        let mut response = String::new();
        let mut n_cur = tokens.len() as i32;
        let mut generation_batch = LlamaBatch::new(1, 1);
        
        for _ in 0..max_tokens {
            // Get logits from last position
            let logits = ctx.candidates();
            
            // Simple greedy sampling - just take the highest probability token
            let mut max_prob = f32::NEG_INFINITY;
            let mut best_token = None;
            
            for candidate in logits {
                if candidate.logit() > max_prob {
                    max_prob = candidate.logit();
                    best_token = Some(candidate.id());
                }
            }
            
            let new_token_id = match best_token {
                Some(token) => token,
                None => {
                    error!("No candidates available");
                    break;
                }
            };
            
            // Check for EOS or Qwen special tokens (im_end, im_start)
            let token_id_i32 = new_token_id.0; // Extract i32 from LlamaToken
            let eos_tokens = [151643_i32, 151645_i32]; // Qwen chat template tokens
            if model.is_eog_token(new_token_id) || eos_tokens.contains(&token_id_i32) {
                break;
            }
            
            // Convert token to bytes then string
            #[allow(deprecated)]
            let piece_buf = model.token_to_bytes(new_token_id, llama_cpp_2::model::Special::Tokenize)?;
            let piece = String::from_utf8_lossy(&piece_buf).to_string();
            
            print!("{}", piece);
            io::stdout().flush()?;
            
            response.push_str(&piece);
            generated_tokens += 1;
            
            // Prepare next batch
            generation_batch.clear();
            if let Err(e) = generation_batch.add(new_token_id, n_cur, &[0], true) {
                error!("Failed to add token: {}", e);
                break;
            }
            n_cur += 1;
            
            // Decode
            if let Err(e) = ctx.decode(&mut generation_batch) {
                error!("Failed to decode: {}", e);
                break;
            }
        }
        
        println!(); // New line after response
        
        // Calculate metrics
        let elapsed = start_time.elapsed();
        let elapsed_secs = elapsed.as_secs_f64();
        let tokens_per_sec = if elapsed_secs > 0.0 {
            generated_tokens as f64 / elapsed_secs
        } else {
            0.0
        };
        
        sys.refresh_all();
        let end_memory = sys.used_memory();
        let memory_delta = (end_memory.saturating_sub(start_memory)) / 1024 / 1024; // MB
        
        // Display metrics
        println!("\n╔════════════════════════════════════════════════════════╗");
        println!("║                  PERFORMANCE METRICS                   ║");
        println!("╠════════════════════════════════════════════════════════╣");
        println!("║ Model:                 Qwen 3.5 0.8B Q4_K_M            ║");
        println!("║ Input Tokens:          {:>6}                          ║", tokens.len());
        println!("║ Generated Tokens:      {:>6}                          ║", generated_tokens);
        println!("║ Time Taken:            {:>6.2} seconds                 ║", elapsed_secs);
        println!("║ Tokens/Second:         {:>6.2} tok/s                   ║", tokens_per_sec);
        println!("║ Memory Delta:          {:>6} MB                        ║", memory_delta);
        println!("║ Total Memory:          {:>6} MB                        ║", sys.used_memory() / 1024 / 1024);
        println!("║ CPU Usage:             {:>6.1}%                         ║", sys.global_cpu_usage());
        println!("╚════════════════════════════════════════════════════════╝");
    }

    Ok(())
}
