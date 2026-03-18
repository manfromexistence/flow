//! Performance overlay rendering for displaying real-time metrics

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::app::ChatApp;

impl ChatApp {
    /// Render performance overlay in the top-right corner
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
            Color::Green
        } else if stats.avg_frame_render_ms < 50.0 {
            Color::Yellow
        } else {
            Color::Red
        };

        // Build content lines
        let mut lines = vec![
            Line::from(vec![
                Span::styled("⚡ ", Style::default().fg(Color::Yellow)),
                Span::styled("Performance Monitor", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("Input:    "),
                Span::styled(
                    format!("{:.2}ms", stats.avg_input_render_ms),
                    Style::default().fg(if stats.avg_input_render_ms < 16.0 { Color::Green } else { Color::Yellow })
                ),
                Span::raw(" / "),
                Span::styled(
                    format!("{:.2}ms", stats.max_input_render_ms),
                    Style::default().fg(Color::DarkGray)
                ),
            ]),
            Line::from(vec![
                Span::raw("Keystroke: "),
                Span::styled(
                    format!("{:.2}ms", stats.avg_keystroke_latency_ms),
                    Style::default().fg(if stats.avg_keystroke_latency_ms < 50.0 { Color::Green } else { Color::Yellow })
                ),
                Span::raw(" / "),
                Span::styled(
                    format!("{:.2}ms", stats.max_keystroke_latency_ms),
                    Style::default().fg(Color::DarkGray)
                ),
            ]),
            Line::from(vec![
                Span::raw("Frame:    "),
                Span::styled(
                    format!("{:.2}ms", stats.avg_frame_render_ms),
                    Style::default().fg(if stats.avg_frame_render_ms < 33.0 { Color::Green } else { Color::Yellow })
                ),
                Span::raw(" / "),
                Span::styled(
                    format!("{:.2}ms", stats.max_frame_render_ms),
                    Style::default().fg(Color::DarkGray)
                ),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("Samples: "),
                Span::styled(
                    format!("{}", stats.total_samples),
                    Style::default().fg(Color::Cyan)
                ),
            ]),
            Line::from(vec![
                Span::raw("Status:  "),
                Span::styled(
                    if self.perf_monitor.is_meeting_targets() { "✓ EXCELLENT" } else { "○ GOOD" },
                    Style::default().fg(status_color).add_modifier(Modifier::BOLD)
                ),
            ]),
        ];

        // Add help text at bottom
        lines.push(Line::from(vec![
            Span::styled("Ctrl+P", Style::default().fg(Color::DarkGray)),
            Span::raw(" to toggle"),
        ]));

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(status_color))
            .border_type(ratatui::widgets::BorderType::Rounded)
            .style(Style::default().bg(Color::Black));

        let paragraph = Paragraph::new(lines)
            .block(block)
            .style(Style::default().fg(self.theme.fg));

        paragraph.render(overlay_area, buf);
    }

    /// Render compact performance status in status bar
    pub fn render_perf_status(&self) -> String {
        if self.show_perf_overlay {
            self.perf_monitor.get_compact_status()
        } else {
            String::new()
        }
    }
}
