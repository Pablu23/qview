use std::collections::HashSet;

use ratatui::{
    Frame,
    crossterm::event::KeyCode,
    layout::{Constraint, Layout, Rect},
    style::Color,
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListState, Paragraph},
};

use crate::{
    app::LoadingState,
    gentoo::package::{Package, PackageKey, PackageVersion},
    screens::screen::Screen,
    signal::{Event, Signal},
    theme::Theme,
    widgets::{
        package_metadata::render_package_metadata, search_popup::SearchPopup,
        use_flags::render_use_flags,
    },
};

#[derive(Debug)]
enum CurrentList {
    PackageList,
    VariantList,
}

#[derive(Debug)]
pub struct AvailablePackagesScreen {
    chosen_list: CurrentList,

    pkg_list: Option<List<'static>>,
    pkg_list_state: ListState,

    variant_list_state: ListState,

    loading_state: LoadingState,

    search_popup: SearchPopup,
}

fn version_to_variant(pkg_version: &PackageVersion) -> String {
    format!(
        "{}::{}",
        pkg_version.version, pkg_version.metadata.repository
    )
}

impl AvailablePackagesScreen {
    fn render_packages(&mut self, frame: &mut Frame, area: Rect) {
        // might not need to be own function
        if let Some(list) = &self.pkg_list {
            frame.render_stateful_widget(list, area, &mut self.pkg_list_state);
        }
    }

    fn render_package_variants(
        &mut self,
        frame: &mut Frame,
        area: Rect,
        variants: Vec<&PackageVersion>,
    ) {
        // TODO: Put this into screen, instead of rebuilding the list every frame
        let list = List::new(variants.iter().map(|v| version_to_variant(v)))
            .style(Color::White)
            .highlight_style(Theme::selected())
            .highlight_symbol("> ")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Theme::block())
                    .title("Variants"),
            );

        frame.render_stateful_widget(list, area, &mut self.variant_list_state);
    }

    fn cycle_list(&mut self) {
        self.variant_list_state.select(Some(0));

        self.chosen_list = match self.chosen_list {
            CurrentList::PackageList => CurrentList::VariantList,
            CurrentList::VariantList => CurrentList::PackageList,
        }
    }
}

impl Default for AvailablePackagesScreen {
    fn default() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        let mut variant_state = ListState::default();
        variant_state.select(Some(0));

        Self {
            chosen_list: CurrentList::PackageList,
            pkg_list: None,
            pkg_list_state: list_state,
            variant_list_state: variant_state,
            loading_state: LoadingState::Idle,
            search_popup: SearchPopup::default(),
        }
    }
}

impl Screen for AvailablePackagesScreen {
    fn draw(
        &mut self,
        frame: &mut ratatui::Frame,
        area: ratatui::prelude::Rect,
        repo: &crate::gentoo::Portage,
    ) {
        if !matches!(self.loading_state, LoadingState::Complete) {
            // TODO: Implement loading screen
            frame.render_widget(Text::styled("Not implemented yet :P", Theme::text()), area);
            return;
        }

        let [top, bottom] = area.layout(&Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(3),
        ]));

        let constraints = [Constraint::Percentage(50), Constraint::Percentage(50)];
        let [left, right] = top.layout(&Layout::horizontal(constraints));

        let [package_list, use_flags] = left.layout(&Layout::vertical(constraints));
        let [package_variants, metadata] = right.layout(&Layout::vertical(constraints));

        let packages: Vec<&PackageKey> =
            repo.available_packages().iter().map(|x| &x.atom).collect();

        let selected_package_key = packages.get(self.pkg_list_state.selected().unwrap_or(0));
        let pkg: Option<&Package> = match selected_package_key {
            Some(pkg_key) => repo.get_available_package(pkg_key),
            None => None,
        };

        self.render_packages(frame, package_list);

        if let Some(pkg) = pkg {
            let mut pkg_versions: Vec<&PackageVersion> = pkg.versions.iter().collect();
            pkg_versions.sort();
            pkg_versions.reverse();

            self.render_package_variants(frame, package_variants, pkg_versions);

            let selected_version = &pkg.versions[self.variant_list_state.selected().unwrap_or(0)];

            render_use_flags(
                frame,
                use_flags,
                selected_version.iuse.iter().collect(),
                // Get default active use flags for package, and globally defined ones
                HashSet::new(),
            );

            render_package_metadata(
                frame,
                metadata,
                &selected_version.version,
                &selected_version.metadata,
                None,
                None,
            );
        }

        let text = if self.search_popup.visible {
            "(esc) to quit search | (enter) to search".to_string()
        } else {
            "(q) to quit | (j) down | (k) up | (space) switch panel | (/) to search".to_string()
        };

        let key_hint = Span::styled(text, Theme::muted());

        let key_notes_footer = Paragraph::new(Line::from(key_hint)).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Theme::block()),
        );
        frame.render_widget(key_notes_footer, bottom);

        self.search_popup.draw(frame, area);
    }

    fn update(
        &mut self,
        event: &Event,
        repo: &crate::gentoo::Portage,
    ) -> Option<crate::signal::Signal> {
        match event {
            Event::KeyEvent(key) => {
                if self.search_popup.visible {
                    match key.code {
                        KeyCode::Esc => self.search_popup.toggle(),
                        KeyCode::Enter => {
                            let result = self.search_popup.search(
                                repo.available_packages().iter().map(|p| &p.atom).collect(),
                            );
                            self.search_popup.toggle();

                            if let Some(index) = result {
                                self.pkg_list_state.select(Some(index));
                            }
                        }
                        _ => {
                            self.search_popup.textarea.input_without_shortcuts(*key);
                            let result = self.search_popup.search(
                                repo.available_packages().iter().map(|p| &p.atom).collect(),
                            );

                            if let Some(index) = result {
                                self.pkg_list_state.select(Some(index));
                            }
                        }
                    }
                } else {
                    match key.code {
                        KeyCode::Char('q') => return Some(Signal::Quit),
                        KeyCode::Tab => return Some(Signal::CycleTab),

                        // TODO: This should cycle, also space isnt a good key for this
                        KeyCode::Char(' ') => self.cycle_list(),
                        KeyCode::Char('/') => self.search_popup.toggle(),

                        _ => match self.chosen_list {
                            CurrentList::PackageList => match key.code {
                                KeyCode::Char('j') => self.pkg_list_state.select_next(),
                                KeyCode::Char('k') => self.pkg_list_state.select_previous(),

                                _ => {}
                            },
                            CurrentList::VariantList => match key.code {
                                KeyCode::Char('j') => self.variant_list_state.select_next(),
                                KeyCode::Char('k') => self.variant_list_state.select_previous(),

                                _ => {}
                            },
                        },
                    }
                }
            }
            Event::LoadStateUpdate(loading_state) => match loading_state {
                LoadingState::Complete => {
                    let packages: Vec<String> = repo
                        .available_packages()
                        .iter()
                        .map(|x| &x.atom)
                        .map(|x| x.qualified_name())
                        .collect();

                    let list = List::new(packages)
                        .style(Color::White)
                        .highlight_style(Theme::selected())
                        .highlight_symbol("> ")
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .border_style(Theme::block())
                                .title("Available packages"),
                        );

                    self.pkg_list = Some(list);
                    self.loading_state = LoadingState::Complete
                }

                loading_state => self.loading_state = *loading_state,
            },
        }

        None
    }
}
