// Your actual TUI modules (EXACT copies from src/)
pub mod components;
pub mod effects;
pub mod input;
pub mod llm;
pub mod theme;
pub mod splash;
pub mod perf;
pub mod autocomplete;
pub mod tachyonfx_demo;
pub mod modal;
pub mod font;
pub mod exit_animation;

// Integration with yazi
pub mod state;
pub mod bridge;
pub mod render;
pub mod animations;

// Re-exports for convenience
pub use state::AnimationType;
pub use bridge::{YaziChatBridge, AppMode};
pub use components::MessageRole;
