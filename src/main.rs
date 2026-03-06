use anyhow::Result;
use colored::Colorize;
use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::{AddBos, LlamaModel};
use llama_cpp_2::sampling::LlamaSampler;
use std::collections::HashSet;
use std::io::{self, Write};
use std::time::{Instant, SystemTime, UNIX_EPOCH};
use sysinfo::System;

const MODEL_PATH: &str = "models/llm/Qwen3.5-0.8B-Q4_K_M.gguf";
const MODEL_NAME: &str = "Qwen 3.5 0.8B Q4_K_M";
const THINK_OPEN_TAG: &str = "<think>";
const THINK_CLOSE_TAG: &str = "</think>";
const INFERENCE_CONTEXT_TOKENS: u32 = 32768;
const DEFAULT_MAX_TOKENS: usize = INFERENCE_CONTEXT_TOKENS as usize;
const HARD_MAX_TOKENS: usize = INFERENCE_CONTEXT_TOKENS as usize;
const REPETITION_SUFFIX_NGRAM: usize = 24;
const REPETITION_SUFFIX_REPEATS: usize = 6;
const REPETITION_TEXT_SUFFIX_BYTES: usize = 192;
const REPETITION_TEXT_SUFFIX_REPEATS: usize = 3;
const DIVERSITY_WINDOW_TOKENS: usize = 256;
const DIVERSITY_MIN_RATIO: f32 = 0.22;
const DEFAULT_MAX_GENERATION_SECONDS: u64 = 60;
const HARD_MAX_GENERATION_SECONDS: u64 = 900;
const SAMPLER_REPEAT_LAST_N: i32 = 256;
const SAMPLER_REPEAT_PENALTY: f32 = 1.12;
const SAMPLER_FREQUENCY_PENALTY: f32 = 0.0;
const SAMPLER_PRESENCE_PENALTY: f32 = 0.0;
const SAMPLER_DRY_MULTIPLIER: f32 = 0.8;
const SAMPLER_DRY_BASE: f32 = 1.75;
const SAMPLER_DRY_ALLOWED_LENGTH: i32 = 2;
const SAMPLER_DRY_PENALTY_LAST_N: i32 = 256;
const SAMPLER_TOP_K: i32 = 40;
const SAMPLER_TOP_P: f32 = 0.9;
const SAMPLER_MIN_P: f32 = 0.05;
const SAMPLER_TEMPERATURE: f32 = 0.75;

// Parcel-inspired palette
const COLOR_PRIMARY: (u8, u8, u8) = (122, 92, 255);
const COLOR_ACCENT: (u8, u8, u8) = (56, 189, 248);
const COLOR_MUTED: (u8, u8, u8) = (148, 163, 184);
const COLOR_TEXT: (u8, u8, u8) = (226, 232, 240);
const COLOR_SUCCESS: (u8, u8, u8) = (74, 222, 128);
const COLOR_WARNING: (u8, u8, u8) = (251, 191, 36);
const COLOR_ERROR: (u8, u8, u8) = (248, 113, 113);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RenderSection {
    None,
    Answer,
    Thinking,
}

fn has_repeated_text_suffix(text: &str, chunk_size_bytes: usize, repeats: usize) -> bool {
    if chunk_size_bytes == 0 || repeats < 2 {
        return false;
    }

    let bytes = text.as_bytes();
    let needed_len = chunk_size_bytes.saturating_mul(repeats);
    if bytes.len() < needed_len {
        return false;
    }

    let start = bytes.len() - needed_len;
    let reference = &bytes[start..start + chunk_size_bytes];

    (1..repeats).all(|i| {
        let chunk_start = start + i * chunk_size_bytes;
        let chunk_end = chunk_start + chunk_size_bytes;
        &bytes[chunk_start..chunk_end] == reference
    })
}

fn sampler_seed() -> u32 {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos() as u64)
        .unwrap_or(0xA5A5_5A5A_u64);
    nanos as u32
}

struct StreamRenderer {
    pending: String,
    thinking_buffer: String,
    in_thinking: bool,
    show_thinking: bool,
    active_section: RenderSection,
    thinking_chars: usize,
    answer_chars: usize,
}

