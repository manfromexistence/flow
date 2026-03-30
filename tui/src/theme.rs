use ratatui::style::{Color, Modifier, Style};
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
// Temporarily disabled - MachineFormat issue
// use serializer::{DxLlmValue, MachineFormat, machine_to_document};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeVariant {
	Dark,
	#[allow(dead_code)]
	Light,
}

// Structures for loading themes from JSON
#[derive(Debug, Deserialize, Serialize, Clone)]
struct RgbColor {
	r: u8,
	g: u8,
	b: u8,
}

impl From<RgbColor> for Color {
	fn from(rgb: RgbColor) -> Self {
		Color::Rgb(rgb.r, rgb.g, rgb.b)
	}
}

#[derive(Debug, Deserialize, Clone)]
struct ThemeColors {
	background: RgbColor,
	foreground: RgbColor,
	card: RgbColor,
	card_foreground: RgbColor,
	primary: RgbColor,
	primary_foreground: RgbColor,
	secondary: RgbColor,
	secondary_foreground: RgbColor,
	muted: RgbColor,
	muted_foreground: RgbColor,
	accent: RgbColor,
	accent_foreground: RgbColor,
	destructive: RgbColor,
	destructive_foreground: RgbColor,
	border: RgbColor,
	input: RgbColor,
	ring: RgbColor,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ThemeDefinition {
	pub name: String,
	pub title: String,
	#[allow(dead_code)]
	pub description: String,
	dark: ThemeColors,
	light: ThemeColors,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ThemeRegistry {
	pub(crate) themes: Vec<ThemeDefinition>,
}

static THEME_REGISTRY: OnceLock<ThemeRegistry> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct ChatTheme {
	#[allow(dead_code)]
	pub variant: ThemeVariant,
	// Core shadcn-ui/Vercel design system colors
	pub bg: Color,
	pub fg: Color,
	#[allow(dead_code)]
	pub card: Color,
	#[allow(dead_code)]
	pub card_fg: Color,
	#[allow(dead_code)]
	pub popover: Color,
	#[allow(dead_code)]
	pub popover_fg: Color,
	#[allow(dead_code)]
	pub primary: Color,
	#[allow(dead_code)]
	pub primary_fg: Color,
	#[allow(dead_code)]
	pub secondary: Color,
	#[allow(dead_code)]
	pub secondary_fg: Color,
	#[allow(dead_code)]
	pub muted: Color,
	#[allow(dead_code)]
	pub muted_fg: Color,
	pub accent: Color,
	#[allow(dead_code)]
	pub accent_fg: Color,
	#[allow(dead_code)]
	pub destructive: Color,
	#[allow(dead_code)]
	pub destructive_fg: Color,
	pub border: Color,
	#[allow(dead_code)]
	pub border_focused: Color,
	#[allow(dead_code)]
	pub input: Color,
	#[allow(dead_code)]
	pub ring: Color,
	// Legacy compatibility
	#[allow(dead_code)]
	pub user_msg_bg: Color,
	#[allow(dead_code)]
	pub ai_msg_bg: Color,
	#[allow(dead_code)]
	pub accent_secondary: Color,
	#[allow(dead_code)]
	pub shimmer_colors: Vec<Color>,
	#[allow(dead_code)]
	pub mode_colors: ModeColors,
}

#[derive(Debug, Clone)]
pub struct ModeColors {
	#[allow(dead_code)]
	pub agent: Color,
	#[allow(dead_code)]
	pub plan: Color,
	#[allow(dead_code)]
	pub ask: Color,
}

impl ChatTheme {
	/// Load all available themes from themes.json
	#[allow(private_interfaces)]
	pub fn load_themes() -> &'static ThemeRegistry {
		THEME_REGISTRY.get_or_init(|| {
			let themes_json = include_str!("../themes.json");
			serde_json::from_str(themes_json).expect("Failed to parse themes.json")
		})
	}

	/// Get a list of all available theme names and titles
	pub fn available_themes() -> Vec<(String, String)> {
		Self::load_themes().themes.iter().map(|t| (t.name.clone(), t.title.clone())).collect()
	}

	/// Create a theme from a theme definition
	pub fn from_definition(def: &ThemeDefinition, variant: ThemeVariant) -> Self {
		let colors = match variant {
			ThemeVariant::Dark => &def.dark,
			ThemeVariant::Light => &def.light,
		};

		let primary: Color = colors.primary.clone().into();
		let accent: Color = colors.accent.clone().into();

		Self {
			variant,
			bg: colors.background.clone().into(),
			fg: colors.foreground.clone().into(),
			card: colors.card.clone().into(),
			card_fg: colors.card_foreground.clone().into(),
			popover: colors.card.clone().into(),
			popover_fg: colors.card_foreground.clone().into(),
			primary,
			primary_fg: colors.primary_foreground.clone().into(),
			secondary: colors.secondary.clone().into(),
			secondary_fg: colors.secondary_foreground.clone().into(),
			muted: colors.muted.clone().into(),
			muted_fg: colors.muted_foreground.clone().into(),
			accent,
			accent_fg: colors.accent_foreground.clone().into(),
			destructive: colors.destructive.clone().into(),
			destructive_fg: colors.destructive_foreground.clone().into(),
			border: colors.border.clone().into(),
			border_focused: primary,
			input: colors.input.clone().into(),
			ring: colors.ring.clone().into(),
			// Legacy compatibility
			user_msg_bg: colors.card.clone().into(),
			ai_msg_bg: colors.muted.clone().into(),
			accent_secondary: accent,
			shimmer_colors: vec![
				colors.border.clone().into(),
				primary,
				colors.muted_foreground.clone().into(),
				primary,
				colors.border.clone().into(),
			],
			mode_colors: ModeColors {
				agent: primary,
				plan: colors.ring.clone().into(),
				ask: colors.accent.clone().into(),
			},
		}
	}

	/// Create a theme by name
	pub fn by_name(name: &str, variant: ThemeVariant) -> Option<Self> {
		Self::load_themes()
			.themes
			.iter()
			.find(|t| t.name == name)
			.map(|def| Self::from_definition(def, variant))
	}

	#[allow(dead_code)]
	pub fn new(variant: ThemeVariant) -> Self {
		// Try to load from theme.sr, fallback to hardcoded if it fails
		Self::from_theme_sr(variant).unwrap_or_else(|_| match variant {
			ThemeVariant::Dark => Self::dark_fallback(),
			ThemeVariant::Light => Self::light_fallback(),
		})
	}

	/// Load theme from embedded theme.machine file
	#[allow(dead_code)]
	fn from_theme_sr(_variant: ThemeVariant) -> Result<Self, Box<dyn std::error::Error>> {
		// Temporarily disabled - MachineFormat constructor issue
		// Return dark fallback for now
		Ok(Self::dark_fallback())
	}

	pub fn dark_fallback() -> Self {
		// Dark mode DX theme - matches themes.json exactly
		Self {
			variant: ThemeVariant::Dark,
			bg: Color::Rgb(0, 0, 0),                 // background
			fg: Color::Rgb(255, 255, 255),           // foreground
			card: Color::Rgb(14, 14, 14),            // card
			card_fg: Color::Rgb(255, 255, 255),      // card-foreground
			popover: Color::Rgb(14, 14, 14),         // popover (same as card)
			popover_fg: Color::Rgb(255, 255, 255),   // popover-foreground
			primary: Color::Rgb(0, 255, 42),         // primary (bright green)
			primary_fg: Color::Rgb(0, 0, 0),         // primary-foreground (black for contrast)
			secondary: Color::Rgb(34, 34, 34),       // secondary
			secondary_fg: Color::Rgb(255, 255, 255), // secondary-foreground
			muted: Color::Rgb(29, 29, 29),           // muted
			muted_fg: Color::Rgb(155, 155, 155),     // muted-foreground
			accent: Color::Rgb(0, 255, 42),          // accent (bright green)
			accent_fg: Color::Rgb(0, 0, 0),          // accent-foreground (black for contrast)
			destructive: Color::Rgb(203, 154, 151),  // destructive
			destructive_fg: Color::Rgb(0, 0, 0),     // destructive-foreground
			border: Color::Rgb(36, 36, 36),          // border
			border_focused: Color::Rgb(0, 255, 42),  // bright green for focus
			input: Color::Rgb(48, 48, 48),           // input
			ring: Color::Rgb(155, 155, 155),         // ring
			// Legacy compatibility
			user_msg_bg: Color::Rgb(14, 14, 14),      // card
			ai_msg_bg: Color::Rgb(29, 29, 29),        // muted
			accent_secondary: Color::Rgb(0, 255, 42), // bright green
			shimmer_colors: vec![
				Color::Rgb(36, 36, 36),    // border
				Color::Rgb(0, 255, 42),    // bright green
				Color::Rgb(155, 155, 155), // muted_fg
				Color::Rgb(0, 255, 42),    // bright green
				Color::Rgb(36, 36, 36),    // border
			],
			mode_colors: ModeColors {
				agent: Color::Rgb(0, 255, 42),   // bright green
				plan: Color::Rgb(155, 155, 155), // ring
				ask: Color::Rgb(0, 255, 42),     // accent
			},
		}
	}

	#[allow(dead_code)]
	fn light_fallback() -> Self {
		// Light mode DX theme - matches themes.json exactly
		Self {
			variant: ThemeVariant::Light,
			bg: Color::Rgb(252, 252, 252),             // background
			fg: Color::Rgb(0, 0, 0),                   // foreground
			card: Color::Rgb(255, 255, 255),           // card
			card_fg: Color::Rgb(0, 0, 0),              // card-foreground
			popover: Color::Rgb(255, 255, 255),        // popover (same as card)
			popover_fg: Color::Rgb(0, 0, 0),           // popover-foreground
			primary: Color::Rgb(0, 255, 42),           // primary (bright green)
			primary_fg: Color::Rgb(0, 0, 0),           // primary-foreground (black for contrast)
			secondary: Color::Rgb(232, 232, 232),      // secondary
			secondary_fg: Color::Rgb(0, 0, 0),         // secondary-foreground
			muted: Color::Rgb(243, 243, 243),          // muted
			muted_fg: Color::Rgb(75, 75, 75),          // muted-foreground
			accent: Color::Rgb(0, 255, 42),            // accent (bright green)
			accent_fg: Color::Rgb(0, 0, 0),            // accent-foreground (black for contrast)
			destructive: Color::Rgb(184, 131, 129),    // destructive
			destructive_fg: Color::Rgb(255, 255, 255), // destructive-foreground
			border: Color::Rgb(223, 224, 223),         // border
			border_focused: Color::Rgb(0, 255, 42),    // bright green for focus
			input: Color::Rgb(232, 232, 232),          // input
			ring: Color::Rgb(0, 255, 42),              // ring (bright green)
			// Legacy compatibility
			user_msg_bg: Color::Rgb(255, 255, 255),   // card
			ai_msg_bg: Color::Rgb(243, 243, 243),     // muted
			accent_secondary: Color::Rgb(0, 255, 42), // bright green
			shimmer_colors: vec![
				Color::Rgb(223, 224, 223), // border
				Color::Rgb(0, 255, 42),    // bright green
				Color::Rgb(75, 75, 75),    // muted_fg
				Color::Rgb(0, 255, 42),    // bright green
				Color::Rgb(223, 224, 223), // border
			],
			mode_colors: ModeColors {
				agent: Color::Rgb(0, 255, 42), // bright green
				plan: Color::Rgb(0, 255, 42),  // ring
				ask: Color::Rgb(0, 255, 42),   // accent
			},
		}
	}

	#[allow(dead_code)]
	pub fn title_style(&self) -> Style {
		Style::default().fg(self.fg).add_modifier(Modifier::BOLD)
	}

	#[allow(dead_code)]
	pub fn border_style(&self, focused: bool) -> Style {
		Style::default().fg(if focused { self.border_focused } else { self.border })
	}

	#[allow(dead_code)]
	pub fn accent_style(&self) -> Style {
		Style::default().fg(self.primary).add_modifier(Modifier::BOLD)
	}
}
