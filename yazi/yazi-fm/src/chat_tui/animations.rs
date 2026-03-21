//! Animation rendering functions adapted for Buffer rendering

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Paragraph, Widget},
};

use super::state::ChatState;

impl ChatState {
    /// Helper: get a rainbow color as ratatui Color
    pub fn rainbow_color(&self, index: usize) -> ratatui::style::Color {
        let c = self.rainbow_animation.rgb_color_at(index);
        ratatui::style::Color::Rgb(c.r, c.g, c.b)
    }

    /// Helper: get theme bg as ratatui Color
    pub fn theme_bg_color(&self) -> ratatui::style::Color {
        self.theme.bg
    }

    pub fn render_matrix_animation_in_area(&self, area: Rect, buf: &mut Buffer) {
        use ratatui::style::Color;

        let bg_color = self.theme_bg_color();

        // Authentic Matrix characters
        let chars = vec![
            'ﾊ', 'ﾐ', 'ﾋ', 'ｰ', 'ｳ', 'ｼ', 'ﾅ', 'ﾓ', 'ﾆ', 'ｻ', 'ﾜ', 'ﾂ', 'ｵ', 'ﾘ', 'ｱ', 'ﾎ', 'ﾃ',
            'ﾏ', 'ｹ', 'ﾒ', 'ｴ', 'ｶ', 'ｷ', 'ﾑ', 'ﾕ', 'ﾗ', 'ｾ', 'ﾈ', 'ｽ', 'ﾀ', 'ﾇ', 'ﾍ', '0', '1',
            '2', '3', '4', '5', '6', '7', '8', '9', ':', '.', '"', '=', '*', '+', '-', '<', '>',
            '¦', '|', 'Z',
        ];

        let elapsed_ms = self
            .animation_start_time
            .map(|t| t.elapsed().as_millis() as usize)
            .unwrap_or(0);

        // Pure Matrix green colors
        let green_g = 255;

        let mut screen: Vec<Vec<(char, Color)>> =
            vec![vec![(' ', bg_color); area.width as usize]; area.height as usize];

        for x in 0..area.width {
            if (x * 7) % 3 != 0 {
                continue;
            }

            let column_speed = 1 + ((x * 11) % 2) as usize;
            let column_length = 8 + ((x * 13) % 12);
            let column_offset = (x * 17) % 40;

            let fall_progress =
                ((elapsed_ms / (150 / column_speed)) + column_offset as usize) as i32;
            let head_y = (fall_progress % (area.height as i32 + 30)) - 10;

            for trail_pos in 0..column_length {
                let y = head_y - trail_pos as i32;

                if y >= 0 && y < area.height as i32 {
                    let char_idx = ((x as usize * 31 + y as usize * 17 + elapsed_ms / 200)
                        % chars.len()) as usize;

                    let color = if trail_pos == 0 {
                        // Bright white-green head
                        Color::Rgb(200, 255, 200)
                    } else {
                        // Green fade trail
                        let fade = 1.0 - (trail_pos as f32 / column_length as f32) * 0.85;
                        Color::Rgb(0, (green_g as f32 * fade) as u8, 0)
                    };

                    screen[y as usize][x as usize] = (chars[char_idx], color);
                }
            }
        }

        let mut lines = vec![];
        for row in screen {
            let mut spans = vec![];
            for (ch, color) in row {
                spans.push(Span::styled(ch.to_string(), Style::default().fg(color)));
            }
            lines.push(Line::from(spans));
        }

        Paragraph::new(lines)
            .style(Style::default().bg(bg_color))
            .render(area, buf);
    }

