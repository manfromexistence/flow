//! ASCII art splash screen with rainbow colors

use crate::effects::RainbowEffect;
use figlet_rs::FIGfont;
use owo_colors::OwoColorize;
use std::io::{self, Write};

pub fn render_dx_logo(rainbow: &RainbowEffect) -> io::Result<()> {
    let all_fonts = get_valid_fonts();
    let current_font = all_fonts[0]; // Use first font (Block)

    let figlet_lines = if let Ok(font_data) = dx_font::figlet::read_font(current_font)
        && let Ok(font_str) = String::from_utf8(font_data)
        && let Ok(font) = FIGfont::from_content(&font_str)
        && let Some(figure) = font.convert("DX")
    {
        figure.to_string().lines().map(|s| s.to_string()).collect()
    } else {
        // Fallback ASCII art
        vec![
            "██████╗ ██╗  ██╗".to_string(),
            "██╔══██╗╚██╗██╔╝".to_string(),
            "██║  ██║ ╚███╔╝ ".to_string(),
            "██║  ██║ ██╔██╗ ".to_string(),
            "██████╔╝██╔╝ ██╗".to_string(),
            "╚═════╝ ╚═╝  ╚═╝".to_string(),
        ]
    };

    // Render each line with rainbow colors
    for line in figlet_lines {
        for (i, ch) in line.chars().enumerate() {
            let color = rainbow.color_at(i);
            print!("{}", ch.to_string().truecolor(color.r, color.g, color.b));
        }
        println!();
    }

    io::stdout().flush()?;
    Ok(())
}

pub fn render_train_animation(rainbow: &RainbowEffect, frame: usize) -> io::Result<()> {
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

    // Add animated smoke
    let smoke_frames: Vec<&[&str]> = vec![
        &["    (  )", "   (    )", "  (      )"],
        &["   (   )", "  (     )", " (       )"],
        &["  (    )", " (      )", "(        )"],
    ];
    let smoke_frame_idx = frame % smoke_frames.len();
    let smoke = smoke_frames[smoke_frame_idx];

    // Calculate train position (moves from right to left)
    let terminal_width = 120i32; // Assume standard width
    let train_width = 60i32;
    let total_travel = terminal_width + train_width + 20;
    let x_pos = (terminal_width - 10) - ((frame as i32 * 2) % total_travel);

    // Render smoke above the train
    for smoke_line in smoke {
        let smoke_x_offset = x_pos + 6; // position smoke above the smokestack
        if smoke_x_offset >= 0 && smoke_x_offset < terminal_width {
            print!("{}", " ".repeat(smoke_x_offset as usize));
            for (ci, ch) in smoke_line.chars().enumerate() {
                let color = rainbow.color_at(ci + frame);
                print!("{}", ch.to_string().truecolor(color.r, color.g, color.b));
            }
        }
        println!();
    }

    // Render train
    for (line_idx, line) in train.iter().enumerate() {
        if x_pos >= 0 && x_pos < terminal_width {
            print!("{}", " ".repeat(x_pos as usize));
            for (char_idx, ch) in line.chars().enumerate() {
                let color_idx = char_idx + line_idx * 3 + frame;
                let color = rainbow.color_at(color_idx);
                print!("{}", ch.to_string().truecolor(color.r, color.g, color.b));
            }
        }
        println!();
    }

    // Render animated tracks
    let track_offset = frame;
    for x in 0..terminal_width as usize {
        let ch = if (x + track_offset).is_multiple_of(4) {
            '╫'
        } else {
            '═'
        };
        let color = rainbow.color_at(x + frame);
        print!("{}", ch.to_string().truecolor(color.r, color.g, color.b));
    }
    println!();

    io::stdout().flush()?;
    Ok(())
}

fn get_valid_fonts() -> Vec<&'static str> {
    vec![
        // Fonts verified to work with figlet-rs
        "Block",
        "Colossal", 
        "Banner3",
        "Doom",
        "Epic",
        "Graffiti",
        "Isometric1",
        "Isometric2",
        "Ogre",
        "Slant",
        "Shadow",
        "3d",
        "Broadway",
        "Chunky",
        "Cyberlarge",
        "Doh",
        "Gothic",
        "Graceful",
        "Gradient",
        "Hollywood",
        "Lean",
        "Mini",
        "Rounded",
        "Small",
        "Speed",
        "Stellar",
        "Thick",
        "Thin",
        "ansi_shadow",
        "big_chief",
        "banner3_d",
        "Bloody",
        "Bolger",
        "Braced",
        "Bright",
        "Bulbhead",
        "Caligraphy",
        "Cards",
        "Catwalk",
        "Computer",
        "Contrast",
        "Crawford",
        "Cricket",
        "Cursive",
        "Cybersmall",
        "Cygnet",
        "DANC4",
        "Decimal",
        "Diamond",
        "Double",
        "Electronic",
        "Elite",
        "Fender",
        "Fraktur",
        "Fuzzy",
        "Goofy",
        "Hex",
        "Invita",
        "Italic",
        "Jazmine",
        "Jerusalem",
        "Katakana",
        "Keyboard",
        "LCD",
        "Letters",
        "Linux",
        "Madrid",
        "Marquee",
        "Mike",
        "Mirror",
        "Mnemonic",
        "Moscow1",
        "NScript",
        "Nancyj",
        "O8",
        "OS2",
        "Octal",
        "Pawp",
        "Peaks",
        "Pebbles",
        "Pepper",
        "Poison",
        "Puffy",
        "Puzzle",
        "Rectangles",
        "Relief",
        "Relief2",
        "Reverse",
        "Roman",
        "Rozzo",
        "Runic",
        "Script",
        "Serifcap",
        "Shimrod",
        "Short",
        "Slide",
        "Stacey",
        "Stampate",
        "Stop",
        "Straight",
        "Swan",
        "THIS",
        "Tanja",
        "Tengwar",
        "Test1",
        "Ticks",
        "Tiles",
        "Tombstone",
        "Trek",
        "Tubular",
        "Univers",
        "Weird",
        "Whimsy",
    ]
}