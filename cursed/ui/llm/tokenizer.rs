//! Tokenizer utilities
//!
//! Note: Tokenizers crate removed due to CRT conflict with llama-cpp-2.
//! This is a stub implementation. Use llama-cpp-2's built-in tokenizer instead.

use anyhow::Result;

pub struct TokenizerWrapper;

impl TokenizerWrapper {
    pub fn from_file(_path: &str) -> Result<Self> {
        anyhow::bail!("Tokenizer support removed. Use llama-cpp-2's built-in tokenizer instead.")
    }

    pub fn encode(&self, _text: &str) -> Result<Vec<u32>> {
        anyhow::bail!("Tokenizer support removed. Use llama-cpp-2's built-in tokenizer instead.")
    }

    pub fn decode(&self, _ids: &[u32]) -> Result<String> {
        anyhow::bail!("Tokenizer support removed. Use llama-cpp-2's built-in tokenizer instead.")
    }

    pub fn count_tokens(&self, _text: &str) -> Result<usize> {
        anyhow::bail!("Tokenizer support removed. Use llama-cpp-2's built-in tokenizer instead.")
    }
}
