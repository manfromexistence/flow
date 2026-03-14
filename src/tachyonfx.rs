use std::time::Instant;

use crate::theme::ChatTheme;
use ratatui::{
    layout::{Constraint, Layout, Margin},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Clear, Widget},
    Frame,
};
use tachyonfx::{
    fx::{self, never_complete, parallel, sequence, ExpandDirection, Glitch},
    CellFilter, CenteredShrink, Duration, Effect, EffectRenderer, IntoEffect, Interpolation::*,
    Motion, SimpleRng, color_from_hsl,
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
        // Vercel Geist + ShadCN UI inspired modern palette
        let screen_bg: Color = Color::Rgb(0, 0, 0); // Pure black - Vercel style
        let content_bg: Color = Color::Rgb(10, 10, 10); // Near black - subtle depth
        let border_color: Color = Color::Rgb(38, 38, 38); // Geist border

        Clear.render(f.area(), f.buffer_mut());
        Block::default()
            .style(Style::default().bg(screen_bg))
            .render(f.area(), f.buffer_mut());

        let content_area = f.area().inner_centered(80, 17);
        Block::default()
            .style(Style::default().bg(content_bg))
            .border_style(Style::default().fg(border_color))
            .render(content_area, f.buffer_mut());

        // Modern vibrant palette - ShadCN chart colors + favorites
        let accent_blue = Color::Rgb(59, 130, 246); // #3B82F6 - ShadCN blue (chart-2)
        let accent_cyan = Color::Rgb(6, 182, 212); // #06B6D4 - Tailwind cyan
        let text_fg = Color::Rgb(250, 250, 250); // #FAFAFA - High contrast white
        let text_muted = Color::Rgb(115, 115, 115); // #737373 - Geist gray
        let success_green = Color::Rgb(34, 197, 94); // #22C55E - Vibrant green (chart-1)
        let warning_yellow = Color::Rgb(234, 179, 8); // #EAB308 - Bold yellow
        let error_red = Color::Rgb(239, 68, 68); // #EF4444 - Bright red
        let info_purple = Color::Rgb(168, 85, 247); // #A855F7 - Rich purple (chart-5)

        let anim_style = [
            Style::default().fg(accent_cyan).add_modifier(Modifier::BOLD),
            Style::default().fg(warning_yellow).add_modifier(Modifier::BOLD),
        ];
        let text_style = Style::default().fg(text_fg);
        let shortcut_style = [
            Style::default()
                .fg(accent_blue)
                .add_modifier(Modifier::BOLD),
            Style::default().fg(text_muted),
        ];

        let layout = Layout::vertical([
            Constraint::Length(2),
            Constraint::Length(7),
            Constraint::Length(6),
        ])
        .split(content_area.inner(Margin::new(1, 1)));

        let active_animation: Line = Line::from(vec![
            Span::from("Active animation: ").style(anim_style[0]),
            Span::from(self.active_effect.0).style(anim_style[1]),
        ]);

        let main_text = Text::from(vec![
            Line::from(vec![
                Span::styled("Many effects are ", text_style),
                Span::styled("composable", Style::default().fg(success_green).add_modifier(Modifier::BOLD)),
                Span::styled(", e.g. ", text_style),
                Span::styled("`parallel`", Style::default().fg(accent_cyan)),
                Span::styled(", ", text_style),
                Span::styled("`sequence`", Style::default().fg(accent_blue)),
                Span::styled(", ", text_style),
                Span::styled("`repeating`", Style::default().fg(info_purple)),
                Span::styled(".", text_style),
            ]),
            Line::from(vec![
                Span::styled("Most effects have a ", text_style),
                Span::styled("lifetime", Style::default().fg(warning_yellow).add_modifier(Modifier::BOLD)),
                Span::styled(", after which they report ", text_style),
                Span::styled("done()", Style::default().fg(success_green)),
                Span::styled(".", text_style),
            ]),
            Line::from(vec![
                Span::styled("Effects such as ", text_style),
                Span::styled("`never_complete`", Style::default().fg(error_red).add_modifier(Modifier::BOLD)),
                Span::styled(", ", text_style),
                Span::styled("`temporary`", Style::default().fg(info_purple).add_modifier(Modifier::BOLD)),
                Span::styled(" influence or override this.", text_style),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::styled("The ", text_style),
                Span::styled("text in this window", Style::default().fg(accent_cyan).add_modifier(Modifier::BOLD)),
                Span::styled(" will undergo a ", text_style),
                Span::styled("random transition", Style::default().fg(warning_yellow).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(vec![
                Span::styled("when any of the following ", text_style),
                Span::styled("keys", Style::default().fg(success_green).add_modifier(Modifier::BOLD)),
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
    fn new(_theme: ChatTheme) -> Self {
        // Vercel Geist + ShadCN UI modern color palette
        let screen_bg = Color::Rgb(0, 0, 0); // Pure black - Vercel style
        let _content_bg = Color::Rgb(10, 10, 10); // Near black
        
        // Vibrant modern palette - ShadCN charts + favorites
        let blue = Color::Rgb(59, 130, 246); // #3B82F6 - ShadCN blue
        let cyan = Color::Rgb(6, 182, 212); // #06B6D4 - Tailwind cyan
        let green = Color::Rgb(34, 197, 94); // #22C55E - Vibrant green
        let yellow = Color::Rgb(234, 179, 8); // #EAB308 - Bold yellow
        let red = Color::Rgb(239, 68, 68); // #EF4444 - Bright red
        let purple = Color::Rgb(168, 85, 247); // #A855F7 - Rich purple
        let orange = Color::Rgb(249, 115, 22); // #F97316 - Vibrant orange
        let pink = Color::Rgb(236, 72, 153); // #EC4899 - Hot pink
        let text_fg = Color::Rgb(250, 250, 250); // #FAFAFA - High contrast

        let slow = Duration::from_millis(1250);
        let medium = Duration::from_millis(750);

        let _glitch: Effect = Glitch::builder()
            .rng(SimpleRng::default())
            .action_ms(200..400)
            .action_start_delay_ms(0..1)
            .cell_glitch_ratio(1.0)
            .build()
            .into_effect();

        // Custom color cycle using Tokyo Night palette
        let custom_color_cycle = fx::effect_fn(Instant::now(), slow, |state, _ctx, cell_iter| {
            let cycle: f32 = (state.elapsed().as_millis() % 3600) as f32;

            cell_iter
                .filter(|(_, cell)| cell.symbol() != " ")
                .enumerate()
                .for_each(|(i, (_pos, cell))| {
                    let hue = (2.0 * i as f32 + cycle * 0.2) % 360.0;
                    let color = color_from_hsl(hue, 80.0, 65.0); // Professional saturation and lightness
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
                        Style::new().fg(pink).bg(screen_bg),
                        1200,
                    ),
                    fx::sleep(slow),
                    fx::expand(
                        ExpandDirection::Horizontal,
                        Style::new().fg(cyan).bg(screen_bg),
                        1200,
                    )
                    .reversed(),
                ]),
            ),
            (
                "irregular sweep out/sweep in",
                sequence(&[
                    fx::sweep_out(Motion::DownToUp, 5, 20, orange, (2000, QuadOut)),
                    fx::sweep_in(Motion::UpToDown, 5, 20, blue, (2000, QuadOut)),
                    fx::sweep_out(Motion::UpToDown, 5, 20, green, (2000, QuadOut)),
                    fx::sweep_in(Motion::DownToUp, 5, 20, purple, (2000, QuadOut)),
                ]),
            ),
            (
                "coalesce",
                fx::sequence(&[
                    fx::coalesce((medium, CubicOut)),
                    fx::sleep(medium),
                    fx::prolong_end(
                        medium,
                        fx::dissolve_to(Style::default().bg(screen_bg).fg(cyan), medium),
                    ),
                ]),
            ),
            (
                "slide in/out",
                fx::repeating(sequence(&[
                    parallel(&[
                        fx::fade_from_fg(yellow, (2000, ExpoInOut)),
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


// Public API for integrating tachyonfx effects into the chat app
pub struct TachyonEffects {
    effects: Vec<(&'static str, Effect)>,
}

impl TachyonEffects {
    pub fn new(bg_color: Color, screen_bg: Color) -> Self {
        let slow = Duration::from_millis(1250);
        let medium = Duration::from_millis(750);

        // Custom color cycle effect
        let custom_color_cycle = fx::effect_fn(Instant::now(), slow, |state, _ctx, cell_iter| {
            let cycle: f32 = (state.elapsed().as_millis() % 3600) as f32;

            cell_iter
                .filter(|(_, cell)| cell.symbol() != " ")
                .enumerate()
                .for_each(|(i, (_pos, cell))| {
                    let hue = (2.0 * i as f32 + cycle * 0.2) % 360.0;
                    let color = color_from_hsl(hue, 100.0, 50.0);
                    cell.set_fg(color);
                });
        })
        .with_filter(CellFilter::FgColor(Color::Rgb(200, 200, 200)));

        let effects = vec![
            (
                "sweep in",
                fx::sweep_in(Motion::LeftToRight, 30, 0, screen_bg, (slow, QuadOut)),
            ),
            (
                "smooth expand",
                sequence(&[
                    fx::expand(
                        ExpandDirection::Vertical,
                        Style::new().fg(bg_color).bg(screen_bg),
                        1200,
                    ),
                    fx::sleep(slow),
                    fx::expand(
                        ExpandDirection::Horizontal,
                        Style::new().fg(bg_color).bg(screen_bg),
                        1200,
                    )
                    .reversed(),
                ]),
            ),
            (
                "sweep out/in",
                sequence(&[
                    fx::sweep_out(Motion::DownToUp, 5, 20, bg_color, (2000, QuadOut)),
                    fx::sweep_in(Motion::UpToDown, 5, 20, bg_color, (2000, QuadOut)),
                    fx::sweep_out(Motion::UpToDown, 5, 20, bg_color, (2000, QuadOut)),
                    fx::sweep_in(Motion::DownToUp, 5, 20, bg_color, (2000, QuadOut)),
                ]),
            ),
            (
                "coalesce",
                fx::sequence(&[
                    fx::coalesce((medium, CubicOut)),
                    fx::sleep(medium),
                    fx::prolong_end(
                        medium,
                        fx::dissolve_to(Style::default().bg(screen_bg), medium),
                    ),
                ]),
            ),
            (
                "slide in/out",
                fx::repeating(sequence(&[
                    parallel(&[
                        fx::fade_from_fg(bg_color, (2000, ExpoInOut)),
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
                "hsl shift",
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
            ("color cycle", never_complete(custom_color_cycle)),
        ];

        Self { effects }
    }

    pub fn get_effect(&self, idx: usize) -> (&'static str, Effect) {
        self.effects[idx % self.effects.len()].clone()
    }
}
