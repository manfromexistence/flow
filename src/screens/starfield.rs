//! Starfield animation screen - Stars flying toward the viewer

use std::io::{self, Write};
use terminal_size::{Width, Height, terminal_size};
use owo_colors::OwoColorize;

pub struct StarfieldScreen {
    width: usize,
    height: usize,
}

impl StarfieldScreen {
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
        let star_chars = ['.', '·', '*', '✦', '★'];
        
        // Clear screen
        print!("\x1B[2J\x1B[H");
        
        println!("🌌 Starfield\n");
        
        // Generate stars moving toward viewer
        for i in 0..100 {
            let star_z = ((frame + i * 3) % 100) as f32;
            let star_x = (i * 7) % self.width.min(80);
            let star_y = (i * 11) % self.height.min(20);
            
            // Project 3D to 2D
            let depth = star_z / 100.0;
            let size_idx = (depth * (star_chars.len() - 1) as f32) as usize;
            let brightness = (depth * 255.0) as u8;
            
            if star_y < self.height.min(20) && star_x < self.width.min(80) {
                print!("\x1B[{};{}H", (star_y + 3).max(1), (star_x + 1).max(1));
                print!("{}", star_chars[size_idx].to_string().truecolor(brightness, brightness, brightness));
            }
        }
        
        io::stdout().flush()
    }
}