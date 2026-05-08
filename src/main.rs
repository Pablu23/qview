mod app;
mod event;
mod gentoo;
mod ui;

use std::{
    io::{self, Stdout, stdout},
    panic::{set_hook, take_hook},
};

use ratatui::{
    Terminal,
    backend::Backend,
    crossterm::{
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    prelude::CrosstermBackend,
};

use crate::{app::App, event::handle_event, gentoo::Portage, ui::ui};

struct TuiGuard;

impl Drop for TuiGuard {
    fn drop(&mut self) {
        _ = restore_tui();
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    init_panic_hook();

    let _guard = TuiGuard;

    let mut tui = init_tui()?;

    let mut p = Portage::new();
    p.load_world_packages()?;
    p.load_installed_packages()?;

    let mut app = App::new(p);

    run(&mut tui, &mut app)?;

    Ok(())
}

fn run<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()>
where
    io::Error: From<B::Error>,
{
    loop {
        terminal.draw(|frame| ui(frame, app))?;
        let should_exit = handle_event(app)?;
        if should_exit {
            break;
        }
    }

    Ok(())
}

fn init_tui() -> io::Result<ratatui::Terminal<CrosstermBackend<Stdout>>> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    Terminal::new(CrosstermBackend::new(stdout()))
}

fn init_panic_hook() {
    let original_hook = take_hook();
    set_hook(Box::new(move |panic_info| {
        let _ = restore_tui();
        original_hook(panic_info);
    }));
}

fn restore_tui() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}
