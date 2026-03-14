//! Rendering logic for chat application

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};
use std::time::Duration;

use super::app_state::ModalType;

use super::app_state::ChatApp;
use super::{app_data::Focus, app_splash, components::MessageList, modals, modes::ChatMode};
use crate::ui::{demos, integrations};
use tachyonfx::{CellFilter, Effect, EffectRenderer, Interpolation, Shader, fx};

impl ChatApp {
    pub fn render(&mut self, frame: &mut ratatui::Frame) {
        // Update tachyon effects timing
        let elapsed = self.last_render.elapsed();
        self.tachyon_last_tick = tachyonfx::Duration::from(elapsed);

        // Update effects demo if it's showing
        if self.show_effects_demo_modal {
            self.effects_demo.update(elapsed);
        }

        // Both animations show in chat area only, keeping input visible
        if self.show_train_animation || self.show_matrix_animation {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(10),
                    Constraint::Length(3),
                    Constraint::Length(1),
                ])
                .split(frame.area());

            self.input_area = chunks[1];

            // Render appropriate animation in the chat area
            if self.show_train_animation {
                self.render_train_animation_in_area(chunks[0], frame);
            } else if self.show_matrix_animation {
                self.render_matrix_animation_in_area(chunks[0], frame);
            }

            // Still render the input bar and bottom controls
            self.render_input_box(chunks[1], frame.buffer_mut());

            let (plan_area, model_area, _token_area, local_area) =
                self.render_bottom_controls(chunks[2], frame.buffer_mut());

