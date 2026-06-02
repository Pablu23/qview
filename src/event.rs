use std::io;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};

use crate::app::App;

pub fn handle_event(app: &mut App) -> io::Result<bool> {
    if let Event::Key(key) = event::read()? {
        match app.view {
            crate::app::ViewState::Dashboard => {
                if handle_dashboard_keybinds(key, app) {
                    return Ok(true);
                }
            }
            crate::app::ViewState::InstalledPackages => {
                // if app.showing_search_window {
                //     handle_search_keybinds(key, app);
                // } else if handle_installed_packages_keybinds(key, app) {
                //     return Ok(true);
                // }
            }
            crate::app::ViewState::AvailablePackages => {
                if handle_available_packages_keybinds(key, app) {
                    return Ok(true);
                }
            }
        }
    }

    Ok(false)
}

fn handle_available_packages_keybinds(key: KeyEvent, app: &mut App) -> bool {
    match key.code {
        KeyCode::Char('q') => return true,
        KeyCode::Tab => app.cycle_current_tab(),

        _ => {}
    }

    false
}
//
// fn handle_search_keybinds(key: KeyEvent, app: &mut App) {
//     match key.code {
//         KeyCode::Esc => app.toggle_search_window(),
//         KeyCode::Enter => {
//             app.search();
//             app.toggle_search_window();
//         }
//         _ => {
//             app.textarea.input_without_shortcuts(key);
//             app.search_found = Some(app.search());
//         }
//     }
// }
//
// fn handle_installed_packages_keybinds(key: KeyEvent, app: &mut App) -> bool {
//     match key.modifiers {
//         KeyModifiers::CONTROL => match key.code {
//             KeyCode::Char('d') => app
//                 .list_state
//                 .select(Some(app.list_state.selected().unwrap_or(0) + 30)),
//             KeyCode::Char('u') => {
//                 if let Some(selected) = app.list_state.selected() {
//                     if selected < 30 {
//                         app.list_state.select(Some(0));
//                     } else {
//                         app.list_state.select(Some(selected - 30));
//                     }
//                 }
//             }
//             _ => {}
//         },
//         _ => match key.code {
//             KeyCode::Char('q') => return true,
//             KeyCode::Char('j') => app.list_state.select_next(),
//             KeyCode::Char('k') => app.list_state.select_previous(),
//             KeyCode::Char('/') => app.toggle_search_window(),
//             KeyCode::Char('n') => app.next_search(),
//             KeyCode::Char('N') => app.prev_search(),
//
//             KeyCode::Tab => app.cycle_current_tab(),
//
//             _ => {}
//         },
//     }
//
//     false
// }
//
fn handle_dashboard_keybinds(key: KeyEvent, app: &mut App) -> bool {
    match key.code {
        KeyCode::Char('q') => return true,
        KeyCode::Tab => app.cycle_current_tab(),

        _ => {}
    }

    false
}
