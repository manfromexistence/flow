//! Bubbles animation screen - Floating bubbles rising upward

use std::io::{self, Write};
use terminal_size::{Width, Height, terminal_size};
use owo_colors::OwoColorize;

pub struct BubblesScreen {
    width: usize,
    height: usize,
}

impl BubblesScreen {
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
        let bubble_chars = ['○', '◯', '●', '◉'];
        
        // Clear screen
        print!("\x1B[2J\x1B[H");
        
        println!("🫧 Bubbles\n");
        
        // Generate bubbles
        for i in 0..15 {
            let x = (i * 8 + frame * 2) % self.width.min(80);
            let y_offset = (frame + i * 3) % (self.height.min(20) + 10);
            
            if y_offset < self.height.min(20) {
                let y = self.height.min(20) - 1 - y_offset;
                if y > 2 && y < self.height.min(20) {
                    print!("\x1B[{};{}H", y + 3, (x + 1).min(self.width.min(80)));
                    let bubble_idx = (frame + i) % bubble_chars.len();
                    
                    // Bubble colors - light blues and whites
                    let colors = [
                        (173, 216, 230), // Light blue
                        (135, 206, 235), // Sky blue
                        (176, 224, 230), // Powder blue
                        (255, 255, 255), // White
                    ];
                    let color = colors[i % colors.len()];
                    
                    print!("{}", bubble_chars[bubble_idx].to_string().truecolor(color.0, color.1, color.2));
                }
            }
        }
        
        io::stdout().flush()
    }
}