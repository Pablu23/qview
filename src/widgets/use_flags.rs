use std::collections::HashSet;

use ratatui::{
    Frame,
    layout::Rect,
    style::Modifier,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
};

use crate::{gentoo::UseFlag, theme::Theme};

pub fn render_use_flags(
    frame: &mut Frame,
    area: Rect,
    use_flags: Vec<&UseFlag>,
    enabled_use_flags: &HashSet<String>,
) {
    let max_len = use_flags.iter().map(|f| f.name.len()).max().unwrap_or(1);

    // width per column (+ padding between columns)
    let col_width = max_len as u16 + 3;

    let columns = (area.width / col_width).max(1) as usize;
    let rows = use_flags.len().div_ceil(columns);

    let mut lines = Vec::new();

    for row in 0..rows {
        let mut spans = Vec::new();

        for col in 0..columns {
            let idx = row * columns + col;

            if let Some(flag) = use_flags.get(idx) {
                let mut style = if enabled_use_flags.contains(&flag.name) {
                    Theme::success()
                } else if flag.default {
                    Theme::info()
                } else {
                    Theme::error()
                };

                if flag.default {
                    style = style.add_modifier(Modifier::ITALIC);
                }

                spans.push(Span::styled(
                    format!("{:<width$}", flag.name, width = max_len + 2),
                    style,
                ));
            }
        }

        lines.push(Line::from(spans));
    }

    let paragraph = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::ALL)
            .title("USE Flags")
            .border_style(Theme::block()),
    );

    frame.render_widget(paragraph, area);
}
