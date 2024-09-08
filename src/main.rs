use crate::{app::App, args::Args, ui::ui};
use clap::Parser;
use event::EventHandler;
use polars::io::SerReader;
use polars::lazy::frame::IntoLazy;
use polars::prelude::CsvParseOptions;
use polars::{frame::DataFrame, prelude::CsvReadOptions};
use polars_sql::SQLContext;
use std::error::Error;
use std::result::Result;

mod app;
mod args;
mod event;
mod handler;
mod tui;
mod ui;

fn main() -> Result<(), Box<dyn Error>> {
    let mut terminal = tui::init()?;
    let args = Args::parse();
    let df = csv_reader(args)?;

    let mut ctx = SQLContext::new();
    ctx.register("df", df.clone().lazy());

    let mut app = App::new(df, ctx);

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
        .with_parse_options(CsvParseOptions::default().with_separator(args.separator as u8))
        .with_n_rows(args.n_rows)
        .try_into_reader_with_file_path(args.filename.into())?
        .finish()?;

    Ok(df)
}
