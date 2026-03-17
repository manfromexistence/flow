//! Spinners animation screen - Various loading spinner styles

use std::io::{self, Write};
use terminal_size::{Width, Height, terminal_size};
use owo_colors::OwoColorize;

pub struct SpinnersScreen {
    width: usize,
    height: usize,
}

impl SpinnersScreen {
    pub fn new() -> Self {
        let size = terminal_size();
        let (width, height) = if let Some((Width(w), Height(h))) = size {
            (w as usize, h as usize)
        } else {
            (120, 30)
        };
        
        Self { width, height }
    }

    pub fn render(&self, frame: usize) -> io::Result<()> {
        let spinners: Vec<Vec<char>> = vec![
            vec!['|', '/', '-', '\\'],
            vec!['◐', '◓', '◑', '◒'],
            vec!['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'],
            vec!['▁', '▃', '▄', '▅', '▆', '▇', '█', '▇', '▆', '▅', '▄', '▃'],
            vec!['◜', '◠', '◝', '◞', '◡', '◟'],
            vec!['⣾', '⣽', '⣻', '⢿', '⡿', '⣟', '⣯', '⣷'],
        ];
        
        let spinner_names = [
            "Classic",
            "Dots",
            "Braille",
            "Blocks",
            "Arc",
            "Braille Dots",
        ];
        
        // Clear screen
        print!("\x1B[2J\x1B[H");
        
        println!("⏳ Spinners\n");
        
        for (i, (spinner_set, name)) in spinners.iter().zip(spinner_names.iter()).enumerate() {
            let y = i * 3 + 5;
            if y < self.height.min(20) {
                let char_idx = frame % spinner_set.len();
                
                // Rainbow colors for each spinner
                let colors = [
                    (255, 0, 0),   // Red
                    (255, 165, 0), // Orange
                    (255, 255, 0), // Yellow
                    (0, 255, 0),   // Green
                    (0, 0, 255),   // Blue
                    (75, 0, 130),  // Indigo
                ];
                let color = colors[i % colors.len()];
                
                print!("\x1B[{};{}H", y, 20);
                print!("{} {}", 
                    spinner_set[char_idx].to_string().truecolor(color.0, color.1, color.2),
                    name
                );
            }
        }
        
        io::stdout().flush()
    }
}