use ratatui::{Frame, crossterm::event::KeyEvent, layout::Rect};

use crate::{
    actions::{Action, Signal},
    gentoo::Portage,
};

pub(crate) trait Screen {
    fn draw(&mut self, frame: &mut Frame, area: Rect, repo: &Portage);
    fn update(&mut self, action: KeyEvent) -> Option<Signal>;
}
