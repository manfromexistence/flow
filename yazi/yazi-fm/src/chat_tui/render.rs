use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    widgets::Widget,
};

use super::{
    components::MessageList,
    state::{AnimationType, ChatState},
};

impl ChatState {
    pub fn render(&mut self, area: Rect, buf: &mut Buffer) {
        // Update tachyon effects timing
        let _elapsed = self.last_render.elapsed();

        // Both animations show in chat area only, keeping input visible
        if self.show_train_animation || self.show_matrix_animation {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(10),
                    Constraint::Length(3),
                    Constraint::Length(1),
                ])
                .split(area);

            self.input_area = chunks[1];

            // Render appropriate animation in the chat area
            if self.show_train_animation {
                self.render_train_animation_in_area(chunks[0], buf);
            } else if self.show_matrix_animation {
                self.render_matrix_animation_in_area(chunks[0], buf);
            }

            // Still render the input bar and bottom controls
            self.render_input_box(chunks[1], buf);

            let (plan_area, model_area, _token_area, local_area) =
                self.render_bottom_controls(chunks[2], buf);

            self.plan_button_area = plan_area;
            self.model_button_area = model_area;
            self.local_button_area = local_area;
            return;
        }

        // Animation carousel mode
        if self.animation_mode {
            let animations = AnimationType::all();
            let current_anim = animations[self.current_animation_index];

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(10),
                    Constraint::Length(3),
                    Constraint::Length(1),
                ])
                .split(area);

            self.input_area = chunks[1];

            // Render the current animation in the chat area
            match current_anim {
                AnimationType::Splash => {
                    super::splash::render(
                        chunks[0],
                        buf,
                        &self.theme,
                        self.splash_font_index,
                        &self.rainbow_animation,
                    );
                }
                AnimationType::Matrix => {
                    self.render_matrix_animation_in_area(chunks[0], buf);
                }
                AnimationType::Train => {
                    self.render_train_animation_in_area(chunks[0], buf);
                }
                AnimationType::Confetti => {
                    self.render_confetti_animation_in_area(chunks[0], buf);
                }
                AnimationType::GameOfLife => {
                    self.render_gameoflife_animation_in_area(chunks[0], buf);
                }
                AnimationType::Starfield => {
                    self.render_starfield_animation_in_area(chunks[0], buf);
                }
                AnimationType::Rain => {
                    self.render_rain_animation_in_area(chunks[0], buf);
                }
                AnimationType::NyanCat => {
                    self.render_nyancat_animation_in_area(chunks[0], buf);
                }
                AnimationType::DVDLogo => {
                    self.render_dvdlogo_animation_in_area(chunks[0], buf);
                }
                AnimationType::TachyonDemo => {
                    // TachyonDemo needs Frame, skip for now
                    super::splash::render(
                        chunks[0],
                        buf,
                        &self.theme,
                        self.splash_font_index,
                        &self.rainbow_animation,
                    );
                }
                AnimationType::Fire => {
                    self.render_fire_animation_in_area(chunks[0], buf);
                }
                AnimationType::Plasma => {
                    self.render_plasma_animation_in_area(chunks[0], buf);
                }
                AnimationType::Spinners => {
                    self.render_spinners_animation_in_area(chunks[0], buf);
                }
                AnimationType::Waves => {
                    self.render_waves_animation_in_area(chunks[0], buf);
                }
                AnimationType::Fireworks => {
                    self.render_fireworks_animation_in_area(chunks[0], buf);
                }
            }

            // Render input box and bottom controls
            self.render_input_box(chunks[1], buf);

            let (plan_area, model_area, _token_area, local_area) =
                self.render_bottom_controls(chunks[2], buf);

            self.plan_button_area = plan_area;
            self.model_button_area = model_area;
            self.local_button_area = local_area;
            return;
        }

        if self.show_dx_splash {
            // Show DX splash screen
            super::splash::render(
                area,
                buf,
                &self.theme,
                self.splash_font_index,
                &self.rainbow_animation,
            );
            return;
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(10),
                Constraint::Length(3),
                Constraint::Length(1),
            ])
            .split(area);

        self.input_area = chunks[1];

        // Always render messages (even if empty) instead of splash screen
        MessageList::with_effects(
            &self.messages,
            &self.theme,
            self.chat_scroll_offset,
            &self.shimmer,
            &self.typing_indicator,
        )
        .render(chunks[0], buf);

        self.render_input_box(chunks[1], buf);

        let (plan_area, model_area, _token_area, local_area) =
            self.render_bottom_controls(chunks[2], buf);

        self.plan_button_area = plan_area;
        self.model_button_area = model_area;
        self.local_button_area = local_area;

        // Render performance overlay if enabled
        self.render_perf_overlay(area, buf);
    }
    
    pub fn render_dimmed(&mut self, area: Rect, buf: &mut Buffer) {
        // Simplified render for FilePicker mode - just show input box and controls
        // Split into input (3 lines) and controls (1 line)
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),   // Input box (3 lines)
                Constraint::Length(1),   // Bottom controls (1 line)
            ])
            .split(area);

        self.input_area = chunks[0];

        // Render input box
        self.render_input_box(chunks[0], buf);

        // Render bottom controls
        let (plan_area, model_area, _token_area, local_area) =
            self.render_bottom_controls(chunks[1], buf);

        self.plan_button_area = plan_area;
        self.model_button_area = model_area;
        self.local_button_area = local_area;
    }
}


