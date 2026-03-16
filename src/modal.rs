use std::time::Instant;

use crate::theme::ChatTheme;
use ratatui::{
    Frame,
    layout::{Margin, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Clear, List, ListItem, ListState, Widget},
};
use tachyonfx::{
    CellFilter, Duration, Effect, EffectRenderer, Interpolation::*,
    Motion,
    fx::{self, ExpandDirection},
};

/// Modal animation types
#[derive(Debug, Clone, Copy)]
pub enum ModalAnimation {
    FadeIn,
    SlideDown,
    Expand,
    None,
}

/// Reusable modal component with tachyonfx effects
pub struct Modal {
    theme: ChatTheme,
    animation: Option<Effect>,
    animation_start: Instant,
    show_animation: bool,
}

impl Modal {
    pub fn new(theme: ChatTheme) -> Self {
        Self {
            theme,
            animation: None,
            animation_start: Instant::now(),
            show_animation: false,
        }
    }

    /// Show modal with animation
    pub fn show(&mut self, animation_type: ModalAnimation) {
        self.show_animation = true;
        self.animation_start = Instant::now();
        
        let duration = Duration::from_millis(300);
        
        self.animation = Some(match animation_type {
            ModalAnimation::FadeIn => {
                fx::fade_from_fg(self.theme.bg, (duration.as_millis() as u32, QuadOut))
            }
            ModalAnimation::SlideDown => {
                fx::slide_in(Motion::UpToDown, 20, 0, self.theme.bg, (duration.as_millis() as u32, QuadOut))
            }
            ModalAnimation::Expand => {
                fx::expand(
                    ExpandDirection::Vertical,
                    Style::new().fg(self.theme.accent).bg(self.theme.bg),
                    duration.as_millis() as u32,
                )
            }
            ModalAnimation::None => return,
        });
    }

    /// Hide modal
    pub fn hide(&mut self) {
        self.show_animation = false;
        self.animation = None;
    }

    /// Check if modal is visible
    pub fn is_visible(&self) -> bool {
        self.show_animation
    }

    /// Render modal with content
    pub fn render<F>(&mut self, f: &mut Frame, area: Rect, render_content: F)
    where
        F: FnOnce(&mut Frame, Rect, &ChatTheme),
    {
        if !self.show_animation {
            return;
        }

        // Clear background
        Clear.render(area, f.buffer_mut());
        
        // Render modal background
        Block::default()
            .style(Style::default().bg(self.theme.card))
            .render(area, f.buffer_mut());

        // Render content with padding
        let content_area = area.inner(Margin::new(2, 1));
        render_content(f, content_area, &self.theme);

        // Apply animation effect
        if let Some(ref mut effect) = self.animation {
            let elapsed = self.animation_start.elapsed().into();
            if effect.running() {
                f.render_effect(effect, area, elapsed);
            }
        }
    }
}

/// Animated suggestion list component
pub struct AnimatedSuggestionList {
    modal: Modal,
    list_state: ListState,
    items: Vec<String>,
    descriptions: Vec<String>,
    selected_index: usize,
    shimmer_effect: Option<Effect>,
    shimmer_start: Instant,
}

impl AnimatedSuggestionList {
    pub fn new(theme: ChatTheme) -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            modal: Modal::new(theme),
            list_state,
            items: Vec::new(),
            descriptions: Vec::new(),
            selected_index: 0,
            shimmer_effect: None,
            shimmer_start: Instant::now(),
        }
    }

    /// Update suggestions and show modal
    pub fn update_suggestions(&mut self, items: Vec<String>, descriptions: Vec<String>) {
        self.items = items;
        self.descriptions = descriptions;
        self.selected_index = 0;
        self.list_state.select(Some(0));
        
        if !self.items.is_empty() {
            self.modal.show(ModalAnimation::SlideDown);
            self.start_shimmer_effect();
        } else {
            self.modal.hide();
        }
    }

    /// Hide suggestions
    pub fn hide(&mut self) {
        self.modal.hide();
        self.items.clear();
        self.descriptions.clear();
    }

    /// Check if suggestions are visible
    pub fn is_visible(&self) -> bool {
        self.modal.is_visible() && !self.items.is_empty()
    }

    /// Move selection up
    pub fn select_previous(&mut self) {
        if !self.items.is_empty() {
            self.selected_index = if self.selected_index == 0 {
                self.items.len() - 1
            } else {
                self.selected_index - 1
            };
            self.list_state.select(Some(self.selected_index));
        }
    }

    /// Move selection down
    pub fn select_next(&mut self) {
        if !self.items.is_empty() {
            self.selected_index = (self.selected_index + 1) % self.items.len();
            self.list_state.select(Some(self.selected_index));
        }
    }

    /// Get selected item
    pub fn get_selected(&self) -> Option<&String> {
        self.items.get(self.selected_index)
    }

    /// Start shimmer effect for list items
    fn start_shimmer_effect(&mut self) {
        self.shimmer_start = Instant::now();
        let duration = Duration::from_millis(2000);
        
        // Create a subtle shimmer effect using theme colors
        let theme_fg = self.modal.theme.fg;
        self.shimmer_effect = Some(
            fx::effect_fn(Instant::now(), duration, move |state, _ctx, cell_iter| {
                let cycle: f32 = (state.elapsed().as_millis() % 2000) as f32;
                let wave = (cycle / 2000.0 * std::f32::consts::PI * 2.0).sin();
                let brightness = (wave * 0.1 + 0.9).max(0.8).min(1.0);
                
                cell_iter
                    .filter(|(_, cell)| cell.symbol() != " ")
                    .for_each(|(_pos, cell)| {
                        if let ratatui::style::Color::Rgb(r, g, b) = theme_fg {
                            let new_r = (r as f32 * brightness) as u8;
                            let new_g = (g as f32 * brightness) as u8;
                            let new_b = (b as f32 * brightness) as u8;
                            cell.set_fg(ratatui::style::Color::Rgb(new_r, new_g, new_b));
                        }
                    });
            })
            .with_filter(CellFilter::Text)
        );
    }

    /// Render suggestions list
    pub fn render(&mut self, f: &mut Frame, area: Rect) {
        if !self.is_visible() {
            return;
        }

        self.modal.render(f, area, |f, content_area, theme| {
            // Create list items with descriptions
            let list_items: Vec<ListItem> = self.items
                .iter()
                .zip(self.descriptions.iter())
                .enumerate()
                .map(|(i, (item, desc))| {
                    let is_selected = i == self.selected_index;
                    
                    let style = if is_selected {
                        Style::default()
                            .fg(theme.accent)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default().fg(theme.fg)
                    };

                    let desc_style = if is_selected {
                        Style::default().fg(theme.mode_colors.plan)
                    } else {
                        Style::default().fg(theme.muted_fg)
                    };

                    // Create spans for item and description
                    let line = Line::from(vec![
                        Span::styled(format!("{:<30}", item), style),
                        Span::styled(desc, desc_style),
                    ]);

                    ListItem::new(line)
                })
                .collect();

            // Render list
            let list = List::new(list_items)
                .style(Style::default().fg(theme.fg))
                .highlight_style(
                    Style::default()
                        .bg(theme.accent)
                        .fg(theme.primary_fg)
                        .add_modifier(Modifier::BOLD)
                );

            f.render_stateful_widget(list, content_area, &mut self.list_state);

            // Apply shimmer effect if active
            if let Some(ref mut effect) = self.shimmer_effect {
                let elapsed = self.shimmer_start.elapsed().into();
                if effect.running() {
                    f.render_effect(effect, content_area, elapsed);
                }
            }
        });
    }
}