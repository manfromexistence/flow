use crate::{
    components::Message,
    effects::{RainbowEffect, ShimmerEffect, TypingIndicator},
    input::{InputAction, InputState},
    llm::LocalLlm,
    theme::ChatTheme,
};
use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::{
    io,
    sync::{
        Arc,
        mpsc::{Receiver, Sender, channel},
    },
    time::{Duration, Instant},
};

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

    #[allow(dead_code)]
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
        }
    }
}

pub struct ChatApp {
    pub theme: ChatTheme,
    pub input: InputState,
    pub messages: Vec<Message>,
    pub is_loading: bool,
    pub typing_indicator: TypingIndicator,
    pub should_quit: bool,
    pub cursor_visible: bool,
    pub splash_font_index: usize,
    pub last_font_change: Instant,
    pub animation_mode: bool,
    pub current_animation_index: usize,
    pub animation_start_time: Option<Instant>,
    pub llm: Arc<LocalLlm>,
    pub llm_tx: Sender<String>,
    pub llm_rx: Receiver<String>,
    // Additional fields needed for render.rs
    pub rainbow_animation: RainbowEffect,
    pub rainbow_cursor: RainbowEffect,
    pub shimmer: ShimmerEffect,
    pub last_render: Instant,
    // Stub fields for compatibility (not used in minimal version)
    #[allow(dead_code)]
    pub tachyon_last_tick: Duration,
    #[allow(dead_code)]
    pub show_effects_demo_modal: bool,
    #[allow(dead_code)]
    pub show_train_animation: bool,
    #[allow(dead_code)]
    pub show_matrix_animation: bool,
    #[allow(dead_code)]
    pub input_area: ratatui::layout::Rect,
    #[allow(dead_code)]
    pub plan_button_area: ratatui::layout::Rect,
    #[allow(dead_code)]
    pub model_button_area: ratatui::layout::Rect,
    #[allow(dead_code)]
    pub local_button_area: ratatui::layout::Rect,
    #[allow(dead_code)]
    pub show_dx_splash: bool,
    #[allow(dead_code)]
    pub chat_scroll_offset: usize,
    #[allow(dead_code)]
    pub audio_processing: bool,
    #[allow(dead_code)]
    pub last_shortcut_pressed: Option<String>,
    #[allow(dead_code)]
    pub last_shortcut_time: Instant,
    #[allow(dead_code)]
    pub focus: u8, // Stub for Focus enum
    #[allow(dead_code)]
    pub shortcut_index: usize,
    #[allow(dead_code)]
    pub mode: u8, // Stub for ChatMode enum
    #[allow(dead_code)]
    pub selected_local_mode: String,
    #[allow(dead_code)]
    pub selected_model: String,
}

impl ChatApp {
    pub fn new() -> Self {
        let (llm_tx, llm_rx) = channel();
        Self {
            theme: ChatTheme::dark_fallback(),
            input: InputState::new(),
            messages: Vec::new(),
            is_loading: false,
            typing_indicator: TypingIndicator::new(),
            should_quit: false,
            cursor_visible: true,
            splash_font_index: 0,
            last_font_change: Instant::now(),
            animation_mode: false,
            current_animation_index: 0,
            animation_start_time: None,
            llm: Arc::new(LocalLlm::new()),
            llm_tx,
            llm_rx,
            rainbow_animation: RainbowEffect::new(),
            rainbow_cursor: RainbowEffect::new(),
            shimmer: ShimmerEffect::new(vec![
                ratatui::style::Color::Rgb(150, 150, 150), // Base gray color
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
        }
    }

    pub async fn run(&mut self) -> Result<()> {
        if let Err(e) = self.llm.initialize().await {
            eprintln!("Failed to initialize LLM: {}", e);
        }

        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, crossterm::cursor::Hide)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        let result = self.run_loop(&mut terminal);

        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            crossterm::cursor::Show
        )?;
        terminal.show_cursor()?;

        result
    }

