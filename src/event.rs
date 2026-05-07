use std::io;

use ratatui::crossterm::event::{self, Event, KeyCode};

use crate::app::App;

pub fn handle_event(app: &mut App) -> io::Result<bool> {
    if let Event::Key(key) = event::read()? {
        if !app.showing_search_window {
            match key.code {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Char('j') => app.list_state.select_next(),
                KeyCode::Char('k') => app.list_state.select_previous(),
                KeyCode::Char('/') => app.toggle_search_window(),
                _ => return Ok(false),
            }
        } else {
            match key.code {
                KeyCode::Esc => app.toggle_search_window(),
                KeyCode::Enter => {
                    app.search();
                    app.toggle_search_window();
                    ()
                }
                _ => {
                    app.textarea.input_without_shortcuts(key);
                    app.search_found = Some(app.search());
                    ()
                }
            }
        }
    }

    Ok(false)
}
