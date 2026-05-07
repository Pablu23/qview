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
    search_indexes: Option<Vec<usize>>,
    pub search_indexes_len: Option<usize>,
    pub current_search_index: Option<usize>,
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
            search_indexes: None,
            current_search_index: None,
            search_indexes_len: None,
        }
    }

    pub fn installed_packages(&self) -> Vec<Package> {
        self.portage.installed_packages.clone()
    }

    pub fn current_package(&self) -> Package {
        let selected_package_index = match self.list_state.selected() {
            Some(selected) => selected,
            None => 0,
        };

        self.portage.installed_packages[selected_package_index].clone()
    }

    pub fn toggle_search_window(&mut self) {
        self.showing_search_window = !self.showing_search_window;
        self.textarea.clear();
        self.search_found = None;
    }

    pub fn search(&mut self) -> bool {
        let indexes: Vec<usize> = self
            .portage
            .installed_packages
            .iter()
            .enumerate()
            .filter(|(_, s)| {
                s.name
                    .to_lowercase()
                    .contains(&self.textarea.lines()[0].to_lowercase())
            })
            .map(|(i, _)| i)
            .collect();

        if let Some(index) = indexes.first() {
            self.list_state.select(Some(index.clone()));
            self.search_indexes_len = Some(indexes.len());
            self.search_indexes = Some(indexes);
            self.current_search_index = Some(0);
            return true;
        }

        self.search_indexes = None;
        return false;
    }

    pub fn next_search(&mut self) {
        let mut current_index = match self.current_search_index {
            Some(i) => i,
            None => return,
        };

        let search_indexes = match &self.search_indexes {
            Some(i) => i,
            None => return,
        };

        if current_index + 1 >= search_indexes.len() {
            current_index = 0;
        } else {
            current_index += 1;
        }

        let list_index = search_indexes[current_index];

        self.list_state.select(Some(list_index));
        self.current_search_index = Some(current_index)
    }

    pub fn prev_search(&mut self) {
        let mut current_index = match self.current_search_index {
            Some(i) => i,
            None => return,
        };

        let search_indexes = match &self.search_indexes {
            Some(i) => i,
            None => return,
        };

        if current_index == 0 {
            current_index = search_indexes.len() - 1;
        } else {
            current_index -= 1;
        }

        let list_index = search_indexes[current_index];

        self.list_state.select(Some(list_index));
        self.current_search_index = Some(current_index)
    }
}
