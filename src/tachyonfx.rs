use std::time::Instant;

use crate::theme::ChatTheme;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Margin},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Clear, Widget},
};
use tachyonfx::{
    CellFilter, CenteredShrink, Duration, Effect, EffectRenderer,
    Interpolation::*,
    IntoEffect, Motion, SimpleRng, color_from_hsl,
    fx::{self, ExpandDirection, Glitch, never_complete, parallel, sequence},
};

pub struct TachyonDemo {
    pub active_effect: (&'static str, Effect),
    pub active_effect_idx: usize,
    pub last_tick: Duration,
    effects: EffectsRepository,
    theme: ChatTheme,
}

impl TachyonDemo {
    pub fn new(theme: ChatTheme) -> Self {
        let effects = EffectsRepository::new(theme.clone());
        let active_effect = effects.get_effect(0);

        Self {
            active_effect,
            active_effect_idx: 0,
            last_tick: Duration::ZERO,
            effects,
            theme,
        }
    }

    pub fn update(&mut self, elapsed: std::time::Duration) {
        self.last_tick = Duration::from_millis(elapsed.as_millis() as u32);
    }

    pub fn next_effect(&mut self) {
        let fx_idx = (self.active_effect_idx + 1) % self.effects.len();
        self.active_effect = self.effects.get_effect(fx_idx);
        self.active_effect_idx = fx_idx;
    }

    pub fn prev_effect(&mut self) {
        let fx_idx = if self.active_effect_idx == 0 {
            self.effects.len() - 1
        } else {
            self.active_effect_idx - 1
        };
        self.active_effect = self.effects.get_effect(fx_idx);
        self.active_effect_idx = fx_idx;
    }

    pub fn restart_effect(&mut self) {
        self.active_effect = self.effects.get_effect(self.active_effect_idx);
    }

