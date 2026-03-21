//! Input box rendering logic

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Widget, Wrap},
};

use crate::app::ChatApp;

impl ChatApp {
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
}
