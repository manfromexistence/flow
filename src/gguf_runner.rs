use anyhow::{Context, Result};
use llama_cpp_2::context::params::LlamaContextParams;
use llama_cpp_2::llama_backend::LlamaBackend;
use llama_cpp_2::llama_batch::LlamaBatch;
use llama_cpp_2::model::params::LlamaModelParams;
use llama_cpp_2::model::{AddBos, LlamaModel};
use llama_cpp_2::sampling::LlamaSampler;
use std::num::NonZeroU32;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

const INFERENCE_CONTEXT_TOKENS: u32 = 4096;
const PROMPT_BATCH_SIZE: usize = 512;
const SAMPLER_TEMPERATURE: f32 = 0.7;
const SAMPLER_TOP_P: f32 = 0.95;
const SAMPLER_TOP_K: i32 = 40;
const SAMPLER_MIN_P: f32 = 0.05;
const SAMPLER_REPEAT_LAST_N: i32 = 64;
const SAMPLER_REPEAT_PENALTY: f32 = 1.10;

/// A generic GGUF model runner using llama-cpp-2
pub struct GgufRunner {
    backend: LlamaBackend,
    model: LlamaModel,
}

impl GgufRunner {
    /// Initialize the backend and load the GGUF model from the given path
    pub fn new<P: AsRef<Path>>(model_path: P) -> Result<Self> {
        let mut backend = LlamaBackend::init().context("Failed to initialize llama backend")?;
        backend.void_logs();

        let model_params = LlamaModelParams::default();
        let model = LlamaModel::load_from_file(
            &backend,
            model_path.as_ref().to_string_lossy().as_ref(),
            &model_params,
        )
        .with_context(|| {
            format!(
                "Failed to load model from path: {}",
                model_path.as_ref().display()
            )
        })?;

        Ok(Self { backend, model })
    }

    /// Generate text based on a given prompt
    pub fn generate(&self, prompt: &str, max_tokens_to_generate: usize) -> Result<String> {
        let n_threads = Self::optimal_thread_count();
        let ctx_params = LlamaContextParams::default()
            .with_n_ctx(NonZeroU32::new(INFERENCE_CONTEXT_TOKENS))
            .with_n_batch(PROMPT_BATCH_SIZE as u32)
            .with_n_threads(n_threads)
            .with_n_threads_batch(n_threads)
            // .with_flash_attention_policy(1) // Uncomment if your built llama.cpp supports FA
            ;

        let mut ctx = self
            .model
            .new_context(&self.backend, ctx_params)
            .context("Failed to create inference context")?;

        ctx.clear_kv_cache();

        let tokens = self
            .model
            .str_to_token(prompt, AddBos::Always)
            .context("Tokenization failed")?;

        let available = (INFERENCE_CONTEXT_TOKENS as usize).saturating_sub(tokens.len());
        let max_tokens = available.min(max_tokens_to_generate);

        // Batched prompt evaluation
        let mut pos: i32 = 0;
        let total = tokens.len();
        let mut offset = 0;

        while offset < total {
            let end = (offset + PROMPT_BATCH_SIZE).min(total);
            let chunk = &tokens[offset..end];
            let is_last_chunk = end == total;

            let mut batch = LlamaBatch::new(chunk.len(), 1);
            for (i, &token) in chunk.iter().enumerate() {
                let logits = is_last_chunk && i == chunk.len() - 1;
                batch.add(token, pos, &[0], logits)?;
                pos += 1;
            }
            ctx.decode(&mut batch)?;
            offset = end;
        }

        // Sampler chain Setup
        let mut sampler = LlamaSampler::chain_simple([
            LlamaSampler::penalties(SAMPLER_REPEAT_LAST_N, SAMPLER_REPEAT_PENALTY, 0.0, 0.0),
            LlamaSampler::top_k(SAMPLER_TOP_K),
            LlamaSampler::top_p(SAMPLER_TOP_P, 1),
            LlamaSampler::min_p(SAMPLER_MIN_P, 1),
            LlamaSampler::temp(SAMPLER_TEMPERATURE),
            LlamaSampler::dist(Self::sampler_seed()),
        ]);
        sampler.accept_many(tokens.iter().copied());

        // Generation loop
        let mut n_cur = tokens.len() as i32;
        let mut generated_text = String::with_capacity(max_tokens * 4);
        let mut gen_batch = LlamaBatch::new(1, 1);

        let mut hit_limit = false;
        let mut extra_tokens = 0;
        let max_loop = max_tokens + 50;

        for i in 0..max_loop {
            if i >= max_tokens {
                hit_limit = true;
            }
            if n_cur >= INFERENCE_CONTEXT_TOKENS as i32 {
                break;
            }

            let token = sampler.sample(&ctx, -1);

            if self.model.is_eog_token(token) {
                break;
            }

            #[allow(deprecated)]
            let piece_bytes = self
                .model
                .token_to_bytes(token, llama_cpp_2::model::Special::Tokenize)?;
            let piece = String::from_utf8_lossy(&piece_bytes);
            generated_text.push_str(&piece);

            // Feed new token to sampler
            sampler.accept(token);

            gen_batch.clear();
            gen_batch.add(token, n_cur, &[0], true)?;
            n_cur += 1;

            ctx.decode(&mut gen_batch)?;

            if hit_limit {
                let last_char = piece.chars().last().unwrap_or(' ');
                if last_char == '.' || last_char == '?' || last_char == '!' || piece.contains('\n') {
                    break;
                }
                extra_tokens += 1;
                if extra_tokens >= 50 {
                    generated_text.push_str("...");
                    break;
                }
            }
        }

        Ok(generated_text)
    }

