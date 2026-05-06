use ratatui::widgets::ListState;

use crate::gentoo::{Package, portage::Portage};

#[derive(Debug)]
#[allow(dead_code)]
pub struct App {
    portage: Portage,
    pub list_state: ListState,
}

#[allow(dead_code)]
impl App {
    pub fn new(portage: Portage) -> Self {
        App {
            portage,
            list_state: ListState::default().with_selected(Some(0)),
        }
    }

    pub fn installed_packages(&self) -> Vec<Package> {
        self.portage.installed_packages.clone()
    }
}