    pub fn render_train_animation_in_area(&self, area: Rect, buf: &mut Buffer) {
        let bg_color = self.theme_bg_color();
        let elapsed_ms = self
            .animation_start_time
            .map(|t| t.elapsed().as_millis() as i32)
            .unwrap_or(0);

        let train_width = 60;

        // Train starts just inside the right edge (appears earlier)
        // and moves slower (divisor 50 instead of 15) and loops infinitely
        let total_travel = area.width as i32 + train_width + 20;
        let x_pos = (area.width as i32 - 10) - ((elapsed_ms / 50) % total_travel);

        let train = vec![
            "      ====        ________                ___________",
            "  _D _|  |_______/        \\__I_I_____===__|_________|",
            "   |(_)---  |   H\\________/ |   |        =|___ ___|",
            "   /     |  |   H  |  |     |   |         ||_| |_||",
            "  |      |  |   H  |__--------------------| [___] |",
            "  | ________|___H__/__|_____/[][]~\\_______|       |",
            "  |/ |   |-----------I_____I [][] []  D   |=======|",
            "__/ =| o |=-~~\\  /~~\\  /~~\\  /~~\\ ____Y___________|",
            " |/-=|___|=O=====O=====O=====O   |_____/~\\___/",
            "  \\_/      \\__/  \\__/  \\__/  \\__/      \\_/",
        ];

        // Add smoke that animates above the train
        let smoke_frames: Vec<&[&str]> = vec![
            &["    (  )", "   (    )", "  (      )"],
            &["   (   )", "  (     )", " (       )"],
            &["  (    )", " (      )", "(        )"],
        ];
        let smoke_frame_idx = ((elapsed_ms / 200) as usize) % smoke_frames.len();
        let smoke = smoke_frames[smoke_frame_idx];

        let y_start = (area
            .height
            .saturating_sub((train.len() + smoke.len()) as u16))
            / 2;
        let mut lines = vec![];

        for _ in 0..y_start {
            lines.push(Line::from(""));
        }

        // Render smoke above the train
        let smoke_x_offset = x_pos + 6; // position smoke above the smokestack
        for smoke_line in smoke {
            if smoke_x_offset >= -train_width && smoke_x_offset < area.width as i32 {
                let mut spans = Vec::new();
                if smoke_x_offset >= 0 {
                    let padding = " ".repeat(smoke_x_offset as usize);
                    spans.push(Span::raw(padding));
                    for (ci, ch) in smoke_line.chars().enumerate() {
                        let color_idx = (ci + (elapsed_ms / 150) as usize) % 50;
                        let color = self.rainbow_color(color_idx);
                        spans.push(Span::styled(ch.to_string(), Style::default().fg(color)));
                    }
                } else {
                    let visible_start = (-smoke_x_offset) as usize;
                    if visible_start < smoke_line.len() {
                        for (ci, ch) in smoke_line[visible_start..].chars().enumerate() {
                            let color_idx = (ci + visible_start + (elapsed_ms / 150) as usize) % 50;
                            let color = self.rainbow_color(color_idx);
                            spans.push(Span::styled(ch.to_string(), Style::default().fg(color)));
                        }
                    }
                }
                lines.push(Line::from(spans));
            } else {
                lines.push(Line::from(""));
            }
        }

        for (line_idx, line) in train.iter().enumerate() {
            if x_pos >= -train_width && x_pos < area.width as i32 {
                if x_pos >= 0 {
                    let padding = " ".repeat(x_pos as usize);
                    let mut spans = vec![Span::raw(padding)];

                    for (char_idx, ch) in line.chars().enumerate() {
                        let color_idx =
                            (char_idx + line_idx * 3 + (elapsed_ms / 100) as usize) % 50;
                        let ratatui_color = self.rainbow_color(color_idx);
                        spans.push(Span::styled(
                            ch.to_string(),
                            Style::default().fg(ratatui_color),
                        ));
                    }
                    lines.push(Line::from(spans));
                } else {
                    let visible_start = (-x_pos) as usize;
                    if visible_start < line.len() {
                        let mut spans = Vec::new();
                        for (char_idx, ch) in line[visible_start..].chars().enumerate() {
                            let color_idx = (char_idx
                                + visible_start
                                + line_idx * 3
                                + (elapsed_ms / 100) as usize)
                                % 50;
                            let ratatui_color = self.rainbow_color(color_idx);
                            spans.push(Span::styled(
                                ch.to_string(),
                                Style::default().fg(ratatui_color),
                            ));
                        }
                        lines.push(Line::from(spans));
                    } else {
                        lines.push(Line::from(""));
                    }
                }
            } else {
                lines.push(Line::from(""));
            }
        }

        // Render tracks under the train
        let track_offset = (elapsed_ms / 50) as usize;
        let mut track_spans = Vec::new();
        for x in 0..area.width as usize {
            let ch = if (x + track_offset).is_multiple_of(4) {
                '╫'
            } else {
                '═'
            };
            let color_idx = (x + (elapsed_ms / 200) as usize) % 50;
            let color = self.rainbow_color(color_idx);
            track_spans.push(Span::styled(ch.to_string(), Style::default().fg(color)));
        }
        lines.push(Line::from(track_spans));

        Paragraph::new(lines)
            .style(Style::default().bg(bg_color))
            .render(area, buf);
    }

