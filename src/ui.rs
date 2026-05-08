use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Spacing},
    prelude::Rect,
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph, Wrap},
};

use crate::{app::App, gentoo::Package};

pub fn ui(frame: &mut Frame, app: &mut App) {
    match app.view {
        crate::app::ViewState::Dashboard => render_dashboard(frame, app),
        crate::app::ViewState::InstalledPackages => render_installed_packages(frame, app),
    }
}

fn render_dashboard(frame: &mut Frame, app: &mut App) {
    let logo = "
 ██████╗ ██╗   ██╗██╗███████╗██╗    ██╗
██╔═══██╗██║   ██║██║██╔════╝██║    ██║
██║   ██║██║   ██║██║█████╗  ██║ █╗ ██║
██║▄▄ ██║╚██╗ ██╔╝██║██╔══╝  ██║███╗██║
╚██████╔╝ ╚████╔╝ ██║███████╗╚███╔███╔╝
 ╚══▀▀═╝   ╚═══╝  ╚═╝╚══════╝ ╚══╝╚══╝
        gentoo portage dashboard
        ";

    let constraints = [
        Constraint::Length(9),
        Constraint::Percentage(30),
        Constraint::Fill(1),
        Constraint::Length(3),
    ];

    let layout = Layout::vertical(constraints);

    let [logo_top, stats, _fill, key_hints] = frame.area().layout(&layout);

    let [stats_top, stats_middle, stats_bottom] = Layout::vertical([
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ])
    .spacing(Spacing::Overlap(1))
    .areas(stats);

    let splits = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ])
    .spacing(Spacing::Overlap(1))
    .split(stats_top);

    let logo = Paragraph::new(logo);
    frame.render_widget(logo, logo_top);

    let keys = Paragraph::new("(q) to quit | (tab) to switch tabs".fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL));
    frame.render_widget(keys, key_hints);

    let installed_packages_len = app.installed_packages().len().to_string();
    let installed_packages = create_stats("Installed Packages", installed_packages_len.as_str());
    let global_use_flags = create_stats("Global USE Flags", "COMING SOON");
    let repository_state = create_stats("Repository state", "COMING SOON");
    frame.render_widget(installed_packages, splits[0]);
    frame.render_widget(global_use_flags, splits[1]);
    frame.render_widget(repository_state, splits[2]);

    let splits = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ])
    .spacing(Spacing::Overlap(1))
    .split(stats_middle);

    let world_package_count = app.world_count().to_string();
    let world_packages = create_stats("World Packages", &world_package_count);
    let pending_updates = create_stats("Pending Updates", "COMING SOON");
    let last_emerge = create_stats("Last Emerge", "COMING SOON");

    frame.render_widget(world_packages, splits[0]);
    frame.render_widget(pending_updates, splits[1]);
    frame.render_widget(last_emerge, splits[2]);

    let splits = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Fill(1),
        Constraint::Fill(1),
    ])
    .spacing(Spacing::Overlap(1))
    .split(stats_bottom);

    let installed_size = human_size(app.total_installed_size());

    let installed_size = create_stats("Installed Size", &installed_size);
    let distfiles_cache = create_stats("Distfiles Cache", "COMING SOON");
    let portage_news = create_stats("Portage News", "COMING SOON");

    frame.render_widget(installed_size, splits[0]);
    frame.render_widget(distfiles_cache, splits[1]);
    frame.render_widget(portage_news, splits[2]);
}

fn human_size(bytes: usize) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];

    let mut size = bytes as f64;
    let mut unit = 0;

    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }

    if unit == 0 {
        format!("{} {}", bytes, UNITS[unit])
    } else {
        format!("{:.1} {}", size, UNITS[unit])
    }
}

