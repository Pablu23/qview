use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout},
    style::Stylize,
    text::Text,
    widgets::Block,
};

use crate::{
    background_loader::PackageLoader,
    gentoo::portage::Portage,
    screens::{
        available_packages::AvailablePackagesScreen, dashboard::DashboardScreen,
        installed_packages::InstalledPackagesScreen, screen::Screen,
    },
    signal::Signal,
    theme::Theme,
    widgets::tabs::render_tab,
};

#[derive(Debug)]
pub enum ViewState {
    Dashboard = 0,
    InstalledPackages = 1,
    AvailablePackages = 2,
}

#[derive(Debug, Clone, Copy)]
pub enum LoadingState {
    Idle,
    Loading,
    Complete,
    Error,
}

#[derive(Debug)]
pub struct App {
    portage: Portage,

    dashboard_screen: DashboardScreen,
    installed_package_screen: InstalledPackagesScreen,
    available_package_screen: AvailablePackagesScreen,

    pub view: ViewState,

    available_loader: Option<PackageLoader>,
    pub loading_state: LoadingState,
    pub loading_error: Option<String>,
}

impl App {
    pub fn new(portage: Portage) -> Self {
        App {
            portage,

            dashboard_screen: DashboardScreen::default(),
            installed_package_screen: InstalledPackagesScreen::default(),
            available_package_screen: AvailablePackagesScreen::default(),

            view: ViewState::Dashboard,

            available_loader: None,
            loading_state: LoadingState::Idle,
            loading_error: None,
        }
    }

    pub fn draw(&mut self, frame: &mut Frame) {
        frame.render_widget(Block::default().bg(Theme::BG), frame.area());

        let layout = Layout::vertical(vec![Constraint::Length(3), Constraint::Fill(1)]);
        let [tab_bar, rest] = frame.area().layout(&layout);

        render_tab(frame, tab_bar, &self.view);

        match self.view {
            ViewState::InstalledPackages => {
                self.installed_package_screen
                    .draw(frame, rest, &self.portage, &self.loading_state);
            }
            ViewState::Dashboard => {
                self.dashboard_screen
                    .draw(frame, rest, &self.portage, &self.loading_state);
            }
            ViewState::AvailablePackages => {
                self.available_package_screen
                    .draw(frame, rest, &self.portage, &self.loading_state);
            }
        }
    }

    pub fn update(&mut self) -> color_eyre::Result<bool> {
        if let Ok(Event::Key(key)) = event::read() {
            let signal = match self.view {
                crate::app::ViewState::InstalledPackages => {
                    self.installed_package_screen.update(key, &self.portage)
                }
                ViewState::AvailablePackages => {
                    self.available_package_screen.update(key, &self.portage)
                }

                // Fallback, primarly while implementing
                _ => match key.code {
                    KeyCode::Char('r') => {
                        if !matches!(self.loading_state, LoadingState::Loading) {
                            self.start_available_packages_load();
                        }
                        None
                    }
                    KeyCode::Char('q') => Some(Signal::Quit),
                    KeyCode::Tab => Some(Signal::CycleTab),
                    _ => None,
                },
            };

            if let Some(signal) = signal {
                match signal {
                    Signal::Quit => return Ok(true),
                    Signal::CycleTab => self.cycle_current_tab(),
                }
            }
        }

        Ok(false)
    }

    pub fn poll_available_packages(&mut self) {
        if let Some(loader) = &self.available_loader {
            if let Some(msg) = loader.try_recv() {
                match msg {
                    crate::background_loader::LoaderMessage::Loading => {
                        self.loading_state = LoadingState::Loading;
                    }
                    crate::background_loader::LoaderMessage::Complete(packages) => {
                        self.portage.available = packages;
                        self.loading_state = LoadingState::Complete;

                        self.available_loader = None;
                    }
                    crate::background_loader::LoaderMessage::Error(e) => {
                        self.loading_error = Some(e);
                        self.loading_state = LoadingState::Error;

                        self.available_loader = None;
                    }
                }
            }
        }
    }

    pub fn start_available_packages_load(&mut self) {
        self.available_loader = Some(PackageLoader::spawn());
        self.loading_state = LoadingState::Loading;
    }

    pub fn cycle_current_tab(&mut self) {
        match self.view {
            ViewState::Dashboard => self.view = ViewState::InstalledPackages,
            ViewState::InstalledPackages => self.view = ViewState::AvailablePackages,
            ViewState::AvailablePackages => self.view = ViewState::Dashboard,
        }
    }
}
