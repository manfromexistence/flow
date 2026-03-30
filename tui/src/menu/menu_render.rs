// Menu rendering logic
use super::menu_data::Menu;
use ratatui::{
	buffer::Buffer,
	layout::Rect,
	style::{Modifier, Style},
	text::{Line, Span, Text},
	widgets::{Block, Widget},
};

impl Menu {
	pub fn render_in_area(
		&mut self,
		area: Rect,
		buf: &mut Buffer,
		theme_mode: &crate::theme::ThemeVariant,
	) {
		let content_bg = self.theme.card;

		// Create a centered content area
		let content_width = (area.width * 7 / 10).min(80);
		let content_height = (area.height * 75 / 100).min(32);

		let x_offset = (area.width - content_width) / 2;
		let y_offset = (area.height - content_height) / 2;

		let content_area = Rect {
			x: area.x + x_offset,
			y: area.y + y_offset,
			width: content_width,
			height: content_height,
		};

		self.menu_area = content_area;

		// Determine menu title
		let menu_title = if let Some(submenu_idx) = self.current_submenu {
			let parent_name = self.main_menu[submenu_idx]
				.0
				.trim_start_matches(|c: char| c.is_numeric() || c == '.' || c.is_whitespace());
			let item_count = self.menu_items.len() - 1;

			if submenu_idx == 0 {
				let mode_str = match theme_mode {
					crate::theme::ThemeVariant::Dark => "Dark",
					crate::theme::ThemeVariant::Light => "Light",
				};
				format!("{} - {} Mode ({} items)", parent_name, mode_str, item_count)
			} else {
				format!("{} ({} items)", parent_name, item_count)
			}
		} else {
			let item_count = self.menu_items.len();
			format!("Command Palette ({} items)", item_count)
		};

		Block::default()
			.borders(ratatui::widgets::Borders::ALL)
			.border_style(Style::default().fg(self.theme.border))
			.border_type(ratatui::widgets::BorderType::Rounded)
			.title(Span::styled(
				format!(" {} ", menu_title),
				Style::default().fg(self.theme.accent).add_modifier(Modifier::BOLD),
			))
			.style(Style::default().bg(content_bg))
			.render(content_area, buf);

		let padded_area = Rect {
			x: content_area.x + 2,
			y: content_area.y + 1,
			width: content_area.width.saturating_sub(4),
			height: content_area.height.saturating_sub(2),
		};

		let text_fg = self.theme.fg;
		let selected_bg = self.theme.accent;
		let selected_fg = self.theme.bg;
		let hover_bg = self.theme.primary;

		let visible_items = padded_area.height as usize;

		if self.selected_menu_item < self.scroll_offset {
			self.scroll_offset = self.selected_menu_item;
		} else if self.selected_menu_item >= self.scroll_offset + visible_items {
			self.scroll_offset = self.selected_menu_item - visible_items + 1;
		}

		let mut lines = Vec::new();
		let end_idx = (self.scroll_offset + visible_items).min(self.menu_items.len());
		let exact_width = padded_area.width as usize;

		self.menu_item_areas.clear();
		let mut current_y = padded_area.y;

		for idx in self.scroll_offset..end_idx {
			let (title, description) = &self.menu_items[idx];
			let is_selected = idx == self.selected_menu_item;
			let is_hovered = self.hovered_menu_item == Some(idx);

			self.menu_item_areas.push(Rect {
				x: padded_area.x,
				y: current_y,
				width: padded_area.width,
				height: 1,
			});
			current_y += 1;

			// Format the line
			let item_text = if description == "TOGGLE_MODE" {
				let mode_indicator = match theme_mode {
					crate::theme::ThemeVariant::Dark => "(Dark)",
					crate::theme::ThemeVariant::Light => "(Light)",
				};
				format!("{} {}", title, mode_indicator)
			} else if description == "TOGGLE_RECORDING" {
				let mode_indicator = if self.recording_mode { "(Recording)" } else { "(Viewing)" };
				format!("{} {}", title, mode_indicator)
			} else if !description.is_empty()
				&& description != "TOGGLE_MODE"
				&& description != "TOGGLE_RECORDING"
			{
				let left_part = title.to_string();
				let right_part = description;
				let available_width = exact_width.saturating_sub(right_part.len() + 3);

				if left_part.len() > available_width {
					let truncate_at = available_width.saturating_sub(3);
					format!("{}...  {}", &left_part[..truncate_at], right_part)
				} else {
					let padding = available_width.saturating_sub(left_part.len());
					format!("{}{}  {}", left_part, " ".repeat(padding), right_part)
				}
			} else {
				title.to_string()
			};

			let line_text = if item_text.len() > exact_width {
				let truncate_at = exact_width.saturating_sub(3);
				format!("{}...", &item_text[..truncate_at])
			} else {
				item_text
			};

			let (fg, bg) = if is_selected {
				(selected_fg, selected_bg)
			} else if is_hovered {
				(self.theme.bg, hover_bg)
			} else {
				(text_fg, content_bg)
			};

			let style = if is_selected {
				Style::default().fg(fg).bg(bg).add_modifier(Modifier::BOLD)
			} else {
				Style::default().fg(fg).bg(bg)
			};

			let padded_line = format!(" {:<width$}", line_text, width = exact_width.saturating_sub(1));
			lines.push(Line::from(Span::styled(padded_line, style)));
		}

		// Fill remaining lines
		let items_shown = end_idx - self.scroll_offset;
		for _ in items_shown..visible_items {
			let empty_line = " ".repeat(exact_width);
			lines.push(Line::from(Span::styled(empty_line, Style::default().bg(content_bg))));
		}

		let main_text = Text::from(lines);
		ratatui::widgets::Paragraph::new(main_text).render(padded_area, buf);
	}
}