    pub fn random_effect(&mut self, rng: &mut SimpleRng) {
        let fx_idx = (rng.r#gen() % self.effects.len() as u32) as usize;
        self.active_effect = self.effects.get_effect(fx_idx);
        self.active_effect_idx = fx_idx;
    }

    pub fn scramble_effect(&mut self) {
        let duration = Duration::from_secs(7);
        self.active_effect = (
            "scramble",
            fx::with_duration(
                duration,
                Glitch::builder()
                    .cell_glitch_ratio(1f32)
                    .action_start_delay_ms(0..3000)
                    .action_ms(8000..10_000)
                    .build()
                    .into_effect(),
            ),
        );
    }

    pub fn render(&mut self, f: &mut Frame) {
        // Use theme colors for consistency
        let screen_bg = self.theme.bg; // Main background
        let content_bg = self.theme.card; // Use card background for modal

        Clear.render(f.area(), f.buffer_mut());
        Block::default()
            .style(Style::default().bg(screen_bg))
            .render(f.area(), f.buffer_mut());

        // Create content area without border
        let content_area = f.area().inner_centered(82, 19);

        // Render block with card background, no border
        Block::default()
            .style(Style::default().bg(content_bg))
            .render(content_area, f.buffer_mut());

        // Add padding only for text layout, not for animations
        let padded_area = content_area.inner(Margin::new(2, 1));

        // Use theme colors throughout
        let text_fg = self.theme.fg;
        let text_muted = self.theme.muted_fg;
        let accent_primary = self.theme.accent; // Green
        let accent_secondary = self.theme.mode_colors.plan; // Yellow
        let accent_tertiary = self.theme.mode_colors.ask; // Blue

        let anim_style = [
            Style::default()
                .fg(accent_primary)
                .add_modifier(Modifier::BOLD),
            Style::default()
                .fg(accent_secondary)
                .add_modifier(Modifier::BOLD),
        ];
        let text_style = Style::default().fg(text_fg);
        let shortcut_style = [
            Style::default()
                .fg(accent_tertiary)
                .add_modifier(Modifier::BOLD),
            Style::default().fg(text_muted),
        ];

        let layout = Layout::vertical([
            Constraint::Length(2),
            Constraint::Length(7),
            Constraint::Length(6),
        ])
        .split(padded_area);

        let active_animation: Line = Line::from(vec![
            Span::from("Active animation: ").style(anim_style[0]),
            Span::from(self.active_effect.0).style(anim_style[1]),
        ]);

        let main_text = Text::from(vec![
            Line::from(vec![
                Span::styled("Many effects are ", text_style),
                Span::styled(
                    "composable",
                    Style::default()
                        .fg(accent_primary)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(", e.g. ", text_style),
                Span::styled("`parallel`", Style::default().fg(accent_tertiary)),
                Span::styled(", ", text_style),
                Span::styled("`sequence`", Style::default().fg(accent_secondary)),
                Span::styled(", ", text_style),
                Span::styled("`repeating`", Style::default().fg(accent_primary)),
                Span::styled(".", text_style),
            ]),
            Line::from(vec![
                Span::styled("Most effects have a ", text_style),
                Span::styled(
                    "lifetime",
                    Style::default()
                        .fg(accent_secondary)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(", after which they report ", text_style),
                Span::styled("done()", Style::default().fg(accent_primary)),
                Span::styled(".", text_style),
            ]),
            Line::from(vec![
                Span::styled("Effects such as ", text_style),
                Span::styled(
                    "`never_complete`",
                    Style::default()
                        .fg(self.theme.destructive)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(", ", text_style),
                Span::styled(
                    "`temporary`",
                    Style::default()
                        .fg(accent_tertiary)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" influence or override this.", text_style),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("The ", text_style),
                Span::styled(
                    "text in this window",
                    Style::default()
                        .fg(accent_primary)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" will undergo a ", text_style),
                Span::styled(
                    "random transition",
                    Style::default()
                        .fg(accent_secondary)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(vec![
                Span::styled("when any of the following ", text_style),
                Span::styled(
                    "keys",
                    Style::default()
                        .fg(accent_tertiary)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(" are pressed:", text_style),
            ]),
        ]);

        let shortcut = |key: &'static str, desc: &'static str| {
            Line::from(vec![
                Span::from(key).style(shortcut_style[0]),
                Span::from(desc).style(shortcut_style[1]),
            ])
        };

        let shortcuts = Text::from(vec![
            shortcut("↵   ", "next transition"),
            shortcut("⌫   ", "previous transition"),
            shortcut("␣   ", "restart transition"),
            shortcut("r   ", "random transition"),
            shortcut("s   ", "scramble text toggle"),
            shortcut("ESC ", "quit"),
        ]);

        f.render_widget(active_animation, layout[0]);
        f.render_widget(main_text, layout[1]);
        f.render_widget(shortcuts, layout[2]);

        // Apply effect to the entire content area (animations cover full area)
        let duration = self.last_tick;
        if self.active_effect.1.running() {
            f.render_effect(&mut self.active_effect.1, content_area, duration);
        }
    }
}

struct EffectsRepository {
    effects: Vec<(&'static str, Effect)>,
}

impl EffectsRepository {
    fn new(theme: ChatTheme) -> Self {
        // Use theme colors for effects
        let screen_bg = theme.bg;

        // Theme accent colors
        let primary = theme.accent; // Green
        let secondary = theme.mode_colors.plan; // Yellow
        let tertiary = theme.mode_colors.ask; // Blue
        let destructive = theme.destructive; // Red
        let text_fg = theme.fg;

        let slow = Duration::from_millis(1250);
        let medium = Duration::from_millis(750);

        // Custom color cycle using theme colors
        let custom_color_cycle =
            fx::effect_fn(Instant::now(), slow, move |state, _ctx, cell_iter| {
                let cycle: f32 = (state.elapsed().as_millis() % 3600) as f32;

                cell_iter
                    .filter(|(_, cell)| cell.symbol() != " ")
                    .enumerate()
                    .for_each(|(i, (_pos, cell))| {
                        let hue = (2.0 * i as f32 + cycle * 0.2) % 360.0;
                        let color = color_from_hsl(hue, 80.0, 65.0);
                        cell.set_fg(color);
                    });
            })
            .with_filter(CellFilter::FgColor(text_fg));

        let effects = vec![
            (
                "sweep in",
                fx::sweep_in(Motion::LeftToRight, 30, 0, screen_bg, (slow, QuadOut)),
            ),
            (
                "smooth expand and reversed",
                sequence(&[
                    fx::expand(
                        ExpandDirection::Vertical,
                        Style::new().fg(destructive).bg(screen_bg),
                        1200,
                    ),
                    fx::sleep(slow),
                    fx::expand(
                        ExpandDirection::Horizontal,
                        Style::new().fg(primary).bg(screen_bg),
                        1200,
                    )
                    .reversed(),
                ]),
            ),
            (
                "irregular sweep out/sweep in",
                sequence(&[
                    fx::sweep_out(Motion::DownToUp, 5, 20, secondary, (2000, QuadOut)),
                    fx::sweep_in(Motion::UpToDown, 5, 20, tertiary, (2000, QuadOut)),
                    fx::sweep_out(Motion::UpToDown, 5, 20, primary, (2000, QuadOut)),
                    fx::sweep_in(Motion::DownToUp, 5, 20, destructive, (2000, QuadOut)),
                ]),
            ),
            (
                "coalesce",
                fx::sequence(&[
                    fx::coalesce((medium, CubicOut)),
                    fx::sleep(medium),
                    fx::prolong_end(
                        medium,
                        fx::dissolve_to(Style::default().bg(screen_bg).fg(tertiary), medium),
                    ),
                ]),
            ),
            (
                "slide in/out",
                fx::repeating(sequence(&[
                    parallel(&[
                        fx::fade_from_fg(secondary, (2000, ExpoInOut)),
                        fx::slide_in(Motion::UpToDown, 20, 0, screen_bg, medium),
                    ]),
                    fx::sleep(medium),
                    fx::prolong_end(
                        medium,
                        fx::slide_out(Motion::LeftToRight, 80, 0, screen_bg, medium),
                    ),
                ])),
            ),
            (
                "change hue, saturation and lightness",
                sequence(&[
                    fx::hsl_shift_fg([360.0, 0.0, 0.0], medium),
                    fx::hsl_shift_fg([0.0, -100.0, 0.0], medium),
                    fx::hsl_shift_fg([0.0, -100.0, 0.0], medium).reversed(),
                    fx::hsl_shift_fg([0.0, 100.0, 0.0], medium),
                    fx::hsl_shift_fg([0.0, 100.0, 0.0], medium).reversed(),
                    fx::hsl_shift_fg([0.0, 0.0, -100.0], medium),
                    fx::hsl_shift_fg([0.0, 0.0, -100.0], medium).reversed(),
                    fx::hsl_shift_fg([0.0, 0.0, 100.0], medium),
                    fx::hsl_shift_fg([0.0, 0.0, 100.0], medium).reversed(),
                ]),
            ),
            ("custom color cycle", never_complete(custom_color_cycle)),
        ];

        Self { effects }
    }

    fn get_effect(&self, idx: usize) -> (&'static str, Effect) {
        self.effects[idx].clone()
    }

    fn len(&self) -> usize {
        self.effects.len()
    }
}
