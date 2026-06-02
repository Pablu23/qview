use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListState, Paragraph},
};
use ratatui_textarea::TextArea;

use crate::{
    actions::Signal,
    gentoo::{InstalledPackage, Portage},
    screens::screen::Screen,
    theme::Theme,
    widgets::{
        helpers::search_popup_rect, package_metadata::render_package_metadata,
        use_flags::render_use_flags,
    },
};

#[derive(Debug)]
struct SearchPopup {
    pub(crate) visible: bool,
    pub(crate) textarea: TextArea<'static>,
    pub(crate) result: Option<usize>,
}

impl Default for SearchPopup {
    fn default() -> Self {
        let mut search_popup = Self {
            visible: bool::default(),
            textarea: TextArea::default(),
            result: Option::default(),
        };

        search_popup.textarea.set_style(Theme::muted());
        search_popup
            .textarea
            .set_cursor_line_style(Style::default());

        return search_popup;
    }
}

impl SearchPopup {
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
        self.textarea.clear();
    }

    pub fn search(&mut self, repo: &Portage) -> Option<usize> {
        let indexes: Vec<usize> = repo
            .installed_packages
            .iter()
            .enumerate()
            .filter(|(_, s)| {
                s.atom
                    .qualified_name()
                    .to_lowercase()
                    .contains(&self.textarea.lines()[0].to_lowercase())
            })
            .map(|(i, _)| i)
            .collect();

        if indexes.len() >= 1 {
            self.result = Some(indexes[0]);
            return Some(indexes[0]);
        } else {
            self.result = None;
            return None;
        }
    }
}

#[derive(Debug)]
pub struct InstalledPackagesScreen {
    list_state: ListState,

    search_popup: SearchPopup,
}

impl InstalledPackagesScreen {
    fn render_packages(&mut self, frame: &mut Frame, area: Rect, repo: &Portage) {
        // TODO: Put this into portage method
        let items: Vec<String> = repo
            .installed_packages
            .iter()
            .map(|x| x.atom.qualified_name())
            .collect();

        let list = List::new(items)
            .style(Color::White)
            .highlight_style(Theme::selected())
            .highlight_symbol("> ")
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .border_style(Theme::block())
                    .title("Installed packages"),
            );

        frame.render_stateful_widget(list, area, &mut self.list_state);
    }
}

impl Default for InstalledPackagesScreen {
    fn default() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        Self {
            list_state: list_state,
            search_popup: Default::default(),
        }
    }
}

impl Screen for InstalledPackagesScreen {
    fn draw(&mut self, frame: &mut Frame, area: Rect, repo: &Portage) {
        let constraints = [Constraint::Fill(1), Constraint::Length(3)];

        let layout = Layout::vertical(constraints);
        let [top, bottom] = area.layout(&layout);

        let split =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(top);

        let split_vert = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(split[1]);

        // TODO: Atleast this doesnt crash but its still weird, it should show some Info that this
        // package is completly unkown or placeholder data, or at best a warning
        let mut pkg: Option<&InstalledPackage> = None;
        if repo.installed_packages.len() > self.list_state.selected().unwrap_or(0) {
            pkg = Some(&repo.installed_packages[self.list_state.selected().unwrap_or(0)]);
        }

        self.render_packages(frame, split[0], repo);
        render_use_flags(frame, split_vert[0], pkg);
        render_package_metadata(frame, split_vert[1], pkg);

        let text = if self.search_popup.visible {
            "(esc) to quit search | (enter) to search".to_string()
        } else {
            let mut main_key_hint = "(q) to quit | (j) down | (k) up | (/) to search".to_string();
            // if let (Some(current), Some(total)) = (app.current_search_index, app.search_indexes_len)
            // {
            //     let _ = write!(
            //         main_key_hint,
            //         " | (n) for next search | (N) for previous search | Searches found: {} / {}",
            //         current + 1,
            //         total
            //     );
            // }

            main_key_hint
        };

        let key_hint = Span::styled(text, Theme::muted());

        let key_notes_footer = Paragraph::new(Line::from(key_hint)).block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Theme::block()),
        );
        frame.render_widget(key_notes_footer, bottom);

        if self.search_popup.visible {
            let mut popup_block = Block::default()
                .title("Search")
                .borders(Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(Theme::block())
                .style(Style::default().bg(Theme::BG));

            if let Some(_) = self.search_popup.result {
                popup_block = popup_block.border_style(Theme::success());
            } else {
                popup_block = popup_block.border_style(Theme::error());
            }

            self.search_popup.textarea.set_block(popup_block);

            let area = search_popup_rect(70, area);

            frame.render_widget(
                Block::default().style(Style::default().bg(Theme::BG).add_modifier(Modifier::DIM)),
                frame.area(),
            );
            frame.render_widget(Clear, area);
            frame.render_widget(Block::default().style(Style::default().bg(Theme::BG)), area);
            frame.render_widget(&self.search_popup.textarea, area);
        }
    }

    fn update(&mut self, key: KeyEvent, repo: &Portage) -> Option<Signal> {
        if self.search_popup.visible {
            match key.code {
                KeyCode::Esc => self.search_popup.toggle(),
                KeyCode::Enter => {
                    let result = self.search_popup.search(repo);
                    self.search_popup.toggle();

                    if let Some(index) = result {
                        self.list_state.select(Some(index));
                    }
                }
                _ => {
                    self.search_popup.textarea.input_without_shortcuts(key);
                    let result = self.search_popup.search(repo);

                    if let Some(index) = result {
                        self.list_state.select(Some(index));
                    }
                }
            };
        } else {
            match key.modifiers {
                KeyModifiers::CONTROL => match key.code {
                    KeyCode::Char('d') => self
                        .list_state
                        .select(Some(self.list_state.selected().unwrap_or(0) + 30)),
                    KeyCode::Char('u') => {
                        if let Some(selected) = self.list_state.selected() {
                            if selected < 30 {
                                self.list_state.select(Some(0));
                            } else {
                                self.list_state.select(Some(selected - 30));
                            }
                        }
                    }

                    _ => {}
                },

                _ => match key.code {
                    KeyCode::Char('q') => return Some(Signal::Quit),
                    KeyCode::Tab => return Some(Signal::CycleTab),
                    KeyCode::Char('j') => self.list_state.select_next(),
                    KeyCode::Char('k') => self.list_state.select_previous(),
                    KeyCode::Char('/') => self.search_popup.toggle(),
                    KeyCode::Char('n') => todo!("Next search"),
                    KeyCode::Char('N') => todo!("prev search"),

                    _ => {}
                },
            }
        }

        None
    }
}
