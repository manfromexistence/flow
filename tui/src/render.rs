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

		// PRIORITY 0: Playing intro/outro transition animations
		if self.playing_intro || self.playing_outro {
			// Split screen: animation area + input + controls (like animation carousel)
			let chunks = Layout::default()
				.direction(Direction::Vertical)
				.constraints([Constraint::Min(10), Constraint::Length(3), Constraint::Length(1)])
				.split(area);

			self.input_area = chunks[1];

			// Clear the animation area first
			for y in chunks[0].top()..chunks[0].bottom() {
				for x in chunks[0].left()..chunks[0].right() {
					buf[(x, y)].reset();
					buf[(x, y)].set_bg(self.theme_bg_color());
				}
			}

			// Render the appropriate transition animation
			let anim_type = if self.playing_intro { self.intro_animation } else { self.outro_animation };

			match anim_type {
				AnimationType::Matrix => {
					self.render_matrix_animation_in_area(chunks[0], buf);
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
				AnimationType::Fire => {
					self.render_fire_animation_in_area(chunks[0], buf);
				}
				AnimationType::Plasma => {
					self.render_plasma_animation_in_area(chunks[0], buf);
				}
				AnimationType::Waves => {
					self.render_waves_animation_in_area(chunks[0], buf);
				}
				AnimationType::Fireworks => {
					self.render_fireworks_animation_in_area(chunks[0], buf);
				}
				_ => {
					// For Splash or Yazi, just show a blank screen during transition
				}
			}

			// Render input box and bottom controls
			self.render_input_box(chunks[1], buf);
			let (plan_area, model_area, _token_area, local_area) =
				self.render_bottom_controls(chunks[2], buf);
			self.plan_button_area = plan_area;
			self.model_button_area = model_area;
			self.local_button_area = local_area;

			// Render toast notification (on top of everything)
			self.render_toast(area, buf);
			return;
		}

		// Both animations show in full screen, no input or controls
		if self.show_train_animation || self.show_matrix_animation {
			// Clear the entire area first
			for y in area.top()..area.bottom() {
				for x in area.left()..area.right() {
					buf[(x, y)].reset();
					buf[(x, y)].set_bg(self.theme_bg_color());
				}
			}

			// Render appropriate animation in the full area
			if self.show_train_animation {
				self.render_train_animation_in_area(area, buf);
			} else if self.show_matrix_animation {
				self.render_matrix_animation_in_area(area, buf);
			}
			return;
		}

		// Animation carousel mode
		if self.animation_mode {
			let animations = AnimationType::all();
			let current_anim = animations[self.current_animation_index];

			// Special handling for Yazi screen - don't render here, let root handle it
			if current_anim == AnimationType::Yazi {
				// Just render input and controls, yazi will be rendered by root widget
				let chunks = Layout::default()
					.direction(Direction::Vertical)
					.constraints([Constraint::Min(10), Constraint::Length(3), Constraint::Length(1)])
					.split(area);

				self.input_area = chunks[1];
				self.render_input_box(chunks[1], buf);
				let (plan_area, model_area, _token_area, local_area) =
					self.render_bottom_controls(chunks[2], buf);
				self.plan_button_area = plan_area;
				self.model_button_area = model_area;
				self.local_button_area = local_area;
				return;
			}

			// Matrix animation - show with input and controls
			if current_anim == AnimationType::Matrix {
				let chunks = Layout::default()
					.direction(Direction::Vertical)
					.constraints([Constraint::Min(10), Constraint::Length(3), Constraint::Length(1)])
					.split(area);

				self.input_area = chunks[1];

				// Clear the animation area first
				for y in chunks[0].top()..chunks[0].bottom() {
					for x in chunks[0].left()..chunks[0].right() {
						buf[(x, y)].reset();
						buf[(x, y)].set_bg(self.theme_bg_color());
					}
				}

				// Render animation in the main area
				self.render_matrix_animation_in_area(chunks[0], buf);

				// Render intro/outro indicators (top-left corner)
				self.render_animation_indicators(chunks[0], current_anim, buf);

				// Render input box and bottom controls
				self.render_input_box(chunks[1], buf);
				let (plan_area, model_area, _token_area, local_area) =
					self.render_bottom_controls(chunks[2], buf);
				self.plan_button_area = plan_area;
				self.model_button_area = model_area;
				self.local_button_area = local_area;

				// Render menu overlay if visible
				if self.show_tachyon_menu || self.menu_is_closing {
					self.render_menu_in_area(area, buf);
				}

				// Render toast notification (on top of everything)
				self.render_toast(area, buf);
				return;
			}

			let chunks = Layout::default()
				.direction(Direction::Vertical)
				.constraints([Constraint::Min(10), Constraint::Length(3), Constraint::Length(1)])
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
					// Already handled above
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
				AnimationType::Fire => {
					self.render_fire_animation_in_area(chunks[0], buf);
				}
				AnimationType::Plasma => {
					self.render_plasma_animation_in_area(chunks[0], buf);
				}
				// AnimationType::Spinners => {
				// 	self.render_spinners_animation_in_area(chunks[0], buf);
				// } // COMMENTED OUT: Temporary screen removed
				AnimationType::Waves => {
					self.render_waves_animation_in_area(chunks[0], buf);
				}
				AnimationType::Fireworks => {
					self.render_fireworks_animation_in_area(chunks[0], buf);
				}
				AnimationType::Yazi => {
					// Exit animation mode and show yazi file picker
					// This will be handled by the root widget
					return;
				}
			}

			// Render intro/outro indicators (top-left corner)
			self.render_animation_indicators(chunks[0], current_anim, buf);

			// Render input box and bottom controls
			self.render_input_box(chunks[1], buf);

			let (plan_area, model_area, _token_area, local_area) =
				self.render_bottom_controls(chunks[2], buf);

			self.plan_button_area = plan_area;
			self.model_button_area = model_area;
			self.local_button_area = local_area;

			// Render menu overlay if visible (on top of animations)
			if self.show_tachyon_menu || self.menu_is_closing {
				self.render_menu_in_area(area, buf);
			}

			// Render toast notification (on top of everything)
			self.render_toast(area, buf);
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
			.constraints([Constraint::Min(10), Constraint::Length(3), Constraint::Length(1)])
			.split(area);

		self.input_area = chunks[1];

		// Show splash when no messages, otherwise show message list
		if self.messages.is_empty() {
			super::splash::render(
				chunks[0],
				buf,
				&self.theme,
				self.splash_font_index,
				&self.rainbow_animation,
			);
		} else {
			MessageList::with_effects(
				&self.messages,
				&self.theme,
				self.chat_scroll_offset,
				&self.shimmer,
				&self.typing_indicator,
			)
			.render(chunks[0], buf);
		}

		self.render_input_box(chunks[1], buf);

		let (plan_area, model_area, _token_area, local_area) =
			self.render_bottom_controls(chunks[2], buf);

		self.plan_button_area = plan_area;
		self.model_button_area = model_area;
		self.local_button_area = local_area;

		// Render performance overlay if enabled
		self.render_perf_overlay(area, buf);

		// Render menu overlay globally if visible (on top of everything)
		if self.show_tachyon_menu || self.menu_is_closing {
			self.render_menu_in_area(area, buf);
		}

		// Render toast notification (on top of everything)
		self.render_toast(area, buf);
	}

	pub fn render_dimmed(&mut self, area: Rect, full_area: Rect, buf: &mut Buffer) {
		// Simplified render for FilePicker mode - just show input box and controls
		// Split into input (3 lines) and controls (1 line)
		let chunks = Layout::default()
			.direction(Direction::Vertical)
			.constraints([
				Constraint::Length(3), // Input box (3 lines)
				Constraint::Length(1), // Bottom controls (1 line)
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

		// Render menu overlay globally if visible (on top of everything)
		// Use full_area to center menu in the entire terminal, not just the chat area
		if self.show_tachyon_menu || self.menu_is_closing {
			self.render_menu_in_area(full_area, buf);
		}

		// Render toast notification (on top of everything)
		self.render_toast(full_area, buf);
	}
}

// Input rendering methods
use ratatui::{
	style::{Modifier, Style},
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

		// Render spinner on the right side if space is held
		if self.space_held {
			self.render_input_spinner(padded_inner, buf);
		}

		// Record input render time
		self.last_input_render_time = self.perf_monitor.record_input_render();
	}

	fn render_input_text(&self, area: Rect, buf: &mut Buffer) {
		let placeholder = "A question or a prompt... (Enter to send, Shift+Enter for new line)";
		let text = if self.input.content.is_empty() {
			Text::from(Line::from(Span::styled(placeholder, Style::default().fg(self.theme.border))))
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
		let (sel_start, sel_end) =
			if let (Some(start), Some(end)) = (self.input.selection_start, self.input.selection_end) {
				if start < end { (start, end) } else { (end, start) }
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
			// Check if cursor revert animation is active
			if self.cursor_revert_animation {
				if let Some(start_time) = self.cursor_revert_start {
					let elapsed = start_time.elapsed().as_millis() as f32;
					let animation_duration = 300.0; // 300ms animation
					
					if elapsed < animation_duration {
						// Calculate interpolation progress (0.0 to 1.0)
						let progress = elapsed / animation_duration;
						// Use ease-out cubic for smooth deceleration
						let eased_progress = 1.0 - (1.0 - progress).powi(3);
						
						// Interpolate between old position and new position
						let from_pos = self.cursor_revert_from_pos as f32;
						let to_pos = self.input.cursor_position as f32;
						let animated_pos = from_pos + (to_pos - from_pos) * eased_progress;
						
						// Render animated cursor with trail effect
						let cursor_x = area.x + (animated_pos as u16 % area.width);
						let cursor_y = area.y + (animated_pos as u16 / area.width);
						
						if cursor_x < area.right() && cursor_y < area.bottom() {
							let cell = &mut buf[(cursor_x, cursor_y)];
							let rainbow_color = self.rainbow_cursor.current_color();
							
							// Pulsing effect during animation
							let pulse_char = if (elapsed as u32 / 50) % 2 == 0 { '◆' } else { '◇' };
							cell.set_char(pulse_char);
							cell.set_style(Style::default().fg(rainbow_color));
						}
						
						return;
					}
				}
			}
			
			// Normal cursor rendering
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

	fn render_input_spinner(&self, area: Rect, buf: &mut Buffer) {
		// Block spinner frames
		let spinner_frames = ['▁', '▃', '▄', '▅', '▆', '▇', '█', '▇', '▆', '▅', '▄', '▃'];
		let frame_char = spinner_frames[self.spinner_frame % spinner_frames.len()];

		// Position spinner on the far right inside the input box
		// area is the inner area (already inside the border)
		let spinner_x = area.right().saturating_sub(1); // 1 char from right edge
		let spinner_y = area.y + (area.height / 2); // Vertically centered

		if spinner_x < area.right() && spinner_y < area.bottom() {
			let cell = &mut buf[(spinner_x, spinner_y)];

			// Use rainbow color for the spinner
			let color = self.rainbow_animation.current_color();

			cell.set_char(frame_char);
			cell.set_style(Style::default().fg(color).add_modifier(Modifier::BOLD));
		}
	}

	pub fn render_bottom_controls(&self, area: Rect, buf: &mut Buffer) -> (Rect, Rect, Rect, Rect) {
		// Context-aware shortcuts based on current screen
		let shortcuts = if self.animation_mode {
			let animations = crate::AnimationType::all();
			let current_anim = animations[self.current_animation_index];

			if current_anim == crate::AnimationType::Yazi {
				// File Browser tips
				[
					"Left/Right Arrow: Return to Splash | Navigate files with arrows",
					"Enter: Select file | Tab: Switch panes | /: Search",
					"Space: Select multiple | d: Delete | r: Rename | y: Copy",
				]
			} else if current_anim == crate::AnimationType::Splash {
				// Splash screen tips
				[
					"Right Arrow: File Browser | Left Arrow: Animation Carousel",
					"Type a message and press Enter to start chatting",
					"0/Ctrl+P: Command Palette | Ctrl+T: Theme | Ctrl+C: Exit",
				]
			} else {
				// Animation Carousel tips
				[
					"Up Arrow: Set as INTRO animation | Down Arrow: Set as OUTRO animation",
					"Left Arrow: Previous animation | Right Arrow: Return to Splash",
					"Intro plays when entering chat | Outro plays when exiting (Ctrl+C)",
				]
			}
		} else {
			// Normal chat mode tips (rotating)
			[
				"0/Ctrl+P: Toggle Command Palette | Space(Hold): Voice Input",
				"Left/Right Arrow: Explore Screens | Ctrl+C: Exit to Splash",
				"1/2/3/(Numbers): Toggle Menus | Ctrl+T: Theme",
			]
		};

		let current_shortcut = if self.animation_mode {
			// In animation mode, show all tips without rotation
			let animations = crate::AnimationType::all();
			let current_anim = animations[self.current_animation_index];
			if current_anim == crate::AnimationType::Yazi || current_anim == crate::AnimationType::Splash
			{
				// Show first tip for Yazi and Splash
				shortcuts[0]
			} else {
				// Rotate tips for animation carousel
				shortcuts[self.shortcut_index % shortcuts.len()]
			}
		} else {
			// Normal rotation for chat mode
			shortcuts[self.shortcut_index % shortcuts.len()]
		};

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
		let token_info =
			format!("{:.1}K/{}K({}%)", total_tokens as f32 / 1000.0, context_limit / 1000, token_ratio);
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

		let bottom_chunks =
			Layout::default().direction(Direction::Horizontal).constraints(constraints).split(area);

		Paragraph::new(Span::styled(&self.selected_local_mode, Style::default().fg(self.theme.fg)))
			.alignment(ratatui::layout::Alignment::Left)
			.render(bottom_chunks[0], buf);

		Paragraph::new(Span::styled(mode_text, Style::default().fg(self.theme.fg)))
			.alignment(ratatui::layout::Alignment::Left)
			.render(bottom_chunks[2], buf);

		Paragraph::new(Span::styled(&self.selected_model, Style::default().fg(self.theme.fg)))
			.alignment(ratatui::layout::Alignment::Left)
			.render(bottom_chunks[4], buf);

		Paragraph::new(Span::styled(current_shortcut, Style::default().fg(self.theme.border)))
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

		Paragraph::new(Span::styled(&truncated_path, Style::default().fg(self.theme.fg)))
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
				Style::default().fg(ratatui_color).add_modifier(Modifier::BOLD),
			))
			.alignment(ratatui::layout::Alignment::Left)
			.render(bottom_chunks[10], buf);
		}

		(bottom_chunks[2], bottom_chunks[4], bottom_chunks[6], bottom_chunks[0])
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
		let lines = vec![
			Line::from(vec![
				Span::styled("⚡ ", Style::default().fg(ratatui::style::Color::Yellow)),
				Span::styled(
					"Performance Monitor",
					Style::default().fg(ratatui::style::Color::Cyan).add_modifier(Modifier::BOLD),
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
					if self.perf_monitor.is_meeting_targets() { "✓ EXCELLENT" } else { "○ GOOD" },
					Style::default().fg(status_color).add_modifier(Modifier::BOLD),
				),
			]),
		];

		let block = Block::default()
			.borders(Borders::ALL)
			.border_style(Style::default().fg(status_color))
			.border_type(ratatui::widgets::BorderType::Rounded)
			.style(Style::default().bg(ratatui::style::Color::Black));

		let paragraph = Paragraph::new(lines).block(block).style(Style::default().fg(self.theme.fg));

		paragraph.render(overlay_area, buf);
	}
}