fn create_stats<'a>(title: &'a str, value: &'a str) -> Paragraph<'a> {
    Paragraph::new(vec![
        Line::from(Span::styled(title, Style::default().bold())),
        Line::from(""),
        Line::from(value),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .merge_borders(ratatui::symbols::merge::MergeStrategy::Exact),
    )
    .alignment(Alignment::Center)
}

fn render_installed_packages(frame: &mut Frame, app: &mut App) {
    let constraints = [Constraint::Fill(1), Constraint::Length(3)];

    let layout = Layout::vertical(constraints);
    let [top, bottom] = frame.area().layout(&layout);

    let split =
        Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(top);

    let split_vert =
        Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)]).split(split[1]);

    render_packages(frame, split[0], app);
    render_use_flags(frame, split_vert[0], app);
    render_package_metadata(frame, split_vert[1], app);

    let text = match app.showing_search_window {
        false => {
            let mut main_key_hint = "(q) to quit | (j) down | (k) up | (/) to search".to_string();
            if let (Some(current), Some(total)) = (app.current_search_index, app.search_indexes_len)
            {
                main_key_hint.push_str(&format!(
                    " | (n) for next search | (N) for previous search | Searches found: {} / {}",
                    current + 1,
                    total
                ));
            }

            main_key_hint
        }
        true => "(esc) to quit search | (enter) to search".to_string(),
    };

    let key_hint = Span::styled(text, Style::default().fg(Color::Yellow));

    let key_notes_footer =
        Paragraph::new(Line::from(key_hint)).block(Block::default().borders(Borders::ALL));
    frame.render_widget(key_notes_footer, bottom);

    if app.showing_search_window {
        let mut popup_block = Block::default()
            .title("Search")
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded);

        if let Some(found) = app.search_found {
            if found {
                popup_block = popup_block.border_style(Style::default().fg(Color::Green))
            } else {
                popup_block = popup_block.border_style(Style::default().fg(Color::Red))
            }
        }

        app.textarea.set_block(popup_block);
        app.textarea.set_style(Style::default().fg(Color::Yellow));
        app.textarea.set_cursor_line_style(Style::default());

        let area = search_popup_rect(70, frame.area());
        frame.render_widget(Clear, area);
        frame.render_widget(&app.textarea, area);
    }
}

fn render_package_metadata(frame: &mut Frame, area: Rect, app: &mut App) {
    let pkg = app.current_package();

    let bold_style = Style::default().add_modifier(Modifier::BOLD);
    let maintainer = Line::from_iter([
        Span::styled("Maintainer: ", bold_style),
        Span::raw(pkg.maintainer.as_deref().unwrap_or("Unknown")),
    ]);

    let version = Line::from_iter([
        Span::styled("Version: ", bold_style),
        Span::raw(&pkg.version),
    ]);

    let repository = Line::from_iter([
        Span::styled("Repository: ", bold_style),
        Span::raw(&pkg.repository),
    ]);

    let license = Line::from_iter([
        Span::styled("License: ", bold_style),
        Span::raw(pkg.license.as_deref().unwrap_or("Unknown")),
    ]);

    let description = Line::from_iter([
        Span::styled("Description: ", bold_style),
        Span::raw(pkg.description.as_deref().unwrap_or("Unknown")),
    ]);

    let size = Line::from_iter([
        Span::styled("Size: ", bold_style),
        Span::raw(human_size(pkg.size)),
    ]);

    let mut lines = vec![maintainer, version, repository, license, description, size];

    let mut homepage = homepage_lines(&pkg, bold_style);
    lines.append(&mut homepage);

    let paragraph = Paragraph::new(lines)
        .block(Block::default().title("Metadata").borders(Borders::ALL))
        .wrap(Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

fn homepage_lines(pkg: &Package, bold_style: Style) -> Vec<Line<'_>> {
    let mut lines = Vec::new();
    lines.push(Line::from(vec![
        Span::styled("Homepage: ", bold_style),
        Span::raw(""),
    ]));

    if let Some(homepages) = &pkg.homepage {
        for url in homepages.iter().filter(|s| !s.is_empty()) {
            lines.push(Line::from(vec![
                Span::raw("  - "), // indentation
                Span::styled(
                    url,
                    Style::default()
                        .fg(Color::LightBlue)
                        .add_modifier(Modifier::UNDERLINED),
                ),
            ]));
        }
    } else {
        lines.push(Line::from(vec![
            Span::raw("  "),
            Span::styled("Unknown", Style::default().fg(Color::DarkGray)),
        ]));
    }

    lines
}

fn search_popup_rect(percent_x: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(10), Constraint::Length(3)])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

fn render_packages(frame: &mut Frame, area: Rect, app: &mut App) {
    let items = app.installed_packages().into_iter().map(|x| x.name.clone());
    let list = List::new(items)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ")
        .block(
            Block::new()
                .borders(Borders::ALL)
                .title("Installed packages"),
        );

    frame.render_stateful_widget(list, area, &mut app.list_state);
}

fn render_use_flags(frame: &mut Frame, area: Rect, app: &mut App) {
    let selected_package = app.current_package();

    let items = selected_package.use_flags.iter().map(|x| {
        let color = if x.enabled {
            Color::Green
        } else if x.default {
            Color::Blue
        } else {
            Color::Red
        };

        let text_style = if x.default {
            Style::default().add_modifier(Modifier::ITALIC).fg(color)
        } else {
            Style::default().fg(color)
        };

        ListItem::new(x.name.clone()).style(text_style)
    });
    let list = List::new(items)
        .style(Color::White)
        .scroll_padding(1)
        .direction(ratatui::widgets::ListDirection::TopToBottom)
        .block(Block::new().borders(Borders::ALL).title("USE Flags"));

    let mut list_state = ListState::default();
    list_state.select_first();

    frame.render_stateful_widget(list, area, &mut list_state);
}
