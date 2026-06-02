use ratatui::{Frame, crossterm::event::KeyEvent, layout::Rect};

use crate::{gentoo::Portage, signal::Signal};

pub(crate) trait Screen {
    fn draw(&mut self, frame: &mut Frame, area: Rect, repo: &Portage);
    fn update(&mut self, key: KeyEvent, repo: &Portage) -> Option<Signal>;
}
