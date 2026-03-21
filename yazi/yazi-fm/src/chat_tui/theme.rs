use ratatui::style::{Color, Modifier, Style};
// Temporarily disabled - MachineFormat issue
// use serializer::{DxLlmValue, MachineFormat, machine_to_document};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeVariant {
    Dark,
    #[allow(dead_code)]
    Light,
}

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
        // Dark mode from your CSS theme - oklch values converted to RGB
        // Using --primary (green) as the main accent throughout the UI
        Self {
            variant: ThemeVariant::Dark,
            bg: Color::Rgb(0, 0, 0),                 // --background
            fg: Color::Rgb(255, 255, 255),           // --foreground
            card: Color::Rgb(9, 9, 9),               // --card
            card_fg: Color::Rgb(255, 255, 255),      // --card-foreground
            popover: Color::Rgb(18, 18, 18),         // --popover
            popover_fg: Color::Rgb(255, 255, 255),   // --popover-foreground
            primary: Color::Rgb(0, 201, 80),         // --primary (green)
            primary_fg: Color::Rgb(255, 255, 255),   // --primary-foreground
            secondary: Color::Rgb(34, 34, 34),       // --secondary
            secondary_fg: Color::Rgb(255, 255, 255), // --secondary-foreground
            muted: Color::Rgb(29, 29, 29),           // --muted
            muted_fg: Color::Rgb(164, 164, 164),     // --muted-foreground
            accent: Color::Rgb(0, 201, 80),          // Use primary green as accent
            accent_fg: Color::Rgb(255, 255, 255),    // --accent-foreground
            destructive: Color::Rgb(255, 91, 91),    // --destructive
            destructive_fg: Color::Rgb(0, 0, 0),     // --destructive-foreground
            border: Color::Rgb(36, 36, 36),          // --border
            border_focused: Color::Rgb(0, 201, 80),  // Use primary green for focus
            input: Color::Rgb(51, 51, 51),           // --input
            ring: Color::Rgb(164, 164, 164),         // --ring
            // Legacy compatibility
            user_msg_bg: Color::Rgb(9, 9, 9),         // card
            ai_msg_bg: Color::Rgb(18, 18, 18),        // popover
            accent_secondary: Color::Rgb(0, 201, 80), // primary green
            shimmer_colors: vec![
                Color::Rgb(36, 36, 36),    // border
                Color::Rgb(0, 201, 80),    // primary green
                Color::Rgb(164, 164, 164), // muted_fg
                Color::Rgb(0, 201, 80),    // primary green
                Color::Rgb(36, 36, 36),    // border
            ],
            mode_colors: ModeColors {
                agent: Color::Rgb(0, 201, 80), // primary green
                plan: Color::Rgb(255, 174, 4), // chart-1 yellow
                ask: Color::Rgb(38, 113, 244), // chart-2 blue
            },
        }
    }

    #[allow(dead_code)]
    fn light_fallback() -> Self {
        // Light mode from theme.css - shadcn-ui/Vercel design system
        Self {
            variant: ThemeVariant::Light,
            bg: Color::Rgb(252, 252, 252),             // --background
            fg: Color::Rgb(0, 0, 0),                   // --foreground
            card: Color::Rgb(255, 255, 255),           // --card
            card_fg: Color::Rgb(0, 0, 0),              // --card-foreground
            popover: Color::Rgb(252, 252, 252),        // --popover
            popover_fg: Color::Rgb(0, 0, 0),           // --popover-foreground
            primary: Color::Rgb(0, 0, 0),              // --primary
            primary_fg: Color::Rgb(255, 255, 255),     // --primary-foreground
            secondary: Color::Rgb(235, 235, 235),      // --secondary
            secondary_fg: Color::Rgb(0, 0, 0),         // --secondary-foreground
            muted: Color::Rgb(245, 245, 245),          // --muted
            muted_fg: Color::Rgb(82, 82, 82),          // --muted-foreground
            accent: Color::Rgb(235, 235, 235),         // --accent
            accent_fg: Color::Rgb(0, 0, 0),            // --accent-foreground
            destructive: Color::Rgb(229, 75, 79),      // --destructive
            destructive_fg: Color::Rgb(255, 255, 255), // --destructive-foreground
            border: Color::Rgb(228, 228, 228),         // --border
            border_focused: Color::Rgb(0, 0, 0),       // --ring
            input: Color::Rgb(235, 235, 235),          // --input
            ring: Color::Rgb(0, 0, 0),                 // --ring
            // Legacy compatibility
            user_msg_bg: Color::Rgb(255, 255, 255), // card
            ai_msg_bg: Color::Rgb(252, 252, 252),   // popover
            accent_secondary: Color::Rgb(235, 235, 235), // accent
            shimmer_colors: vec![
                Color::Rgb(228, 228, 228), // border
                Color::Rgb(235, 235, 235), // accent
                Color::Rgb(82, 82, 82),    // muted_fg
                Color::Rgb(235, 235, 235), // accent
                Color::Rgb(228, 228, 228), // border
            ],
            mode_colors: ModeColors {
                agent: Color::Rgb(0, 160, 60), // darker green for light mode
                plan: Color::Rgb(200, 130, 0), // darker yellow for light mode
                ask: Color::Rgb(30, 90, 200),  // darker blue for light mode
            },
        }
    }

    #[allow(dead_code)]
    pub fn title_style(&self) -> Style {
        Style::default().fg(self.fg).add_modifier(Modifier::BOLD)
    }

    #[allow(dead_code)]
    pub fn border_style(&self, focused: bool) -> Style {
        Style::default().fg(if focused {
            self.border_focused
        } else {
            self.border
        })
    }

    #[allow(dead_code)]
    pub fn accent_style(&self) -> Style {
        Style::default()
            .fg(self.primary)
            .add_modifier(Modifier::BOLD)
    }
}
