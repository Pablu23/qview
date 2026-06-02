use color_eyre::eyre::Ok;
use ratatui::{
    Frame,
    crossterm::event::{self, Event},
    layout::{Constraint, Layout},
    style::Stylize,
    text::Text,
    widgets::Block,
};

use crate::{
    actions::{Signal, signal},
    gentoo::{InstalledPackage, portage::Portage},
    screens::{installed_packages::InstalledPackagesScreen, screen::Screen},
    theme::Theme,
};

#[derive(Debug)]
pub enum ViewState {
    Dashboard = 0,
    InstalledPackages = 1,
    AvailablePackages = 2,
}

#[derive(Debug)]
pub struct App {
    portage: Portage,

    installed_package_screen: InstalledPackagesScreen,

    // pub search_found: Option<bool>,
    // search_indexes: Option<Vec<usize>>,
    // pub search_indexes_len: Option<usize>,
    // pub current_search_index: Option<usize>,
    pub view: ViewState,
}

impl App {
    pub fn new(portage: Portage) -> Self {
        App {
            portage,

            installed_package_screen: InstalledPackagesScreen::default(),

            view: ViewState::InstalledPackages,
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(Block::default().bg(Theme::BG), frame.area());

        let layout = Layout::vertical(vec![Constraint::Length(3), Constraint::Fill(1)]);
        let [tab_bar, rest] = frame.area().layout(&layout);

        // render_tab(frame, tab_bar, &app.view);

        match self.view {
            crate::app::ViewState::InstalledPackages => {
                self.installed_package_screen
                    .draw(frame, rest, &self.portage)
            }
            _ => frame.render_widget(Text::from("NOT IMPLEMENTED"), rest),
        }
    }

    pub fn update(&mut self) -> color_eyre::Result<bool> {
        if let Event::Key(key) = event::read()? {
            let signal = match self.view {
                crate::app::ViewState::InstalledPackages => {
                    self.installed_package_screen.update(key)
                }

                _ => None,
            };

            if let Some(signal) = signal {
                match signal {
                    Signal::Quit => return Ok(true),
                    Signal::CycleTab => todo!(),
                }
            }
        }

        Ok(false)
    }

    pub fn world_count(&self) -> usize {
        self.portage.world_packages.len()
    }

    pub fn installed_packages(&self) -> &[InstalledPackage] {
        &self.portage.installed_packages
    }

    // pub fn current_package(&self) -> &InstalledPackage {
    //     let selected_package_index = self.list_state.selected().unwrap_or_default();
    //     &self.portage.installed_packages[selected_package_index]
    // }

    pub fn total_installed_size(&self) -> usize {
        self.portage.installed_packages.iter().map(|p| p.size).sum()
    }

    // pub fn toggle_search_window(&mut self) {
    //     self.showing_search_window = !self.showing_search_window;
    //     self.textarea.clear();
    //     self.search_found = None;
    // }

    pub fn cycle_current_tab(&mut self) {
        match self.view {
            ViewState::Dashboard => self.view = ViewState::InstalledPackages,
            ViewState::InstalledPackages => self.view = ViewState::AvailablePackages,
            ViewState::AvailablePackages => self.view = ViewState::Dashboard,
        }
    }

    // pub fn search(&mut self) -> bool {
    //     let indexes: Vec<usize> = self
    //         .portage
    //         .installed_packages
    //         .iter()
    //         .enumerate()
    //         .filter(|(_, s)| {
    //             s.atom
    //                 .qualified_name()
    //                 .to_lowercase()
    //                 .contains(&self.textarea.lines()[0].to_lowercase())
    //         })
    //         .map(|(i, _)| i)
    //         .collect();
    //
    //     if let Some(index) = indexes.first() {
    //         self.list_state.select(Some(*index));
    //         self.search_indexes_len = Some(indexes.len());
    //         self.search_indexes = Some(indexes);
    //         self.current_search_index = Some(0);
    //
    //         return true;
    //     }
    //
    //     self.search_indexes = None;
    //
    //     false
    // }
    //
    // pub fn next_search(&mut self) {
    //     let Some(mut current_index) = self.current_search_index else {
    //         return;
    //     };
    //
    //     let Some(search_indexes) = &self.search_indexes else {
    //         return;
    //     };
    //
    //     if current_index + 1 >= search_indexes.len() {
    //         current_index = 0;
    //     } else {
    //         current_index += 1;
    //     }
    //
    //     let list_index = search_indexes[current_index];
    //
    //     self.list_state.select(Some(list_index));
    //     self.current_search_index = Some(current_index);
    // }
    //
    // pub fn prev_search(&mut self) {
    //     let Some(mut current_index) = self.current_search_index else {
    //         return;
    //     };
    //
    //     let Some(search_indexes) = &self.search_indexes else {
    //         return;
    //     };
    //
    //     if current_index == 0 {
    //         current_index = search_indexes.len() - 1;
    //     } else {
    //         current_index -= 1;
    //     }
    //
    //     let list_index = search_indexes[current_index];
    //
    //     self.list_state.select(Some(list_index));
    //     self.current_search_index = Some(current_index);
    // }
}
