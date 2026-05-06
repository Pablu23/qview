use ratatui::{
    Frame,
    layout::{Constraint, Layout},
    prelude::Rect,
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &mut App) {
    let constraints = [Constraint::Length(1), Constraint::Fill(1)];

    let layout = Layout::vertical(constraints).spacing(1);
    let [top, first] = frame.area().layout(&layout);

    let split = Layout::horizontal([Constraint::Percentage(70), Constraint::Percentage(30)])
        .spacing(1)
        .split(first);

    let title = Line::from_iter([
        Span::from("Installed Packages").bold(),
        Span::from(" (Press 'q' to quit and arrow keys to navigate)"),
    ]);
    frame.render_widget(title.centered(), top);

    render_packages(frame, split[0], app);
    render_use_flags(frame, split[1], app);
}

fn render_packages(frame: &mut Frame, area: Rect, app: &mut App) {
    let items = app.installed_packages().into_iter().map(|x| x.name.clone());
    let list = List::new(items)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ")
        .block(Block::new().borders(Borders::ALL));

    frame.render_stateful_widget(list, area, &mut app.list_state);
}

fn render_use_flags(frame: &mut Frame, area: Rect, app: &mut App) {
    let selected_package_index = match app.list_state.selected() {
        Some(selected) => selected,
        None => 0,
    };
    let selected_package = app.installed_packages()[selected_package_index].clone();

    let items = selected_package.use_flags.into_iter().map(|x| {
        let color = if x.enabled {
            Color::Green
        } else if x.default {
            Color::Blue
        } else {
            Color::Red
        };

        let text_style = if x.default {
            Style::default().add_modifier(Modifier::ITALIC).fg(color)
        } else {
            Style::default().fg(color)
        };

        ListItem::new(x.name).style(text_style)
    });
    let list = List::new(items)
        .style(Color::White)
        .scroll_padding(1)
        .direction(ratatui::widgets::ListDirection::TopToBottom)
        .block(Block::new().borders(Borders::ALL));

    let mut list_state = ListState::default();
    list_state.select_first();

    frame.render_stateful_widget(list, area, &mut list_state);
}
