// Flow - Open-source voice assistant library
// 
// This library provides STT, LLM, and TTS capabilities for building
// voice-enabled applications.

pub mod audio;
pub mod cli;
pub mod models;
pub mod pipeline;
pub mod utils;

// Re-export commonly used types
pub use audio::{AudioLoader, MelSpectrogramConfig, compute_mel_spectrogram};
pub use cli::{Args, Command, execute};
pub use models::{LocalLlm, MoonshineSTT, KokoroTTS};
pub use pipeline::VoicePipeline;
pub use utils::{get_memory_info, check_memory_requirements};
