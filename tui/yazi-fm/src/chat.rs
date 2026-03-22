use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::llm::LocalLlm;
use crate::chat_input::{InputState, InputAction};
use crate::chat_components::{Message, MessageRole};

use std::sync::{Arc, mpsc::{Sender, Receiver, channel}};

pub struct ChatPanel {
    pub input: InputState,
    pub messages: Vec<Message>,
    pub is_loading: bool,
    pub llm: Arc<LocalLlm>,
    pub llm_tx: Sender<String>,
    pub llm_rx: Receiver<String>,
}

impl ChatPanel {
    pub fn new() -> Self {
        let (llm_tx, llm_rx) = channel();
        
        Self {
            input: InputState::new(),
            messages: Vec::new(),
            is_loading: false,
            llm: Arc::new(LocalLlm::new()),
            llm_tx,
            llm_rx,
        }
    }
    
    pub async fn initialize(&self) {
        if let Err(e) = self.llm.initialize().await {
            eprintln!("Failed to initialize LLM: {}", e);
        }
    }
    
    pub fn handle_input(&mut self, key: crossterm::event::KeyEvent) -> InputAction {
        self.input.handle_key(key)
    }
    
    pub fn send_message(&mut self, content: String) {
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
    
    pub fn update(&mut self) {
        if let Ok(chunk) = self.llm_rx.try_recv() {
            if chunk == "\n__END__" {
                self.is_loading = false;
            } else if let Some(last_msg) = self.messages.last_mut() {
                last_msg.content.push_str(&chunk);
            }
        }
    }
}

pub struct ChatWidget<'a> {
    panel: &'a ChatPanel,
}

impl<'a> ChatWidget<'a> {
    pub fn new(panel: &'a ChatPanel) -> Self {
        Self { panel }
    }
}

impl Widget for ChatWidget<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use ratatui::widgets::{Block, Borders, Paragraph};
        use ratatui::style::{Style, Color};
        
        // Simple chat input at the bottom
        let input_text = format!("> {}", self.panel.input.content);
        let input_widget = Paragraph::new(input_text)
            .block(Block::default()
                .borders(Borders::ALL)
                .title(" Chat ")
                .style(Style::default().fg(Color::Cyan)))
            .style(Style::default().fg(Color::White));
        
        input_widget.render(area, buf);
    }
}
