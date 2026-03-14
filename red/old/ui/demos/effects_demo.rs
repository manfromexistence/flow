use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};
use std::time::Instant;
use tachyonfx::{
    CellFilter, Duration, Effect, EffectRenderer, Interpolation, IntoEffect, Motion, Shader,
    SimpleRng, color_from_hsl,
    fx::{self, ExpandDirection, never_complete, parallel, sequence},
};

use crate::ui::chat::{modal_list::ModalList, theme::ChatTheme};

pub struct EffectsDemoModal {
    pub active_effect: (&'static str, Effect),
    pub active_effect_idx: usize,
    pub last_tick: Duration,
    pub effects: EffectsRepository,
    pub auto_cycle: bool,
    pub last_cycle_time: Instant,
}

impl EffectsDemoModal {
    pub fn new() -> Self {
        let effects = EffectsRepository::new();
        let active_effect = effects.get_effect(0);

        Self {
            active_effect,
            active_effect_idx: 0,
            last_tick: Duration::ZERO,
            effects,
            auto_cycle: true,
            last_cycle_time: Instant::now(),
        }
    }

    pub fn update(&mut self, elapsed: std::time::Duration) {
        self.last_tick = elapsed.into();

        // Auto-cycle through effects every 2 seconds (faster)
        if self.auto_cycle && self.last_cycle_time.elapsed().as_secs() >= 2 {
            let fx_idx = (self.active_effect_idx + 1) % self.effects.len();
            self.active_effect = self.effects.get_effect(fx_idx);
            self.active_effect_idx = fx_idx;
            self.last_cycle_time = Instant::now();
        }
    }

    pub fn next_effect(&mut self) {
        let fx_idx = (self.active_effect_idx + 1) % self.effects.len();
        self.active_effect = self.effects.get_effect(fx_idx);
        self.active_effect_idx = fx_idx;
        self.last_cycle_time = Instant::now();
    }

    pub fn prev_effect(&mut self) {
        let fx_idx = if self.active_effect_idx == 0 {
            self.effects.len() - 1
        } else {
            self.active_effect_idx - 1
        };
        self.active_effect = self.effects.get_effect(fx_idx);
        self.active_effect_idx = fx_idx;
        self.last_cycle_time = Instant::now();
    }

    pub fn restart_effect(&mut self) {
        self.active_effect = self.effects.get_effect(self.active_effect_idx);
        self.last_cycle_time = Instant::now();
    }

    pub fn toggle_auto_cycle(&mut self) {
        self.auto_cycle = !self.auto_cycle;
        self.last_cycle_time = Instant::now();
    }

    pub fn random_effect(&mut self) {
        use tachyonfx::SimpleRng;
        let mut rng = SimpleRng::default();
        let fx_idx = (rng.r#gen() % self.effects.len() as u32) as usize;
        self.active_effect = self.effects.get_effect(fx_idx);
        self.active_effect_idx = fx_idx;
        self.last_cycle_time = Instant::now();
    }
}

pub fn render(area: Rect, buf: &mut Buffer, theme: &ChatTheme, demo: &mut EffectsDemoModal) {
    let modal_width = area.width.saturating_sub(10).min(100);
    let modal_height = area.height.saturating_sub(6).min(30);
    let modal_area = Rect {
        x: (area.width.saturating_sub(modal_width)) / 2,
        y: (area.height.saturating_sub(modal_height)) / 2,
        width: modal_width,
        height: modal_height,
    };

    // Clear the entire modal area first
    Clear.render(modal_area, buf);

    // Use tachyonfx original colors for proper effect rendering
    let screen_bg = Color::Black;
    let bg = Color::Rgb(40, 40, 40); // Dark gray similar to original

    // Render screen background
    Block::default()
        .style(Style::default().bg(screen_bg))
        .render(modal_area, buf);

    // Create content area with border
    let content_area = modal_area.inner(Margin::new(2, 1));

    // Render content background FIRST (before text)
    Block::default()
        .style(Style::default().bg(bg))
        .render(content_area, buf);

    // Render border on top
    Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(251, 191, 36))) // Orange/yellow border
        .border_type(ratatui::widgets::BorderType::Rounded)
        .title(Span::styled(
            " TachyonFX Effects Demo ",
            Style::default()
                .fg(Color::Rgb(254, 215, 170)) // Bright orange
                .add_modifier(Modifier::BOLD),
        ))
        .render(content_area, buf);

    let inner = content_area.inner(Margin::new(1, 1));

