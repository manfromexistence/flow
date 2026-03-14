use std::time::Instant;
use ratatui::style::{Color, Style};
use tachyonfx::{
    CellFilter, Duration, Effect, Interpolation::*, Motion,
    fx::{self, ExpandDirection, never_complete, parallel, sequence},
};

/// Public API for integrating tachyonfx effects into the chat app
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
                    let color = tachyonfx::color_from_hsl(hue, 100.0, 50.0);
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

    pub fn len(&self) -> usize {
        self.effects.len()
    }
}
