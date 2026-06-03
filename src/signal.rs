use ratatui::crossterm::event::KeyEvent;

use crate::app::LoadingState;

pub enum Signal {
    Quit,
    CycleTab,
}

pub enum Event {
    KeyEvent(KeyEvent),
    LoadStateUpdate(LoadingState),
}