    pub fn render_confetti_animation_in_area(&self, area: Rect, buf: &mut Buffer) {
        let elapsed = self.rainbow_animation.elapsed();
        let elapsed_ms = (elapsed * 1000.0) as u64;
        let bg_color = self.theme_bg_color();

        let confetti_chars = [
            '*', '+', 'o', '~', '#', '@', '%', '&', '!', '^', '▪', '▫', '●', '◆', '◇',
        ];

        let w = area.width as usize;
        let h = area.height as usize;
        let mut grid: Vec<Vec<Option<(char, ratatui::style::Color)>>> = vec![vec![None; w]; h];

        // Multiple explosion sources that trigger periodically
        let num_explosions = 3;
        let explosion_cycle_ms: u64 = 5000;

        for explosion_id in 0..num_explosions {
            // Each explosion has a different center and timing
            let explosion_offset = explosion_id * (explosion_cycle_ms / num_explosions);
            let local_time = (elapsed_ms.wrapping_add(explosion_offset)) % explosion_cycle_ms;
            let age = local_time as f64 / 1000.0; // seconds since this explosion

            // Explosion center - varies per explosion
            let seed = explosion_id;
            let center_x = match explosion_id {
                0 => w as f64 / 2.0,
                1 => w as f64 / 4.0,
                _ => 3.0 * w as f64 / 4.0,
            };
            let center_y = match explosion_id {
                0 => h as f64 / 3.0,
                1 => h as f64 / 2.0,
                _ => h as f64 / 4.0,
            };

            let num_particles = 80;
            let gravity = 12.0;
            let air_drag: f64 = 0.97;

            for i in 0..num_particles {
                let particle_seed = seed * 1000 + i as u64;

                // Explosion: particles burst outward from center in all directions
                // Use golden angle for even distribution
                let angle = (i as f64 * 2.39996322) + (seed as f64 * 1.7);
                let speed_base = 8.0 + ((particle_seed * 7919) % 200) as f64 / 10.0; // 8-28 units/s
                let speed_variation = 1.0 + ((particle_seed * 3571) % 100) as f64 / 200.0;
                let speed = speed_base * speed_variation;

                let vx = angle.cos() * speed * 1.8; // wider horizontal spread for terminal
                let vy = angle.sin() * speed * 0.8 - 5.0; // bias upward initially

                // Physics with drag: approximate position
                // With drag factor d per second: v(t) = v0 * d^t, x(t) = v0 * (d^t - 1) / ln(d)
                let drag_t = air_drag.powf(age * 30.0); // approximate drag over frames
                let px = center_x + vx * age * drag_t;
                let py = center_y + vy * age * drag_t + 0.5 * gravity * age * age;

                let px_i = px as i32;
                let py_i = py as i32;

                if px_i >= 0 && px_i < w as i32 && py_i >= 0 && py_i < h as i32 {
                    // Fade out over time
                    let fade = (1.0 - age / (explosion_cycle_ms as f64 / 1000.0)).max(0.0);
                    if fade > 0.05 {
                        let char_idx = (particle_seed as usize + (elapsed_ms / 150) as usize)
                            % confetti_chars.len();
                        let color_idx =
                            (i * 7 + explosion_id as usize * 13 + (elapsed_ms / 60) as usize) % 50;
                        let c = self.rainbow_animation.rgb_color_at(color_idx);
                        let color = ratatui::style::Color::Rgb(
                            (c.r as f64 * fade) as u8,
                            (c.g as f64 * fade) as u8,
                            (c.b as f64 * fade) as u8,
                        );

                        // Spinning character effect based on particle rotation
                        let spin_chars = ['|', '/', '─', '\\'];
                        let spin_idx = ((age * 8.0) as usize + i) % spin_chars.len();
                        let ch = if i % 3 == 0 {
                            spin_chars[spin_idx]
                        } else {
                            confetti_chars[char_idx]
                        };

                        grid[py_i as usize][px_i as usize] = Some((ch, color));
                    }
                }
            }
        }

        let mut lines = Vec::new();
        for row in grid.iter().take(h) {
            let mut spans = Vec::new();
            for cell in row.iter().take(w) {
                if let Some((ch, color)) = cell {
                    spans.push(Span::styled(ch.to_string(), Style::default().fg(*color)));
                } else {
                    spans.push(Span::raw(" "));
                }
            }
            lines.push(Line::from(spans));
        }

        Paragraph::new(lines)
            .style(Style::default().bg(bg_color))
            .render(area, buf);
    }

    // Placeholder stubs for other animations - implement as needed
    pub fn render_gameoflife_animation_in_area(&self, _area: Rect, _buf: &mut Buffer) {
        // TODO: Implement Game of Life animation
    }

    pub fn render_starfield_animation_in_area(&self, _area: Rect, _buf: &mut Buffer) {
        // TODO: Implement Starfield animation
    }

    pub fn render_rain_animation_in_area(&self, _area: Rect, _buf: &mut Buffer) {
        // TODO: Implement Rain animation
    }

    pub fn render_nyancat_animation_in_area(&self, _area: Rect, _buf: &mut Buffer) {
        // TODO: Implement Nyan Cat animation
    }

    pub fn render_dvdlogo_animation_in_area(&self, _area: Rect, _buf: &mut Buffer) {
        // TODO: Implement DVD Logo animation
    }

    pub fn render_fire_animation_in_area(&self, _area: Rect, _buf: &mut Buffer) {
        // TODO: Implement Fire animation
    }

    pub fn render_plasma_animation_in_area(&self, _area: Rect, _buf: &mut Buffer) {
        // TODO: Implement Plasma animation
    }

    pub fn render_spinners_animation_in_area(&self, _area: Rect, _buf: &mut Buffer) {
        // TODO: Implement Spinners animation
    }

    pub fn render_waves_animation_in_area(&self, _area: Rect, _buf: &mut Buffer) {
        // TODO: Implement Waves animation
    }

    pub fn render_fireworks_animation_in_area(&self, _area: Rect, _buf: &mut Buffer) {
        // TODO: Implement Fireworks animation
    }
}
