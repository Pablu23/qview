use std::io;

use ratatui::crossterm::event::{self, Event, KeyCode};

use crate::app::App;

pub fn handle_event(app: &mut App) -> io::Result<bool> {
    if let Event::Key(key) = event::read()? {
        match key.code {
            KeyCode::Char('q') => return Ok(true),
            KeyCode::Char('j') => app.list_state.select_next(),
            KeyCode::Char('k') => app.list_state.select_previous(),
            _ => return Ok(false),
        }
    }

    Ok(false)
}
