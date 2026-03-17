//! Plasma animation screen - Colorful sine wave plasma effect

use owo_colors::OwoColorize;
use std::io::{self, Write};
use terminal_size::{Height, Width, terminal_size};

pub struct PlasmaScreen {
    width: usize,
    height: usize,
}

impl PlasmaScreen {
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
        let t = frame as f32 * 0.1;

        // Clear screen
        print!("\x1B[2J\x1B[H");

        println!("🌊 Plasma Effect\n");

        for y in 0..self.height.min(20) {
            for x in 0..self.width.min(80) {
                let fx = x as f32 / 10.0;
                let fy = y as f32 / 10.0;

                let value = (fx + t).sin() + (fy + t).sin() + ((fx + fy) / 2.0 + t).sin();
                let _normalized = ((value + 3.0) / 6.0 * 255.0) as u8;

                // Create plasma colors
                let r = ((value + 1.0) * 127.5) as u8;
                let g = ((value.sin() + 1.0) * 127.5) as u8;
                let b = ((value.cos() + 1.0) * 127.5) as u8;

                let ch = if value > 1.0 {
                    '█'
                } else if value > 0.0 {
                    '▓'
                } else if value > -1.0 {
                    '▒'
                } else {
                    '░'
                };

                print!("{}", ch.to_string().truecolor(r, g, b));
            }
            println!();
        }

        io::stdout().flush()
    }
}
