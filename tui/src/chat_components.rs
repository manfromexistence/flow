use ratatui::{
	buffer::Buffer,
	layout::{Alignment, Rect},
	style::{Modifier, Style},
	text::{Line, Span, Text},
	widgets::{
		Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, StatefulWidget,
		Widget, Wrap,
	},
};
use tiktoken_rs::cl100k_base;

#[allow(unused_imports)]
use super::{
	effects::{ShimmerEffect, TypingIndicator},
	input::InputState,
	theme::ChatTheme,
};

// Helper function to count tokens accurately
#[allow(dead_code)]
fn count_tokens(text: &str) -> usize {
	match cl100k_base() {
		Ok(bpe) => bpe.encode_with_special_tokens(text).len(),
		Err(_) => text.len() / 4, // Fallback to rough estimate
	}
}

// Simple markdown parser - just returns plain text lines for now
#[allow(dead_code)]
fn parse_markdown_to_lines<'a>(content: &'a str, _theme: &'a ChatTheme) -> Vec<Line<'a>> {
	content.lines().map(Line::from).collect()
}

// Parse content and extract thinking sections
fn parse_content_with_thinking<'a>(
	content: &'a str,
	theme: &'a ChatTheme,
	show_thinking: bool,
) -> Vec<Line<'a>> {
	let mut lines = Vec::new();
	let mut in_thinking = false;
	let mut thinking_content = Vec::new();
	let mut has_thinking = false;
	let mut just_closed_thinking = false;
	let mut response_content = Vec::new();

	for line in content.lines() {
		let trimmed = line.trim();
		if trimmed == "<think>" {
			in_thinking = true;
			has_thinking = true;
			continue;
		} else if trimmed == "</think>" {
			in_thinking = false;
			just_closed_thinking = true;
			continue;
		}

		if in_thinking {
			thinking_content.push(line);
		} else {
			// Skip the first empty line after closing thinking tag
			if just_closed_thinking && line.trim().is_empty() {
				just_closed_thinking = false;
				continue;
			}
			just_closed_thinking = false;
			response_content.push(line);
		}
	}

	// If we're still in thinking (streaming), show the accordion with current content
	if in_thinking && has_thinking {
		if show_thinking {
			lines.push(Line::from(vec![
				Span::styled("▼ ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
				Span::styled("Thinking", Style::default().fg(theme.accent).add_modifier(Modifier::ITALIC)),
			]));
			// Add thinking content being streamed (with indentation)
			for think_line in &thinking_content {
				lines.push(Line::from(Span::styled(
					format!("  {}", think_line),
					Style::default().fg(theme.border).add_modifier(Modifier::ITALIC),
				)));
			}
		} else {
			lines.push(Line::from(vec![
				Span::styled("▶ ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
				Span::styled("Thinking", Style::default().fg(theme.accent).add_modifier(Modifier::ITALIC)),
			]));
		}
		return lines;
	}

	// If thinking is complete (has </think>), show accordion followed by response
	if has_thinking {
		// Add thinking accordion header
		if show_thinking {
			lines.push(Line::from(vec![
				Span::styled("▼ ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
				Span::styled("Thinking", Style::default().fg(theme.accent).add_modifier(Modifier::ITALIC)),
			]));
			// Add thinking content (with indentation) - show all lines including empty ones
			for think_line in &thinking_content {
				lines.push(Line::from(Span::styled(
					format!("  {}", think_line),
					Style::default().fg(theme.border).add_modifier(Modifier::ITALIC),
				)));
			}
		} else {
			lines.push(Line::from(vec![
				Span::styled("▶ ", Style::default().fg(theme.accent).add_modifier(Modifier::BOLD)),
				Span::styled("Thinking", Style::default().fg(theme.accent).add_modifier(Modifier::ITALIC)),
			]));
		}

		// Add response content immediately after (no gap)
		for response_line in &response_content {
			lines.push(Line::from(*response_line));
		}

		return lines;
	}

	// If no thinking tags were found, just return the plain lines
	content.lines().map(Line::from).collect()
}

#[derive(Debug, Clone)]
pub struct Message {
	pub role: MessageRole,
	pub content: String,
	pub timestamp: chrono::DateTime<chrono::Local>,
	pub token_count: usize,
	pub thinking_expanded: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum MessageRole {
	User,
	Assistant,
}

impl Message {
	#[allow(dead_code)]
	pub fn user(content: String) -> Self {
		let token_count = count_tokens(&content);
		Self {
			role: MessageRole::User,
			content,
			timestamp: chrono::Local::now(),
			token_count,
			thinking_expanded: false,
		}
	}

	#[allow(dead_code)]
	pub fn assistant(content: String) -> Self {
		let token_count = count_tokens(&content);
		Self {
			role: MessageRole::Assistant,
			content,
			timestamp: chrono::Local::now(),
			token_count,
			thinking_expanded: false, // Start collapsed, will expand when <think> is detected
		}
	}
}

pub struct MessageList<'a> {
	messages: &'a [Message],
	theme: &'a ChatTheme,
	scroll_offset: usize,
	shimmer: Option<&'a ShimmerEffect>,
	typing_indicator: Option<&'a TypingIndicator>,
	#[allow(dead_code)]
	selected_message_index: Option<usize>,
}

impl<'a> MessageList<'a> {
	#[allow(dead_code)]
	pub fn new(messages: &'a [Message], theme: &'a ChatTheme) -> Self {
		Self {
			messages,
			theme,
			scroll_offset: 0,
			shimmer: None,
			typing_indicator: None,
			selected_message_index: None,
		}
	}

	#[allow(dead_code)]
	pub fn with_scroll(messages: &'a [Message], theme: &'a ChatTheme, scroll_offset: usize) -> Self {
		Self {
			messages,
			theme,
			scroll_offset,
			shimmer: None,
			typing_indicator: None,
			selected_message_index: None,
		}
	}

	#[allow(dead_code)]
	pub fn with_effects(
		messages: &'a [Message],
		theme: &'a ChatTheme,
		scroll_offset: usize,
		shimmer: &'a ShimmerEffect,
		typing_indicator: &'a TypingIndicator,
	) -> Self {
		Self {
			messages,
			theme,
			scroll_offset,
			shimmer: Some(shimmer),
			typing_indicator: Some(typing_indicator),
			selected_message_index: None,
		}
	}

	/// Calculate total height of all messages based on actual rendered content
	pub fn calculate_total_height(&self) -> usize {
		self
			.messages
			.iter()
			.map(|msg| {
				let content_lines = if msg.content.is_empty() {
					1 // Shimmer "Thinking..." line
				} else if msg.role == MessageRole::Assistant {
					// For assistant messages, parse with thinking accordion to get actual line count
					let parsed_lines =
						parse_content_with_thinking(&msg.content, self.theme, msg.thinking_expanded);
					parsed_lines.len()
				} else {
					// For user messages, just count lines
					msg.content.lines().count()
				};

				match msg.role {
					MessageRole::User => {
						// User message: content + header + borders + gap
						content_lines + 3 + 1
					}
					MessageRole::Assistant => {
						// Assistant message: content + header + gap (no borders)
						content_lines + 1 + 1
					}
				}
			})
			.sum()
	}
}

impl Widget for MessageList<'_> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let mut y = area.y;
		let mut skipped_lines = 0usize;

		for msg in self.messages.iter() {
			if y >= area.bottom() {
				break;
			}

			match msg.role {
				MessageRole::User => {
					// User message: minimal padding, right-aligned, rounded border
					let token_text = format!("({} tokens)", msg.token_count);
					let header = Line::from(vec![
						Span::styled(
							"User",
							Style::default().fg(self.theme.accent).add_modifier(Modifier::BOLD),
						),
						Span::raw(""),
						Span::styled(token_text, Style::default().fg(self.theme.border)),
					]);

					let content_lines: Vec<Line> =
						msg.content.lines().map(|line| Line::from(Span::raw(line))).collect();

					// Calculate dynamic width based on content
					let max_content_width = content_lines
						.iter()
						.map(|line| line.spans.iter().map(|span| span.content.len()).sum::<usize>())
						.max()
						.unwrap_or(0);

					// Width: fit content tightly with minimal padding
					// Add only 4 for borders (2) + minimal spacing (2)
					// Minimum width to show "User X tokens" properly
					let header_width = format!("User({} tokens)", msg.token_count).len();
					let needed_width = max_content_width.max(header_width) + 4;
					let max_width = (area.width * 60 / 100) as usize;
					let msg_width = (needed_width.min(max_width).max(12)) as u16;
					let msg_x = area.x + area.width.saturating_sub(msg_width);

					// Minimal padding: 2 for borders + 1 for header + content lines
					let msg_height = (content_lines.len() + 3).min((area.bottom() - y) as usize);

					// Handle scrolling
					if skipped_lines < self.scroll_offset {
						let skip_amount = (msg_height + 1).min(self.scroll_offset - skipped_lines);
						skipped_lines += skip_amount;
						continue;
					}

					let msg_area = Rect { x: msg_x, y, width: msg_width, height: msg_height as u16 };

					let block = Block::default()
						.borders(Borders::ALL)
						.border_type(ratatui::widgets::BorderType::Rounded)
						.border_style(Style::default().fg(self.theme.border))
						.style(Style::default().bg(self.theme.bg).fg(self.theme.fg));

					let inner = block.inner(msg_area);
					block.render(msg_area, buf);

					// Minimal padding: just 1 space on each side
					let padded = Rect {
						x: inner.x + 1,
						y: inner.y,
						width: inner.width.saturating_sub(2),
						height: inner.height,
					};

					// Render header right-aligned
					Paragraph::new(header)
						.style(Style::default().bg(self.theme.bg))
						.alignment(ratatui::layout::Alignment::Right)
						.render(Rect { x: padded.x, y: padded.y, width: padded.width, height: 1 }, buf);

					// Render content right-aligned
					if padded.height > 1 {
						let content_area =
							Rect { x: padded.x, y: padded.y + 1, width: padded.width, height: padded.height - 1 };

						Paragraph::new(content_lines)
							.style(Style::default().bg(self.theme.bg).fg(self.theme.fg))
							.alignment(ratatui::layout::Alignment::Right)
							.wrap(Wrap { trim: false })
							.render(content_area, buf);
					}

					y += msg_height as u16 + 1; // Add 1 line gap
				}
				MessageRole::Assistant => {
					// AI message: full width, no border, markdown rendered
					let time = msg.timestamp.format("%I:%M %p").to_string();
					let header = Line::from(vec![Span::styled(time, Style::default().fg(self.theme.border))]);

					// Check if content is empty and show shimmer effect
					let content_lines = if msg.content.is_empty() {
						// Show shimmer loading indicator when content is empty
						if let (Some(shimmer), Some(indicator)) = (self.shimmer, self.typing_indicator) {
							let text = format!("Thinking{}", indicator.text(indicator.is_visible()));
							let mut spans = Vec::new();

							// Apply shimmer effect to each character
							for (i, ch) in text.chars().enumerate() {
								let position = i as f32 / text.len().max(1) as f32;
								let shimmer_color = shimmer.shimmer_color_at(position);
								spans.push(Span::styled(
									ch.to_string(),
									Style::default().fg(shimmer_color).add_modifier(Modifier::ITALIC),
								));
							}

							vec![Line::from(spans)]
						} else {
							vec![Line::from("Thinking...")]
						}
					} else {
						// Parse content with thinking accordion using message's expansion state
						parse_content_with_thinking(&msg.content, self.theme, msg.thinking_expanded)
					};

					// Calculate message height based on content
					let msg_height = (content_lines.len() + 2).min((area.bottom() - y) as usize);

					// Handle scrolling
					if skipped_lines < self.scroll_offset {
						let skip_amount = (msg_height + 1).min(self.scroll_offset - skipped_lines);
						skipped_lines += skip_amount;
						continue;
					}

					let msg_area = Rect { x: area.x, y, width: area.width, height: msg_height as u16 };

					// Render header
					Paragraph::new(header)
						.style(Style::default().bg(self.theme.bg))
						.render(Rect { x: msg_area.x, y: msg_area.y, width: msg_area.width, height: 1 }, buf);

					// Render content
					if msg_height > 1 {
						let content_area = Rect {
							x: msg_area.x,
							y: msg_area.y + 1,
							width: msg_area.width,
							height: (msg_height - 1) as u16,
						};

						// Create Text from lines for proper rendering
						let text = Text::from(content_lines);
						Paragraph::new(text)
							.style(Style::default().bg(self.theme.bg).fg(self.theme.fg))
							.wrap(Wrap { trim: false })
							.render(content_area, buf);
					}

					y += msg_height as u16 + 1; // Add 1 line gap
				}
			}
		}

		// Render scrollbar
		let total_height = self.calculate_total_height();
		let viewport_height = area.height as usize;

		if total_height > viewport_height {
			// content_length should be the total scrollable content
			// position should be the current scroll offset
			// viewport_content_length should be how much is visible
			let mut scrollbar_state = ScrollbarState::new(total_height.saturating_sub(viewport_height))
				.position(self.scroll_offset)
				.viewport_content_length(viewport_height);

			let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
				.style(Style::default().fg(self.theme.border))
				.begin_symbol(Some("↑"))
				.end_symbol(Some("↓"));

			scrollbar.render(area, buf, &mut scrollbar_state);
		}
	}
}

