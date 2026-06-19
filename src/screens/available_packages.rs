use std::{
    collections::HashSet,
    time::{SystemTime, UNIX_EPOCH},
};

use ratatui::{
    Frame,
    crossterm::event::KeyCode,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Color,
    text::{Line, Span, Text},
    widgets::{Block, Borders, List, ListState, Paragraph},
};

use crate::{
    app::LoadingState,
    gentoo::{
        Portage,
        package::{Package, PackageKey, PackageVersion},
    },
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

    variant_list: Option<List<'static>>,
    variant_list_state: ListState,

    loading_state: LoadingState,

    search_popup: SearchPopup,

    selected_version: Option<PackageVersion>,
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

    fn render_package_variants(&mut self, frame: &mut Frame, area: Rect) {
        // TODO: Put this into screen, instead of rebuilding the list every frame

        if let Some(list) = &self.variant_list {
            frame.render_stateful_widget(list, area, &mut self.variant_list_state);
        }
    }

    fn cycle_list(&mut self) {
        self.variant_list_state.select(Some(0));

        self.chosen_list = match self.chosen_list {
            CurrentList::PackageList => CurrentList::VariantList,
            CurrentList::VariantList => CurrentList::PackageList,
        }
    }

    fn selected_package<'a>(&self, repo: &'a Portage) -> Option<&'a Package> {
        let packages: Vec<&PackageKey> =
            repo.available_packages().iter().map(|x| &x.atom).collect();

        let Some(selected_package_key) = packages.get(self.pkg_list_state.selected().unwrap_or(0))
        else {
            return None;
        };

        repo.get_available_package(selected_package_key)
    }

    fn selected_package_versions<'a>(&self, repo: &'a Portage) -> Option<Vec<&'a PackageVersion>> {
        let Some(pkg) = self.selected_package(repo) else {
            return None;
        };

        let mut pkg_versions: Vec<&PackageVersion> = pkg.versions.iter().collect();
        pkg_versions.sort();
        pkg_versions.reverse();

        Some(pkg_versions)
    }

    fn build_pkg_version_list(&mut self, repo: &Portage) {
        let Some(pkg_versions) = self.selected_package_versions(repo) else {
            return;
        };

        self.variant_list = Some(
            List::new(pkg_versions.iter().map(|v| version_to_variant(v)))
                .style(Color::White)
                .highlight_style(Theme::selected())
                .highlight_symbol("> ")
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Theme::block())
                        .title("Variants"),
                ),
        );

        self.selected_version = Some(pkg_versions[0].clone());
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

            variant_list: None,
            variant_list_state: variant_state,

            loading_state: LoadingState::Idle,

            search_popup: SearchPopup::default(),

            selected_version: None,
        }
    }
}

fn loading_spinner() -> &'static str {
    const FRAMES: [&str; 8] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧"];

    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis();

    FRAMES[((millis / 100) as usize) % FRAMES.len()]
}

impl Screen for AvailablePackagesScreen {
    fn draw(
        &mut self,
        frame: &mut ratatui::Frame,
        area: ratatui::prelude::Rect,
        _repo: &crate::gentoo::Portage,
    ) {
        if !matches!(self.loading_state, LoadingState::Complete) {
            let text = match self.loading_state {
                LoadingState::Loading => Line::from(vec![
                    Span::styled(loading_spinner(), Theme::muted()),
                    Span::raw(" "),
                    Span::styled("Loading available packages...", Theme::text()),
                ]),
                LoadingState::Error => Line::from(Span::styled(
                    "Failed to load available packages",
                    Theme::error(),
                )),
                LoadingState::Idle => Line::from(Span::styled(
                    "Waiting to load available packages...",
                    Theme::muted(),
                )),
                LoadingState::Complete => unreachable!(),
            };

            frame.render_widget(
                Paragraph::new(text).alignment(Alignment::Center).block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Theme::block())
                        .title("Available packages"),
                ),
                area,
            );
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

        self.render_packages(frame, package_list);

        if let Some(selected_version) = self.selected_version.clone() {
            self.render_package_variants(frame, package_variants);

            render_use_flags(
                frame,
                use_flags,
                selected_version.iuse.iter().collect(),
                // Get default active use flags for package, and globally defined ones
                &HashSet::new(),
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

    fn update(&mut self, event: &Event, repo: &Portage) -> Option<Signal> {
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
                                self.build_pkg_version_list(repo);
                            }
                        }
                        _ => {
                            self.search_popup.textarea.input_without_shortcuts(*key);
                            let result = self.search_popup.search(
                                repo.available_packages().iter().map(|p| &p.atom).collect(),
                            );

                            if let Some(index) = result {
                                self.pkg_list_state.select(Some(index));
                                self.build_pkg_version_list(repo);
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
                                KeyCode::Char('j') => {
                                    self.pkg_list_state.select_next();
                                    self.build_pkg_version_list(repo);
                                }
                                KeyCode::Char('k') => {
                                    self.pkg_list_state.select_previous();
                                    self.build_pkg_version_list(repo);
                                }

                                _ => {}
                            },
                            CurrentList::VariantList => match key.code {
                                KeyCode::Char('j') => {
                                    if let Some(pkg_versions) = self.selected_package_versions(repo)
                                    {
                                        self.variant_list_state.select_next();

                                        let index = self.variant_list_state.selected().unwrap_or(0);
                                        if index >= pkg_versions.len() {
                                            self.variant_list_state.select_previous();
                                        } else {
                                            self.selected_version =
                                                Some(pkg_versions[index].clone());
                                        }
                                    };
                                }
                                KeyCode::Char('k') => {
                                    if let Some(pkg_versions) = self.selected_package_versions(repo)
                                    {
                                        self.variant_list_state.select_previous();

                                        let index = self.variant_list_state.selected().unwrap_or(0);
                                        self.selected_version = Some(pkg_versions[index].clone());
                                    };
                                }

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
                    self.loading_state = LoadingState::Complete;

                    self.build_pkg_version_list(repo);
                }

                loading_state => self.loading_state = *loading_state,
            },
        }

        None
    }
}
