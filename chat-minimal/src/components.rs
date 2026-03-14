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
fn count_tokens(text: &str) -> usize {
    match cl100k_base() {
        Ok(bpe) => bpe.encode_with_special_tokens(text).len(),
        Err(_) => text.len() / 4, // Fallback to rough estimate
    }
}

// Simple markdown parser - just returns plain text lines for now
#[allow(dead_code)]
fn parse_markdown_to_lines<'a>(content: &'a str, _theme: &'a ChatTheme) -> Vec<Line<'a>> {
    content.lines().map(|line| Line::from(line)).collect()
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
        if line.trim() == "<think>" {
            in_thinking = true;
            has_thinking = true;
            continue;
        } else if line.trim() == "</think>" {
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
                Span::styled(
                    "▼ ",
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "Thinking",
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::ITALIC),
                ),
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
                Span::styled(
                    "▶ ",
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "Thinking",
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::ITALIC),
                ),
            ]));
        }
        return lines;
    }

    // If thinking is complete (has </think>), show accordion followed by response
    if has_thinking {
        // Add thinking accordion header
        if show_thinking {
            lines.push(Line::from(vec![
                Span::styled(
                    "▼ ",
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "Thinking",
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::ITALIC),
                ),
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
                Span::styled(
                    "▶ ",
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(
                    "Thinking",
                    Style::default()
                        .fg(theme.accent)
                        .add_modifier(Modifier::ITALIC),
                ),
            ]));
        }
        
        // Add response content immediately after (no gap)
        for response_line in &response_content {
            lines.push(Line::from(*response_line));
        }
        
        return lines;
    }

    // If no thinking tags were found, just return the plain lines
    content.lines().map(|line| Line::from(line)).collect()
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
pub enum MessageRole {
    User,
    Assistant,
}

impl Message {
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

