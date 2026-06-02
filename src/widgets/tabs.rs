use ratatui::{
    Frame,
    layout::Rect,
    symbols,
    widgets::{Block, Borders, Tabs},
};

use crate::{app::ViewState, theme::Theme};

pub fn render_tab(frame: &mut Frame, area: Rect, selected_tab: &ViewState) {
    let selected_index = match selected_tab {
        ViewState::Dashboard => 0,
        ViewState::InstalledPackages => 1,
        ViewState::AvailablePackages => 2,
    };

    let tabs = Tabs::new(vec!["Dashboard", "Installed", "Browse"])
        .block(Block::default().borders(Borders::ALL).style(Theme::block()))
        .highlight_style(Theme::success())
        .select(selected_index)
        .divider(symbols::DOT)
        .padding(" ", " ");
    frame.render_widget(tabs, area);
}
