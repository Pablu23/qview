use ratatui::{
    Frame,
    layout::Rect,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::{
    app::App,
    gentoo::InstalledPackage,
    theme::Theme,
    widgets::helpers::{homepage_lines, human_size},
};

pub fn render_package_metadata(frame: &mut Frame, area: Rect, pkg: Option<&InstalledPackage>) {
    let Some(pkg) = pkg else {
        return;
    };

    let bold_style = Theme::title(); //Style::default().add_modifier(Modifier::BOLD);
    let maintainer = Line::from_iter([
        Span::styled("Maintainer: ", bold_style),
        Span::raw(pkg.metadata.maintainer.as_deref().unwrap_or("Unknown")),
    ]);

    let version = Line::from_iter([
        Span::styled("Version: ", bold_style),
        Span::raw(&pkg.version),
    ]);

    let repository = Line::from_iter([
        Span::styled("Repository: ", bold_style),
        Span::raw(&pkg.metadata.repository),
    ]);

    let license = Line::from_iter([
        Span::styled("License: ", bold_style),
        Span::raw(pkg.metadata.license.as_deref().unwrap_or("Unknown")),
    ]);

    let description = Line::from_iter([
        Span::styled("Description: ", bold_style),
        Span::raw(pkg.metadata.description.as_deref().unwrap_or("Unknown")),
    ]);

    let size = Line::from_iter([
        Span::styled("Size: ", bold_style),
        Span::raw(human_size(pkg.size)),
    ]);

    let build_time = Line::from_iter([
        Span::styled("Build time: ", bold_style),
        Span::raw(pkg.build_time.to_string()),
    ]);

    let mut lines = vec![
        maintainer,
        version,
        repository,
        build_time,
        license,
        description,
        size,
    ];

    let mut homepage = homepage_lines(pkg, bold_style);
    lines.append(&mut homepage);

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
