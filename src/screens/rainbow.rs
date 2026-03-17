//! Rainbow animation screen - Flowing rainbow colored text

use std::io::{self, Write};
use terminal_size::{Width, Height, terminal_size};
use owo_colors::OwoColorize;

pub struct RainbowScreen {
    width: usize,
    height: usize,
}

impl RainbowScreen {
    pub fn new() -> Self {
        let size = terminal_size();
        let (width, height) = if let Some((Width(w), Height(h))) = size {
            (w as usize, h as usize)
        } else {
            (120, 30)
        };
        
        Self { width, height }
    }

    fn hsv_to_rgb(&self, h: f32, s: f32, v: f32) -> (u8, u8, u8) {
        let c = v * s;
        let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
        let m = v - c;
        
        let (r, g, b) = if h < 60.0 {
            (c, x, 0.0)
        } else if h < 120.0 {
            (x, c, 0.0)
        } else if h < 180.0 {
            (0.0, c, x)
        } else if h < 240.0 {
            (0.0, x, c)
        } else if h < 300.0 {
            (x, 0.0, c)
        } else {
            (c, 0.0, x)
        };
        
        (
            ((r + m) * 255.0) as u8,
            ((g + m) * 255.0) as u8,
            ((b + m) * 255.0) as u8,
        )
    }

    pub fn render(&self, frame: usize) -> io::Result<()> {
        let text = "RAINBOW COLORS FLOWING ACROSS THE TERMINAL";
        let y_pos = self.height / 2;
        
        // Clear screen
        print!("\x1B[2J\x1B[H");
        
        println!("🌈 Rainbow Text\n");
        
        print!("\x1B[{};1H", y_pos);
        for (i, ch) in text.chars().enumerate() {
            let hue = ((i * 10 + frame * 5) % 360) as f32;
            let color = self.hsv_to_rgb(hue, 1.0, 1.0);
            print!("{}", ch.to_string().truecolor(color.0, color.1, color.2));
        }
        
        io::stdout().flush()
    }
}