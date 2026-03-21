use crate::chat_tui::{
    autocomplete::Autocomplete,
    components::Message,
    effects::{RainbowEffect, ShimmerEffect, TypingIndicator},
    input::{InputAction, InputState},
    llm::LocalLlm,
    perf::PerfMonitor,
    tachyonfx_demo::TachyonDemo,
    theme::ChatTheme,
};
use std::{
    path::PathBuf,
    sync::{
        Arc,
        mpsc::{Receiver, Sender, channel},
    },
    time::{Duration, Instant},
};
use tachyonfx::SimpleRng;

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
    TachyonDemo,
    Fire,
    Plasma,
    Spinners,
    Waves,
    Fireworks,
    Yazi,
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
            Self::TachyonDemo,
            Self::Fire,
            Self::Plasma,
            Self::Spinners,
            Self::Waves,
            Self::Fireworks,
            Self::Yazi, // Last screen
        ]
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Splash => "Splash Screen",
            Self::Matrix => "Matrix Rain",
            Self::Train => "ASCII Train",
            Self::Confetti => "Confetti",
            Self::GameOfLife => "Game of Life",
            Self::Starfield => "Starfield",
            Self::Rain => "Rain",
            Self::NyanCat => "Nyan Cat",
            Self::DVDLogo => "DVD Logo",
            Self::TachyonDemo => "Tachyon FX Demo",
            Self::Fire => "Fire Animation",
            Self::Plasma => "Plasma Effect",
            Self::Spinners => "Spinners",
            Self::Waves => "Ocean Waves",
            Self::Fireworks => "Fireworks",
            Self::Yazi => "Yazi File Manager",
        }
    }
}

pub struct ChatState {
    pub theme: ChatTheme,
    pub input: InputState,
    pub messages: Vec<Message>,
    pub is_loading: bool,
    pub typing_indicator: TypingIndicator,
    pub cursor_visible: bool,
    pub splash_font_index: usize,
    pub last_font_change: Instant,
    pub animation_mode: bool,
    pub current_animation_index: usize,
    pub animation_start_time: Option<Instant>,
    pub llm: Arc<LocalLlm>,
    pub llm_tx: Sender<String>,
    pub llm_rx: Receiver<String>,
    pub rainbow_animation: RainbowEffect,
    pub rainbow_cursor: RainbowEffect,
    pub shimmer: ShimmerEffect,
    pub last_render: Instant,
    pub tachyon_last_tick: Duration,
    pub show_effects_demo_modal: bool,
    pub show_train_animation: bool,
    pub show_matrix_animation: bool,
    pub input_area: ratatui::layout::Rect,
    pub plan_button_area: ratatui::layout::Rect,
    pub model_button_area: ratatui::layout::Rect,
    pub local_button_area: ratatui::layout::Rect,
    pub show_dx_splash: bool,
    pub chat_scroll_offset: usize,
    pub audio_processing: bool,
    pub last_shortcut_pressed: Option<String>,
    pub last_shortcut_time: Instant,
    pub focus: u8,
    pub shortcut_index: usize,
    pub mode: u8,
    pub selected_local_mode: String,
    pub selected_model: String,
    pub autocomplete: Autocomplete,
    pub last_input_change: Instant,
    pub last_input_content: String,
    pub tachyon_demo: TachyonDemo,
    pub tachyon_rng: SimpleRng,
    pub last_frame_instant: Instant,
    pub show_tachyon_modal: bool,
    pub perf_monitor: PerfMonitor,
    pub show_perf_overlay: bool,
    pub last_keystroke_time: Duration,
    pub last_input_render_time: Duration,
    
    // NEW: File picker integration
    pub show_file_picker: bool,
    pub selected_file: Option<PathBuf>,
}

impl ChatState {
    pub fn new() -> Self {
        let (llm_tx, llm_rx) = channel();
        let theme = ChatTheme::dark_fallback();

        Self {
            theme: theme.clone(),
            input: InputState::new(),
            messages: Vec::new(),
            is_loading: false,
            typing_indicator: TypingIndicator::new(),
            cursor_visible: true,
            splash_font_index: 0,
            last_font_change: Instant::now(),
            animation_mode: true, // Start in animation mode to show splash
            current_animation_index: 0, // Start with splash
            animation_start_time: Some(Instant::now()),
            llm: Arc::new(LocalLlm::new()),
            llm_tx,
            llm_rx,
            rainbow_animation: RainbowEffect::new(),
            rainbow_cursor: RainbowEffect::new(),
            shimmer: ShimmerEffect::new(vec![
                ratatui::style::Color::Rgb(150, 150, 150),
            ]),
            last_render: Instant::now(),
            tachyon_last_tick: Duration::from_secs(0),
            show_effects_demo_modal: false,
            show_train_animation: false,
            show_matrix_animation: false,
            input_area: ratatui::layout::Rect::default(),
            plan_button_area: ratatui::layout::Rect::default(),
            model_button_area: ratatui::layout::Rect::default(),
            local_button_area: ratatui::layout::Rect::default(),
            show_dx_splash: false,
            chat_scroll_offset: 0,
            audio_processing: false,
            last_shortcut_pressed: None,
            last_shortcut_time: Instant::now(),
            focus: 0,
            shortcut_index: 0,
            mode: 0,
            selected_local_mode: "Local".to_string(),
            selected_model: "Qwen3.5-0.8B".to_string(),
            autocomplete: Autocomplete::new(theme.clone()),
            last_input_change: Instant::now(),
            last_input_content: String::new(),
            tachyon_demo: TachyonDemo::new(theme),
            tachyon_rng: SimpleRng::default(),
            last_frame_instant: Instant::now(),
            show_tachyon_modal: false,
            perf_monitor: PerfMonitor::new(),
            show_perf_overlay: false,
            last_keystroke_time: Duration::from_secs(0),
            last_input_render_time: Duration::from_secs(0),
            show_file_picker: false,
            selected_file: None,
        }
    }
    
    pub async fn initialize_llm(&self) {
        if let Err(e) = self.llm.initialize().await {
            eprintln!("Failed to initialize LLM: {}", e);
        }
    }
    
    pub fn insert_file_path(&mut self, path: PathBuf) {
        let path_str = path.to_string_lossy();
        self.input.content.push_str(&path_str);
        self.input.cursor_position = self.input.content.len();
        self.selected_file = Some(path);
    }
    
    pub fn toggle_file_picker(&mut self) {
        self.show_file_picker = !self.show_file_picker;
    }
    
    pub fn add_user_message(&mut self, content: String) {
        let message = Message::user(content);
        self.messages.push(message);
        
        // Exit animation mode when first message is sent
        if self.animation_mode {
            self.animation_mode = false;
        }
        
        // Reset scroll to bottom
        self.chat_scroll_offset = 0;
    }
}
