use ratatui::{Frame, crossterm::event::KeyEvent, layout::Rect};

use crate::{app::LoadingState, gentoo::Portage, signal::Signal};

pub(crate) trait Screen {
    fn draw(&mut self, frame: &mut Frame, area: Rect, repo: &Portage, loading_state: &LoadingState);
    fn update(&mut self, key: KeyEvent, repo: &Portage) -> Option<Signal>;
}