    fn run_loop(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
        loop {
            terminal.draw(|f| self.render(f))?;

            if self.should_quit {
                break;
            }

            if event::poll(Duration::from_millis(50))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.handle_key(key);
                    }
                }
            }

            self.update();
        }
        Ok(())
    }

    fn handle_key(&mut self, key: event::KeyEvent) {
        // Handle thinking accordion toggle with '0' key
        if key.code == KeyCode::Char('0') {
            if !self.messages.is_empty() {
                // Toggle thinking expansion for the last assistant message
                for msg in self.messages.iter_mut().rev() {
                    if msg.role == crate::components::MessageRole::Assistant {
                        msg.thinking_expanded = !msg.thinking_expanded;
                        break;
                    }
                }
                return;
            }
        }
        
        // Handle scrolling when messages exist
        if !self.messages.is_empty() && self.input.content.is_empty() {
            match key.code {
                KeyCode::Up => {
                    self.chat_scroll_offset = self.chat_scroll_offset.saturating_sub(1);
                    return;
                }
                KeyCode::Down => {
                    // Calculate max scroll based on content height using MessageList's calculation
                    let message_list = crate::components::MessageList::with_effects(
                        &self.messages,
                        &self.theme,
                        0,
                        &self.shimmer,
                        &self.typing_indicator,
                    );
                    let total_height = message_list.calculate_total_height();
                    let viewport_height = self.get_chat_viewport_height();
                    let max_scroll = total_height.saturating_sub(viewport_height);
                    
                    self.chat_scroll_offset = (self.chat_scroll_offset + 1).min(max_scroll);
                    return;
                }
                _ => {}
            }
        }
        
        // Handle animation navigation when input is empty and no messages
        if self.input.content.is_empty() && self.messages.is_empty() {
            match key.code {
                KeyCode::Left => {
                    self.handle_animation_previous();
                    return;
                }
                KeyCode::Right => {
                    self.handle_animation_next();
                    return;
                }
                _ => {}
            }
        }

        let action = self.input.handle_key(key);
        match action {
            InputAction::Submit(msg) => {
                self.send_message(msg);
            }
            InputAction::Exit => {
                self.should_quit = true;
            }
            _ => {}
        }
    }

    fn handle_animation_previous(&mut self) {
        let animations = AnimationType::all();
        if !self.animation_mode {
            self.animation_mode = true;
            self.current_animation_index = animations.len() - 1;
        } else {
            if self.current_animation_index == 0 {
                self.current_animation_index = animations.len() - 1;
            } else {
                self.current_animation_index -= 1;
            }
        }
        self.animation_start_time = Some(Instant::now());
    }

    fn handle_animation_next(&mut self) {
        let animations = AnimationType::all();
        if !self.animation_mode {
            self.animation_mode = true;
            self.current_animation_index = if self.messages.is_empty() { 1 } else { 0 };
        } else {
            self.current_animation_index = (self.current_animation_index + 1) % animations.len();
        }
        self.animation_start_time = Some(Instant::now());
    }

    fn send_message(&mut self, content: String) {
        // Exit animation mode when sending a message
        self.animation_mode = false;
        
        self.messages.push(Message::user(content.clone()));
        self.is_loading = true;
        self.messages.push(Message::assistant(String::new()));

        let llm = self.llm.clone();
        let tx = self.llm_tx.clone();

        tokio::spawn(async move {
            let tx_clone = tx.clone();
            match llm
                .generate_stream(&content, move |chunk| {
                    let _ = tx_clone.send(chunk);
                })
                .await
            {
                Ok(_) => {
                    let _ = tx.send("\n__END__".to_string());
                }
                Err(e) => {
                    let _ = tx.send(format!("Error: {}", e));
                    let _ = tx.send("\n__END__".to_string());
                }
            }
        });
    }

    fn update(&mut self) {
        if let Ok(chunk) = self.llm_rx.try_recv() {
            if chunk == "\n__END__" {
                self.is_loading = false;
            } else if let Some(last_msg) = self.messages.last_mut() {
                last_msg.content.push_str(&chunk);
            }
        }

        if self.is_loading {
            self.typing_indicator.update();
        }

        if self.messages.is_empty() && self.last_font_change.elapsed() >= Duration::from_secs(3) {
            self.splash_font_index = (self.splash_font_index + 1) % 382;
            self.last_font_change = Instant::now();
        }
    }

    /// Get the chat viewport height (terminal height - input box - status bar)
    fn get_chat_viewport_height(&self) -> usize {
        // Approximate: terminal height - input box (3 lines) - status bar (1 line)
        // This is a rough estimate; actual value comes from layout
        let term_height = crossterm::terminal::size().map(|(_, h)| h).unwrap_or(24);
        (term_height as usize).saturating_sub(4)
    }
}
