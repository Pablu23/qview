use ratatui::{
    Frame,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListState, Paragraph},
};
use ratatui_textarea::{Key, TextArea};

use crate::{
    actions::Signal,
    gentoo::Portage,
    screens::screen::Screen,
    theme::Theme,
    widgets::{
        helpers::search_popup_rect, package_metadata::render_package_metadata,
        use_flags::render_use_flags,
    },
};

#[derive(Debug, Default)]
struct SearchPopup {
    pub(crate) visible: bool,
    pub(crate) textarea: TextArea<'static>,
}

#[derive(Debug, Default)]
pub struct InstalledPackagesScreen {
    list_state: ListState,

    search_popup: SearchPopup,
}

impl InstalledPackagesScreen {
    fn render_packages(&mut self, frame: &mut Frame, area: Rect, repo: &Portage) {
        // let items: Vec<String> = repo
        //     .installed_packages()
        //     .iter()
        //     .map(|x| x.atom.qualified_name())
        //     .collect();
        let items: Vec<String> = vec![];

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

impl Screen for InstalledPackagesScreen {
    fn draw(&mut self, frame: &mut Frame, area: Rect, repo: &Portage) {
        let constraints = [Constraint::Fill(1), Constraint::Length(3)];

        let layout = Layout::vertical(constraints);
        let [top, bottom] = area.layout(&layout);

        let split =
            Layout::horizontal([Constraint::Percentage(50), Constraint::Percentage(50)]).split(top);

        let split_vert = Layout::vertical([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(split[1]);

        self.render_packages(frame, split[0], repo);
        render_use_flags(frame, split_vert[0], &repo.installed_packages[0]);
        render_package_metadata(frame, split_vert[1], &repo.installed_packages[0]);

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

            // if let Some(found) = app.search_found {
            //     if found {
            //         popup_block = popup_block.border_style(Theme::success());
            //     } else {
            //         popup_block = popup_block.border_style(Theme::error());
            //     }
            // }

            // TODO: Only do this in init, not every time
            self.search_popup.textarea.set_block(popup_block);
            self.search_popup.textarea.set_style(Theme::muted());
            self.search_popup
                .textarea
                .set_cursor_line_style(Style::default());

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

    fn update(&mut self, key: KeyEvent) -> Option<Signal> {
        if self.search_popup.visible {
            match key.code {
                KeyCode::Esc => todo!("Toggle search window"),
                KeyCode::Enter => todo!("Send search, get back matches"),
                _ => self.search_popup.textarea.input_without_shortcuts(key), //TODO: Run "search
                                                                              //logic" to get
                                                                              //jumping while typing,
            };
        } else {
            match key.modifiers {
                KeyModifiers::CONTROL => match key.code {
                    KeyCode::Char('d') => todo!("next page"),
                    KeyCode::Char('u') => todo!("prev page"),
                    _ => {}
                },

                _ => match key.code {
                    KeyCode::Char('q') => return Some(Signal::Quit),
                    KeyCode::Tab => return Some(Signal::CycleTab),
                    KeyCode::Char('j') => self.list_state.select_next(),
                    KeyCode::Char('k') => self.list_state.select_previous(),
                    KeyCode::Char('/') => todo!("Toggle search window"),
                    KeyCode::Char('n') => todo!("Next search"),
                    KeyCode::Char('N') => todo!("prev search"),

                    _ => {}
                },
            }
        }

        None
    }
}
