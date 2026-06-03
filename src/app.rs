use color_eyre::eyre::Ok;
use ratatui::{
    Frame,
    crossterm::event::{self, Event, KeyCode},
    layout::{Constraint, Layout},
    style::Stylize,
    text::Text,
    widgets::Block,
};

use crate::{
    gentoo::portage::Portage,
    screens::{
        dashboard::DashboardScreen, installed_packages::InstalledPackagesScreen, screen::Screen,
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

#[derive(Debug)]
pub struct App {
    portage: Portage,

    installed_package_screen: InstalledPackagesScreen,
    dashboard_screen: DashboardScreen,

    pub view: ViewState,
}

impl App {
    pub fn new(portage: Portage) -> Self {
        App {
            portage,

            dashboard_screen: DashboardScreen::default(),
            installed_package_screen: InstalledPackagesScreen::default(),

            view: ViewState::Dashboard,
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
                    .draw(frame, rest, &self.portage);
            }
            ViewState::Dashboard => {
                self.dashboard_screen.draw(frame, rest, &self.portage);
            }
            ViewState::AvailablePackages => {
                frame.render_widget(Text::from("NOT IMPLEMENTED"), rest);
            }
        }
    }

    pub fn update(&mut self) -> color_eyre::Result<bool> {
        if let Event::Key(key) = event::read()? {
            let signal = match self.view {
                crate::app::ViewState::InstalledPackages => {
                    self.installed_package_screen.update(key, &self.portage)
                }

                // Fallback, primarly while implementing
                _ => match key.code {
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

    pub fn cycle_current_tab(&mut self) {
        match self.view {
            ViewState::Dashboard => self.view = ViewState::InstalledPackages,
            ViewState::InstalledPackages => self.view = ViewState::AvailablePackages,
            ViewState::AvailablePackages => self.view = ViewState::Dashboard,
        }
    }
}
