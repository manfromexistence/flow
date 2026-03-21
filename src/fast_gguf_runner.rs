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

/// A highly optimized GGUF runner for maximum native performance.
/// To get true maximum performance, you must compile `llama-cpp-2`
/// with the appropriate feature flags in Cargo.toml (e.g., `cuda`, `metal`, `vulkan`).
pub struct FastGgufRunner {
    backend: LlamaBackend,
    model: LlamaModel,
}

impl FastGgufRunner {
    /// Initialize with MAXIMUM PERFORMANCE configuration for the given GGUF model
    pub fn new<P: AsRef<Path>>(model_path: P) -> Result<Self> {
        let mut backend = LlamaBackend::init().context("Failed to initialize llama backend")?;
        backend.void_logs();

        // 🚀 PERFORMANCE CRITICAL: Model loading parameters
        let model_params = LlamaModelParams::default()
            // Offload ALL layers to GPU (CUDA/Metal) if compiled with those features
            // Setting this to a very high number guarantees 100% offload if your VRAM permits.
            .with_n_gpu_layers(999);

        let model = LlamaModel::load_from_file(
            &backend,
            model_path.as_ref().to_string_lossy().as_ref(),
            &model_params,
        )
        .with_context(|| format!("Failed to load highly-optimized model from path: {}", model_path.as_ref().display()))?;

        Ok(Self { backend, model })
    }

    /// Optimized streaming generation
    pub fn generate_stream<F>(
        &self,
        prompt: &str,
        max_tokens_to_generate: usize,
        mut callback: F,
    ) -> Result<()>
    where
        F: FnMut(&str),
    {
        // 🚀 PERFORMANCE CRITICAL: Context parameters
        let n_threads = std::thread::available_parallelism().map(|p| p.get() as u32).unwrap_or(4);
        
        let ctx_params = LlamaContextParams::default()
            .with_n_ctx(NonZeroU32::new(4096)) // Optimize context limit based on actual needs
            .with_n_batch(512)
            // Optimize thread count: Max available physical cores minus 1 (leaves room for OS/GPU sync)
            .with_n_threads(n_threads.saturating_sub(1).max(1))
            .with_n_threads_batch(n_threads.saturating_sub(1).max(1))
            // Enable Flash Attention! (Massive speedup + radically lower memory usage for long contexts)
            .with_flash_attention_policy(1);

        let mut ctx = self
            .model
            .new_context(&self.backend, ctx_params)
            .context("Failed to create high-performance context")?;

        ctx.clear_kv_cache();

        let tokens = self.model.str_to_token(prompt, AddBos::Always)?;
        let max_tokens = (4096usize).saturating_sub(tokens.len()).min(max_tokens_to_generate);

        let mut batch = LlamaBatch::new(512, 1);
        let mut pos: i32 = 0;
        
        // Highly batched prompt ingestion
        for chunk in tokens.chunks(512) {
            batch.clear();
            for (i, &token) in chunk.iter().enumerate() {
                let logits = i == chunk.len() - 1;
                batch.add(token, pos, &[0], logits)?;
                pos += 1;
            }
            ctx.decode(&mut batch)?;
        }

        // Fast minimal sampler
        let mut sampler = LlamaSampler::chain_simple([
            LlamaSampler::temp(0.7),
            LlamaSampler::top_p(0.95, 1),
            LlamaSampler::dist(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u32),
        ]);
        sampler.accept_many(tokens.iter().copied());

        let mut n_cur = tokens.len() as i32;
        let mut gen_batch = LlamaBatch::new(1, 1);

        // Fast generation loop
        let mut hit_limit = false;
        let mut extra_tokens = 0;
        let max_loop = max_tokens + 50;

        for i in 0..max_loop {
            if i >= max_tokens {
                hit_limit = true;
            }
            let token = sampler.sample(&ctx, -1);
            if self.model.is_eog_token(token) { break; }

            #[allow(deprecated)]
            let piece = String::from_utf8_lossy(&self.model.token_to_bytes(token, llama_cpp_2::model::Special::Tokenize)?);
            callback(&piece);

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
}