impl StreamRenderer {
    fn new(show_thinking: bool) -> Self {
        Self {
            pending: String::new(),
            thinking_buffer: String::new(),
            in_thinking: false,
            show_thinking,
            active_section: RenderSection::None,
            thinking_chars: 0,
            answer_chars: 0,
        }
    }

    fn ingest_piece(&mut self, piece: &str) -> io::Result<()> {
        self.pending.push_str(piece);
        self.consume_pending(false)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.consume_pending(true)
    }

    fn thinking_chars(&self) -> usize {
        self.thinking_chars
    }

    fn answer_chars(&self) -> usize {
        self.answer_chars
    }

    fn consume_pending(&mut self, final_flush: bool) -> io::Result<()> {
        loop {
            if self.in_thinking {
                if let Some(end_idx) = self.pending.find(THINK_CLOSE_TAG) {
                    let chunk = self.take_prefix(end_idx);
                    self.thinking_buffer.push_str(&chunk);
                    self.pending.drain(..THINK_CLOSE_TAG.len());
                    self.in_thinking = false;
                    self.flush_thinking_block()?;
                    continue;
                }

                let keep = if final_flush {
                    0
                } else {
                    THINK_CLOSE_TAG.len().saturating_sub(1)
                };
                let safe_len = self.pending.len().saturating_sub(keep);
                if safe_len == 0 {
                    break;
                }
                let chunk = self.take_prefix(safe_len);
                self.thinking_buffer.push_str(&chunk);
                continue;
            }

            if let Some(start_idx) = self.pending.find(THINK_OPEN_TAG) {
                let chunk = self.take_prefix(start_idx);
                self.print_answer_chunk(&chunk)?;
                self.pending.drain(..THINK_OPEN_TAG.len());
                self.in_thinking = true;
                continue;
            }

            let keep = if final_flush {
                0
            } else {
                THINK_OPEN_TAG.len().saturating_sub(1)
            };
            let safe_len = self.pending.len().saturating_sub(keep);
            if safe_len == 0 {
                break;
            }
            let chunk = self.take_prefix(safe_len);
            self.print_answer_chunk(&chunk)?;
        }

        if final_flush && self.in_thinking {
            self.in_thinking = false;
            self.flush_thinking_block()?;
        }

        Ok(())
    }

    fn print_answer_chunk(&mut self, chunk: &str) -> io::Result<()> {
        if chunk.is_empty() {
            return Ok(());
        }

        self.answer_chars += chunk.chars().count();
        self.begin_section(RenderSection::Answer)?;
        print!("{}", chunk.truecolor(COLOR_TEXT.0, COLOR_TEXT.1, COLOR_TEXT.2));
        io::stdout().flush()
    }

    fn flush_thinking_block(&mut self) -> io::Result<()> {
        if self.thinking_buffer.is_empty() {
            return Ok(());
        }

        let thinking_text = self.thinking_buffer.trim().to_string();
        self.thinking_buffer.clear();

        if thinking_text.is_empty() {
            return Ok(());
        }

        self.thinking_chars += thinking_text.chars().count();

        if !self.show_thinking {
            return Ok(());
        }

        self.begin_section(RenderSection::Thinking)?;
        print!(
            "{}",
            thinking_text.truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2)
        );
        io::stdout().flush()
    }

    fn begin_section(&mut self, section: RenderSection) -> io::Result<()> {
        if section == self.active_section {
            return Ok(());
        }

        if self.active_section != RenderSection::None {
            println!();
        }

        match section {
            RenderSection::None => {}
            RenderSection::Answer => {
                print!(
                    "{}",
                    "Edith ›"
                        .truecolor(COLOR_PRIMARY.0, COLOR_PRIMARY.1, COLOR_PRIMARY.2)
                        .bold()
                );
                print!(" ");
            }
            RenderSection::Thinking => {
                if !self.show_thinking {
                    return Ok(());
                }
                print!(
                    "{}",
                    "Reasoning ›"
                        .truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2)
                        .bold()
                );
                print!(" ");
            }
        }

        self.active_section = section;
        io::stdout().flush()
    }

    fn take_prefix(&mut self, tentative_len: usize) -> String {
        let mut boundary = tentative_len.min(self.pending.len());
        while boundary > 0 && !self.pending.is_char_boundary(boundary) {
            boundary -= 1;
        }

        if boundary == 0 {
            return String::new();
        }

        self.pending.drain(..boundary).collect()
    }
}