    pub fn assistant(content: String) -> Self {
        let token_count = count_tokens(&content);
        Self {
            role: MessageRole::Assistant,
            content,
            timestamp: chrono::Local::now(),
            token_count,
            thinking_expanded: false,
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
    pub fn with_scroll(
        messages: &'a [Message],
        theme: &'a ChatTheme,
        scroll_offset: usize,
    ) -> Self {
        Self {
            messages,
            theme,
            scroll_offset,
            shimmer: None,
            typing_indicator: None,
            selected_message_index: None,
        }
    }
    
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

    /// Calculate total height of all messages
    pub fn calculate_total_height(&self) -> usize {
        self.messages
            .iter()
            .map(|msg| {
                let content_lines = msg.content.lines().count();
                content_lines + 3 + 1 // content + header + borders + gap
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
                    let token_text = format!("{}tokens", msg.token_count);
                    let header = Line::from(vec![
                        Span::styled(
                            "Sumon",
                            Style::default()
                                .fg(self.theme.accent)
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::raw("  "),
                        Span::styled(token_text, Style::default().fg(self.theme.border)),
                    ]);

                    let content_lines: Vec<Line> = msg
                        .content
                        .lines()
                        .map(|line| Line::from(Span::raw(line)))
                        .collect();

                    // Calculate dynamic width based on content
                    let max_content_width = content_lines
                        .iter()
                        .map(|line| {
                            line.spans
                                .iter()
                                .map(|span| span.content.len())
                                .sum::<usize>()
                        })
                        .max()
                        .unwrap_or(0);

                    // Width: fit content tightly with minimal padding
                    // Add only 4 for borders (2) + minimal spacing (2)
                    // Minimum width to show "Sumon X tokens" properly
                    let header_width = format!("Sumon  {}tokens", msg.token_count).len();
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

                    let msg_area = Rect {
                        x: msg_x,
                        y,
                        width: msg_width,
                        height: msg_height as u16,
                    };

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
                        .render(
                            Rect {
                                x: padded.x,
                                y: padded.y,
                                width: padded.width,
                                height: 1,
                            },
                            buf,
                        );

                    // Render content right-aligned
                    if padded.height > 1 {
                        let content_area = Rect {
                            x: padded.x,
                            y: padded.y + 1,
                            width: padded.width,
                            height: padded.height - 1,
                        };

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
                    let header = Line::from(vec![Span::styled(
                        time,
                        Style::default().fg(self.theme.border),
                    )]);

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
                                    Style::default()
                                        .fg(shimmer_color)
                                        .add_modifier(Modifier::ITALIC),
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

                    let msg_area = Rect {
                        x: area.x,
                        y,
                        width: area.width,
                        height: msg_height as u16,
                    };

                    // Render header
                    Paragraph::new(header)
                        .style(Style::default().bg(self.theme.bg))
                        .render(
                            Rect {
                                x: msg_area.x,
                                y: msg_area.y,
                                width: msg_area.width,
                                height: 1,
                            },
                            buf,
                        );

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
            let mut scrollbar_state =
                ScrollbarState::new(total_height.saturating_sub(viewport_height))
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

// Commented out - not needed for minimal version
/*
pub struct ModeSelector<'a> {
    current_mode: ChatMode,
    theme: &'a ChatTheme,
    focused: bool,
}

impl<'a> ModeSelector<'a> {
    pub fn new(current_mode: ChatMode, theme: &'a ChatTheme, focused: bool) -> Self {
        Self {
            current_mode,
            theme,
            focused,
        }
    }
}

impl Widget for ModeSelector<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(33),
                Constraint::Percentage(34),
                Constraint::Percentage(33),
            ])
            .split(area);

        let modes = [ChatMode::Agent, ChatMode::Plan, ChatMode::Ask];
        let symbols = ["▸", "◆", "◉"];

        for (i, (mode, symbol)) in modes.iter().zip(symbols.iter()).enumerate() {
            let is_selected = *mode == self.current_mode;
            let style = if is_selected {
                self.theme.mode_style(mode).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(self.theme.border)
            };

            let border_style = if is_selected && self.focused {
                self.theme.border_style(true)
            } else {
                self.theme.border_style(false)
            };

            let block = Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .style(Style::default().bg(self.theme.bg));

            let text = vec![
                Line::from(Span::styled(*symbol, style)),
                Line::from(Span::styled(mode.to_string(), style)),
            ];

            let paragraph = Paragraph::new(text)
                .block(block)
                .alignment(Alignment::Center);

            paragraph.render(chunks[i], buf);
        }
    }
}
*/

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
            Text::from(Line::from(Span::styled(
                self.placeholder,
                Style::default().fg(self.theme.border),
            )))
        } else {
            Text::from(self.content)
        };

        Paragraph::new(text)
            .wrap(Wrap { trim: false })
            .render(inner, buf);

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
        Self {
            indicator,
            shimmer,
            theme,
        }
    }
}

impl Widget for LoadingIndicator<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let shimmer_color = self.shimmer.current_color();

        // AI loading: just show the animated dots without "Assistant" text
        let text = Line::from(vec![Span::styled(
            self.indicator.text(self.indicator.is_visible()),
            Style::default()
                .fg(shimmer_color)
                .add_modifier(Modifier::ITALIC),
        )]);

        Paragraph::new(text)
            .style(Style::default().bg(self.theme.bg))
            .alignment(Alignment::Left)
            .render(area, buf);
    }
}

// Commented out - not needed for minimal version
/*
pub struct CombinedInputBar<'a> {
    input: &'a InputState,
    theme: &'a ChatTheme,
    mode: ChatMode,
    audio_mode: bool,
    audio_processing: bool,
    focused: bool,
    cursor_visible: bool,
}

impl<'a> CombinedInputBar<'a> {
    pub fn new(
        input: &'a InputState,
        theme: &'a ChatTheme,
        mode: ChatMode,
        audio_mode: bool,
        audio_processing: bool,
        focused: bool,
        cursor_visible: bool,
    ) -> Self {
        Self {
            input,
            theme,
            mode,
            audio_mode,
            audio_processing,
            focused,
            cursor_visible,
        }
    }
}

impl Widget for CombinedInputBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(self.theme.border))
            .border_type(ratatui::widgets::BorderType::Rounded)
            .style(Style::default().bg(self.theme.bg));

        let inner = block.inner(area);
        block.render(area, buf);

        // Add horizontal padding, minimal vertical padding
        let padded = Rect {
            x: inner.x + 2,
            y: inner.y,
            width: inner.width.saturating_sub(4),
            height: inner.height,
        };

        // Split into input area and bottom bar with no gap
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Length(1),
                Constraint::Length(0),
            ])
            .split(padded);

        // Input area
        let placeholder = "A question or a prompt... (Enter to send, Shift+Enter for new line)";
        let text = if self.input.content.is_empty() {
            Text::from(Line::from(Span::styled(
                placeholder,
                Style::default().fg(self.theme.border),
            )))
        } else {
            Text::from(self.input.content.as_str())
        };

        // Render text with selection highlighting
        if self.input.has_selection() {
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

            let mut x = chunks[0].x;
            let mut y = chunks[0].y;

            for (i, ch) in self.input.content.chars().enumerate() {
                if x >= chunks[0].right() {
                    x = chunks[0].x;
                    y += 1;
                    if y >= chunks[0].bottom() {
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
        } else {
            Paragraph::new(text)
                .wrap(Wrap { trim: false })
                .style(Style::default().bg(self.theme.bg).fg(self.theme.fg))
                .render(chunks[0], buf);
        }

        // Render cursor when focused and visible (blinking) - smaller cursor
        if self.focused && self.cursor_visible {
            let cursor_x = chunks[0].x + (self.input.cursor_position as u16 % chunks[0].width);
            let cursor_y = chunks[0].y + (self.input.cursor_position as u16 / chunks[0].width);

            if cursor_x < chunks[0].right() && cursor_y < chunks[0].bottom() {
                let cell = &mut buf[(cursor_x, cursor_y)];
                // Keep the existing character, just change the style to show cursor
                let existing_char = cell.symbol().chars().next().unwrap_or(' ');
                if existing_char == ' ' || self.input.content.is_empty() {
                    // Show cursor bar when at empty space
                    cell.set_char('▎');
                    cell.set_style(Style::default().fg(self.theme.accent));
                } else {
                    // Invert colors to show cursor on existing character
                    cell.set_style(Style::default().bg(self.theme.accent).fg(self.theme.bg));
                }
            }
        }

        // Bottom bar
        let bottom_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(6),
                Constraint::Length(6),
                Constraint::Length(15),
                Constraint::Min(10),
                Constraint::Length(8),
                Constraint::Length(8),
                Constraint::Length(6),
            ])
            .split(chunks[1]);

        // Add button
        Paragraph::new(Span::styled("Add", Style::default().fg(self.theme.fg)))
            .alignment(Alignment::Left)
            .render(bottom_chunks[0], buf);

        // Plan button
        Paragraph::new(Span::styled("Plan", Style::default().fg(self.theme.fg)))
            .alignment(Alignment::Left)
            .render(bottom_chunks[1], buf);

        // Model selector
        Paragraph::new(Span::styled(
            "Gemini 3 Pro",
            Style::default().fg(self.theme.fg),
        ))
        .alignment(Alignment::Left)
        .render(bottom_chunks[2], buf);

        // Audio button
        let (audio_text_str, audio_style) = if self.audio_processing {
            (
                "Processing...",
                Style::default()
                    .fg(self.theme.accent)
                    .add_modifier(Modifier::BOLD),
            )
        } else if self.audio_mode {
            (
                "Audio",
                Style::default()
                    .fg(self.theme.accent)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            ("Audio", Style::default().fg(self.theme.fg))
        };
        Paragraph::new(Span::styled(audio_text_str, audio_style))
            .alignment(Alignment::Right)
            .render(bottom_chunks[4], buf);

        // Local button
        Paragraph::new(Span::styled("Local", Style::default().fg(self.theme.fg)))
            .alignment(Alignment::Right)
            .render(bottom_chunks[5], buf);

        // Send button
        Paragraph::new(Span::styled(
            "Send",
            Style::default()
                .fg(self.theme.accent)
                .add_modifier(Modifier::BOLD),
        ))
        .alignment(Alignment::Right)
        .render(bottom_chunks[6], buf);
    }
}
*/

// Commented out - not needed for minimal version
/*
pub struct HeaderBar<'a> {
    theme: &'a ChatTheme,
    mode: ChatMode,
}

impl<'a> HeaderBar<'a> {
    pub fn new(theme: &'a ChatTheme, mode: ChatMode) -> Self {
        Self { theme, mode }
    }
}

impl Widget for HeaderBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(vec![
            Span::styled("▸ ", Style::default().fg(self.theme.accent)),
            Span::styled("DX Chat", self.theme.title_style()),
            Span::raw(" │ "),
            Span::styled(
                format!("{} {}", self.mode.icon(), self.mode),
                self.theme.mode_style(&self.mode),
            ),
        ]);

        Paragraph::new(title)
            .alignment(Alignment::Center)
            .style(Style::default().bg(self.theme.bg))
            .render(area, buf);
    }
}
*/

// Commented out - not needed for minimal version
/*
pub struct BottomBar<'a> {
    theme: &'a ChatTheme,
    mode: ChatMode,
    audio_mode: bool,
    audio_processing: bool,
}

impl<'a> BottomBar<'a> {
    pub fn new(
        theme: &'a ChatTheme,
        mode: ChatMode,
        audio_mode: bool,
        audio_processing: bool,
    ) -> Self {
        Self {
            theme,
            mode,
            audio_mode,
            audio_processing,
        }
    }
}

impl Widget for BottomBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(15),
                Constraint::Length(6),
                Constraint::Length(6),
                Constraint::Min(10),
                Constraint::Length(8),
                Constraint::Length(8),
                Constraint::Length(6),
            ])
            .split(area);

        // Model selector (left)
        let model_text = Line::from(Span::styled(
            "Gemini 3 Pro",
            Style::default().fg(self.theme.fg),
        ));
        Paragraph::new(model_text)
            .alignment(Alignment::Left)
            .style(Style::default().bg(self.theme.bg))
            .render(chunks[0], buf);

        // Add button
        let add_text = Line::from(Span::styled("Add", Style::default().fg(self.theme.fg)));
        Paragraph::new(add_text)
            .alignment(Alignment::Left)
            .style(Style::default().bg(self.theme.bg))
            .render(chunks[1], buf);

        // Plan button
        let plan_text = Line::from(Span::styled("Plan", Style::default().fg(self.theme.fg)));
        Paragraph::new(plan_text)
            .alignment(Alignment::Left)
            .style(Style::default().bg(self.theme.bg))
            .render(chunks[2], buf);

        // Center - empty
        Paragraph::new("")
            .style(Style::default().bg(self.theme.bg))
            .render(chunks[3], buf);

        // Audio button (highlighted when active, shows processing status)
        let (audio_text_str, audio_style) = if self.audio_processing {
            (
                "Processing...",
                Style::default()
                    .fg(self.theme.accent)
                    .add_modifier(Modifier::BOLD),
            )
        } else if self.audio_mode {
            (
                "Audio",
                Style::default()
                    .fg(self.theme.accent)
                    .add_modifier(Modifier::BOLD),
            )
        } else {
            ("Audio", Style::default().fg(self.theme.fg))
        };
        let audio_text = Line::from(Span::styled(audio_text_str, audio_style));
        Paragraph::new(audio_text)
            .alignment(Alignment::Right)
            .style(Style::default().bg(self.theme.bg))
            .render(chunks[4], buf);

        // Local button
        let local_text = Line::from(Span::styled("Local", Style::default().fg(self.theme.fg)));
        Paragraph::new(local_text)
            .alignment(Alignment::Right)
            .style(Style::default().bg(self.theme.bg))
            .render(chunks[5], buf);

        // Send button
        let send_text = Line::from(Span::styled(
            "Send",
            Style::default()
                .fg(self.theme.accent)
                .add_modifier(Modifier::BOLD),
        ));
        Paragraph::new(send_text)
            .alignment(Alignment::Right)
            .style(Style::default().bg(self.theme.bg))
            .render(chunks[6], buf);
    }
}
*/

// Commented out - not needed for minimal version
/*
pub struct SecondaryBar<'a> {
    theme: &'a ChatTheme,
    shortcut_index: usize,
    changes_count: usize,
    tasks_count: usize,
    agents_count: usize,
    memory_mode: &'a str,
    audio_mode: bool,
    audio_processing: bool,
}

impl<'a> SecondaryBar<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        theme: &'a ChatTheme,
        shortcut_index: usize,
        changes_count: usize,
        tasks_count: usize,
        agents_count: usize,
        memory_mode: &'a str,
        audio_mode: bool,
        audio_processing: bool,
    ) -> Self {
        Self {
            theme,
            shortcut_index,
            changes_count,
            tasks_count,
            agents_count,
            memory_mode,
            audio_mode,
            audio_processing,
        }
    }
}

impl Widget for SecondaryBar<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let shortcuts = [
            "Ctrl+M: Toggle Menu | Alt+A: Audio",
            "Alt+1/2/3: Switch Modes | Alt+T: Theme",
            "Enter: Send | Shift+Enter: New Line",
            "Ctrl+A/E: Start/End | Ctrl+U/K: Clear",
            "Ctrl+W: Delete Word | Ctrl+D: Exit",
            "Tab: Switch Focus | Arrows: Navigate",
        ];

        let current_shortcut = shortcuts[self.shortcut_index % shortcuts.len()];

        // Add minimal horizontal padding
        let padded = Rect {
            x: area.x + 1,
            y: area.y,
            width: area.width.saturating_sub(2),
            height: area.height,
        };

        // Calculate dynamic widths based on content
        let changes_text = if self.changes_count > 0 {
            if self.changes_count >= 1_000_000 {
                format!("Changes:{}M", self.changes_count / 1_000_000)
            } else if self.changes_count >= 1_000 {
                format!("Changes:{}k", self.changes_count / 1_000)
            } else {
                format!("Changes:{}", self.changes_count)
            }
        } else {
            "Changes".to_string()
        };

        let changes_width = changes_text.len() as u16;
        let tasks_text = if self.tasks_count > 0 {
            if self.tasks_count >= 1_000_000 {
                format!("Drivens:{}M", self.tasks_count / 1_000_000)
            } else if self.tasks_count >= 1_000 {
                format!("Drivens:{}k", self.tasks_count / 1_000)
            } else {
                format!("Drivens:{}", self.tasks_count)
            }
        } else {
            "Drivens".to_string()
        };
        let tasks_width = tasks_text.len() as u16;
        let agents_text = if self.agents_count > 0 {
            if self.agents_count >= 1_000_000 {
                format!("Workspaces:{}M", self.agents_count / 1_000_000)
            } else if self.agents_count >= 1_000 {
                format!("Workspaces:{}k", self.agents_count / 1_000)
            } else {
                format!("Workspaces:{}", self.agents_count)
            }
        } else {
            "Workspaces".to_string()
        };
        let agents_width = agents_text.len() as u16;
        let memory_text = self.memory_mode.to_string();
        let memory_width = memory_text.len() as u16;
        let tools_width = 5; // "Tools"
        let more_width = 4; // "More"

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Length(changes_width),
                Constraint::Length(1), // Minimal gap
                Constraint::Length(tasks_width),
                Constraint::Length(1), // Minimal gap
                Constraint::Length(agents_width),
                Constraint::Min(5), // Center shortcuts - flexible space
                Constraint::Length(memory_width),
                Constraint::Length(1), // Minimal gap
                Constraint::Length(tools_width),
                Constraint::Length(1), // Minimal gap
                Constraint::Length(more_width),
            ])
            .split(padded);

        // Left items - muted colors
        Paragraph::new(Span::styled(
            &changes_text,
            Style::default().fg(self.theme.border),
        ))
        .alignment(Alignment::Left)
        .render(chunks[0], buf);

        Paragraph::new(Span::styled(
            &tasks_text,
            Style::default().fg(self.theme.border),
        ))
        .alignment(Alignment::Left)
        .render(chunks[2], buf);

        Paragraph::new(Span::styled(
            &agents_text,
            Style::default().fg(self.theme.border),
        ))
        .alignment(Alignment::Left)
        .render(chunks[4], buf);

        // Center - rotating shortcuts
        Paragraph::new(Span::styled(
            current_shortcut,
            Style::default().fg(self.theme.border),
        ))
        .alignment(Alignment::Center)
        .render(chunks[5], buf);

        // Right items - muted colors
        Paragraph::new(Span::styled(
            &memory_text,
            Style::default().fg(self.theme.border),
        ))
        .alignment(Alignment::Left)
        .render(chunks[6], buf);

        Paragraph::new(Span::styled(
            "Tools",
            Style::default().fg(self.theme.border),
        ))
        .alignment(Alignment::Left)
        .render(chunks[8], buf);

        Paragraph::new(Span::styled("More", Style::default().fg(self.theme.border)))
            .alignment(Alignment::Left)
            .render(chunks[10], buf);
    }
}

/// Parse markdown content and convert to styled ratatui Lines
fn parse_markdown_to_lines(content: &str, theme: &ChatTheme) -> Vec<Line<'static>> {
    let mut lines = Vec::new();

    // Filter out <think> tags and their content
    let filtered_content = filter_think_tags(content);

    for line in filtered_content.lines() {
        let trimmed = line.trim();

        // Headers
        if let Some(rest) = trimmed.strip_prefix("### ") {
            lines.push(Line::from(Span::styled(
                rest.to_string(),
                Style::default().fg(theme.fg).add_modifier(Modifier::BOLD),
            )));
        } else if let Some(rest) = trimmed.strip_prefix("## ") {
            lines.push(Line::from(Span::styled(
                rest.to_string(),
                Style::default()
                    .fg(theme.accent_secondary)
                    .add_modifier(Modifier::BOLD),
            )));
        } else if let Some(rest) = trimmed.strip_prefix("# ") {
            lines.push(Line::from(Span::styled(
                rest.to_string(),
                Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD),
            )));
        }
        // Code blocks
        else if trimmed.starts_with("```") {
            lines.push(Line::from(Span::styled(
                line.to_string(),
                Style::default().fg(theme.border),
            )));
        }
        // Lists
        else if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
            let mut spans = vec![Span::styled(
                "• ".to_string(),
                Style::default().fg(theme.accent),
            )];
            // Parse the rest of the line for inline formatting
            spans.extend(parse_inline_markdown(&trimmed[2..], theme));
            lines.push(Line::from(spans));
        }
        // Numbered lists
        else if trimmed.chars().next().map_or(false, |c| c.is_numeric()) && trimmed.contains(". ")
        {
            lines.push(Line::from(parse_inline_markdown(line, theme)));
        }
        // Regular text with inline formatting
        else {
            lines.push(Line::from(parse_inline_markdown(line, theme)));
        }
    }

    lines
}

/// Filter out <think> tags and their content
fn filter_think_tags(content: &str) -> String {
    let mut result = String::with_capacity(content.len());
    let mut chars = content.chars().peekable();
    let mut in_think_tag = false;

    while let Some(ch) = chars.next() {
        if ch == '<' {
            // Check if this is a <think> or </think> tag
            let remaining: String = chars.clone().collect();
            if remaining.starts_with("think>") {
                // Opening tag
                in_think_tag = true;
                // Skip "think>"
                for _ in 0..6 {
                    chars.next();
                }
                continue;
            } else if remaining.starts_with("/think>") {
                // Closing tag
                in_think_tag = false;
                // Skip "/think>"
                for _ in 0..7 {
                    chars.next();
                }
                continue;
            }
        }

        if !in_think_tag {
            result.push(ch);
        }
    }

    result.trim().to_string()
}

/// Parse inline markdown (bold, italic, code) within a line
fn parse_inline_markdown(text: &str, theme: &ChatTheme) -> Vec<Span<'static>> {
    let mut spans = Vec::new();
    let mut remaining = text;

    while !remaining.is_empty() {
        // Try bold first (**text**)
        if let Some(start) = remaining.find("**") {
            // Add text before bold
            if start > 0 {
                spans.push(Span::raw(remaining[..start].to_string()));
            }

            // Find closing **
            let after_start = &remaining[start + 2..];
            if let Some(end) = after_start.find("**") {
                // Found closing **, add bold text
                spans.push(Span::styled(
                    after_start[..end].to_string(),
                    Style::default().add_modifier(Modifier::BOLD),
                ));
                remaining = &after_start[end + 2..];
            } else {
                // No closing **, treat as literal
                spans.push(Span::raw("**".to_string()));
                remaining = after_start;
            }
        }
        // Try inline code (`text`)
        else if let Some(start) = remaining.find('`') {
            // Add text before code
            if start > 0 {
                spans.push(Span::raw(remaining[..start].to_string()));
            }

            // Find closing `
            let after_start = &remaining[start + 1..];
            if let Some(end) = after_start.find('`') {
                // Found closing `, add code text
                spans.push(Span::styled(
                    after_start[..end].to_string(),
                    Style::default().fg(theme.accent).bg(theme.border),
                ));
                remaining = &after_start[end + 1..];
            } else {
                // No closing `, treat as literal
                spans.push(Span::raw("`".to_string()));
                remaining = after_start;
            }
        }
        // Try italic (*text*) - but not at start of line (could be list)
        else if let Some(start) = remaining.find('*') {
            // Add text before italic
            if start > 0 {
                spans.push(Span::raw(remaining[..start].to_string()));
            }

            // Find closing *
            let after_start = &remaining[start + 1..];
            if let Some(end) = after_start.find('*') {
                // Found closing *, add italic text
                spans.push(Span::styled(
                    after_start[..end].to_string(),
                    Style::default().add_modifier(Modifier::ITALIC),
                ));
                remaining = &after_start[end + 1..];
            } else {
                // No closing *, treat as literal
                spans.push(Span::raw("*".to_string()));
                remaining = after_start;
            }
        } else {
            // No more markdown, add remaining text
            spans.push(Span::raw(remaining.to_string()));
            break;
        }
    }

    spans
}
*/