    // Use tachyonfx original color scheme
    let anim_style = [
        Style::default().fg(Color::Rgb(254, 128, 25)), // Orange
        Style::default()
            .fg(Color::Rgb(254, 215, 170))
            .add_modifier(Modifier::BOLD), // Bright orange
    ];
    let text_style = Style::default().fg(Color::Rgb(235, 219, 178)); // Light beige
    let shortcut_style = [
        Style::default()
            .fg(Color::Rgb(250, 189, 47)) // Yellow
            .add_modifier(Modifier::BOLD),
        Style::default().fg(Color::Rgb(168, 153, 132)), // Muted gray
    ];

    let layout = Layout::vertical([
        Constraint::Length(2),
        Constraint::Length(7),
        Constraint::Length(8),
    ])
    .split(inner);

    let active_animation: Line = Line::from(vec![
        Span::from("Active animation: ").style(anim_style[0]),
        Span::from(demo.active_effect.0).style(anim_style[1]),
        Span::from(format!(
            " [{}/{}]",
            demo.active_effect_idx + 1,
            demo.effects.len()
        ))
        .style(
            Style::default()
                .fg(Color::Rgb(250, 189, 47))
                .add_modifier(Modifier::BOLD),
        ),
    ]);

    let main_text = Text::from(vec![
        Line::from("Many effects are composable, e.g. `parallel`, `sequence`, `repeating`."),
        Line::from("Most effects have a lifetime, after which they report done()."),
        Line::from("Effects such as `never_complete`, `temporary` influence or override this."),
        Line::from(""),
        Line::from("The text in this window will undergo transitions automatically"),
        Line::from("every 2 seconds, or use the following keys:"),
    ])
    .style(text_style);

    let shortcut = |key: &'static str, desc: &'static str| {
        Line::from(vec![
            Span::from(key).style(shortcut_style[0]),
            Span::from(desc).style(shortcut_style[1]),
        ])
    };

    let auto_status_text = if demo.auto_cycle {
        "toggle auto-cycle (ON)"
    } else {
        "toggle auto-cycle (OFF)"
    };
    let shortcuts = Text::from(vec![
        shortcut("↵   ", "next transition"),
        shortcut("⌫   ", "previous transition"),
        shortcut("␣   ", "restart transition"),
        shortcut("a   ", auto_status_text),
        shortcut("r   ", "random transition"),
        shortcut("ESC ", "close"),
    ]);

    // Render all text widgets FIRST
    Paragraph::new(active_animation).render(layout[0], buf);
    Paragraph::new(main_text).render(layout[1], buf);
    Paragraph::new(shortcuts).render(layout[2], buf);

    // CRITICAL: Apply effect to the ENTIRE content_area (not just text area)
    // This is how tachyonfx works - effects operate on all rendered cells
    if demo.active_effect.1.running() {
        demo.active_effect
            .1
            .process(demo.last_tick, buf, content_area);
    }
}

struct EffectsRepository {
    effects: Vec<(&'static str, Effect)>,
}

impl EffectsRepository {
    fn new() -> Self {
        // Use EXACT colors from tachyonfx basic-effects example
        let screen_bg = Color::Black;
        let bg = Color::Rgb(40, 40, 40); // Dark gray (Dark0Soft equivalent)

        let slow = Duration::from_millis(600);
        let medium = Duration::from_millis(400);

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
        .with_filter(CellFilter::FgColor(Color::Rgb(235, 219, 178))); // Light3 equivalent

        let effects = vec![
            (
                "sweep in",
                fx::sweep_in(
                    Motion::LeftToRight,
                    30,
                    0,
                    screen_bg,
                    (slow, Interpolation::QuadOut),
                ),
            ),
            (
                "smooth expand and reversed",
                sequence(&[
                    fx::expand(
                        ExpandDirection::Vertical,
                        Style::new().fg(bg).bg(screen_bg),
                        (600, Interpolation::BounceOut),
                    ),
                    fx::sleep(slow),
                    fx::expand(
                        ExpandDirection::Horizontal,
                        Style::new().fg(bg).bg(screen_bg),
                        (600, Interpolation::BounceOut),
                    ),
                ]),
            ),
            (
                "irregular sweep out/sweep in",
                sequence(&[
                    fx::sweep_out(Motion::DownToUp, 5, 20, bg, (800, Interpolation::QuadOut)),
                    fx::sweep_in(Motion::UpToDown, 5, 20, bg, (800, Interpolation::QuadOut)),
                    fx::sweep_out(Motion::UpToDown, 5, 20, bg, (800, Interpolation::QuadOut)),
                    fx::sweep_in(Motion::DownToUp, 5, 20, bg, (800, Interpolation::QuadOut)),
                ]),
            ),
            (
                "coalesce",
                fx::sequence(&[
                    fx::coalesce((medium, Interpolation::CubicOut)),
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
                        fx::fade_from_fg(bg, (800, Interpolation::ExpoInOut)),
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
