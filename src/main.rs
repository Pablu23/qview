mod app;
mod gentoo;
mod screens;
mod signal;
mod theme;
mod widgets;

use std::{
    io::{self, Stdout, stdout},
    panic::{set_hook, take_hook},
};

use clap::command;
use ratatui::{
    Terminal,
    crossterm::{
        execute,
        terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
    },
    prelude::CrosstermBackend,
};

use crate::{
    app::App,
    gentoo::{Portage, portage_loader},
};

struct TuiGuard;

impl Drop for TuiGuard {
    fn drop(&mut self) {
        _ = restore_tui();
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // Just for --help and --version right now, later --config and possably more
    let _ = command!().get_matches();
    let _guard = TuiGuard;

    let tui = init_tui()?;

    let world_packages = portage_loader::load_world_packages()?;
    let installed_packages = portage_loader::load_installed_packages()?;
    // p.available_packages = portage_loader::load_available_packages()?;
    let p = Portage::new(installed_packages, world_packages, vec![]);

    let mut app = App::new(p);

    run(tui, &mut app)?;

    Ok(())
}

fn run(mut terminal: Terminal<CrosstermBackend<Stdout>>, app: &mut App) -> color_eyre::Result<()> {
    loop {
        terminal.draw(|frame| app.draw(frame))?;
        let should_exit = app.update()?;
        if should_exit {
            break;
        }
    }

    Ok(())
}

fn init_tui() -> io::Result<ratatui::Terminal<CrosstermBackend<Stdout>>> {
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    init_panic_hook();
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
