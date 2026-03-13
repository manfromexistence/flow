//! Chat application state management

use ratatui::layout::Rect;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender, channel};
use std::time::Instant;

use super::components::Message;
use super::modals::add::AddModalFocus;
use super::{
    app_data::{Agent, Focus, GitChange, Task},
    effects::{RainbowEffect, ShimmerEffect, TypingIndicator},
    input::InputState,
    modal_list::ModalList,
    modes::ChatMode,
    text_input::TextInput,
    theme::{ChatTheme, ThemeVariant},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ModalType {
    Focus,
    Add,
    Plan,
    Model,
    Local,
    Changes,
    Tasks,
    Agents,
    Memory,
    Tools,
    More,
    GoogleApi,
    ElevenlabsApi,
    EffectsDemo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimationType {
    Splash,
    Matrix,
    Train,
    Confetti,
    GameOfLife,
    Starfield,
    Rain,
    NyanCat,
    DVDLogo,
}

impl AnimationType {
    pub fn all() -> Vec<Self> {
        vec![
            Self::Splash,
            Self::Matrix,
            Self::Train,
            Self::Confetti,
            Self::GameOfLife,
            Self::Starfield,
            Self::Rain,
            Self::NyanCat,
            Self::DVDLogo,
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Splash => "Splash Screen",
            Self::Matrix => "Matrix Rain",
            Self::Train => "ASCII Train",
            Self::Confetti => "Confetti Explosion",
            Self::GameOfLife => "Conway's Game of Life",
            Self::Starfield => "Starfield",
            Self::Rain => "Rain",
            Self::NyanCat => "Nyan Cat",
            Self::DVDLogo => "DVD Logo Bounce",
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GoogleModel {
    pub display_name: String, // Camel case for UI
    pub api_name: String,     // Lowercase for API calls
}

/// Main chat application state
pub struct ChatApp {
    // Theme and mode
    pub theme: ChatTheme,
    pub mode: ChatMode,

    // Input state
    pub input: InputState,
    pub messages: Vec<Message>,
    pub is_loading: bool,
    pub typing_indicator: TypingIndicator,
    pub shimmer: ShimmerEffect,
    pub rainbow_cursor: RainbowEffect,
    pub focus: Focus,
    pub should_quit: bool,

    // Timing
    pub last_render: Instant,
    pub last_shortcut_update: Instant,
    pub last_cursor_blink: Instant,
    pub last_interaction: Instant,
    pub last_font_change: Instant,
    pub last_shortcut_time: Instant,

    // UI state
    pub show_bottom_menu: bool,
    pub audio_mode: bool,
    pub audio_processing: bool,
    pub shortcut_index: usize,
    pub cursor_visible: bool,
    pub terminal_focused: bool,
    pub splash_font_index: usize,

    // Animation state
    pub show_matrix_animation: bool,
    pub show_train_animation: bool,
    pub show_dx_splash: bool,
    pub animation_start_time: Option<Instant>,
    pub current_workspace: Option<String>,
    pub switching_workspace: bool,
    pub animation_sequence_active: bool, // Track if '0' key animation sequence is active
    pub current_animation_index: usize,  // Index for animation carousel
    pub animation_mode: bool,            // Whether we're in animation viewing mode

    // History
    pub prompt_history: Vec<String>,
    pub history_index: Option<usize>,

    // Button positions for click detection
    pub input_area: Rect,
    pub plan_button_area: Rect,
    pub model_button_area: Rect,
    pub local_button_area: Rect,

    // Debug
    pub last_shortcut_pressed: Option<String>,

    // Focus menu
    pub show_focus_menu: bool,
    pub focus_menu_list: ModalList,

    // Add modal
    pub show_add_modal: bool,
    pub add_modal_search: TextInput,
    pub add_modal_list: ModalList,
    pub add_modal_focus: AddModalFocus,

    // Plan modal
    pub show_plan_modal: bool,
    pub plan_modal_list: ModalList,

    // Model modal
    pub show_model_modal: bool,
    pub model_modal_search: TextInput,
    pub model_modal_list: ModalList,
    pub selected_model: String,
    pub selected_models: Vec<String>,
    pub auto_mode: bool,
    pub max_mode: bool,
    pub use_multiple_models: bool,

    // Local modal
    pub show_local_modal: bool,
    pub local_modal_list: ModalList,
    pub selected_local_mode: String,

    // Changes modal
    pub show_changes_modal: bool,
    pub changes_modal_list: ModalList,
    pub git_changes: Vec<GitChange>,
    pub changes_count: usize,

    // Tasks modal
    pub show_tasks_modal: bool,
    pub tasks_modal_list: ModalList,
    pub tasks: Vec<Task>,
    pub tasks_count: usize,

    // Agents modal
    pub show_agents_modal: bool,
    pub agents_modal_list: ModalList,
    pub agents: Vec<Agent>,
    pub agents_count: usize,

    // Workspaces modal
    pub show_workspaces_modal: bool,
    pub workspaces_modal_list: ModalList,
    pub workspace_create_input: TextInput,
    pub workspace_create_mode: bool,

    // Memory modal
    pub show_memory_modal: bool,
    pub memory_modal_list: ModalList,
    pub selected_memory_mode: String,

    // Google API modal
    pub show_google_api_modal: bool,
    pub google_api_input: TextInput,
    pub google_models: Vec<GoogleModel>,

    // ElevenLabs API modal
    pub show_elevenlabs_api_modal: bool,
    pub elevenlabs_api_input: TextInput,

    // Effects Demo modal
    pub show_effects_demo_modal: bool,
    pub effects_demo: super::modals::effects_demo::EffectsDemoModal,

    // Tools modal
    pub show_tools_modal: bool,
    pub tools_modal_list: ModalList,
    pub tools: Vec<super::modals::tools::Tool>,

    // More modal
    pub show_more_modal: bool,
    pub more_modal_list: ModalList,
    pub more_options: Vec<super::modals::more::MoreOption>,

    // Chat scroll
    pub chat_scroll_offset: usize,

    // Audio transcription channel
    pub audio_tx: Sender<String>,
    pub audio_rx: Receiver<String>,

    // LLM integration
    pub llm: Option<Arc<crate::ui::chat::ChatLlm>>,
    pub llm_initialized: bool,

    // LLM response channel
    pub llm_tx: Sender<String>,
    pub llm_rx: Receiver<String>,

    // Modal animation state - using EffectManager for proper animation handling
    pub modal_effect_manager: tachyonfx::EffectManager<()>,
    pub modal_opening: Option<(ModalType, Instant)>,
    pub modal_closing: Option<(ModalType, Instant)>,
    
    // Current modal effect for rendering
    pub current_modal_effect: Option<tachyonfx::Effect>,
    pub modal_effect_start_time: Option<Instant>,

    // Rainbow animation for splash and spinner
    pub rainbow_animation: crate::ui::theme::animation::RainbowAnimation,
}

impl ChatApp {
    pub fn new() -> Self {
        let theme = ChatTheme::new(ThemeVariant::Dark);
        let shimmer = ShimmerEffect::new(theme.shimmer_colors.clone());
        let rainbow_cursor = RainbowEffect::new();
        let (audio_tx, audio_rx) = channel();
        let (llm_tx, llm_rx) = channel();

        // Create LLM and get model name
        let llm = Arc::new(crate::ui::chat::ChatLlm::new());
        let selected_model = llm.get_model_name();

        Self {
            theme,
            mode: ChatMode::Agent,
            input: InputState::new(),
            messages: Vec::new(),
            is_loading: false,
            typing_indicator: TypingIndicator::new(),
            shimmer,
            rainbow_cursor,
            focus: Focus::ModeSelector, // Default to shortcuts mode
            should_quit: false,
            last_render: Instant::now(),
            show_bottom_menu: true,
            audio_mode: false,
            audio_processing: false,
            shortcut_index: 0,
            last_shortcut_update: Instant::now(),
            cursor_visible: true,
            last_cursor_blink: Instant::now(),
            last_interaction: Instant::now(),
            prompt_history: Vec::new(),
            history_index: None,
            terminal_focused: true,
            splash_font_index: 0,
            last_font_change: Instant::now(),
            show_matrix_animation: false,
            show_train_animation: false,
            show_dx_splash: false,
            animation_start_time: None,
            current_workspace: None,
            switching_workspace: false,
            animation_sequence_active: false,
            current_animation_index: 0,
            animation_mode: false,
            input_area: Rect::default(),
            plan_button_area: Rect::default(),
            model_button_area: Rect::default(),
            local_button_area: Rect::default(),
            last_shortcut_pressed: None,
            last_shortcut_time: Instant::now(),
            show_focus_menu: false,
            focus_menu_list: ModalList::new(10),
            show_add_modal: false,
            add_modal_search: TextInput::new(),
            add_modal_list: ModalList::new(10),
            add_modal_focus: AddModalFocus::Search,
            show_plan_modal: false,
            plan_modal_list: ModalList::new(4),
            show_model_modal: false,
            model_modal_search: TextInput::new(),
            model_modal_list: ModalList::new(20),
            selected_model,
            selected_models: Vec::new(),
            auto_mode: false,
            max_mode: false,
            use_multiple_models: false,
            show_local_modal: false,
            local_modal_list: ModalList::new(3),
            selected_local_mode: "Local".to_string(),
            show_changes_modal: false,
            changes_modal_list: ModalList::new(10),
            git_changes: Vec::new(),
            changes_count: 0,
            show_tasks_modal: false,
            tasks_modal_list: ModalList::new(4),
            tasks: Vec::new(),
            tasks_count: 0,
            show_agents_modal: false,
            agents_modal_list: ModalList::new(10),
            agents: Vec::new(),
            agents_count: 0,
            show_workspaces_modal: false,
            workspaces_modal_list: ModalList::new(10),
            workspace_create_input: TextInput::new(),
            workspace_create_mode: false,
            show_memory_modal: false,
            memory_modal_list: ModalList::new(3),
            selected_memory_mode: "Checkpoints".to_string(),
            show_google_api_modal: false,
            google_api_input: TextInput::new(),
            google_models: Vec::new(),
            show_elevenlabs_api_modal: false,
            elevenlabs_api_input: TextInput::new(),
            show_effects_demo_modal: false, // Commented out - was showing by default
            effects_demo: super::modals::effects_demo::EffectsDemoModal::new(),
            show_tools_modal: false,
            tools_modal_list: ModalList::new(9),
            tools: super::modals::tools::get_available_tools(),
            show_more_modal: false,
            more_modal_list: ModalList::new(6),
            more_options: super::modals::more::get_more_options(),

            chat_scroll_offset: 0,
            audio_tx,
            audio_rx,
            llm: Some(llm),
            llm_initialized: false,
            llm_tx,
            llm_rx,
            modal_effect_manager: tachyonfx::EffectManager::default(),
            modal_opening: None,
            modal_closing: None,
            current_modal_effect: None,
            modal_effect_start_time: None,
            rainbow_animation: crate::ui::theme::animation::RainbowAnimation::new()
                .with_speed(0.5)
                .with_saturation(0.9)
                .with_lightness(0.6),
        }
    }

    pub fn is_in_rect(&self, x: u16, y: u16, rect: Rect) -> bool {
        x >= rect.x && x < rect.x + rect.width && y >= rect.y && y < rect.y + rect.height
    }

    pub fn cycle_theme(&mut self) {
        self.theme = ChatTheme::new(match self.theme.variant {
            ThemeVariant::Dark => ThemeVariant::Light,
            ThemeVariant::Light => ThemeVariant::Dark,
        });
        self.shimmer = ShimmerEffect::new(self.theme.shimmer_colors.clone());
    }

    pub fn toggle_loading(&mut self) {
        self.is_loading = !self.is_loading;
        if self.is_loading {
            self.shimmer.reset();
        }
    }

    /// Start a modal effect animation
    pub fn start_modal_effect(&mut self, modal_type: ModalType) {
        use super::modal_effects;
        
        let bg = self.theme.bg;
        let screen_bg = self.theme.bg;
        
        // Create the effect based on modal type
        let effect = modal_effects::get_modal_open_effect(modal_type, bg, screen_bg);
        
        self.current_modal_effect = Some(effect);
        self.modal_effect_start_time = Some(Instant::now());
    }

    /// Stop the current modal effect
    pub fn stop_modal_effect(&mut self) {
        self.current_modal_effect = None;
        self.modal_effect_start_time = None;
    }

    pub fn send_message(&mut self, content: String) {
        if content.trim().is_empty() {
            return;
        }

        // Add user message
        self.messages
            .push(super::components::Message::user(content.clone()));

        // Trigger matrix animation on first message (2 seconds)
        if self.messages.len() == 1 {
            self.show_matrix_animation = true;
            self.animation_start_time = Some(Instant::now());
        }

        // Show loading
        self.is_loading = true;
        self.shimmer.reset();

        // Add empty assistant message for streaming
        self.messages
            .push(super::components::Message::assistant(String::new()));

        // Generate LLM response with streaming
        if let Some(llm) = self.llm.clone() {
            let tx = self.llm_tx.clone();
            let prompt = content.clone();

            tokio::spawn(async move {
                // Ensure LLM is initialized before generating
                if !llm.is_initialized() {
                    let _ = tx.send("🔄 Initializing local LLM model...\n".to_string());
                    let _ = tx.send(format!("📁 Model: {}\n", llm.get_model_name()));

                    match llm.initialize().await {
                        Ok(_) => {
                            let _ = tx.send("✅ Model initialized successfully!\n\n".to_string());
                        }
                        Err(e) => {
                            let _ = tx.send(format!("❌ Initialization failed: {}\n", e));
                            let _ = tx.send(
                                "📍 Model path: f:/edith/models/llm/Qwen3.5-0.8B-Q4_K_M.gguf\n"
                                    .to_string(),
                            );
                            let _ = tx.send(
                                "💡 Please check if the model file exists at this path.\n"
                                    .to_string(),
                            );
                            let _ = tx.send("\n__END__".to_string());
                            return;
                        }
                    }
                }

                let tx_clone = tx.clone();
                match llm
                    .generate_stream(&prompt, move |chunk| {
                        // Send each chunk as it arrives
                        let _ = tx_clone.send(chunk);
                    })
                    .await
                {
                    Ok(_) => {
                        // Send end marker
                        let _ = tx.send("\n__END__".to_string());
                    }
                    Err(e) => {
                        let _ = tx.send(format!("Error: {}", e));
                        let _ = tx.send("\n__END__".to_string());
                    }
                }
            });
        } else {
            // Fallback if LLM not available
            if let Some(last_msg) = self.messages.last_mut() {
                last_msg.content =
                    "LLM not initialized. Please run `dx llm init` first.".to_string();
            }
            self.is_loading = false;
        }

        // Clear input
        self.input.content.clear();
        self.input.cursor_position = 0;
        self.input.scroll_offset = 0;
    }

    pub fn check_llm_response(&mut self) {
        // Check if there's an LLM response chunk ready
        if let Ok(chunk) = self.llm_rx.try_recv() {
            if chunk.starts_with("__GOOGLE_MODELS_WITH_MODAL__:") {
                // Parse Google models and open modal
                let models_json = chunk
                    .strip_prefix("__GOOGLE_MODELS_WITH_MODAL__:")
                    .unwrap_or("");
                if let Ok(models) = serde_json::from_str::<Vec<GoogleModel>>(models_json) {
                    self.google_models = models;
                    // Ensure model modal is open
                    self.show_model_modal = true;
                    let total_models =
                        super::modals::model::get_filtered_models(&self.model_modal_search.content)
                            .len();
                    let new_count = 1 + 1 + 3 + self.google_models.len() + total_models; // Configure API Key + Sign in + 3 config + Google models + regular models
                    self.model_modal_list.set_items_count(new_count);
                    self.model_modal_list.reset();
                }
            } else if chunk.starts_with("__GOOGLE_MODELS__:") {
                // Parse Google models (without opening modal - for startup)
                let models_json = chunk.strip_prefix("__GOOGLE_MODELS__:").unwrap_or("");
                if let Ok(models) = serde_json::from_str::<Vec<GoogleModel>>(models_json) {
                    self.google_models = models;
                    // Update model modal list count if it's open
                    if self.show_model_modal {
                        let total_models = super::modals::model::get_filtered_models(
                            &self.model_modal_search.content,
                        )
                        .len();
                        self.model_modal_list
                            .set_items_count(4 + self.google_models.len() + total_models);
                    }
                }
            } else if chunk.starts_with("__GOOGLE_ERROR__:") {
                let error = chunk
                    .strip_prefix("__GOOGLE_ERROR__:")
                    .unwrap_or("Unknown error");
                // Show error as last shortcut pressed
                self.last_shortcut_pressed = Some(error.to_string());
                self.last_shortcut_time = Instant::now();
            } else if chunk == "\n__END__" {
                // End of stream
                self.is_loading = false;
            } else if let Some(last_msg) = self.messages.last_mut() {
                // Append chunk to last message
                if last_msg.role == super::components::MessageRole::Assistant {
                    last_msg.content.push_str(&chunk);
                }
            }
        }
    }

    pub fn start_audio_recording(&mut self) {
        let tx = self.audio_tx.clone();
        self.audio_processing = true;

        // Get API key from LLM config if available
        let api_key = self.llm.as_ref().and_then(|llm| llm.get_google_api_key());

        // Spawn async task to handle audio recording
        tokio::spawn(async move {
            // Record audio for 5 seconds
            let temp_dir = std::env::temp_dir();
            let audio_path = temp_dir.join("dx_chat_recording.wav");

            match super::audio::AudioRecorder::new() {
                Ok(mut recorder) => {
                    if let Err(e) = recorder.start_recording() {
                        let _ = tx.send(format!("[Recording Error: {}]", e));
                        return;
                    }

                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;

                    if let Err(e) = recorder.stop_recording(&audio_path) {
                        let _ = tx.send(format!("[Recording Error: {}]", e));
                        return;
                    }

                    // Try Gemini first if API key is available
                    let transcription = if let Some(_key) = api_key {
                        // TODO: Re-enable when audio command is available
                        // match crate::commands::audio::transcribe_with_gemini(
                        //     &audio_path,
                        //     &key,
                        //     "gemini-2.0-flash-exp",
                        // )
                        // .await
                        // {
                        //     Ok(text) => text,
                        //     Err(_) => {
                        //         // Fallback to local whisper
                        //         match super::audio::transcribe_audio(&audio_path) {
                        //             Ok(text) => text,
                        //             Err(e) => format!("[Transcription Error: {}]", e),
                        //         }
                        //     }
                        // }
                        match super::audio::transcribe_audio(&audio_path) {
                            Ok(text) => text,
                            Err(e) => format!("[Transcription Error: {}]", e),
                        }
                    } else {
                        // Use local whisper
                        match super::audio::transcribe_audio(&audio_path) {
                            Ok(text) => text,
                            Err(e) => format!("[Transcription Error: {}]", e),
                        }
                    };

                    // Clean up temp file
                    let _ = std::fs::remove_file(&audio_path);

                    // Send transcribed text back to main thread
                    let _ = tx.send(transcription);
                }
                Err(e) => {
                    let _ = tx.send(format!("[Audio Error: {}]", e));
                }
            }
        });
    }

    pub fn stop_audio_recording(&mut self) {
        // Audio recording is handled automatically after duration
        // This method can be used for manual stop in the future
    }

    pub fn check_audio_transcription(&mut self) {
        // Check if there's a transcription ready
        if let Ok(text) = self.audio_rx.try_recv() {
            // Update input with transcribed text
            self.input.content = text;
            self.input.cursor_position = self.input.content.len();
            self.audio_mode = false;
            self.audio_processing = false;
        }
    }

    /// Check if matrix animation should end
    pub fn check_matrix_animation(&mut self) {
        // Skip if animation sequence is active (handled by check_workspace_animation)
        if self.animation_sequence_active {
            return;
        }

        if self.show_matrix_animation {
            if let Some(start) = self.animation_start_time {
                if start.elapsed().as_secs() >= 5 {
                    self.show_matrix_animation = false;
                    self.animation_start_time = None;
                }
            }
        }
    }

    /// Start workspace switch animation sequence
    pub fn start_workspace_switch(&mut self, workspace_name: String) {
        self.switching_workspace = true;
        self.show_train_animation = true;
        self.animation_start_time = Some(Instant::now());
        self.current_workspace = Some(workspace_name);
    }

    /// Check workspace switch animation progress
    pub fn check_workspace_animation(&mut self) {
        // Handle '0' key animation sequence (matrix → train → splash)
        if self.animation_sequence_active {
            if let Some(start) = self.animation_start_time {
                let elapsed = start.elapsed().as_secs();

                // Check current state and transition
                if self.show_matrix_animation {
                    // Currently showing matrix (5 seconds)
                    if elapsed >= 5 {
                        // Matrix done, switch to train
                        self.show_matrix_animation = false;
                        self.show_train_animation = true;
                        self.animation_start_time = Some(Instant::now());
                    }
                } else if self.show_train_animation {
                    // Currently showing train (5 seconds)
                    if elapsed >= 5 {
                        // Train done, end sequence
                        self.show_train_animation = false;
                        self.animation_sequence_active = false;
                        self.animation_start_time = None;
                    }
                }
            }
            return;
        }

        // Handle first message matrix animation (not workspace switch)
        if self.show_matrix_animation && !self.switching_workspace {
            if let Some(start) = self.animation_start_time {
                let elapsed = start.elapsed().as_secs();
                // Stop matrix animation after 2 seconds for first message
                if elapsed >= 2 {
                    self.show_matrix_animation = false;
                    self.animation_start_time = None;
                }
            }
            return;
        }

        // Handle workspace switch animation sequence
        if self.switching_workspace {
            if let Some(start) = self.animation_start_time {
                let elapsed = start.elapsed().as_secs();

                // Train animation: 0-3 seconds
                if elapsed < 3 && self.show_train_animation {
                    return;
                }

                // DX splash: 3-4 seconds
                if elapsed >= 3 && elapsed < 4 {
                    self.show_train_animation = false;
                    self.show_dx_splash = true;
                    return;
                }

                // Matrix animation: 4-6 seconds
                if elapsed >= 4 && elapsed < 6 {
                    self.show_dx_splash = false;
                    self.show_matrix_animation = true;
                    return;
                }

                // End all animations
                if elapsed >= 6 {
                    self.show_matrix_animation = false;
                    self.switching_workspace = false;
                    self.animation_start_time = None;
                }
            }
        }
    }

    /// Create a new workspace
    pub fn create_workspace(&mut self, name: String) {
        // Add workspace creation logic here
        self.start_workspace_switch(name);
        self.workspace_create_mode = false;
        self.workspace_create_input.clear();
    }

    /// Open a modal with the most eye-catching raster-based animation
    pub fn open_modal(&mut self, modal_type: ModalType) {
        use tachyonfx::{Interpolation, fx};

        // Set the modal flag to true immediately
        match modal_type {
            ModalType::Focus => self.show_focus_menu = true,
            ModalType::Add => self.show_add_modal = true,
            ModalType::Plan => self.show_plan_modal = true,
            ModalType::Model => self.show_model_modal = true,
            ModalType::Local => self.show_local_modal = true,
            ModalType::Changes => self.show_changes_modal = true,
            ModalType::Tasks => self.show_tasks_modal = true,
            ModalType::Agents => self.show_agents_modal = true,
            ModalType::Memory => self.show_memory_modal = true,
            ModalType::Tools => self.show_tools_modal = true,
            ModalType::More => self.show_more_modal = true,
            ModalType::GoogleApi => self.show_google_api_modal = true,
            ModalType::ElevenlabsApi => self.show_elevenlabs_api_modal = true,
            ModalType::EffectsDemo => self.show_effects_demo_modal = true,
        }

        // Create the most eye-catching effect: coalesce (organic particle materialization)
        let coalesce_effect = fx::coalesce((400, Interpolation::BounceOut));

        // Add fade in for smooth appearance
        let fade_effect =
            fx::fade_from_fg(ratatui::style::Color::Black, (400, Interpolation::ExpoOut));

        // Combine effects in parallel for maximum eye-catching impact
        let combined = fx::parallel(&[coalesce_effect, fade_effect]);

        // Add effect
        self.modal_effect_manager.add_effect(combined);
        self.modal_opening = Some((modal_type, Instant::now()));
    }

    /// Close a modal with the most eye-catching raster-based animation
    pub fn close_modal(&mut self, modal_type: ModalType) {
        use tachyonfx::{Interpolation, fx};

        // Don't close if not open
        let is_open = match modal_type {
            ModalType::Focus => self.show_focus_menu,
            ModalType::Add => self.show_add_modal,
            ModalType::Plan => self.show_plan_modal,
            ModalType::Model => self.show_model_modal,
            ModalType::Local => self.show_local_modal,
            ModalType::Changes => self.show_changes_modal,
            ModalType::Tasks => self.show_tasks_modal,
            ModalType::Agents => self.show_agents_modal,
            ModalType::Memory => self.show_memory_modal,
            ModalType::Tools => self.show_tools_modal,
            ModalType::More => self.show_more_modal,
            ModalType::GoogleApi => self.show_google_api_modal,
            ModalType::ElevenlabsApi => self.show_elevenlabs_api_modal,
            ModalType::EffectsDemo => self.show_effects_demo_modal,
        };

        if !is_open {
            return;
        }

        // Create the most eye-catching exit: dissolve (organic particle disintegration)
        let dissolve_effect = fx::dissolve((400, Interpolation::BackIn));

        // Add fade to black for dramatic exit
        let fade_effect =
            fx::fade_to_fg(ratatui::style::Color::Black, (400, Interpolation::ExpoIn));

        // Combine effects in parallel for maximum eye-catching impact
        let combined = fx::parallel(&[dissolve_effect, fade_effect]);

        // Add effect
        self.modal_effect_manager.add_effect(combined);
        self.modal_closing = Some((modal_type, Instant::now()));
    }

    /// Check if modal animation is complete and update state accordingly
    pub fn update_modal_animations(&mut self) {
        const ANIMATION_DURATION_MS: u128 = 400; // Updated to match new animation duration

        // Check if closing animation is complete
        if let Some((closing_modal, start_time)) = self.modal_closing {
            if start_time.elapsed().as_millis() >= ANIMATION_DURATION_MS {
                // Animation complete, actually close the modal
                match closing_modal {
                    ModalType::Focus => self.show_focus_menu = false,
                    ModalType::Add => self.show_add_modal = false,
                    ModalType::Plan => self.show_plan_modal = false,
                    ModalType::Model => self.show_model_modal = false,
                    ModalType::Local => self.show_local_modal = false,
                    ModalType::Changes => self.show_changes_modal = false,
                    ModalType::Tasks => self.show_tasks_modal = false,
                    ModalType::Agents => self.show_agents_modal = false,
                    ModalType::Memory => self.show_memory_modal = false,
                    ModalType::Tools => self.show_tools_modal = false,
                    ModalType::More => self.show_more_modal = false,
                    ModalType::GoogleApi => self.show_google_api_modal = false,
                    ModalType::ElevenlabsApi => self.show_elevenlabs_api_modal = false,
                    ModalType::EffectsDemo => self.show_effects_demo_modal = false,
                }
                self.modal_closing = None;
            }
        }

        // Check if opening animation is complete
        if let Some((_opening_modal, start_time)) = self.modal_opening {
            if start_time.elapsed().as_millis() >= ANIMATION_DURATION_MS {
                self.modal_opening = None;
            }
        }
    }
}

impl Default for ChatApp {
    fn default() -> Self {
        Self::new()
    }
}
