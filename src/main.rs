use crate::{app::App, ui::ui, args::Args};
use clap::Parser;
use event::EventHandler;
use polars::io::SerReader;
use polars::{frame::DataFrame, prelude::CsvReadOptions};
use std::error::Error;
use std::result::Result;

mod app;
mod event;
mod tui;
mod ui;
mod args;

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = tui::init()?;
    let args = Args::parse();
    let df = csv_reader(args)?;
    let mut app = App::new(df);

    let result = run(&mut terminal, &mut app);
    tui::restore(&mut terminal)?;

    if let Err(err) = result {
        println!("{err:?}");
    }

    Ok(())
}

fn run(terminal: &mut tui::Tui, app: &mut App) -> Result<(), Box<dyn Error>> {
    while !app.exit {
        terminal.draw(|frame| ui(frame, app))?;
        EventHandler::default().handle_events(app)?;
    }

    Ok(())
}

fn csv_reader(args: Args) -> Result<DataFrame, Box<dyn Error>> {
    let df = CsvReadOptions::default()
        .with_ignore_errors(true)
        .try_into_reader_with_file_path(args.filename.into())?
        .finish()?;

    Ok(df)
}