fn truthy_env(var_name: &str, default: bool) -> bool {
    match std::env::var(var_name) {
        Ok(value) => match value.trim().to_ascii_lowercase().as_str() {
            "1" | "true" | "yes" | "on" => true,
            "0" | "false" | "no" | "off" => false,
            _ => default,
        },
        Err(_) => default,
    }
}

fn max_tokens_from_env(default: usize) -> usize {
    let configured = std::env::var("EDITH_MAX_TOKENS")
        .ok()
        .and_then(|v| v.trim().parse::<usize>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(default);

    configured.min(HARD_MAX_TOKENS)
}

fn max_generation_seconds_from_env(default: u64) -> u64 {
    let configured = std::env::var("EDITH_MAX_GENERATION_SECONDS")
        .ok()
        .and_then(|v| v.trim().parse::<u64>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(default);

    configured.min(HARD_MAX_GENERATION_SECONDS)
}

fn has_repeated_suffix(token_ids: &[i32], ngram_size: usize, repeats: usize) -> bool {
    if ngram_size == 0 || repeats < 2 {
        return false;
    }

    let needed_len = ngram_size.saturating_mul(repeats);
    if token_ids.len() < needed_len {
        return false;
    }

    let start = token_ids.len() - needed_len;
    let reference = &token_ids[start..start + ngram_size];

    (1..repeats).all(|i| {
        let chunk_start = start + i * ngram_size;
        let chunk_end = chunk_start + ngram_size;
        &token_ids[chunk_start..chunk_end] == reference
    })
}

fn has_low_token_diversity(token_ids: &[i32], window_size: usize, min_ratio: f32) -> bool {
    if window_size == 0 || token_ids.len() < window_size {
        return false;
    }

    let recent = &token_ids[token_ids.len() - window_size..];
    let unique_count = recent.iter().copied().collect::<HashSet<_>>().len();
    let ratio = unique_count as f32 / window_size as f32;
    ratio < min_ratio
}

fn print_metric_row(label: &str, value: impl Into<String>) {
    println!(
        "  {} {}",
        format!("{:<18}", format!("{label}:"))
            .truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2),
        value
            .into()
            .truecolor(COLOR_TEXT.0, COLOR_TEXT.1, COLOR_TEXT.2)
    );
}

fn print_metrics(
    input_tokens: usize,
    generated_tokens: usize,
    elapsed_secs: f64,
    tokens_per_sec: f64,
    memory_delta_mb: u64,
    total_memory_mb: u64,
    cpu_usage_percent: f32,
    max_tokens: usize,
    thinking_chars: usize,
    answer_chars: usize,
) {
    println!();
    println!(
        "{}",
        "◆ Performance metrics"
            .truecolor(COLOR_PRIMARY.0, COLOR_PRIMARY.1, COLOR_PRIMARY.2)
            .bold()
    );
    print_metric_row("Model", MODEL_NAME);
    print_metric_row("Input Tokens", input_tokens.to_string());
    print_metric_row("Generated Tokens", generated_tokens.to_string());
    print_metric_row("Output Cap", max_tokens.to_string());
    print_metric_row("Answer Chars", answer_chars.to_string());
    print_metric_row("Thinking Chars", thinking_chars.to_string());
    print_metric_row("Time", format!("{elapsed_secs:.2} sec"));
    print_metric_row("Tokens/Sec", format!("{tokens_per_sec:.2} tok/s"));
    print_metric_row("Memory Delta", format!("{memory_delta_mb} MB"));
    print_metric_row("Total Memory", format!("{total_memory_mb} MB"));
    print_metric_row("CPU Usage", format!("{cpu_usage_percent:.1}%"));
}

fn print_error(message: impl AsRef<str>) {
    eprintln!(
        "{} {}",
        "Error:".truecolor(COLOR_ERROR.0, COLOR_ERROR.1, COLOR_ERROR.2).bold(),
        message
            .as_ref()
            .truecolor(COLOR_ERROR.0, COLOR_ERROR.1, COLOR_ERROR.2)
    );
}

#[tokio::main]
async fn main() -> Result<()> {
    let requested_max_tokens = max_tokens_from_env(DEFAULT_MAX_TOKENS);
    let max_generation_seconds = max_generation_seconds_from_env(DEFAULT_MAX_GENERATION_SECONDS);
    let show_thinking = truthy_env("EDITH_SHOW_THINKING", true);

    println!();
    println!(
        "{}",
        "◆ Edith AI CLI"
            .truecolor(COLOR_PRIMARY.0, COLOR_PRIMARY.1, COLOR_PRIMARY.2)
            .bold()
    );
    println!(
        "{}",
        "  Professional streaming interface"
            .truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2)
    );
    println!();

    println!(
        "{}",
        "Initializing llama.cpp backend..."
            .truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2)
    );
    let mut backend = LlamaBackend::init()?;
    backend.void_logs();

    let model_params = LlamaModelParams::default();
    let model = LlamaModel::load_from_file(&backend, MODEL_PATH, &model_params)?;
    let n_ctx_train = model.n_ctx_train();
    let n_vocab = model.n_vocab();

    // Qwen 3.5 supports long context windows; use a practical runtime window for memory usage.
    let ctx_params = LlamaContextParams::default()
        .with_n_ctx(std::num::NonZeroU32::new(INFERENCE_CONTEXT_TOKENS));
    let mut ctx = model.new_context(&backend, ctx_params)?;

    println!(
        "{}",
        format!("Model loaded: {MODEL_NAME}")
            .truecolor(COLOR_SUCCESS.0, COLOR_SUCCESS.1, COLOR_SUCCESS.2)
    );
    println!(
        "{}",
        format!("Model max context: {n_ctx_train} tokens")
            .truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2)
    );
    println!(
        "{}",
        format!("Runtime context: {INFERENCE_CONTEXT_TOKENS} tokens")
            .truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2)
    );
    println!(
        "{}",
        format!("Vocabulary: {n_vocab} tokens")
            .truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2)
    );
    println!(
        "{}",
        format!(
            "Requested max output tokens: {requested_max_tokens} (set EDITH_MAX_TOKENS=1..{HARD_MAX_TOKENS})"
        )
            .truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2)
    );
    println!(
        "{}",
        format!(
            "Max generation time: {max_generation_seconds}s (set EDITH_MAX_GENERATION_SECONDS=1..{HARD_MAX_GENERATION_SECONDS})"
        )
            .truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2)
    );
    println!(
        "{}",
        if show_thinking {
            "Thinking view: ON (set EDITH_SHOW_THINKING=0 to hide)"
        } else {
            "Thinking view: OFF (set EDITH_SHOW_THINKING=1 to show)"
        }
        .truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2)
    );
    println!(
        "{}",
        format!(
            "Decoding: temp={SAMPLER_TEMPERATURE}, top_p={SAMPLER_TOP_P}, top_k={SAMPLER_TOP_K}, min_p={SAMPLER_MIN_P}, repeat_penalty={SAMPLER_REPEAT_PENALTY}"
        )
        .truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2)
    );
    println!(
        "{}",
        "Type 'exit' or 'quit' to end the session."
            .truecolor(COLOR_ACCENT.0, COLOR_ACCENT.1, COLOR_ACCENT.2)
    );

    let mut sys = System::new_all();
    sys.refresh_all();

    loop {
        print!(
            "\n{} ",
            "You ›"
                .truecolor(COLOR_ACCENT.0, COLOR_ACCENT.1, COLOR_ACCENT.2)
                .bold()
        );
        io::stdout().flush()?;

        let mut input = String::new();
        let bytes_read = io::stdin().read_line(&mut input)?;
        if bytes_read == 0 {
            println!();
            println!(
                "{}",
                "Session closed."
                    .truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2)
            );
            break;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        let lowered = input.to_ascii_lowercase();
        if lowered == "exit" || lowered == "quit" {
            println!(
                "{}",
                "Session closed."
                    .truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2)
            );
            break;
        }

        let start_time = Instant::now();
        sys.refresh_all();
        let start_memory = sys.used_memory();

        // Handle each prompt independently to avoid stale KV positions.
        ctx.clear_kv_cache();

        let prompt = format!(
            "<|im_start|>user\n{}<|im_end|>\n<|im_start|>assistant\n",
            input
        );
        let tokens = match model.str_to_token(&prompt, AddBos::Always) {
            Ok(t) => t,
            Err(e) => {
                print_error(format!("Failed to tokenize prompt: {e}"));
                continue;
            }
        };

        if tokens.is_empty() {
            print_error("Tokenization produced zero tokens");
            continue;
        }

        let available_output_tokens = (INFERENCE_CONTEXT_TOKENS as usize).saturating_sub(tokens.len());
        if available_output_tokens == 0 {
            print_error("Prompt exhausted runtime context; reduce prompt size");
            continue;
        }
        let max_tokens = requested_max_tokens.min(available_output_tokens);

        println!(
            "{}",
            format!("Input tokens: {}", tokens.len())
                .truecolor(COLOR_MUTED.0, COLOR_MUTED.1, COLOR_MUTED.2)
        );

        let mut prompt_batch = LlamaBatch::new(tokens.len(), 1);
        if let Err(e) = prompt_batch.add_sequence(&tokens, 0, false) {
            print_error(format!("Failed to add prompt tokens to batch: {e}"));
            continue;
        }
        if let Err(e) = ctx.decode(&mut prompt_batch) {
            print_error(format!("Failed to decode prompt: {e}"));
            continue;
        }

        let mut generated_tokens = 0usize;
        let mut n_cur = tokens.len() as i32;
        let mut generation_batch = LlamaBatch::new(1, 1);
        let mut renderer = StreamRenderer::new(show_thinking);
        let mut generated_token_ids: Vec<i32> = Vec::with_capacity(max_tokens.min(2048));
        let mut generated_text = String::new();
        let mut sampler = LlamaSampler::chain_simple([
            LlamaSampler::penalties(
                SAMPLER_REPEAT_LAST_N,
                SAMPLER_REPEAT_PENALTY,
                SAMPLER_FREQUENCY_PENALTY,
                SAMPLER_PRESENCE_PENALTY,
            ),
            LlamaSampler::dry(
                &model,
                SAMPLER_DRY_MULTIPLIER,
                SAMPLER_DRY_BASE,
                SAMPLER_DRY_ALLOWED_LENGTH,
                SAMPLER_DRY_PENALTY_LAST_N,
                ["\n", "\n\n", ".", "!", "?", ":", ";", "\"", "'"],
            ),
            LlamaSampler::top_k(SAMPLER_TOP_K),
            LlamaSampler::top_p(SAMPLER_TOP_P, 1),
            LlamaSampler::min_p(SAMPLER_MIN_P, 1),
            LlamaSampler::temp(SAMPLER_TEMPERATURE),
            LlamaSampler::dist(sampler_seed()),
        ]);
        sampler.accept_many(tokens.iter().copied());

        for _ in 0..max_tokens {
            if start_time.elapsed().as_secs() >= max_generation_seconds {
                println!(
                    "{}",
                    format!(
                        "⚠ Stopped generation at time limit ({max_generation_seconds}s)."
                    )
                    .truecolor(COLOR_WARNING.0, COLOR_WARNING.1, COLOR_WARNING.2)
                    .bold()
                );
                break;
            }
            if n_cur >= INFERENCE_CONTEXT_TOKENS as i32 {
                println!(
                    "{}",
                    format!(
                        "⚠ Stopped generation at runtime context limit ({INFERENCE_CONTEXT_TOKENS} tokens)."
                    )
                    .truecolor(COLOR_WARNING.0, COLOR_WARNING.1, COLOR_WARNING.2)
                    .bold()
                );
                break;
            }
            let new_token_id = sampler.sample(&ctx, -1);

            // Stop on EOS or Qwen chat boundary tokens.
            let token_id_i32 = new_token_id.0;
            let qwen_stop_tokens = [151643_i32, 151644_i32, 151645_i32];
            if model.is_eog_token(new_token_id) || qwen_stop_tokens.contains(&token_id_i32) {
                break;
            }

            #[allow(deprecated)]
            let piece_buf = match model.token_to_bytes(
                new_token_id,
                llama_cpp_2::model::Special::Tokenize,
            ) {
                Ok(bytes) => bytes,
                Err(e) => {
                    print_error(format!("Failed to decode token bytes: {e}"));
                    break;
                }
            };

            let piece = String::from_utf8_lossy(&piece_buf);
            generated_text.push_str(piece.as_ref());
            if let Err(e) = renderer.ingest_piece(piece.as_ref()) {
                print_error(format!("Failed to stream output: {e}"));
                break;
            }

            generated_tokens += 1;
            generated_token_ids.push(token_id_i32);
            if has_repeated_suffix(
                &generated_token_ids,
                REPETITION_SUFFIX_NGRAM,
                REPETITION_SUFFIX_REPEATS,
            ) {
                println!(
                    "{}",
                    "⚠ Stopped generation due to repetition loop detection."
                        .truecolor(COLOR_WARNING.0, COLOR_WARNING.1, COLOR_WARNING.2)
                        .bold()
                );
                break;
            }
            if has_low_token_diversity(
                &generated_token_ids,
                DIVERSITY_WINDOW_TOKENS,
                DIVERSITY_MIN_RATIO,
            ) {
                println!(
                    "{}",
                    "⚠ Stopped generation due to low token diversity (loop prevention)."
                        .truecolor(COLOR_WARNING.0, COLOR_WARNING.1, COLOR_WARNING.2)
                        .bold()
                );
                break;
            }
            if has_repeated_text_suffix(
                &generated_text,
                REPETITION_TEXT_SUFFIX_BYTES,
                REPETITION_TEXT_SUFFIX_REPEATS,
            ) {
                println!(
                    "{}",
                    "⚠ Stopped generation due to repeated text loop detection."
                        .truecolor(COLOR_WARNING.0, COLOR_WARNING.1, COLOR_WARNING.2)
                        .bold()
                );
                break;
            }

            generation_batch.clear();
            if let Err(e) = generation_batch.add(new_token_id, n_cur, &[0], true) {
                print_error(format!("Failed to add token to generation batch: {e}"));
                break;
            }
            n_cur += 1;

            if let Err(e) = ctx.decode(&mut generation_batch) {
                print_error(format!("Failed to decode generation token: {e}"));
                break;
            }
        }

        if let Err(e) = renderer.flush() {
            print_error(format!("Failed to flush output stream: {e}"));
        }
        println!();

        let hit_token_cap = generated_tokens >= max_tokens;
        if hit_token_cap {
            let cap_message = if max_tokens < requested_max_tokens {
                format!(
                    "⚠ Output reached prompt context cap ({max_tokens}). Reduce prompt size for longer replies."
                )
            } else {
                format!(
                    "⚠ Output reached token cap ({max_tokens}). Increase EDITH_MAX_TOKENS for longer responses."
                )
            };
            println!(
                "{}",
                cap_message
                    .truecolor(COLOR_WARNING.0, COLOR_WARNING.1, COLOR_WARNING.2)
                    .bold()
            );
        }

        let elapsed_secs = start_time.elapsed().as_secs_f64();
        let tokens_per_sec = if elapsed_secs > 0.0 {
            generated_tokens as f64 / elapsed_secs
        } else {
            0.0
        };

        sys.refresh_all();
        let end_memory = sys.used_memory();
        let memory_delta_mb = (end_memory.saturating_sub(start_memory)) / 1024 / 1024;
        let total_memory_mb = end_memory / 1024 / 1024;

        print_metrics(
            tokens.len(),
            generated_tokens,
            elapsed_secs,
            tokens_per_sec,
            memory_delta_mb,
            total_memory_mb,
            sys.global_cpu_usage(),
            max_tokens,
            renderer.thinking_chars(),
            renderer.answer_chars(),
        );
    }

    Ok(())
}
