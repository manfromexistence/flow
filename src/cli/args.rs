/// Command-line arguments
#[derive(Debug)]
pub struct Args {
    pub command: Command,
}

#[derive(Debug)]
pub enum Command {
    /// Transcribe audio file
    Transcribe { file: String },
    /// Full Wispr Flow pipeline (STT + LLM enhancement)
    Wispr { file: String },
    /// Speak text using TTS
    Speak { text: String },
    /// Live recording mode (microphone → STT → enhance → TTS)
    Live,
    /// Interactive mode
    Interactive,
}

impl Args {
    /// Parse command-line arguments
    pub fn parse() -> Self {
        let args: Vec<String> = std::env::args().collect();
        
        if args.len() < 2 {
            return Self {
                command: Command::Interactive,
            };
        }
        
        let command = match args[1].as_str() {
            "--transcribe" | "-t" => {
                let file = args.get(2).cloned().unwrap_or_else(|| "tests/fixtures/audio.mp3".to_string());
                Command::Transcribe { file }
            }
            "--wispr" | "-w" => {
                let file = args.get(2).cloned().unwrap_or_else(|| "tests/fixtures/audio.mp3".to_string());
                Command::Wispr { file }
            }
            "--speak" | "-s" => {
                let text = args[2..].join(" ");
                Command::Speak { text }
            }
            "--live" | "-l" => {
                Command::Live
            }
            _ => Command::Interactive,
        };
        
        Self { command }
    }
}