    /// Stream generated text through a callback function
    pub fn generate_stream<F>(
        &self,
        prompt: &str,
        max_tokens_to_generate: usize,
        mut callback: F,
    ) -> Result<()>
    where
        F: FnMut(&str),
    {
        let n_threads = Self::optimal_thread_count();
        let ctx_params = LlamaContextParams::default()
            .with_n_ctx(NonZeroU32::new(INFERENCE_CONTEXT_TOKENS))
            .with_n_batch(PROMPT_BATCH_SIZE as u32)
            .with_n_threads(n_threads)
            .with_n_threads_batch(n_threads)
            // .with_flash_attention_policy(1) // Uncomment if your built llama.cpp supports FA
            ;

        let mut ctx = self
            .model
            .new_context(&self.backend, ctx_params)
            .context("Failed to create inference context")?;

        ctx.clear_kv_cache();

        let tokens = self
            .model
            .str_to_token(prompt, AddBos::Always)
            .context("Tokenization failed")?;

        let available = (INFERENCE_CONTEXT_TOKENS as usize).saturating_sub(tokens.len());
        let max_tokens = available.min(max_tokens_to_generate);

        // Batched prompt evaluation
        let mut pos: i32 = 0;
        let total = tokens.len();
        let mut offset = 0;

        while offset < total {
            let end = (offset + PROMPT_BATCH_SIZE).min(total);
            let chunk = &tokens[offset..end];
            let is_last_chunk = end == total;

            let mut batch = LlamaBatch::new(chunk.len(), 1);
            for (i, &token) in chunk.iter().enumerate() {
                let logits = is_last_chunk && i == chunk.len() - 1;
                batch.add(token, pos, &[0], logits)?;
                pos += 1;
            }
            ctx.decode(&mut batch)?;
            offset = end;
        }

        // Sampler chain
        let mut sampler = LlamaSampler::chain_simple([
            LlamaSampler::penalties(SAMPLER_REPEAT_LAST_N, SAMPLER_REPEAT_PENALTY, 0.0, 0.0),
            LlamaSampler::top_k(SAMPLER_TOP_K),
            LlamaSampler::top_p(SAMPLER_TOP_P, 1),
            LlamaSampler::min_p(SAMPLER_MIN_P, 1),
            LlamaSampler::temp(SAMPLER_TEMPERATURE),
            LlamaSampler::dist(Self::sampler_seed()),
        ]);
        sampler.accept_many(tokens.iter().copied());

        // Generation loop with streaming
        let mut n_cur = tokens.len() as i32;
        let mut gen_batch = LlamaBatch::new(1, 1);

        let mut hit_limit = false;
        let mut extra_tokens = 0;
        let max_loop = max_tokens + 50;

        for i in 0..max_loop {
            if i >= max_tokens {
                hit_limit = true;
            }
            if n_cur >= INFERENCE_CONTEXT_TOKENS as i32 {
                break;
            }

            let token = sampler.sample(&ctx, -1);

            if self.model.is_eog_token(token) {
                break;
            }

            #[allow(deprecated)]
            let piece_bytes = self
                .model
                .token_to_bytes(token, llama_cpp_2::model::Special::Tokenize)?;
            let piece = String::from_utf8_lossy(&piece_bytes);

            callback(&piece);

            // Feed new token to sampler
            sampler.accept(token);

            gen_batch.clear();
            gen_batch.add(token, n_cur, &[0], true)?;
            n_cur += 1;

            ctx.decode(&mut gen_batch)?;

            if hit_limit {
                let last_char = piece.chars().last().unwrap_or(' ');
                if last_char == '.' || last_char == '?' || last_char == '!' || piece.contains('\n') {
                    break;
                }
                extra_tokens += 1;
                if extra_tokens >= 50 {
                    callback("...");
                    break;
                }
            }
        }

        Ok(())
    }

    fn optimal_thread_count() -> i32 {
        let physical = std::thread::available_parallelism().map(|p| p.get()).unwrap_or(1);
        if physical > 4 {
            (physical - 1) as i32
        } else {
            physical as i32
        }
    }

    fn sampler_seed() -> u32 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_nanos() as u32)
            .unwrap_or(0xDEAD_BEEF)
    }
}
