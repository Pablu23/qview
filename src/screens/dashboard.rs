use ratatui::{
    crossterm::event::KeyCode,
    layout::{Alignment, Constraint, Layout, Spacing},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph},
};

use crate::{actions::Signal, screens::screen::Screen, theme::Theme, widgets::helpers::human_size};

#[derive(Debug, Default)]
pub struct DashboardScreen {}

fn create_stats<'a>(title: &'a str, value: &'a str) -> Paragraph<'a> {
    Paragraph::new(vec![
        Line::from(Span::styled(title, Theme::title())),
        Line::from(""),
        Line::from(value),
    ])
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Theme::block())
            .merge_borders(ratatui::symbols::merge::MergeStrategy::Exact),
    )
    .style(Theme::text())
    .alignment(Alignment::Center)
}

impl Screen for DashboardScreen {
    fn draw(
        &mut self,
        frame: &mut ratatui::Frame,
        area: ratatui::prelude::Rect,
        repo: &crate::gentoo::Portage,
    ) {
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
            Constraint::Min(15),
            Constraint::Fill(1),
            Constraint::Length(3),
        ];

        let layout = Layout::vertical(constraints);

        let [logo_top, stats, _fill, key_hints] = area.layout(&layout);

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

        let logo = Paragraph::new(Text::styled(logo, Theme::title()));
        frame.render_widget(logo, logo_top);

        let keys = Paragraph::new(Span::styled(
            "(q) to quit | (tab) to switch tabs",
            Theme::muted(),
        ))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Theme::block()),
        );
        frame.render_widget(keys, key_hints);

        let installed_packages_len = repo.installed_packages.len().to_string();
        let installed_packages =
            create_stats("Installed Packages", installed_packages_len.as_str());
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

        let world_package_count = repo.world_packages.len().to_string();
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

        let installed_size = human_size(repo.total_installed_size());

        let installed_size = create_stats("Installed Size", &installed_size);
        let distfiles_cache = create_stats("Distfiles Cache", "COMING SOON");
        let portage_news = create_stats("Portage News", "COMING SOON");

        frame.render_widget(installed_size, splits[0]);
        frame.render_widget(distfiles_cache, splits[1]);
        frame.render_widget(portage_news, splits[2]);
    }

    fn update(
        &mut self,
        key: ratatui::crossterm::event::KeyEvent,
        _: &crate::gentoo::Portage,
    ) -> Option<crate::actions::Signal> {
        match key.code {
            KeyCode::Char('q') => Some(Signal::Quit),
            KeyCode::Tab => Some(Signal::CycleTab),

            _ => None,
        }
    }
}
