use super::super::{modal_list::ModalList, theme::ChatTheme};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph, Widget},
};

pub fn render(area: Rect, buf: &mut Buffer, theme: &ChatTheme, list: &ModalList) {
    let modal_width = area.width.saturating_sub(20).min(60);
    let modal_height = area.height.saturating_sub(10).min(18);
    let modal_area = Rect {
        x: (area.width.saturating_sub(modal_width)) / 2,
        y: (area.height.saturating_sub(modal_height)) / 2,
        width: modal_width,
        height: modal_height,
    };

    Clear.render(modal_area, buf);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .border_type(ratatui::widgets::BorderType::Rounded)
        .title(Span::styled(
            " Focus Menu ",
            Style::default().fg(theme.accent).add_modifier(Modifier::BOLD),
        ))
        .style(Style::default().bg(theme.bg));

    let inner = block.inner(modal_area);
    block.render(modal_area, buf);

    let mut y = inner.y + 1;

    let options = [
        ("1", "Agent Mode", "Switch to Agent mode"),
        ("2", "Plan Mode", "Switch to Plan mode"),
        ("3", "Ask Mode", "Switch to Ask mode"),
        ("4", "Model Selector", "Choose AI model"),
        ("5", "Audio Toggle", "Enable/disable audio input"),
        ("6", "Local Mode", "Switch local processing mode"),
    ];

    for (number, title, description) in options.iter() {
        if y >= inner.bottom() - 1 {
            break;
        }

        let idx = number.parse::<usize>().unwrap() - 1;
        let is_selected = idx == list.selected;

        let bg_style = if is_selected {
            Style::default().bg(theme.accent).fg(theme.bg)
        } else {
            Style::default().bg(theme.bg).fg(theme.fg)
        };

        let title_style = if is_selected {
            bg_style.add_modifier(Modifier::BOLD)
        } else {
            bg_style
        };

        if is_selected {
            for x in inner.x + 1..inner.x + inner.width - 1 {
                let cell = &mut buf[(x, y)];
                cell.set_style(bg_style);
            }
        }

        let spans = vec![
            Span::styled("  ", bg_style),
            Span::styled(
                *number,
                if is_selected {
                    bg_style.add_modifier(Modifier::BOLD)
                } else {
                    Style::default().fg(theme.accent).bg(theme.bg)
                },
            ),
            Span::styled("  ", bg_style),
            Span::styled(*title, title_style),
            Span::styled("  ", bg_style),
            Span::styled(
                *description,
                if is_selected {
                    bg_style
                } else {
                    Style::default().fg(theme.border).bg(theme.bg)
                },
            ),
        ];

        let line = Line::from(spans);

        Paragraph::new(line).render(
            Rect {
                x: inner.x + 1,
                y,
                width: inner.width.saturating_sub(2),
                height: 1,
            },
            buf,
        );

        y += 1;
    }

    let help_text = Line::from(vec![
        Span::styled("1-6", Style::default().fg(theme.accent)),
        Span::styled(" Quick Select  ", Style::default().fg(theme.border)),
        Span::styled("↑↓", Style::default().fg(theme.accent)),
        Span::styled(" Navigate  ", Style::default().fg(theme.border)),
        Span::styled("Enter", Style::default().fg(theme.accent)),
        Span::styled(" Select  ", Style::default().fg(theme.border)),
        Span::styled("Tab/Esc", Style::default().fg(theme.accent)),
        Span::styled(" Close", Style::default().fg(theme.border)),
    ]);

    Paragraph::new(help_text).alignment(ratatui::layout::Alignment::Center).render(
        Rect {
            x: inner.x,
            y: inner.y + inner.height.saturating_sub(1),
            width: inner.width,
            height: 1,
        },
        buf,
    );
}
