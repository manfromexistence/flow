use anyhow::Result;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute,
    terminal::{
        Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode,
        enable_raw_mode,
    },
};
use figlet_rs::FIGfont;
use owo_colors::OwoColorize;
use rand::seq::SliceRandom;
use std::{
    io::{self, Write},
    time::{Duration, Instant},
};

const SPLASH_TEXT: &str = "Dx";
const SUBTITLE: &str = "Enhanced Development Experience";
const FONT_CHANGE_INTERVAL: Duration = Duration::from_secs(5);

/// Renders an animated splash screen with cycling figlet fonts
pub fn show_splash() -> Result<()> {
    // Only use fonts known to work with figlet-rs (tested and verified)
    let fonts = vec![
        "Block",
        "Colossal",
        "Banner3",
        "Banner4",
        "Doom",
        "Epic",
        "Graffiti",
        "Isometric1",
        "Isometric2",
        "Isometric3",
        "Isometric4",
        "Ogre",
        "Slant",
        "Shadow",
        "3d",
        "3d_ascii",
        "Broadway",
        "Chunky",
        "Cyberlarge",
        "Cybermedium",
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
        "Stampatello",
        "Stellar",
        "Thick",
        "Thin",
        "ansi_shadow",
        "big_chief",
        "big_money_ne",
        "big_money_nw",
        "big_money_se",
        "big_money_sw",
        "banner3_d",
        "Bloody",
        "Bolger",
        "Braced",
        "Bright",
        "Bulbhead",
        "Caligraphy",
        "Caligraphy2",
        "Cards",
        "Catwalk",
        "Coinstak",
        "Computer",
        "Contessa",
        "Contrast",
        "Cosmike",
        "Crawford",
        "Crawford2",
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
        "Georgi16",
        "Georgia11",
        "Goofy",
        "Hex",
        "Invita",
        "Italic",
        "Jazmine",
        "Jerusalem",
        "Katakana",
        "Kban",
        "Keyboard",
        "LCD",
        "Letters",
        "Linux",
        "Lockergnome",
        "Madrid",
        "Marquee",
        "Mike",
        "Mirror",
        "Mnemonic",
        "Moscow1",
        "NScript",
        "Nancyj",
        "Nipples",
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
        "Runyc",
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
    ];

    // Filter to only fonts that actually exist and render successfully
    let mut working_fonts = Vec::new();
    for font_name in fonts {
        if load_and_render_font(font_name).is_some() {
            working_fonts.push(font_name.to_string());
        }
    }

    if working_fonts.is_empty() {
        eprintln!("No figlet fonts found!");
        return Ok(());
    }

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;

    let result = run_splash_loop(&working_fonts);

    disable_raw_mode()?;
    execute!(stdout, LeaveAlternateScreen, cursor::Show)?;

    result
}

fn run_splash_loop(fonts: &[String]) -> Result<()> {
    let mut current_font_index = 0;
    let mut last_change = Instant::now();
    let mut current_rendered: Option<Vec<String>> = None;
    let mut last_displayed_font = String::new();

    loop {
        // Check for key press to exit (non-blocking)
        if event::poll(Duration::from_millis(16))?
            && let Event::Key(key) = event::read()?
            && matches!(key.code, KeyCode::Char('q') | KeyCode::Esc | KeyCode::Enter)
        {
            break;
        }

        // Change font every 5 seconds
        let should_change = last_change.elapsed() >= FONT_CHANGE_INTERVAL;
        if should_change {
            // Skip to next working font
            loop {
                current_font_index = (current_font_index + 1) % fonts.len();
                let test_render = load_and_render_font(&fonts[current_font_index]);
                if test_render.is_some() {
                    break;
                }
                // If we've tried all fonts and none work, break
                if current_font_index == 0 {
                    break;
                }
            }
            last_change = Instant::now();
            current_rendered = None; // Force re-render
        }

        // Only render if font changed
        let current_font = &fonts[current_font_index];
        if current_font != &last_displayed_font {
            // Load and render new font
            if current_rendered.is_none() {
                current_rendered = load_and_render_font(current_font);
            }

            // Display (only when changed)
            if current_rendered.is_some() {
                render_splash(&current_rendered)?;
                last_displayed_font = current_font.clone();
            }
        }

        // Sleep to reduce CPU usage
        std::thread::sleep(Duration::from_millis(100));
    }

    Ok(())
}

fn load_and_render_font(font_name: &str) -> Option<Vec<String>> {
    if let Ok(font_data) = dx_font::figlet::read_font(font_name)
        && let Ok(font_str) = String::from_utf8(font_data)
        && let Ok(font) = FIGfont::from_content(&font_str)
        && let Some(figure) = font.convert(SPLASH_TEXT)
    {
        return Some(figure.to_string().lines().map(|s| s.to_string()).collect());
    }
    None
}

fn render_splash(rendered: &Option<Vec<String>>) -> Result<()> {
    let mut stdout = io::stdout();

    // Clear and move to top
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;

    println!();
    println!();

    // Display rendered figlet text or fallback
    if let Some(lines) = rendered {
        for line in lines {
            println!("  {}", line.bright_cyan().bold());
        }
    } else {
        // Fallback block text
        display_block_text(SPLASH_TEXT);
    }

    println!();
    println!();
    println!("  {}", SUBTITLE.bright_cyan());
    println!();
    println!();
    println!(
        "  {} Press {} {} or {} to continue",
        "→".bright_cyan(),
        "Enter".bright_yellow(),
        ",".bright_black(),
        "Esc".bright_yellow()
    );

    stdout.flush()?;
    Ok(())
}

fn display_block_text(text: &str) {
    // Simple block letter rendering as fallback
    let lines = match text {
        "Dx" => vec![
            "  ██████╗ ██╗  ██╗",
            "  ██╔══██╗╚██╗██╔╝",
            "  ██║  ██║ ╚███╔╝ ",
            "  ██║  ██║ ██╔██╗ ",
            "  ██████╔╝██╔╝ ██╗",
            "  ╚═════╝ ╚═╝  ╚═╝",
        ],
        _ => vec![text],
    };

    for line in lines {
        println!("{}", line.bright_cyan().bold());
    }
}

/// Quick splash that shows for a brief moment on startup
pub fn show_quick_splash() {
    println!();
    display_block_text("Dx");
    println!();
    println!("  {}", SUBTITLE.bright_cyan());
    println!();
    std::thread::sleep(Duration::from_millis(800));
}
