use chrono::{DateTime, Utc};
use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::{
    gentoo::package::Metadata,
    theme::Theme,
    widgets::helpers::{homepage_lines, human_size},
};

pub fn render_package_metadata(
    frame: &mut Frame,
    area: Rect,
    pkg_version: &String,
    metadata: &Metadata,
    size: Option<usize>,
    build_time: Option<DateTime<Utc>>,
) {
    let bold_style = Theme::title(); //Style::default().add_modifier(Modifier::BOLD);
    let maintainer = Line::from_iter([
        Span::styled("Maintainer: ", bold_style),
        Span::raw(metadata.maintainer.as_deref().unwrap_or("Unknown")),
    ]);

    let version = Line::from_iter([
        Span::styled("Version: ", bold_style),
        Span::raw(pkg_version),
    ]);

    let repository = Line::from_iter([
        Span::styled("Repository: ", bold_style),
        Span::raw(&metadata.repository),
    ]);

    let license = Line::from_iter([
        Span::styled("License: ", bold_style),
        Span::raw(metadata.license.as_deref().unwrap_or("Unknown")),
    ]);

    let description = Line::from_iter([
        Span::styled("Description: ", bold_style),
        Span::raw(metadata.description.as_deref().unwrap_or("Unknown")),
    ]);

    let mut lines = vec![maintainer, version, repository, license, description];

    let mut homepage = homepage_lines(metadata, bold_style);
    lines.append(&mut homepage);

    if let Some(size) = size {
        let size = Line::from_iter([
            Span::styled("Size: ", bold_style),
            Span::raw(human_size(size)),
        ]);
        lines.push(size);
    }

    if let Some(build_time) = build_time {
        let build_time = Line::from_iter([
            Span::styled("Build time: ", bold_style),
            Span::raw(build_time.to_string()),
        ]);

        lines.push(build_time);
    }

    let paragraph = Paragraph::new(lines)
        .block(
            Block::default()
                .title("Metadata")
                .borders(Borders::ALL)
                .border_style(Theme::block()),
        )
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}
