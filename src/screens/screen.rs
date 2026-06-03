use ratatui::{Frame, layout::Rect};

use crate::{
    gentoo::Portage,
    signal::{Event, Signal},
};

pub(crate) trait Screen {
    fn draw(&mut self, frame: &mut Frame, area: Rect, repo: &Portage);
    fn update(&mut self, event: &Event, repo: &Portage) -> Option<Signal>;
}
