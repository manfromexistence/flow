use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use super::effects::RainbowEffect;

pub fn show_train_farewell() {
    let rainbow = RainbowEffect::new();
    
    // Clear the entire screen first
    print!("\x1B[2J"); // Clear screen
    print!("\x1B[H");  // Move cursor to home (top-left)
    let _ = io::stdout().flush();
    
    // Get terminal size
    let size = crossterm::terminal::size().unwrap_or((120, 30));
    let terminal_width = size.0 as usize;
    
    // Show train animation for 15 frames
    for frame in 0..15 {
        // Move cursor to top for each frame
        print!("\x1B[H");
        
        render_train_frame(&rainbow, frame, terminal_width);
        
        thread::sleep(Duration::from_millis(200));
    }
    
    println!();
}

fn render_train_frame(rainbow: &RainbowEffect, frame: usize, terminal_width: usize) {
    let elapsed_ms = frame * 200;
    let train_width = 55;
    
    // Train moves from right to left
    let total_travel = terminal_width + train_width + 10;
    let cycle_duration = 3000;
    let progress = (elapsed_ms % cycle_duration) as f32 / cycle_duration as f32;
    let x_pos = (terminal_width as f32 + 10.0 - progress * total_travel as f32) as i32;
    
    let train = vec![
        "      ====        ________                ___________",
        "  _D _|  |_______/        \\__I_I_____===__|_________|",
        "   |(_)---  |   H\\________/ |   |        =|___ ___|",
        "   /     |  |   H  |  |     |   |         ||_| |_||",
        "  |      |  |   H  |__--------------------| [___] |",
        "  | ________|___H__/__|_____/[][]~\\_______|       |",
        "  |/ |   |-----------I_____I [][] []  D   |=======|",
        "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|",
        " |/-=|___|=O=====O=====O=====O   |_____/~\\___/",
        "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/",
    ];
    
    // Smoke animation
    let smoke_frames: Vec<&[&str]> = vec![
        &["    (  )", "   (    )", "  (      )"],
        &["   (   )", "  (     )", " (       )"],
        &["  (    )", " (      )", "(        )"],
    ];
    let smoke_frame_idx = (elapsed_ms / 300) % smoke_frames.len();
    let smoke = smoke_frames[smoke_frame_idx];
    
    // Render smoke
    let smoke_x_offset = x_pos + 6;
    for smoke_line in smoke {
        // Clear line and render
        for x in 0..terminal_width {
            if smoke_x_offset >= 0 && x >= smoke_x_offset as usize && x < (smoke_x_offset as usize + smoke_line.len()) {
                let char_idx = x - smoke_x_offset as usize;
                if let Some(ch) = smoke_line.chars().nth(char_idx) {
                    let color_idx = (char_idx + (elapsed_ms / 200)) % 50;
                    let c = rainbow.rgb_color_at(color_idx);
                    print!("\x1B[38;2;{};{};{}m{}\x1B[0m", c.r, c.g, c.b, ch);
                    continue;
                }
            } else if smoke_x_offset < 0 {
                let visible_start = (-smoke_x_offset) as usize;
                if x < smoke_line.len().saturating_sub(visible_start)
                    && let Some(ch) = smoke_line.chars().nth(x + visible_start) {
                        let color_idx = (x + visible_start + (elapsed_ms / 200)) % 50;
                        let c = rainbow.rgb_color_at(color_idx);
                        print!("\x1B[38;2;{};{};{}m{}\x1B[0m", c.r, c.g, c.b, ch);
                        continue;
                    }
            }
            print!(" ");
        }
        println!();
    }
    
    // Render train
    for (line_idx, line) in train.iter().enumerate() {
        for x in 0..terminal_width {
            if x_pos >= 0 && x >= x_pos as usize && x < (x_pos as usize + line.len()) {
                let char_idx = x - x_pos as usize;
                if let Some(ch) = line.chars().nth(char_idx) {
                    let color_idx = (char_idx + line_idx * 3 + (elapsed_ms / 150)) % 50;
                    let c = rainbow.rgb_color_at(color_idx);
                    print!("\x1B[38;2;{};{};{}m{}\x1B[0m", c.r, c.g, c.b, ch);
                    continue;
                }
            } else if x_pos < 0 {
                let visible_start = (-x_pos) as usize;
                if x < line.len().saturating_sub(visible_start)
                    && let Some(ch) = line.chars().nth(x + visible_start) {
                        let color_idx = (x + visible_start + line_idx * 3 + (elapsed_ms / 150)) % 50;
                        let c = rainbow.rgb_color_at(color_idx);
                        print!("\x1B[38;2;{};{};{}m{}\x1B[0m", c.r, c.g, c.b, ch);
                        continue;
                    }
            }
            print!(" ");
        }
        println!();
    }
    
    // Render tracks
    for x in 0..terminal_width {
        let ch = if (x + (elapsed_ms / 300)).is_multiple_of(4) {
            '╫'
        } else {
            '═'
        };
        let color_idx = (x + (elapsed_ms / 300)) % 50;
        let c = rainbow.rgb_color_at(color_idx);
        print!("\x1B[38;2;{};{};{}m{}\x1B[0m", c.r, c.g, c.b, ch);
    }
    println!();
    
    let _ = io::stdout().flush();
}
