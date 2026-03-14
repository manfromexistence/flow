//! LLM integration for chat UI - local model only

use anyhow::Result;
use std::sync::Arc;

use super::app_state::GoogleModel;
use super::local_llm::LocalLlm;

#[derive(Clone)]
pub struct ChatLlm {
    local_llm: Arc<LocalLlm>,
}

impl ChatLlm {
    pub fn new() -> Self {
        Self {
            local_llm: Arc::new(LocalLlm::new()),
        }
    }

    pub async fn initialize(&self) -> Result<()> {
        self.local_llm.initialize().await
    }

    pub async fn generate(&self, prompt: &str) -> Result<String> {
        self.local_llm.generate(prompt).await
    }

    pub async fn generate_stream<F>(&self, prompt: &str, callback: F) -> Result<()>
    where
        F: Fn(String) + Send + 'static,
    {
        self.local_llm.generate_stream(prompt, callback).await
    }

    pub fn is_initialized(&self) -> bool {
        self.local_llm.is_initialized()
    }

    pub fn get_model_name(&self) -> String {
        self.local_llm.get_model_name()
    }

    pub fn get_backend(&self) -> String {
        "local".to_string()
    }

    pub async fn set_backend(&self, _backend: String) -> Result<()> {
        // Only local backend supported
        Ok(())
    }

    pub fn get_google_api_key(&self) -> Option<String> {
        None
    }

    pub async fn set_google_api_key(&self, _api_key: String) -> Result<()> {
        Ok(())
    }

    pub fn get_elevenlabs_api_key(&self) -> Option<String> {
        None
    }

    pub async fn set_elevenlabs_api_key(&self, _api_key: String) -> Result<()> {
        Ok(())
    }

    pub async fn set_google_model(&self, _model: String) -> Result<()> {
        Ok(())
    }

    pub async fn set_antigravity_oauth_token(&self, _token: String) -> Result<()> {
        Ok(())
    }

    pub async fn set_antigravity_model(&self, _model: String) -> Result<()> {
        Ok(())
    }

    pub fn get_cached_google_models(&self) -> Vec<GoogleModel> {
        Vec::new()
    }

    pub async fn fetch_google_models(&self) -> Result<Vec<GoogleModel>> {
        Ok(Vec::new())
    }
}

impl Default for ChatLlm {
    fn default() -> Self {
        Self::new()
    }
}
