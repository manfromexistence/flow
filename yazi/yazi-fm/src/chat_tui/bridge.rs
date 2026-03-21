use super::state::ChatState;
use yazi_core::Core;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    Chat,        // Chat TUI is active
    FilePicker,  // Yazi file picker is active (modal)
}

pub struct YaziChatBridge {
    pub chat_state: ChatState,
    pub mode: AppMode,
}

impl YaziChatBridge {
    pub fn new() -> Self {
        Self {
            chat_state: ChatState::new(),
            mode: AppMode::Chat,  // Start in Chat mode to show animations/messages
        }
    }
    
    #[allow(dead_code)]
    pub fn is_chat_mode(&self) -> bool {
        matches!(self.mode, AppMode::Chat)
    }
    
    #[allow(dead_code)]
    pub fn is_file_picker_mode(&self) -> bool {
        matches!(self.mode, AppMode::FilePicker)
    }
    
    #[allow(dead_code)]
    pub fn enter_file_picker(&mut self) {
        self.mode = AppMode::FilePicker;
        self.chat_state.show_file_picker = true;
    }
    
    #[allow(dead_code)]
    pub fn exit_file_picker(&mut self) {
        self.mode = AppMode::Chat;
        self.chat_state.show_file_picker = false;
    }
    
    #[allow(dead_code)]
    pub fn select_file(&mut self, core: &Core) {
        // Get the currently selected file from yazi
        let tab = core.active();
        let folder = &tab.current;
        
        if let Some(file) = folder.files.get(folder.cursor) {
            // Use into_local() to get PathBuf from UrlBuf
            if let Some(path) = file.url.clone().into_local() {
                self.chat_state.insert_file_path(path);
            }
        }
        
        self.exit_file_picker();
    }
}
