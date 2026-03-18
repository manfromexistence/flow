//! Bottom controls and helper rendering functions

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};

use crate::app::ChatApp;

impl ChatApp {
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

    #[allow(dead_code)]
    pub fn render_shortcut_debug(&self, area: Rect, buf: &mut Buffer, shortcut: &str) {
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

    #[allow(dead_code)]
    pub fn render_audio_recording_indicator(&self, area: Rect, buf: &mut Buffer) {
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
}
