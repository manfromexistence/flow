//! Splash screen rendering with figlet fonts

use super::theme::ChatTheme;
use crate::effects::RainbowEffect;
use figlet_rs::FIGlet;
use ratatui::{
	buffer::Buffer,
	layout::Rect,
	style::Style,
	text::{Line, Span},
	widgets::{Block, Paragraph, Widget},
};

pub fn render(
	area: Rect,
	buf: &mut Buffer,
	theme: &ChatTheme,
	font_index: usize,
	rainbow: &RainbowEffect,
) {
	let all_fonts = get_valid_fonts();
	let current_font = all_fonts[font_index % all_fonts.len()];

	let figlet_lines: Vec<String> = if let Ok(font_data) = crate::font::read_font(current_font)
		&& let Ok(font_str) = String::from_utf8(font_data)
		&& let Ok(font) = FIGlet::from_content(&font_str)
		&& let Some(figure) = font.convert("DX")
	{
		figure.to_string().lines().map(|s| s.to_string()).collect()
	} else {
		vec![]
	};

	let mut splash_lines = vec![];

	if !figlet_lines.is_empty() {
		// Apply rainbow colors to each character using ratatui Spans
		for line in figlet_lines {
			let mut spans = Vec::new();
			for (i, ch) in line.chars().enumerate() {
				let color = rainbow.color_at(i);
				spans.push(Span::styled(ch.to_string(), Style::default().fg(color)));
			}
			splash_lines.push(Line::from(spans));
		}
	} else {
		// Fallback with rainbow colors
		let text = "DX";
		let mut spans = vec![Span::styled("▸ ", Style::default().fg(theme.accent))];
		for (i, ch) in text.chars().enumerate() {
			let color = rainbow.color_at(i);
			spans.push(Span::styled(ch.to_string(), Style::default().fg(color)));
		}
		splash_lines.push(Line::from(spans));
	}

	splash_lines.push(Line::from(""));
	splash_lines.push(Line::from(Span::styled(
		"Enhanced Development Experience",
		Style::default().fg(theme.border),
	)));

	// Calculate vertical centering
	let content_height = splash_lines.len() as u16;
	let available_height = area.height;

	// If content is too large, shift it up to keep description visible
	let vertical_offset = if content_height >= available_height {
		0 // Start from top if content is too large
	} else {
		(available_height.saturating_sub(content_height)) / 2
	};

	let centered_area = Rect {
		x: area.x,
		y: area.y + vertical_offset,
		width: area.width,
		height: content_height.min(available_height),
	};

	Paragraph::new(splash_lines)
		.alignment(ratatui::layout::Alignment::Center)
		.block(Block::default())
		.render(centered_area, buf);
}

fn get_valid_fonts() -> Vec<&'static str> {
	vec![
		// Fonts verified to work with figlet-rs (203 total tested)
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
