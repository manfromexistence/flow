use std::time::Instant;

use crate::gruvbox::Gruvbox::{self, Light3, Orange, OrangeBright};
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
}

impl TachyonDemo {
    pub fn new() -> Self {
        let effects = EffectsRepository::new();
        let active_effect = effects.get_effect(0);

        Self {
            active_effect,
            active_effect_idx: 0,
            last_tick: Duration::ZERO,
            effects,
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
        let screen_bg: Color = Gruvbox::Dark0Hard.into();
        let bg: Color = Gruvbox::Dark0Soft.into();

        Clear.render(f.area(), f.buffer_mut());
        Block::default()
            .style(Style::default().bg(screen_bg))
            .render(f.area(), f.buffer_mut());

        let content_area = f.area().inner_centered(80, 17);
        Block::default()
            .style(Style::default().bg(bg))
            .render(content_area, f.buffer_mut());

        let anim_style = [
            Style::default().fg(Orange.into()),
            Style::default().fg(OrangeBright.into()),
        ];
        let text_style = Style::default().fg(Light3.into());
        let shortcut_style = [
            Style::default()
                .fg(Gruvbox::YellowBright.into())
                .add_modifier(Modifier::BOLD),
            Style::default().fg(Gruvbox::Light4.into()),
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
            Line::from("Many effects are composable, e.g. `parallel`, `sequence`, `repeating`."),
            Line::from("Most effects have a lifetime, after which they report done()."),
            Line::from("Effects such as `never_complete`, `temporary` influence or override this."),
            Line::from(""),
            Line::from("The text in this window will undergo a random transition"),
            Line::from("when any of the following keys are pressed:"),
        ])
        .style(text_style);

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
    fn new() -> Self {
        let screen_bg = Gruvbox::Dark0Hard.into();
        let bg = Gruvbox::Dark0Soft.into();

        let slow = Duration::from_millis(1250);
        let medium = Duration::from_millis(750);

        let _glitch: Effect = Glitch::builder()
            .rng(SimpleRng::default())
            .action_ms(200..400)
            .action_start_delay_ms(0..1)
            .cell_glitch_ratio(1.0)
            .build()
            .into_effect();

        // fx from lambdas
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
        .with_filter(CellFilter::FgColor(Light3.into()));

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
                        Style::new().fg(bg).bg(screen_bg),
                        1200,
                    ),
                    fx::sleep(slow),
                    fx::expand(
                        ExpandDirection::Horizontal,
                        Style::new().fg(bg).bg(screen_bg),
                        1200,
                    )
                    .reversed(),
                ]),
            ),
            (
                "irregular sweep out/sweep in",
                sequence(&[
                    fx::sweep_out(Motion::DownToUp, 5, 20, bg, (2000, QuadOut)),
                    fx::sweep_in(Motion::UpToDown, 5, 20, bg, (2000, QuadOut)),
                    fx::sweep_out(Motion::UpToDown, 5, 20, bg, (2000, QuadOut)),
                    fx::sweep_in(Motion::DownToUp, 5, 20, bg, (2000, QuadOut)),
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
                        fx::fade_from_fg(bg, (2000, ExpoInOut)),
                        fx::slide_in(Motion::UpToDown, 20, 0, Gruvbox::Dark0Hard, medium),
                    ]),
                    fx::sleep(medium),
                    fx::prolong_end(
                        medium,
                        fx::slide_out(Motion::LeftToRight, 80, 0, Gruvbox::Dark0Hard, medium),
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
