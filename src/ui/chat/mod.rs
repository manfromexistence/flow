//! Chat application - core TUI implementation

pub mod app;
pub mod app_data;
pub mod app_events;
pub mod app_handlers;
pub mod app_helpers;
pub mod app_llm;
pub mod app_render;
pub mod app_splash;
pub mod app_state;
pub mod components;
pub mod effects;
pub mod input;
pub mod llm_integration;
pub mod local_llm;
pub mod modal_effects;
pub mod modal_list;
pub mod modals;
pub mod modes;
pub mod text_input;
pub mod theme;

pub use app::ChatApp;
pub use llm_integration::ChatLlm;
pub use local_llm::LocalLlm;
