use ratatui::{
    Frame,
    layout::Rect,
    style::{Modifier, Style},
    widgets::{Block, Borders, Clear},
};
use ratatui_textarea::TextArea;

use crate::{gentoo::package::PackageKey, theme::Theme, widgets::helpers::search_popup_rect};

#[derive(Debug)]
pub struct SearchPopup {
    pub(crate) visible: bool,
    pub(crate) textarea: TextArea<'static>,

    matches: Vec<usize>,
    selected_match: Option<usize>,
}

impl Default for SearchPopup {
    fn default() -> Self {
        let mut search_popup = Self {
            visible: bool::default(),
            textarea: TextArea::default(),

            matches: vec![],
            selected_match: None,
        };

        search_popup.textarea.set_style(Theme::muted());
        search_popup
            .textarea
            .set_cursor_line_style(Style::default());

        search_popup
    }
}

impl SearchPopup {
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
        self.textarea.clear();
    }

    pub fn search(&mut self, packages: Vec<&PackageKey>) -> Option<usize> {
        let indexes: Vec<usize> = packages
            .iter()
            .enumerate()
            .filter(|(_, s)| {
                s.qualified_name()
                    .to_lowercase()
                    .contains(&self.textarea.lines()[0].to_lowercase())
            })
            .map(|(i, _)| i)
            .collect();

        if indexes.is_empty() {
            self.matches = vec![];
            self.selected_match = None;
            None
        } else {
            self.matches = indexes;
            self.selected_match = Some(0);
            return Some(self.matches[0]);
        }
    }

    pub fn current_match(&self) -> Option<usize> {
        if let Some(index) = self.selected_match {
            self.matches.get(index).copied()
        } else {
            None
        }
    }

    pub fn next_match(&mut self) -> Option<usize> {
        if let Some(index) = self.selected_match {
            self.selected_match = Some((index + 1) % self.matches.len());
        }

        self.current_match()
    }

    pub fn previous_match(&mut self) -> Option<usize> {
        if let Some(index) = self.selected_match {
            self.selected_match = Some((index + self.matches.len() - 1) % self.matches.len());
        }

        self.current_match()
    }

    pub fn match_status(&self) -> Option<(usize, usize)> {
        self.selected_match
            .map(|selected| (selected + 1, self.matches.len()))
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect) {
        if self.visible {
            let title = match self.match_status() {
                Some((current, total)) => format!("Search {current}/{total}"),
                None => "Search".to_string(),
            };

            let mut popup_block = Block::default()
                .title(title)
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(Theme::block())
                .style(Style::default().bg(Theme::BG));

            if self.selected_match.is_some() {
                popup_block = popup_block.border_style(Theme::success());
            } else {
                popup_block = popup_block.border_style(Theme::error());
            }

            self.textarea.set_block(popup_block);

            let area = search_popup_rect(70, area);

            frame.render_widget(
                Block::default().style(Style::default().bg(Theme::BG).add_modifier(Modifier::DIM)),
                frame.area(),
            );
            frame.render_widget(Clear, area);
            frame.render_widget(Block::default().style(Style::default().bg(Theme::BG)), area);
            frame.render_widget(&self.textarea, area);
        }
    }
}
