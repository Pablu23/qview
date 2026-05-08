use std::io;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyModifiers};

use crate::app::App;

pub fn handle_event(app: &mut App) -> io::Result<bool> {
    if let Event::Key(key) = event::read()? {
        if !app.showing_search_window {
            match key.code {
                KeyCode::Char('q') => return Ok(true),
                KeyCode::Char('j') => app.list_state.select_next(),
                KeyCode::Char('k') => app.list_state.select_previous(),
                KeyCode::Char('/') => app.toggle_search_window(),
                KeyCode::Char('n') => app.next_search(),
                KeyCode::Char('N') => app.prev_search(),

                KeyCode::Tab => app.cycle_current_tab(),

                KeyCode::Char('d') => match key.modifiers {
                    KeyModifiers::CONTROL => app
                        .list_state
                        .select(Some(app.list_state.selected().unwrap_or(0) + 30)),

                    _ => return Ok(false),
                },

                KeyCode::Char('u') => match key.modifiers {
                    KeyModifiers::CONTROL => {
                        if let Some(selected) = app.list_state.selected() {
                            if selected < 30 {
                                app.list_state.select(Some(0));
                            } else {
                                app.list_state.select(Some(selected - 30))
                            }
                        }
                    }

                    _ => return Ok(false),
                },

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
