use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Modifier},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::{gentoo::InstalledPackage, theme::Theme};

pub fn render_use_flags(frame: &mut Frame, area: Rect, package: Option<&InstalledPackage>) {
    let Some(package) = package else {
        return;
    };

    let items = package.iuse.iter().map(|x| {
        let text_style = if package.enabled_use_flags.contains(&x.name) {
            Theme::success()
        } else if x.default {
            Theme::info()
        } else {
            Theme::error()
        };

        let text_style = if x.default {
            text_style.add_modifier(Modifier::ITALIC)
        } else {
            text_style
        };

        ListItem::new(x.name.clone()).style(text_style)
    });
    let list = List::new(items)
        .style(Color::White)
        .scroll_padding(1)
        .direction(ratatui::widgets::ListDirection::TopToBottom)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("USE Flags")
                .border_style(Theme::block()),
        );

    let mut list_state = ListState::default();
    list_state.select_first();

    frame.render_stateful_widget(list, area, &mut list_state);
}