            self.plan_button_area = plan_area;
            self.model_button_area = model_area;
            self.local_button_area = local_area;
            return;
        }

        // Animation carousel mode
        if self.animation_mode {
            use super::app_state::AnimationType;
            let animations = AnimationType::all();
            let current_anim = animations[self.current_animation_index];

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Min(10),
                    Constraint::Length(3),
                    Constraint::Length(1),
                ])
                .split(frame.area());

            self.input_area = chunks[1];

            // Render the current animation in the chat area
            match current_anim {
                AnimationType::Splash => {
                    app_splash::render(
                        chunks[0],
                        frame.buffer_mut(),
                        &self.theme,
                        self.splash_font_index,
                        &self.rainbow_animation,
                    );
                }
                AnimationType::Matrix => {
                    self.render_matrix_animation_in_area(chunks[0], frame);
                }
                AnimationType::Train => {
                    self.render_train_animation_in_area(chunks[0], frame);
                }
                AnimationType::Confetti => {
                    self.render_confetti_animation_in_area(chunks[0], frame);
                }
                AnimationType::GameOfLife => {
                    self.render_gameoflife_animation_in_area(chunks[0], frame);
                }
                AnimationType::Starfield => {
                    self.render_starfield_animation_in_area(chunks[0], frame);
                }
                AnimationType::Rain => {
                    self.render_rain_animation_in_area(chunks[0], frame);
                }
                AnimationType::NyanCat => {
                    self.render_nyancat_animation_in_area(chunks[0], frame);
                }
                AnimationType::DVDLogo => {
                    self.render_dvdlogo_animation_in_area(chunks[0], frame);
                }
                AnimationType::TachyonEffects => {
                    self.render_tachyon_effects_in_area(chunks[0], frame);
                }
            }

            // Render input box and bottom controls
            self.render_input_box(chunks[1], frame.buffer_mut());
            let (plan_area, model_area, _token_area, local_area) =
                self.render_bottom_controls(chunks[2], frame.buffer_mut());

            self.plan_button_area = plan_area;
            self.model_button_area = model_area;
            self.local_button_area = local_area;
            return;
        }

        if self.show_dx_splash {
            // Show DX splash screen
            app_splash::render(
                frame.area(),
                frame.buffer_mut(),
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
            .split(frame.area());

        self.input_area = chunks[1];

        if self.messages.is_empty() {
            app_splash::render(
                chunks[0],
                frame.buffer_mut(),
                &self.theme,
                self.splash_font_index,
                &self.rainbow_animation,
            );
        } else {
            MessageList::with_scroll(&self.messages, &self.theme, self.chat_scroll_offset)
                .render(chunks[0], frame.buffer_mut());
        }

        self.render_input_box(chunks[1], frame.buffer_mut());

        let (plan_area, model_area, _token_area, local_area) =
            self.render_bottom_controls(chunks[2], frame.buffer_mut());

        self.plan_button_area = plan_area;
        self.model_button_area = model_area;
        self.local_button_area = local_area;

        // Update modal animations before rendering
        self.update_modal_animations();
        self.render_modals(frame);

        // Render audio recording indicator in top right
        if self.audio_processing {
            self.render_audio_recording_indicator(frame.area(), frame.buffer_mut());
        }

        if let Some(ref shortcut) = self.last_shortcut_pressed
            && self.last_shortcut_time.elapsed() < Duration::from_secs(2)
        {
            self.render_shortcut_debug(frame.area(), frame.buffer_mut(), shortcut);
        }
    }

    fn render_modals(&mut self, frame: &mut ratatui::Frame) {
        let area = frame.area();

        // Render the modal content first
        if self.show_focus_menu {
            modals::focus::render(area, frame.buffer_mut(), &self.theme, &self.focus_menu_list);
        } else if self.show_add_modal {
            modals::add::render(
                area,
                frame.buffer_mut(),
                &self.theme,
                &self.add_modal_search,
                &self.add_modal_list,
                self.add_modal_focus,
            );
        } else if self.show_plan_modal {
            modals::plan::render(
                area,
                frame.buffer_mut(),
                &self.theme,
                &self.plan_modal_list,
                self.mode,
            );
        } else if self.show_model_modal {
            let config = modals::model::ModelConfig {
                auto_mode: self.auto_mode,
                max_mode: self.max_mode,
                use_multiple_models: self.use_multiple_models,
                selected_model: &self.selected_model,
                selected_models: &self.selected_models,
                google_models: &self.google_models,
            };
            modals::model::render(
                area,
                frame.buffer_mut(),
                &self.theme,
                &self.model_modal_search,
                &self.model_modal_list,
                &config,
            );
        } else if self.show_local_modal {
            modals::local::render(
                area,
                frame.buffer_mut(),
                &self.theme,
                &self.local_modal_list,
                &self.selected_local_mode,
            );
        } else if self.show_changes_modal {
            modals::changes::render(
                area,
                frame.buffer_mut(),
                &self.theme,
                &self.changes_modal_list,
                &self.git_changes,
                self.changes_count,
            );
        } else if self.show_tasks_modal {
            modals::drivens::render(
                area,
                frame.buffer_mut(),
                &self.theme,
                &self.tasks_modal_list,
                &self.tasks,
                self.tasks_count,
            );
        } else if self.show_agents_modal {
            modals::workspaces::render(
                area,
                frame.buffer_mut(),
                &self.agents,
                &self.agents_modal_list,
                &self.theme,
                self.workspace_create_mode,
                &self.workspace_create_input,
            );
        } else if self.show_memory_modal {
            modals::checkpoints::render(
                area,
                frame.buffer_mut(),
                &self.theme,
                &self.memory_modal_list,
                &self.selected_memory_mode,
            );
        } else if self.show_tools_modal {
            modals::tools::render(
                area,
                frame.buffer_mut(),
                &self.theme,
                &self.tools_modal_list,
                &self.tools,
            );
        } else if self.show_more_modal {
            modals::more::render(
                area,
                frame.buffer_mut(),
                &self.theme,
                &self.more_modal_list,
                &self.more_options,
            );
        } else if self.show_google_api_modal {
            integrations::google_api_modal::render(
                area,
                frame.buffer_mut(),
                &self.theme,
                &self.google_api_input,
                self.cursor_visible,
            );
        } else if self.show_elevenlabs_api_modal {
            integrations::elevenlabs_api_modal::render(
                area,
                frame.buffer_mut(),
                &self.theme,
                &self.elevenlabs_api_input,
                self.cursor_visible,
            );
        } else if self.show_effects_demo_modal {
            demos::effects_demo::render(
                area,
                frame.buffer_mut(),
                &self.theme,
                &mut self.effects_demo,
            );

            // Render the effects demo effect
            if self.effects_demo.active_effect.1.running() {
                let modal_width = area.width.saturating_sub(10).min(100);
                let modal_height = area.height.saturating_sub(6).min(30);
                let modal_area = Rect {
                    x: (area.width.saturating_sub(modal_width)) / 2,
                    y: (area.height.saturating_sub(modal_height)) / 2,
                    width: modal_width,
                    height: modal_height,
                };
                let content_area = modal_area.inner(ratatui::layout::Margin::new(2, 1));
                frame.render_effect(
                    &mut self.effects_demo.active_effect.1,
                    content_area,
                    self.effects_demo.last_tick,
                );
            }
        }

        // Apply sweep animation effects after rendering modal content
        let elapsed = self.last_render.elapsed();

        // Render the current modal effect if active
        if let Some(ref mut effect) = self.current_modal_effect {
            if effect.running() {
                if let Some(start_time) = self.modal_effect_start_time {
                    let duration = start_time.elapsed();
                    frame.render_effect(effect, area, duration.into());
                }
            } else {
                // Effect finished, clear it
                self.current_modal_effect = None;
                self.modal_effect_start_time = None;
            }
        }

        self.modal_effect_manager
            .process_effects(elapsed.into(), frame.buffer_mut(), area);
    }
    fn render_shortcut_debug(&self, area: Rect, buf: &mut Buffer, shortcut: &str) {
        let max_len = area.width.saturating_sub(10).max(20) as usize;
        let display_text = if shortcut.len() > max_len {
            format!("{}...", &shortcut[..max_len.saturating_sub(3)])
        } else {
            shortcut.to_string()
        };

        let width = (display_text.len() as u16 + 4).min(area.width.saturating_sub(4));
        let debug_area = Rect {
            x: area.width.saturating_sub(width + 2),
            y: 1,
            width,
            height: 3,
        };

        if debug_area.x >= area.width || debug_area.y >= area.height {
            return;
        }

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.accent))
            .style(Style::default().bg(self.theme.bg));

        let text = Line::from(Span::styled(
            display_text,
            Style::default()
                .fg(self.theme.accent)
                .add_modifier(Modifier::BOLD),
        ));

        Paragraph::new(text)
            .block(block)
            .alignment(ratatui::layout::Alignment::Center)
            .render(debug_area, buf);
    }

    fn render_audio_recording_indicator(&self, area: Rect, buf: &mut Buffer) {
        let indicator_text = "[REC] Recording...";
        let width = indicator_text.len() as u16 + 4;
        let indicator_area = Rect {
            x: area.width.saturating_sub(width + 2),
            y: 1,
            width,
            height: 3,
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.accent))
            .border_type(ratatui::widgets::BorderType::Rounded)
            .style(Style::default().bg(self.theme.bg));

        let shimmer_color = self.shimmer.current_color();
        let text = Line::from(vec![
            Span::styled("[REC] ", Style::default().fg(shimmer_color)),
            Span::styled(
                "Recording...",
                Style::default()
                    .fg(shimmer_color)
                    .add_modifier(Modifier::BOLD),
            ),
        ]);

        Paragraph::new(text)
            .block(block)
            .alignment(ratatui::layout::Alignment::Center)
            .render(indicator_area, buf);
    }

    pub fn render_input_box(&self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.border))
            .border_type(ratatui::widgets::BorderType::Rounded)
            .style(Style::default().bg(self.theme.bg));

        let inner = block.inner(area);
        block.render(area, buf);

        self.render_input_text(inner, buf);
        self.render_input_cursor(inner, buf);
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
                .style(Style::default().bg(self.theme.bg).fg(self.theme.fg))
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
        if self.focus == Focus::Input && self.cursor_visible {
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

    fn render_bottom_controls(&self, area: Rect, buf: &mut Buffer) -> (Rect, Rect, Rect, Rect) {
        let shortcuts = [
            "Ctrl+M: Toggle Menu | Alt+A: Audio",
            "Alt+1/2/3: Switch Modes | Alt+T: Theme",
            "Enter: Send | Shift+Enter: New Line",
            "Ctrl+A/E: Start/End | Ctrl+U/K: Clear",
            "Ctrl+W: Delete Word | Ctrl+D: Exit",
            "Tab: Switch Focus | Arrows: Navigate",
        ];

        let current_shortcut = shortcuts[self.shortcut_index % shortcuts.len()];

        let mode_text = match self.mode {
            ChatMode::Agent => "Agent",
            ChatMode::Plan => "Plan",
            ChatMode::Ask => "Ask",
        };

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

            let color = self.rainbow_animation.color_at(frame_idx);
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

    /// Helper: get a rainbow color as ratatui Color
    fn rainbow_color(&self, index: usize) -> ratatui::style::Color {
        let c = self.rainbow_animation.color_at(index);
        ratatui::style::Color::Rgb(c.r, c.g, c.b)
    }

    /// Helper: get theme accent as ratatui Color
    fn theme_accent_color(&self) -> ratatui::style::Color {
        self.theme.accent
    }

    /// Helper: get theme fg as ratatui Color
    fn theme_fg_color(&self) -> ratatui::style::Color {
        self.theme.fg
    }

    /// Helper: get theme bg as ratatui Color
    fn theme_bg_color(&self) -> ratatui::style::Color {
        self.theme.bg
    }

    fn render_matrix_animation(&self, frame: &mut ratatui::Frame) {
        let area = frame.area();
        self.render_matrix_animation_in_area(area, frame);
    }

    fn render_matrix_animation_in_area(&self, area: Rect, frame: &mut ratatui::Frame) {
        use ratatui::style::{Color, Style};
        use ratatui::text::{Line, Span};
        use ratatui::widgets::Paragraph;

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
            .render(area, frame.buffer_mut());
    }

    fn render_train_animation(&self, frame: &mut ratatui::Frame) {
        let area = frame.area();
        self.render_train_animation_in_area(area, frame);
    }

    fn render_train_animation_in_area(&self, area: Rect, frame: &mut ratatui::Frame) {
        use ratatui::style::Style;
        use ratatui::text::{Line, Span};
        use ratatui::widgets::Paragraph;

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
            if smoke_x_offset >= -(train_width as i32) && smoke_x_offset < area.width as i32 {
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
            if x_pos >= -(train_width as i32) && x_pos < area.width as i32 {
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
            let ch = if (x + track_offset) % 4 == 0 {
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
            .render(area, frame.buffer_mut());
    }

    fn render_animation_placeholder(&self, area: Rect, frame: &mut ratatui::Frame, title: &str) {
        use ratatui::style::{Modifier, Style};
        use ratatui::text::{Line, Span};
        use ratatui::widgets::{Block, Paragraph};

        let mut lines = vec![Line::from("")];

        let mut title_spans = Vec::new();
        for (i, ch) in title.chars().enumerate() {
            let ratatui_color = self.rainbow_color(i);
            title_spans.push(Span::styled(
                ch.to_string(),
                Style::default()
                    .fg(ratatui_color)
                    .add_modifier(Modifier::BOLD),
            ));
        }
        lines.push(Line::from(title_spans));
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "Press Left/Right arrows to switch animations",
            Style::default().fg(self.theme.border),
        )));

        Paragraph::new(lines)
            .alignment(ratatui::layout::Alignment::Center)
            .block(Block::default())
            .render(area, frame.buffer_mut());
    }
}

impl ChatApp {
    fn render_confetti_animation_in_area(&self, area: Rect, frame: &mut ratatui::Frame) {
        use ratatui::style::Style;
        use ratatui::text::{Line, Span};
        use ratatui::widgets::Paragraph;

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
            let explosion_offset = explosion_id as u64 * (explosion_cycle_ms / num_explosions);
            let local_time = (elapsed_ms.wrapping_add(explosion_offset)) % explosion_cycle_ms;
            let age = local_time as f64 / 1000.0; // seconds since this explosion

            // Explosion center - varies per explosion
            let seed = explosion_id as u64;
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
                        let color_idx = (i as usize * 7
                            + explosion_id as usize * 13
                            + (elapsed_ms / 60) as usize)
                            % 50;
                        let c = self.rainbow_animation.color_at(color_idx);
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

        // Render sparkle effects at explosion centers during initial burst
        for explosion_id in 0..num_explosions {
            let explosion_offset = explosion_id as u64 * (explosion_cycle_ms / num_explosions);
            let local_time = (elapsed_ms.wrapping_add(explosion_offset)) % explosion_cycle_ms;

            if local_time < 300 {
                let center_x = match explosion_id {
                    0 => w / 2,
                    1 => w / 4,
                    _ => 3 * w / 4,
                };
                let center_y = match explosion_id {
                    0 => h / 3,
                    1 => h / 2,
                    _ => h / 4,
                };

                // Bright flash at center
                let flash_chars = ['✦', '✧', '★', '☆', '✴', '✵'];
                let flash_radius = (local_time as f64 / 100.0) as i32 + 1;
                for dy in -flash_radius..=flash_radius {
                    for dx in -flash_radius..=flash_radius {
                        let fx = (center_x as i32 + dx) as usize;
                        let fy = (center_y as i32 + dy) as usize;
                        if fx < w && fy < h && (dx * dx + dy * dy) <= flash_radius * flash_radius {
                            let flash_idx = ((dx.unsigned_abs() + dy.unsigned_abs()) as usize
                                + (elapsed_ms / 50) as usize)
                                % flash_chars.len();
                            let brightness = 1.0 - (local_time as f64 / 300.0);
                            let color_idx = (fx + fy + (elapsed_ms / 40) as usize) % 50;
                            let c = self.rainbow_animation.color_at(color_idx);
                            let color = ratatui::style::Color::Rgb(
                                (c.r as f64 * brightness + 255.0 * (1.0 - brightness) * brightness)
                                    as u8,
                                (c.g as f64 * brightness + 255.0 * (1.0 - brightness) * brightness)
                                    as u8,
                                (c.b as f64 * brightness + 255.0 * (1.0 - brightness) * brightness)
                                    as u8,
                            );
                            grid[fy][fx] = Some((flash_chars[flash_idx], color));
                        }
                    }
                }
            }
        }

        let mut lines = Vec::new();
        for y in 0..h {
            let mut spans = Vec::new();
            for x in 0..w {
                if let Some((ch, color)) = grid[y][x] {
                    spans.push(Span::styled(ch.to_string(), Style::default().fg(color)));
                } else {
                    spans.push(Span::raw(" "));
                }
            }
            lines.push(Line::from(spans));
        }

        Paragraph::new(lines)
            .style(Style::default().bg(bg_color))
            .render(area, frame.buffer_mut());
    }

    fn render_gameoflife_animation_in_area(&self, area: Rect, frame: &mut ratatui::Frame) {
        use ratatui::style::Style;
        use ratatui::text::{Line, Span};
        use ratatui::widgets::Paragraph;

        let elapsed = self.rainbow_animation.elapsed();
        let bg_color = self.theme_bg_color();

        let w = area.width as usize;
        let h = area.height as usize;

        if w == 0 || h == 0 {
            return;
        }

        // Use a simple deterministic hash to create evolving patterns
        // Instead of recomputing from seed each frame, we compute generation N
        // using a fast cellular automaton that's computed incrementally
        let generation = ((elapsed * 1000.0) / 150.0) as usize; // ~6.6 fps evolution

        // Initialize grid from a deterministic seed
        let mut grid = vec![vec![false; w]; h];

        // Seed with a pseudo-random but deterministic pattern
        // Use different seed patterns that create interesting evolution
        let seed_gen = generation / 200; // restart every 200 generations for variety
        let local_gen = generation % 200;

        // Create initial pattern based on seed_gen for variety
        let seed_hash = seed_gen.wrapping_mul(2654435761);
        for y in 0..h {
            for x in 0..w {
                let hash = (x.wrapping_mul(374761393) ^ y.wrapping_mul(668265263) ^ seed_hash)
                    .wrapping_mul(2246822519);
                // ~25% fill rate for interesting patterns
                grid[y][x] = (hash % 100) < 25;
            }
        }

        // Place some classic patterns for visual interest
        let place = |grid: &mut Vec<Vec<bool>>, cx: usize, cy: usize, pattern: &[(i32, i32)]| {
            for &(dx, dy) in pattern {
                let x = (cx as i32 + dx).rem_euclid(w as i32) as usize;
                let y = (cy as i32 + dy).rem_euclid(h as i32) as usize;
                grid[y][x] = true;
            }
        };

        // R-pentomino creates chaos
        let r_pentomino = [(0, -1), (1, -1), (-1, 0), (0, 0), (0, 1)];
        place(&mut grid, w / 2, h / 2, &r_pentomino);

        // Evolve for local_gen steps (capped for performance)
        let actual_steps = local_gen.min(60);
        for _ in 0..actual_steps {
            let mut new_grid = vec![vec![false; w]; h];
            for y in 0..h {
                for x in 0..w {
                    let mut neighbors = 0u8;
                    for dy in [h - 1, 0, 1] {
                        for dx in [w - 1, 0, 1] {
                            if dy == 0 && dx == 0 {
                                continue;
                            }
                            if grid[(y + dy) % h][(x + dx) % w] {
                                neighbors += 1;
                            }
                        }
                    }
                    new_grid[y][x] = if grid[y][x] {
                        matches!(neighbors, 2 | 3)
                    } else {
                        neighbors == 3
                    };
                }
            }
            grid = new_grid;
        }

        // Count neighbors for glow effect
        let mut neighbor_count = vec![vec![0u8; w]; h];
        for y in 0..h {
            for x in 0..w {
                let mut count = 0u8;
                for dy in [h - 1, 0, 1] {
                    for dx in [w - 1, 0, 1] {
                        if dy == 0 && dx == 0 {
                            continue;
                        }
                        if grid[(y + dy) % h][(x + dx) % w] {
                            count += 1;
                        }
                    }
                }
                neighbor_count[y][x] = count;
            }
        }

        // Pulsing effect based on elapsed time
        let pulse = ((elapsed * 3.0).sin() * 0.3 + 0.7) as f32;

        let mut lines = Vec::new();
        for y in 0..h {
            let mut spans = Vec::new();
            for x in 0..w {
                if grid[y][x] {
                    // Alive cells get rainbow colors based on position + time
                    let color_idx = (x * 3 + y * 7 + (elapsed * 5.0) as usize) % 50;
                    let c = self.rainbow_animation.color_at(color_idx);
                    let color = ratatui::style::Color::Rgb(
                        (c.r as f32 * pulse) as u8,
                        (c.g as f32 * pulse) as u8,
                        (c.b as f32 * pulse) as u8,
                    );

                    // Different characters based on neighbor count for variety
                    let ch = match neighbor_count[y][x] {
                        0 | 1 => '·',
                        2 => '●',
                        3 => '◉',
                        _ => '★',
                    };
                    spans.push(Span::styled(ch.to_string(), Style::default().fg(color)));
                } else if neighbor_count[y][x] > 0 {
                    // Dead cells near alive ones get a subtle animated glow
                    let glow_intensity = neighbor_count[y][x] as f32;
                    let glow_pulse = ((elapsed * 4.0 + x as f32 * 0.1 + y as f32 * 0.1).sin() * 0.5
                        + 0.5) as f32;
                    let color_idx = (x + y + (elapsed * 2.0) as usize) % 50;
                    let c = self.rainbow_animation.color_at(color_idx);
                    let dim = 0.12 * glow_intensity * glow_pulse;
                    let glow_color = ratatui::style::Color::Rgb(
                        (c.r as f32 * dim) as u8,
                        (c.g as f32 * dim) as u8,
                        (c.b as f32 * dim) as u8,
                    );
                    let glow_ch = if glow_pulse > 0.6 { '·' } else { '.' };
                    spans.push(Span::styled(
                        glow_ch.to_string(),
                        Style::default().fg(glow_color),
                    ));
                } else {
                    spans.push(Span::raw(" "));
                }
            }
            lines.push(Line::from(spans));
        }

        Paragraph::new(lines)
            .style(Style::default().bg(bg_color))
            .render(area, frame.buffer_mut());
    }

    fn render_starfield_animation_in_area(&self, area: Rect, frame: &mut ratatui::Frame) {
        use ratatui::style::Style;
        use ratatui::text::{Line, Span};
        use ratatui::widgets::Paragraph;

        let elapsed = self.rainbow_animation.elapsed();
        let elapsed_ms = (elapsed * 1000.0) as u64;
        let bg_color = self.theme_bg_color();

        let center_x = area.width as f64 / 2.0;
        let center_y = area.height as f64 / 2.0;

        let num_stars = 120;
        let mut star_positions: Vec<(u16, u16, f64, usize)> = Vec::new();

        for i in 0..num_stars {
            let angle = (i as f64 * 2.39996) % (2.0 * std::f64::consts::PI);
            let speed = 0.5 + (i % 5) as f64 * 0.4;
            let birth = (i * 300) % 5000;

            let age = (elapsed_ms.wrapping_sub(birth as u64) % 5000) as f64;
            let dist = age * speed / 100.0;

            let sx = center_x + angle.cos() * dist * 3.0;
            let sy = center_y + angle.sin() * dist;

            if sx >= 0.0 && sx < area.width as f64 && sy >= 0.0 && sy < area.height as f64 {
                let brightness = (dist / 15.0).min(1.0);
                star_positions.push((sx as u16, sy as u16, brightness, i));
            }
        }

        let mut lines = Vec::new();
        for y in 0..area.height {
            let mut spans = Vec::new();
            for x in 0..area.width {
                let mut found = false;
                for &(sx, sy, brightness, idx) in &star_positions {
                    if sx == x && sy == y {
                        let ch = if brightness > 0.7 {
                            '★'
                        } else if brightness > 0.4 {
                            '*'
                        } else {
                            '·'
                        };
                        let color_idx = (idx * 3 + (elapsed * 2.0) as usize) % 50;
                        let c = self.rainbow_animation.color_at(color_idx);
                        let r = (c.r as f32 * brightness as f32) as u8;
                        let g = (c.g as f32 * brightness as f32) as u8;
                        let b = (c.b as f32 * brightness as f32) as u8;
                        spans.push(Span::styled(
                            ch.to_string(),
                            Style::default().fg(ratatui::style::Color::Rgb(r, g, b)),
                        ));
                        found = true;
                        break;
                    }
                }
                if !found {
                    spans.push(Span::raw(" "));
                }
            }
            lines.push(Line::from(spans));
        }

        Paragraph::new(lines)
            .style(Style::default().bg(bg_color))
            .render(area, frame.buffer_mut());
    }

    fn render_rain_animation_in_area(&self, area: Rect, frame: &mut ratatui::Frame) {
        use ratatui::style::Style;
        use ratatui::text::{Line, Span};
        use ratatui::widgets::Paragraph;

        let elapsed = self.rainbow_animation.elapsed();
        let elapsed_ms = (elapsed * 1000.0) as u64;
        let bg_color = self.theme_bg_color();

        let w = area.width as usize;
        let h = area.height as usize;
        let mut screen: Vec<Vec<Option<(char, ratatui::style::Color)>>> = vec![vec![None; w]; h];

        for col in 0..area.width {
            for drop_id in 0..3u64 {
                let seed = col as u64 * 31 + drop_id * 997;
                let speed = 80 + (seed % 60);
                let drop_len = 2 + (seed % 3) as i32;
                let offset = (seed * 13) % (h as u64 * 3);

                let head_y = ((elapsed_ms + offset * speed) / speed) as i32 % (h as i32 * 2)
                    - (h as i32 / 2);

                for t in 0..drop_len {
                    let y = head_y - t;
                    if y >= 0 && y < h as i32 {
                        let brightness = 1.0 - (t as f32 / drop_len as f32) * 0.6;

                        // Rainbow animated colors - each drop gets a color that shifts over time
                        let color_idx = (col as usize * 3
                            + drop_id as usize * 11
                            + (elapsed_ms / 100) as usize)
                            % 50;
                        let c = self.rainbow_animation.color_at(color_idx);
                        let r = (c.r as f32 * brightness) as u8;
                        let g = (c.g as f32 * brightness) as u8;
                        let b = (c.b as f32 * brightness) as u8;
                        let ch = if t == 0 { '|' } else { '│' };
                        screen[y as usize][col as usize] =
                            Some((ch, ratatui::style::Color::Rgb(r, g, b)));
                    }
                }
            }

            // Splash at bottom with rainbow colors
            let splash_seed = col as u64 * 37;
            let splash_time = (elapsed_ms + splash_seed * 50) % 2000;
            if splash_time < 200 && h > 0 {
                let bottom = h - 1;
                let color_idx = (col as usize * 5 + (elapsed_ms / 80) as usize) % 50;
                let c = self.rainbow_animation.color_at(color_idx);
                screen[bottom][col as usize] =
                    Some(('~', ratatui::style::Color::Rgb(c.r, c.g, c.b)));
            }
        }

        let mut lines = Vec::new();
        for row in &screen {
            let mut spans = Vec::new();
            for cell in row {
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
            .render(area, frame.buffer_mut());
    }

    fn render_nyancat_animation_in_area(&self, area: Rect, frame: &mut ratatui::Frame) {
        use ratatui::style::Style;
        use ratatui::text::{Line, Span};
        use ratatui::widgets::Paragraph;

        let elapsed = self.rainbow_animation.elapsed();
        let elapsed_ms = (elapsed * 1000.0) as u64;

        let cat_speed = 80;
        let total_width = area.width as i32 + 40;
        let x_pos = ((elapsed_ms / cat_speed) as i32 % total_width) - 20;

        // Nyan cat ASCII art (just the cat, no rectangle)
        let cat_art = vec![
            r#" /\_/\"#,
            r#"( o.o )"#,
            r#" > ^ < "#,
            r#"/|   |\"#,
            r#"(_|   |)"#,
        ];

        let cat_height = cat_art.len() as u16;
        let cat_width = cat_art.iter().map(|l| l.len()).max().unwrap_or(0) as i32;

        // Rainbow trail colors - animated
        let rainbow_band_count = 6;

        // Vertical bob
        let bob = ((elapsed_ms / 200) % 4) as i16;
        let bob_offset: i16 = match bob {
            0 => 0,
            1 => -1,
            2 => 0,
            3 => 1,
            _ => 0,
        };

        let y_center = (area.height as i16 / 2) - (cat_height as i16 / 2) + bob_offset;

        let mut lines = Vec::new();

        for y in 0..area.height {
            let mut spans = Vec::new();
            let row_y = y as i16;

            let cat_top = y_center;
            let cat_row = row_y - cat_top;

            // Which rainbow band is this row?
            let rainbow_row_offset = row_y - (cat_top - 1);

            for x in 0..area.width {
                let xi = x as i32;

                // Check if we're in the cat area
                if cat_row >= 0
                    && (cat_row as usize) < cat_art.len()
                    && xi >= x_pos
                    && xi < x_pos + cat_width
                {
                    let line = cat_art[cat_row as usize];
                    let char_offset = (xi - x_pos) as usize;
                    let ch = line.chars().nth(char_offset).unwrap_or(' ');
                    if ch != ' ' {
                        // Animated rainbow colors for the cat
                        let color_idx =
                            (char_offset + cat_row as usize * 3 + (elapsed_ms / 100) as usize) % 50;
                        let cat_color = self.rainbow_color(color_idx);
                        spans.push(Span::styled(ch.to_string(), Style::default().fg(cat_color)));
                    } else {
                        // Transparent background - just space
                        spans.push(Span::styled(" ", Style::default()));
                    }
                } else if rainbow_row_offset >= 0
                    && (rainbow_row_offset as usize) < rainbow_band_count
                    && xi < x_pos
                    && xi >= x_pos.saturating_sub(30)
                {
                    // Rainbow trail behind cat - animated
                    let wave = ((xi + elapsed_ms as i32 / 50) % 2) == 0;
                    let trail_ch = if wave { '=' } else { '-' };
                    let trail_color_idx =
                        (rainbow_row_offset as usize * 8 + (elapsed_ms / 100) as usize) % 50;
                    let trail_color = self.rainbow_color(trail_color_idx);
                    spans.push(Span::styled(
                        trail_ch.to_string(),
                        Style::default().fg(trail_color),
                    ));
                } else {
                    // Transparent background with subtle twinkling stars
                    let star_seed = (x as u64 * 31 + y as u64 * 17) % 200;
                    if star_seed < 3 {
                        let twinkle = (elapsed_ms / 300 + x as u64 + y as u64) % 2;
                        let ch = if twinkle == 0 { '.' } else { '*' };
                        let star_color_idx = (x as usize + y as usize) % 50;
                        let c = self.rainbow_animation.color_at(star_color_idx);
                        spans.push(Span::styled(
                            ch.to_string(),
                            Style::default().fg(ratatui::style::Color::Rgb(
                                c.r / 3,
                                c.g / 3,
                                c.b / 3,
                            )),
                        ));
                    } else {
                        spans.push(Span::styled(" ", Style::default()));
                    }
                }
            }
            lines.push(Line::from(spans));
        }

        // Render with transparent background
        Paragraph::new(lines).render(area, frame.buffer_mut());
    }

    fn render_dvdlogo_animation_in_area(&self, area: Rect, frame: &mut ratatui::Frame) {
        use ratatui::style::Style;
        use ratatui::text::{Line, Span};
        use ratatui::widgets::Paragraph;

        let elapsed = self.rainbow_animation.elapsed();
        let elapsed_ms = (elapsed * 1000.0) as u64;
        let bg_color = self.theme_bg_color();

        let logo = vec![
            " DDDD  X   X",
            " D   D  X X ",
            " D   D   X  ",
            " D   D  X X ",
            " DDDD  X   X",
        ];

        let logo_height = logo.len() as i32;
        let logo_width = 13i32;

        let max_x = (area.width as i32 - logo_width).max(1);
        let max_y = (area.height as i32 - logo_height).max(1);

        let speed_x: i32 = 3;
        let speed_y: i32 = 1;
        let tick = (elapsed_ms / 100) as i32;

        let raw_x = tick * speed_x;
        let raw_y = tick * speed_y;

        let cycle_x = max_x * 2;
        let cycle_y = max_y * 2;

        let pos_in_cycle_x = ((raw_x % cycle_x) + cycle_x) % cycle_x;
        let pos_in_cycle_y = ((raw_y % cycle_y) + cycle_y) % cycle_y;

        let x_pos = if pos_in_cycle_x < max_x {
            pos_in_cycle_x
        } else {
            cycle_x - pos_in_cycle_x
        };

        let y_pos = if pos_in_cycle_y < max_y {
            pos_in_cycle_y
        } else {
            cycle_y - pos_in_cycle_y
        };

        // Color changes on bounce - use rainbow colors
        let bounce_count_x = raw_x / max_x.max(1);
        let bounce_count_y = raw_y / max_y.max(1);
        let color_index = ((bounce_count_x + bounce_count_y).unsigned_abs() as usize * 7) % 50;
        let logo_color = self.rainbow_color(color_index);

        let mut lines = Vec::new();

        for row in 0..area.height as i32 {
            let mut spans = Vec::new();

            if row >= y_pos && row < y_pos + logo_height {
                let logo_line = logo[(row - y_pos) as usize];

                for col in 0..area.width as i32 {
                    if col >= x_pos && col < x_pos + logo_width {
                        let char_offset = (col - x_pos) as usize;
                        let ch = logo_line
                            .as_bytes()
                            .get(char_offset)
                            .map(|&b| b as char)
                            .unwrap_or(' ');
                        if ch != ' ' {
                            spans.push(Span::styled(
                                ch.to_string(),
                                Style::default().fg(logo_color),
                            ));
                        } else {
                            spans.push(Span::raw(" "));
                        }
                    } else {
                        spans.push(Span::raw(" "));
                    }
                }
            } else {
                for _ in 0..area.width {
                    spans.push(Span::raw(" "));
                }
            }

            lines.push(Line::from(spans));
        }

        Paragraph::new(lines)
            .style(Style::default().bg(bg_color))
            .render(area, frame.buffer_mut());
    }

    fn render_tachyon_effects_in_area(&mut self, area: Rect, frame: &mut ratatui::Frame) {
        use ratatui::layout::{Constraint, Layout, Margin};
        use ratatui::style::{Color, Modifier, Style};
        use ratatui::text::{Line, Span, Text};
        use ratatui::widgets::{Block, Clear, Paragraph, Widget};
        use tachyonfx::{CenteredShrink, EffectRenderer};

        let screen_bg = Color::Rgb(17, 17, 27);
        let bg = Color::Rgb(30, 30, 46);

        // Initialize effect if not present or if index changed
        if self.tachyon_active_effect.is_none() {
            let effects = self.get_tachyon_effects_repository();
            self.tachyon_active_effect = Some(effects[self.tachyon_active_effect_idx].1.clone());
        }

        // Clear and render background
        Clear.render(area, frame.buffer_mut());
        Block::default()
            .style(Style::default().bg(screen_bg))
            .render(area, frame.buffer_mut());

        let content_area = area.inner_centered(80, 17);
        Block::default()
            .style(Style::default().bg(bg))
            .render(content_area, frame.buffer_mut());

        let layout = Layout::vertical([
            Constraint::Length(2),
            Constraint::Length(7),
            Constraint::Length(6),
        ])
        .split(content_area.inner(Margin::new(1, 1)));

        let anim_style = [
            Style::default().fg(Color::Rgb(249, 226, 175)),
            Style::default().fg(Color::Rgb(137, 180, 250)),
        ];
        let text_style = Style::default().fg(Color::Rgb(205, 214, 244));
        let shortcut_style = [
            Style::default()
                .fg(Color::Rgb(249, 226, 175))
                .add_modifier(Modifier::BOLD),
            Style::default().fg(Color::Rgb(147, 153, 178)),
        ];

        // Get effects repository
        let effects = self.get_tachyon_effects_repository();
        let active_effect_name = effects[self.tachyon_active_effect_idx].0;

        let active_animation = Line::from(vec![
            Span::from("Active animation: ").style(anim_style[0]),
            Span::from(active_effect_name).style(anim_style[1]),
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
            shortcut("→   ", "next transition"),
            shortcut("←   ", "previous transition"),
            shortcut("␣   ", "restart transition"),
            shortcut("r   ", "random transition"),
            shortcut("s   ", "scramble text toggle"),
            shortcut("ESC ", "exit animation mode"),
        ]);

        frame.render_widget(Paragraph::new(active_animation), layout[0]);
        frame.render_widget(Paragraph::new(main_text), layout[1]);
        frame.render_widget(Paragraph::new(shortcuts), layout[2]);

        // Apply effect if we have one
        if let Some(effect) = &mut self.tachyon_active_effect {
            if effect.running() {
                frame.render_effect(effect, content_area, self.tachyon_last_tick);
            }
        }
    }
}