// Input rendering methods
use ratatui::{
    style::{Style, Modifier},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
};

impl ChatState {
    pub fn render_input_box(&mut self, area: Rect, buf: &mut Buffer) {
        // Start timing input render
        self.perf_monitor.start_timing();

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.border))
            .border_type(ratatui::widgets::BorderType::Rounded)
            .style(Style::default()); // Transparent background - no bg color set

        let inner = block.inner(area);
        block.render(area, buf);

        // Add horizontal padding inside the input box
        let padded_inner = Rect {
            x: inner.x + 1,
            y: inner.y,
            width: inner.width.saturating_sub(2),
            height: inner.height,
        };

        self.render_input_text(padded_inner, buf);
        self.render_input_cursor(padded_inner, buf);

        // Record input render time
        self.last_input_render_time = self.perf_monitor.record_input_render();
    }

    fn render_input_text(&self, area: Rect, buf: &mut Buffer) {
        let placeholder = "A question or a prompt... (Enter to send, Shift+Enter for new line)";
        let text = if self.input.content.is_empty() {
            Text::from(Line::from(Span::styled(
                placeholder,
                Style::default().fg(self.theme.border),
            )))
        } else {
            Text::from(self.input.content.as_str())
        };

        if self.input.has_selection() {
            self.render_selection(area, buf);
        } else {
            Paragraph::new(text)
                .wrap(Wrap { trim: false })
                .style(Style::default().fg(self.theme.fg)) // No bg - transparent
                .render(area, buf);
        }
    }

    fn render_selection(&self, area: Rect, buf: &mut Buffer) {
        let (sel_start, sel_end) = if let (Some(start), Some(end)) =
            (self.input.selection_start, self.input.selection_end)
        {
            if start < end {
                (start, end)
            } else {
                (end, start)
            }
        } else {
            (0, 0)
        };

        let mut x = area.x;
        let mut y = area.y;

        for (i, ch) in self.input.content.chars().enumerate() {
            if x >= area.right() {
                x = area.x;
                y += 1;
                if y >= area.bottom() {
                    break;
                }
            }

            let is_selected = i >= sel_start && i < sel_end;
            let style = if is_selected {
                Style::default().bg(self.theme.fg).fg(self.theme.bg)
            } else {
                Style::default().bg(self.theme.bg).fg(self.theme.fg)
            };

            let cell = &mut buf[(x, y)];
            cell.set_char(ch);
            cell.set_style(style);
            x += 1;
        }
    }

    fn render_input_cursor(&self, area: Rect, buf: &mut Buffer) {
        if self.cursor_visible {
            let cursor_x = area.x + (self.input.cursor_position as u16 % area.width);
            let cursor_y = area.y + (self.input.cursor_position as u16 / area.width);

            if cursor_x < area.right() && cursor_y < area.bottom() {
                let cell = &mut buf[(cursor_x, cursor_y)];
                let existing_char = cell.symbol().chars().next().unwrap_or(' ');
                let rainbow_color = self.rainbow_cursor.current_color();

                if existing_char == ' ' || self.input.content.is_empty() {
                    cell.set_char('▎');
                    cell.set_style(Style::default().fg(rainbow_color));
                } else {
                    cell.set_style(Style::default().bg(rainbow_color).fg(self.theme.bg));
                }
            }
        }
    }

    pub fn render_bottom_controls(&self, area: Rect, buf: &mut Buffer) -> (Rect, Rect, Rect, Rect) {
        let shortcuts = [
            "Ctrl+M: Toggle Menu | Alt+A: Audio",
            "Alt+1/2/3: Switch Modes | Alt+T: Theme",
            "Enter: Send | Shift+Enter: New Line",
            "Ctrl+A/E: Start/End | Ctrl+U/K: Clear",
            "Ctrl+W: Delete Word | Ctrl+D: Exit",
            "Up/Down: Scroll | 0: Toggle Thinking",
        ];

        let current_shortcut = shortcuts[self.shortcut_index % shortcuts.len()];

        let mode_text = "Agent"; // Simplified for minimal version

        let local_width = self.selected_local_mode.len() as u16;
        let mode_width = mode_text.len() as u16;
        let model_width = self.selected_model.len() as u16;

        // Calculate token usage
        let total_tokens: usize = self
            .messages
            .iter()
            .map(|msg| msg.content.len() / 4) // Rough estimate: 1 token ≈ 4 chars
            .sum();
        let context_limit = 128_000; // 128K context window
        let token_ratio = if context_limit > 0 {
            (total_tokens as f32 / context_limit as f32 * 100.0) as u32
        } else {
            0
        };
        let token_info = format!(
            "{:.1}K/{}K({}%)",
            total_tokens as f32 / 1000.0,
            context_limit / 1000,
            token_ratio
        );
        let token_width = token_info.len() as u16;

        // Get current working directory and truncate
        let cwd = std::env::current_dir()
            .ok()
            .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
            .unwrap_or_else(|| "~".to_string());
        let path_info = format!("/{}", cwd);
        let path_width = path_info.len().min(20) as u16; // Truncate to max 20 chars
        let truncated_path = if path_info.len() > 20 {
            format!("..{}", &path_info[path_info.len() - 17..])
        } else {
            path_info.clone()
        };

        let spinner_width = if self.is_loading { 2 } else { 0 };

        let mut constraints = vec![
            Constraint::Length(local_width),
            Constraint::Length(1),
            Constraint::Length(mode_width),
            Constraint::Length(1),
            Constraint::Length(model_width),
            Constraint::Min(10),
            Constraint::Length(token_width),
            Constraint::Length(1),
            Constraint::Length(path_width),
        ];

        if self.is_loading {
            constraints.push(Constraint::Length(1));
            constraints.push(Constraint::Length(spinner_width));
        }

        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(constraints)
            .split(area);

        Paragraph::new(Span::styled(
            &self.selected_local_mode,
            Style::default().fg(self.theme.fg),
        ))
        .alignment(ratatui::layout::Alignment::Left)
        .render(bottom_chunks[0], buf);

        Paragraph::new(Span::styled(mode_text, Style::default().fg(self.theme.fg)))
            .alignment(ratatui::layout::Alignment::Left)
            .render(bottom_chunks[2], buf);

        Paragraph::new(Span::styled(
            &self.selected_model,
            Style::default().fg(self.theme.fg),
        ))
        .alignment(ratatui::layout::Alignment::Left)
        .render(bottom_chunks[4], buf);

        Paragraph::new(Span::styled(
            current_shortcut,
            Style::default().fg(self.theme.border),
        ))
        .alignment(ratatui::layout::Alignment::Center)
        .render(bottom_chunks[5], buf);

        // Token usage with color based on ratio
        let token_color = if token_ratio > 80 {
            ratatui::style::Color::Red
        } else if token_ratio > 60 {
            ratatui::style::Color::Yellow
        } else {
            self.theme.fg
        };

        Paragraph::new(Span::styled(&token_info, Style::default().fg(token_color)))
            .alignment(ratatui::layout::Alignment::Left)
            .render(bottom_chunks[6], buf);

        Paragraph::new(Span::styled(
            &truncated_path,
            Style::default().fg(self.theme.fg),
        ))
        .alignment(ratatui::layout::Alignment::Left)
        .render(bottom_chunks[8], buf);

        // Only show spinner when loading
        if self.is_loading {
            let spinner_frames = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];
            let elapsed_ms = (self.rainbow_animation.elapsed() * 1000.0) as u64;
            let frame_idx = ((elapsed_ms / 80) as usize) % spinner_frames.len();
            let spinner_char = spinner_frames[frame_idx];

            let color = self.rainbow_animation.rgb_color_at(frame_idx);
            let ratatui_color = ratatui::style::Color::Rgb(color.r, color.g, color.b);

            Paragraph::new(Span::styled(
                spinner_char.to_string(),
                Style::default()
                    .fg(ratatui_color)
                    .add_modifier(Modifier::BOLD),
            ))
            .alignment(ratatui::layout::Alignment::Left)
            .render(bottom_chunks[10], buf);
        }

        (
            bottom_chunks[2],
            bottom_chunks[4],
            bottom_chunks[6],
            bottom_chunks[0],
        )
    }

    pub fn render_perf_overlay(&self, area: Rect, buf: &mut Buffer) {
        if !self.show_perf_overlay {
            return;
        }

        let stats = self.perf_monitor.get_stats();

        // Create overlay area (top-right corner, 50 chars wide, 10 lines tall)
        let overlay_width = 52.min(area.width);
        let overlay_height = 10.min(area.height);
        let overlay_area = Rect {
            x: area.width.saturating_sub(overlay_width),
            y: 0,
            width: overlay_width,
            height: overlay_height,
        };

        // Determine status color
        let status_color = if self.perf_monitor.is_meeting_targets() {
            ratatui::style::Color::Green
        } else if stats.avg_frame_render_ms < 50.0 {
            ratatui::style::Color::Yellow
        } else {
            ratatui::style::Color::Red
        };

        // Build content lines
        let mut lines = vec![
            Line::from(vec![
                Span::styled("⚡ ", Style::default().fg(ratatui::style::Color::Yellow)),
                Span::styled(
                    "Performance Monitor",
                    Style::default()
                        .fg(ratatui::style::Color::Cyan)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("Input:    "),
                Span::styled(
                    format!("{:.2}ms", stats.avg_input_render_ms),
                    Style::default().fg(if stats.avg_input_render_ms < 16.0 {
                        ratatui::style::Color::Green
                    } else {
                        ratatui::style::Color::Yellow
                    }),
                ),
            ]),
            Line::from(vec![
                Span::raw("Status:  "),
                Span::styled(
                    if self.perf_monitor.is_meeting_targets() {
                        "✓ EXCELLENT"
                    } else {
                        "○ GOOD"
                    },
                    Style::default()
                        .fg(status_color)
                        .add_modifier(Modifier::BOLD),
                ),
            ]),
        ];

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(status_color))
            .border_type(ratatui::widgets::BorderType::Rounded)
            .style(Style::default().bg(ratatui::style::Color::Black));

        let paragraph = Paragraph::new(lines)
            .block(block)
            .style(Style::default().fg(self.theme.fg));

        paragraph.render(overlay_area, buf);
    }
}
