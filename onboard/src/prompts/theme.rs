//! Theme system for all prompts
//! 
//! This module provides a unified theming system that all prompts use for consistent
//! visual appearance across the onboarding experience.

use console::Style;
use once_cell::sync::Lazy;
use std::sync::RwLock;
use owo_colors::OwoColorize;
use crate::effects::RainbowEffect;

// ─────────────────────────────────────────────────────────────────────────────
// Theme Configuration
// ─────────────────────────────────────────────────────────────────────────────

/// Core theme colors and styles used across all prompts
pub struct DxTheme {
    pub primary: Style,
    pub success: Style,
    pub warning: Style,
    pub error: Style,
    pub dim: Style,
}

impl Default for DxTheme {
    fn default() -> Self {
        Self {
            primary: Style::new().cyan(),
            success: Style::new().green(),
            warning: Style::new().yellow(),
            error: Style::new().red(),
            dim: Style::new().dim(),
        }
    }
}

/// Global theme instance
pub static THEME: Lazy<RwLock<DxTheme>> = Lazy::new(|| RwLock::new(DxTheme::default()));

// ─────────────────────────────────────────────────────────────────────────────
// Rainbow Animation
// ─────────────────────────────────────────────────────────────────────────────

/// Global rainbow effect for animated symbols
pub static RAINBOW: Lazy<RwLock<RainbowEffect>> = Lazy::new(|| RwLock::new(RainbowEffect::new()));

/// Get a rainbow-colored symbol at a specific index
pub fn rainbow_symbol(symbol: &str, index: usize) -> String {
    if let Ok(rainbow) = RAINBOW.read() {
        let color = rainbow.color_at(index);
        symbol.truecolor(color.r, color.g, color.b).to_string()
    } else {
        symbol.to_string()
    }
}

/// Get a rainbow-colored step_submit symbol (♦)
pub fn rainbow_step_submit() -> String {
    let symbols = &*SYMBOLS;
    rainbow_symbol(symbols.step_submit, 0)
}

/// Get a rainbow-colored step_active symbol  
pub fn rainbow_step_active() -> String {
    let symbols = &*SYMBOLS;
    rainbow_symbol(symbols.step_active, 1)
}

// ─────────────────────────────────────────────────────────────────────────────
// Symbols
// ─────────────────────────────────────────────────────────────────────────────

/// Unicode symbols used across all prompts for consistent visual appearance
pub struct Symbols {
    pub step_active: &'static str,
    pub step_cancel: &'static str,
    pub step_error: &'static str,
    pub step_submit: &'static str,
    pub bar_start: &'static str,
    pub bar: &'static str,
    pub bar_end: &'static str,
    pub radio_active: &'static str,
    pub radio_inactive: &'static str,
    pub checkbox_active: &'static str,
    pub checkbox_selected: &'static str,
    pub checkbox_inactive: &'static str,
    pub password_mask: char,
    pub bar_h: &'static str,
    pub corner_top_right: &'static str,
    pub connect_left: &'static str,
    pub corner_bottom_right: &'static str,
    pub box_top_left: &'static str,
    pub box_top_right: &'static str,
    pub box_bottom_left: &'static str,
    pub box_bottom_right: &'static str,
    pub box_horizontal: &'static str,
    pub box_vertical: &'static str,
    pub box_left_t: &'static str,
    pub box_right_t: &'static str,
}

impl Symbols {
    const fn unicode() -> Self {
        Self {
            step_active: "♦",
            step_cancel: "■",
            step_error: "▲",
            step_submit: "♦",
            bar_start: "╭",
            bar: "│",
            bar_end: "╰",
            radio_active: "●",
            radio_inactive: "○",
            checkbox_active: "◻",
            checkbox_selected: "◼",
            checkbox_inactive: "◻",
            password_mask: '•',
            bar_h: "─",
            corner_top_right: "╮",
            connect_left: "├",
            corner_bottom_right: "╯",
            box_top_left: "╭",
            box_top_right: "╮",
            box_bottom_left: "╰",
            box_bottom_right: "╯",
            box_horizontal: "─",
            box_vertical: "│",
            box_left_t: "├",
            box_right_t: "╯",
        }
    }
}

/// Global symbols instance
pub static SYMBOLS: Lazy<Symbols> = Lazy::new(Symbols::unicode);
