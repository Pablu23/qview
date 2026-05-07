use ratatui::widgets::ListState;
use ratatui_textarea::TextArea;

use crate::gentoo::{Package, portage::Portage};

#[derive(Debug)]
#[allow(dead_code)]
pub struct App<'a> {
    portage: Portage,
    pub list_state: ListState,
    pub showing_search_window: bool,
    pub textarea: TextArea<'a>,
    pub search_found: Option<bool>,
}

#[allow(dead_code)]
impl<'a> App<'a> {
    pub fn new(portage: Portage) -> Self {
        App {
            portage,
            list_state: ListState::default().with_selected(Some(0)),
            showing_search_window: false,
            textarea: TextArea::default(),
            search_found: None,
        }
    }

    pub fn installed_packages(&self) -> Vec<Package> {
        self.portage.installed_packages.clone()
    }

    pub fn toggle_search_window(&mut self) {
        self.showing_search_window = !self.showing_search_window;
        self.textarea.clear();
        self.search_found = None;
    }

    pub fn search(&mut self) -> bool {
        let index = self.portage.installed_packages.iter().position(|s| {
            s.name
                .to_lowercase()
                .contains(&self.textarea.lines()[0].to_lowercase())
        });

        if let Some(index) = index {
            self.list_state.select(Some(index));
            return true;
        }

        return false;
    }
}