impl ChatState {
	/// Render toast notification in top-right corner
	pub fn render_toast(&self, area: Rect, buf: &mut Buffer) {
		if let Some(ref message) = self.toast_message {
			// Toast dimensions
			let toast_width = (message.len() as u16 + 4).min(area.width);
			let toast_height = 3;

			// Position in top-right corner
			let toast_x = area.width.saturating_sub(toast_width);
			let toast_y = 0;

			let toast_area = Rect { x: toast_x, y: toast_y, width: toast_width, height: toast_height };

			// Create toast with border
			let block = Block::default()
				.borders(Borders::ALL)
				.border_style(Style::default().fg(self.theme.accent))
				.border_type(ratatui::widgets::BorderType::Rounded)
				.style(Style::default().bg(self.theme.bg));

			let inner = block.inner(toast_area);
			block.render(toast_area, buf);

			// Render message text
			let text = Paragraph::new(message.as_str())
				.style(Style::default().fg(self.theme.fg))
				.alignment(ratatui::layout::Alignment::Center);

			text.render(inner, buf);
		}
	}

	/// Render intro/outro indicators in top-left corner (for carousel screens)
	pub fn render_animation_indicators(
		&self,
		area: Rect,
		current_anim: AnimationType,
		buf: &mut Buffer,
	) {
		// Only show on carousel animations (not Splash or Yazi)
		if current_anim == AnimationType::Splash || current_anim == AnimationType::Yazi {
			return;
		}

		let mut lines = Vec::new();

		// Show intro indicator
		if self.intro_animation == current_anim {
			lines.push(Line::from(vec![
				Span::styled("▲ ", Style::default().fg(self.theme.accent)),
				Span::styled("INTRO", Style::default().fg(self.theme.fg)),
			]));
		}

		// Show outro indicator
		if self.outro_animation == current_anim {
			lines.push(Line::from(vec![
				Span::styled("▼ ", Style::default().fg(self.theme.accent)),
				Span::styled("OUTRO", Style::default().fg(self.theme.fg)),
			]));
		}

		if lines.is_empty() {
			return;
		}

		// Calculate dimensions
		let indicator_height = lines.len() as u16 + 2; // +2 for border
		let indicator_width = 12; // Fixed width for "▼ OUTRO" + padding

		let indicator_area = Rect { x: 0, y: 0, width: indicator_width, height: indicator_height };

		// Create indicator box with border
		let block = Block::default()
			.borders(Borders::ALL)
			.border_style(Style::default().fg(self.theme.accent))
			.border_type(ratatui::widgets::BorderType::Rounded)
			.style(Style::default().bg(self.theme.bg));

		let inner = block.inner(indicator_area);
		block.render(indicator_area, buf);

		// Render indicator text
		let text = Paragraph::new(lines).style(Style::default().fg(self.theme.fg));

		text.render(inner, buf);
	}
}
