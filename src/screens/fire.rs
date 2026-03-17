//! Fire animation screen - Classic ASCII fire simulation

use std::io::{self, Write};
use terminal_size::{Width, Height, terminal_size};
use owo_colors::OwoColorize;
use rand::Rng;

pub struct FireScreen {
    width: usize,
    height: usize,
}

impl FireScreen {
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
        let mut rng = rand::thread_rng();
        let fire_chars = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
        
        // Clear screen
        print!("\x1B[2J\x1B[H");
        
        println!("🔥 Fire Animation\n");
        
        for y in 0..self.height.min(20) {
            for x in 0..self.width.min(80) {
                let heat = if y > self.height.min(20) - 3 {
                    rng.gen_range(7..10) // Hot bottom
                } else {
                    let base_heat = 10 - (y * 10 / self.height.min(20));
                    let noise = (frame + x + y) % 3;
                    (base_heat + noise).min(9)
                };
                
                let ch = fire_chars[heat];
                
                // Fire colors: red to yellow to white
                let color = match heat {
                    0..=2 => (50, 0, 0),
                    3..=4 => (150, 50, 0),
                    5..=6 => (255, 100, 0),
                    7..=8 => (255, 200, 0),
                    _ => (255, 255, 100),
                };
                
                print!("{}", ch.to_string().truecolor(color.0, color.1, color.2));
            }
            println!();
        }
        
        io::stdout().flush()
    }
}