#[allow(dead_code)]
pub struct InputBox<'a> {
	content: &'a str,
	cursor_pos: usize,
	theme: &'a ChatTheme,
	focused: bool,
	placeholder: &'a str,
}

#[allow(dead_code)]
impl<'a> InputBox<'a> {
	pub fn new(content: &'a str, cursor_pos: usize, theme: &'a ChatTheme, focused: bool) -> Self {
		Self {
			content,
			cursor_pos,
			theme,
			focused,
			placeholder: "Type a prompt... (Enter to send, Shift+Enter for new line)",
		}
	}
}

impl Widget for InputBox<'_> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let block = Block::default()
			.borders(Borders::TOP)
			.border_style(Style::default().fg(self.theme.border))
			.style(Style::default().bg(self.theme.bg).fg(self.theme.fg));

		let inner = block.inner(area);
		block.render(area, buf);

		let text = if self.content.is_empty() {
			Text::from(Line::from(Span::styled(self.placeholder, Style::default().fg(self.theme.border))))
		} else {
			Text::from(self.content)
		};

		Paragraph::new(text).wrap(Wrap { trim: false }).render(inner, buf);

		// Render cursor when focused
		if self.focused {
			let cursor_x = inner.x + (self.cursor_pos as u16 % inner.width);
			let cursor_y = inner.y + (self.cursor_pos as u16 / inner.width);

			if cursor_x < inner.right() && cursor_y < inner.bottom() {
				let cell = &mut buf[(cursor_x, cursor_y)];
				// Show cursor as inverted colors
				if self.content.is_empty() {
					// Show cursor at start when empty
					cell.set_style(Style::default().bg(self.theme.fg).fg(self.theme.bg));
				} else {
					cell.set_style(Style::default().bg(self.theme.accent).fg(self.theme.bg));
				}
			}
		}
	}
}

#[allow(dead_code)]
pub struct LoadingIndicator<'a> {
	indicator: &'a TypingIndicator,
	shimmer: &'a ShimmerEffect,
	theme: &'a ChatTheme,
}

#[allow(dead_code)]
impl<'a> LoadingIndicator<'a> {
	pub fn new(
		indicator: &'a TypingIndicator,
		shimmer: &'a ShimmerEffect,
		theme: &'a ChatTheme,
	) -> Self {
		Self { indicator, shimmer, theme }
	}
}

impl Widget for LoadingIndicator<'_> {
	fn render(self, area: Rect, buf: &mut Buffer) {
		let shimmer_color = self.shimmer.current_color();

		// AI loading: just show the animated dots without "Assistant" text
		let text = Line::from(vec![Span::styled(
			self.indicator.text(self.indicator.is_visible()),
			Style::default().fg(shimmer_color).add_modifier(Modifier::ITALIC),
		)]);

		Paragraph::new(text)
			.style(Style::default().bg(self.theme.bg))
			.alignment(Alignment::Left)
			.render(area, buf);
	}
}
