use crate::{app::App, ui::ui};
use event::EventHandler;
use std::io::Result;

mod app;
mod event;
mod tui;
mod ui;

fn main() -> Result<()> {
    let mut terminal = tui::init()?;
    let result = run(&mut terminal);
    tui::restore()?;
    result
}

fn run(terminal: &mut tui::Tui) -> Result<()> {
    let mut app = App::new();
    while !app.exit {
        terminal.draw(|frame| ui(frame, &mut app))?;
        EventHandler::default().handle_events(&mut app)?;
    }
    Ok(())
}
