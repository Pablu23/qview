use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout},
    prelude::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, ListState, Paragraph},
};

use crate::app::App;

pub fn ui(frame: &mut Frame, app: &mut App) {
    let constraints = [Constraint::Fill(1), Constraint::Length(3)];

    let layout = Layout::vertical(constraints);
    let [top, bottom] = frame.area().layout(&layout);

    let split = Layout::horizontal([Constraint::Percentage(70), Constraint::Percentage(30)])
        .spacing(1)
        .split(top);

    render_packages(frame, split[0], app);
    render_use_flags(frame, split[1], app);

    let key_hint = match app.showing_search_window {
        false => Span::styled(
            "(q) to quit | (/) to search | (j) down | (k) up",
            Style::default().fg(Color::Yellow),
        ),
        true => Span::styled(
            "(esc) to quit search | (enter) to search",
            Style::default().fg(Color::Yellow),
        ),
    };

    let key_notes_footer =
        Paragraph::new(Line::from(key_hint)).block(Block::default().borders(Borders::ALL));
    frame.render_widget(key_notes_footer, bottom);

    if app.showing_search_window {
        let mut popup_block = Block::default()
            .title("Search")
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded);

        if let Some(found) = app.search_found {
            if found {
                popup_block = popup_block.border_style(Style::default().fg(Color::Green))
            } else {
                popup_block = popup_block.border_style(Style::default().fg(Color::Red))
            }
        }

        app.textarea.set_block(popup_block);
        app.textarea.set_style(Style::default().fg(Color::Yellow));
        app.textarea.set_cursor_line_style(Style::default());

        let area = search_popup_rect(70, frame.area());
        frame.render_widget(Clear, area);
        frame.render_widget(&app.textarea, area);
    }
}

fn search_popup_rect(percent_x: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(10), Constraint::Length(3)])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}

fn render_packages(frame: &mut Frame, area: Rect, app: &mut App) {
    let items = app.installed_packages().into_iter().map(|x| x.name.clone());
    let list = List::new(items)
        .style(Color::White)
        .highlight_style(Modifier::REVERSED)
        .highlight_symbol("> ")
        .block(
            Block::new()
                .borders(Borders::ALL)
                .title("Installed packages"),
        );

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
        .block(Block::new().borders(Borders::ALL).title("USE Flags"));

    let mut list_state = ListState::default();
    list_state.select_first();

    frame.render_stateful_widget(list, area, &mut list_state);
}
